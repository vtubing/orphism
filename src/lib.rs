mod model;
mod runtime;

pub use model::Model;
pub use runtime::{Runtime, RuntimeError};

pub use cdi3;
pub use exp3;
pub use moc3;
pub use model3;
pub use motion3;
pub use physics3;
pub use pose3;
pub use userdata3;

// pub mod moc3 {
//   pub use ::moc3::{CanvasFlags, CanvasInfo, Error, Header, Model, Result, Version};
//   mod v3_00_00 {
//     pub use ::moc3::v3_00_00::{CountInfoTable, Model, SectionOffsetTable};
//   }

//   mod v3_03_00 {
//     pub use ::moc3::v3_03_00::{CountInfoTable, Model, SectionOffsetTable};
//   }

//   mod v4_00_00 {
//     pub use ::moc3::v4_00_00::{CountInfoTable, Model, SectionOffsetTable};
//   }

//   mod v4_02_00 {
//     pub use ::moc3::v4_02_00::{CountInfoTable, Model, SectionOffsetTable};
//   }

//   mod v5_00_00 {
//     pub use ::moc3::v5_00_00::{CountInfoTable, Model, SectionOffsetTable};
//   }
// }
