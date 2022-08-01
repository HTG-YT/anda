use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::error::ProjectError;

#[derive(Deserialize)]
pub struct AndaConfig {
    pub project: HashMap<String, Project>,
}

impl AndaConfig {
    pub fn find_key_for_value(&self, value: &Project) -> Option<&String> {
        self.project.iter().find_map(|(key, val)| {
            if val == value {
                Some(key)
            } else {
                None
            }
        })
    }
}

#[derive(Deserialize, PartialEq, Eq)]
pub struct Project {
    pub rpmbuild: Option<RpmBuild>,
    pub docker: Option<Docker>,
    pub pre_script: Option<PreScript>,
    pub script: Option<Script>,
    pub post_script: Option<PostScript>,
    pub rollback: Option<Script>,
}
#[derive(Deserialize, PartialEq, Eq)]
pub struct Script {
    pub stage: HashMap<String, Stage>,
}

impl Script {
    pub fn get_stage(&self, name: &str) -> Option<&Stage> {
        self.stage.get(name)
    }
    pub fn find_key_for_value(&self, value: &Stage) -> Option<&String> {
        self.stage.iter().find_map(|(key, val)| {
            if val == value {
                Some(key)
            } else {
                None
            }
        })
    }
}

#[derive(Deserialize, Eq, PartialEq, Hash)]
pub struct Stage {
    pub depends: Option<Vec<String>>,
    pub commands: Vec<String>,
}

#[derive(Deserialize, Eq, PartialEq, Hash)]
pub struct PreScript {
    pub commands: Vec<String>,
}

#[derive(Deserialize, Eq, PartialEq, Hash)]
pub struct PostScript {
    pub commands: Vec<String>,
}

#[derive(Deserialize, PartialEq, Eq)]
pub struct RpmBuild {
    pub spec: PathBuf,
}

#[derive(Deserialize, PartialEq, Eq)]
pub struct Docker {
    pub image: HashMap<String, DockerImage>, // tag, file
}

#[derive(Deserialize, PartialEq, Eq)]
pub struct DockerImage {
    pub workdir: PathBuf,
    pub version: Option<String>,
}

pub fn load_config(root: &PathBuf) -> Result<AndaConfig, ProjectError> {
    let config_path = root.join("anda.hcl");

    if !config_path.exists() {
        return Err(ProjectError::NoManifest);
    }

    let config: Result<AndaConfig, hcl::error::Error> = hcl::from_str(
        std::fs::read_to_string(config_path)
            .with_context(|| {
                format!(
                    "could not read `anda.toml` in directory {}",
                    fs::canonicalize(root).unwrap().display()
                )
            })?
            .as_str(),
    );

    config.map_err(ProjectError::HclError)
}
