use lib_core::ctx::Ctx;
use lib_core::model::ModelManager;
use lib_core::model::teacher::{
    Teacher, TeacherBmc, TeacherFilter, TeacherForCreate, TeacherForUpdate,
};

use crate::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use crate::Result;
use crate::router::RpcRouter;
use crate::rpc_router;

pub fn rpc_router() -> RpcRouter {
    rpc_router!(
        // Same as RpcRouter::new().add...
        create_teacher,
        get_teacher,
        list_teachers,
        update_teacher,
        delete_teacher,
        teachers_by_department,
        count_teachers_by_department,
    )
}

pub async fn create_teacher(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<TeacherForCreate>,
) -> Result<Teacher> {
    let ParamsForCreate { data } = params;

    let id = TeacherBmc::create(&ctx, &mm, data.clone()).await?;

    let teacher = TeacherBmc::get(&ctx, &mm, id).await?;

    Ok(teacher)
}

pub async fn get_teacher(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Teacher> {
    let ParamsIded { id } = params;

    let teacher = TeacherBmc::get(&ctx, &mm, id).await?;

    Ok(teacher)
}

pub async fn list_teachers(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsList<TeacherFilter>,
) -> Result<Vec<Teacher>> {
    let teachers = TeacherBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

    Ok(teachers)
}

pub async fn update_teacher(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<TeacherForUpdate>,
) -> Result<Teacher> {
    let ParamsForUpdate { id, data } = params;

    TeacherBmc::update(&ctx, &mm, id, data).await?;

    let teacher = TeacherBmc::get(&ctx, &mm, id).await?;

    Ok(teacher)
}

pub async fn delete_teacher(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Teacher> {
    let ParamsIded { id } = params;

    let teacher = TeacherBmc::get(&ctx, &mm, id).await?;
    TeacherBmc::delete(&ctx, &mm, id).await?;

    Ok(teacher)
}

pub async fn count_teachers_by_department(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded,
) -> Result<i64> {
    let ParamsIded { id } = params;
    let teacher_number = TeacherBmc::count_teachers_by_department(&ctx, &mm, id).await?;
    Ok(teacher_number)
}

pub async fn teachers_by_department(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded,
) -> Result<Vec<Teacher>> {
    let ParamsIded { id } = params;
    let teacher_number = TeacherBmc::teachers_by_department(&ctx, &mm, id).await?;
    Ok(teacher_number)
}
