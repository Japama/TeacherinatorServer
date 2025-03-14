use std::sync::Arc;

use axum::{Json, Router};
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use serde_json::{json, Value};

use lib_core::model::ModelManager;
use lib_rpc::{building_rpc, center_schedule_hour_rpc, classroom_rpc, classroom_type_rpc, department_rpc, group_rpc, schedule_hour_rpc, schedule_rpc, subject_rpc, user_rpc, RpcRequest, RpcResources};
use lib_rpc::router::RpcRouter;

use crate::web::mw_auth::CtxW;

/// The RpcState is the Axum State that will
/// be used for the Axum RPC router handler.
///
/// Note: Not to be confused with the RpcResources that are for the lib-rpc
///      layer for the RpcRouter System. The RpcResources typically contains some elements
///      from the RpcState
#[derive(Clone)]
pub struct RpcState {
    pub mm: ModelManager,
}

#[derive(Debug)]
pub struct RpcInfo {
    pub id: Option<Value>,
    pub method: String,
}

// Axum router for '/api/rpc'
pub fn routes(rpc_state: RpcState) -> Router {

    // Build the combined RpcRouter.
    let rpc_router = RpcRouter::new()
        .extend(user_rpc::rpc_router())
        .extend(department_rpc::rpc_router())
        .extend(subject_rpc::rpc_router())
        .extend(classroom_rpc::rpc_router())
        .extend(schedule_rpc::rpc_router())
        .extend(schedule_hour_rpc::rpc_router())
        .extend(center_schedule_hour_rpc::rpc_router())
        .extend(group_rpc::rpc_router())
        .extend(building_rpc::rpc_router())
        .extend(classroom_type_rpc::rpc_router());

    // Build the Axum Router for '/rpc'
    Router::new()
        .route("/rpc", post(rpc_axum_handler))
        .with_state((rpc_state, Arc::new(rpc_router)))
}

async fn rpc_axum_handler(
    State((rpc_state, rpc_router)): State<(RpcState, Arc<RpcRouter>)>,
    ctx: CtxW,
    Json(rpc_req): Json<RpcRequest>,
) -> Response {
    let ctx = ctx.0;

    // -- Create the RPC Info
    //    (will be set to the response.extensions)
    let rpc_info = RpcInfo {
        id: rpc_req.id.clone(),
        method: rpc_req.method.clone(),
    };
    let rpc_method = &rpc_info.method;
    let rpc_params = rpc_req.params;
    let rpc_resources = RpcResources {
        ctx: Some(ctx),
        mm: rpc_state.mm,
    };

    // -- Exec Rpc Route
    let res = rpc_router.call(rpc_method, rpc_resources, rpc_params).await;

    // -- Build Rpc Success Response
    let res = res.map(|v| {
        let body_response = json!({
            "id": rpc_info.id,
            "result": v
        });
        Json(body_response)
    });

    // -- Create and Update Axum Response
    let res: crate::web::Result<_> = res.map_err(crate::web::Error::from);
    let mut res = res.into_response();
    res.extensions_mut().insert(Arc::new(rpc_info));

    res
}
