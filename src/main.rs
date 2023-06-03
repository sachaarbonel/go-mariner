mod config;
mod helper;
mod issue;
mod linter;
mod rules;

use clap::{Parser, Subcommand};
use config::{Config, RuleConfig};
use linter::Linter;
use serde_yaml;
use std::fs;
use std::io::Read;
use std::path::PathBuf;


#[derive(Parser)]
#[clap(name = "golang_linter", version = "0.1.0", about = "A Golang Linter")]
struct GolangLinterOpts {
    /// Path to the Golang file to be linted
    #[clap(short, long)]
    file: Option<String>,

    /// Sets a custom config file
    #[clap(short, long)]
    config: Option<PathBuf>,
}

fn main() {
    let opts: GolangLinterOpts = GolangLinterOpts::parse();

    let code = match opts.file {
        Some(file) => fs::read_to_string(file).expect("Failed to read the Golang file"),
        None => String::from_utf8_lossy(
            &std::io::stdin()
                .lock()
                .bytes()
                .flatten()
                .collect::<Vec<u8>>(),
        )
        .into_owned(),
    };

    let config = if let Some(config_path) = opts.config.as_deref() {
        let config_content =
            fs::read_to_string(config_path).expect("Failed to read the config file");
        serde_yaml::from_str(&config_content).expect("Invalid configuration")
    } else {
        // Use default configuration if no config file is provided.
        Config {
            rules: vec![RuleConfig {
                name: "unused_variable".to_string(),
                enabled: true,
            }],
        }
    };

    let linter = Linter::new(config);
    let issues = linter.lint(&code);

    if issues.is_empty() {
        println!("No issues found.");
    } else {
        println!("Issues found:");
        for issue in issues {
            println!("{}", issue);
        }
    }
}
