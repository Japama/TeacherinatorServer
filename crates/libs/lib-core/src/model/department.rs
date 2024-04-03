use modql::field::{Fields, HasFields};
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString, OpValsValue};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use sqlx::postgres::PgRow;
use crate::model::modql_utils::time_to_sea_value;
use crate::ctx::Ctx;
use crate::model::base::{self, PostgresDbBmc};
use crate::model::ModelManager;
use crate::model::Result;

// region:    --- Department Types
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Department {
    pub id: i64,
    pub name: String,
}

#[derive(Fields, Deserialize, Clone)]
pub struct DepartmentForCreate {
    pub name: String,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct DepartmentFilter {
    id: Option<OpValsInt64>,
    name: Option<OpValsString>,

    cid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    ctime: Option<OpValsValue>,
    mid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    mtime: Option<OpValsValue>,
}

#[derive(Fields, Default, Deserialize)]
pub struct DepartmentForUpdate {
    pub name: String,
}


#[derive(Fields)]
pub struct DepartmentForInsert {
    pub name: String,
}

/// Marker trait
pub trait DepartmentBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl DepartmentBy for Department {}

// endregion: --- Department Types

pub struct DepartmentBmc;

impl PostgresDbBmc for DepartmentBmc {
    const TABLE: &'static str = "departments";
}

impl DepartmentBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, department_c: DepartmentForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, department_c).await
    }

    pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
    where
        E: DepartmentBy,
    {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<DepartmentFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Department>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }


    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        department_u: DepartmentForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, department_u).await
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
    use crate::model::department::{Department, DepartmentBmc, DepartmentForCreate, DepartmentForUpdate};

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_name = "Informática_create_ok";

        // -- Exec
        let department_c = DepartmentForCreate {
            name: fx_name.to_string(),
        };

        let id = DepartmentBmc::create(&ctx, &mm, department_c).await?;

        // -- Check
        let department: Department = DepartmentBmc::get(&ctx, &mm, id).await?;
        assert_eq!(department.name, fx_name);

        // -- Clean
        DepartmentBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }
    
    #[serial]
    #[tokio::test]
    async fn test_update_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_name = "Prueba_update_ok";
        let fx_name_new = "Resultado prueba_update_ok";
        let fx_department_id = _dev_utils::seed_department(&ctx, &mm, fx_name).await?;

        // -- Exec
        DepartmentBmc::update(
            &ctx,
            &mm,
            fx_department_id,
            DepartmentForUpdate {
                name: fx_name_new.to_string(),
            },
        )
            .await?;

        // -- Check
        let department: Department = DepartmentBmc::get(&ctx, &mm, fx_department_id).await?;
        assert_eq!(department.name, fx_name_new);

        // -- Clean
        DepartmentBmc::delete(&ctx, &mm, fx_department_id).await?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_by_name_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_names = &[
            "Prueba_list_by_name_ok",
            "Prueba2_list_by_name_ok"
        ];
        let fx_id_01 = _dev_utils::seed_department(&ctx, &mm, fx_names[0]).await?;
        let fx_id_02 = _dev_utils::seed_department(&ctx, &mm, fx_names[1]).await?;

        // -- Exec
        let filter_json = json!({
            "name": {"$contains": "Prueba"}, // time in Rfc3339
        });
        let filter = vec![serde_json::from_value(filter_json)?];

        let departments = DepartmentBmc::list(&ctx, &mm, Some(filter), None).await?;

        // -- Check
        let departments: Vec<String> = departments.into_iter().map(|s| s.name).collect();
        assert_eq!(departments.len(), 2);
        assert_eq!(&departments, fx_names);

        // -- Cleanup
        DepartmentBmc::delete(&ctx, &mm, fx_id_01).await?;
        DepartmentBmc::delete(&ctx, &mm, fx_id_02).await?;

        Ok(())
    }
}
// endregion: --- Tests
