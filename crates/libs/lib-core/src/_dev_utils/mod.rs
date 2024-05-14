// region:    --- Modules

use time::Time;
use tokio::sync::OnceCell;
use tracing::info;

use crate::ctx::Ctx;
use crate::model::{self, ModelManager};
use crate::model::center_schedule_hour::{CenterScheduleHourBmc, CenterScheduleHourForCreate};
use crate::model::classroom::{ClassroomBmc, ClassroomForCreate};
use crate::model::department::{DepartmentBmc, DepartmentForCreate};
use crate::model::group::{GroupBmc, GroupForCreate};
use crate::model::schedule::{ScheduleBmc, ScheduleForCreate};
use crate::model::schedule_hour::{ScheduleHourBmc, ScheduleHourForCreate};
use crate::model::subject::{SubjectBmc, SubjectForCreate};
use crate::model::user::{UserBmc, UserForCreate};

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
            is_admin: false,
            department_id: None,
            active: true,
            substituting_id: None
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


pub async fn seed_group(ctx: &Ctx, mm: &ModelManager, letter: &str, course: i32, stage: i32, year: i32, tutor_name: String) -> model::Result<i64> {
    GroupBmc::create(
        ctx,
        mm,
        GroupForCreate {
            course,
            stage,
            year,
            tutor_name,
            letter: letter.to_string(),
        },
    )
        .await
}


pub async fn seed_classroom(ctx: &Ctx, mm: &ModelManager, building: &str, floor: i32, number: i32, name: &str, type_c: i32, description: &str) -> model::Result<i64> {
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

pub async fn seed_schedule(ctx: &Ctx, mm: &ModelManager, course: i32, teacher_id: i64, group_id: i64) -> model::Result<i64> {
    let teacher = if teacher_id == -1 { None } else { Some(teacher_id) };
    let group= if group_id == -1 { None } else { Some(group_id) };

    ScheduleBmc::create(
        ctx,
        mm,
        ScheduleForCreate {
            course,
            user_id: teacher,
            group_id: group
        },
    )
        .await
}


pub async fn seed_schedule_hour(ctx: &Ctx, mm: &ModelManager, schedule_id: i64, subject_name: &str, classroom_name: &str, week_day: i32, n_hour: i32, course: i32) -> model::Result<i64> {
    ScheduleHourBmc::create(
        ctx,
        mm,
        ScheduleHourForCreate {
            schedule_id,
            subject_name: subject_name.to_string(),
            classroom_name: classroom_name.to_string(),
            week_day,
            n_hour,
            course,
            notes: Some("".to_string())
        },
    )
        .await
}

pub async fn seed_center_schedule_hour(ctx: &Ctx, mm: &ModelManager, week_day: i32, n_hour: i32, start_time: Time, end_time: Time, course: i32) -> model::Result<i64> {
    CenterScheduleHourBmc::create(
        ctx,
        mm,
        CenterScheduleHourForCreate {
            n_hour,
            start_time,
            end_time,
        },
    )
        .await
}