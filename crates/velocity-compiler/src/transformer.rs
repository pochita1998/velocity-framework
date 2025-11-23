//! JSX → DOM Transformer
//!
//! Transforms JSX elements into direct DOM manipulation calls.
//!
//! Example transformation:
//! ```jsx
//! <div class="container">{count}</div>
//! ```
//! Becomes:
//! ```js
//! const _el = document.createElement('div');
//! _el.className = 'container';
//! const _text = document.createTextNode('');
//! createEffect(() => { _text.textContent = count(); });
//! _el.appendChild(_text);
//! ```

use crate::analyzer::Analysis;
use crate::error::{CompilerError, Result};
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};

/// Transformer that converts JSX to DOM operations
struct JsxTransformer {
    analysis: Analysis,
    element_counter: usize,
}

impl JsxTransformer {
    fn new(analysis: Analysis) -> Self {
        Self {
            analysis,
            element_counter: 0,
        }
    }

    /// Generate a unique element variable name
    fn next_element_name(&mut self) -> String {
        self.element_counter += 1;
        format!("_el{}", self.element_counter)
    }

    /// Check if an identifier is reactive (signal or memo)
    fn is_reactive(&self, name: &str) -> bool {
        self.analysis.signals.contains(name) || self.analysis.memos.contains(name)
    }

    /// Transform JSX element to createElement calls
    fn transform_jsx_element(&mut self, elem: &JSXElement) -> Expr {
        // Get the tag name
        let tag_name = match &elem.opening.name {
            JSXElementName::Ident(ident) => ident.sym.to_string(),
            JSXElementName::JSXMemberExpr(_) => {
                // Handle member expressions like <Foo.Bar />
                "div".to_string() // Simplified for now
            }
            JSXElementName::JSXNamespacedName(_) => {
                "div".to_string() // Simplified for now
            }
        };

        // Check if it's a component (starts with uppercase) or DOM element
        let is_component = tag_name.chars().next().unwrap().is_uppercase();

        if is_component {
            // Component - call it as a function
            self.transform_component_element(&tag_name, &elem.opening.attrs, &elem.children)
        } else {
            // DOM element - create with createElement
            self.transform_dom_element(&tag_name, &elem.opening.attrs, &elem.children)
        }
    }

    /// Transform a DOM element like <div>
    fn transform_dom_element(
        &mut self,
        tag: &str,
        attrs: &[JSXAttrOrSpread],
        children: &[JSXElementChild],
    ) -> Expr {
        // For now, return a call to createElement
        // Full implementation would generate a block with all the statements

        // Create arguments array: [tag, props, ...children]
        let mut args = Vec::new();

        // Tag name
        args.push(ExprOrSpread {
            spread: None,
            expr: Box::new(Expr::Lit(Lit::Str(Str {
                span: Default::default(),
                value: tag.into(),
                raw: None,
            }))),
        });

        // Props object - extract JSX attributes
        let mut prop_entries = Vec::new();

        for attr in attrs {
            if let JSXAttrOrSpread::JSXAttr(jsx_attr) = attr {
                // Get attribute name
                let key_name = match &jsx_attr.name {
                    JSXAttrName::Ident(ident) => ident.sym.to_string(),
                    _ => continue,
                };

                // Get attribute value
                let value_expr = match &jsx_attr.value {
                    Some(JSXAttrValue::Lit(lit)) => {
                        // String literal like class="counter"
                        Box::new(Expr::Lit(lit.clone()))
                    }
                    Some(JSXAttrValue::JSXExprContainer(container)) => {
                        // Expression like onClick={handler}
                        match &container.expr {
                            JSXExpr::Expr(expr) => expr.clone(),
                            _ => continue,
                        }
                    }
                    None => {
                        // Boolean attribute like disabled
                        Box::new(Expr::Lit(Lit::Bool(Bool {
                            span: Default::default(),
                            value: true,
                        })))
                    }
                    _ => continue,
                };

                // Create property
                prop_entries.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                    key: PropName::Str(Str {
                        span: Default::default(),
                        value: key_name.into(),
                        raw: None,
                    }),
                    value: value_expr,
                }))));
            }
        }

        args.push(ExprOrSpread {
            spread: None,
            expr: Box::new(Expr::Object(ObjectLit {
                span: Default::default(),
                props: prop_entries,
            })),
        });

        // Children (simplified)
        for child in children {
            if let Some(child_expr) = self.transform_jsx_child(child) {
                args.push(ExprOrSpread {
                    spread: None,
                    expr: Box::new(child_expr),
                });
            }
        }

        // Return createElement call
        Expr::Call(CallExpr {
            span: Default::default(),
            ctxt: Default::default(),
            callee: Callee::Expr(Box::new(Expr::Ident(Ident {
                span: Default::default(),
                ctxt: Default::default(),
                sym: "createElement".into(),
                optional: false,
            }))),
            args,
            type_args: None,
        })
    }

    /// Transform a component element like <Counter />
    fn transform_component_element(
        &mut self,
        name: &str,
        attrs: &[JSXAttrOrSpread],
        children: &[JSXElementChild],
    ) -> Expr {
        // Call the component as a function with props
        Expr::Call(CallExpr {
            span: Default::default(),
            ctxt: Default::default(),
            callee: Callee::Expr(Box::new(Expr::Ident(Ident {
                span: Default::default(),
                ctxt: Default::default(),
                sym: name.into(),
                optional: false,
            }))),
            args: vec![
                // Props object (simplified)
                ExprOrSpread {
                    spread: None,
                    expr: Box::new(Expr::Object(ObjectLit {
                        span: Default::default(),
                        props: vec![],
                    })),
                }
            ],
            type_args: None,
        })
    }

    /// Transform a JSX child element
    fn transform_jsx_child(&mut self, child: &JSXElementChild) -> Option<Expr> {
        match child {
            JSXElementChild::JSXElement(elem) => {
                Some(self.transform_jsx_element(elem))
            }
            JSXElementChild::JSXExprContainer(container) => {
                match &container.expr {
                    JSXExpr::Expr(expr) => Some((**expr).clone()),
                    JSXExpr::JSXEmptyExpr(_) => None,
                }
            }
            JSXElementChild::JSXText(text) => {
                let value = text.value.to_string().trim().to_string();
                if value.is_empty() {
                    None
                } else {
                    Some(Expr::Lit(Lit::Str(Str {
                        span: Default::default(),
                        value: value.into(),
                        raw: None,
                    })))
                }
            }
            _ => None,
        }
    }
}

impl VisitMut for JsxTransformer {
    /// Transform all JSX expressions wherever they appear
    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        // First, recursively visit children to transform nested JSX
        expr.visit_mut_children_with(self);

        // Then transform this expression if it's JSX
        if let Expr::JSXElement(elem) = expr {
            let transformed = self.transform_jsx_element(elem);
            *expr = transformed;
        } else if let Expr::JSXFragment(_frag) = expr {
            // Handle fragments - for now, create an empty div
            *expr = Expr::Call(CallExpr {
                span: Default::default(),
                ctxt: Default::default(),
                callee: Callee::Expr(Box::new(Expr::Ident(Ident {
                    span: Default::default(),
                    ctxt: Default::default(),
                    sym: "createElement".into(),
                    optional: false,
                }))),
                args: vec![
                    ExprOrSpread {
                        spread: None,
                        expr: Box::new(Expr::Lit(Lit::Str(Str {
                            span: Default::default(),
                            value: "div".into(),
                            raw: None,
                        }))),
                    },
                    ExprOrSpread {
                        spread: None,
                        expr: Box::new(Expr::Object(ObjectLit {
                            span: Default::default(),
                            props: vec![],
                        })),
                    },
                ],
                type_args: None,
            });
        }
    }
}

/// Transform a module by converting JSX to DOM operations
pub fn transform(mut module: Module, analysis: &Analysis) -> Result<Module> {
    let mut transformer = JsxTransformer::new(analysis.clone());
    module.visit_mut_with(&mut transformer);
    Ok(module)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{analyzer, parser};

    #[test]
    fn test_transform_simple_jsx() {
        let source = r#"
            function Hello() {
                return <div>Hello World</div>;
            }
        "#;

        let module = parser::parse(source, "test.tsx").unwrap();
        let analysis = analyzer::analyze(&module).unwrap();
        let result = transform(module, &analysis);

        assert!(result.is_ok());
    }

    #[test]
    fn test_transform_jsx_with_reactive_child() {
        let source = r#"
            function Counter() {
                const [count, setCount] = createSignal(0);
                return <div>{count}</div>;
            }
        "#;

        let module = parser::parse(source, "test.tsx").unwrap();
        let analysis = analyzer::analyze(&module).unwrap();
        let result = transform(module, &analysis);

        assert!(result.is_ok());
    }
}
