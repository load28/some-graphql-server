use actix_web::{
    guard,
    web::{self},
    App, HttpResponse, HttpServer,
};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, FieldResult, Object, Schema, SimpleObject};
use async_graphql_actix_web::GraphQL;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
struct Owner {
    pub id: String,
    pub name: String,
}
struct Query;

#[Object]
impl Query {
    async fn get_data(&self) -> FieldResult<Owner> {
        Ok(Owner {
            id: String::from("hi"),
            name: String::from("hi"),
        })
    }
}

struct Mutation;

#[Object]
impl Mutation {
    async fn create_data(&self) -> FieldResult<Owner> {
        Ok(Owner {
            id: String::from("hi"),
            name: String::from("hi"),
        })
    }
}

async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let schema = Schema::build(Query, Mutation, EmptySubscription).finish();

        App::new()
            .service(
                web::resource("/")
                    .guard(guard::Post())
                    .to(GraphQL::new(schema)),
            )
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .to(graphql_playground),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
