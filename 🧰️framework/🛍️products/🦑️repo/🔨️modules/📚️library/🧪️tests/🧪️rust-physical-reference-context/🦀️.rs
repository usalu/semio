//#region 🧪️Oracle
use syn::{parse::Parser, punctuated::Punctuated, visit::Visit, Expr, Lit, Token};
use std::collections::HashMap;

struct AssertionMessages(Vec<serde_json::Value>);

impl<'ast> Visit<'ast> for AssertionMessages {
    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        let name = node.path.segments.last().unwrap().ident.to_string();
        let index = match name.as_str() {
            "assert" | "debug_assert" => 1,
            "assert_eq" | "assert_ne" | "debug_assert_eq" | "debug_assert_ne" => 2,
            _ => return,
        };
        let Ok(arguments) = Punctuated::<Expr, Token![,]>::parse_terminated.parse2(node.tokens.clone()) else { return };
        let Some(Expr::Lit(expression)) = arguments.iter().nth(index) else { return };
        let Lit::Str(literal) = &expression.lit else { return };
        let token = literal.token().to_string();
        if !token.starts_with('"') || token.contains('\\') { return }
        self.0.push(serde_json::json!({ "macroName": name, "value": literal.value() }));
    }
}

#[derive(Clone)]
enum Binding { Path(Vec<String>), Loop(Vec<String>) }

#[derive(Default)]
struct ManifestPaths { bindings: HashMap<String, Binding>, rows: Vec<serde_json::Value> }

fn literal(expression: &Expr) -> Option<String> {
    let Expr::Lit(expression) = expression else { return None };
    let Lit::Str(value) = &expression.lit else { return None };
    let token = value.token().to_string();
    (token.starts_with('"') && !token.contains('\\')).then(|| value.value())
}

fn name(expression: &Expr) -> Option<String> {
    let Expr::Path(path) = expression else { return None };
    (path.path.segments.len() == 1).then(|| path.path.segments[0].ident.to_string())
}

fn resolve_path(expression: &Expr, bindings: &HashMap<String, Binding>) -> Option<Vec<String>> {
    if let Some(Binding::Path(value)) = name(expression).and_then(|key| bindings.get(&key)) { return Some(value.clone()) }
    match expression {
        Expr::Paren(value) => resolve_path(&value.expr, bindings),
        Expr::Call(call) => {
            let Expr::Path(path) = call.func.as_ref() else { return None };
            let segments: Vec<_> = path.path.segments.iter().map(|segment| segment.ident.to_string()).collect();
            if segments != ["std", "path", "Path", "new"] || call.args.len() != 1 { return None }
            let Expr::Macro(value) = &call.args[0] else { return None };
            if !value.mac.path.is_ident("env") { return None }
            let argument = syn::parse2::<Expr>(value.mac.tokens.clone()).ok()?;
            (literal(&argument).as_deref() == Some("CARGO_MANIFEST_DIR")).then(Vec::new)
        }
        Expr::MethodCall(call) if call.method == "join" && call.args.len() == 1 => {
            let mut base = resolve_path(&call.receiver, bindings)?;
            base.push(literal(&call.args[0])?);
            Some(base)
        }
        _ => None,
    }
}

struct IdentifierCount { target: String, count: usize }

#[derive(Default)]
struct PatternBindings(Vec<String>);

impl<'ast> Visit<'ast> for PatternBindings {
    fn visit_pat_ident(&mut self, pattern: &'ast syn::PatIdent) {
        self.0.push(pattern.ident.to_string());
        syn::visit::visit_pat_ident(self, pattern);
    }
}

fn clear_pattern(pattern: &syn::Pat, bindings: &mut HashMap<String, Binding>) {
    let mut names = PatternBindings::default();
    names.visit_pat(pattern);
    for name in names.0 { bindings.remove(&name); }
}

impl<'ast> Visit<'ast> for IdentifierCount {
    fn visit_ident(&mut self, identifier: &'ast syn::Ident) {
        self.count += usize::from(identifier == self.target.as_str());
    }
    fn visit_macro(&mut self, value: &'ast syn::Macro) {
        let text = value.tokens.to_string();
        self.count += text.split(|c: char| !c.is_alphanumeric() && c != '_').filter(|part| *part == self.target).count();
    }
}

impl<'ast> Visit<'ast> for ManifestPaths {
    fn visit_item_fn(&mut self, function: &'ast syn::ItemFn) {
        let previous = std::mem::take(&mut self.bindings);
        self.visit_block(&function.block);
        self.bindings = previous;
    }
    fn visit_block(&mut self, block: &'ast syn::Block) {
        let previous = self.bindings.clone();
        for statement in &block.stmts { self.visit_stmt(statement); }
        self.bindings = previous;
    }
    fn visit_local(&mut self, local: &'ast syn::Local) {
        let resolved = local.init.as_ref().and_then(|init| {
            self.visit_expr(&init.expr);
            resolve_path(&init.expr, &self.bindings)
        });
        clear_pattern(&local.pat, &mut self.bindings);
        if let syn::Pat::Ident(pattern) = &local.pat {
            self.bindings.remove(&pattern.ident.to_string());
            if pattern.mutability.is_none() && pattern.by_ref.is_none() {
                if let Some(value) = resolved { self.bindings.insert(pattern.ident.to_string(), Binding::Path(value)); }
            }
        }
    }
    fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {
        self.visit_expr(&call.receiver);
        for argument in &call.args { self.visit_expr(argument); }
        if call.method != "join" || call.args.len() != 1 { return }
        let Some(base) = resolve_path(&call.receiver, &self.bindings) else { return };
        let values = if let Some(value) = literal(&call.args[0]) { vec![value] }
            else if let Some(Binding::Loop(values)) = name(&call.args[0]).and_then(|key| self.bindings.get(&key)) { values.clone() }
            else { return };
        for value in values { self.rows.push(serde_json::json!({ "value": value, "base": base })); }
    }
    fn visit_expr_for_loop(&mut self, expression: &'ast syn::ExprForLoop) {
        let previous = self.bindings.clone();
        if let syn::Pat::Ident(pattern) = expression.pat.as_ref() {
            self.bindings.remove(&pattern.ident.to_string());
            if let Expr::Array(array) = expression.expr.as_ref() {
                let values: Option<Vec<_>> = array.elems.iter().map(literal).collect();
                let mut count = IdentifierCount { target: pattern.ident.to_string(), count: 0 };
                count.visit_block(&expression.body);
                if count.count == 1 && pattern.mutability.is_none() {
                    if let Some(values) = values { self.bindings.insert(pattern.ident.to_string(), Binding::Loop(values)); }
                }
            }
        }
        self.visit_block(&expression.body);
        self.bindings = previous;
    }
    fn visit_expr_closure(&mut self, expression: &'ast syn::ExprClosure) {
        let previous = self.bindings.clone();
        for input in &expression.inputs { clear_pattern(input, &mut self.bindings); }
        self.visit_expr(&expression.body);
        self.bindings = previous;
    }
    fn visit_expr_if(&mut self, expression: &'ast syn::ExprIf) {
        let previous = self.bindings.clone();
        if let Expr::Let(binding) = expression.cond.as_ref() {
            self.visit_expr(&binding.expr);
            clear_pattern(&binding.pat, &mut self.bindings);
        } else { self.visit_expr(&expression.cond); }
        self.visit_block(&expression.then_branch);
        self.bindings = previous;
        if let Some((_, branch)) = &expression.else_branch { self.visit_expr(branch); }
    }
    fn visit_expr_match(&mut self, expression: &'ast syn::ExprMatch) {
        self.visit_expr(&expression.expr);
        let previous = self.bindings.clone();
        for arm in &expression.arms {
            clear_pattern(&arm.pat, &mut self.bindings);
            self.visit_expr(&arm.body);
            self.bindings = previous.clone();
        }
    }
    fn visit_macro(&mut self, _: &'ast syn::Macro) {}
}

fn main() {
    let path = std::env::args().nth(1).expect("language-neutral vector path");
    let data: serde_json::Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    let rows: Vec<_> = data["assertionMessages"]["cases"].as_array().unwrap().iter().map(|row| {
        let source = syn::parse_file(row["source"].as_str().unwrap()).unwrap();
        let mut visitor = AssertionMessages(Vec::new());
        visitor.visit_file(&source);
        serde_json::json!({ "id": row["id"], "messages": visitor.0 })
    }).collect();
    let paths: Vec<_> = data["manifestPaths"]["cases"].as_array().unwrap().iter().map(|row| {
        let source = syn::parse_file(row["source"].as_str().unwrap()).unwrap();
        let mut visitor = ManifestPaths::default();
        visitor.visit_file(&source);
        serde_json::json!({ "id": row["id"], "references": visitor.rows })
    }).collect();
    println!("{}", serde_json::json!({ "assertionMessages": rows, "manifestPaths": paths }));
}
//#endregion 🧪️Oracle
