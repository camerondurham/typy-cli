use std::fs;
use toml;
use serde::{Deserialize, Serialize};
use dirs::home_dir;
use std::path::PathBuf;
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct ThemeTable {
    pub fg: Option<String>,
    pub missing: Option<String>,
    pub error: Option<String>,
    pub accent: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct GraphTable {
    pub data: Option<String>,
    pub title: Option<String>,
    pub axis: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct CursorTable {
    pub style: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct ModesTable {
    pub default_mode: Option<String>,
    pub uppercase_chance: Option<String>,
    pub punctuation_chance: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct LanguageTable {
    pub lang: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct ConfigToml {
    theme: Option<ThemeTable>,
    graph: Option<GraphTable>,
    cursor: Option<CursorTable>,
    modes: Option<ModesTable>,
    language: Option<LanguageTable>,
}

impl ConfigToml {
    pub fn new() -> Self {
        let mut config_filepaths: Vec<PathBuf> = vec![PathBuf::from("./config.toml")];

        if let Some(home_path) = home_dir() {
            config_filepaths.push(home_path.join(".config/typy/config.toml"));
        }

        let mut content = "".to_owned();

        for filepath in config_filepaths {
            let result = fs::read_to_string(filepath);

            if result.is_ok() {
                content = result.unwrap();
                break;
            }
        }
        let config_toml: ConfigToml =
            toml::from_str(&content).unwrap_or_else(|_| ConfigToml::default());
        config_toml
    }

    pub fn get_theme(&self) -> Option<ThemeTable> {
        self.theme.clone()
    }

    pub fn get_graph(&self) -> Option<GraphTable> {
        self.graph.clone()
    }

    pub fn get_cursor(&self) -> Option<CursorTable> {
        self.cursor.clone()
    }

    pub fn get_modes(&self) -> Option<ModesTable> {
        self.modes.clone()
    }

    pub fn get_language(&self) -> Option<LanguageTable> {
        self.language.clone()
    }
}

impl Default for ConfigToml {
    fn default() -> Self {
        ConfigToml { 
            theme: None,
            graph: None,
            cursor: None,
            modes: None,
            language: None,
        }
    }
}

// Declare the static instance of ConfigToml using lazy_static
lazy_static! {
    static ref CONFIG: Mutex<ConfigToml> = Mutex::new(ConfigToml::new());
}

// Helper function to access the static CONFIG
pub fn get_config() -> &'static Mutex<ConfigToml> {
    &CONFIG
}

