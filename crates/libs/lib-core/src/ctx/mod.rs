// region:    --- Modules

pub use self::error::{Error, Result};

mod error;

// endregion: --- Modules

#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: i64,
    isadmin: bool
}

// Constructors.
impl Ctx {
    pub fn root_ctx() -> Self {
        Ctx { user_id: 0 , isadmin: true}
    }

    pub fn new(user_id: i64, isadmin: bool) -> Result<Self> {
        if user_id == 0 {
            Err(Error::CtxCannotNewRootCtx)
        } else {
            Ok(Self { user_id, isadmin })
        }
    }
}

// Property Accessors.
impl Ctx {
    pub fn user_id(&self) -> i64 {
        self.user_id
    }
    pub fn admin(&self) -> bool {
        self.isadmin
    }
}
