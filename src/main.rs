use clap::{Parser, Subcommand};
use prettytable::{Table, row};
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Language {
    name: String,
    year: u32,
    creators: Vec<String>,
    paradigm: Vec<String>,
    typing: String,
    influenced_by: Vec<String>,
}

#[derive(Debug, Parser)]
#[clap(
    name = "kapa",
    version = "1.0",
    about = "Programming language information tool"
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// List all languages
    List,

    /// Search for a specific language
    Search {
        #[clap(help = "Language name to search for")]
        name: String,
    },

    /// Display languages created in a specific year
    Year {
        #[clap(help = "Year to filter languages by")]
        year: u32,
    },

    /// Display languages by creator
    Creator {
        #[clap(help = "Creator name to filter by")]
        name: String,
    },

    /// Display statistics
    Stats,
}

fn load_languages() -> Vec<Language> {
    // Try multiple possible locations for the data file
    let paths = [
        // Development location
        PathBuf::from("languages.json"),
        // Next to executable
        env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("languages.json"),
        // System data directory
        PathBuf::from("/usr/local/share/kapa/languages.json"),
        // User data directory
        dirs::data_local_dir().unwrap().join("kapa/languages.json"),
    ];

    let data = paths
        .iter()
        .find_map(|path| fs::read_to_string(path).ok())
        .unwrap_or_else(|| {
            let searched_paths = paths
                .iter()
                .map(|p| p.display().to_string())
                .collect::<Vec<_>>()
                .join("\n- ");
            panic!(
                "Could not find languages.json in any of these locations:\n- {}\n\n\
                Please ensure the data file exists in one of these paths",
                searched_paths
            )
        });

    serde_json::from_str(&data).expect("Failed to parse JSON data")
}

fn print_languages_table(languages: &[Language]) {
    let mut table = Table::new();

    table.add_row(row![bFg=> "Name", "Year", "Creators", "Paradigm", "Typing"]);

    for lang in languages {
        table.add_row(row![
            lang.name,
            lang.year,
            lang.creators.join(", "),
            lang.paradigm.join(", "),
            lang.typing
        ]);
    }

    table.printstd();
}

fn main() {
    let cli = Cli::parse();
    let languages = load_languages();

    match cli.command {
        Commands::List => {
            println!("Displaying all programming languages:");
            print_languages_table(&languages);
        }
        Commands::Search { name } => {
            let filtered: Vec<_> = languages
                .iter()
                .filter(|lang| lang.name.to_lowercase().contains(&name.to_lowercase()))
                .cloned()
                .collect();

            if filtered.is_empty() {
                println!("No languages found matching '{}'", name);
            } else {
                println!("Search results for '{}':", name);
                print_languages_table(&filtered);
            }
        }
        Commands::Year { year } => {
            let filtered: Vec<_> = languages
                .iter()
                .filter(|lang| lang.year == year)
                .cloned()
                .collect();

            if filtered.is_empty() {
                println!("No languages created in {}", year);
            } else {
                println!("Languages created in {}:", year);
                print_languages_table(&filtered);
            }
        }
        Commands::Creator { name } => {
            let filtered: Vec<_> = languages
                .iter()
                .filter(|lang| {
                    lang.creators
                        .iter()
                        .any(|c| c.to_lowercase().contains(&name.to_lowercase()))
                })
                .cloned()
                .collect();

            if filtered.is_empty() {
                println!("No languages found created by '{}'", name);
            } else {
                println!("Languages created by '{}':", name);
                print_languages_table(&filtered);
            }
        }
        Commands::Stats => {
            let count = languages.len();
            let earliest = languages.iter().min_by_key(|l| l.year).unwrap();
            let latest = languages.iter().max_by_key(|l| l.year).unwrap();

            println!("Programming Language Statistics:");
            println!("- Total languages: {}", count);
            println!("- Earliest language: {} ({})", earliest.name, earliest.year);
            println!("- Latest language: {} ({})", latest.name, latest.year);

            let mut paradigm_counts = std::collections::HashMap::new();
            for lang in &languages {
                for paradigm in &lang.paradigm {
                    *paradigm_counts.entry(paradigm).or_insert(0) += 1;
                }
            }

            println!("\nParadigm Counts:");
            let mut table = Table::new();
            table.add_row(row![bFg=> "Paradigm", "Count"]);
            for (paradigm, count) in paradigm_counts {
                table.add_row(row![paradigm, count]);
            }
            table.printstd();
        }
    }
}
