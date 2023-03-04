use crate::domain::error::MyError;
use std::fmt;
use ulid::Ulid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositionId(Ulid);

impl TryFrom<String> for PositionId {
    type Error = MyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match Ulid::from_string(&value) {
            Ok(ulid) => Ok(Self(ulid)),
            Err(_) => Err(MyError::ValidationError("PositionIdが不正です".into())),
        }
    }
}

impl fmt::Display for PositionId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PositionId {
    pub fn new() -> Self {
        Self(Ulid::new())
    }
}
