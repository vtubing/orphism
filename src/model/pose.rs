use crate::{Result, Runtime};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Pose {
  pub data: pose3::Pose3,
  pub file: PathBuf,
}

impl Pose {
  pub(super) fn load(runtime: &Runtime, file: PathBuf) -> Result<Self> {
    let data = runtime.read_json(&file)?;

    log::debug!("loaded: file={file:?}, data={data:?}");

    Ok(Self { data, file })
  }
}
