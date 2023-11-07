// main.rs

mod ai;
mod file_processing;
mod input_utility;

use colored::*;
use log::info;

use file_processing::process_files;
use input_utility::{get_language_input, get_user_choice};
use simple_logger;
use std::{
    collections::HashMap,
    error::Error,
    io,
    path::{Path, PathBuf},
};
use tokio;

const JAVASCRIPT: &str = "JavaScript";
const TYPESCRIPT: &str = "TypeScript";

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum TargetLanguage {
    JavaScript,
    TypeScript,
}

impl Default for TargetLanguage {
    fn default() -> Self {
        TargetLanguage::JavaScript
    }
}

impl std::fmt::Display for TargetLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetLanguage::JavaScript => write!(f, "JavaScript"),
            TargetLanguage::TypeScript => write!(f, "TypeScript"),
        }
    }
}

fn check_directory(dir: &str) -> Result<(), io::Error> {
    if !std::path::Path::new(dir).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Error: The directory '{}' does not exist.", dir),
        ));
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let problems_folder = get_user_choice(
        "Enter the path to the 'problems' folder (relative to the root directory):\nPress Enter for default value: ",
        Some("problems/"),
        Some(colored::Color::BrightCyan),
    );

    let solutions_folder = get_user_choice(
        "Enter the path to the 'solutions' folder (relative to the root directory):\nPress Enter for default value",
        Some("solutions/"),
        Some(colored::Color::BrightBlue),
    );

    check_directory(&problems_folder)?;
    check_directory(&solutions_folder)?;

    let mut output_folders = HashMap::new();
    output_folders.insert(TargetLanguage::TypeScript, PathBuf::from(TYPESCRIPT));
    output_folders.insert(TargetLanguage::JavaScript, PathBuf::from(JAVASCRIPT));

    for folder in output_folders.values() {
        for subfolder in ["problems", "solutions"].iter() {
            tokio::fs::create_dir_all(folder.join(subfolder)).await?;
        }
    }

    simple_logger::init().expect("Failed to initialize logger");

    let target_language = get_language_input("Select your language:");

    process_files(
        Path::new(&problems_folder),
        &output_folders,
        "problem_",
        &target_language,
    )
    .await?;
    process_files(
        Path::new(&solutions_folder),
        &output_folders,
        "solution_",
        &target_language,
    )
    .await?;

    info!("{}", "Conversion complete!".bright_green());
    log::logger().flush();

    Ok(())
}
