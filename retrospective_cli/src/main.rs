extern crate dotenv;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::{fs};
use chrono::{Local, NaiveDate};
use dotenv::dotenv;
use retrospective_logic::config::LocalConfig;
use retrospective_logic::{local_retrospective, RetrospectiveError};
use retrospective_api::dev::baileytownsend::retrospective::day;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Creates a new local entry of your day for filling out
    NewDay(NewDayArgs),
    ///Ketch up, catch up, get it? (Runs through your local repo files and syncs your day lexicon to your repo
    Ketchup(KetchupArgs)
}


#[derive(Parser, Debug)]
struct NewDayArgs {
    #[arg(short, long, default_value = "./days")]
    local_repo: PathBuf,
}

#[derive(Parser, Debug)]
struct KetchupArgs {
    #[arg(short, long, default_value = "./days")]
    local_repo: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv().ok();

    let cli = Cli::parse();
    match &cli.command {
        Commands::NewDay(args) => {
            let path = args.local_repo.clone();
            let today = Local::now();
            let filename = format!("{}.json", today.date().format("%Y-%m-%d"));
            let empty_record =  day::RecordData {
                countables: vec![day::CountableData{ count: 0, key: "place_holder".to_string() }.into()],
                events: vec![day::EventData { description: "A place holder".to_string(), key: "place_holder".to_string() }.into()],
                summary: Some("".to_string()),
            };
            let serialized =  serde_json::to_string_pretty(&empty_record)?;
            fs::create_dir_all(&path)?;
            let file_path = path.join(filename);
            fs::write(file_path, serialized)?;
        },
        Commands::Ketchup(args) => {
            let path = args.local_repo.clone();
            let local_config = LocalConfig{
                handle: std::env::var("HANDLE").expect("HANDLE environment variable not set").to_string(),
                password: std::env::var("APP_PASSWORD").expect("APP_PASSWORD environment variable not set").to_string(),
                lexicon_nsid_base: std::env::var("LEXICON_COLLECTION_BASE").expect("LEXICON_COLLECTION_BASE environment variable not set").to_string(),
                local_repo_path: path,
            };

            match local_retrospective(local_config).await{
                Ok(_) => {}
                Err(err) => {
                    log::error!("{:?}", err);
                }
            }
        }
    }

    Ok(())
}
