use lib_core::ctx::Ctx;
use lib_core::model::ModelManager;
use lib_core::model::group::{Group, GroupBmc, GroupFilter, GroupForCreate, GroupForUpdate};

use crate::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use crate::Result;
use crate::router::RpcRouter;
use crate::rpc_router;

pub fn rpc_router() -> RpcRouter {
    rpc_router!(
        // Same as RpcRouter::new().add...
        create_group,
        get_group,
        list_groups,
        update_group,
        delete_group,
    )
}

pub async fn create_group(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<GroupForCreate>,
) -> Result<Group> {
    let ParamsForCreate { data } = params;

    let id = GroupBmc::create(&ctx, &mm, data).await?;
    let group = GroupBmc::get(&ctx, &mm, id).await?;

    Ok(group)
}

pub async fn get_group(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Group> {
    let ParamsIded { id } = params;

    let group = GroupBmc::get(&ctx, &mm, id).await?;

    Ok(group)
}

pub async fn list_groups(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsList<GroupFilter>,
) -> Result<Vec<Group>> {
    let groups = GroupBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

    Ok(groups)
}

pub async fn update_group(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<GroupForUpdate>,
) -> Result<Group> {
    let ParamsForUpdate { id, data } = params;

    GroupBmc::update(&ctx, &mm, id, data).await?;

    let group = GroupBmc::get(&ctx, &mm, id).await?;

    Ok(group)
}

pub async fn delete_group(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Group> {
    let ParamsIded { id } = params;

    let group = GroupBmc::get(&ctx, &mm, id).await?;
    GroupBmc::delete(&ctx, &mm, id).await?;

    Ok(group)
}
