//! JSX/TSX Parser using SWC
//!
//! Parses JavaScript/TypeScript files with JSX syntax into an AST.

use crate::error::{CompilerError, Result};
use swc_core::common::{
    sync::Lrc,
    SourceMap, FileName,
};
use swc_core::ecma::ast::Module;
use swc_core::ecma::parser::{lexer::Lexer, Parser, StringInput, Syntax, TsSyntax, EsSyntax};

/// Parse a source file into a SWC AST
pub fn parse(source: &str, filename: &str) -> Result<Module> {
    // Create a source map for error reporting
    let cm: Lrc<SourceMap> = Default::default();

    // Add the source file to the source map
    let fm = cm.new_source_file(
        FileName::Custom(filename.to_string()).into(),
        source.to_string(),
    );

    // Create lexer for parsing
    let lexer = Lexer::new(
        // Use TypeScript syntax with JSX enabled
        Syntax::Typescript(TsSyntax {
            tsx: true,
            decorators: true,
            dts: false,
            no_early_errors: false,
            disallow_ambiguous_jsx_like: true,
        }),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    // Create parser
    let mut parser = Parser::new_from(lexer);

    // Parse the module
    parser
        .parse_module()
        .map_err(|e| {
            // Format the error with source context
            let error_msg = format!("Failed to parse {}: {:?}", filename, e);
            CompilerError::ParseError(error_msg)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_jsx() {
        let source = r#"
            function Hello() {
                return <div>Hello World</div>;
            }
        "#;

        let result = parse(source, "test.tsx");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_jsx_with_props() {
        let source = r#"
            function Button({ onClick, children }) {
                return <button onClick={onClick}>{children}</button>;
            }
        "#;

        let result = parse(source, "test.tsx");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_jsx_with_signals() {
        let source = r#"
            function Counter() {
                const [count, setCount] = createSignal(0);
                return <div onClick={() => setCount(count() + 1)}>{count}</div>;
            }
        "#;

        let result = parse(source, "test.tsx");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_typescript() {
        let source = r#"
            interface Props {
                name: string;
            }

            function Greeting({ name }: Props) {
                return <h1>Hello {name}</h1>;
            }
        "#;

        let result = parse(source, "test.tsx");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_invalid_syntax() {
        let source = r#"
            function Invalid() {
                return <div>
            }
        "#;

        let result = parse(source, "test.tsx");
        assert!(result.is_err());
    }
}
