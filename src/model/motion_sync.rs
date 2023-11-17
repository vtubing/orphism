use crate::{Result, Runtime};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct MotionSync {
  pub data: motionsync3::MotionSync3,
  pub file: PathBuf,
}

impl MotionSync {
  pub(super) fn load(runtime: &Runtime, file: PathBuf) -> Result<Self> {
    let data = runtime.read_json(&file)?;

    log::debug!("loaded: file={file:?}, data={data:?}");

    Ok(Self { data, file })
  }
}
