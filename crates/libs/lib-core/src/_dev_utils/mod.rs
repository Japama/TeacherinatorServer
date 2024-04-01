// region:    --- Modules

use tokio::sync::OnceCell;
use tracing::info;

use crate::ctx::Ctx;
use crate::model::{self, ModelManager};
use crate::model::classroom::{ClassroomBmc, ClassroomForCreate};
use crate::model::department::{DepartmentBmc, DepartmentForCreate};
use crate::model::group::{GroupBmc, GroupForCreate};
use crate::model::schedule::{ScheduleBmc, ScheduleForCreate};
use crate::model::subject::{SubjectBmc, SubjectForCreate};
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


pub async fn seed_teacher(ctx: &Ctx, mm: &ModelManager, name: &str , department_id: i64, user_id: i64) -> model::Result<i64> {
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

pub async fn seed_subject(ctx: &Ctx, mm: &ModelManager, name: &str, department_id: i64, is_guard: bool, is_complementary: bool) -> model::Result<i64> {
    SubjectBmc::create(
        ctx,
        mm,
        SubjectForCreate {
            name: name.to_string(),
            department_id,
            is_guard,
            is_complementary
        },
    )
        .await
}


pub async fn seed_group(ctx: &Ctx, mm: &ModelManager, letter: &str, course: i32, stage: i32, year: i32, tutor_id: i64) -> model::Result<i64> {
    GroupBmc::create(
        ctx,
        mm,
        GroupForCreate {
            course,
            stage,
            year,
            tutor_id,
            letter: letter.to_string(),
        },
    )
        .await
}


pub async fn seed_classroom(ctx: &Ctx, mm: &ModelManager, building: &str, floor: i32, number: i32, name: &str, type_c: i32, description: &str)  -> model::Result<i64> {
    ClassroomBmc::create(
        ctx,
        mm,
        ClassroomForCreate {
            building: building.to_string(),
            floor,
            number,
            name: name.to_string(),
            type_c,
            description: description.to_string(),
        },
    )
        .await
}

pub async fn seed_schedule(ctx: &Ctx, mm: &ModelManager, course: i32, teacher_id: Option<i64>, group_id: Option<i64>)  -> model::Result<i64> {
    ScheduleBmc::create(
        ctx,
        mm,
        ScheduleForCreate {
            course,
            teacher_id,
            group_id
        },
    )
        .await
}