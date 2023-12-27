use std::{ops::Deref, path::Path};

use anyhow::Result;
use figment::{
    providers::{Format, Json, Toml, Yaml},
    Figment,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {}

impl Config {
    pub fn parse<T: AsRef<Path>>(dir: Option<T>) -> Result<Self> {
        dir.map(Self::parse_from_file)
            .unwrap_or_else(Self::parse_from_cfgdir)
    }

    pub fn parse_from_cfgdir() -> Result<Self> {
        let dirs = dirs::config_dir()
            .map(|d| d.join("crow"))
            .ok_or_else(|| anyhow::anyhow!("could not resolve project directories"))?;

        Ok(Figment::new()
            .merge(Toml::file("fw.toml"))
            .merge(Yaml::file("fw.yml"))
            .merge(Json::file("fw.json"))
            .merge(Toml::file(dirs.join("config.toml")))
            .merge(Yaml::file(dirs.join("config.yml")))
            .merge(Json::file(dirs.join("config.json")))
            .extract()?)
    }

    pub fn parse_from_file<T: AsRef<Path>>(path: T) -> Result<Self> {
        let ext = path.as_ref().extension().unwrap_or_default();
        let mut figment = Figment::new();

        figment = match ext.to_string_lossy().deref() {
            "yml" | "yaml" => figment.merge(Yaml::file(path)),
            "toml" => figment.merge(Toml::file(path)),
            "json" => figment.merge(Json::file(path)),
            _ => anyhow::bail!("invalid config file type"),
        };

        Ok(figment.extract()?)
    }
}
