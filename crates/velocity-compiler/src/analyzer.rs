//! Static Analysis for Reactivity
//!
//! Analyzes the AST to determine:
//! - Which values are reactive (signals, memos)
//! - Which JSX elements need to be reactive
//! - Dependency graphs for effects
//! - Optimization opportunities

use crate::error::Result;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{Visit, VisitWith};
use std::collections::{HashMap, HashSet};

/// Analysis results
#[derive(Debug, Clone)]
pub struct Analysis {
    /// Set of identifiers that are reactive signals
    pub signals: HashSet<String>,

    /// Set of identifiers that are computed/memos
    pub memos: HashSet<String>,

    /// Set of identifiers that are effects
    pub effects: HashSet<String>,

    /// Map of JSX elements to their reactive dependencies
    pub jsx_dependencies: HashMap<usize, Vec<String>>,

    /// Set of function names that create reactivity
    pub reactive_functions: HashSet<String>,
}

impl Default for Analysis {
    fn default() -> Self {
        let mut reactive_functions = HashSet::new();
        // Velocity API
        reactive_functions.insert("createSignal".to_string());
        reactive_functions.insert("createMemo".to_string());
        reactive_functions.insert("createEffect".to_string());
        reactive_functions.insert("createResource".to_string());

        // React API (drop-in replacement)
        reactive_functions.insert("useState".to_string());
        reactive_functions.insert("useMemo".to_string());
        reactive_functions.insert("useEffect".to_string());
        reactive_functions.insert("useCallback".to_string());

        Self {
            signals: HashSet::new(),
            memos: HashSet::new(),
            effects: HashSet::new(),
            jsx_dependencies: HashMap::new(),
            reactive_functions,
        }
    }
}

/// Visitor that analyzes reactivity in the AST
struct ReactivityAnalyzer {
    analysis: Analysis,
    current_jsx_key: usize,
}

impl ReactivityAnalyzer {
    fn new() -> Self {
        Self {
            analysis: Analysis::default(),
            current_jsx_key: 0,
        }
    }

    /// Check if a call expression creates a signal (Velocity or React API)
    fn is_create_signal(&self, callee: &Callee) -> bool {
        if let Callee::Expr(expr) = callee {
            if let Expr::Ident(ident) = &**expr {
                let name = ident.sym.as_ref();
                return name == "createSignal" || name == "useState";
            }
        }
        false
    }

    /// Check if a call expression creates a memo (Velocity or React API)
    fn is_create_memo(&self, callee: &Callee) -> bool {
        if let Callee::Expr(expr) = callee {
            if let Expr::Ident(ident) = &**expr {
                let name = ident.sym.as_ref();
                return name == "createMemo" || name == "useMemo" || name == "useCallback";
            }
        }
        false
    }

    /// Check if a call expression creates an effect (Velocity or React API)
    fn is_create_effect(&self, callee: &Callee) -> bool {
        if let Callee::Expr(expr) = callee {
            if let Expr::Ident(ident) = &**expr {
                let name = ident.sym.as_ref();
                return name == "createEffect" || name == "useEffect";
            }
        }
        false
    }

    /// Extract identifier from a pattern (e.g., destructuring)
    fn extract_identifiers(&self, pat: &Pat, identifiers: &mut Vec<String>) {
        match pat {
            Pat::Ident(ident) => {
                identifiers.push(ident.id.sym.to_string());
            }
            Pat::Array(array) => {
                for elem in &array.elems {
                    if let Some(elem) = elem {
                        self.extract_identifiers(elem, identifiers);
                    }
                }
            }
            Pat::Object(obj) => {
                for prop in &obj.props {
                    match prop {
                        ObjectPatProp::KeyValue(kv) => {
                            self.extract_identifiers(&kv.value, identifiers);
                        }
                        ObjectPatProp::Assign(assign) => {
                            identifiers.push(assign.key.sym.to_string());
                        }
                        ObjectPatProp::Rest(rest) => {
                            self.extract_identifiers(&rest.arg, identifiers);
                        }
                    }
                }
            }
            Pat::Rest(rest) => {
                self.extract_identifiers(&rest.arg, identifiers);
            }
            Pat::Assign(assign) => {
                self.extract_identifiers(&assign.left, identifiers);
            }
            _ => {}
        }
    }
}

impl Visit for ReactivityAnalyzer {
    /// Visit variable declarations to find signals, memos, and effects
    fn visit_var_declarator(&mut self, decl: &VarDeclarator) {
        if let Some(init) = &decl.init {
            if let Expr::Call(call) = &**init {
                let mut identifiers = Vec::new();
                self.extract_identifiers(&decl.name, &mut identifiers);

                if self.is_create_signal(&call.callee) {
                    // createSignal returns [getter, setter]
                    // Usually destructured as: const [count, setCount] = createSignal(0)
                    if identifiers.len() >= 1 {
                        self.analysis.signals.insert(identifiers[0].clone());
                    }
                } else if self.is_create_memo(&call.callee) {
                    for ident in identifiers {
                        self.analysis.memos.insert(ident);
                    }
                } else if self.is_create_effect(&call.callee) {
                    for ident in identifiers {
                        self.analysis.effects.insert(ident);
                    }
                }
            }
        }

        decl.visit_children_with(self);
    }

    /// Visit JSX elements to track dependencies
    fn visit_jsx_element(&mut self, elem: &JSXElement) {
        self.current_jsx_key += 1;
        let key = self.current_jsx_key;

        // Track dependencies for this JSX element
        let deps = Vec::new();

        // Visit children and attributes to find reactive dependencies
        // This is a simplified version - full implementation would track
        // all identifiers used in the JSX that are reactive

        self.analysis.jsx_dependencies.insert(key, deps);

        elem.visit_children_with(self);
    }

    /// Visit call expressions
    fn visit_call_expr(&mut self, call: &CallExpr) {
        call.visit_children_with(self);
    }
}

/// Analyze a module for reactivity
pub fn analyze(module: &Module) -> Result<Analysis> {
    let mut analyzer = ReactivityAnalyzer::new();
    module.visit_with(&mut analyzer);
    Ok(analyzer.analysis)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn test_analyze_signals() {
        let source = r#"
            function Counter() {
                const [count, setCount] = createSignal(0);
                const [name, setName] = createSignal("test");
                return <div>{count}</div>;
            }
        "#;

        let module = parser::parse(source, "test.tsx").unwrap();
        let analysis = analyze(&module).unwrap();

        assert!(analysis.signals.contains("count"));
        assert!(analysis.signals.contains("name"));
    }

    #[test]
    fn test_analyze_memos() {
        let source = r#"
            function App() {
                const [count, setCount] = createSignal(0);
                const doubled = createMemo(() => count() * 2);
                return <div>{doubled}</div>;
            }
        "#;

        let module = parser::parse(source, "test.tsx").unwrap();
        let analysis = analyze(&module).unwrap();

        assert!(analysis.signals.contains("count"));
        assert!(analysis.memos.contains("doubled"));
    }
}
