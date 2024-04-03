use modql::field::{Fields, HasFields};
use modql::filter::{FilterNodes, ListOptions, OpValsBool, OpValsInt64, OpValsString, OpValsValue};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use sqlx::postgres::PgRow;
use crate::model::modql_utils::time_to_sea_value;
use crate::ctx::Ctx;
use crate::model::base::{self, PostgresDbBmc};
use crate::model::ModelManager;
use crate::model::Result;

// region:    --- Teacher Types
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Teacher {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub active: bool,
    pub department_id: i64
}

#[derive(Fields, Deserialize, Clone)]
pub struct TeacherForCreate {
    pub user_id: i64,
    pub name: String,
    pub active: bool,
    pub department_id: i64
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct TeacherFilter {
    id: Option<OpValsInt64>,
    user_id: Option<OpValsInt64>,
    name: Option<OpValsString>,
    active: Option<OpValsBool>,
    department_id: Option<OpValsInt64>,

    cid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    ctime: Option<OpValsValue>,
    mid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    mtime: Option<OpValsValue>,
}

#[derive(Fields, Default, Deserialize)]
pub struct TeacherForUpdate {
    pub user_id: i64,
    pub name: String,
    pub active: bool,
    pub department_id: i64
}

/// Marker trait
pub trait TeacherBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl TeacherBy for Teacher {}

// endregion: --- Teacher Types

pub struct TeacherBmc;

impl PostgresDbBmc for TeacherBmc {
    const TABLE: &'static str = "teachers";
}

impl TeacherBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, teacher_c: TeacherForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, teacher_c).await
    }

    pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
    where
        E: TeacherBy,
    {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<TeacherFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Teacher>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }


    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        teacher_u: TeacherForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, teacher_u).await
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
    use crate::ctx::Ctx;
    use crate::model::department::DepartmentBmc;
    use crate::model::teacher::{Teacher, TeacherBmc, TeacherForCreate, TeacherForUpdate};
    use crate::model::user::{UserBmc};

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_name = "Informática_create_ok";
        let fx_username = "Pepito_create_ok";
        let fx_department_name = "Informática_create_ok";
        
        let fx_user_id = _dev_utils::seed_user(&ctx, &mm, fx_username).await?;
        let fx_department_id = _dev_utils::seed_department(&ctx, &mm, fx_department_name).await?;

        // -- Exec
        let teacher_c = TeacherForCreate {
            name: fx_name.to_string(),
            active: true,
            department_id: fx_department_id,
            user_id: fx_user_id
        };

        let id = TeacherBmc::create(&ctx, &mm, teacher_c).await?;

        // -- Check
        let teacher: Teacher = TeacherBmc::get(&ctx, &mm, id).await?;
        assert_eq!(teacher.name, fx_name);

        // -- Clean
        TeacherBmc::delete(&ctx, &mm, id).await?;
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
        let fx_name = "Prueba";
        let fx_name_new = "Resultado prueba";
        let fx_username = "Usuario";
        let fx_department_name= "Departamento";
        let fx_user_id = _dev_utils::seed_user(&ctx, &mm, fx_username).await?;
        let fx_department_id = _dev_utils::seed_department(&ctx, &mm, fx_department_name).await?;
        let fx_teacher_id = _dev_utils::seed_teacher(&ctx, &mm, fx_name, fx_department_id, fx_user_id).await?;

        // -- Exec
        TeacherBmc::update(
            &ctx,
            &mm,
            fx_teacher_id,
            TeacherForUpdate {
                user_id: fx_user_id,
                active: true,
                department_id: fx_department_id,
                name: fx_name_new.to_string(),
            },
        )
            .await?;

        // -- Check
        let teacher: Teacher = TeacherBmc::get(&ctx, &mm, fx_teacher_id).await?;
        assert_eq!(teacher.name, fx_name_new);

        // -- Clean
        TeacherBmc::delete(&ctx, &mm, fx_teacher_id).await?;
        DepartmentBmc::delete(&ctx, &mm, fx_department_id).await?;
        UserBmc::delete(&ctx, &mm, fx_user_id).await?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_by_name_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_names = &["Prueba", "Prueba2"];
        let fx_usernames = &["Prueba", "Prueba2"];
        let fx_department_name= "Departamento";
        
        let fx_department_id = _dev_utils::seed_department(&ctx, &mm, fx_department_name).await?;
        let fx_user_id_01 = _dev_utils::seed_user(&ctx, &mm, fx_usernames[0]).await?;
        let fx_user_id_02 = _dev_utils::seed_user(&ctx, &mm, fx_usernames[1]).await?;
        let fx_id_01 = _dev_utils::seed_teacher(&ctx, &mm, fx_names[0], fx_department_id, fx_user_id_01).await?;
        let fx_id_02 = _dev_utils::seed_teacher(&ctx, &mm, fx_names[1], fx_department_id, fx_user_id_02).await?;

        // -- Exec
        let filter_json = json!({
            "name": {"$contains": "Prueba"}, // time in Rfc3339
        });
        let filter = vec![serde_json::from_value(filter_json)?];

        let teachers = TeacherBmc::list(&ctx, &mm, Some(filter), None).await?;

        // -- Check
        let teachers: Vec<String> = teachers.into_iter().map(|s| s.name).collect();
        assert_eq!(teachers.len(), 2);
        assert_eq!(&teachers, fx_names);

        // -- Cleanup
        TeacherBmc::delete(&ctx, &mm, fx_id_01).await?;
        TeacherBmc::delete(&ctx, &mm, fx_id_02).await?;
        UserBmc::delete(&ctx, &mm, fx_user_id_01).await?;
        UserBmc::delete(&ctx, &mm, fx_user_id_02).await?;
        DepartmentBmc::delete(&ctx, &mm, fx_department_id).await?;
        
        Ok(())
    }
}
// endregion: --- Tests
