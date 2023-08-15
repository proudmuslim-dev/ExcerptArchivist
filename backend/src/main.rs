use color_eyre::eyre::Result;
use excerpt_archivist::api::Api;
use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;

type DbPool = sqlx::SqlitePool;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let pool = DbPool::connect("sqlite:devel.db").await?;

    let api_service = OpenApiService::new(Api, "ExcerptArchivist", "0.1").server("http://localhost:3000");
    let ui = api_service.swagger_ui();
    let spec = api_service.spec();

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(
            Route::new()
                .nest("/", api_service)
                .nest("/ui", ui)
                .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
                .with(Cors::new())
                .data(pool),
        )
        .await?;

    Ok(())
}
