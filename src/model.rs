use crate::runtime::{Runtime, RuntimeError};
use std::collections::BTreeMap;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Model {
  pub data: ModelData,
  pub groups: Vec<model3::Group>,
  pub hit_areas: Vec<model3::HitArea>,
  pub version: u8,
}

impl Model {
  pub fn load(runtime: &Runtime, model: &model3::Model3) -> Result<Self, RuntimeError> {
    let data = ModelData::load(runtime, &model.file_references)?;
    let groups = model.groups.clone();
    let hit_areas = model.hit_areas.clone();
    let version = model.version;

    Ok(Self { data, groups, hit_areas, version })
  }
}

#[derive(derivative::Derivative, serde::Serialize, serde::Deserialize)]
#[derivative(Debug)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct ModelData {
  pub display_info: cdi3::DisplayInfo3,
  #[serde(default)]
  pub expressions: Vec<Expression>,
  #[derivative(Debug = "ignore")]
  pub moc: Vec<u8>,
  pub motion_sync: Option<motionsync3::MotionSync3>,
  pub motions: std::collections::BTreeMap<String, Vec<Motion>>,
  pub physics: Option<physics3::Physics3>,
  pub pose: Option<pose3::Pose3>,
  #[derivative(Debug = "ignore")]
  pub textures: Vec<Vec<u8>>,
}

impl ModelData {
  pub fn load(runtime: &Runtime, file_references: &model3::FileReferences) -> Result<Self, RuntimeError> {
    let display_info = runtime.read_json(&file_references.display_info)?;

    let expressions = {
      let mut expressions = Vec::new();
      for expression in &file_references.expressions {
        let expression = Expression::load(runtime, expression)?;
        expressions.push(expression);
      }
      expressions
    };

    let moc = runtime.read_bytes(&file_references.moc)?;

    let motion_sync = match &file_references.motion_sync {
      None => None,
      Some(path) => Some(runtime.read_json(path)?),
    };

    let motions = {
      let mut motions = BTreeMap::new();
      for (key, values) in &file_references.motions {
        let mut value = Vec::new();
        for motion in values {
          let motion = Motion::load(runtime, motion)?;
          value.push(motion);
        }
        let key = key.to_owned();
        motions.insert(key, value);
      }
      motions
    };

    let physics = match &file_references.physics {
      None => None,
      Some(path) => Some(runtime.read_json(path)?),
    };

    let pose = match &file_references.pose {
      None => None,
      Some(path) => Some(runtime.read_json(path)?),
    };

    let textures = {
      let mut textures = Vec::new();
      for texture in &file_references.textures {
        let texture = runtime.read_bytes(texture)?;
        textures.push(texture);
      }
      textures
    };

    Ok(Self {
      display_info,
      expressions,
      moc,
      motion_sync,
      motions,
      physics,
      pose,
      textures,
    })
  }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Expression {
  pub data: exp3::Expression3,
  pub name: String,
}

impl Expression {
  fn load(runtime: &Runtime, expression: &model3::Expression) -> Result<Self, RuntimeError> {
    let data = runtime.read_json(&expression.file)?;
    let name = expression.name.to_owned();
    Ok(Self { data, name })
  }
}

#[derive(derivative::Derivative, serde::Serialize, serde::Deserialize)]
#[derivative(Debug)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Motion {
  pub data: motion3::Motion3,
  pub motion_sync: Option<String>,
  #[derivative(Debug = "ignore")]
  pub sound: Option<Vec<u8>>,
}

impl Motion {
  fn load(runtime: &Runtime, motion: &model3::Motion) -> Result<Self, RuntimeError> {
    let data = runtime.read_json(&motion.file)?;
    let motion_sync = motion.motion_sync.to_owned();
    let sound = match &motion.sound {
      None => None,
      Some(path) => Some(runtime.read_bytes(path)?),
    };
    Ok(Self { data, motion_sync, sound })
  }
}
