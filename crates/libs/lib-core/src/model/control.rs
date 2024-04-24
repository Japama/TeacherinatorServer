use chrono::{ Timelike };
use modql::field::{Fields, HasFields};
use modql::filter::{ListOptions, OpValsInt64};
use sea_query::{Iden};
use sea_query_binder::SqlxBinder;
use serde::Serialize;
use serde_with::serde_as;
use sqlx::postgres::PgRow;
use sqlx::{Executor, FromRow};
use time::{OffsetDateTime, Time};

use crate::ctx::Ctx;
use crate::model::base::PostgresDbBmc;
use crate::model::center_schedule_hour::CenterScheduleHourBmc;
use crate::model::schedule_hour::{ScheduleHour, ScheduleHourBmc, ScheduleHourFilter};
use crate::model::teacher::{ TeacherBmc };
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

#[derive(Iden)]
enum ControlIden {
    Id,
    Controlname,
    Pwd,
}

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
    pub async fn update_guards(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<ScheduleHour>> {
        let teachers = TeacherBmc::list(&ctx, &mm, None, None).await?;
        let users = UserBmc::list(&ctx, &mm, None, None).await?;
        let center_schedule_hours = CenterScheduleHourBmc::list(&ctx, &mm, None, None).await?;
        let mut now: Time = OffsetDateTime::now_utc().time();
        // let hour = 12;
        // let minutes = 35;
        // now = now.replace_hour(hour).unwrap();
        // now = now.replace_minute(minutes).unwrap();
        let mut current_n_hour: i64 = 0;
        let mut current_week_day: i64 = 0;
        let course =  OffsetDateTime::now_utc().year() as i64;

         for schedule_hour in center_schedule_hours {
            if now >= schedule_hour.start_time && now <= schedule_hour.end_time {
                current_n_hour = schedule_hour.n_hour as i64;
                current_week_day = schedule_hour.week_day as i64;
                break;
            }
        }

        let filters = Some(vec![ScheduleHourFilter {
            id: None, schedule_id: None, classroom_name: None, subject_name: None,
            week_day: Some(OpValsInt64::from(current_n_hour)),
            course: Some(OpValsInt64::from(course)),
            n_hour: Some(OpValsInt64::from(current_n_hour)),
            cid: None, ctime: None, mid: None, mtime: None
        }]);

        let list_options = Some(ListOptions { limit: None, offset: None, order_bys: None });
        let schedule_hours = ScheduleHourBmc::list(&ctx, &mm, filters, list_options).await?;

        Ok(schedule_hours)
    }
}

// region:    --- Tests
#[cfg(test)]
mod tests {
    use anyhow::{Context, Result};
    use serde_json::json;
    use serial_test::serial;

    use crate::_dev_utils;
    use crate::ctx::Ctx;

    #[serial]
    #[tokio::test]
    async fn test_first_ok_admin() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_controlname = "admin";

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
