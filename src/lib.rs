#[cfg(test)]
mod tests {
    use std::{
        fs::{read_to_string, File},
        io::{BufReader, Cursor},
    };

    use hf_hub::{api::sync::Api, Repo, RepoType};
    use tokenizers::Tokenizer;

    use super::*;

    #[test]
    fn test_whisper() -> Result<(), Box<dyn std::error::Error>> {
        let model_id = "openai/whisper-tiny";
        let revision = "main";

        let api = Api::new()?;
        let repo = api.repo(Repo::with_revision(
            model_id.to_owned(),
            RepoType::Model,
            revision.to_owned(),
        ));

        let tokenizer_path = repo.get("tokenizer.json")?;
        eprintln!("Loading tokenizer from {}", tokenizer_path.display());

        // BUG: Tokenizer from_reader fails but not from_str
        let content = read_to_string(&tokenizer_path)?;
        eprintln!("Loading with from_str");
        let tokenizer: Tokenizer = serde_json::from_str(&content)?; // works

        eprintln!("Loading with from_reader");
        let tokenizer: Tokenizer =
            serde_json::from_reader(Cursor::new(&content)).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                serde_json::from_str(&content).unwrap()
            });

        eprintln!("Loading with from_reader directly");
        let tokenizer: Tokenizer =
            serde_json::from_reader(BufReader::new(File::open(tokenizer_path)?))?; // fails

        Ok(())
    }
}
