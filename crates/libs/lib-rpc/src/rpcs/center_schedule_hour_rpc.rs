use serde_json::json;
use lib_core::ctx::Ctx;
use lib_core::model::ModelManager;
use lib_core::model::center_schedule_hour::{CenterScheduleHour, CenterScheduleHourBmc, CenterScheduleHourFilter, CenterScheduleHourForCheck, CenterScheduleHourForCreate, CenterScheduleHourForUpdate};

use crate::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use crate::Error::UserNotAdmin;
use crate::Result;
use crate::router::RpcRouter;
use crate::rpc_router;

pub fn rpc_router() -> RpcRouter {
    rpc_router!(
        // Same as RpcRouter::new().add...
        create_center_schedule_hour,
        get_center_schedule_hour,
        list_center_schedule_hours,
        update_center_schedule_hour,
        delete_center_schedule_hour,
        check_hour_exists,
    )
}

pub async fn create_center_schedule_hour(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<CenterScheduleHourForCreate>,
) -> Result<CenterScheduleHour> {
    if !&ctx.admin() { return Err(UserNotAdmin); }
    let ParamsForCreate { data } = params;
    let id = CenterScheduleHourBmc::create(&ctx, &mm, data.clone()).await?;
    let center_schedule_hour = CenterScheduleHourBmc::get(&ctx, &mm, id).await?;

    Ok(center_schedule_hour)
}

pub async fn get_center_schedule_hour(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<CenterScheduleHour> {
    if !&ctx.admin() { return Err(UserNotAdmin); }
    let ParamsIded { id } = params;
    let center_schedule_hour = CenterScheduleHourBmc::get(&ctx, &mm, id).await?;
    Ok(center_schedule_hour)
}


pub async fn list_center_schedule_hours(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsList<CenterScheduleHourFilter>,
) -> Result<Vec<CenterScheduleHour>> {
    // if !&ctx.admin() { return Err(UserNotAdmin); }
    let center_schedule_hours = CenterScheduleHourBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

    Ok(center_schedule_hours)
}

pub async fn update_center_schedule_hour(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<CenterScheduleHourForUpdate>,
) -> Result<CenterScheduleHour> {
    if !&ctx.admin() { return Err(UserNotAdmin); }
    let ParamsForUpdate { id, data } = params;

    CenterScheduleHourBmc::update(&ctx, &mm, id, data).await?;

    let center_schedule_hour = CenterScheduleHourBmc::get(&ctx, &mm, id).await?;

    Ok(center_schedule_hour)
}

pub async fn delete_center_schedule_hour(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<CenterScheduleHour> {
    if !&ctx.admin() { return Err(UserNotAdmin); }
    let ParamsIded { id } = params;

    let center_schedule_hour = CenterScheduleHourBmc::get(&ctx, &mm, id).await?;
    CenterScheduleHourBmc::delete(&ctx, &mm, id).await?;

    Ok(center_schedule_hour)
}

pub async fn check_hour_exists(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<CenterScheduleHourForCheck>,
) -> Result<bool> {
    if !&ctx.admin() {
        return Err(UserNotAdmin);
    }
    let ParamsForCreate { data } = params;

    let filter_json = json!({
        "n_hour": {"$eq": data.n_hour },
    });
    let filter = vec![serde_json::from_value(filter_json)?];

    let hours = CenterScheduleHourBmc::list(&ctx, &mm, Some(filter), None).await?;

    Ok(hours.len() > 0)
}
