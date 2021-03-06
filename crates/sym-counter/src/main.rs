use common::{logger::Logger, project::Project};
use std::path::Path;

const INPUT_TAGGED_USERS: &str = "INPUT_TAGGED_USERS";
const GITHUB_TOKEN: &str = "GITHUB_TOKEN";
const COUNT_FILES_EXTENSION: &str = ".md";

type Result<T> = anyhow::Result<T>;

#[tokio::main]
async fn main() -> Result<()> {
    Logger::init();

    let users = std::env::var(INPUT_TAGGED_USERS).unwrap_or_default();

    let mut symbols = 0;
    let mut files = 0;
    let gh_token = std::env::var(GITHUB_TOKEN)?;
    let project = Project::new(gh_token)?;
    let changed_files = project.changed_files()?;
    for filename in changed_files {
        let symbols_count = count_for_file(&filename);
        if filename.ends_with(COUNT_FILES_EXTENSION) {
            symbols += symbols_count;
            files += 1;
        }

        log::info!("File `{}`, symbols: {}", filename, symbols_count);
    }

    log::info!("Filtered files: {}, symbols: {}", files, symbols);

    project
        .comment_pr(format!(
            "Файлов: {}, символов: {}. {} fyi",
            files, symbols, users
        ))
        .await
}

fn count_for_file<P: AsRef<Path>>(file: P) -> usize {
    match std::fs::read_to_string(file) {
        Ok(content) => content.chars().count(),
        Err(e) => {
            log::warn!("Read file to string error: {:?}", e);
            0
        }
    }
}
