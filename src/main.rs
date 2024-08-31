mod query;

use crate::query::RootQuery;
use actix_web::{
    guard,
    web::{self},
    App, HttpResponse, HttpServer,
};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::GraphQL;

async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

/*
Todo list
1. 게시물 인터페이스 생성
2. 유저 추가/수정/삭제 뮤테이션 추가
3. 게시물 추가/삭제/수정 뮤테이션 추가
4. 아마존 R디비 연동
 */

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let schema = Schema::build(RootQuery::default(), EmptyMutation, EmptySubscription).finish();

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
