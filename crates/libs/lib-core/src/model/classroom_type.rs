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

// region:    --- ClassroomType Types
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct ClassroomType {
    pub id: i64,
    pub type_name: String,
}

#[derive(Fields, Deserialize, Clone)]
pub struct ClassroomTypeForCreate {
    pub type_name: String,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ClassroomTypeFilter {
    type_name: Option<OpValsString>,
    cid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    ctime: Option<OpValsValue>,
    mid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    mtime: Option<OpValsValue>,
}

#[derive(Fields, Default, Deserialize)]
pub struct ClassroomTypeForUpdate {
    pub type_name: String,
}

/// Marker trait
pub trait ClassroomTypeBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl ClassroomTypeBy for ClassroomType {}

// endregion: --- ClassroomType Types

pub struct ClassroomTypeBmc;

impl PostgresDbBmc for ClassroomTypeBmc {
    const TABLE: &'static str = "classroom_types";
}

impl ClassroomTypeBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, classroom_type_c: ClassroomTypeForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, classroom_type_c).await
    }

    pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
    where
        E: ClassroomTypeBy,
    {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<ClassroomTypeFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<ClassroomType>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        classroom_type_u: ClassroomTypeForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, classroom_type_u).await
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
    use crate::model::classroom_type::{ClassroomType, ClassroomTypeBmc, ClassroomTypeForCreate, ClassroomTypeForUpdate};

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        
        let fx_type_name = "Laboratory";
        
        // -- Exec
        let classroom_type_c = ClassroomTypeForCreate {
            type_name: fx_type_name.to_string(),
        };

        let id = ClassroomTypeBmc::create(&ctx, &mm, classroom_type_c).await?;

        // -- Check
        let classroom_type: ClassroomType = ClassroomTypeBmc::get(&ctx, &mm, id).await?;
        assert_eq!(classroom_type.type_name, fx_type_name);

        // -- Clean
        ClassroomTypeBmc::delete(&ctx, &mm, id).await?;
        
        Ok(())
    }
    
    #[serial]
    #[tokio::test]
    async fn test_update_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_type_name = "Laboratory";

        let fx_type_name_new = "Lecture Hall";
        
        let fx_classroom_type_id = _dev_utils::seed_classroom_type(&ctx, &mm, fx_type_name).await?;

        // -- Exec
        ClassroomTypeBmc::update(
            &ctx,
            &mm,
            fx_classroom_type_id,
            ClassroomTypeForUpdate {
                type_name: fx_type_name_new.to_string(),
            },
        )
            .await?;

        // -- Check
        let classroom_type: ClassroomType = ClassroomTypeBmc::get(&ctx, &mm, fx_classroom_type_id).await?;
        assert_eq!(classroom_type.type_name, fx_type_name_new);

        // -- Clean
        ClassroomTypeBmc::delete(&ctx, &mm, fx_classroom_type_id).await?;
        
        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_by_type_name_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_type_name = "Laboratory";
        let fx_type_name_new = "Lecture Hall";

        let fx_classroom_type_id_01 = _dev_utils::seed_classroom_type(&ctx, &mm, fx_type_name).await?;
        let fx_classroom_type_id_02 = _dev_utils::seed_classroom_type(&ctx, &mm, fx_type_name_new).await?;

        // -- Exec
        let filter_json = json!({
            "type_name": {"$contains": "Lab"}, // partial match example
        });
        let filter = vec![serde_json::from_value(filter_json)?];

        let classroom_types = ClassroomTypeBmc::list(&ctx, &mm, Some(filter), None).await?;

        // -- Check
        let classroom_types: Vec<String> = classroom_types.into_iter()
            .map(|s| s.type_name)
            .collect();
        assert_eq!(classroom_types.len(), 1);
        assert_eq!(classroom_types[0], fx_type_name);

        // -- Cleanup
        ClassroomTypeBmc::delete(&ctx, &mm, fx_classroom_type_id_01).await?;
        ClassroomTypeBmc::delete(&ctx, &mm, fx_classroom_type_id_02).await?;
        
        Ok(())
    }
}
// endregion: --- Tests
