use modql::filter::{ListOptions, OpValsInt64};
use lib_core::ctx::Ctx;
use lib_core::model::ModelManager;
use lib_core::model::schedule::ScheduleBmc;
use lib_core::model::schedule_hour::{ScheduleHour, ScheduleHourBmc, ScheduleHourFilter, ScheduleHourForCreate, ScheduleHourForUpdate};

use crate::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use crate::Error::UserNotAdmin;
use crate::Result;
use crate::router::RpcRouter;
use crate::rpc_router;

pub fn rpc_router() -> RpcRouter {
    rpc_router!(
        // Same as RpcRouter::new().add...
        create_schedule_hour,
        get_schedule_hour,
        get_user_schedule_hours,
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
    if !&ctx.admin() { return Err(UserNotAdmin); }
    let ParamsForCreate { data } = params;
    let id = ScheduleHourBmc::create(&ctx, &mm, data.clone()).await?;
    let schedule_hour = ScheduleHourBmc::get(&ctx, &mm, id).await?;

    Ok(schedule_hour)
}

pub async fn get_schedule_hour(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<ScheduleHour> {
    if !&ctx.admin() { return Err(UserNotAdmin); }
    let ParamsIded { id } = params;
    let schedule_hour = ScheduleHourBmc::get(&ctx, &mm, id).await?;
    Ok(schedule_hour)
}
pub async fn get_user_schedule_hours(ctx: Ctx, mm: ModelManager) -> Result<Vec<ScheduleHour>>{
    let schedules = ScheduleBmc::get_teacher_schedule(&ctx, &mm, ctx.user_id()).await?;
    let schedule = schedules.first().unwrap().clone();

    let filters = Some(vec![ScheduleHourFilter {
        id: None,
        schedule_id: Some(OpValsInt64::from(schedule.id)),
        classroom_name: None, subject_name: None, week_day: None, course: None, n_hour: None, notes: None, cid: None, ctime: None, mid: None, mtime: None
    }]);

    let list_options = Some(ListOptions { limit: None, offset: None, order_bys: None });

    let schedule_hours = ScheduleHourBmc::list(&ctx, &mm, filters, list_options).await?;
    Ok(schedule_hours)
}

pub async fn list_schedule_hours(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsList<ScheduleHourFilter>,
) -> Result<Vec<ScheduleHour>> {
    if !&ctx.admin() { return Err(UserNotAdmin); }
    let schedule_hours = ScheduleHourBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

    Ok(schedule_hours)
}

pub async fn update_schedule_hour(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<ScheduleHourForUpdate>,
) -> Result<ScheduleHour> {
    if !&ctx.admin() { return Err(UserNotAdmin); }
    let ParamsForUpdate { id, data } = params;

    ScheduleHourBmc::update(&ctx, &mm, id, data).await?;

    let schedule_hour = ScheduleHourBmc::get(&ctx, &mm, id).await?;

    Ok(schedule_hour)
}

pub async fn delete_schedule_hour(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<ScheduleHour> {
    if !&ctx.admin() { return Err(UserNotAdmin); }
    let ParamsIded { id } = params;

    let schedule_hour = ScheduleHourBmc::get(&ctx, &mm, id).await?;
    ScheduleHourBmc::delete(&ctx, &mm, id).await?;

    Ok(schedule_hour)
}
