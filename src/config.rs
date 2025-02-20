use config::{Config as ConfigLib, File};
use serde::Deserialize;
use color_eyre::eyre::{Result, Report};

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    pub colors: Colors,
}

#[derive(Debug, Deserialize)]
pub struct Colors {
    pub background_color: Vec<u8>,
    pub log_background_color: Vec<u8>,
    pub header_background_color: Vec<u8>,
    pub line_header_color: Vec<u8>,
    pub list_background_color: Vec<u8>,
    pub list_background_color_alt_row: Vec<u8>,
    pub list_selected_background_color: Vec<u8>,
    pub list_selected_foreground_color: Vec<u8>,
    pub search_bar_foreground_color: Vec<u8>,
    pub login_foreground_color: Vec<u8>,
}

/// load config from `config.toml` file
pub fn load_config() -> Result<ConfigFile> {
    let config = ConfigLib::builder()
        .add_source(File::with_name("../config.toml"))
        .build()
        .map_err(|e| Report::new(e))?;

    let colors: Colors = config.get("colors")
        .map_err(|e| Report::new(e))?;

    Ok(ConfigFile { colors })
}

