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

// region:    --- Classroom Types
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Classroom {
    pub id: i64,
    pub building: String,
    pub floor: i32,
    pub number: i32,
    pub name: String,
    pub type_c: i32,
    pub description: String,
}

#[derive(Fields, Deserialize, Clone)]
pub struct ClassroomForCreate {
    pub building: String,
    pub floor: i32,
    pub number: i32,
    pub name: String,
    pub type_c: i32,
    pub description: String,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ClassroomFilter {
    building: Option<OpValsString>,
    floor: Option<OpValsInt64>,
    number: Option<OpValsInt64>,
     name: Option<OpValsString>,
    type_c: Option<OpValsInt64>,
    description: Option<OpValsString>,

    cid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    ctime: Option<OpValsValue>,
    mid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    mtime: Option<OpValsValue>,
}

#[derive(Fields, Default, Deserialize)]
pub struct ClassroomForUpdate {
    pub building: String,
    pub floor: i32,
    pub number: i32,
    pub name: String,
    pub type_c: i32,
    pub description: String,
}

/// Marker trait
pub trait ClassroomBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl ClassroomBy for Classroom {}

// endregion: --- Classroom Types

pub struct ClassroomBmc;

impl PostgresDbBmc for ClassroomBmc {
    const TABLE: &'static str = "classrooms";
}

impl ClassroomBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, classroom_c: ClassroomForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, classroom_c).await
    }

    pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
    where
        E: ClassroomBy,
    {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<ClassroomFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Classroom>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }


    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        classroom_u: ClassroomForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, classroom_u).await
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
    use crate::model::classroom::{Classroom, ClassroomBmc, ClassroomForCreate, ClassroomForUpdate};

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        
        let fx_building = "Principal";
        let fx_name = "Info5";
        let fx_description = "Aula de informática con 30 ordenadores";
        let fx_floor = 1;
        let fx_number = 19;
        let fx_type = 1;
        
        // -- Exec
        let classroom_c = ClassroomForCreate {
            building: fx_building.to_string(),
            floor: fx_floor,
            number: fx_number,
            name: fx_name.to_string(),
            type_c: fx_type,
            description: fx_description.to_string(),
        };

        let id = ClassroomBmc::create(&ctx, &mm, classroom_c).await?;

        // -- Check
        let classroom: Classroom = ClassroomBmc::get(&ctx, &mm, id).await?;
        assert_eq!(classroom.building, fx_building);
        assert_eq!(classroom.floor, fx_floor);
        assert_eq!(classroom.number, fx_number);
        assert_eq!(classroom.name, fx_name);
        assert_eq!(classroom.type_c, fx_type);
        assert_eq!(classroom.description, fx_description);

        // -- Clean
        ClassroomBmc::delete(&ctx, &mm, id).await?;
        
        Ok(())
    }
    
    #[serial]
    #[tokio::test]
    async fn test_update_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_building = "Principal";
        let fx_name = "Info5";
        let fx_description = "Aula de informática con 30 ordenadores";
        let fx_floor = 1;
        let fx_number = 19;
        let fx_type = 1;

        let fx_building_new = "Grande";
        let fx_name_new = "Info6";
        let fx_description_new = "Aula de informática con 35 ordenadores";
        let fx_floor_new = 2;
        let fx_number_new = 29;
        let fx_type_new = 2;
        
        let fx_classroom_id = _dev_utils::seed_classroom(&ctx, &mm, fx_building, fx_floor, fx_number, fx_name, fx_type, fx_description).await?;

        // -- Exec
        ClassroomBmc::update(
            &ctx,
            &mm,
            fx_classroom_id,
            ClassroomForUpdate {
                building: fx_building_new.to_string(),
                floor: fx_floor_new,
                number: fx_number_new,
                name: fx_name_new.to_string(),
                type_c: fx_type_new,
                description: fx_description_new.to_string(),
            },
        )
            .await?;

        // -- Check
        let classroom: Classroom = ClassroomBmc::get(&ctx, &mm, fx_classroom_id).await?;
        assert_eq!(classroom.building, fx_building_new);
        assert_eq!(classroom.floor, fx_floor_new);
        assert_eq!(classroom.number, fx_number_new);
        assert_eq!(classroom.name, fx_name_new);
        assert_eq!(classroom.type_c, fx_type_new);
        assert_eq!(classroom.description, fx_description_new);

        // -- Clean
        ClassroomBmc::delete(&ctx, &mm, fx_classroom_id).await?;
        
        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_by_name_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_building = "Principal";
        let fx_name = "Info5";
        let fx_description = "Aula de informática con 30 ordenadores";
        let fx_floor = 1;
        let fx_number = 19;
        let fx_type = 1;

        let fx_building_new = "Grande";
        let fx_name_new = "Info6";
        let fx_description_new = "Aula de informática con 35 ordenadores";
        let fx_floor_new = 2;
        let fx_number_new = 29;
        let fx_type_new = 2;

        let fx_classroom_id_01 = _dev_utils::seed_classroom(&ctx, &mm, fx_building, fx_floor, fx_number, fx_name, fx_type, fx_description).await?;
        let fx_classroom_id_02 = _dev_utils::seed_classroom(&ctx, &mm, fx_building_new, fx_floor_new, fx_number_new, fx_name_new, fx_type_new, fx_description_new).await?;

        // -- Exec
        let filter_json = json!({
            "description": {"$contains": "Aula de informática"}, // time in Rfc3339
        });
        let filter = vec![serde_json::from_value(filter_json)?];

        let classrooms = ClassroomBmc::list(&ctx, &mm, Some(filter), None).await?;

        // -- Check
        let classrooms: Vec<String> = classrooms.into_iter()
            // .filter(|s| s.letter.starts_with("test_"))
            .map(|s| s.building)
            .collect();
        assert_eq!(classrooms.len(), 2);
        // assert_eq!(&classrooms, fx_letters);

        // -- Cleanup
        ClassroomBmc::delete(&ctx, &mm, fx_classroom_id_01).await?;
        ClassroomBmc::delete(&ctx, &mm, fx_classroom_id_02).await?;
        
        Ok(())
    }
}
// endregion: --- Tests
