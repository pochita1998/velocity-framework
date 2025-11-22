mod dev_server;
mod bundler;
mod module_graph;

pub use dev_server::DevServer;
pub use bundler::{Bundler, BundlerConfig};

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Module {
    pub path: PathBuf,
    pub content: String,
    pub transformed: String,
    pub dependencies: Vec<String>,
}
