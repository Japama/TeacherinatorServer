use log::debug;
use lib_core::ctx::Ctx;
use lib_core::model::ModelManager;
use lib_core::model::user::{User, UserBmc, UserFilter, UserForCreate, UserForUpdate, UserForUpdatePwd};

use crate::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsIdedString, ParamsList};
use crate::Error::UserNotAdmin;
use crate::Result;
use crate::router::RpcRouter;
use crate::rpc_router;

pub fn rpc_router() -> RpcRouter {
    rpc_router!(
        // Same as RpcRouter::new().add...
        create_user,
        get_user,
        get_current_user,
        list_users,
        update_user,
        update_user_pwd,
        delete_user,
        check_duplicate_username,
        user_checkin,
        user_checkout,
        count_users_by_department,
        users_by_department,
    )
}

pub async fn create_user(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<UserForCreate>,
) -> Result<User> {
    if !&ctx.admin() { return Err(UserNotAdmin); }
    let ParamsForCreate { data } = params;

    let id = UserBmc::create(&ctx, &mm, data.clone()).await?;
    let pwd = data.pwd;
    UserBmc::update_pwd(&ctx, &mm, id, &pwd).await?;
    let user = UserBmc::get(&ctx, &mm, id).await?;

    Ok(user)
}

pub async fn get_user(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<User> {
    if !&ctx.admin() { return Err(UserNotAdmin); }
    let ParamsIded { id } = params;

    let user = UserBmc::get(&ctx, &mm, id).await?;

    Ok(user)
}

pub async fn list_users(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsList<UserFilter>,
) -> Result<Vec<User>> {
    if !&ctx.admin() { return Err(UserNotAdmin); }
    let users = UserBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

    Ok(users)
}

pub async fn update_user(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<UserForUpdate>,
) -> Result<User> {
    if !&ctx.admin() { return Err(UserNotAdmin); }
    let ParamsForUpdate { id, data } = params;

    UserBmc::update(&ctx, &mm, id, data.clone()).await?;
    // let pwd = data.pwd;
    // UserBmc::update_pwd(&ctx, &mm, id, &pwd).await?;
    let user = UserBmc::get(&ctx, &mm, id).await?;

    Ok(user)
}

pub async fn get_current_user(
    ctx: Ctx,
    mm: ModelManager,
) -> Result<User> {
    let user = UserBmc::get_current(&ctx, &mm).await?;
    Ok(user)
}

pub async fn user_checkin(
    ctx: Ctx,
    mm: ModelManager,
) -> Result<User> {
    UserBmc::update_checkin(&ctx, &mm, true).await?;
    let user = UserBmc::get(&ctx, &mm, ctx.user_id()).await?;

    Ok(user)
}

pub async fn user_checkout(
    ctx: Ctx,
    mm: ModelManager,
) -> Result<User> {
    UserBmc::update_checkin(&ctx, &mm, false).await?;
    let user = UserBmc::get(&ctx, &mm, ctx.user_id()).await?;

    Ok(user)
}

pub async fn update_user_pwd(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<UserForUpdatePwd>,
) -> Result<User> {
    if !&ctx.admin() { return Err(UserNotAdmin); }
    let ParamsForUpdate { id, data } = params;

    let user_for_update = UserForUpdate {
        username: data.username,
        is_admin: data.is_admin,
        active: data.active,
        department_id: data.department_id,
        last_checkin: data.last_checkin,
        last_checkout: data.last_checkout,
        in_center: data.in_center,
        substituting_id: data.substituting_id,
        substitutions: data.substitutions
    };

    if data.department_id.is_some(){
        debug!("Departamento: {}", data.department_id.unwrap())
    }

    UserBmc::update(&ctx, &mm, id, user_for_update).await?;
    let pwd = data.pwd;
    if pwd != "" {
        UserBmc::update_pwd(&ctx, &mm, id, &pwd).await?;
    }
    let user = UserBmc::get(&ctx, &mm, id).await?;

    Ok(user)
}

pub async fn delete_user(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<User> {
    if !&ctx.admin() { return Err(UserNotAdmin); }
    let ParamsIded { id } = params;

    let user = UserBmc::get(&ctx, &mm, id).await?;
    UserBmc::delete(&ctx, &mm, id).await?;

    Ok(user)
}

pub async fn check_duplicate_username(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIdedString,
) -> Result<bool> {
    let user: Option<User> = UserBmc::check_username(&ctx, &mm, &params.data).await?;
    Ok(user.is_some())
}


pub async fn count_users_by_department(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded,
) -> Result<i64> {
    let ParamsIded { id } = params;
    let teacher_number = UserBmc::count_users_by_department(&ctx, &mm, id).await?;
    Ok(teacher_number)
}

pub async fn users_by_department(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded,
) -> Result<Vec<User>> {
    let ParamsIded { id } = params;
    let teacher_number = UserBmc::users_by_department(&ctx, &mm, id).await?;
    Ok(teacher_number)
}