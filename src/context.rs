use crate::config::Config;
use crate::error::Errors::{self, InvalidPath};
use crate::request::Request;

#[derive(Debug, Clone)]
pub struct Context {
    pub config: &'static Config,
    pub request: Request,
}

impl Context {
    pub fn new(config: &'static Config, request: Request) -> Self {
        Self { config, request }
    }

    pub fn convert_to_path(&self, path: &str) -> Result<String, Errors> {
        let mut relative_path = path.to_owned();
        let base_dir = self.config.base_directory.as_str();

        if let Some(fi) = relative_path.find(base_dir) {
            relative_path.replace_range(fi..fi + base_dir.len(), "");
            Ok(format!("{}{}", base_dir, relative_path))
        } else {
            Err(InvalidPath)
        }
    }

    pub fn is_managed_path(path: &str) -> bool {
        use std::fs::metadata;

        let md = metadata(path).unwrap();

        if !md.is_dir()
            || path.contains("../") != false
            || path.contains("/..") != false
            || path.contains("..") != false
        {
            return false;
        }

        true
    }
}
