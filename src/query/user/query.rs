use crate::query::user::model::User;
use async_graphql::{FieldResult, Object};

#[derive(Default)]
pub struct UserQuery {}

#[Object]
impl UserQuery {
    async fn get_user(&self, _user_id: String) -> FieldResult<User> {
        Ok(User {
            id: String::from("id"),
            name: String::from("name"),
            avatar_url: String::from("url"),
        })
    }
}
