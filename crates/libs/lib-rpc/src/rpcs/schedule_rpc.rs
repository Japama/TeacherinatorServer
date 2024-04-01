use lib_core::ctx::Ctx;
use lib_core::model::ModelManager;
use lib_core::model::schedule::{Schedule, ScheduleBmc, ScheduleFilter, ScheduleForCreate, ScheduleForUpdate};

use crate::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use crate::Result;
use crate::router::RpcRouter;
use crate::rpc_router;

pub fn rpc_router() -> RpcRouter {
    rpc_router!(
        // Same as RpcRouter::new().add...
        create_schedule,
        get_schedule,
        list_schedules,
        update_schedule,
        delete_schedule,
    )
}

pub async fn create_schedule(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<ScheduleForCreate>,
) -> Result<Schedule> {
    let ParamsForCreate { data } = params;
    let id = ScheduleBmc::create(&ctx, &mm, data.clone()).await?;
    let schedule = ScheduleBmc::get(&ctx, &mm, id).await?;

    Ok(schedule)
}

pub async fn get_schedule(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Schedule> {
    let ParamsIded { id } = params;

    let schedule = ScheduleBmc::get(&ctx, &mm, id).await?;

    Ok(schedule)
}

pub async fn list_schedules(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsList<ScheduleFilter>,
) -> Result<Vec<Schedule>> {
    let schedules = ScheduleBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

    Ok(schedules)
}

pub async fn update_schedule(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<ScheduleForUpdate>,
) -> Result<Schedule> {
    let ParamsForUpdate { id, data } = params;

    ScheduleBmc::update(&ctx, &mm, id, data).await?;

    let schedule = ScheduleBmc::get(&ctx, &mm, id).await?;

    Ok(schedule)
}

pub async fn delete_schedule(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Schedule> {
    let ParamsIded { id } = params;

    let schedule = ScheduleBmc::get(&ctx, &mm, id).await?;
    ScheduleBmc::delete(&ctx, &mm, id).await?;

    Ok(schedule)
}
