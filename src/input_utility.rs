use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::TargetLanguage;
const JAVASCRIPT: &str = "JavaScript";
const TYPESCRIPT: &str = "TypeScript";

pub fn prompt_with_color(prompt: &str, prompt_color: Option<Color>) -> String {
    match prompt_color {
        Some(prompt_color) => prompt.color(prompt_color).to_string(),
        None => prompt.color(Color::White).to_string(),
    }
}

pub fn get_language_input(prompt: &str) -> TargetLanguage {
    let choices = vec![TargetLanguage::JavaScript, TargetLanguage::TypeScript];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt_with_color(prompt, Some(Color::BrightWhite)))
        .items(&choices)
        .interact()
        .unwrap();

    choices[selection].clone()
}

pub fn get_user_choice(
    prompt: &str,
    default_value: Option<&str>,
    prompt_color: Option<Color>,
) -> String {
    match default_value {
        Some(val) => {
            let mut input: Input<String> = Input::new();
            let prompt_string = prompt_with_color(prompt, prompt_color);
            input
                .with_prompt(&prompt_string)
                .default(val.to_string())
                .interact_text()
                .unwrap_or_else(|_| "".to_string())
        }
        None => {
            let choices = vec![JAVASCRIPT, TYPESCRIPT];
            let prompt_string = prompt_with_color(prompt, prompt_color);
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt(&prompt_string)
                .items(&choices)
                .interact()
                .unwrap();
            choices[selection].to_string()
        }
    }
}
