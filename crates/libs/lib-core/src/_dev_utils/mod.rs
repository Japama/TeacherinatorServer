// region:    --- Modules

use tokio::sync::OnceCell;
use tracing::info;

use crate::ctx::Ctx;
use crate::model::{self, ModelManager};
use crate::model::project::{ProjectBmc, ProjectForCreate};
use crate::model::department::{DepartmentBmc, DepartmentForCreate};
use crate::model::task::{Task, TaskBmc, TaskForCreate};
use crate::model::user::{UserBmc, UserForCreate};
use crate::model::teacher::{TeacherBmc, TeacherForCreate};

mod dev_db;

// endregion: --- Modules

/// Initialize environment for local development.
/// (for early development, will be called from main()).
pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        info!("{:<12} - init_dev_all()", "FOR-DEV-ONLY");

        dev_db::init_dev_db().await.unwrap();
    })
    .await;
}

/// Initialize test environment.
pub async fn init_test() -> ModelManager {
    static INIT: OnceCell<ModelManager> = OnceCell::const_new();

    let mm = INIT
        .get_or_init(|| async {
            init_dev().await;
            ModelManager::new().await.unwrap()
        })
        .await;

    mm.clone()
}

pub async fn seed_tasks(
    ctx: &Ctx,
    mm: &ModelManager,
    project_id: i64,
    titles: &[&str],
) -> model::Result<Vec<Task>> {
    let mut tasks = Vec::new();

    for title in titles {
        let id = TaskBmc::create(
            ctx,
            mm,
            TaskForCreate {
                project_id,
                title: title.to_string(),
            },
        )
        .await?;
        let task = TaskBmc::get(ctx, mm, id).await?;

        tasks.push(task);
    }

    Ok(tasks)
}

pub async fn seed_project(ctx: &Ctx, mm: &ModelManager, name: &str) -> model::Result<i64> {
    ProjectBmc::create(
        ctx,
        mm,
        ProjectForCreate {
            name: name.to_string(),
        },
    )
        .await
}

pub async fn seed_user(ctx: &Ctx, mm: &ModelManager, name: &str) -> model::Result<i64> {
    UserBmc::create(
        ctx,
        mm,
        UserForCreate {
            username: name.to_string(),
            pwd: "pwd".to_string(),
            isadmin: false
        },
    )
        .await
}

pub async fn seed_department(ctx: &Ctx, mm: &ModelManager, name: &str) -> model::Result<i64> {
    DepartmentBmc::create(
        ctx,
        mm,
        DepartmentForCreate {
            name: name.to_string()
        },
    )
        .await
}


pub async fn seed_teacher(ctx: &Ctx, mm: &ModelManager, name: &str , department_id: i64) -> model::Result<i64> {
    let user_id = UserBmc::create(
        ctx,
        mm,
        UserForCreate {
            username: name.to_string(),
            pwd: "pwd".to_string(),
            isadmin: false
        },
    ).await?;
    
    TeacherBmc::create(
        ctx,
        mm,
        TeacherForCreate {
            name: name.to_string(),
            active: true,
            department_id,
            user_id,
        },
    )
        .await
}
