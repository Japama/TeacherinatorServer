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

// region:    --- Building Types
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Building {
    pub id: i64,
    pub building_name: String,
}

#[derive(Fields, Deserialize, Clone)]
pub struct BuildingForCreate {
    pub building_name: String,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct BuildingFilter {
    id: Option<OpValsInt64>,
    building_name: Option<OpValsString>,
    cid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    ctime: Option<OpValsValue>,
    mid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    mtime: Option<OpValsValue>,
}

#[derive(Fields, Default, Deserialize)]
pub struct BuildingForUpdate {
    pub building_name: String,
}

/// Marker trait
pub trait BuildingBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl BuildingBy for Building {}

// endregion: --- Building Types

pub struct BuildingBmc;

impl PostgresDbBmc for BuildingBmc {
    const TABLE: &'static str = "buildings";
}

impl BuildingBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, building_c: BuildingForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, building_c).await
    }

    pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
    where
        E: BuildingBy,
    {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<BuildingFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Building>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        building_u: BuildingForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, building_u).await
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
    use crate::model::building::{Building, BuildingBmc, BuildingForCreate, BuildingForUpdate};

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        
        let fx_building_name = "Principal_test_create_ok_building";
        
        // -- Exec
        let building_c = BuildingForCreate {
            building_name: fx_building_name.to_string(),
        };

        let id = BuildingBmc::create(&ctx, &mm, building_c).await?;

        // -- Check
        let building: Building = BuildingBmc::get(&ctx, &mm, id).await?;
        assert_eq!(building.building_name, fx_building_name);

        // -- Clean
        BuildingBmc::delete(&ctx, &mm, id).await?;
        
        Ok(())
    }
    
    #[serial]
    #[tokio::test]
    async fn test_update_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_building_name = "Principal_test_update_ok_building";
        let fx_building_name_new = "Grande_test_update_ok_building";
       
        let fx_building_id = _dev_utils::seed_building(&ctx, &mm, fx_building_name).await?;

        // -- Exec
        BuildingBmc::update(
            &ctx,
            &mm,
            fx_building_id,
            BuildingForUpdate {
                building_name: fx_building_name_new.to_string(),
            },
        )
            .await?;

        // -- Check
        let building: Building = BuildingBmc::get(&ctx, &mm, fx_building_id).await?;
        assert_eq!(building.building_name, fx_building_name_new);

        // -- Clean
        BuildingBmc::delete(&ctx, &mm, fx_building_id).await?;
        
        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_by_building_name_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_building_name = "Principal_test_list_by_building_name_ok_building";
        let fx_building_name_new = "Grande_test_list_by_building_name_ok_building";

        let fx_building_id_01 = _dev_utils::seed_building(&ctx, &mm, fx_building_name).await?;
        let fx_building_id_02 = _dev_utils::seed_building(&ctx, &mm, fx_building_name_new).await?;

        // -- Exec
        let filter_json = json!({
            "building_name": {"$contains": "_building"}, // partial match example
        });
        let filter = vec![serde_json::from_value(filter_json)?];

        let buildings = BuildingBmc::list(&ctx, &mm, Some(filter), None).await?;

        // -- Check
        let buildings: Vec<String> = buildings.into_iter()
            .map(|s| s.building_name)
            .collect();
        assert_eq!(buildings.len(), 2);
        assert_eq!(buildings[0], fx_building_name);

        // -- Cleanup
        BuildingBmc::delete(&ctx, &mm, fx_building_id_01).await?;
        BuildingBmc::delete(&ctx, &mm, fx_building_id_02).await?;
        
        Ok(())
    }
}
// endregion: --- Tests
