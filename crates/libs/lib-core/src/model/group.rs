use modql::field::{Fields, HasFields};
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString, OpValsValue};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use sqlx::postgres::PgRow;

use crate::ctx::Ctx;
use crate::model::base::{self, PostgresDbBmc};
use crate::model::ModelManager;
use crate::model::modql_utils::time_to_sea_value;
use crate::model::Result;

// region:    --- Group Types
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Group {
    pub id: i64,
    pub course: i32,    // 1º, 2º
    pub stage: i32,     // ESO, Bachiller, Ciclos
    pub year: i32,      // 2023/2024, 2024/2025
    pub letter: String,
    pub tutor_id: i64,
}

#[derive(Fields, Deserialize, Clone)]
pub struct GroupForCreate {
    pub course: i32,    // 1º, 2º
    pub stage: i32,     // ESO, Bachiller, Ciclos
    pub year: i32,      // 2023/2024, 2024/2025
    pub letter: String,
    pub tutor_id: i64,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct GroupFilter {
    id: Option<OpValsInt64>,
    course: Option<OpValsInt64>,
    stage: Option<OpValsInt64>,
    year: Option<OpValsInt64>,
    letter: Option<OpValsString>,
    tutor_id: Option<OpValsInt64>,

    cid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    ctime: Option<OpValsValue>,
    mid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    mtime: Option<OpValsValue>,
}

#[derive(Fields, Default, Deserialize)]
pub struct GroupForUpdate {
    pub course: i32,    // 1º, 2º
    pub stage: i32,     // ESO, Bachiller, Ciclos
    pub year: i32,      // 2023/2024, 2024/2025
    pub letter: String,
    pub tutor_id: i64,
}

/// Marker trait
pub trait GroupBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl GroupBy for Group {}

// endregion: --- Group Types

pub struct GroupBmc;

impl PostgresDbBmc for GroupBmc {
    const TABLE: &'static str = "groups";
}

impl GroupBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, group_c: GroupForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, group_c).await
    }

    pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
    where
        E: GroupBy,
    {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<GroupFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Group>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }


    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        group_u: GroupForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, group_u).await
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
    use crate::ctx::Ctx;
    use crate::model::department::DepartmentBmc;
    use crate::model::group::{Group, GroupBmc, GroupForCreate, GroupForUpdate};
    use crate::model::teacher::TeacherBmc;
    use crate::model::user::UserBmc;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_course = 1;
        let fx_stage = 1;
        let fx_year = 2024;
        let fx_letter = "A";

        let fx_username = "username_create_ok";
        let fx_teacher_name = "teacher_name_create_ok";
        let fx_department_name = "department_name_create_ok";

        let fx_user_id = _dev_utils::seed_user(&ctx, &mm, fx_username).await?;
        let fx_department_id = _dev_utils::seed_department(&ctx, &mm, fx_department_name).await?;
        let fx_tutor_id = _dev_utils::seed_teacher(&ctx, &mm, fx_teacher_name, fx_department_id, fx_user_id).await?;
        
        // -- Exec
        let group_c = GroupForCreate {
            course: fx_course,
            stage: fx_stage,
            tutor_id: fx_tutor_id,
            letter: fx_letter.to_string(),
            year: fx_year
        };

        let id = GroupBmc::create(&ctx, &mm, group_c).await?;

        // -- Check
        let group: Group = GroupBmc::get(&ctx, &mm, id).await?;
        assert_eq!(group.letter, fx_letter);
        assert_eq!(group.year, fx_year);
        assert_eq!(group.stage, fx_stage);
        assert_eq!(group.tutor_id, fx_tutor_id);
        assert_eq!(group.course, fx_course);

        // -- Clean
        GroupBmc::delete(&ctx, &mm, id).await?;
        TeacherBmc::delete(&ctx, &mm, fx_tutor_id).await?;
        DepartmentBmc::delete(&ctx, &mm, fx_department_id).await?;
        UserBmc::delete(&ctx, &mm, fx_user_id).await?;
        
        Ok(())
    }
    
    #[serial]
    #[tokio::test]
    async fn test_update_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_course = 1;
        let fx_stage = 1;
        let fx_year = 2024;
        let fx_letter = "A";
        
        let fx_course_new = 2;
        let fx_stage_new = 2;
        let fx_year_new = 2025;
        let fx_letter_new = "B";

        let fx_usernames = &["Prueba01", "Prueba02"];
        let fx_teacher_names =  &["teacher_name_update_ok_01", "teacher_name_update_ok_02"];
        let fx_department_name = "department_name_update_ok";

        let fx_user_id = _dev_utils::seed_user(&ctx, &mm, fx_usernames[0]).await?;
        let fx_user_id_new = _dev_utils::seed_user(&ctx, &mm, fx_usernames[1]).await?;
        let fx_department_id = _dev_utils::seed_department(&ctx, &mm, fx_department_name).await?;
        let fx_tutor_id = _dev_utils::seed_teacher(&ctx, &mm, fx_teacher_names[0], fx_department_id, fx_user_id).await?;
        let fx_tutor_id_new = _dev_utils::seed_teacher(&ctx, &mm, fx_teacher_names[1], fx_department_id, fx_user_id_new).await?;
        let fx_group_id = _dev_utils::seed_group(&ctx, &mm, fx_letter, fx_course, fx_stage, fx_year, fx_tutor_id).await?;

        // -- Exec
        GroupBmc::update(
            &ctx,
            &mm,
            fx_group_id,
            GroupForUpdate {
                course: fx_course_new,
                stage: fx_stage_new,
                tutor_id: fx_tutor_id_new,
                letter: fx_letter_new.to_string(),
                year: fx_year_new
            },
        )
            .await?;

        // -- Check
        let group: Group = GroupBmc::get(&ctx, &mm, fx_group_id).await?;
        assert_eq!(group.letter, fx_letter_new);
        assert_eq!(group.year, fx_year_new);
        assert_eq!(group.course, fx_course_new);
        assert_eq!(group.tutor_id, fx_tutor_id_new);
        assert_eq!(group.stage, fx_stage_new);

        // -- Clean
        GroupBmc::delete(&ctx, &mm, fx_group_id).await?;
        TeacherBmc::delete(&ctx, &mm, fx_tutor_id).await?;
        TeacherBmc::delete(&ctx, &mm, fx_tutor_id_new).await?;
        DepartmentBmc::delete(&ctx, &mm, fx_department_id).await?;
        UserBmc::delete(&ctx, &mm, fx_user_id).await?;
        UserBmc::delete(&ctx, &mm, fx_user_id_new).await?;
        
        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_by_name_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_letters = &[
            "test_A",
            "test_B"
        ];
        let fx_course = 1;
        let fx_stage = 1;
        let fx_year = 2024;

        let fx_username = "username_list_by_name_ok";
        let fx_teacher_name = "teacher_name_list_by_name_ok";
        let fx_department_name = "department_name_list_by_name_ok";

        let fx_user_id = _dev_utils::seed_user(&ctx, &mm, fx_username).await?;
        let fx_department_id = _dev_utils::seed_department(&ctx, &mm, fx_department_name).await?;
        let fx_tutor_id = _dev_utils::seed_teacher(&ctx, &mm, fx_teacher_name, fx_department_id, fx_user_id).await?;


        let fx_id_01 = _dev_utils::seed_group(&ctx, &mm, fx_letters[0], fx_course, fx_stage, fx_year, fx_tutor_id).await?;
        let fx_id_02 = _dev_utils::seed_group(&ctx, &mm, fx_letters[1], fx_course, fx_stage, fx_year, fx_tutor_id).await?;

        // -- Exec
        let filter_json = json!({
            "name": {"$contains": "Prueba"}, // time in Rfc3339
        });
        let filter = vec![serde_json::from_value(filter_json)?];

        let groups = GroupBmc::list(&ctx, &mm, Some(filter), None).await?;

        // -- Check
        let groups: Vec<String> = groups.into_iter()
            .filter(|s| s.letter.starts_with("test_"))
            .map(|s| s.letter)
            .collect();
        assert_eq!(groups.len(), 2);
        assert_eq!(&groups, fx_letters);

        // -- Cleanup
        GroupBmc::delete(&ctx, &mm, fx_id_01).await?;
        GroupBmc::delete(&ctx, &mm, fx_id_02).await?;
        TeacherBmc::delete(&ctx, &mm, fx_tutor_id).await?;
        UserBmc::delete(&ctx, &mm, fx_user_id).await?;
        DepartmentBmc::delete(&ctx, &mm, fx_department_id).await?;
        
        Ok(())
    }
}
// endregion: --- Tests
