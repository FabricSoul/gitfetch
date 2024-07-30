use crate::contribution_analyzer::calculate_contribution_ranges;
use chrono::{Datelike, Duration, Local, NaiveDate};
use colored::Colorize;
use colored::CustomColor;

use crate::config_manager::{Config, GraphColors, TextColors};

pub fn generate_contribution_graph(
    daily_contributions: &[(NaiveDate, u32)],
    year_specified: bool,
    config: &Config,
) -> String {
    let mut graph = String::new();
    let current_date = Local::now().naive_local().date();
    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    // Calculate the date range
    let start_date = daily_contributions
        .first()
        .map(|(date, _)| *date)
        .unwrap_or(current_date);
    let end_date = daily_contributions
        .last()
        .map(|(date, _)| *date)
        .unwrap_or(current_date);
    let total_days = (end_date - start_date).num_days() as usize;
    let num_weeks = (total_days / 7) + 1;

    let percentiles = config
        .graph_data
        .as_ref()
        .map(|data| data.percentiles)
        .unwrap_or([0, 30, 60, 90]);

    // Calculate contribution ranges
    let contribution_ranges = calculate_contribution_ranges(daily_contributions, &percentiles);
    // Calculate column-based month spans
    let month_spans = calculate_month_spans(start_date, end_date, num_weeks, year_specified);

    // Add month names with proper spacing
    graph += "    ";
    for (month, span) in month_spans.iter() {
        graph += &format!("{:<width$}", months[*month], width = span * 2);
    }
    graph += "\n";

    // Generate the graph
    for day in 0..7 {
        let weekday = match day {
            1 => "Mon",
            3 => "Wed",
            5 => "Fri",
            _ => "   ",
        };
        graph += &format!("{} ", weekday);
        for week in 0..num_weeks {
            let index = week * 7 + day;
            if index < daily_contributions.len() {
                let (_, count) = daily_contributions[index];
                let symbol = match count {
                    0 => "  ",
                    c if c <= contribution_ranges[1] => "░░",
                    c if c <= contribution_ranges[2] => "▒▒",
                    c if c <= contribution_ranges[3] => "▓▓",
                    _ => "██",
                };
                graph += symbol;
            } else {
                graph += "  ";
            }
        }
        graph += "\n";
    }
    graph += "\nLess   ░░  ▒▒  ▓▓  ██ More\n";
    graph
}

pub fn print_colored_graph(graph: &str, info: &[String], config: &Config) {
    let graph_lines: Vec<&str> = graph.lines().collect();

    // Print the first line
    println!("{}", graph_lines[0]);

    // Define default graph colors
    let default_graph_colors = GraphColors {
        level1: "13,68,41".to_string(),
        level2: "1,108,49".to_string(),
        level3: "38,166,65".to_string(),
        level4: "57,211,83".to_string(),
    };

    // Get graph colors from config or use defaults
    let graph_colors = config
        .graph_colors
        .as_ref()
        .unwrap_or(&default_graph_colors);

    // Print graph
    for (_index, graph_line) in graph_lines.iter().enumerate().skip(1) {
        for c in graph_line.chars() {
            let colored_char = match c {
                '░' => c
                    .to_string()
                    .custom_color(get_color(&graph_colors.level1).unwrap_or(CustomColor {
                        r: 13,
                        g: 68,
                        b: 41,
                    })),
                '▒' => c
                    .to_string()
                    .custom_color(get_color(&graph_colors.level2).unwrap_or(CustomColor {
                        r: 1,
                        g: 108,
                        b: 49,
                    })),
                '▓' => c
                    .to_string()
                    .custom_color(get_color(&graph_colors.level3).unwrap_or(CustomColor {
                        r: 38,
                        g: 166,
                        b: 65,
                    })),
                '█' => c
                    .to_string()
                    .custom_color(get_color(&graph_colors.level4).unwrap_or(CustomColor {
                        r: 57,
                        g: 211,
                        b: 83,
                    })),
                _ => c.to_string().normal(),
            };
            print!("{}", colored_char);
        }
        println!();
    }

    // Print separator
    println!();

    // Define default text colors
    let default_text_colors = TextColors {
        info_color: "0,255,255".to_string(), // Cyan
    };

    // Get text colors from config or use defaults
    let text_colors = config.text_colors.as_ref().unwrap_or(&default_text_colors);

    // Print colored info
    for (index, info_line) in info.iter().enumerate() {
        if index == 0 {
            let parts: Vec<&str> = info_line.split('@').collect();
            print!(
                "{}@",
                parts[0]
                    .custom_color(get_color(&text_colors.info_color).unwrap_or(CustomColor {
                        r: 255,
                        g: 255,
                        b: 255
                    }))
                    .bold()
            );
            println!(
                "{}",
                parts[1]
                    .custom_color(get_color(&text_colors.info_color).unwrap_or(CustomColor {
                        r: 255,
                        g: 255,
                        b: 255
                    }))
                    .bold()
            );
        } else {
            let parts: Vec<&str> = info_line.split(": ").collect();
            if parts.len() == 2 {
                print!(
                    "{}: ",
                    parts[0]
                        .custom_color(get_color(&text_colors.info_color).unwrap_or(CustomColor {
                            r: 0,
                            g: 255,
                            b: 255
                        }))
                        .bold()
                );
                println!("{}", parts[1]);
            } else {
                println!("{}", info_line);
            }
        }
    }
}

fn calculate_month_spans(
    start_date: NaiveDate,
    end_date: NaiveDate,
    num_weeks: usize,
    year_specified: bool,
) -> Vec<(usize, usize)> {
    let mut month_spans = Vec::new();
    let mut current_date = start_date;
    let mut span_start = 0;
    let mut current_month = (current_date.year(), current_date.month0() as usize);

    for week in 0..num_weeks {
        let week_start_month = (current_date.year(), current_date.month0() as usize);
        let week_end = current_date + Duration::days(6);

        if week_start_month != current_month || (year_specified && week == num_weeks - 1) {
            // Month changed or last week reached
            let span = week - span_start;
            if span > 0 {
                month_spans.push((current_month.1, span));
            }

            if year_specified && week_start_month != current_month {
                // If year is specified and month changed, add empty spans for skipped months
                let mut next_month = (current_month.1 + 1) % 12;
                while next_month != week_start_month.1 {
                    month_spans.push((next_month, 0));
                    next_month = (next_month + 1) % 12;
                }
            }

            current_month = week_start_month;
            span_start = week;
        }

        current_date = week_end + Duration::days(1);
        if current_date > end_date {
            break;
        }
    }

    // // Add the last month if it's not empty
    if span_start < num_weeks {
        month_spans.push((current_month.1, num_weeks - span_start));
    }

    // Ensure all months are represented when year is specified
    if year_specified {
        let mut full_year_spans = vec![(0, 0); 12];
        for (month, span) in month_spans {
            full_year_spans[month] = (month, span);
        }
        month_spans = full_year_spans;
    }
    // Remove the first index if the span is lesser than 3
    if !year_specified {
        if let Some(&(_, span)) = month_spans.first() {
            if span < 3 {
                month_spans.remove(0);
            }
        }
    }

    // Ensure we don't have an empty graph if the date range is too short
    if month_spans.is_empty() {
        month_spans.push((start_date.month0() as usize, 1));
    }
    month_spans
}

fn get_color(color_str: &str) -> Result<CustomColor, anyhow::Error> {
    let parts: Vec<&str> = color_str.split(',').collect();
    if parts.len() != 3 {
        anyhow::bail!("Invalid color format. Expected 'r,g,b'");
    }
    Ok(CustomColor {
        r: parts[0].parse()?,
        g: parts[1].parse()?,
        b: parts[2].parse()?,
    })
}
