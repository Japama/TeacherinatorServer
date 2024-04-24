use modql::field::{Fields, HasFields};
use modql::filter::{FilterNodes, ListOptions, OpValsBool, OpValsInt64, OpValsString, OpValsValue};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use sqlx::postgres::PgRow;

use crate::ctx::Ctx;
use crate::model::base::{self, PostgresDbBmc};
use crate::model::ModelManager;
use crate::model::modql_utils::time_to_sea_value;
use crate::model::Result;

// region:    --- Subject Types
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Subject {
    pub id: i64,
    pub name: String,
    pub department_id: i64,
    pub is_guard: bool,
    pub is_complementary: bool
}

#[derive(Fields, Deserialize, Clone)]
pub struct SubjectForCreate {
    pub name: String,
    pub department_id: i64,
    pub is_guard: bool,
    pub is_complementary: bool
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct SubjectFilter {
    id: Option<OpValsInt64>,
    name: Option<OpValsString>,
    department_id: Option<OpValsInt64>,
    is_guard: Option<OpValsBool>,
    is_complementary: Option<OpValsBool>,

    cid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    ctime: Option<OpValsValue>,
    mid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    mtime: Option<OpValsValue>,
}

#[derive(Fields, Default, Deserialize)]
pub struct SubjectForUpdate {
    pub name: String,
    pub department_id: i64,
    pub is_guard: bool,
    pub is_complementary: bool
}

/// Marker trait
pub trait SubjectBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl SubjectBy for Subject {}

// endregion: --- Subject Types

pub struct SubjectBmc;

impl PostgresDbBmc for SubjectBmc {
    const TABLE: &'static str = "subjects";
}

impl SubjectBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, subject_c: SubjectForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, subject_c).await
    }

    pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
    where
        E: SubjectBy,
    {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<SubjectFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Subject>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }


    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        subject_u: SubjectForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, subject_u).await
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
    use crate::_dev_utils::seed_department;
    use crate::ctx::Ctx;
    use crate::model::department::DepartmentBmc;
    use crate::model::subject::{Subject, SubjectBmc, SubjectForCreate, SubjectForUpdate};

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_name = "Informática";
        let fx_is_guard = false;
        let fx_is_complementary = false;
        let fx_department_name = "Departamento";
        
        let fx_department_id = seed_department(&ctx, &mm, fx_department_name).await?;
        // -- Exec
        let subject_c = SubjectForCreate {
            name: fx_name.to_string(),
            department_id: fx_department_id,
            is_guard: fx_is_guard,
            is_complementary: fx_is_complementary
        };

        let id = SubjectBmc::create(&ctx, &mm, subject_c).await?;

        // -- Check
        let subject: Subject = SubjectBmc::get(&ctx, &mm, id).await?;
        assert_eq!(subject.name, fx_name);

        // -- Clean
        SubjectBmc::delete(&ctx, &mm, id).await?;
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
        let fx_department_name = "Departamento prueba";

        let fx_department_id = seed_department(&ctx, &mm, fx_department_name).await?;
        let fx_subject_id = _dev_utils::seed_subject(&ctx, &mm, fx_name, fx_department_id, false, false).await?;

        // -- Exec
        SubjectBmc::update(
            &ctx,
            &mm,
            fx_subject_id,
            SubjectForUpdate {
                is_guard: true,
                is_complementary: true,
                department_id: fx_department_id,
                name: fx_name_new.to_string(),
            },
        )
            .await?;

        // -- Check
        let subject: Subject = SubjectBmc::get(&ctx, &mm, fx_subject_id).await?;
        assert_eq!(subject.name, fx_name_new);

        // -- Clean
        SubjectBmc::delete(&ctx, &mm, fx_subject_id).await?;
        DepartmentBmc::delete(&ctx, &mm, fx_department_id).await?;
        
        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_by_name_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_names = &["Prueba", "Prueba2"];
        let fx_department_name = "Departamento prueba";

        let fx_department_id = seed_department(&ctx, &mm, fx_department_name).await?;
        let fx_id_01 = _dev_utils::seed_subject(&ctx, &mm, fx_names[0], fx_department_id, false, false).await?;
        let fx_id_02 = _dev_utils::seed_subject(&ctx, &mm, fx_names[1], fx_department_id, true, true).await?;

        // -- Exec
        let filter_json = json!({
            "name": {"$contains": "Prueba"}, // time in Rfc3339
        });
        let filter = vec![serde_json::from_value(filter_json)?];

        let subjects = SubjectBmc::list(&ctx, &mm, Some(filter), None).await?;

        // -- Check
        let subjects: Vec<String> = subjects.into_iter().map(|s| s.name).collect();
        assert_eq!(subjects.len(), 2);
        assert_eq!(&subjects, fx_names);

        // -- Cleanup
        SubjectBmc::delete(&ctx, &mm, fx_id_01).await?;
        SubjectBmc::delete(&ctx, &mm, fx_id_02).await?;
        DepartmentBmc::delete(&ctx, &mm, fx_department_id).await?;

        Ok(())
    }
}
// endregion: --- Tests
