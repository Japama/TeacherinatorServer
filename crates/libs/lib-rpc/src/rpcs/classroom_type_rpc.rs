use lib_core::ctx::Ctx;
use lib_core::model::classroom_type::{ClassroomType, ClassroomTypeBmc, ClassroomTypeFilter, ClassroomTypeForCreate, ClassroomTypeForUpdate};
use lib_core::model::ModelManager;

use crate::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use crate::Result;
use crate::router::RpcRouter;
use crate::rpc_router;

pub fn rpc_router() -> RpcRouter {
    rpc_router!(
        create_classroom_type,
        get_classroom_type,
        list_classroom_types,
        update_classroom_type,
        delete_classroom_type,
    )
}

pub async fn create_classroom_type(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<ClassroomTypeForCreate>,
) -> Result<ClassroomType> {
    let ParamsForCreate { data } = params;

    let id = ClassroomTypeBmc::create(&ctx, &mm, data).await?;
    let classroom_type = ClassroomTypeBmc::get(&ctx, &mm, id).await?;

    Ok(classroom_type)
}

pub async fn get_classroom_type(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<ClassroomType> {
    let ParamsIded { id } = params;

    let classroom_type = ClassroomTypeBmc::get(&ctx, &mm, id).await?;

    Ok(classroom_type)
}

pub async fn list_classroom_types(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsList<ClassroomTypeFilter>,
) -> Result<Vec<ClassroomType>> {
    let classroom_types = ClassroomTypeBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

    Ok(classroom_types)
}

pub async fn update_classroom_type(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<ClassroomTypeForUpdate>,
) -> Result<ClassroomType> {
    let ParamsForUpdate { id, data } = params;

    ClassroomTypeBmc::update(&ctx, &mm, id, data).await?;

    let classroom_type = ClassroomTypeBmc::get(&ctx, &mm, id).await?;

    Ok(classroom_type)
}

pub async fn delete_classroom_type(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<ClassroomType> {
    let ParamsIded { id } = params;

    let classroom_type = ClassroomTypeBmc::get(&ctx, &mm, id).await?;
    ClassroomTypeBmc::delete(&ctx, &mm, id).await?;

    Ok(classroom_type)
}
