use modql::field::{Fields, HasFields};
use modql::filter::{FilterNodes, ListOptions, OpValsInt32, OpValsInt64, OpValsString, OpValsValue};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use sqlx::postgres::PgRow;

use crate::ctx::Ctx;
use crate::model::base::{self, PostgresDbBmc};
use crate::model::ModelManager;
use crate::model::modql_utils::time_to_sea_value;
use crate::model::Result;

// region:    --- ScheduleHour Types


#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct ScheduleHour {
    pub id: i64,
    pub schedule_id: i64,
    pub subject_name: String,
    pub classroom_name: String,
    pub week_day: i32,
    pub n_hour: i32,
    pub course: i32,
    pub notes: Option<String>
}

#[derive(Fields, Deserialize, Clone)]
pub struct ScheduleHourForCreate {
    pub schedule_id: i64,
    pub subject_name: String,
    pub classroom_name: String,
    pub week_day: i32,
    pub n_hour: i32,
    pub course: i32,
    pub notes: Option<String>
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ScheduleHourFilter {
    pub id: Option<OpValsInt64>,

    pub schedule_id: Option<OpValsInt64>,
    pub subject_name: Option<OpValsString>,
    pub classroom_name: Option<OpValsString>,
    pub week_day: Option<OpValsInt32>,
    pub n_hour: Option<OpValsInt32>,
    pub course: Option<OpValsInt32>,
    pub notes: Option<OpValsString>,

    pub cid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    pub ctime: Option<OpValsValue>,
    pub mid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    pub mtime: Option<OpValsValue>,
}

impl Default for ScheduleHourForUpdate {
    fn default() -> Self {
        Self {
            schedule_id: 0,
            subject_name: "".to_string(), // default value
            classroom_name: "".to_string(), // default value
            week_day: 0,   // default value
            n_hour: 0,     // default value
            course: 0,     // default value
            notes: Some("".to_string())
        }
    }
}

#[derive(Fields, Deserialize)]
pub struct ScheduleHourForUpdate {
    pub schedule_id: i64,
    pub subject_name: String,
    pub classroom_name: String,
    pub week_day: i32,
    pub n_hour: i32,
    pub course: i32,
    pub notes: Option<String>
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

    // Función para convertir DateTime<Utc> a NaiveTime
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
    use anyhow::Result;
    use serde_json::json;
    use serial_test::serial;

    use crate::_dev_utils;
    use crate::_dev_utils::{seed_department, seed_schedule, seed_subject, seed_teacher, seed_user};
    use crate::ctx::Ctx;
    use crate::model::department::DepartmentBmc;
    use crate::model::schedule::ScheduleBmc;
    use crate::model::schedule_hour::{ScheduleHour, ScheduleHourBmc, ScheduleHourForCreate, ScheduleHourForUpdate};
    use crate::model::subject::SubjectBmc;
    use crate::model::teacher::TeacherBmc;
    use crate::model::user::UserBmc;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_subject_name = "subject_create_ok";
        let fx_week_day = 1; // lunes
        let fx_n_hour = 1; // 08:00-08:50
        let fx_course = 2024;

        let fx_username = "Prueba_schedule_hour_create_ok";
        let fx_department_name = "Departamento_schedule_hour_create_ok";
        let fx_classroom_name = "Clase 54";

        let fx_user_id = seed_user(&ctx, &mm, fx_username).await?;
        let fx_department_id = seed_department(&ctx, &mm, fx_department_name).await?;
        let fx_subject_id = seed_subject(&ctx, &mm, fx_subject_name, fx_department_id, false, false).await?;
        let fx_teacher_id = seed_teacher(&ctx, &mm,  fx_department_id ,fx_user_id).await?;
        let fx_schedule_id= seed_schedule(&ctx, &mm, fx_course, fx_teacher_id, -1).await?;

        // -- Exec
        let schedule_hour_c = ScheduleHourForCreate {
            schedule_id: fx_schedule_id,
            subject_name: fx_subject_name.to_string(),
            classroom_name: fx_classroom_name.to_string(),
            week_day: fx_week_day,
            n_hour: fx_n_hour,
            course: fx_course,
            notes: None
        };

        let id = ScheduleHourBmc::create(&ctx, &mm, schedule_hour_c).await?;

        // -- Check
        let schedule_hour: ScheduleHour = ScheduleHourBmc::get(&ctx, &mm, id).await?;
        assert_eq!(schedule_hour.course, fx_course);

        // -- Clean
        ScheduleHourBmc::delete(&ctx, &mm, id).await?;
        ScheduleBmc::delete(&ctx, &mm, fx_schedule_id).await?;
        TeacherBmc::delete(&ctx, &mm, fx_teacher_id).await?;
        UserBmc::delete(&ctx, &mm, fx_user_id).await?;
        SubjectBmc::delete(&ctx, &mm, fx_subject_id).await?;
        DepartmentBmc::delete(&ctx, &mm, fx_department_id).await?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_update_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_week_day = 1; // Lunes
        let fx_week_day_new= 2; // Martes
        let fx_n_hour = 1; // 08:00-08:50
        let fx_course = 2024;

        let fx_username = "Prueba_schedule_hour_create_ok";
        let fx_department_name = "Departamento_schedule_hour_create_ok";
        let fx_classroom_name = "Clase 34";
        let fx_subject_name = "Subject_update_ok";

        let fx_user_id = seed_user(&ctx, &mm, fx_username).await?;
        let fx_department_id = seed_department(&ctx, &mm, fx_department_name).await?;
        let fx_subject_id = seed_subject(&ctx, &mm, fx_subject_name, fx_department_id, false, false).await?;
        let fx_teacher_id = seed_teacher(&ctx, &mm,  fx_department_id ,fx_user_id).await?;
        let fx_schedule_id= seed_schedule(&ctx, &mm, fx_course, fx_teacher_id, -1).await?;
        let fx_schedule_hour_id = _dev_utils::seed_schedule_hour(&ctx, &mm, fx_schedule_id, fx_subject_name, fx_classroom_name , fx_week_day, fx_n_hour, fx_course).await?;

        // -- Exec
        ScheduleHourBmc::update(
            &ctx,
            &mm,
            fx_schedule_hour_id,
            ScheduleHourForUpdate {
                schedule_id: fx_schedule_id,
                subject_name: fx_subject_name.to_string(),
                classroom_name: fx_classroom_name.to_string(),
                week_day: fx_week_day_new,
                course: fx_course,
                n_hour: fx_n_hour,
                notes: None
            },
        )
            .await?;

        // -- Check
        let schedule_hour: ScheduleHour = ScheduleHourBmc::get(&ctx, &mm, fx_schedule_hour_id).await?;
        assert_eq!(schedule_hour.week_day, fx_week_day_new);

        // -- Clean
        ScheduleHourBmc::delete(&ctx, &mm, fx_schedule_hour_id).await?;
        ScheduleBmc::delete(&ctx, &mm, fx_schedule_id).await?;
        TeacherBmc::delete(&ctx, &mm, fx_teacher_id).await?;
        UserBmc::delete(&ctx, &mm, fx_user_id).await?;
        SubjectBmc::delete(&ctx, &mm, fx_subject_id).await?;
        DepartmentBmc::delete(&ctx, &mm, fx_department_id).await?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_by_course_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_week_day = 1; // Lunes
        let fx_week_day_2= 2; // Martes
        let fx_n_hour = 1; // 08:00-08:50
        let fx_course = 2024;

        let fx_username = "Prueba_list_by_course_ok";
        let fx_department_name = "Departamento_list_by_course_ok";
        let fx_classroom_name = "Clase 56";
        let fx_subject_name = "Subject_list_ok";

        let fx_user_id = seed_user(&ctx, &mm, fx_username).await?;
        let fx_department_id = seed_department(&ctx, &mm, fx_department_name).await?;
        let fx_subject_id = seed_subject(&ctx, &mm, fx_subject_name, fx_department_id, false, false).await?;
        let fx_teacher_id = seed_teacher(&ctx, &mm,  fx_department_id ,fx_user_id).await?;
        let fx_schedule_id= seed_schedule(&ctx, &mm, fx_course, fx_teacher_id, -1).await?;
        let fx_schedule_hour_id_01 = _dev_utils::seed_schedule_hour(&ctx, &mm, fx_schedule_id, fx_subject_name, fx_classroom_name, fx_week_day, fx_n_hour, fx_course).await?;
        let fx_schedule_hour_id_02 = _dev_utils::seed_schedule_hour(&ctx, &mm, fx_schedule_id, fx_subject_name, fx_classroom_name, fx_week_day_2, fx_n_hour, fx_course).await?;

        // -- Exec
        let filter_json = json!({
            "schedule_id": {"$eq": fx_schedule_id},
        });
        let filter = vec![serde_json::from_value(filter_json)?];
    
        let schedule_hours = ScheduleHourBmc::list(&ctx, &mm, Some(filter), None).await?;
    
        // -- Check
        // let schedule_hours: Vec<String> = schedule_hours.into_iter().map(|s| s.course.to_string()).collect();
        // let fx_schedule_hours: Vec<String> = fx_courses.into_iter().map(|c| c.to_string()).collect();
    
        assert_eq!(schedule_hours.len(), 2);
        // assert_eq!(&schedule_hours, &fx_schedule_hours);
    
        // -- Cleanup
        ScheduleHourBmc::delete(&ctx, &mm, fx_schedule_hour_id_01).await?;
        ScheduleHourBmc::delete(&ctx, &mm, fx_schedule_hour_id_02).await?;
        ScheduleBmc::delete(&ctx, &mm, fx_schedule_id).await?;
        TeacherBmc::delete(&ctx, &mm, fx_teacher_id).await?;
        UserBmc::delete(&ctx, &mm, fx_user_id).await?;
        SubjectBmc::delete(&ctx, &mm, fx_subject_id).await?;
        DepartmentBmc::delete(&ctx, &mm, fx_department_id).await?;
    
        Ok(())
    }
}
// endregion: --- Tests
