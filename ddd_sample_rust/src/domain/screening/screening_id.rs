use crate::domain::error::MyError;
use std::fmt;
use ulid::Ulid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScreeningId(Ulid);

impl TryFrom<String> for ScreeningId {
    type Error = MyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match Ulid::from_string(&value) {
            Ok(ulid) => Ok(Self(ulid)),
            Err(_) => Err(MyError::ValidationError("ScreeningIdが不正です".into())),
        }
    }
}

impl fmt::Display for ScreeningId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ScreeningId {
    pub fn new() -> Self {
        Self(Ulid::new())
    }
}
