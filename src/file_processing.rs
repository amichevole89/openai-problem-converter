use crate::{ai, TargetLanguage};
use ai::{extract_ai_response, send_ai_request};
use std::collections::HashMap;
use std::fs;
use std::{
    error::Error,
    io,
    path::{Path, PathBuf},
};

fn write_file(file_path: &Path, content: &str) -> Result<(), io::Error> {
    println!("Writing to file: {:?}", file_path);
    fs::write(file_path, content)
}

pub async fn process_files(
    input_folder: &Path,
    output_folders: &HashMap<TargetLanguage, PathBuf>,
    prefix: &str,
    target_language: &TargetLanguage,
) -> Result<(), Box<dyn Error>> {
    let files = fs::read_dir(input_folder)?;
    let mut file_entries: Vec<_> = files.collect::<Result<_, _>>()?;

    file_entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    for entry in file_entries {
        let file_path = entry.path();
        let content = fs::read_to_string(&file_path)?;
        let response = send_ai_request(&content, &target_language.to_string()).await?;

        let question_version = extract_ai_response(&response);

        let file_index = file_path
            .file_stem()
            .expect("Invalid file path")
            .to_string_lossy()
            .replace(prefix, "");
        let modified_prefix = prefix[..prefix.len() - 1].to_string();
        let file_extension = match target_language {
            TargetLanguage::TypeScript => ".ts",
            TargetLanguage::JavaScript => ".js",
        };
        let output_folder = output_folders
            .get(target_language)
            .cloned()
            .unwrap_or_default();

        let output_file = output_folder.join(format!(
            "{}{}{}{}",
            if !output_folder.exists() {
                ""
            } else {
                input_folder.to_str().unwrap_or("")
            },
            modified_prefix,
            file_index,
            file_extension
        ));

        write_file(&output_file, &question_version)?;
    }

    Ok(())
}
