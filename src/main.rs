use api::{schema::query::QueryRoot, Database};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use dotenvy::dotenv;
use salvo::logging::Logger;
use salvo::prelude::*;

#[handler]
async fn graphiql_playground(res: &mut Response) {
    res.render(Text::Html(
        GraphiQLSource::build().endpoint("/graphql").finish(),
    ));
}

#[handler]
async fn graphql(req: &mut Request, res: &mut Response) {
    let schema = build_schema().await;
    let data = req.parse_json::<async_graphql::Request>().await.unwrap();
    let response = schema.execute(data).await;
    res.render(Json(response))
}

async fn build_schema() -> Schema<QueryRoot, EmptyMutation, EmptySubscription> {
    let db = Database::new().await;
    //TODO: migration 処理

    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(db)
        .finish()
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    dotenv().ok();

    let router = Router::new().hoop(Logger).push(
        Router::with_path("graphql")
            .get(graphiql_playground)
            .post(graphql),
    );
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .serve(router)
        .await;
}
