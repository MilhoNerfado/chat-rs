#[allow(dead_code)]
pub struct User {
    username: String,
    nickname: String,
    uuid: u128,
}

#[allow(dead_code)]
pub struct Message {
    user: User,
    message: String,
}