#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    check_task().await?;
    // check_activity().await?;

    Ok(())
}

async fn check_task() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:8081")?;

    // hc.do_get("/index.html").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "admin",
            "pwd": "welcome"
        }),
    );
    req_login.await?.print().await?;

    let req_create_project = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "create_project",
            "params": {
                "data": {
                    "name": "project AAA"
                }
            }
        }),
    );
    let result = req_create_project.await?;
    result.print().await?;
    let project_id = result.json_value::<i64>("/result/id")?;

    let mut task_ids: Vec<i64> = Vec::new();
    for i in 1..=5 {
        let req_create_task = hc.do_post(
            "/api/rpc",
            json!({
                "id": 1,
                "method": "create_task",
                "params": {
                    "data": {
                        "project_id": project_id,
                        "title": format!("task AAA {i}")
                    }
                }
            }),
        );
        let result = req_create_task.await?;
        task_ids.push(result.json_value::<i64>("/result/id")?);
    }

    let req_update_task = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "update_task",
            "params": {
                "id": task_ids[0], // The first task created.
                "data": {
                    "title": "task BB"
                }
            }
        }),
    );
    req_update_task.await?.print().await?;

    let req_delete_task = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "delete_task",
            "params": {
                "id": task_ids[1] // The second task created.
            }
        }),
    );
    req_delete_task.await?.print().await?;

    let req_list_all_tasks = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "list_tasks",
            "params": {
                "filters": {
                    "project_id": project_id
                },
                "list_options": {
                    "order_bys": "!title"
                }
            }
        }),
    );
    req_list_all_tasks.await?.print().await?;

    let req_list_b_tasks = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "list_tasks",
            "params": {
                "filters": [
                {
                    "project_id": project_id,
                    "title": {"$contains": "BB"},
                },
                // Shows how to use other $in
                {

                    "project_id": { "$in": [project_id] },
                    "title": {"$in": ["task AAA 3", "task AAA 4"]}
                },
                // This won't match any projects, so, won't return anything.
                {
                    "project_id": { "$in": [ 123, 124]},
                    "title": {"$in": ["task AAA 2", "task AAA 5"]}
                }
                ]
            }
        }),
    );
    req_list_b_tasks.await?.print().await?;

    let req_logoff = hc.do_post(
        "/api/logoff",
        json!({
            "logoff": true
        }),
    );
    // req_logoff.await?.print().await?;

    Ok(())
}

async fn check_activity() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:8081")?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "admin",
            "pwd": "welcome"
        }),
    );
    req_login.await?.print().await?;

    let req_create_activity = hc.do_post(
        "/api/rpc",
        json!({
            "id" : 1,
            "method": "create_activity",
            "params": {
                "data": {
                    "name": "Ejemplo de Actividad",
                     "sport_id": 123,
                    "category": "Senior",
                    "description": "Esta es una actividad de muestra.",
                    "multimedia_links": [
                        "https://ejemplo.com/imagen1.jpg",
                        "https://ejemplo.com/video.mp4"
                    ],
                    "rating": 4.5,
                    "tags": ["deporte", "aire libre"],
                    "user_id": 1
                }
            }
        }),
    );
    let activity_c_response = req_create_activity.await?;
    activity_c_response.print().await?;
    let activity_c_body = activity_c_response.json_body()?;
    let id = activity_c_body["result"]["_id"]["$oid"].as_str().unwrap();

    let req_get_activity = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "get_activity",
            "params": {
                "id": id
            }
        }),
    );

    req_get_activity.await?.print().await?;

    let req_list_activity = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "list_activities"
        }),
    );

    req_list_activity.await?.print().await?;

    let req_update_activity = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "update_activity",
            "params": {
                "id": id,
                "data": {
                    "category": "Senior",
                    "description": "Esta es una actividad de muestra MODIFICADA.",
                    "multimedia_links": [
                        "https://ejemplo.com/imagen1.jpg",
                        "https://ejemplo.com/video.mp4"
                    ],
                    "rating": 4.5,
                    "tags": ["deporte", "aire libre"],
                }
            }
        }),
    );
    req_update_activity.await?.print().await?;

    let req_delete_activity = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "delete_activity",
            "params": {
                "id": id
            }
        }),
    );
    req_delete_activity.await?.print().await?;

    let req_logoff = hc.do_post(
        "/api/logoff",
        json!({
            "logoff": true
        }),
    );

    req_logoff.await?.print().await?;
    Ok(())
}
