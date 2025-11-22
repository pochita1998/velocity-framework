//! Velocity Compiler
//!
//! Transforms JSX/TSX into optimized JavaScript that uses the Velocity runtime.
//!
//! ## Key Features
//! - Parse JSX/TSX using SWC (Rust-based, 10-40x faster than Babel)
//! - Static analysis of reactivity graphs
//! - Transform JSX → Direct DOM operations
//! - Optimization passes (dead code elimination, effect pruning, template cloning)
//! - Generate minimal, optimized JavaScript

pub mod parser;
pub mod analyzer;
pub mod transformer;
pub mod optimizer;
pub mod codegen;
pub mod error;

pub use error::{CompilerError, Result};
pub use codegen::GenerateResult;

/// Compiler configuration
#[derive(Debug, Clone)]
pub struct CompilerOptions {
    /// Whether to enable optimization passes
    pub optimize: bool,
    /// Whether to generate source maps
    pub source_maps: bool,
    /// Target environment (e.g., "es2015", "es2020")
    pub target: String,
    /// Whether to minify output
    pub minify: bool,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        Self {
            optimize: true,
            source_maps: true,
            target: "es2020".to_string(),
            minify: false,
        }
    }
}

/// Main compiler struct
pub struct Compiler {
    options: CompilerOptions,
}

impl Compiler {
    /// Create a new compiler with the given options
    pub fn new(options: CompilerOptions) -> Self {
        Self { options }
    }

    /// Create a new compiler with default options
    pub fn default() -> Self {
        Self {
            options: CompilerOptions::default(),
        }
    }

    /// Compile a single file from source code
    pub fn compile(&self, source: &str, filename: &str) -> Result<String> {
        // 1. Parse JSX/TSX → AST
        let module = parser::parse(source, filename)?;

        // 2. Analyze reactivity
        let analysis = analyzer::analyze(&module)?;

        // 3. Transform JSX → DOM operations
        let transformed = transformer::transform(module, &analysis)?;

        // 4. Optimize (if enabled)
        let optimized = if self.options.optimize {
            optimizer::optimize(transformed, &analysis)?
        } else {
            transformed
        };

        // 5. Generate JavaScript code
        let code = codegen::generate(&optimized, &self.options)?;

        Ok(code)
    }

    /// Compile with source map generation
    pub fn compile_with_source_map(&self, source: &str, filename: &str) -> Result<GenerateResult> {
        // 1. Parse JSX/TSX → AST
        let module = parser::parse(source, filename)?;

        // 2. Analyze reactivity
        let analysis = analyzer::analyze(&module)?;

        // 3. Transform JSX → DOM operations
        let transformed = transformer::transform(module, &analysis)?;

        // 4. Optimize (if enabled)
        let optimized = if self.options.optimize {
            optimizer::optimize(transformed, &analysis)?
        } else {
            transformed
        };

        // 5. Generate JavaScript code with source map
        codegen::generate_with_source_map(&optimized, &self.options, Some(filename))
    }

    /// Compile a file from disk
    pub fn compile_file(&self, path: &str) -> Result<String> {
        let source = std::fs::read_to_string(path)
            .map_err(|e| CompilerError::IoError(e.to_string()))?;

        self.compile(&source, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_simple_jsx() {
        let compiler = Compiler::default();
        let source = r#"
            function Counter() {
                const [count, setCount] = createSignal(0);
                return <div onClick={() => setCount(count() + 1)}>{count}</div>;
            }
        "#;

        let result = compiler.compile(source, "test.tsx");
        assert!(result.is_ok(), "Compilation should succeed");
    }
}
