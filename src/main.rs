use anyhow::Ok;
use tokio::signal;

pub mod api;
pub mod appctx;
pub mod errors;
pub mod log;
pub mod service;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log::init();
    let ctx = appctx::Context::new_static_from_path("conf/application.yaml").await?;
    let port = ctx
        .get_environment()
        .get_string("server.port")
        .unwrap_or("8000".to_string());
    let app = axum::Router::new().merge(api::markets(ctx));
    let app = log::axum_tracing_layer(app);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
