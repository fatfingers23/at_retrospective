struct LocalConfig {
    /// Your handle for your atproto account
    pub handle: String,
    /// Your atproto password
    pub password: String,
    /// base you want to use for lexicon to show up in your atproto repo. Mine is reversed of my handle dev.baileytownsend
    pub lexicon_nsid_base: String,
    /// The local folders for your atproto json files to build daily lexicon records
    pub local_repo_path: PathBuf,
}
