use hands_on_lib::construct_app;

#[tokio::main]
async fn main() {
    let app = construct_app();
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

