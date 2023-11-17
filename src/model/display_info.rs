use crate::{Result, Runtime};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct DisplayInfo {
  pub data: cdi3::DisplayInfo3,
  pub file: PathBuf,
}

impl DisplayInfo {
  pub(super) fn load(runtime: &Runtime, file: PathBuf) -> Result<Self> {
    let data = runtime.read_json(&file)?;

    log::debug!("loaded: file={file:?}, data={data:?}");

    Ok(Self { data, file })
  }
}
