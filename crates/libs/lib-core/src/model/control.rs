use modql::field::{Fields, HasFields};
use modql::filter::{OpValInt32, OpValInt64, OpValString, OpValsInt32, OpValsInt64, OpValsString};
use sea_query::Iden;
use serde::Serialize;
use serde_with::serde_as;
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use time::{OffsetDateTime, Time};

use crate::ctx::Ctx;
use crate::model::base::PostgresDbBmc;
use crate::model::center_schedule_hour::CenterScheduleHourBmc;
use crate::model::schedule::{Schedule, ScheduleBmc, ScheduleFilter};
use crate::model::schedule_hour::{
    ScheduleHour, ScheduleHourBmc, ScheduleHourFilter, ScheduleHourForUpdate,
};
use crate::model::user::UserBmc;
use crate::model::ModelManager;
use crate::model::Result;

// region:    --- Control Types
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Control {
    pub id: i64,
}

/// Marker trait
pub trait ControlBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl ControlBy for Control {}

// #[derive(Iden)]
// enum ControlIden {
//     Id,
//     Controlname,
//     Pwd,
// }

// endregion: --- Control Types

pub struct ControlBmc;

impl PostgresDbBmc for ControlBmc {
    const TABLE: &'static str = "controls";
}

pub struct CustomIden(&'static str);

impl Iden for CustomIden {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "{}", self.0).unwrap();
    }
}

impl ControlBmc {
    pub async fn update_guards(ctx: &Ctx, mm: &ModelManager) -> Result<()> {
        let now: Time = get_current_time(true, 12, 35);
        let current_week_day: i32 = OffsetDateTime::now_utc().weekday() as i32;
        let course = OffsetDateTime::now_utc().year();

        let center_schedule_hours = CenterScheduleHourBmc::list(&ctx, &mm, None, None).await?;
        let current_n_hour = center_schedule_hours
            .iter()
            .find(|schedule_hour| now >= schedule_hour.start_time && now <= schedule_hour.end_time)
            .map_or(0, |schedule_hour| schedule_hour.n_hour);

        let users = UserBmc::list(&ctx, &mm, None, None).await?;
        let not_in_center_users: Vec<_> = users.iter().filter(|user| !user.in_center).collect();
        let not_in_center_user_ids: Vec<_> =
            not_in_center_users.iter().map(|user| user.id).collect();

        let schedules = ScheduleBmc::list(
            &ctx,
            &mm,
            current_schedule_filters(not_in_center_user_ids, course as i64),
            None,
        )
        .await?;
        let not_in_center_user_schedules_ids: Vec<_> =
            schedules.iter().map(|schedule| schedule.id).collect();

        let current_schedule_hours = ScheduleHourBmc::list(
            &ctx,
            &mm,
            current_hour_filters(current_week_day, course, current_n_hour),
            None,
        )
        .await?;
        let mut current_guard_hours = ScheduleHourBmc::list(
            &ctx,
            &mm,
            guard_filters(
                not_in_center_user_schedules_ids,
                current_week_day,
                course,
                current_n_hour,
            ),
            None,
        )
        .await?
        .into_iter();

        let not_in_center_users_ids: Vec<i64> =
            not_in_center_users.iter().map(|user| user.id).collect();
        let non_cover_schedules: Vec<&Schedule> = schedules
            .iter()
            .filter(|schedule| {
                if let Some(user_id) = schedule.user_id {
                    not_in_center_users_ids.contains(&user_id)
                } else {
                    false
                }
            })
            .collect();

        let non_cover_schedule_ids: Vec<i64> = non_cover_schedules
            .iter()
            .map(|schedule| schedule.id)
            .collect();
        let relevant_schedule_hours: Vec<&ScheduleHour> = current_schedule_hours
            .iter()
            .filter(|schedule_hour| {
                non_cover_schedule_ids.contains(&schedule_hour.schedule_id)
                    && schedule_hour.subject_name != "Libre"
                    && schedule_hour.subject_name != "Guardia"
                    && schedule_hour.notes != Some("Cubierta".to_string())
            })
            .collect();

        for schedule_hour in relevant_schedule_hours {
            if let Some(guard_hour) = current_guard_hours.next() {
                let notes = Some(format!("Sustitución en {}", schedule_hour.classroom_name));
                let notes2 = Some("Cubierta".to_string());

                update_schedule_hour(&ctx, &mm, &guard_hour, notes).await?;
                update_schedule_hour(&ctx, &mm, schedule_hour, notes2).await?;
            }
        }

        Ok(())
    }
}

fn get_current_time(testing: bool, hour: u8, minute: u8) -> Time {
    let mut now: Time = OffsetDateTime::now_utc().time();
    if testing {
        now = now.replace_hour(hour).unwrap();
        now = now.replace_minute(minute).unwrap();
    }
    now
}

async fn update_schedule_hour(
    ctx: &Ctx,
    mm: &ModelManager,
    schedule_hour: &ScheduleHour,
    notes: Option<String>,
) -> Result<()> {
    let schedule_hour_u = ScheduleHourForUpdate {
        schedule_id: schedule_hour.schedule_id,
        classroom_name: schedule_hour.classroom_name.to_string(),
        subject_name: schedule_hour.subject_name.to_string(),
        week_day: schedule_hour.week_day,
        n_hour: schedule_hour.n_hour,
        course: schedule_hour.course,
        notes,
    };
    ScheduleHourBmc::update(&ctx, &mm, schedule_hour.id, schedule_hour_u).await
}

fn current_schedule_filters(
    not_in_center_user_ids: Vec<i64>,
    course: i64,
) -> Option<Vec<ScheduleFilter>> {
    Some(vec![ScheduleFilter {
        id: None,
        user_id: Some(OpValsInt64(vec![OpValInt64::In(not_in_center_user_ids)])),
        group_id: None,
        course: Some(OpValsInt64(vec![OpValInt64::Eq(course)])),
        cid: None,
        ctime: None,
        mid: None,
        mtime: None,
    }])
}

fn current_hour_filters(
    current_week_day: i32,
    course: i32,
    current_n_hour: i32,
) -> Option<Vec<ScheduleHourFilter>> {
    Some(vec![ScheduleHourFilter {
        id: None,
        schedule_id: None,
        classroom_name: None,
        subject_name: None,
        week_day: Some(OpValsInt32(vec![OpValInt32::Eq(current_week_day)])),
        course: Some(OpValsInt32(vec![OpValInt32::Eq(course)])),
        n_hour: Some(OpValsInt32(vec![OpValInt32::Eq(current_n_hour)])),
        notes: None,
        cid: None,
        ctime: None,
        mid: None,
        mtime: None,
    }])
}

fn guard_filters(
    not_in_center_user_schedules_ids: Vec<i64>,
    current_week_day: i32,
    course: i32,
    current_n_hour: i32,
) -> Option<Vec<ScheduleHourFilter>> {
    Some(vec![ScheduleHourFilter {
        id: None,
        schedule_id: Some(OpValsInt64(vec![OpValInt64::NotIn(
            not_in_center_user_schedules_ids,
        )])),
        classroom_name: None,
        subject_name: Some(OpValsString(vec![OpValString::Eq("Guardia".to_string())])),
        week_day: Some(OpValsInt32(vec![OpValInt32::Eq(current_week_day)])),
        course: Some(OpValsInt32(vec![OpValInt32::Eq(course)])),
        n_hour: Some(OpValsInt32(vec![OpValInt32::Eq(current_n_hour)])),
        notes: Some(OpValsString(vec![OpValString::Null(true)])),
        cid: None,
        ctime: None,
        mid: None,
        mtime: None,
    }])
}

// region:    --- Tests
#[cfg(test)]
mod tests {
    // use anyhow::{Context, Result};
    use anyhow::Result;
    // use serde_json::json;
    use serial_test::serial;

    // use crate::_dev_utils;
    // use crate::ctx::Ctx;

    #[serial]
    #[tokio::test]
    async fn test_first_ok_admin() -> Result<()> {
        // -- Setup & Fixtures
        // let mm = _dev_utils::init_test().await;
        // let ctx = Ctx::root_ctx();
        // let fx_controlname = "admin";

        // -- Exec
        // let control: Control = ControlBmc::first_by_controlname(&ctx, &mm, fx_controlname)
        //     .await?
        //     .context("Should have control 'admin'")?;

        // -- Check
        // assert_eq!(control.controlname, fx_controlname);

        Ok(())
    }
}
// endregion: --- Tests
