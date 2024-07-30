use crate::contribution_analyzer::ContributionData;
use crate::errors::FetchError;
use chrono::NaiveDate;
use octocrab::Octocrab;
use serde_json::Value;

pub async fn fetch_contributions(
    username: &str,
    year: &str,
    year_specified: bool,
    octocrab: Octocrab,
) -> Result<ContributionData, FetchError> {
    let query: &str;
    let response: serde_json::Value;

    if year_specified {
        let year_int: i32 = year.parse()?;
        let from = format!("{}-01-01T00:00:00Z", year_int);
        let to = format!("{}-12-31T23:59:59Z", year_int);

        query = r#"
        query($userName:String!, $from:DateTime!, $to:DateTime!) {
            user(login: $userName) {
                contributionsCollection(from: $from, to: $to) {
                    contributionCalendar {
                        totalContributions
                        weeks {
                            contributionDays {
                                contributionCount
                                date
                            }
                        }
                    }
                }
            }
        }
    "#;

        response = octocrab
            .graphql(&serde_json::json!({
                    "query": query,
                    "variables": {
                    "userName": username,
                    "from": from,
                    "to": to
                }
            }))
            .await?;
    } else {
        query = r#"
        query($userName:String!) {
            user(login: $userName) {
                contributionsCollection {
                    contributionCalendar {
                        totalContributions
                        weeks {
                            contributionDays {
                                contributionCount
                                date
                            }
                        }
                    }
                }
            }
        }
    "#;

        response = octocrab
            .graphql(&serde_json::json!({
                "query": query,
                "variables": {
                    "userName": username
                }
            }))
            .await?;
    }

    parse_contribution_data(&response)
}

fn parse_contribution_data(value: &Value) -> Result<ContributionData, FetchError> {
    let calendar = &value["data"]["user"]["contributionsCollection"]["contributionCalendar"];

    let total = calendar["totalContributions"]
        .as_u64()
        .ok_or(FetchError::UnexpectedResponseFormat)? as u32;

    let mut daily_contributions = Vec::new();
    let mut longest_streak = 0;
    let mut max_contributions = 0;
    let mut streak = 0;

    if let Some(weeks) = calendar["weeks"].as_array() {
        for week in weeks {
            if let Some(days) = week["contributionDays"].as_array() {
                for day in days {
                    let count = day["contributionCount"]
                        .as_u64()
                        .ok_or(FetchError::UnexpectedResponseFormat)?
                        as u32;
                    let date = NaiveDate::parse_from_str(
                        day["date"]
                            .as_str()
                            .ok_or(FetchError::UnexpectedResponseFormat)?,
                        "%Y-%m-%d",
                    )
                    .map_err(|_| FetchError::DateCreationError)?;

                    daily_contributions.push((date, count));

                    if count > 0 {
                        streak += 1;
                        longest_streak = longest_streak.max(streak);
                        max_contributions = max_contributions.max(count);
                    } else {
                        streak = 0;
                    }
                }
            }
        }
    }

    // Calculate current streak
    let current_streak = daily_contributions
        .iter()
        .rev()
        .take_while(|(_, count)| *count > 0)
        .count() as u32;

    Ok(ContributionData {
        total,
        longest_streak,
        current_streak,
        max_contributions,
        daily_contributions,
    })
}
