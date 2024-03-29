use lib_core::ctx::Ctx;
use lib_core::model::ModelManager;
use lib_core::model::specialty::{Specialty, SpecialtyBmc, SpecialtyFilter, SpecialtyForCreate, SpecialtyForUpdate};

use crate::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use crate::Result;
use crate::router::RpcRouter;
use crate::rpc_router;

pub fn rpc_router() -> RpcRouter {
    rpc_router!(
        // Same as RpcRouter::new().add...
        create_specialty,
        get_specialty,
        list_specialties,
        update_specialty,
        delete_specialty,
    )
}

pub async fn create_specialty(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<SpecialtyForCreate>,
) -> Result<Specialty> {
    let ParamsForCreate { data } = params;

    let id = SpecialtyBmc::create(&ctx, &mm, data.clone()).await?;
    
    let specialty = SpecialtyBmc::get(&ctx, &mm, id).await?;

    Ok(specialty)
}

pub async fn get_specialty(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Specialty> {
    let ParamsIded { id } = params;

    let specialty = SpecialtyBmc::get(&ctx, &mm, id).await?;

    Ok(specialty)
}

pub async fn list_specialties(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsList<SpecialtyFilter>,
) -> Result<Vec<Specialty>> {
    let specialties = SpecialtyBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

    Ok(specialties)
}

pub async fn update_specialty(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<SpecialtyForUpdate>,
) -> Result<Specialty> {
    let ParamsForUpdate { id, data } = params;

    SpecialtyBmc::update(&ctx, &mm, id, data).await?;

    let specialty = SpecialtyBmc::get(&ctx, &mm, id).await?;

    Ok(specialty)
}

pub async fn delete_specialty(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Specialty> {
    let ParamsIded { id } = params;

    let specialty = SpecialtyBmc::get(&ctx, &mm, id).await?;
    SpecialtyBmc::delete(&ctx, &mm, id).await?;

    Ok(specialty)
}
