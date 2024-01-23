// region:    --- Modules

pub use params::*;
pub use resources::RpcResources;
pub use router::RpcRequest;
pub use rpcs::*;

pub use self::error::{Error, Result};

mod error;
mod params;
mod params_mongo;
mod resources;
pub mod router;
mod rpcs;

// endregion: --- Modules
