/// ユーザテーブル
#[derive(Debug, Clone)]
pub struct User {
    user_id: String,
    name: String,
    mail_addresses: Vec<UserMailAddress>,
}

/// メールアドレステーブル
#[derive(Debug, Clone)]
pub struct UserMailAddress {
    user: User,
    mail_addresses: String,
}
