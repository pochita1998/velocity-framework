//! Error types for the Velocity compiler

use thiserror::Error;

/// Result type alias for compiler operations
pub type Result<T> = std::result::Result<T, CompilerError>;

/// Compiler error types
#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Analysis error: {0}")]
    AnalysisError(String),

    #[error("Transform error: {0}")]
    TransformError(String),

    #[error("Optimization error: {0}")]
    OptimizationError(String),

    #[error("Codegen error: {0}")]
    CodegenError(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Invalid JSX: {0}")]
    InvalidJsx(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}
