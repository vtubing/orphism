use crate::{Result, Runtime};
use std::collections::BTreeMap;

mod display_info;
mod expression;
mod motion;
mod motion_sync;
mod physics;
mod pose;
mod sound;
mod texture;

pub use display_info::DisplayInfo;
pub use expression::Expression;
pub use motion::Motion;
pub use motion_sync::MotionSync;
pub use physics::Physics;
pub use pose::Pose;
pub use sound::Sound;
pub use texture::Texture;

#[derive(derivative::Derivative, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[derivative(Debug)]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Model {
  #[derivative(Debug = "ignore")]
  pub data: Vec<u8>,
  pub display_info: DisplayInfo,
  pub expressions: Vec<Expression>,
  pub groups: Vec<model3::Group>,
  pub hit_areas: Vec<model3::HitArea>,
  pub motion_sync: Option<MotionSync>,
  pub motions: BTreeMap<String, Vec<Motion>>,
  pub physics: Option<Physics>,
  pub pose: Option<Pose>,
  pub textures: Vec<Texture>,
  pub version: u8,
}

impl Model {
  pub fn load(runtime: &Runtime, model: model3::Model3) -> Result<Self> {
    let model3::Model3 {
      file_references: model3::FileReferences {
        display_info,
        expressions,
        moc,
        motion_sync,
        motions,
        physics,
        pose,
        textures,
      },
      groups,
      hit_areas,
      version,
    } = model;

    let display_info = DisplayInfo::load(runtime, display_info)?;

    let motion_sync = match motion_sync {
      Some(file) => Some(MotionSync::load(runtime, file)?),
      None => None,
    };

    let physics = match physics {
      Some(file) => Some(Physics::load(runtime, file)?),
      None => None,
    };

    let pose = match pose {
      Some(file) => Some(Pose::load(runtime, file)?),
      None => None,
    };

    let expressions = expressions.into_iter().map(|expression| Expression::load(runtime, expression)).collect::<Result<Vec<Expression>>>()?;

    let textures = textures.into_iter().map(|texture| Texture::load(runtime, texture)).collect::<Result<Vec<Texture>>>()?;

    let motions = {
      let mut map = BTreeMap::new();
      for (key, values) in motions {
        let mut value = Vec::new();
        for motion in values {
          let motion = Motion::load(runtime, motion)?;
          value.push(motion);
        }
        map.insert(key, value);
      }
      map
    };

    let data = runtime.read_bytes(moc)?;

    Ok(Self {
      data,
      display_info,
      expressions,
      groups,
      hit_areas,
      motion_sync,
      motions,
      physics,
      pose,
      textures,
      version,
    })
  }
}
