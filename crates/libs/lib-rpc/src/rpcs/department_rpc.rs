use lib_core::ctx::Ctx;
use lib_core::model::ModelManager;
use lib_core::model::department::{Department, DepartmentBmc, DepartmentFilter, DepartmentForCreate, DepartmentForUpdate};

use crate::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use crate::Result;
use crate::router::RpcRouter;
use crate::rpc_router;

pub fn rpc_router() -> RpcRouter {
    rpc_router!(
        // Same as RpcRouter::new().add...
        create_department,
        get_department,
        list_departments,
        update_department,
        delete_department,
    )
}

pub async fn create_department(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<DepartmentForCreate>,
) -> Result<Department> {
    let ParamsForCreate { data } = params;

    let id = DepartmentBmc::create(&ctx, &mm, data.clone()).await?;
    
    let department = DepartmentBmc::get(&ctx, &mm, id).await?;

    Ok(department)
}

pub async fn get_department(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Department> {
    let ParamsIded { id } = params;

    let department = DepartmentBmc::get(&ctx, &mm, id).await?;

    Ok(department)
}

pub async fn list_departments(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsList<DepartmentFilter>,
) -> Result<Vec<Department>> {
    let departments = DepartmentBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

    Ok(departments)
}

pub async fn update_department(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<DepartmentForUpdate>,
) -> Result<Department> {
    let ParamsForUpdate { id, data } = params;

    DepartmentBmc::update(&ctx, &mm, id, data).await?;

    let department = DepartmentBmc::get(&ctx, &mm, id).await?;

    Ok(department)
}

pub async fn delete_department(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Department> {
    let ParamsIded { id } = params;

    let department = DepartmentBmc::get(&ctx, &mm, id).await?;
    DepartmentBmc::delete(&ctx, &mm, id).await?;

    Ok(department)
}
