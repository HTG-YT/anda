//! Andaman client error handler

// derive macro that implements the From<anyhow::Error> trait

#[derive(Debug)]
pub enum ProjectError {
    NoManifest,
    InvalidManifest(String),
    Multiple(Vec<Self>),
    HclError(hcl::error::Error),
    Other(String),
}

impl From<hcl::error::Error> for ProjectError {
    fn from(e: hcl::error::Error) -> Self {
        ProjectError::HclError(e)
    }
}

impl std::fmt::Display for ProjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectError::NoManifest => write!(f, "No manifest found"),
            ProjectError::InvalidManifest(e) => write!(f, "Invalid manifest: {}", e),
            ProjectError::Other(msg) => write!(f, "{}", msg),
            ProjectError::HclError(e) => write!(
                f,
                "Error parsing HCL: {e}{}",
                e.location().map(|l| format!(" at {}:{}", l.line, l.col)).unwrap_or_default()
            ),
            ProjectError::Multiple(errors) => {
                write!(f, "Multiple errors:")?;
                for error in errors {
                    write!(f, "\n - {error}")?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for ProjectError {}
