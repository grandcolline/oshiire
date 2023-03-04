use crate::domain::error::MyError;
use regex::Regex;
use std::fmt;

/// ユーザエンティティ
#[derive(Debug, Clone)]
pub struct User {
    user_id: String,
    name: String,
    mail_addresses: Vec<MailAddress>,
}
// EntityはIDで同一性を担保する
impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id
    }
}
impl Eq for User {}

/// メールアドレス
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MailAddress(String);

impl TryFrom<String> for MailAddress {
    type Error = MyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // 書式のバリデーション
        let re = Regex::new(r"^[a-zA-Z0-9_.+-]+@([a-zA-Z0-9-]*\.)+[a-zA-Z]{2,}$").unwrap();
        if re.is_match(&value) {
            Ok(Self(value))
        } else {
            Err(MyError::ValidationError("MailAddressが不正です".into()))
        }
    }
}

impl fmt::Display for MailAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
