use crate::config::LocalConfig;
use std::fs;
use atrium_api::agent::atp_agent::{AtpSession, CredentialSession};
use atrium_api::agent::atp_agent::store::MemorySessionStore;
use atrium_xrpc_client::reqwest::ReqwestClient;
use atrium_api::agent::Agent;
use atrium_common::store::memory::MemoryStore;
use retrospective_api::dev::baileytownsend::retrospective::day;
use retrospective_api::record::KnownRecord;

pub mod config;


#[derive(Debug)]
pub enum RetrospectiveError {
    GeneralError(String),
    LoginError(String)
}




pub async fn local_retrospective(config: LocalConfig) -> Result<(), RetrospectiveError> {
    let files = fs::read_dir(&config.local_repo_path).map_err(|e| {
        RetrospectiveError::GeneralError(format!("Failed to read directory: {}", e))
    })?;

    let json_files: Vec<_> = files
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("json"))
                .unwrap_or(false)
        })
        .collect();

    if json_files.is_empty() {
        return Err(RetrospectiveError::GeneralError("No JSON files found".to_string()));
    }

    let mut days: Vec<(String, day::RecordData)> = vec![];
    for file in json_files {
        match fs::read_to_string(file.path()) {
            Ok(content) => {
                match serde_json::from_str::<day::RecordData>(&content) {
                    Ok(record) => {
                        let just_the_date = file.file_name().to_string_lossy().to_string();
                        days.push((just_the_date, record));
                    },
                    Err(e) => return Err(RetrospectiveError::GeneralError(
                        format!("Failed to parse JSON from {}: {}", file.path().display(), e)
                    ))
                }
            },
            Err(e) => return Err(RetrospectiveError::GeneralError(
                format!("Failed to read file {}: {}", file.path().display(), e)
            ))
        }
    }

    let session = CredentialSession::new(
        ReqwestClient::new("https://bsky.social"),
        MemorySessionStore::default(),
    );
    if let Err(error) = session.login(&config.handle, &config.password).await{
        return Err(RetrospectiveError::LoginError(format!("Failed to login: {}", error)));
    }
    let agent = Agent::new(session);
    sync_retrospective(agent, days, config.lexicon_nsid_base).await
}

pub async fn sync_retrospective(agent: Agent<CredentialSession<MemoryStore<(), AtpSession>, ReqwestClient>>, days: Vec<(String, day::RecordData)>, collection: String)-> Result<(), RetrospectiveError> {

    let handle = agent.did().await.unwrap();

    for day in days {
        let known_record: KnownRecord = day.1.into();
        let collection = format!("{}.day", collection);

        match agent.api.com.atproto.repo.put_record(atrium_api::com::atproto::repo::put_record::InputData{
            collection: collection.parse().unwrap(),
            record: known_record.into(),
            repo: handle.clone().parse().unwrap(),
            rkey: day.0.parse().unwrap(),
            swap_commit: None,
            swap_record: None,
            validate: None,
        }.into()).await{
            Ok(_) => {
                continue;
            }
            Err(err) => {
                return Err(RetrospectiveError::GeneralError(format!("Failed to put record: {}", err)));
            }
        }
    }
    Ok(())
}




