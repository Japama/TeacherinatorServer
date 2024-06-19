use lib_core::ctx::Ctx;
use lib_core::model::classroom::{Classroom, ClassroomBmc, ClassroomFilter, ClassroomForCreate, ClassroomForUpdate};
use lib_core::model::ModelManager;

use crate::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use crate::Result;
use crate::router::RpcRouter;
use crate::rpc_router;

pub fn rpc_router() -> RpcRouter {
    rpc_router!(
        // Same as RpcRouter::new().add...
        create_classroom,
        get_classroom,
        list_classrooms,
        update_classroom,
        delete_classroom,        
        count_classroom_by_classroom_type,
    )
}

pub async fn create_classroom(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<ClassroomForCreate>,
) -> Result<Classroom> {
    let ParamsForCreate { data } = params;

    let id = ClassroomBmc::create(&ctx, &mm, data).await?;
    let classroom = ClassroomBmc::get(&ctx, &mm, id).await?;

    Ok(classroom)
}

pub async fn get_classroom(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Classroom> {
    let ParamsIded { id } = params;

    let classroom = ClassroomBmc::get(&ctx, &mm, id).await?;

    Ok(classroom)
}

pub async fn list_classrooms(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsList<ClassroomFilter>,
) -> Result<Vec<Classroom>> {
    let classrooms = ClassroomBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

    Ok(classrooms)
}

pub async fn update_classroom(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<ClassroomForUpdate>,
) -> Result<Classroom> {
    let ParamsForUpdate { id, data } = params;

    ClassroomBmc::update(&ctx, &mm, id, data).await?;

    let classroom = ClassroomBmc::get(&ctx, &mm, id).await?;

    Ok(classroom)
}

pub async fn delete_classroom(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Classroom> {
    let ParamsIded { id } = params;

    let classroom = ClassroomBmc::get(&ctx, &mm, id).await?;
    ClassroomBmc::delete(&ctx, &mm, id).await?;

    Ok(classroom)
}

pub async fn count_classroom_by_classroom_type(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded,
) -> Result<i64> {
    let ParamsIded { id } = params;
    let classroom_number = ClassroomBmc::count_classrooms_by_classroom_type(&ctx, &mm, id).await?;
    Ok(classroom_number)
}