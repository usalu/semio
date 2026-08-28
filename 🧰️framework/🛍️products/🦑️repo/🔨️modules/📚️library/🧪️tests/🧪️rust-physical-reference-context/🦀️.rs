//#region 🧪️Oracle
use syn::{parse::Parser, punctuated::Punctuated, visit::Visit, Expr, Lit, Token};
use std::{cell::RefCell, collections::{HashMap, HashSet}, rc::Rc};

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
struct ManifestPaths { bindings: HashMap<String, Binding>, rows: Vec<serde_json::Value>, opaque_macros: bool }

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
            if (segments != ["std", "path", "Path", "new"] && segments != ["std", "path", "PathBuf", "from"]) || call.args.len() != 1 || path.qself.is_some() || path.path.segments.iter().any(|segment| !matches!(segment.arguments, syn::PathArguments::None)) { return None }
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

struct IdentifierCount { target: String, count: usize, opaque_macros: bool }

fn format_captures(text: &str, target: &str) -> bool {
    let mut characters = text.chars().peekable();
    while let Some(character) = characters.next() {
        if character != '{' { continue }
        if characters.peek() == Some(&'{') { characters.next(); continue }
        let field: String = characters.by_ref().take_while(|character| *character != '}').collect();
        let (name, format) = field.split_once(':').unwrap_or((&field, ""));
        if name.trim() == target || format.split(|character: char| !character.is_alphanumeric() && character != '_' && character != '$').any(|word| word.strip_suffix('$') == Some(target)) { return true }
    }
    false
}

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
        let segments: Vec<_> = value.path.segments.iter().map(|segment| segment.ident.to_string()).collect();
        let name = segments.last().map(String::as_str).unwrap_or("");
        let standard = segments.len() == 1 && !self.opaque_macros || segments.len() == 2 && segments[0] == "std";
        let argument = match name {
            "format" | "format_args" | "print" | "println" | "eprint" | "eprintln" | "panic" => Some(0),
            "write" | "writeln" | "assert" | "debug_assert" => Some(1),
            "assert_eq" | "assert_ne" | "debug_assert_eq" | "debug_assert_ne" => Some(2),
            _ => None,
        }.filter(|_| standard);
        if let (Some(index), Ok(arguments)) = (argument, Punctuated::<Expr, Token![,]>::parse_terminated.parse2(value.tokens.clone())) {
            if let Some(text) = arguments.iter().nth(index).and_then(literal) {
                for expression in &arguments { self.visit_expr(expression); }
                self.count += usize::from(format_captures(&text, &self.target));
                return;
            }
        }
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
                let mut count = IdentifierCount { target: pattern.ident.to_string(), count: 0, opaque_macros: self.opaque_macros };
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

#[derive(Default)]
struct CollectionShadows { names: HashSet<String>, wildcard: bool, custom_join: bool, module_depth: usize, opaque_macros: bool }

fn path_name(path: &syn::Path) -> String {
    path.segments.iter().map(|segment| segment.ident.to_string()).collect::<Vec<_>>().join("::")
}

impl CollectionShadows {
    fn record(&mut self, value: &syn::Ident) {
        if ["Vec", "String", "std", "format"].contains(&value.to_string().as_str()) { self.names.insert(value.to_string()); }
    }
    fn use_tree(&mut self, tree: &syn::UseTree, parents: &[String]) {
        match tree {
            syn::UseTree::Path(path) => {
                if path.ident != "std" { self.record(&path.ident); }
                let mut prefix = parents.to_vec(); prefix.push(path.ident.to_string());
                self.use_tree(&path.tree, &prefix);
            }
            syn::UseTree::Name(value) => self.record(&value.ident),
            syn::UseTree::Rename(value) => { self.record(&value.ident); self.record(&value.rename); }
            syn::UseTree::Group(value) => for item in &value.items { self.use_tree(item, parents); },
            syn::UseTree::Glob(_) => self.wildcard |= parents.is_empty() || parents.len() > self.module_depth || parents.iter().any(|name| name != "super"),
        }
    }
    fn standard(&self, path: &syn::Path, short: &str, qualified: &str) -> bool {
        let name = path_name(path);
        if name == short { !self.wildcard && !self.names.contains(short.split("::").next().unwrap()) }
        else { name == qualified && !self.names.contains("std") }
    }
    fn constructor(&self, expression: &Expr) -> bool {
        let Expr::Call(call) = expression else { return false };
        let Expr::Path(path) = call.func.as_ref() else { return false };
        call.args.is_empty() && path.path.segments.iter().all(|segment| matches!(segment.arguments, syn::PathArguments::None)) && self.standard(&path.path, "Vec::new", "std::vec::Vec::new")
    }
    fn string_type(&self, ty: &syn::Type) -> bool {
        let syn::Type::Path(value) = ty else { return false };
        if !self.standard(&value.path, "Vec", "std::vec::Vec") { return false }
        let syn::PathArguments::AngleBracketed(arguments) = &value.path.segments.last().unwrap().arguments else { return false };
        let Some(syn::GenericArgument::Type(syn::Type::Path(element))) = arguments.args.first() else { return false };
        arguments.args.len() == 1 && self.standard(&element.path, "String", "std::string::String")
    }
    fn string_value(&self, expression: &Expr) -> bool {
        match expression {
            Expr::Lit(value) => matches!(value.lit, Lit::Str(_)),
            Expr::Macro(value) => self.standard(&value.mac.path, "format", "std::format"),
            _ => false,
        }
    }
}

impl<'ast> Visit<'ast> for CollectionShadows {
    fn visit_item(&mut self, item: &'ast syn::Item) {
        match item {
            syn::Item::Struct(value) => self.record(&value.ident),
            syn::Item::Enum(value) => self.record(&value.ident),
            syn::Item::Union(value) => self.record(&value.ident),
            syn::Item::Type(value) => self.record(&value.ident),
            syn::Item::Trait(value) => self.record(&value.ident),
            syn::Item::Mod(value) => self.record(&value.ident),
            syn::Item::Macro(value) => if let Some(name) = &value.ident { self.record(name); self.opaque_macros = true; },
            _ => {},
        }
        syn::visit::visit_item(self, item);
    }
    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        self.module_depth += usize::from(item.content.is_some());
        syn::visit::visit_item_mod(self, item);
        self.module_depth -= usize::from(item.content.is_some());
    }
    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) { self.opaque_macros = true; self.use_tree(&item.tree, &[]); }
    fn visit_type_param(&mut self, parameter: &'ast syn::TypeParam) { self.record(&parameter.ident); }
    fn visit_pat_ident(&mut self, pattern: &'ast syn::PatIdent) { self.record(&pattern.ident); }
    fn visit_signature(&mut self, signature: &'ast syn::Signature) {
        self.custom_join |= signature.ident == "join";
        syn::visit::visit_signature(self, signature);
    }
    fn visit_attribute(&mut self, attribute: &'ast syn::Attribute) {
        self.wildcard |= attribute.path().is_ident("no_std") || attribute.path().is_ident("no_implicit_prelude");
    }
}

struct StringCollection { valid: bool, strings: bool }
type Collection = Rc<RefCell<StringCollection>>;

struct JoinArguments<'a> {
    shadows: &'a CollectionShadows,
    bindings: HashMap<String, Collection>,
    rows: Vec<(String, Option<Collection>)>,
}

fn join_literal(expression: &Expr) -> Option<String> {
    let Expr::Lit(expression) = expression else { return None };
    let Lit::Str(value) = &expression.lit else { return None };
    let text = value.token().to_string();
    (text.starts_with('"') && text.ends_with('"')).then(|| text[1..text.len() - 1].to_string())
}

impl JoinArguments<'_> {
    fn clear(&mut self, pattern: &syn::Pat) {
        let mut names = PatternBindings::default(); names.visit_pat(pattern);
        for name in names.0 { self.bindings.remove(&name); }
    }
}

impl<'ast> Visit<'ast> for JoinArguments<'_> {
    fn visit_item_fn(&mut self, function: &'ast syn::ItemFn) {
        let previous = std::mem::take(&mut self.bindings);
        self.visit_block(&function.block);
        self.bindings = previous;
    }
    fn visit_impl_item_fn(&mut self, function: &'ast syn::ImplItemFn) {
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
        if let Some(initializer) = &local.init { self.visit_expr(&initializer.expr); }
        self.clear(&local.pat);
        if self.shadows.custom_join || !local.init.as_ref().is_some_and(|value| self.shadows.constructor(&value.expr)) { return }
        let (pattern, strings) = match &local.pat {
            syn::Pat::Type(value) => (value.pat.as_ref(), self.shadows.string_type(&value.ty)),
            pattern => (pattern, false),
        };
        if let syn::Pat::Ident(pattern) = pattern {
            if pattern.by_ref.is_none() && pattern.subpat.is_none() {
                self.bindings.insert(pattern.ident.to_string(), Rc::new(RefCell::new(StringCollection { valid: true, strings })));
            }
        }
    }
    fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {
        let collection = name(&call.receiver).and_then(|name| self.bindings.get(&name)).cloned();
        if collection.is_none() { self.visit_expr(&call.receiver); }
        if call.method == "join" && call.args.len() == 1 {
            if let Some(value) = join_literal(&call.args[0]) { self.rows.push((value, collection.clone())); }
        }
        if let Some(value) = collection {
            let mut state = value.borrow_mut();
            match call.method.to_string().as_str() {
                "push" if call.args.len() == 1 && self.shadows.string_value(&call.args[0]) => state.strings = true,
                "join" if call.args.len() == 1 && join_literal(&call.args[0]).is_some() => {},
                "len" | "is_empty" if call.args.is_empty() => {},
                _ => state.valid = false,
            }
        }
        for argument in &call.args { self.visit_expr(argument); }
    }
    fn visit_expr_path(&mut self, expression: &'ast syn::ExprPath) {
        if expression.path.segments.len() == 1 {
            if let Some(value) = self.bindings.get(&expression.path.segments[0].ident.to_string()) { value.borrow_mut().valid = false; }
        }
    }
    fn visit_expr_for_loop(&mut self, expression: &'ast syn::ExprForLoop) {
        self.visit_expr(&expression.expr);
        let previous = self.bindings.clone(); self.clear(&expression.pat);
        self.visit_block(&expression.body); self.bindings = previous;
    }
    fn visit_expr_closure(&mut self, expression: &'ast syn::ExprClosure) {
        let previous = self.bindings.clone();
        for pattern in &expression.inputs { self.clear(pattern); }
        self.visit_expr(&expression.body); self.bindings = previous;
    }
    fn visit_expr_if(&mut self, expression: &'ast syn::ExprIf) {
        let previous = self.bindings.clone();
        if let Expr::Let(binding) = expression.cond.as_ref() { self.visit_expr(&binding.expr); self.clear(&binding.pat); }
        else { self.visit_expr(&expression.cond); }
        self.visit_block(&expression.then_branch); self.bindings = previous;
        if let Some((_, branch)) = &expression.else_branch { self.visit_expr(branch); }
    }
    fn visit_expr_match(&mut self, expression: &'ast syn::ExprMatch) {
        self.visit_expr(&expression.expr);
        let previous = self.bindings.clone();
        for arm in &expression.arms {
            self.clear(&arm.pat); self.visit_expr(&arm.body); self.bindings = previous.clone();
        }
    }
    fn visit_macro(&mut self, value: &'ast syn::Macro) {
        let name = value.path.segments.last().unwrap().ident.to_string();
        if ["assert", "assert_eq", "assert_ne", "debug_assert", "debug_assert_eq", "debug_assert_ne", "print", "println", "eprint", "eprintln", "format", "write", "writeln", "panic"].contains(&name.as_str()) {
            if let Ok(arguments) = Punctuated::<Expr, Token![,]>::parse_terminated.parse2(value.tokens.clone()) {
                for expression in arguments { self.visit_expr(&expression); }
            }
        } else {
            for (name, collection) in &self.bindings {
                let mut count = IdentifierCount { target: name.clone(), count: 0, opaque_macros: true }; count.visit_macro(value);
                if count.count > 0 { collection.borrow_mut().valid = false; }
            }
        }
    }
}

#[derive(Clone)]
enum CandidateKind { String(String, proc_macro2::Span), Path(Vec<String>), Array(Vec<CandidateValue>), Tuple(Vec<CandidateValue>), Metadata }

#[derive(Clone)]
struct CandidateValue { kind: CandidateKind, valid: Rc<std::cell::Cell<bool>>, dependencies: Vec<Rc<std::cell::Cell<bool>>> }

impl CandidateValue {
    fn new(kind: CandidateKind) -> Self { Self { kind, valid: Rc::new(std::cell::Cell::new(true)), dependencies: Vec::new() } }
    fn states(&self) -> Vec<Rc<std::cell::Cell<bool>>> { let mut values = self.dependencies.clone(); values.push(self.valid.clone()); values }
    fn wrap(&self, parents: Vec<Rc<std::cell::Cell<bool>>>) -> Self { let mut value = Self::new(self.kind.clone()); value.dependencies = self.states(); value.dependencies.extend(parents); value }
}

struct CandidateRow { value: String, span: proc_macro2::Span, targets: std::collections::BTreeSet<Vec<String>>, dependencies: Vec<Rc<std::cell::Cell<bool>>> }

#[derive(Default)]
struct ManifestCandidates { bindings: HashMap<String, CandidateValue>, rows: std::collections::BTreeMap<String, CandidateRow>, expanded: usize, overflow: bool, read_only: bool, macros: CandidateMacroAuthority }

#[derive(Default)]
struct CandidateMacroAuthority { shadowed: HashSet<String>, wildcard: bool, depth: usize, standard_type_shadowed: bool }

impl CandidateMacroAuthority {
    fn known(name: &str) -> bool { ["assert", "assert_eq", "assert_ne", "debug_assert", "debug_assert_eq", "debug_assert_ne", "print", "println", "eprint", "eprintln", "format", "format_args", "panic", "write", "writeln"].contains(&name) }
    fn use_tree(&mut self, tree: &syn::UseTree, parents: &[String]) {
        match tree {
            syn::UseTree::Path(path) => { let mut values = parents.to_vec(); values.push(path.ident.to_string()); self.use_tree(&path.tree, &values); }
            syn::UseTree::Name(value) => { self.standard_type_shadowed |= value.ident == "std"; if value.ident == "env" || Self::known(&value.ident.to_string()) { self.shadowed.insert(value.ident.to_string()); } }
            syn::UseTree::Rename(value) => { self.standard_type_shadowed |= value.rename == "std"; for name in [&value.ident, &value.rename] { if name == "env" || Self::known(&name.to_string()) { self.shadowed.insert(name.to_string()); } } }
            syn::UseTree::Group(value) => for item in &value.items { self.use_tree(item, parents); },
            syn::UseTree::Glob(_) => self.wildcard |= parents.is_empty() || parents.len() > self.depth || parents.iter().any(|parent| parent != "super"),
        }
    }
}

impl<'ast> Visit<'ast> for CandidateMacroAuthority {
    fn visit_item(&mut self, value: &'ast syn::Item) {
        self.standard_type_shadowed |= match value { syn::Item::Mod(value) => value.ident == "std", syn::Item::Struct(value) => value.ident == "std", syn::Item::Type(value) => value.ident == "std", syn::Item::Enum(value) => value.ident == "std", syn::Item::Union(value) => value.ident == "std", syn::Item::Trait(value) => value.ident == "std", _ => false };
        syn::visit::visit_item(self, value);
    }
    fn visit_type_param(&mut self, value: &'ast syn::TypeParam) { self.standard_type_shadowed |= value.ident == "std"; }
    fn visit_local(&mut self, value: &'ast syn::Local) { let mut names = PatternBindings::default(); names.visit_pat(&value.pat); self.standard_type_shadowed |= names.0.iter().any(|name| name == "std"); syn::visit::visit_local(self, value); }
    fn visit_attribute(&mut self, value: &'ast syn::Attribute) { self.wildcard |= value.path().is_ident("no_std") || value.path().is_ident("no_implicit_prelude") || value.path().is_ident("macro_use"); }
    fn visit_item_mod(&mut self, value: &'ast syn::ItemMod) { self.depth += usize::from(value.content.is_some()); syn::visit::visit_item_mod(self, value); self.depth -= usize::from(value.content.is_some()); }
    fn visit_item_use(&mut self, value: &'ast syn::ItemUse) { self.use_tree(&value.tree, &[]); }
    fn visit_item_macro(&mut self, value: &'ast syn::ItemMacro) { if let Some(name) = &value.ident { self.shadowed.insert(name.to_string()); } }
}

impl ManifestCandidates {
    fn evaluate(&mut self, expression: &Expr, emit: bool) -> Option<CandidateValue> {
        if let Some(value) = name(expression).and_then(|name| self.bindings.get(&name)) { return Some(value.clone()) }
        match expression {
            Expr::Lit(value) => match &value.lit {
                Lit::Str(text) => literal(expression).map(|value| CandidateValue::new(CandidateKind::String(value, text.span()))),
                Lit::Int(_) | Lit::Bool(_) | Lit::Float(_) => Some(CandidateValue::new(CandidateKind::Metadata)),
                _ => None,
            },
            Expr::Paren(value) => self.evaluate(&value.expr, emit),
            Expr::Array(value) => value.elems.iter().map(|item| self.evaluate(item, emit)).collect::<Option<Vec<_>>>().map(|items| CandidateValue::new(CandidateKind::Array(items))),
            Expr::Tuple(value) => value.elems.iter().map(|item| self.evaluate(item, emit)).collect::<Option<Vec<_>>>().map(|items| CandidateValue::new(CandidateKind::Tuple(items))),
            Expr::Reference(value) if value.mutability.is_none() => {
                let Expr::Index(index) = value.expr.as_ref() else { return None };
                let Expr::Range(range) = index.index.as_ref() else { return None };
                if range.start.is_some() || range.end.is_some() || !matches!(index.expr.as_ref(), Expr::Array(_)) { return None }
                self.evaluate(&index.expr, emit)
            }
            Expr::Call(_) => resolve_path(expression, &HashMap::new()).map(|parts| CandidateValue::new(CandidateKind::Path(parts))),
            Expr::MethodCall(call) if call.method == "enumerate" && call.args.is_empty() => {
                let Expr::MethodCall(iter) = call.receiver.as_ref() else { return None };
                if iter.method != "iter" || !iter.args.is_empty() { return None }
                let source = self.evaluate(&iter.receiver, emit)?;
                let CandidateKind::Array(values) = &source.kind else { return None };
                Some(CandidateValue::new(CandidateKind::Array(values.iter().map(|item| CandidateValue::new(CandidateKind::Tuple(vec![CandidateValue::new(CandidateKind::Metadata), item.wrap(source.states())]))).collect())))
            }
            Expr::MethodCall(call) if call.method == "join" && call.args.len() == 1 => {
                let base = self.evaluate(&call.receiver, emit)?;
                let argument = self.evaluate(&call.args[0], emit)?;
                let CandidateKind::Path(mut parts) = base.kind.clone() else { return None };
                let CandidateKind::String(value, span) = &argument.kind else { return None };
                if value.starts_with('/') || value.as_bytes().first().is_some_and(u8::is_ascii_alphabetic) && value.as_bytes().get(1) == Some(&b':') { return None }
                parts.push(value.clone());
                let mut dependencies = base.states(); dependencies.extend(argument.states());
                if emit {
                    let key = format!("{}:{}", span.start().line, span.start().column);
                    let row = self.rows.entry(key).or_insert_with(|| CandidateRow { value: value.clone(), span: *span, targets: Default::default(), dependencies: Vec::new() });
                    row.targets.insert(parts.clone()); row.dependencies.extend(dependencies.clone());
                    self.overflow |= row.targets.len() > 256;
                }
                let mut result = CandidateValue::new(CandidateKind::Path(parts)); result.dependencies = dependencies; Some(result)
            }
            _ => None,
        }
    }
    fn bind(&mut self, pattern: &syn::Pat, value: &CandidateValue) -> bool {
        match pattern {
            syn::Pat::Ident(pattern) if pattern.mutability.is_none() && pattern.by_ref.is_none() && pattern.subpat.is_none() => { self.bindings.insert(pattern.ident.to_string(), value.wrap(Vec::new())); true }
            syn::Pat::Tuple(pattern) => {
                let CandidateKind::Tuple(values) = &value.kind else { return false };
                if pattern.elems.len() != values.len() { return false }
                let mut names = PatternBindings::default(); names.visit_pat_tuple(pattern);
                if names.0.iter().collect::<HashSet<_>>().len() != names.0.len() { return false }
                for (pattern, value_) in pattern.elems.iter().zip(values) { if !self.bind(pattern, &value_.wrap(value.states())) { return false } }
                true
            }
            syn::Pat::Wild(_) => true,
            _ => false,
        }
    }
    fn clear(&mut self, pattern: &syn::Pat) {
        let mut names = PatternBindings::default(); names.visit_pat(pattern);
        for name in names.0 { if let Some(value) = self.bindings.remove(&name) { value.valid.set(false); } }
    }
    fn output(&self, selected: &str, source: &str) -> Vec<serde_json::Value> {
        if self.overflow { return Vec::new() }
        let offset = |position: proc_macro2::LineColumn| {
            let lines: Vec<_> = source.split_inclusive('\n').collect();
            lines.iter().take(position.line - 1).map(|line| line.encode_utf16().count()).sum::<usize>() + lines[position.line - 1].chars().take(position.column).map(char::len_utf16).sum::<usize>()
        };
        let mut rows: Vec<_> = self.rows.values().filter(|row| row.value == selected && row.dependencies.iter().all(|value| value.get())).map(|row| serde_json::json!({ "start": offset(row.span.start()) + 1, "end": offset(row.span.end()) - 1, "value": row.value, "targets": row.targets })).collect();
        rows.sort_by_key(|row| row["start"].as_u64()); rows
    }
}

impl<'ast> Visit<'ast> for ManifestCandidates {
    fn visit_item_fn(&mut self, value: &'ast syn::ItemFn) {
        let previous = std::mem::take(&mut self.bindings); self.visit_block(&value.block); self.bindings = previous;
    }
    fn visit_block(&mut self, block: &'ast syn::Block) {
        let previous = self.bindings.clone();
        for statement in &block.stmts { if self.overflow { break } self.visit_stmt(statement); }
        self.bindings = previous;
    }
    fn visit_local(&mut self, local: &'ast syn::Local) {
        let resolved = local.init.as_ref().and_then(|init| self.evaluate(&init.expr, false));
        if let Some(init) = &local.init { let previous = self.read_only; self.read_only = resolved.is_some(); self.visit_expr(&init.expr); self.read_only = previous; }
        self.clear(&local.pat);
        if let Some(value) = resolved { self.bind(&local.pat, &value); }
    }
    fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {
        let expression = Expr::MethodCall(call.clone());
        if self.evaluate(&expression, true).is_some_and(|value| matches!(value.kind, CandidateKind::Path(_))) { return }
        syn::visit::visit_expr_method_call(self, call);
    }
    fn visit_expr_path(&mut self, expression: &'ast syn::ExprPath) {
        if !self.read_only { if let Some(value) = self.bindings.get(&path_name(&expression.path)) { value.valid.set(false); } }
    }
    fn visit_expr_for_loop(&mut self, expression: &'ast syn::ExprForLoop) {
        let sequence = self.evaluate(&expression.expr, false);
        let Some(CandidateValue { kind: CandidateKind::Array(values), .. }) = &sequence else { for value in self.bindings.values() { value.valid.set(false); } return };
        let sequence = sequence.as_ref().unwrap();
        let previous = self.bindings.clone();
        for item in values {
            self.expanded += 1; if self.expanded > 256 { self.overflow = true; break }
            self.bindings = previous.clone();
            if !self.bind(&expression.pat, &item.wrap(sequence.states())) { for value in self.bindings.values() { value.valid.set(false); } break }
            self.visit_block(&expression.body);
        }
        self.bindings = previous;
    }
    fn visit_expr_closure(&mut self, _: &'ast syn::ExprClosure) { for value in self.bindings.values() { value.valid.set(false); } }
    fn visit_expr_unsafe(&mut self, _: &'ast syn::ExprUnsafe) { for value in self.bindings.values() { value.valid.set(false); } }
    fn visit_expr_while(&mut self, _: &'ast syn::ExprWhile) { for value in self.bindings.values() { value.valid.set(false); } }
    fn visit_expr_match(&mut self, _: &'ast syn::ExprMatch) { for value in self.bindings.values() { value.valid.set(false); } }
    fn visit_expr_if(&mut self, value: &'ast syn::ExprIf) {
        if matches!(value.cond.as_ref(), Expr::Let(_)) { for value in self.bindings.values() { value.valid.set(false); } }
        else { syn::visit::visit_expr_if(self, value); }
    }
    fn visit_macro(&mut self, value: &'ast syn::Macro) {
        let segments: Vec<_> = value.path.segments.iter().map(|item| item.ident.to_string()).collect();
        let name = segments.last().map(String::as_str).unwrap_or("");
        if (segments.len() == 1 && !self.macros.wildcard && !self.macros.shadowed.contains(name) || segments.len() == 2 && segments[0] == "std") && CandidateMacroAuthority::known(name) {
            if let Ok(arguments) = Punctuated::<Expr, Token![,]>::parse_terminated.parse2(value.tokens.clone()) {
                let previous = self.read_only; self.read_only = true;
                for argument in arguments { self.visit_expr(&argument); }
                self.read_only = previous; return;
            }
        }
        let text = value.tokens.to_string();
        for (name, value) in &self.bindings { if text.split(|character: char| !character.is_alphanumeric() && character != '_').any(|word| word == name) { value.valid.set(false); } }
    }
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
        let mut shadows = CollectionShadows::default(); shadows.visit_file(&source);
        visitor.opaque_macros = shadows.opaque_macros;
        if !shadows.names.contains("std") { visitor.visit_file(&source); }
        serde_json::json!({ "id": row["id"], "references": visitor.rows })
    }).collect();
    let joins: Vec<_> = data["joinArguments"]["cases"].as_array().unwrap().iter().map(|row| {
        let source = syn::parse_file(row["source"].as_str().unwrap()).unwrap();
        let mut shadows = CollectionShadows::default(); shadows.visit_file(&source);
        let mut visitor = JoinArguments { shadows: &shadows, bindings: HashMap::new(), rows: Vec::new() }; visitor.visit_file(&source);
        let all: Vec<_> = visitor.rows.iter().map(|(value, _)| value).collect();
        let candidates: Vec<_> = visitor.rows.iter().filter(|(_, collection)| !collection.as_ref().is_some_and(|value| { let value = value.borrow(); value.valid && value.strings })).map(|(value, _)| value).collect();
        serde_json::json!({ "id": row["id"], "allArguments": all, "candidates": candidates })
    }).collect();
    let candidates: Vec<_> = data["manifestCandidates"]["cases"].as_array().unwrap().iter().chain(data["manifestCandidates"]["adversarial"]["cases"].as_array().unwrap()).map(|row| {
        let source = syn::parse_file(row["source"].as_str().unwrap()).unwrap();
        let mut visitor = ManifestCandidates::default();
        visitor.macros.visit_file(&source);
        if !visitor.macros.standard_type_shadowed && !visitor.macros.wildcard && !visitor.macros.shadowed.contains("env") { visitor.visit_file(&source); }
        serde_json::json!({ "id": row["id"], "candidates": visitor.output(data["manifestCandidates"]["selectedValue"].as_str().unwrap(), row["source"].as_str().unwrap()) })
    }).collect();
    println!("{}", serde_json::json!({ "assertionMessages": rows, "manifestPaths": paths, "joinArguments": joins, "manifestCandidates": candidates }));
}
//#endregion 🧪️Oracle
