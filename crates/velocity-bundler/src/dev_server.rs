use anyhow::Result;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

pub struct DevServer {
    port: u16,
    root: String,
}

impl DevServer {
    pub fn new(port: u16, root: String) -> Self {
        Self { port, root }
    }

    pub async fn start(self) -> Result<()> {
        let app = Router::new()
            .route("/", get(serve_index))
            .route("/@velocity/client", get(serve_client))
            .route("/src/*path", get(serve_module))
            .nest_service("/public", ServeDir::new(format!("{}/public", self.root)));

        let addr = format!("127.0.0.1:{}", self.port);
        println!("  ➜  Local:   http://{}", addr);

        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}

async fn serve_index() -> Html<&'static str> {
    Html(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Velocity App</title>
</head>
<body>
    <div id="root"></div>
    <script type="module">
        import { render } from '/@velocity/client';
        import App from '/src/index.tsx';
        render(App, document.getElementById('root'));
    </script>
</body>
</html>"#)
}

async fn serve_client() -> impl IntoResponse {
    (
        [("Content-Type", "application/javascript")],
        include_str!("../../velocity-wasm/pkg/velocity_wasm.js")
    )
}

async fn serve_module() -> impl IntoResponse {
    // Simplified - in production would read file and transform
    (
        [("Content-Type", "application/javascript")],
        "export default function App() { return 'Hello Velocity'; }"
    )
}
