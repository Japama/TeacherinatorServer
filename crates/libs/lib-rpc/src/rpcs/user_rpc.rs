use lib_core::ctx::Ctx;
use lib_core::model::ModelManager;
use lib_core::model::user::{User, UserBmc, UserFilter, UserForCreate, UserForUpdate, UserForUpdatePwd};

use crate::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use crate::Result;
use crate::router::RpcRouter;
use crate::rpc_router;

pub fn rpc_router() -> RpcRouter {
    rpc_router!(
        // Same as RpcRouter::new().add...
        create_user,
        get_user,
        list_users,
        update_user,
        update_user_pwd,
        delete_user,
    )
}

pub async fn create_user(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<UserForCreate>,
) -> Result<User> {
    let ParamsForCreate { data } = params;

    let id = UserBmc::create(&ctx, &mm, data.clone()).await?;
    let pwd = data.pwd;
    UserBmc::update_pwd(&ctx, &mm, id, &pwd).await?;
    let user = UserBmc::get(&ctx, &mm, id).await?;

    Ok(user)
}

pub async fn get_user(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<User> {
    let ParamsIded { id } = params;

    let user = UserBmc::get(&ctx, &mm, id).await?;

    Ok(user)
}

pub async fn list_users(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsList<UserFilter>,
) -> Result<Vec<User>> {
    let users = UserBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

    Ok(users)
}

pub async fn update_user(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<UserForUpdate>,
) -> Result<User> {
    let ParamsForUpdate { id, data } = params;

    UserBmc::update(&ctx, &mm, id, data.clone()).await?;
    // let pwd = data.pwd;
    // UserBmc::update_pwd(&ctx, &mm, id, &pwd).await?;
    let user = UserBmc::get(&ctx, &mm, id).await?;


    Ok(user)
}

pub async fn update_user_pwd(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<UserForUpdatePwd>,
) -> Result<User> {
    let ParamsForUpdate { id, data } = params;
    
    let user_for_update = UserForUpdate {
        username: data.username.clone(),
        isadmin: data.isadmin,
    };

    UserBmc::update(&ctx, &mm, id, user_for_update).await?;
    let pwd = data.pwd;
    if pwd != "" {
        UserBmc::update_pwd(&ctx, &mm, id, &pwd).await?;
    }
    let user = UserBmc::get(&ctx, &mm, id).await?;

    Ok(user)
}


pub async fn delete_user(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<User> {
    let ParamsIded { id } = params;

    let user = UserBmc::get(&ctx, &mm, id).await?;
    UserBmc::delete(&ctx, &mm, id).await?;

    Ok(user)
}
