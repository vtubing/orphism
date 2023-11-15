use crate::model::Model;
use cap_std::fs::{Dir, File};
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug, thiserror::Error)]
#[remain::sorted]
pub enum RuntimeError {
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Json(#[from] serde_json::Error),
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

#[derive(Debug)]
pub struct Runtime {
  root: Dir,
  model: PathBuf,
}

impl Runtime {
  pub fn new_from_model_path(model: PathBuf) -> Result<Self, RuntimeError> {
    log::trace!("new_from_model_path({model:?}");

    if !model.is_file() {
      return Err(RuntimeError::ModelPathIsNotAFile(model));
    }

    match model.parent() {
      None => Err(RuntimeError::ModelHasNoParentDirectory(model)),
      Some(parent) => match model.file_name() {
        None => Err(RuntimeError::ModelPathMayNotEndWithTrailingDots(model)),
        Some(file_name) => {
          let root = Dir::from_std_file(std::fs::File::open(parent)?);
          let model = PathBuf::from(file_name);
          Ok(Self { model, root })
        }
      },
    }
  }

  pub fn new_from_runtime_path(root: PathBuf) -> Result<Self, RuntimeError> {
    log::trace!("new_from_runtime_path({root:?}");

    if !root.is_dir() {
      return Err(RuntimeError::RuntimePathIsNotADirectory(root));
    }

    let mut models = Vec::new();

    for entry in root.read_dir()? {
      let file_name = entry?.file_name();
      if file_name.to_string_lossy().ends_with(".model3.json") {
        let model = PathBuf::from(file_name);
        models.push(model);
      }
    }

    match models.len() {
      0 => Err(RuntimeError::RuntimePathContainsNoModels(root)),
      1 => {
        let model = root.join(models.swap_remove(0));
        Self::new_from_model_path(model)
      }
      _ => Err(RuntimeError::RuntimePathContainsMultipleModels(root)),
    }
  }

  pub fn open_dir<P>(&self, path: P) -> Result<Dir, RuntimeError>
  where
    P: AsRef<Path>,
  {
    Ok(self.root.open_dir(path)?)
  }

  pub fn open_file<P>(&self, path: P) -> Result<File, RuntimeError>
  where
    P: AsRef<Path>,
  {
    Ok(self.root.open(path)?)
  }

  pub fn read_bytes<P>(&self, path: P) -> Result<Vec<u8>, RuntimeError>
  where
    P: AsRef<Path>,
  {
    let mut file = self.open_file(path)?;
    let capacity = file.metadata()?.len().try_into().unwrap_or(0);
    let mut buf = Vec::with_capacity(capacity);
    file.read_to_end(&mut buf)?;
    buf.shrink_to_fit();
    Ok(buf)
  }

  pub fn read_json<P, T>(&self, path: P) -> Result<T, RuntimeError>
  where
    P: AsRef<Path>,
    T: serde::de::DeserializeOwned,
  {
    Ok(serde_json::from_slice(&self.read_bytes(path)?)?)
  }

  pub fn load(&self) -> Result<Model, RuntimeError> {
    self.read_json(self.model.as_path()).and_then(|model| Model::load(self, &model))
  }
}
