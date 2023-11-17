use crate::{Error, Result, Runtime};
use mediatype::MediaTypeBuf;
use std::path::PathBuf;

#[derive(derivative::Derivative, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[derivative(Debug)]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Texture {
  #[derivative(Debug = "ignore")]
  pub data: Vec<u8>,
  pub file: PathBuf,
  pub media_type: MediaTypeBuf,
}

impl Texture {
  pub(super) fn load(runtime: &Runtime, file: PathBuf) -> Result<Self> {
    let data = runtime.read_bytes(&file)?;
    let media_type = {
      let inferred = infer::get(data.as_slice()).ok_or(Error::MediaTypeInferenceFailed)?;
      MediaTypeBuf::from_string(inferred.mime_type().to_string())?.canonicalize()
    };

    log::debug!("loaded: type={}, file={file:?}, size={}", media_type.essence(), data.len());

    Ok(Self { data, file, media_type })
  }
}
