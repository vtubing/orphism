use super::Sound;
use crate::{Result, Runtime};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Motion {
  pub data: motion3::Motion3,
  pub fade_in_time: Option<f64>,
  pub fade_out_time: Option<f64>,
  pub file: PathBuf,
  pub motion_sync: Option<String>,
  pub sound: Option<Sound>,
}

impl Motion {
  pub(super) fn load(runtime: &Runtime, motion: model3::Motion) -> Result<Self> {
    let model3::Motion {
      fade_in_time,
      fade_out_time,
      file,
      motion_sync,
      sound,
    } = motion;

    let data = runtime.read_json(&file)?;
    let sound = match sound {
      Some(path) => Some(Sound::load(runtime, path)?),
      None => None,
    };

    log::debug!("loaded: file={file:?}, motion_sync={}, sound={}", motion_sync.is_some(), sound.is_some());

    Ok(Self {
      data,
      fade_in_time,
      fade_out_time,
      file,
      motion_sync,
      sound,
    })
  }
}
