use chrono::{Datelike, Duration, NaiveDate, Weekday};
use clap::{arg, Command as clapCommand};
use colored::*;
use rand::Rng;
use std::process::Command;

fn main() {
    let matches = clapCommand::new("gitfetch")
        .version("0.1.0")
        .about("Fetch and display GitHub contributions")
        .arg(arg!(-u --username <VALUE>).required(false))
        .arg(arg!(-y --year <VALUE>).required(false))
        .get_matches();

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

    let year = matches
        .get_one::<String>("year")
        .cloned()
        .unwrap_or_else(|| chrono::Local::now().format("%Y").to_string());

    println!("Fetching contributions for user: {}", username);

    // Fetch contribution data
    let contributions = fetch_contributions(&username, &year);

    let graph = generate_contribution_graph(&contributions.daily_contributions);

    // Prepare info text
    let info = vec![
        format!("{} contributions in the last year", contributions.total),
        format!("Longest Streak: {} days", contributions.longest_streak),
        format!("Current Streak: {} days", contributions.current_streak),
        format!(
            "Most Contributions in a Day: {}",
            contributions.max_contributions
        ),
    ];

    // Print colored graph
    print_colored_graph(&graph, &info);
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

fn generate_contribution_graph(daily_contributions: &[(NaiveDate, u32)]) -> String {
    let mut graph = String::new();
    let months = [
        "Jul", "Aug", "Sep", "Oct", "Nov", "Dec", "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul",
    ];

    // Add month names
    graph += "    ";
    for month in months.iter() {
        graph += &format!("{:<8}", month);
    }
    graph += "\n";

    // Generate the graph
    for day in 0..7 {
        let weekday = match day {
            0 => "Mon",
            2 => "Wed",
            4 => "Fri",
            _ => "   ",
        };
        graph += &format!("{} ", weekday);

        for week in 0..53 {
            let index = week * 7 + day;
            if index < daily_contributions.len() {
                let (_, count) = daily_contributions[index];
                let symbol = match count {
                    0 => "  ",
                    1..=3 => "░░",
                    4..=6 => "▒▒",
                    7..=8 => "▓▓",
                    _ => "██",
                };
                graph += symbol;
            } else {
                graph += " ";
            }
        }
        graph += "\n";
    }

    graph += "\nLess   ░░  ▒▒  ▓▓  ██ More\n";
    graph
}

fn print_colored_graph(graph: &str, info: &[String]) {
    let graph_lines: Vec<&str> = graph.lines().collect();

    println!("{}", graph_lines[0]);

    // Print graph
    for (_index, graph_line) in graph_lines.iter().enumerate().skip(1) {
        let mut colored_line: Vec<ColoredString> = Vec::new();
        for c in graph_line.chars() {
            let colored_char: ColoredString = match c {
                '░' => c.to_string().custom_color(CustomColor {
                    r: 13,
                    g: 68,
                    b: 41,
                }),
                '▒' => c.to_string().custom_color(CustomColor {
                    r: 1,
                    g: 108,
                    b: 49,
                }),
                '▓' => c.to_string().custom_color(CustomColor {
                    r: 38,
                    g: 166,
                    b: 65,
                }),
                '█' => c.to_string().custom_color(CustomColor {
                    r: 57,
                    g: 211,
                    b: 83,
                }),
                _ => c.to_string().normal(),
            };
            colored_line.push(colored_char);
        }
        for colored_char in colored_line {
            print!("{}", colored_char);
        }
        println!();
    }

    // Print separator
    println!();

    // Print info
    for info_line in info {
        println!("{}", info_line.bright_cyan());
    }
}

struct ContributionData {
    total: u32,
    longest_streak: u32,
    current_streak: u32,
    max_contributions: u32,
    daily_contributions: Vec<(NaiveDate, u32)>,
}

fn fetch_contributions(username: &str, year: &str) -> ContributionData {
    // This should be implemented to actually fetch data from GitHub
    // For now, we'll generate some dummy data
    let start_date = NaiveDate::from_ymd_opt(year.parse().unwrap_or(2024), 7, 1).unwrap();
    let mut contributions = ContributionData {
        total: 500,
        longest_streak: 30,
        current_streak: 5,
        max_contributions: 20,
        daily_contributions: Vec::new(),
    };

    for i in 0..365 {
        let date = start_date + Duration::days(i);
        let count = rand::thread_rng().gen_range(0..=10);
        contributions.daily_contributions.push((date, count));
    }

    contributions
}
