use crate::{Result, Runtime};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Physics {
  pub data: physics3::Physics3,
  pub file: PathBuf,
}

impl Physics {
  pub(super) fn load(runtime: &Runtime, file: PathBuf) -> Result<Self> {
    let data = runtime.read_json(&file)?;

    log::debug!("loaded: file={file:?}, data={data:?}");

    Ok(Self { data, file })
  }
}
