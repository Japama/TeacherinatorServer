use modql::field::{Field, Fields, HasFields};
use modql::filter::{FilterNodes, ListOptions, OpValsBool, OpValsInt64, OpValsString, OpValsValue};
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query::extension::postgres::PgExpr;
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use sqlx::postgres::PgRow;
use time::Time;
use uuid::Uuid;

use lib_auth::pwd::{self, ContentToHash};
use lib_utils::time::now_utc;

use crate::ctx::Ctx;
use crate::model::{Error, ModelManager};
use crate::model::base::{self, add_timestamps_for_update, CommonIden, PostgresDbBmc};
use crate::model::modql_utils::time_to_sea_value;
use crate::model::Result;

// region:    --- User Types
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub is_admin: bool,
    pub in_center: bool,
    pub last_checkin: Option<Time>,
    pub last_checkout: Option<Time>,
    pub active: bool,
    pub department_id: Option<i64>,
    pub substituting_id: Option<i64>,
    pub substitutions: i64,
}

#[derive(Fields, Deserialize, Clone)]
pub struct UserForCreate {
    pub username: String,
    pub is_admin: bool,
    pub pwd: String,
    pub active: bool,
    pub department_id: Option<i64>,
    pub substituting_id: Option<i64>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct UserFilter {
    id: Option<OpValsInt64>,
    username: Option<OpValsString>,
    is_admin: Option<OpValsBool>,
    in_center: Option<OpValsBool>,
    active: Option<OpValsBool>,
    department_id: Option<OpValsInt64>,
    substituting_id: Option<OpValsInt64>,
    substitutions: Option<OpValsInt64>,


    cid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    ctime: Option<OpValsValue>,
    mid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    mtime: Option<OpValsValue>,
}

#[derive(Fields, Default, Deserialize, Clone)]
pub struct UserForUpdate {
    pub username: String,
    pub is_admin: bool,
    pub in_center: bool,
    pub last_checkin: Option<Time>,
    pub last_checkout: Option<Time>,
    pub active: bool,
    pub department_id: Option<i64>,
    pub substituting_id: Option<i64>,
    pub substitutions: i64,
}

#[derive(Fields, Default, Deserialize, Clone)]
pub struct UserForCheckin {
    pub in_center: bool,
    pub last_checkin: Option<Time>,
    pub last_checkout: Option<Time>,
}

#[derive(Fields, Default, Deserialize, Clone)]
pub struct UserForUpdatePwd {
    pub username: String,
    pub pwd: String,
    pub is_admin: bool,
    pub in_center: bool,
    pub last_checkin: Option<Time>,
    pub last_checkout: Option<Time>,
    pub active: bool,
    pub department_id: Option<i64>,
    pub substituting_id: Option<i64>,
    pub substitutions: i64,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForLogin {
    pub id: i64,
    pub username: String,
    pub is_admin: bool,

    // -- pwd and token info
    pub pwd: Option<String>,
    // encrypted, #_scheme_id_#....
    pub pwd_salt: Uuid,
    pub token_salt: Uuid,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForAuth {
    pub id: i64,
    pub username: String,
    pub is_admin: bool,

    // -- token info
    pub token_salt: Uuid,
}

/// Marker trait
pub trait UserBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl UserBy for User {}

impl UserBy for UserForLogin {}

impl UserBy for UserForAuth {}

#[derive(Iden)]
enum UserIden {
    Id,
    Username,
    IsAdmin,
    Pwd,
    DepartmentId,
    InCenter,
    LastCheckin,
    LastCheckout,
    Active,
    SubstitutingId,
    Substitutions
}

// endregion: --- User Types

pub struct UserBmc;

impl PostgresDbBmc for UserBmc {
    const TABLE: &'static str = "users";
}

impl UserBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, user_c: UserForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, user_c).await
    }

    pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
    where
        E: UserBy,
    {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn get_current(ctx: &Ctx, mm: &ModelManager) -> Result<User> {
        base::get::<Self, _>(ctx, mm, ctx.user_id()).await
    }

    pub async fn first_by_username<E>(
        _ctx: &Ctx,
        mm: &ModelManager,
        username: &str,
    ) -> Result<Option<E>>
    where
        E: UserBy,
    {
        let db = mm.postgres_db();

        // -- Build query
        let mut query = Query::select();
        query
            .from(Self::table_ref())
            .columns(E::field_idens())
            .and_where(Expr::col(UserIden::Username).eq(username));

        // -- Exec query
        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let user = sqlx::query_as_with::<_, E, _>(&sql, values)
            .fetch_optional(db)
            .await?;

        Ok(user)
    }

    pub async fn update_pwd(ctx: &Ctx, mm: &ModelManager, id: i64, pwd_clear: &str) -> Result<()> {
        let db = mm.postgres_db();

        // -- Prep password
        let user: UserForLogin = Self::get(ctx, mm, id).await?;
        let pwd = pwd::hash_pwd(ContentToHash {
            content: pwd_clear.to_string(),
            salt: user.pwd_salt,
        })
        .await?;

        // -- Prep the data
        let mut fields = Fields::new(vec![Field::new(UserIden::Pwd, pwd.into())]);
        add_timestamps_for_update(&mut fields, ctx.user_id());

        // -- Build query
        let fields = fields.for_sea_update();
        let mut query = Query::update();
        query
            .table(Self::table_ref())
            .values(fields)
            .and_where(Expr::col(UserIden::Id).eq(id));

        // -- Exec query
        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let _count = sqlx::query_with(&sql, values)
            .execute(db)
            .await?
            .rows_affected();

        Ok(())
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<UserFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<User>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        user_u: UserForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, user_u).await
    }

    pub async fn update_checkin(ctx: &Ctx, mm: &ModelManager, checkin: bool) -> Result<()> {
        let db = mm.postgres_db();
        let now = now_utc();
        let mut user_c = UserForCheckin {
            in_center: checkin,
            last_checkin: None,
            last_checkout: None,
        };
        if checkin {
            user_c.last_checkin = Some(now.time())
        } else {
            user_c.last_checkout = Some(now.time())
        }

        let mut fields = user_c.not_none_fields();

        add_timestamps_for_update(&mut fields, ctx.user_id());
        let fields = fields.for_sea_update();

        // -- Build query
        let mut query = Query::update();
        query
            .table(UserBmc::table_ref())
            .values(fields)
            .and_where(Expr::col(CommonIden::Id).eq(ctx.user_id()));

        // -- Exec query
        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let count = sqlx::query_with(&sql, values)
            .execute(db)
            .await?
            .rows_affected();

        // -- Check result
        if count == 0 {
            Err(Error::EntityNotFound {
                entity: UserBmc::TABLE,
                id: ctx.user_id(),
            })
        } else {
            Ok(())
        }
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }

    pub async fn check_username<E>(
        _ctx: &Ctx,
        mm: &ModelManager,
        username: &str,
    ) -> Result<Option<E>>
    where
        E: UserBy,
    {
        let db = mm.postgres_db();

        // -- Build query
        let mut query = Query::select();
        query
            .from(Self::table_ref())
            .columns(E::field_idens())
            .and_where(Expr::col(UserIden::Username).ilike(username));

        // -- Exec query
        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let user = sqlx::query_as_with::<_, E, _>(&sql, values)
            .fetch_optional(db)
            .await?;

        Ok(user)
    }

    pub async fn count_users_by_department(
        _ctx: &Ctx,
        mm: &ModelManager,
        department_id: i64,
    ) -> Result<i64> {
        let db = mm.postgres_db();

        // -- Build query
        let mut query = Query::select();
        query
            .expr(Expr::col(UserIden::Id).count())
            .from(UserBmc::table_ref())
            .and_where(Expr::col(UserIden::DepartmentId).eq(department_id));

        // -- Exec query
        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let result: (i64,) = sqlx::query_as_with(&sql, values).fetch_one(db).await?;

        Ok(result.0)
    }

    pub async fn users_by_department(
        _ctx: &Ctx,
        mm: &ModelManager,
        department_id: i64,
    ) -> Result<Vec<User>> {
        let db = mm.postgres_db();

        // -- Build query
        let mut query = Query::select();
        query
            .from(UserBmc::table_ref())
            .columns(vec![
                UserIden::Id,
                UserIden::Username,
                UserIden::Pwd,
                UserIden::IsAdmin,
                UserIden::DepartmentId,
                UserIden::InCenter,
                UserIden::LastCheckin,
                UserIden::LastCheckout,
                UserIden::Active,
                UserIden::SubstitutingId,
                UserIden::Substitutions
            ])
            .and_where(Expr::col(UserIden::DepartmentId).eq(department_id));

        // -- Exec query
        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let result = sqlx::query_as_with::<_, User, _>(&sql, values)
            .fetch_all(db)
            .await?;

        Ok(result)
    }

}


// region:    --- Tests
#[cfg(test)]
mod tests {
    use anyhow::{Context, Result};
    use serde_json::json;
    use serial_test::serial;

    use crate::_dev_utils::{self, seed_department};
    use crate::ctx::Ctx;
    use crate::model::user::{User, UserBmc, UserForCreate, UserForUpdate};

    #[serial]
    #[tokio::test]
    async fn test_first_ok_admin() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_username = "admin";

        // -- Exec
        let user: User = UserBmc::first_by_username(&ctx, &mm, fx_username)
            .await?
            .context("Should have user 'admin'")?;

        // -- Check
        assert_eq!(user.username, fx_username);

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_username = "Prueba_Crear";
        let fx_pwd = "Contraseña";
        let fx_department_name = "Department_test_create_user_oK";
        let fx_department_id = seed_department(&ctx, &mm, fx_department_name).await?;

        // -- Exec
        let user_c = UserForCreate {
            username: fx_username.to_string(),
            pwd: fx_pwd.to_string(),
            is_admin: false,
            active: false,
            department_id: Some(fx_department_id),
            substituting_id: None,
        };

        let id = UserBmc::create(&ctx, &mm, user_c).await?;

        // -- Check
        let user: User = UserBmc::get(&ctx, &mm, id).await?;
        assert_eq!(user.username, fx_username);

        // -- Clean
        UserBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_update_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_username = "Juanba test update";
        let fx_username_new = "Juanba test update Petao";
        let fx_user_id = _dev_utils::seed_user(&ctx, &mm, fx_username).await?;
        let fx_admin = true;
        let fx_active = true;
        let fx_department_name = "Department_test_update_user_oK";
        let fx_department_id = seed_department(&ctx, &mm, fx_department_name).await?;


        // -- Exec
        UserBmc::update(
            &ctx,
            &mm,
            fx_user_id,
            UserForUpdate {
                username: fx_username_new.to_string(),
                is_admin: fx_admin,
                active: fx_active,
                department_id: Some(fx_department_id),
                substituting_id: None,
                substitutions: 0,
                in_center: false,
                last_checkin: None,
                last_checkout: None,
            },
        )
        .await?;

        // -- Check
        let user: User = UserBmc::get(&ctx, &mm, fx_user_id).await?;
        assert_eq!(user.username, fx_username_new);

        // -- Clean
        UserBmc::delete(&ctx, &mm, fx_user_id).await?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_by_name_ok() -> Result<()> {
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_usernames = &["Juanba test list", "Marta test list", "Amalia test list"];
        let fx_id_01 = _dev_utils::seed_user(&ctx, &mm, fx_usernames[0]).await?;
        let fx_id_02 = _dev_utils::seed_user(&ctx, &mm, fx_usernames[1]).await?;
        let fx_id_03 = _dev_utils::seed_user(&ctx, &mm, fx_usernames[2]).await?;

        // -- Exec
        let filter_json = json!({
            "username": {"$contains": "test list"}, // time in Rfc3339
        });
        let filter = vec![serde_json::from_value(filter_json)?];

        let users = UserBmc::list(&ctx, &mm, Some(filter), None).await?;

        // -- Check
        let usernames: Vec<String> = users.into_iter().map(|u| u.username).collect();
        assert_eq!(usernames.len(), 3);
        assert_eq!(&usernames, fx_usernames);

        // -- Cleanup
        UserBmc::delete(&ctx, &mm, fx_id_01).await?;
        UserBmc::delete(&ctx, &mm, fx_id_02).await?;
        UserBmc::delete(&ctx, &mm, fx_id_03).await?;

        Ok(())
    }
}
// endregion: --- Tests
