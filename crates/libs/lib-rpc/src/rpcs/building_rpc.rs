use lib_core::ctx::Ctx;
use lib_core::model::building::{Building, BuildingBmc, BuildingFilter, BuildingForCreate, BuildingForUpdate};
use lib_core::model::ModelManager;

use crate::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use crate::Result;
use crate::router::RpcRouter;
use crate::rpc_router;

pub fn rpc_router() -> RpcRouter {
    rpc_router!(
        create_building,
        get_building,
        list_buildings,
        update_building,
        delete_building,
    )
}

pub async fn create_building(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<BuildingForCreate>,
) -> Result<Building> {
    let ParamsForCreate { data } = params;

    let id = BuildingBmc::create(&ctx, &mm, data).await?;
    let building = BuildingBmc::get(&ctx, &mm, id).await?;

    Ok(building)
}

pub async fn get_building(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Building> {
    let ParamsIded { id } = params;

    let building = BuildingBmc::get(&ctx, &mm, id).await?;

    Ok(building)
}

pub async fn list_buildings(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsList<BuildingFilter>,
) -> Result<Vec<Building>> {
    let buildings = BuildingBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

    Ok(buildings)
}

pub async fn update_building(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<BuildingForUpdate>,
) -> Result<Building> {
    let ParamsForUpdate { id, data } = params;

    BuildingBmc::update(&ctx, &mm, id, data).await?;

    let building = BuildingBmc::get(&ctx, &mm, id).await?;

    Ok(building)
}

pub async fn delete_building(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Building> {
    let ParamsIded { id } = params;

    let building = BuildingBmc::get(&ctx, &mm, id).await?;
    BuildingBmc::delete(&ctx, &mm, id).await?;

    Ok(building)
}
