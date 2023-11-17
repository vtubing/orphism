# Changelog

## 0.2.0

- (FEATURE) All types now implement `Clone` (except `Runtime`).
- (FEATURE) `Model` now implements `PartialEq`.
- (FEATURE) Added enriched types that preserve information about the file they were loaded from.
  - `model::DisplayInfo`
  - `model::Expression`
  - `model::MotionSync`
  - `model::Motion`
  - `model::Physics`
  - `model::Pose`
  - `model::Sound`
  - `model::Texture`
- (FEATURE) `model::Sound` and `model::Texture` also perform media type detection when loaded.
  - this should make them easier to load correctly into other programs.
- (BREAKING) merged `Model` and `ModelData` into a single struct: `Model`.
  - `Model::load` now takes an **owned** `model3::Model3` as input.
  - `Model.moc` renamed to `Model.data`
  - `Model.display_info` type changed from `cdi3::DisplayInfo3` to `model::DisplayInfo`, previous data is at the `.data` field of that struct.
  - `Model.expression` type changed from `Vec<exp3::Expression3>` to `Vec<model::Expression>`, previous data is at the `.data` field of that struct.
  - `Model.motion_sync` type changed from `Option<motionsync3::MotionSync3>` to `Option<model::MotionSync>`, previous data is at the `.data` field of that struct.
  - `Model.motion` type changed from `BTreeMap<String, motion3::Motion3>` to `BTreeMap<String, model::Motion>`, previous data is at the `.data` field of that struct.
  - `Model.physics` type changed from `Option<physics3::Physics3>` to `Option<model::Physics>`, previous data is at the `.data` field of that struct.
  - `Model.pose` type changed from `Option<pose3::Pose3>` to `Option<model::Pose>`, previous data is at the `.data` field of that struct.
  - `Model.textures` type changed from `Vec<Vec<u8>>` to `Vec<model::Texture>`, previous data is at the `.data` field of that struct.
  - `model::Motion.sound` type changed from `Option<Vec<u8>>` to `Option<model::Sound>`, previous data is at the `.data` field of that struct.
- (BREAKING) Added three new error variants:
  - `Error::MediaTypeInferenceFailed` for when `infer` cannot detect the media type.
  - `Error::MediaType(mediatype::MediaTypeError)` for when `mediatype` cannot parse the inferred MIME type.
  - `Error::Moc3(moc3::Error)` to unify the error interface.
- (BREAKING) `Runtime::load()` renamed to `Runtime::load_model()` for clarity.

## 0.1.0 - Initial Release
