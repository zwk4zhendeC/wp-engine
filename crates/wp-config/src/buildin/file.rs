use educe::Educe;
use orion_conf::{
    ToStructError,
    error::{ConfIOReason, OrionConfResult},
};
use orion_error::UvsValidationFrom;
use std::path::Path;

#[derive(Educe, Deserialize, Serialize, PartialEq, Clone)]
#[educe(Debug, Default)]
pub struct FileSinkConf {
    #[educe(Default = "./out.dat")]
    pub path: String,
}

impl FileSinkConf {
    pub fn new<S: Into<String>>(path: S) -> Self {
        Self { path: path.into() }
    }

    pub fn new_path<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().display().to_string(),
        }
    }
    pub fn use_cli(&mut self, cli_path: Option<String>) {
        if let Some(cli_path) = cli_path {
            self.path = cli_path;
        }
    }
}

impl crate::structure::Validate for FileSinkConf {
    fn validate(&self) -> OrionConfResult<()> {
        if self.path.trim().is_empty() {
            return ConfIOReason::from_validation("out_file.path must not be empty").err_result();
        }
        let p = std::path::Path::new(&self.path);
        if let Some(parent) = p.parent()
            && !parent.as_os_str().is_empty()
            && !parent.exists()
        {
            std::fs::create_dir_all(parent).map_err(|e| {
                ConfIOReason::from_validation(format!(
                    "create parent dir failed: {:?}, err={}",
                    parent, e
                ))
                .to_err()
            })?;
        }
        Ok(())
    }
}
