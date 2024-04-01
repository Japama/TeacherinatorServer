use lib_core::ctx::Ctx;
use lib_core::model::ModelManager;
use lib_core::model::subject::{Subject, SubjectBmc, SubjectFilter, SubjectForCreate, SubjectForUpdate};

use crate::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use crate::Result;
use crate::router::RpcRouter;
use crate::rpc_router;

pub fn rpc_router() -> RpcRouter {
    rpc_router!(
        // Same as RpcRouter::new().add...
        create_subject,
        get_subject,
        list_subjects,
        update_subject,
        delete_subject,
    )
}

pub async fn create_subject(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<SubjectForCreate>,
) -> Result<Subject> {
    let ParamsForCreate { data } = params;

    let id = SubjectBmc::create(&ctx, &mm, data).await?;
    let subject = SubjectBmc::get(&ctx, &mm, id).await?;

    Ok(subject)
}

pub async fn get_subject(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Subject> {
    let ParamsIded { id } = params;

    let subject = SubjectBmc::get(&ctx, &mm, id).await?;

    Ok(subject)
}

pub async fn list_subjects(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsList<SubjectFilter>,
) -> Result<Vec<Subject>> {
    let subjects = SubjectBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

    Ok(subjects)
}

pub async fn update_subject(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<SubjectForUpdate>,
) -> Result<Subject> {
    let ParamsForUpdate { id, data } = params;

    SubjectBmc::update(&ctx, &mm, id, data).await?;

    let subject = SubjectBmc::get(&ctx, &mm, id).await?;

    Ok(subject)
}

pub async fn delete_subject(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Subject> {
    let ParamsIded { id } = params;

    let subject = SubjectBmc::get(&ctx, &mm, id).await?;
    SubjectBmc::delete(&ctx, &mm, id).await?;

    Ok(subject)
}
