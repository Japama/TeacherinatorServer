use lib_core::ctx::Ctx;
use lib_core::model::ModelManager;

use crate::{Error, Result};
use crate::router::FromResources;

pub struct RpcResources {
    pub mm: ModelManager,
    pub ctx: Option<Ctx>,
}

impl FromResources for Ctx {
    fn from_resources(rpc_resources: &RpcResources) -> Result<Self> {
        rpc_resources.ctx.as_ref().cloned().ok_or(Error::MissingCtx)
    }
}

impl FromResources for Option<Ctx> {
    fn from_resources(rpc_resources: &RpcResources) -> Result<Self> {
        Ok(rpc_resources.ctx.as_ref().cloned())
    }
}

impl FromResources for ModelManager {
    fn from_resources(rpc_resources: &RpcResources) -> Result<Self> {
        Ok(rpc_resources.mm.clone())
    }
}
