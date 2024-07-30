use anyhow::Result;
mod contribution_analyzer;
use chrono::Local;
mod errors;
mod github_client;
mod graph_generator;
use clap::{arg, Command as clapCommand};
use core::result::Result::Ok;
use errors::FetchError;
use github_client::fetch_contributions;
use graph_generator::{generate_contribution_graph, print_colored_graph};
use octocrab::Octocrab;
use std::process::Command;

mod config_manager;
#[tokio::main]
async fn main() -> Result<()> {
    let matches = clapCommand::new("gitfetch")
        .version("0.1.0")
        .about("Fetch and display GitHub contributions")
        .arg(arg!(-u --username <VALUE>).required(false))
        .arg(arg!(-y --year <VALUE>).required(false))
        .subcommand(
            clapCommand::new("add-token")
                .about("Add GitHub access token")
                .arg(arg!(<TOKEN> "GitHub access token")),
        )
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("add-token") {
        let token = matches.get_one::<String>("TOKEN").unwrap();
        return config_manager::add_token(token);
    }

    let config = config_manager::read_config()?;
    let username = match matches.get_one::<String>("username") {
        Some(name) => name.to_string(),
        None => match get_git_global_username() {
            Some(name) => name,
            None => {
                eprintln!("Error: No username provided and couldn't fetch git global user.");
                eprintln!("Please provide a username with -u or set your git global user.name");
                std::process::exit(1);
            }
        },
    };

    let year_specified = matches.contains_id("year");
    let year = matches
        .get_one::<String>("year")
        .cloned()
        .unwrap_or_else(|| Local::now().format("%Y").to_string());

    let token = match config.github_token {
        Some(ref token) => token,
        None => {
            eprintln!("GitHub token not found in config. Please run 'gitfetch add-token <TOKEN>' to add your token.");
            std::process::exit(1);
        }
    };

    let octocrab = Octocrab::builder()
        .personal_token(token.to_string())
        .build()?;

    // Fetch contribution data
    let contributions = match fetch_contributions(&username, &year, year_specified, octocrab).await
    {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error fetching contributions: {}", e);
            match e {
                FetchError::YearParseError(_) => {
                    eprintln!("Invalid year format provided");
                }
                FetchError::DateCreationError => {
                    eprintln!("Failed to create a valid date");
                }
                FetchError::GitHubApiError(api_error) => {
                    eprintln!("GitHub API error: {}", api_error);
                }
                FetchError::UnexpectedResponseFormat => {
                    eprintln!("Received unexpected response format from GitHub");
                }
            }
            std::process::exit(1);
        }
    };

    let graph =
        generate_contribution_graph(&contributions.daily_contributions, year_specified, &config);

    // Prepare info text
    let info = vec![
        format!("{}@{}", username, year),
        format!("Total contributions: {}", contributions.total),
        format!("Longest Streak: {} days", contributions.longest_streak),
        format!("Current Streak: {} days", contributions.current_streak),
        format!(
            "Most Contributions in a Day: {}",
            contributions.max_contributions
        ),
    ];

    // Print colored graph
    print_colored_graph(&graph, &info, &config);

    Ok(())
}

fn get_git_global_username() -> Option<String> {
    let output = Command::new("git")
        .args(["config", "--global", "user.name"])
        .output()
        .ok()?;

    if output.status.success() {
        String::from_utf8(output.stdout)
            .ok()
            .map(|s| s.trim().to_string())
    } else {
        None
    }
}
