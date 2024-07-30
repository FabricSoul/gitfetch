use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub github_token: Option<String>,
    pub graph_colors: Option<GraphColors>,
    pub text_colors: Option<TextColors>,
    pub graph_data: Option<GraphData>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GraphColors {
    pub level1: String,
    pub level2: String,
    pub level3: String,
    pub level4: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TextColors {
    pub info_color: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GraphData {
    pub percentiles: [usize; 4],
}
impl Default for Config {
    fn default() -> Self {
        Config {
            github_token: None,
            graph_colors: Some(GraphColors {
                level1: "13,68,41".to_string(),
                level2: "1,108,49".to_string(),
                level3: "38,166,65".to_string(),
                level4: "57,211,83".to_string(),
            }),
            text_colors: Some(TextColors {
                info_color: "0,255,255".to_string(),
            }),
            graph_data: Some(GraphData {
                percentiles: [0, 30, 60, 90],
            }),
        }
    }
}

fn get_config_path() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME environment variable not set");
    PathBuf::from(home)
        .join(".config")
        .join("gitfetch")
        .join("config.toml")
}

pub fn read_config() -> Result<Config> {
    let config_path = get_config_path();
    let default_config = Config::default();
    if config_path.exists() {
        let config_str = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file: {:?}", config_path))?;
        let mut config: Config = toml::from_str(&config_str)
            .with_context(|| format!("Failed to parse config file: {:?}", config_path))?;

        // Merge with default config
        if config.graph_colors.is_none() {
            config.graph_colors = default_config.graph_colors;
        }
        if config.text_colors.is_none() {
            config.text_colors = default_config.text_colors;
        }
        if config.graph_data.is_none() {
            config.graph_data = default_config.graph_data;
        }

        Ok(config)
    } else {
        Ok(Config::default())
    }
}

fn save_config(config: &Config) -> Result<()> {
    let config_path = get_config_path();
    let config_dir = config_path.parent().unwrap();
    fs::create_dir_all(config_dir)
        .with_context(|| format!("Failed to create config directory: {:?}", config_dir))?;
    let config_str =
        toml::to_string_pretty(config).with_context(|| "Failed to serialize config")?;
    fs::write(&config_path, config_str)
        .with_context(|| format!("Failed to write config file: {:?}", config_path))?;
    Ok(())
}

pub fn add_token(token: &str) -> Result<()> {
    let mut config = read_config()?;
    config.github_token = Some(token.to_string());
    save_config(&config)?;
    println!("Token added successfully.");
    Ok(())
}
