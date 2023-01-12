use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use salvo::prelude::*;
use schema::query::QueryRoot;

#[handler]
async fn graphiql(res: &mut Response) {
    res.render(Text::Html(
        GraphiQLSource::build().endpoint("/graphql").finish(),
    ));
}

#[handler]
async fn graphql(req: &mut Request, res: &mut Response) {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();
    let data = req.parse_json::<async_graphql::Request>().await.unwrap();
    let response = schema.execute(data).await;
    res.render(Json(response))
}

#[tokio::main]
async fn main() {
    let router = Router::new().push(Router::with_path("graphql").get(graphiql).post(graphql));
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .serve(router)
        .await;
}
