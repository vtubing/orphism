use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
#[remain::sorted]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Json(#[from] serde_json::Error),
  #[error(transparent)]
  MediaType(#[from] mediatype::MediaTypeError),
  #[error("media type inference failed")]
  MediaTypeInferenceFailed,
  #[error(transparent)]
  Moc3(#[from] moc3::Error),
  #[error("model has no parent directory: {0:?}")]
  ModelHasNoParentDirectory(PathBuf),
  #[error("model path is not a file: {0:?}")]
  ModelPathIsNotAFile(PathBuf),
  #[error("model path may not end with '..': {0:?}")]
  ModelPathMayNotEndWithTrailingDots(PathBuf),
  #[error("runtime path contains multiple models: {0:?}")]
  RuntimePathContainsMultipleModels(PathBuf),
  #[error("runtime path contains no models: {0:?}")]
  RuntimePathContainsNoModels(PathBuf),
  #[error("runtime path is not a directory: {0:?}")]
  RuntimePathIsNotADirectory(PathBuf),
}
