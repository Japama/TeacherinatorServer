use modql::field::{Fields, HasFields};
use modql::filter::{FilterNodes, ListOptions, OpValsBool, OpValsInt64, OpValsValue};
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::postgres::PgRow;
use sqlx::FromRow;

use crate::ctx::Ctx;
use crate::model::base::{self, PostgresDbBmc};
use crate::model::modql_utils::time_to_sea_value;
use crate::model::ModelManager;
use crate::model::Result;

// region:    --- Teacher Types
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Teacher {
    pub id: i64,
    pub user_id: i64,
    pub active: bool,
    pub department_id: i64,
}

#[derive(Fields, Deserialize, Clone)]
pub struct TeacherForCreate {
    pub user_id: i64,
    pub active: bool,
    pub department_id: i64,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct TeacherFilter {
    id: Option<OpValsInt64>,
    user_id: Option<OpValsInt64>,
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
    pub active: bool,
    pub department_id: i64,
}

#[derive(Iden)]
enum TeacherIden {
    Id,
    UserId,
    DepartmentId,
    Active,
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

    pub async fn get_user_teacher(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Teacher>>
    {
        let filters = Some(TeacherFilter {
            id: None,
            user_id: Some(OpValsInt64::from(ctx.user_id())),
            active: None,
            department_id: None,
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
    pub async fn count_teachers_by_department(
        _ctx: &Ctx,
        mm: &ModelManager,
        department_id: i64,
    ) -> Result<i64> {
        let db = mm.postgres_db();

        // -- Build query
        let mut query = Query::select();
        query
            .expr(Expr::col(TeacherIden::Id).count())
            .from(TeacherBmc::table_ref())
            .and_where(Expr::col(TeacherIden::DepartmentId).eq(department_id));

        // -- Exec query
        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let result: (i64,) = sqlx::query_as_with(&sql, values).fetch_one(db).await?;

        Ok(result.0)
    }

    pub async fn teachers_by_department(
        _ctx: &Ctx,
        mm: &ModelManager,
        department_id: i64,
    ) -> Result<Vec<Teacher>> {
        let db = mm.postgres_db();

        // -- Build query
        let mut query = Query::select();
        query
            // .expr(Expr::col(TeacherIden::Id).count())
            .from(TeacherBmc::table_ref())
            .columns(vec![
                TeacherIden::Id,
                TeacherIden::UserId,
                TeacherIden::Active,
                TeacherIden::DepartmentId,
            ])
            .and_where(Expr::col(TeacherIden::DepartmentId).eq(department_id));

        // -- Exec query
        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let result = sqlx::query_as_with::<_, Teacher, _>(&sql, values)
            .fetch_all(db)
            .await?;

        Ok(result)
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
    use crate::model::teacher::{Teacher, TeacherBmc, TeacherForCreate, TeacherForUpdate};
    use crate::model::user::UserBmc;

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
            user_id: fx_user_id,
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
        let fx_department_name = "Departamento";
        let fx_user_id = _dev_utils::seed_user(&ctx, &mm, fx_username).await?;
        let fx_department_id = _dev_utils::seed_department(&ctx, &mm, fx_department_name).await?;
        let fx_teacher_id =
            _dev_utils::seed_teacher(&ctx, &mm, fx_name, fx_department_id, fx_user_id).await?;

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
        let fx_department_name = "Departamento";

        let fx_department_id = _dev_utils::seed_department(&ctx, &mm, fx_department_name).await?;
        let fx_user_id_01 = _dev_utils::seed_user(&ctx, &mm, fx_usernames[0]).await?;
        let fx_user_id_02 = _dev_utils::seed_user(&ctx, &mm, fx_usernames[1]).await?;
        let fx_id_01 =
            _dev_utils::seed_teacher(&ctx, &mm, fx_names[0], fx_department_id, fx_user_id_01)
                .await?;
        let fx_id_02 =
            _dev_utils::seed_teacher(&ctx, &mm, fx_names[1], fx_department_id, fx_user_id_02)
                .await?;

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
