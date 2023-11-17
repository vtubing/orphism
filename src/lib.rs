mod error;
pub mod model;
mod runtime;

pub use error::Error;
pub use model::Model;
pub use runtime::Runtime;

pub type Result<T> = std::result::Result<T, Error>;

pub use cdi3;
pub use exp3;
pub use moc3;
pub use model3;
pub use motion3;
pub use physics3;
pub use pose3;
pub use userdata3;
