mod user;

use crate::query::user::query::UserQuery;
use async_graphql::Object;

#[derive(Default)]
pub struct RootQuery {
    user_query: UserQuery,
}

#[Object]
impl RootQuery {
    async fn user(&self) -> &UserQuery {
        &self.user_query
    }
}
