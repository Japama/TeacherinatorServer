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

// region:    --- Specialty Types
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Specialty {
    pub id: i64,
    pub name: String,
}

#[derive(Fields, Deserialize, Clone)]
pub struct SpecialtyForCreate {
    pub name: String,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct SpecialtyFilter {
    name: Option<OpValsString>,

    cid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    ctime: Option<OpValsValue>,
    mid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    mtime: Option<OpValsValue>,
}

#[derive(Fields, Default, Deserialize)]
pub struct SpecialtyForUpdate {
    pub name: String,
}


#[derive(Fields)]
pub struct SpecialtyForInsert {
    pub name: String,
}

/// Marker trait
pub trait SpecialtyBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl SpecialtyBy for Specialty {}

// endregion: --- Specialty Types

pub struct SpecialtyBmc;

impl PostgresDbBmc for SpecialtyBmc {
    const TABLE: &'static str = "specialties";
}

impl SpecialtyBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, specialty_c: SpecialtyForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, specialty_c).await
    }

    pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
    where
        E: SpecialtyBy,
    {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<SpecialtyFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Specialty>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }


    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        specialty_u: SpecialtyForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, specialty_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
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
    use crate::model::specialty::{Specialty, SpecialtyBmc, SpecialtyForCreate, SpecialtyForUpdate};

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_name = "Informática";

        // -- Exec
        let specialty_c = SpecialtyForCreate {
            name: fx_name.to_string(),
        };

        let id = SpecialtyBmc::create(&ctx, &mm, specialty_c).await?;

        // -- Check
        let specialty: Specialty = SpecialtyBmc::get(&ctx, &mm, id).await?;
        assert_eq!(specialty.name, fx_name);

        // -- Clean
        SpecialtyBmc::delete(&ctx, &mm, id).await?;

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
        let fx_specialty_id = _dev_utils::seed_specialty(&ctx, &mm, fx_name).await?;

        // -- Exec
        SpecialtyBmc::update(
            &ctx,
            &mm,
            fx_specialty_id,
            SpecialtyForUpdate {
                name: fx_name_new.to_string(),
            },
        )
            .await?;

        // -- Check
        let user: Specialty = SpecialtyBmc::get(&ctx, &mm, fx_specialty_id).await?;
        assert_eq!(user.name, fx_name_new);

        // -- Clean
        SpecialtyBmc::delete(&ctx, &mm, fx_specialty_id).await?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_by_name_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_names = &[
            "Prueba",
            "Prueba2"
        ];
        let fx_id_01 = _dev_utils::seed_specialty(&ctx, &mm, fx_names[0]).await?;
        let fx_id_02 = _dev_utils::seed_specialty(&ctx, &mm, fx_names[1]).await?;

        // -- Exec
        let filter_json = json!({
            "name": {"$contains": "Prueba"}, // time in Rfc3339
        });
        let filter = vec![serde_json::from_value(filter_json)?];

        let specialties = SpecialtyBmc::list(&ctx, &mm, Some(filter), None).await?;

        // -- Check
        let specialties: Vec<String> = specialties.into_iter().map(|s| s.name).collect();
        assert_eq!(specialties.len(), 2);
        assert_eq!(&specialties, fx_names);

        // -- Cleanup
        SpecialtyBmc::delete(&ctx, &mm, fx_id_01).await?;
        SpecialtyBmc::delete(&ctx, &mm, fx_id_02).await?;

        Ok(())
    }
}
// endregion: --- Tests
