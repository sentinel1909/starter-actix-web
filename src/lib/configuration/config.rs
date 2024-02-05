// src/lib/configuration/config.rs

// dependencies
use confik::{Configuration, FileSource};

// a struct type to represent our app configuration items
#[derive(Debug, PartialEq, Configuration)]
pub struct Config {
    pub address: String,
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self::builder()
            .override_with(FileSource::new("config.toml"))
            .try_build()
            .expect("Unable to build the default app configuration")
    }
}
