// #![allow(unused)] // For early development.

// region:    --- Modules

use std::thread;
use std::time::Duration as StdDuration;

use axum::{middleware, Router};
use axum::http::header::{ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE};
use axum::http::Method;
// endregion: --- Modules
use chrono::{Duration, Timelike, Utc};
use clokwerk::{Scheduler, TimeUnits};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

use config::web_config;
use lib_core::_dev_utils;
use lib_core::ctx::Ctx;
use lib_core::model::control::ControlBmc;
use lib_core::model::ModelManager;

use crate::web::{routes_login, routes_static};
use crate::web::mw_auth::{ mw_ctx_require, mw_ctx_resolve};
use crate::web::mw_res_map::mw_reponse_map;
use crate::web::mw_stamp::mw_req_stamp;
use crate::web::routes_rpc::RpcState;

pub use self::error::Result;

mod config;
mod error;
mod log;
mod web;

fn iniciar_programador_tareas() {
    let mut scheduler = Scheduler::new();

    // Supongamos que quieres que la tarea se ejecute todos los días a las 03:00
    let hora_objetivo = 3;

    // Calcula la duración hasta la próxima hora de ejecución
    let ahora = Utc::now().hour() as i64;
    let horas_hasta_objetivo = (hora_objetivo - ahora + 24) % 24;
    let duracion_hasta_objetivo = Duration::hours(horas_hasta_objetivo);
    let segundos_hasta_objetivo = duracion_hasta_objetivo.num_seconds() as u32;
    let intervalo_hasta_objetivo = clokwerk::Interval::Seconds(segundos_hasta_objetivo);

    // Programa la primera tarea para que se ejecute a la hora objetivo
    scheduler.every(intervalo_hasta_objetivo).run(|| {
        println!("Ejecutando tarea diaria...");
    });

    // Programa la tarea recurrente para que se ejecute cada 24 horas
    scheduler.every(10.seconds()).run(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            println!("Ejecutando tarea diaria...");
            let mm = ModelManager::new().await.unwrap();
            let ctx = Ctx::root_ctx(); // o Ctx::new(user_id).unwrap(); si tienes un user_id específico
            ControlBmc::update_guards(&ctx, &mm).await.unwrap();
            // for hour in hours {
            //     println!("{}", hour.start_time)
            // }
        });
        
    });


    // Ejecuta el programador en un nuevo hilo
    let _thread_handle = thread::spawn(move || {
        loop {
            scheduler.run_pending();
            thread::sleep(StdDuration::from_secs(10));
        }
    });
}

#[tokio::main]
async fn main() -> Result<()> {
    iniciar_programador_tareas();
    tracing_subscriber::fmt()
        .without_time() // For early local development.
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // -- FOR DEV ONLY
    _dev_utils::init_dev().await;

    // Initialize ModelManager.
    let mm = ModelManager::new().await?;

    // -- Define Routes
    let rpc_state = RpcState { mm: mm.clone() };
    let routes_rpc =
        web::routes_rpc::routes(rpc_state).route_layer(middleware::from_fn(mw_ctx_require));

    let origins = [
        "http://192.168.3.3:8080".parse().unwrap(),
        "http://localhost:8080".parse().unwrap(),
        "http://localhost:3000".parse().unwrap(),
        "http://127.0.0.1:3000".parse().unwrap(),
        "http://127.0.0.1:8080".parse().unwrap(),
    ];

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::POST, Method::OPTIONS])
        // allow requests from any origin
        .allow_origin(origins)
        .allow_credentials(true)
        .allow_headers([CONTENT_TYPE, ACCESS_CONTROL_ALLOW_ORIGIN]);

    let routes_all = Router::new()
        .merge(routes_login::routes(mm.clone()))
        .nest("/api", routes_rpc)
        .layer(middleware::map_response(mw_reponse_map))
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
        .layer(middleware::from_fn(mw_req_stamp))
        .layer(CookieManagerLayer::new())
        .layer(cors)
        .fallback_service(routes_static::serve_dir());

    // region:    --- Start Server
    // Note: For this block, ok to unwrap.
    let listener = TcpListener::bind("127.0.0.1:8081").await.unwrap();
    info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server

    Ok(())
}
