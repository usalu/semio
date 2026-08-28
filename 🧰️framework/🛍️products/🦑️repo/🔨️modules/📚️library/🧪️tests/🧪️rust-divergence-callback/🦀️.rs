use std::collections::BTreeSet;
use syn::{parse::Parser, punctuated::Punctuated, spanned::Spanned, visit::Visit, Expr, Lit, Pat, Stmt, Token};

#[derive(Default)]
struct Namespace { blocked: bool }

impl<'ast> Visit<'ast> for Namespace {
    fn visit_attribute(&mut self, _: &'ast syn::Attribute) { self.blocked = true; }
    fn visit_item_use(&mut self, _: &'ast syn::ItemUse) { self.blocked = true; }
    fn visit_item_macro(&mut self, _: &'ast syn::ItemMacro) { self.blocked = true; }
    fn visit_type_param(&mut self, value: &'ast syn::TypeParam) { self.blocked |= value.ident == "std"; }
    fn visit_item(&mut self, value: &'ast syn::Item) {
        self.blocked |= match value {
            syn::Item::Mod(value) => value.ident == "std",
            syn::Item::Type(value) => value.ident == "std",
            syn::Item::Struct(value) => value.ident == "std",
            syn::Item::Enum(value) => value.ident == "std",
            syn::Item::Trait(value) => value.ident == "std",
            _ => false,
        };
        syn::visit::visit_item(self, value);
    }
}

/// 📐 Converts independent parser coordinates to the public UTF-16 source-span contract.
fn offset(source: &str, position: proc_macro2::LineColumn) -> usize {
    let lines: Vec<_> = source.split_inclusive('\n').collect();
    lines.iter().take(position.line - 1).map(|line| line.encode_utf16().count()).sum::<usize>()
        + lines[position.line - 1].chars().take(position.column).map(char::len_utf16).sum::<usize>()
}

fn path(value: &syn::Path) -> String {
    format!("{}{}", if value.leading_colon.is_some() { "::" } else { "" }, value.segments.iter().map(|segment| segment.ident.to_string()).collect::<Vec<_>>().join("::"))
}

fn ident(value: &Expr) -> Option<String> {
    let Expr::Path(value) = value else { return None };
    (value.qself.is_none() && value.path.segments.len() == 1 && value.path.leading_colon.is_none()).then(|| value.path.segments[0].ident.to_string())
}

fn binding(value: &Pat) -> Option<&syn::PatIdent> {
    let Pat::Ident(value) = value else { return None };
    (value.attrs.is_empty() && value.by_ref.is_none() && value.mutability.is_none() && value.subpat.is_none()).then_some(value)
}

fn literal(value: &Expr) -> Option<&syn::LitStr> {
    let Expr::Lit(value) = value else { return None };
    let Lit::Str(text) = &value.lit else { return None };
    let token = text.token().to_string();
    (value.attrs.is_empty() && token.starts_with('"') && token.ends_with('"') && !token.contains('\\')).then_some(text)
}

fn macro_arguments(value: &syn::Macro) -> Option<Punctuated<Expr, Token![,]>> {
    Punctuated::<Expr, Token![,]>::parse_terminated.parse2(value.tokens.clone()).ok()
}

/// 🧾 Parses only named capture fields; escaped braces and formatting sublanguages remain unknown.
fn captures(source: &str, literal: &syn::LitStr, allowed: &[&str]) -> Option<Vec<serde_json::Value>> {
    let text = literal.value();
    let base = offset(source, literal.span().start()) + 1;
    let mut rows = Vec::new();
    let mut cursor = 0;
    while cursor < text.len() {
        let character = text[cursor..].chars().next()?;
        if character == '}' { return None }
        if character != '{' { cursor += character.len_utf8(); continue }
        let first = cursor + 1;
        let close = first + text[first..].find('}')?;
        let name = &text[first..close];
        if !allowed.contains(&name) || !name.chars().all(|character| character == '_' || character.is_ascii_alphanumeric()) { return None }
        let start = base + text[..first].encode_utf16().count();
        rows.push(serde_json::json!({ "name": name, "start": start, "end": start + name.encode_utf16().count() }));
        cursor = close + 1;
    }
    (!rows.is_empty()).then_some(rows)
}

fn standard_root(value: &Expr) -> Option<String> {
    let Expr::MethodCall(join) = value else { return None };
    if join.method != "join" || join.args.len() != 1 || join.turbofish.is_some() { return None }
    let prefix = literal(&join.args[0])?.value();
    if prefix.starts_with('/') || prefix.contains('\\') || prefix.contains(':') { return None }
    let Expr::Call(call) = join.receiver.as_ref() else { return None };
    let Expr::Path(constructor) = call.func.as_ref() else { return None };
    if constructor.qself.is_some() || path(&constructor.path) != "std::path::PathBuf::from" || call.args.len() != 1 { return None }
    let Expr::Macro(environment) = &call.args[0] else { return None };
    let arguments = macro_arguments(&environment.mac)?;
    if !environment.mac.path.is_ident("env") || arguments.len() != 1 || literal(&arguments[0])?.value() != "CARGO_MANIFEST_DIR" { return None }
    Some(prefix)
}

fn statement_macro(value: &Stmt) -> Option<&syn::Macro> {
    match value {
        Stmt::Macro(value) if value.attrs.is_empty() => Some(&value.mac),
        Stmt::Expr(Expr::Macro(value), _) if value.attrs.is_empty() => Some(&value.mac),
        _ => None,
    }
}

/// 🧭 Independently recognizes the closed callback fixture grammar from syn nodes, not expected rows.
fn inspect(source: &str) -> Option<(Vec<serde_json::Value>, Vec<serde_json::Value>)> {
    let file = syn::parse_file(source).ok()?;
    let mut namespace = Namespace::default();
    namespace.visit_file(&file);
    if namespace.blocked { return None }
    let functions: Vec<_> = file.items.iter().filter_map(|item| match item { syn::Item::Fn(value) if value.sig.ident == "inspect" => Some(value), _ => None }).collect();
    if functions.len() != 1 { return None }
    let function = functions[0];
    if function.block.stmts.len() != 2 { return None }
    let Stmt::Local(root) = &function.block.stmts[0] else { return None };
    let root_name = binding(&root.pat)?.ident.to_string();
    let prefix = standard_root(&root.init.as_ref()?.expr)?;
    let Stmt::Expr(Expr::ForLoop(loop_), _) = &function.block.stmts[1] else { return None };
    let leaf = binding(&loop_.pat)?.ident.to_string();
    let Expr::Array(array) = loop_.expr.as_ref() else { return None };
    if array.elems.is_empty() || array.elems.len() > 256 || loop_.body.stmts.len() != 2 { return None }
    let values: Option<Vec<_>> = array.elems.iter().map(literal).collect();
    let values = values?;
    let Stmt::Local(local) = &loop_.body.stmts[0] else { return None };
    let result_name = binding(&local.pat)?.ident.to_string();
    let Expr::MethodCall(method) = local.init.as_ref()?.expr.as_ref() else { return None };
    if method.method != "unwrap_or_else" || method.turbofish.is_some() || method.args.len() != 1 { return None }
    let Expr::Call(read) = method.receiver.as_ref() else { return None };
    let Expr::Path(read_path) = read.func.as_ref() else { return None };
    if path(&read_path.path) != "std::fs::read_to_string" || read.args.len() != 1 { return None }
    let Expr::MethodCall(join) = &read.args[0] else { return None };
    if join.method != "join" || join.turbofish.is_some() || join.args.len() != 1 || ident(&join.receiver)? != root_name || ident(&join.args[0])? != leaf { return None }
    let Expr::Closure(closure) = &method.args[0] else { return None };
    if !closure.attrs.is_empty() || closure.asyncness.is_some() || closure.capture.is_some() || closure.movability.is_some() || closure.constness.is_some() || closure.lifetimes.is_some() || !matches!(closure.output, syn::ReturnType::Default) || closure.inputs.len() != 1 { return None }
    let parameter = binding(&closure.inputs[0])?;
    let error = parameter.ident.to_string();
    if error == leaf || error == root_name { return None }
    let Expr::Macro(body) = closure.body.as_ref() else { return None };
    let macro_path = path(&body.mac.path);
    if !["panic", "std::panic", "::std::panic"].contains(&macro_path.as_str()) || !body.attrs.is_empty() { return None }
    let arguments = macro_arguments(&body.mac)?;
    if arguments.len() != 1 { return None }
    let message = literal(&arguments[0])?;
    let captures = captures(source, message, &[&leaf, &error])?;
    let free: BTreeSet<_> = captures.iter().filter_map(|row| row["name"].as_str()).filter(|name| *name != error).collect();
    if free != BTreeSet::from([leaf.as_str()]) || !captures.iter().any(|row| row["name"] == error) { return None }
    let assertion = statement_macro(&loop_.body.stmts[1])?;
    if !assertion.path.is_ident("assert_eq") { return None }
    let assertion_arguments = macro_arguments(assertion)?;
    if assertion_arguments.len() != 3 || ident(&assertion_arguments[0])? != result_name || ident(&assertion_arguments[1])? != leaf || literal(&assertion_arguments[2])?.value() != format!("{{{leaf}}}") { return None }
    let rows = values.into_iter().map(|literal| {
        let value = literal.value();
        serde_json::json!({ "start": offset(source, literal.span().start()) + 1, "end": offset(source, literal.span().end()) - 1, "value": value, "targets": [[prefix, value]] })
    }).collect();
    let callback = serde_json::json!({
        "start": offset(source, closure.span().start()), "end": offset(source, closure.span().end()),
        "bodyStart": offset(source, body.span().start()), "bodyEnd": offset(source, body.span().end()),
        "parameter": { "name": error, "start": offset(source, parameter.ident.span().start()), "end": offset(source, parameter.ident.span().end()) },
        "macroPath": macro_path, "captures": captures, "freeVariables": free
    });
    Some((rows, vec![callback]))
}

fn main() {
    let input = std::env::args().nth(1).expect("neutral vector");
    let vector: serde_json::Value = serde_json::from_slice(&std::fs::read(input).expect("vector bytes")).expect("vector JSON");
    let rows: Vec<_> = vector["cases"].as_array().expect("cases").iter().map(|row| {
        let source = row["source"].as_str().expect("source");
        let parsed = syn::parse_file(source);
        let (candidates, callbacks) = inspect(source).unwrap_or_default();
        serde_json::json!({ "id": row["id"], "parseable": parsed.is_ok(), "candidates": candidates, "callbacks": callbacks })
    }).collect();
    println!("{}", serde_json::json!({ "rows": rows }));
}
