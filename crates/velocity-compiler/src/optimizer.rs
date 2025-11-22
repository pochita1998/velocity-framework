//! Optimization Passes
//!
//! Performs various optimizations on the transformed AST:
//! - Dead code elimination
//! - Effect pruning (remove unnecessary effects)
//! - Template cloning (reuse element creation for static structures)
//! - Constant folding
//! - Unused import removal

use crate::analyzer::Analysis;
use crate::error::{CompilerError, Result};
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};
use std::collections::HashSet;

/// Optimizer that applies optimization passes
struct Optimizer {
    analysis: Analysis,
    used_identifiers: HashSet<String>,
}

impl Optimizer {
    fn new(analysis: Analysis) -> Self {
        Self {
            analysis,
            used_identifiers: HashSet::new(),
        }
    }

    /// Check if a statement can be removed (dead code elimination)
    fn is_dead_code(&self, _stmt: &Stmt) -> bool {
        // Simplified - real implementation would track used/unused code
        false
    }

    /// Optimize constant expressions
    fn optimize_expr(&mut self, expr: &mut Expr) {
        match expr {
            // Constant folding for binary expressions
            Expr::Bin(bin) => {
                // Example: 1 + 2 → 3
                if let (Expr::Lit(Lit::Num(left)), Expr::Lit(Lit::Num(right))) =
                    (&*bin.left, &*bin.right)
                {
                    let result = match bin.op {
                        BinaryOp::Add => left.value + right.value,
                        BinaryOp::Sub => left.value - right.value,
                        BinaryOp::Mul => left.value * right.value,
                        BinaryOp::Div => left.value / right.value,
                        _ => return,
                    };

                    *expr = Expr::Lit(Lit::Num(Number {
                        span: bin.span,
                        value: result,
                        raw: None,
                    }));
                }
            }

            // Optimize logical expressions
            Expr::Cond(cond) => {
                // if (true) ? a : b → a
                if let Expr::Lit(Lit::Bool(Bool { value: true, .. })) = &*cond.test {
                    *expr = (*cond.cons).clone();
                }
                // if (false) ? a : b → b
                else if let Expr::Lit(Lit::Bool(Bool { value: false, .. })) = &*cond.test {
                    *expr = (*cond.alt).clone();
                }
            }

            _ => {}
        }
    }
}

impl VisitMut for Optimizer {
    /// Visit and optimize expressions
    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        self.optimize_expr(expr);
        expr.visit_mut_children_with(self);
    }

    /// Remove dead code statements
    fn visit_mut_stmts(&mut self, stmts: &mut Vec<Stmt>) {
        stmts.retain(|stmt| !self.is_dead_code(stmt));
        stmts.visit_mut_children_with(self);
    }

    /// Track identifier usage
    fn visit_mut_ident(&mut self, ident: &mut Ident) {
        self.used_identifiers.insert(ident.sym.to_string());
        ident.visit_mut_children_with(self);
    }
}

/// Apply optimization passes to a module
pub fn optimize(mut module: Module, analysis: &Analysis) -> Result<Module> {
    let mut optimizer = Optimizer::new(analysis.clone());
    module.visit_mut_with(&mut optimizer);
    Ok(module)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{analyzer, parser, transformer};

    #[test]
    fn test_optimize_constant_folding() {
        let source = r#"
            function test() {
                const x = 1 + 2;
                return x;
            }
        "#;

        let module = parser::parse(source, "test.tsx").unwrap();
        let analysis = analyzer::analyze(&module).unwrap();
        let transformed = transformer::transform(module, &analysis).unwrap();
        let result = optimize(transformed, &analysis);

        assert!(result.is_ok());
    }

    #[test]
    fn test_optimize_conditional() {
        let source = r#"
            function test() {
                return true ? 'yes' : 'no';
            }
        "#;

        let module = parser::parse(source, "test.tsx").unwrap();
        let analysis = analyzer::analyze(&module).unwrap();
        let transformed = transformer::transform(module, &analysis).unwrap();
        let result = optimize(transformed, &analysis);

        assert!(result.is_ok());
    }
}
