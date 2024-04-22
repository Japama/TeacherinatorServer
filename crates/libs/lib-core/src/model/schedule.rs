use modql::field::{Fields, HasFields};
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsValue};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use sqlx::postgres::PgRow;
use crate::model::modql_utils::time_to_sea_value;
use crate::ctx::Ctx;
use crate::model::base::{self, PostgresDbBmc};
use crate::model::ModelManager;
use crate::model::Result;

// region:    --- Schedule Types
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Schedule {
    pub id: i64,
    pub teacher_id: Option<i64>,
    pub group_id: Option<i64>,
    pub course: i32,
}

#[derive(Fields, Deserialize, Clone)]
pub struct ScheduleForCreate {
    pub teacher_id: Option<i64>,
    pub group_id: Option<i64>,
    pub course: i32,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ScheduleFilter {
    id: Option<OpValsInt64>,

    teacher_id: Option<OpValsInt64>,
    group_id: Option<OpValsInt64>,
    course: Option<OpValsInt64>,

    cid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    ctime: Option<OpValsValue>,
    mid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    mtime: Option<OpValsValue>,
}

#[derive(Fields, Default, Deserialize)]
pub struct ScheduleForUpdate {
    pub teacher_id: Option<i64>,
    pub group_id: Option<i64>,
    pub course: i32,
}

/// Marker trait
pub trait ScheduleBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}
impl ScheduleBy for Schedule {}

// endregion: --- Schedule Types

pub struct ScheduleBmc;

impl PostgresDbBmc for ScheduleBmc {
    const TABLE: &'static str = "schedules";
}

impl ScheduleBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, schedule_c: ScheduleForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, schedule_c).await
    }

    pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
    where
        E: ScheduleBy,
    {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn get_teacher_schedule(ctx: &Ctx, mm: &ModelManager, teacher_id: i64) -> Result<Vec<Schedule>>
    {
        let filters = Some(ScheduleFilter {
            id: None,
            teacher_id: Some(OpValsInt64::from(teacher_id)),
            group_id: None,
            course: None,
            cid: None,
            ctime: None,
            mid: None,
            mtime: None,
        });

        let list_options = Some(ListOptions {
            limit: None,
            offset: None,
            order_bys: None
        });

       base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }


    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<ScheduleFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Schedule>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }


    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        schedule_u: ScheduleForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, schedule_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}

// region:    --- Tests
#[cfg(test)]
mod tests {
    use anyhow::{Result};
    use serde_json::json;
    use serial_test::serial;
    use crate::_dev_utils;
    use crate::_dev_utils::{seed_department, seed_user};
    use crate::ctx::Ctx;
    use crate::model::department::DepartmentBmc;
    use crate::model::schedule::{Schedule, ScheduleBmc, ScheduleForCreate, ScheduleForUpdate};
    use crate::model::teacher::TeacherBmc;
    use crate::model::user::UserBmc;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_course = 2024;
        let fx_username = "Prueba_schedule_create_ok";
        let fx_teacher_name = "Profe_schedule_create_ok";
        let fx_department_name = "Departamento_schedule_create_ok";
        
        let fx_user_id = seed_user(&ctx, &mm, fx_username).await?;
        let fx_department_id= seed_department(&ctx, &mm, fx_department_name).await?;
        let fx_teacher_id = Some(_dev_utils::seed_teacher(&ctx, &mm, fx_teacher_name, fx_department_id ,fx_user_id).await?);

        // -- Exec
        let schedule_c = ScheduleForCreate {
            course: fx_course,
            group_id: None,
            teacher_id: fx_teacher_id,
        };

        let id = ScheduleBmc::create(&ctx, &mm, schedule_c).await?;

        // -- Check
        let schedule: Schedule = ScheduleBmc::get(&ctx, &mm, id).await?;
        assert_eq!(schedule.course, fx_course);

        // -- Clean
        ScheduleBmc::delete(&ctx, &mm, id).await?;
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
        let fx_username = "Prueba_schedule_update_ok";
        let fx_teacher_name = "Profe_schedule_update_ok";
        let fx_department_name = "Departamento_schedule_update_ok";
        let fx_group_id = -1;
        
        let fx_user_id = seed_user(&ctx, &mm, fx_username).await?;
        let fx_department_id= seed_department(&ctx, &mm, fx_department_name).await?;
        let fx_teacher_id = _dev_utils::seed_teacher(&ctx, &mm, fx_teacher_name, fx_department_id ,fx_user_id).await?;
        let fx_schedule_id = _dev_utils::seed_schedule(&ctx, &mm, fx_course, fx_teacher_id, fx_group_id).await?;
        
        // -- Exec
        ScheduleBmc::update(
            &ctx,
            &mm,
            fx_schedule_id,
            ScheduleForUpdate {
                course: fx_course_new,
                group_id: None,
                teacher_id: Some(fx_teacher_id)
            },
        )
            .await?;

        // -- Check
        let schedule: Schedule = ScheduleBmc::get(&ctx, &mm, fx_schedule_id).await?;
        assert_eq!(schedule.course, fx_course_new);

        // -- Clean
        ScheduleBmc::delete(&ctx, &mm, fx_schedule_id).await?;
        TeacherBmc::delete(&ctx, &mm, fx_teacher_id).await?;
        DepartmentBmc::delete(&ctx, &mm, fx_department_id).await?;
        UserBmc::delete(&ctx, &mm, fx_user_id).await?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_by_course_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_courses = &[2022, 2022] ;
        let fx_username = "Usuario_schedule_list_by_name_ok";
        let fx_username_2 = "Usuario_schedule_list_by_name_ok_2";
        let fx_department_name = "Departamento_schedule_list_by_name_ok";
        let fx_teacher_name = "Profe_schedule_list_by_name_ok";
        let fx_teacher_name_2 = "Profe_schedule_list_by_name_ok_2";

        let fx_department_id = _dev_utils::seed_department(&ctx, &mm, fx_department_name).await?;
        let fx_user_id = _dev_utils::seed_user(&ctx, &mm, fx_username).await?;
        let fx_teacher_id = _dev_utils::seed_teacher(&ctx, &mm, fx_teacher_name, fx_department_id, fx_user_id).await?;
        let fx_user_id_2 = _dev_utils::seed_user(&ctx, &mm, fx_username_2).await?;
        let fx_teacher_id_2 = _dev_utils::seed_teacher(&ctx, &mm, fx_teacher_name_2, fx_department_id, fx_user_id_2).await?;

        let fx_id_01 = _dev_utils::seed_schedule(&ctx, &mm, fx_courses[0], fx_teacher_id, -1).await?;
        let fx_id_02 = _dev_utils::seed_schedule(&ctx, &mm, fx_courses[1], fx_teacher_id_2, -1).await?;

        // -- Exec
        let filter_json = json!({
            "course": {"$eq": fx_courses[0]}, // time in Rfc3339
        });
        let filter = vec![serde_json::from_value(filter_json)?];

        let schedules = ScheduleBmc::list(&ctx, &mm, Some(filter), None).await?;

        // -- Check
        let schedules: Vec<String> = schedules.into_iter().map(|s| s.course.to_string()).collect();
        let fx_schedules: Vec<String> = fx_courses.into_iter().map(|c| c.to_string()).collect();

        assert_eq!(schedules.len(), 2);
        assert_eq!(&schedules, &fx_schedules);

        // -- Cleanup
        ScheduleBmc::delete(&ctx, &mm, fx_id_01).await?;
        ScheduleBmc::delete(&ctx, &mm, fx_id_02).await?;
        TeacherBmc::delete(&ctx, &mm, fx_teacher_id).await?;
        TeacherBmc::delete(&ctx, &mm, fx_teacher_id_2).await?;
        UserBmc::delete(&ctx, &mm, fx_user_id).await?;
        UserBmc::delete(&ctx, &mm, fx_user_id_2).await?;
        DepartmentBmc::delete(&ctx, &mm, fx_department_id).await?;        

        Ok(())
    }
}
// endregion: --- Tests
