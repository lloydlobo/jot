use std::{
    env::var,
    fs,
    io::{
        self,
        ErrorKind,
        Read,
        Write,
    },
    path::PathBuf,
};

use serde::{
    Deserialize,
    Serialize,
};

use crate::dirs::home_dir;

const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Serialize, Deserialize, Default)]
struct Config {
    repo: PathBuf,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConfigType {
    Repo,
}

pub trait ConfigManagement {
    fn config_dir_create(&self) -> io::Result<()>;
    fn config_dir_exists(&self) -> bool;
    fn config_read(&self, config_type: ConfigType) -> io::Result<String>;
    fn config_write(&self, config_type: ConfigType, value: String) -> io::Result<()>;
    fn config_rm(&self) -> io::Result<()>;
}

#[derive(Default)]
pub struct ConfigManager;

impl ConfigManagement for ConfigManager {
    fn config_dir_create(&self) -> io::Result<()> {
        self.config_dir_path().and_then(fs::create_dir_all)
    }

    fn config_dir_exists(&self) -> bool {
        self.config_dir_path().and_then(fs::metadata).is_ok()
    }

    fn config_read(&self, config_type: ConfigType) -> io::Result<String> {
        let config = self.config()?;
        let config_value = match config_type {
            ConfigType::Repo => config.repo.display().to_string(),
        };
        Ok(config_value)
    }

    fn config_write(&self, config_type: ConfigType, value: String) -> io::Result<()> {
        let config_path = self.config_path()?;

        // Create file if it doesn't exist, otherwise get it
        let mut file = fs::File::create(config_path)?;

        let mut config = self.config()?;
        match config_type {
            ConfigType::Repo => config.repo = PathBuf::from(value),
        }

        let json = serde_json::to_string(&config)?;

        file.write_all(json.as_bytes())
    }

    /// * Given a path, query the file system to get information about a file, directory, etc.
    /// * Removes a file from the filesystem.
    fn config_rm(&self) -> io::Result<()> {
        let config_path = self.config_path()?;
        // Make sure the file exists
        fs::metadata(&config_path)?;
        fs::remove_file(&config_path)
    }
}

impl ConfigManager {
    fn config_path(&self) -> io::Result<PathBuf> {
        Ok(self.config_dir_path()?.join(CONFIG_FILE_NAME))
    }

    fn config_dir_path(&self) -> io::Result<PathBuf> {
        self.resolve_xdg_config_home()
            .or_else(|| Some(home_dir().unwrap().join(".config").join("jot")))
            .ok_or_else(|| {
                io::Error::new(ErrorKind::NotFound, "Could not resolve your $HOME directory")
            })
    }

    #[allow(clippy::unused_self)]
    fn resolve_xdg_config_home(&self) -> Option<PathBuf> {
        var("XDG_CONFIG_HOME").map_or(None, |path| Some(PathBuf::from(path).join("jot")))
    }

    fn config(&self) -> io::Result<Config> {
        let config_file: PathBuf = self.config_path()?;
        // Make sure the file exists
        fs::metadata(&config_file)?;

        let mut file = fs::File::open(&config_file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        if contents.is_empty() {
            return Ok(Config::default());
        }

        Ok(serde_json::from_str(&contents)?)
    }
}
