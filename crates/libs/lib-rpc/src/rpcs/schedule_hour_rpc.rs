use lib_core::ctx::Ctx;
use lib_core::model::ModelManager;
use lib_core::model::schedule_hour::{ScheduleHour, ScheduleHourBmc, ScheduleHourFilter, ScheduleHourForCreate, ScheduleHourForUpdate};

use crate::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use crate::Result;
use crate::router::RpcRouter;
use crate::rpc_router;

pub fn rpc_router() -> RpcRouter {
    rpc_router!(
        // Same as RpcRouter::new().add...
        create_schedule_hour,
        get_schedule_hour,
        list_schedule_hours,
        update_schedule_hour,
        delete_schedule_hour,
    )
}

pub async fn create_schedule_hour(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<ScheduleHourForCreate>,
) -> Result<ScheduleHour> {
    let ParamsForCreate { data } = params;
    let id = ScheduleHourBmc::create(&ctx, &mm, data.clone()).await?;
    let schedule_hour = ScheduleHourBmc::get(&ctx, &mm, id).await?;

    Ok(schedule_hour)
}

pub async fn get_schedule_hour(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<ScheduleHour> {
    let ParamsIded { id } = params;
    let schedule_hour = ScheduleHourBmc::get(&ctx, &mm, id).await?;
    Ok(schedule_hour)
}

pub async fn list_schedule_hours(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsList<ScheduleHourFilter>,
) -> Result<Vec<ScheduleHour>> {
    let schedule_hours = ScheduleHourBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

    Ok(schedule_hours)
}

pub async fn update_schedule_hour(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<ScheduleHourForUpdate>,
) -> Result<ScheduleHour> {
    let ParamsForUpdate { id, data } = params;

    ScheduleHourBmc::update(&ctx, &mm, id, data).await?;

    let schedule_hour = ScheduleHourBmc::get(&ctx, &mm, id).await?;

    Ok(schedule_hour)
}

pub async fn delete_schedule_hour(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<ScheduleHour> {
    let ParamsIded { id } = params;

    let schedule_hour = ScheduleHourBmc::get(&ctx, &mm, id).await?;
    ScheduleHourBmc::delete(&ctx, &mm, id).await?;

    Ok(schedule_hour)
}
