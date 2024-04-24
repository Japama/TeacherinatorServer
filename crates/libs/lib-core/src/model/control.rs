use chrono::{Local, Utc};
use modql::field::{Fields, HasFields};
use modql::filter::{ListOptions, OpValsValue};
use sea_query::{Condition, Expr, Iden, JoinType, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::Serialize;
use serde_with::serde_as;
use sqlx::encode::IsNull::No;
use sqlx::postgres::PgRow;
use sqlx::{Executor, FromRow};
use time::OffsetDateTime;
// use chrono::{Timelike};

use crate::ctx::Ctx;
use crate::model::base::PostgresDbBmc;
use crate::model::schedule_hour::{ScheduleHour, ScheduleHourBmc, ScheduleHourFilter};
use crate::model::teacher::{Teacher, TeacherBmc, TeacherFilter};
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
    pub async fn get_teachers_not_in_center_with_class(
        ctx: &Ctx,
        mm: &ModelManager,
    ) -> Result<i64> {
        // let db = mm.postgres_db();
        //
        // // -- Build query to get the current time
        // let current_time = Local::now().time();
        //
        // // -- Build query to get the teachers
        // let mut teachers_query = Query::select();
        // teachers_query
        //     .from(TeacherBmc::table_ref())
        //     .column(CustomIden("user_id"))
        //     .join(
        //         JoinType::InnerJoin,
        //         UserBmc::table_ref(),
        //         Condition::all().add(Expr::col(CustomIden("user_id")).equals(CustomIden("id")))
        //     )
        //     .and_where(Expr::col(CustomIden("in_center")).eq(false));
        //     // .join(
        //     //     JoinType::InnerJoin,
        //     //     ScheduleHourBmc::table_ref(),
        //     //     Condition::any().add(Expr::tbl(TeacherBmc::table_ref(), CustomIden("user_id")).equals(ScheduleHourBmc::table_ref(), CustomIden("schedule_id")))
        //     // )
        //     // .and_where(Expr::col(CustomIden("start_time")).lte(current_time))
        //     // .and_where(Expr::col(CustomIden("end_time")).gte(current_time))
        //     // .and_where(Expr::col(CustomIden("subject_name")).ne("Libre"));
        //
        // // -- Execute the query
        // let (raw_query, _) = teachers_query.build(PostgresQueryBuilder);
        // let teachers = sqlx::query(&raw_query).fetch_all(&db).await?;
        //
        //
        // Ok(teachers.len() as i64)
        Ok(1)
    }

    pub async fn update_guards(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<ScheduleHour>> {
        let teachers = TeacherBmc::list(&ctx, &mm, None, None).await?;
        let users = UserBmc::list(&ctx, &mm, None, None).await?;
        let now = match OffsetDateTime::now_local() {
            Ok(datetime) => datetime.time(),
            Err(e) => {
                eprintln!("Error obteniendo la hora local: {:?}", e);
                time::Time::MIDNIGHT
            }
        };
        let schedule_hours = ScheduleHourBmc::get_current_schedule_hours(&ctx, &mm).await?;
        let filtered_hours = schedule_hours
            .into_iter()
            // .filter(|hour| hour.start_time < now && hour.end_time > now)
            .collect::<Vec<ScheduleHour>>();

        Ok(filtered_hours)

        //
        // let db = mm.postgres_db();
        //
        // // -- Build query to get the current guard
        // let mut guard_query = Query::select();
        // guard_query
        //     .from(UserBmc::table_ref())
        //     .column(CustomIden("user_id"))
        //     .and_where(Expr::col(CustomIden("isadmin")).eq(true))
        //     .and_where(Expr::col(CustomIden("in_center")).eq(true));
        //
        // // -- Exec query
        // let (guard_sql, guard_values) = guard_query.build_sqlx(PostgresQueryBuilder);
        // let guard: (i64,) = sqlx::query_as_with(&guard_sql, guard_values).fetch_one(db).await?;
        //
        // // -- Build query to get the teachers who have class now
        // let mut teacher_query = Query::select();
        // teacher_query
        //     .from(TeacherBmc::table_ref())
        //     .column(CustomIden("user_id"))
        //     .and_where(Expr::col(CustomIden("active")).eq(true));
        //
        // // -- Exec query
        // let (teacher_sql, teacher_values) = teacher_query.build_sqlx(PostgresQueryBuilder);
        // let teachers: Vec<Teacher> = sqlx::query_as_with(&teacher_sql, teacher_values).fetch_all(db).await?;

        // -- Check if there is any teacher who is not present
        // let missing_teachers: Vec<i64> = teachers.into_iter().filter(|teacher| *teacher.user_id != guard.0).collect();
        //
        // // -- If there are missing teachers, assign the guard to their classes
        // // if !missing_teachers.is_empty() {
        // //     for teacher in missing_teachers {
        // //         // -- Build query to update the schedule
        // //         let mut update_query = Query::update();
        // //         update_query
        // //             .table("schedule_hours")
        // //             .values(vec![("user_id", guard.0.into())])
        // //             .and_where(Expr::col("user_id").eq(teacher));
        // //
        // //         // -- Exec query
        // //         let (update_sql, update_values) = update_query.build_sqlx(PostgresQueryBuilder);
        // //         sqlx::query_with(&update_sql, update_values).execute(db).await?;
        // //     }
        // // }

        // Ok(1)
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
    // use crate::model::control::{Control, ControlBmc, ControlForCreate, ControlForUpdate};

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
