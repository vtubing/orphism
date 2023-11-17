use crate::{Result, Runtime};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Expression {
  pub data: exp3::Expression3,
  pub file: PathBuf,
  pub name: String,
}

impl Expression {
  pub(super) fn load(runtime: &Runtime, expression: model3::Expression) -> Result<Self> {
    let model3::Expression { file, name } = expression;
    let data = runtime.read_json(&file)?;

    log::debug!("loaded: file={file:?}, name={name}, data={data:?}");

    Ok(Self { data, file, name })
  }
}
