use std::fs;
use std::path::Path;

use crate::error::{OpenDawError, Result};

use super::Project;

pub fn save_project(project: &Project, path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(project)
        .map_err(|error| OpenDawError::Io(error.to_string()))?;
    fs::write(path, json).map_err(|error| OpenDawError::Io(error.to_string()))
}

pub fn load_project(path: &Path) -> Result<Project> {
    let content = fs::read_to_string(path).map_err(|error| OpenDawError::Io(error.to_string()))?;
    serde_json::from_str(&content).map_err(|error| OpenDawError::Io(error.to_string()))
}
