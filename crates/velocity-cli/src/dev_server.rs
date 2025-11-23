//! Development server with HMR support

use anyhow::Result;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use notify::{EventKind, RecursiveMode, Watcher};
use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::broadcast;
use tower_http::services::ServeDir;
use velocity_compiler::{Compiler, CompilerOptions};

/// HMR message types
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum HMRMessage {
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "update")]
    Update {
        module: String,
        code: String,
        timestamp: u64,
        /// Modules that depend on this one (for cascade updates)
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        dependents: Vec<String>,
    },
    #[serde(rename = "full-reload")]
    FullReload { reason: String },
    #[serde(rename = "error")]
    Error { error: String },
}

/// Dev server state
#[derive(Clone)]
pub struct DevServerState {
    /// Broadcast channel for HMR updates
    hmr_tx: broadcast::Sender<HMRMessage>,
    /// Root directory
    root: PathBuf,
    /// Compiler options
    compiler_options: CompilerOptions,
}

impl DevServerState {
    pub fn new(root: PathBuf) -> Self {
        let (hmr_tx, _) = broadcast::channel(100);
        Self {
            hmr_tx,
            root,
            compiler_options: CompilerOptions {
                optimize: true,
                source_maps: true,
                target: "es2020".to_string(),
                minify: false,
            },
        }
    }

    /// Broadcast an HMR update
    pub fn broadcast_update(&self, msg: HMRMessage) {
        let _ = self.hmr_tx.send(msg);
    }
}

/// Start the development server
pub async fn start_dev_server(port: u16, root: String) -> Result<()> {
    let root_path = PathBuf::from(&root);
    let state = Arc::new(DevServerState::new(root_path.clone()));

    // Start file watcher in background
    let watcher_state = state.clone();
    let watcher_root = root_path.clone();
    tokio::spawn(async move {
        if let Err(e) = start_file_watcher(watcher_state, watcher_root).await {
            eprintln!("File watcher error: {}", e);
        }
    });

    // Create router
    let app = Router::new()
        .route("/", get(serve_index))
        .route("/__hmr", get(ws_handler))
        .route("/__velocity/hmr-client.js", get(serve_hmr_client))
        .nest_service("/dist", ServeDir::new(root_path.join("dist")))
        .nest_service("/src", ServeDir::new(root_path.join("src")))
        .nest_service("/public", ServeDir::new(root_path.join("public")))
        .nest_service("/examples", ServeDir::new(root_path.join("examples")))
        .with_state(state);

    // Try to bind to the requested port, fallback if busy
    let mut current_port = port;
    let listener = loop {
        let addr = SocketAddr::from(([127, 0, 0, 1], current_port));
        match tokio::net::TcpListener::bind(addr).await {
            Ok(listener) => break listener,
            Err(_) if current_port < port + 10 => {
                println!("⚠️  Port {} is in use, trying {}...", current_port, current_port + 1);
                current_port += 1;
            }
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Failed to bind to port {} (tried ports {}-{}): {}",
                    port,
                    port,
                    current_port,
                    e
                ));
            }
        }
    };

    println!("🚀 Dev server starting on http://localhost:{}", current_port);
    println!("📁 Serving from: {}", root);
    println!("🔥 HMR enabled - changes will update instantly!\n");

    axum::serve(listener, app).await?;

    Ok(())
}

/// Serve the index.html with HMR client injected
async fn serve_index(State(state): State<Arc<DevServerState>>) -> impl IntoResponse {
    let index_path = state.root.join("index.html");

    if index_path.exists() {
        let html = tokio::fs::read_to_string(&index_path)
            .await
            .unwrap_or_else(|_| default_index());

        // Inject HMR client if not already present
        if html.contains("__velocity/hmr-client.js") {
            Html(html)
        } else {
            let injected = html.replace(
                "</body>",
                r#"<script type="module" src="/__velocity/hmr-client.js"></script>
</body>"#,
            );
            Html(injected)
        }
    } else {
        Html(default_index())
    }
}

/// Default index.html if none exists
fn default_index() -> String {
    r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Velocity App</title>
</head>
<body>
  <div id="root"></div>
  <script type="module" src="/dist/app.js"></script>
  <script type="module" src="/__velocity/hmr-client.js"></script>
</body>
</html>"#
        .to_string()
}

/// Serve the HMR client JavaScript
async fn serve_hmr_client() -> impl IntoResponse {
    let client_code = include_str!("hmr_client.js");
    (
        [("content-type", "application/javascript")],
        client_code,
    )
}

/// WebSocket handler for HMR
async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<DevServerState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

/// Handle WebSocket connection
async fn handle_socket(socket: WebSocket, state: Arc<DevServerState>) {
    let (mut sender, mut receiver) = socket.split();

    // Subscribe to HMR updates
    let mut rx = state.hmr_tx.subscribe();

    // Send connected message
    let connected_msg = serde_json::to_string(&HMRMessage::Connected).unwrap();
    let _ = sender.send(Message::Text(connected_msg)).await;

    // Spawn task to forward HMR updates to this client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let json = serde_json::to_string(&msg).unwrap();
            if sender.send(Message::Text(json)).await.is_err() {
                break;
            }
        }
    });

    // Receive messages from client (ping/pong, etc.)
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                // Handle client messages if needed
                println!("Client message: {}", text);
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
}

/// Start file watcher
async fn start_file_watcher(state: Arc<DevServerState>, root: PathBuf) -> Result<()> {
    use std::sync::mpsc::channel;

    let (tx, rx) = channel();
    let mut watcher = notify::recommended_watcher(tx)?;

    // Watch src directory
    let src_dir = root.join("src");
    if src_dir.exists() {
        watcher.watch(&src_dir, RecursiveMode::Recursive)?;
        println!("👀 Watching {}", src_dir.display());
    }

    // Watch index.html in root
    let index_html = root.join("index.html");
    if index_html.exists() {
        watcher.watch(&index_html, RecursiveMode::NonRecursive)?;
        println!("👀 Watching index.html");
    }

    // Watch for file changes
    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                match event.kind {
                    EventKind::Modify(_) | EventKind::Create(_) => {
                        for path in event.paths {
                            // Check if it's index.html
                            if path.file_name().and_then(|n| n.to_str()) == Some("index.html") {
                                use std::time::Instant;
                                let start = Instant::now();

                                state.broadcast_update(HMRMessage::FullReload {
                                    reason: "index.html updated".to_string(),
                                });

                                let elapsed = start.elapsed();
                                println!(
                                    "🔄 index.html → full reload in {:.2}ms",
                                    elapsed.as_secs_f64() * 1000.0
                                );
                            } else if let Some(ext) = path.extension() {
                                if ext == "tsx" || ext == "ts" || ext == "jsx" || ext == "js" {
                                    handle_file_change(&state, &path).await;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(Err(e)) => eprintln!("Watch error: {:?}", e),
            Err(e) => {
                eprintln!("Channel error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}

/// Handle file change - compile and broadcast update
async fn handle_file_change(state: &DevServerState, path: &Path) {
    use std::time::Instant;

    let start = Instant::now();
    println!("🔄 File changed: {}", path.display());

    // Read file
    let source = match tokio::fs::read_to_string(path).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ Failed to read file: {}", e);
            return;
        }
    };

    // Compile
    let compile_start = Instant::now();
    let compiler = Compiler::new(state.compiler_options.clone());
    match compiler.compile(&source, path.to_str().unwrap()) {
        Ok(code) => {
            let compile_time = compile_start.elapsed();

            // Get module path relative to root
            let module_path = path
                .strip_prefix(&state.root)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string();

            // Broadcast update
            // TODO: Implement dependency tracking to populate dependents
            state.broadcast_update(HMRMessage::Update {
                module: module_path.clone(),
                code,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
                dependents: vec![], // Will be populated with module dependency tracking
            });

            let total_time = start.elapsed();
            println!(
                "✅ {} → compiled in {:.2}ms, HMR in {:.2}ms (total: {:.2}ms)",
                module_path,
                compile_time.as_secs_f64() * 1000.0,
                (total_time - compile_time).as_secs_f64() * 1000.0,
                total_time.as_secs_f64() * 1000.0
            );
        }
        Err(e) => {
            eprintln!("❌ Compilation error: {}", e);
            state.broadcast_update(HMRMessage::Error {
                error: e.to_string(),
            });
        }
    }
}
