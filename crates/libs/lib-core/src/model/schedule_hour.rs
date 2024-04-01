use modql::field::{Fields, HasFields};
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsValue};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use sqlx::postgres::PgRow;
use time::Time;
use crate::model::modql_utils::time_to_sea_value;
use crate::ctx::Ctx;
use crate::model::base::{self, PostgresDbBmc};
use crate::model::ModelManager;
use crate::model::Result;

// region:    --- ScheduleHour Types


#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct ScheduleHour {
    pub id: i64,
    pub schedule_id: i64,
    pub subject_id: i64,
    pub week_day: i32,
    pub n_hour: i32,
    pub start_time: Time,
    pub end_time: Time,
    pub course: i32    
}

#[derive(Fields, Deserialize, Clone)]
pub struct ScheduleHourForCreate {
    pub schedule_id: i64,
    pub subject_id: i64,
    pub week_day: i32,
    pub n_hour: i32,
    pub start_time: Time,
    pub end_time: Time,
    pub course: i32
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ScheduleHourFilter {
    id: Option<OpValsInt64>,

    subject_id: Option<OpValsInt64>,
    week_day: Option<OpValsInt64>,
    n_hour: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    start_time: Option<OpValsValue>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    end_time: Option<OpValsValue>,
    course: Option<OpValsInt64>,

    cid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    ctime: Option<OpValsValue>,
    mid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    mtime: Option<OpValsValue>,
}

impl Default for ScheduleHourForUpdate {
    fn default() -> Self {
        Self {
            schedule_id: 0,
            subject_id: 0, // default value
            week_day: 0,   // default value
            n_hour: 0,     // default value
            start_time: Time::MIDNIGHT  /* provide a value */,
            end_time: Time::MIDNIGHT    /* provide a value */,
            course: 0,     // default value
        }
    }
}

#[derive(Fields, Deserialize)]
pub struct ScheduleHourForUpdate {
    pub schedule_id: i64,
    pub subject_id: i64,
    pub week_day: i32,
    pub n_hour: i32,
    pub start_time: Time,
    pub end_time: Time,
    pub course: i32
}

/// Marker trait
pub trait ScheduleHourBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}
impl ScheduleHourBy for ScheduleHour {}

// endregion: --- ScheduleHour Types

pub struct ScheduleHourBmc;

impl PostgresDbBmc for ScheduleHourBmc {
    const TABLE: &'static str = "schedule_hours";
}

impl ScheduleHourBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, schedule_hour_c: ScheduleHourForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, schedule_hour_c).await
    }

    pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
    where
        E: ScheduleHourBy,
    {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<ScheduleHourFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<ScheduleHour>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }


    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        schedule_hour_u: ScheduleHourForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, schedule_hour_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}

// region:    --- Tests
#[cfg(test)]
mod tests {
    use std::iter::Map;
    use std::ptr::null;
    use anyhow::{Result};
    use bson::Bson::Null;
    use futures::TryFutureExt;
    use serde_json::json;
    use serial_test::serial;
    use time::Time;
    use crate::_dev_utils;
    use crate::_dev_utils::{seed_department, seed_subject, seed_teacher, seed_user};
    use crate::ctx::Ctx;
    use crate::model::department::DepartmentBmc;
    use crate::model::schedule_hour::{ScheduleHour, ScheduleHourBmc, ScheduleHourForCreate, ScheduleHourForUpdate};
    use crate::model::teacher::TeacherBmc;
    use crate::model::user::UserBmc;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_subject_name = "subject_create_ok";
        let fx_department_name = "department_create_ok";
        let fx_department_id = seed_department(&ctx, &mm, fx_department_name).await?;
        let fx_subject_id = seed_subject(&ctx, &mm, fx_subject_name, fx_department_id, false, false).await?;
        let fx_week_day = 1; // lunes
        
        let fx_course = 2024;
        let fx_username = "Prueba_schedule_hour_create_ok";
        let fx_teacher_name = "Profe_schedule_hour_create_ok";
        let fx_department_name = "Departamento_schedule_hour_create_ok";
        
        let fx_user_id = seed_user(&ctx, &mm, fx_username).await?;
        let fx_department_id= seed_department(&ctx, &mm, fx_department_name).await?;
        let fx_teacher_id = Some(_dev_utils::seed_teacher(&ctx, &mm, fx_teacher_name, fx_department_id ,fx_user_id).await?);

        // -- Exec
        let schedule_hour_c = ScheduleHourForCreate {
            subject_id: fx_subject_id,
            week_day: i32,
            n_hour: i32,
            start_time: Time,
            end_time: Time,
            course: i32
        };

        let id = ScheduleHourBmc::create(&ctx, &mm, schedule_hour_c).await?;

        // -- Check
        let schedule_hour: ScheduleHour = ScheduleHourBmc::get(&ctx, &mm, id).await?;
        assert_eq!(schedule_hour.course, fx_course);

        // -- Clean
        ScheduleHourBmc::delete(&ctx, &mm, id).await?;
        TeacherBmc::delete(&ctx, &mm, fx_teacher_id.unwrap()).await?;
        UserBmc::delete(&ctx, &mm, fx_user_id).await?;
        DepartmentBmc::delete(&ctx, &mm, fx_department_id).await?;

        Ok(())
    }
    
    #[serial]
    #[tokio::test]
    async fn test_update_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_course = 2024;
        let fx_course_new = 2025;
        let fx_username = "Prueba_schedule_hour_update_ok";
        let fx_teacher_name = "Profe_schedule_hour_update_ok";
        let fx_department_name = "Departamento_schedule_hour_update_ok";
        let fx_group_id = None;
        
        let fx_user_id = seed_user(&ctx, &mm, fx_username).await?;
        let fx_department_id= seed_department(&ctx, &mm, fx_department_name).await?;
        let fx_teacher_id = Some(_dev_utils::seed_teacher(&ctx, &mm, fx_teacher_name, fx_department_id ,fx_user_id).await?);
        let fx_schedule_hour_id = _dev_utils::seed_schedule_hour(&ctx, &mm, fx_course, fx_teacher_id, fx_group_id).await?;
        
        // -- Exec
        ScheduleHourBmc::update(
            &ctx,
            &mm,
            fx_schedule_hour_id,
            ScheduleHourForUpdate {
                course: fx_course_new,
                group_id: None,
                teacher_id: fx_teacher_id
            },
        )
            .await?;

        // -- Check
        let schedule_hour: ScheduleHour = ScheduleHourBmc::get(&ctx, &mm, fx_schedule_hour_id).await?;
        assert_eq!(schedule_hour.course, fx_course_new);

        // -- Clean
        ScheduleHourBmc::delete(&ctx, &mm, fx_schedule_hour_id).await?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_by_course_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_courses = &[2022, 2022] ;
        let fx_username = "Usuario_schedule_hour_list_by_name_ok";
        let fx_username_2 = "Usuario_schedule_hour_list_by_name_ok_2";
        let fx_department_name = "Departamento_schedule_hour_list_by_name_ok";
        let fx_teacher_name = "Profe_schedule_hour_list_by_name_ok";
        let fx_teacher_name_2 = "Profe_schedule_hour_list_by_name_ok_2";

        let fx_department_id = _dev_utils::seed_department(&ctx, &mm, fx_department_name).await?;
        let fx_user_id = _dev_utils::seed_user(&ctx, &mm, fx_username).await?;
        let fx_teacher_id = _dev_utils::seed_teacher(&ctx, &mm, fx_teacher_name, fx_department_id, fx_user_id).await?;
        let fx_user_id_2 = _dev_utils::seed_user(&ctx, &mm, fx_username_2).await?;
        let fx_teacher_id_2 = _dev_utils::seed_teacher(&ctx, &mm, fx_teacher_name_2, fx_department_id, fx_user_id_2).await?;

        let fx_id_01 = _dev_utils::seed_schedule_hour(&ctx, &mm, fx_courses[0], Some(fx_teacher_id), None).await?;
        let fx_id_02 = _dev_utils::seed_schedule_hour(&ctx, &mm, fx_courses[1], Some(fx_teacher_id_2), None).await?;

        // -- Exec
        let filter_json = json!({
            "course": {"$eq": fx_courses[0]}, // time in Rfc3339
        });
        let filter = vec![serde_json::from_value(filter_json)?];

        let schedule_hours = ScheduleHourBmc::list(&ctx, &mm, Some(filter), None).await?;

        // -- Check
        let schedule_hours: Vec<String> = schedule_hours.into_iter().map(|s| s.course.to_string()).collect();
        let fx_schedule_hours: Vec<String> = fx_courses.into_iter().map(|c| c.to_string()).collect();

        assert_eq!(schedule_hours.len(), 2);
        assert_eq!(&schedule_hours, &fx_schedule_hours);

        // -- Cleanup
        ScheduleHourBmc::delete(&ctx, &mm, fx_id_01).await?;
        ScheduleHourBmc::delete(&ctx, &mm, fx_id_02).await?;
        TeacherBmc::delete(&ctx, &mm, fx_teacher_id).await?;
        TeacherBmc::delete(&ctx, &mm, fx_teacher_id_2).await?;
        UserBmc::delete(&ctx, &mm, fx_user_id).await?;
        UserBmc::delete(&ctx, &mm, fx_user_id_2).await?;
        DepartmentBmc::delete(&ctx, &mm, fx_department_id).await?;        

        Ok(())
    }
}
// endregion: --- Tests
