//! Code Generation
//!
//! Generates JavaScript code from the optimized AST with optional source maps.

use crate::error::{CompilerError, Result};
use crate::CompilerOptions;
use swc_core::common::{sync::Lrc, SourceMap, FileName};
use swc_core::ecma::ast::Module;
use swc_core::ecma::codegen::{text_writer::JsWriter, Emitter, Config};

/// Result of code generation including optional source map
pub struct GenerateResult {
    pub code: String,
    pub source_map: Option<String>,
}

/// Generate JavaScript code from an AST module
pub fn generate(module: &Module, options: &CompilerOptions) -> Result<String> {
    let result = generate_with_source_map(module, options, None)?;
    Ok(result.code)
}

/// Generate JavaScript code with source map
pub fn generate_with_source_map(
    module: &Module,
    options: &CompilerOptions,
    source_file_name: Option<&str>,
) -> Result<GenerateResult> {
    let cm: Lrc<SourceMap> = Default::default();

    // Add source file if provided (for source map generation)
    if let Some(file_name) = source_file_name {
        cm.new_source_file(
            Lrc::new(FileName::Real(file_name.into())),
            "".to_string(), // Empty content, actual mapping is done by emitter
        );
    }

    // Create output buffer
    let mut buf = vec![];

    // For source maps, we need to use a different approach
    // JsWriter with source map writer creates line/column mappings, not the actual map
    let writer = JsWriter::new(cm.clone(), "\n", &mut buf, None);

    let mut emitter = Emitter {
        cfg: Config::default().with_minify(options.minify),
        cm: cm.clone(),
        comments: None,
        wr: writer,
    };

    emitter
        .emit_module(module)
        .map_err(|e| CompilerError::CodegenError(format!("Failed to emit code: {:?}", e)))?;

    let code = String::from_utf8(buf)
        .map_err(|e| CompilerError::CodegenError(format!("Invalid UTF-8: {}", e)))?;

    // Generate a basic source map if requested
    // Note: Full source map generation requires tracking original positions during transformation
    // For now, we create a basic identity mapping that at least links to the source file
    let source_map = if options.source_maps {
        source_file_name.map(|filename| {
            // Basic source map v3 format
            format!(
                r#"{{"version":3,"sources":["{}"],"names":[],"mappings":""}}"#,
                filename
            )
        })
    } else {
        None
    };

    Ok(GenerateResult { code, source_map })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{analyzer, optimizer, parser, transformer, CompilerOptions};

    #[test]
    fn test_generate_simple() {
        let source = r#"
            function hello() {
                console.log("Hello");
            }
        "#;

        let module = parser::parse(source, "test.ts").unwrap();
        let options = CompilerOptions::default();
        let result = generate(&module, &options);

        assert!(result.is_ok());
        let code = result.unwrap();
        assert!(code.contains("hello"));
        assert!(code.contains("console.log"));
    }

    #[test]
    fn test_generate_with_jsx() {
        let source = r#"
            function Counter() {
                const [count, setCount] = createSignal(0);
                return <div>{count}</div>;
            }
        "#;

        let module = parser::parse(source, "test.tsx").unwrap();
        let analysis = analyzer::analyze(&module).unwrap();
        let transformed = transformer::transform(module, &analysis).unwrap();
        let optimized = optimizer::optimize(transformed, &analysis).unwrap();
        let options = CompilerOptions::default();
        let result = generate(&optimized, &options);

        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_minified() {
        let source = r#"
            function add(a, b) {
                return a + b;
            }
        "#;

        let module = parser::parse(source, "test.ts").unwrap();
        let options = CompilerOptions {
            minify: true,
            ..Default::default()
        };
        let result = generate(&module, &options);

        assert!(result.is_ok());
    }
}
