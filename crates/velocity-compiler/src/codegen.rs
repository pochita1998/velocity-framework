//! Code Generation
//!
//! Generates JavaScript code from the optimized AST.

use crate::error::{CompilerError, Result};
use crate::CompilerOptions;
use swc_core::common::{sync::Lrc, SourceMap};
use swc_core::ecma::ast::Module;
use swc_core::ecma::codegen::{text_writer::JsWriter, Emitter, Config};

/// Generate JavaScript code from an AST module
pub fn generate(module: &Module, options: &CompilerOptions) -> Result<String> {
    let cm: Lrc<SourceMap> = Default::default();

    // Create output buffer
    let mut buf = vec![];

    {
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
    }

    String::from_utf8(buf)
        .map_err(|e| CompilerError::CodegenError(format!("Invalid UTF-8: {}", e)))
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
