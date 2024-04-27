use chrono::{ Timelike };
use modql::field::{Fields, HasFields};
use modql::filter::{ListOptions, OpValsInt32};
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
use crate::model::schedule_hour::{ScheduleHourBmc, ScheduleHourFilter, ScheduleHourForUpdate};
use crate::model::teacher::{TeacherBmc};
use crate::model::user::UserBmc;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::schedule::ScheduleBmc;

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
    pub async fn update_guards(ctx: &Ctx, mm: &ModelManager) -> Result<()> {
        let teachers = TeacherBmc::list(&ctx, &mm, None, None).await?;
        let users = UserBmc::list(&ctx, &mm, None, None).await?;
        let schedules = ScheduleBmc::list(&ctx, &mm, None, None).await?;
        let center_schedule_hours = CenterScheduleHourBmc::list(&ctx, &mm, None, None).await?;
        let not_in_center_users = users.iter().filter( |user| !user.in_center);
        let mut not_in_center_teachers: Vec<i64> = vec![];
        for not_in_center_user in not_in_center_users {
            for teacher in teachers.clone() {
                if not_in_center_user.id == teacher.user_id{
                    not_in_center_teachers.insert(0, teacher.id);
                }
            }
        }

        let mut now: Time = OffsetDateTime::now_utc().time();
        let trying = false;
        if trying {
            let hour = 12;
            let minutes = 35;
            now = now.replace_hour(hour).unwrap();
            now = now.replace_minute(minutes).unwrap();
        }

        let mut current_n_hour: i32 = 0;
        let current_week_day: i32 = OffsetDateTime::now_utc().weekday() as i32;
        let course: i32 =  OffsetDateTime::now_utc().year();

         for schedule_hour in center_schedule_hours {
            if now >= schedule_hour.start_time && now <= schedule_hour.end_time && current_week_day == schedule_hour.week_day {
                current_n_hour = schedule_hour.n_hour;
                break;
            }
        }

        let filters = Some(vec![ScheduleHourFilter {
            id: None, schedule_id: None, classroom_name: None, subject_name: None,
            week_day: Some(OpValsInt32::from(current_week_day)),
            course: Some(OpValsInt32::from(course)),
            n_hour: Some(OpValsInt32::from(current_n_hour)),
            notes: None,
            cid: None, ctime: None, mid: None, mtime: None
        }]);
        
        let list_options = Some(ListOptions { limit: None, offset: None, order_bys: None });
        let current_schedule_hours = ScheduleHourBmc::list(&ctx, &mm, filters, list_options).await?;
        let temp_current_schedule_hours = current_schedule_hours.clone();
        let mut guard_hours = temp_current_schedule_hours.iter().filter(|current_schedule_hour| current_schedule_hour.subject_name == "Guardia");

        for not_in_center_teacher in not_in_center_teachers {
            let schedule = schedules.iter().find(|schedule| {
                if let Some(teacher_id) = schedule.teacher_id {
                    teacher_id == not_in_center_teacher
                } else {
                    false
                }
            });
            if let Some(schedule) = schedule{
                let schedule_hour = current_schedule_hours.iter().find(|hour| hour.schedule_id == schedule.id);
                if let Some(schedule_hour) = schedule_hour{
                    if schedule_hour.subject_name != "Libre" && schedule_hour.subject_name != "Guardia" {
                        let notes = Some(format!("Sustitución en {}", schedule_hour.classroom_name));
                        let guard_hour = guard_hours.next();
                        if let Some(guard_hour) = guard_hour {
                            let schedule_hour_u = ScheduleHourForUpdate {
                                schedule_id: guard_hour.schedule_id,
                                classroom_name: guard_hour.clone().classroom_name,
                                subject_name: guard_hour.clone().subject_name,
                                week_day: guard_hour.week_day,
                                n_hour: guard_hour.n_hour,
                                course: guard_hour.course,
                                notes,
                            };
                            ScheduleHourBmc::update(&ctx, &mm, guard_hour.id, schedule_hour_u).await?;
                        }
                    }
                }
            }
        }

        Ok(())
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
