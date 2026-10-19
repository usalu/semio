//! 🧭️ Jack language service: completion, lint, format, hover and semantic tokens, plus the embedded `DslIdiom` hooks.

use super::*;

// #region 🔖️Language
#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Completion {
    pub label: String,
    pub kind: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    pub insert: String,
}

const CLAUSE_KEYWORDS: &[&str] = &["MATCH", "WHERE", "RETURN", "CREATE", "DELETE", "SET", "MERGE", "WITH", "UNWIND", "CALL"];
const LOGIC_KEYWORDS: &[&str] = &["AND", "OR"];

fn completion_prefix(source: &str, cursor: usize) -> String {
    let cursor = cursor.min(source.len());
    let bytes = source.as_bytes();
    let mut start = cursor;
    while start > 0 {
        let c = bytes[start - 1];
        if c.is_ascii_alphanumeric() || c == b'_' {
            start -= 1;
        } else {
            break;
        }
    }
    source[start..cursor].to_string()
}

fn tokens_before_cursor(tokens: &[SpannedToken], cursor: usize) -> &[SpannedToken] {
    let mut end = tokens.len();
    for (i, row) in tokens.iter().enumerate() {
        if row.start >= cursor && !matches!(row.token, Token::Eof) {
            end = i;
            break;
        }
    }
    &tokens[..end]
}

fn after_colon_kind_context(source: &str, cursor: usize) -> Option<bool> {
    let cursor = cursor.min(source.len());
    let before = &source[..cursor];
    let colon = before.rfind(':')?;
    let after = &before[colon + 1..];
    if after.chars().any(|c| c.is_whitespace() || matches!(c, '(' | ')' | '[' | ']' | ',' | '@')) {
        return None;
    }
    let left = &before[..colon];
    let bracket = left.rfind('[');
    let paren = left.rfind('(');
    let in_bracket = match (bracket, paren) {
        (Some(b), Some(p)) => b > p,
        (Some(_), None) => true,
        _ => false,
    };
    Some(in_bracket)
}

fn after_dot_property_context(source: &str, cursor: usize) -> bool {
    let cursor = cursor.min(source.len());
    let before = &source[..cursor];
    let Some(dot) = before.rfind('.') else {
        return false;
    };
    let after = &before[dot + 1..];
    !after.chars().any(|c| c.is_whitespace() || matches!(c, '(' | ')' | '[' | ']' | ',' | ':'))
}
fn open_bracket_kind(tokens: &[SpannedToken]) -> Option<char> {
    let mut paren = 0i32;
    let mut bracket = 0i32;
    for row in tokens.iter().rev() {
        match row.token {
            Token::RParen => paren += 1,
            Token::LParen if paren > 0 => paren -= 1,
            Token::LParen if paren == 0 && bracket == 0 => return Some('('),
            Token::RBracket => bracket += 1,
            Token::LBracket if bracket > 0 => bracket -= 1,
            Token::LBracket if bracket == 0 && paren == 0 => return Some('['),
            _ => {}
        }
    }
    None
}

fn collect_bound_vars(tokens: &[SpannedToken]) -> BTreeSet<String> {
    let mut vars = BTreeSet::new();
    let mut i = 0;
    while i + 2 < tokens.len() {
        if matches!(tokens[i].token, Token::LParen | Token::LBracket) {
            if let Token::Ident(var) = &tokens[i + 1].token {
                if matches!(tokens[i + 2].token, Token::Colon) {
                    vars.insert(var.clone());
                }
            }
        }
        i += 1;
    }
    vars
}

fn in_where_clause(tokens: &[SpannedToken]) -> bool {
    let mut seen_where = false;
    let mut seen_return = false;
    for row in tokens {
        match row.token {
            Token::KwWhere => seen_where = true,
            Token::KwReturn => seen_return = true,
            _ => {}
        }
    }
    seen_where && !seen_return
}

fn graph_node_kinds<G: QueryableGraph>(graph: &G) -> Vec<String> {
    manifest_node_kinds(graph)
}

fn graph_edge_kinds<G: QueryableGraph>(graph: &G) -> Vec<String> {
    manifest_edge_kinds(graph)
}

fn graph_property_names<G: QueryableGraph>(graph: &G) -> Vec<String> {
    manifest_property_names(graph)
}

fn graph_port_kinds<G: QueryableGraph>(graph: &G) -> Vec<String> {
    manifest_port_kinds(graph)
}

fn after_at_port_context(source: &str, cursor: usize) -> bool {
    let cursor = cursor.min(source.len());
    let before = &source[..cursor];
    let Some(at) = before.rfind('@') else {
        return false;
    };
    let after = &before[at + 1..];
    !after.chars().any(|c| c.is_whitespace() || matches!(c, '(' | ')' | '[' | ']' | ',' | '-' | '>' | '@'))
}

fn filter_completions(candidates: impl IntoIterator<Item = (String, String, Option<String>)>, prefix: &str) -> Vec<Completion> {
    let prefix_lower = prefix.to_ascii_lowercase();
    let mut out = Vec::new();
    for (label, kind, detail) in candidates {
        if prefix.is_empty() || label.to_ascii_lowercase().starts_with(&prefix_lower) {
            out.push(Completion { insert: label.clone(), label, kind, detail });
        }
    }
    out.sort_by(|a, b| a.label.cmp(&b.label));
    out
}

/// 🔎️ Context-aware jack completions for the editor.
pub fn complete<G: QueryableGraph>(graph: &G, source: &str, cursor: usize) -> Vec<Completion> {
    let cursor = cursor.min(source.len());
    let prefix = completion_prefix(source, cursor);
    let tokens = lex_spanned(source, true).unwrap_or_default();
    let before = tokens_before_cursor(&tokens, cursor);

    if let Some(in_bracket) = after_colon_kind_context(source, cursor) {
        let kinds = if in_bracket { graph_edge_kinds(graph).into_iter().map(|name| (name, "edgeKind".into(), None)).collect::<Vec<_>>() } else { graph_node_kinds(graph).into_iter().map(|name| (name, "nodeKind".into(), None)).collect::<Vec<_>>() };
        return filter_completions(kinds, &prefix);
    }

    if after_dot_property_context(source, cursor) {
        let props = graph_property_names(graph).into_iter().map(|name| (name, "property".into(), None)).collect::<Vec<_>>();
        return filter_completions(props, &prefix);
    }

    if after_at_port_context(source, cursor) {
        let ports = graph_port_kinds(graph).into_iter().map(|name| (name, "portKind".into(), None)).collect::<Vec<_>>();
        return filter_completions(ports, &prefix);
    }

    if let Some(last) = before.last() {
        if matches!(last.token, Token::At) {
            let ports = graph_port_kinds(graph).into_iter().map(|name| (name, "portKind".into(), None)).collect::<Vec<_>>();
            return filter_completions(ports, &prefix);
        }
        if matches!(last.token, Token::Colon) {
            let kinds = if open_bracket_kind(before) == Some('[') {
                graph_edge_kinds(graph).into_iter().map(|name| (name, "edgeKind".into(), None)).collect::<Vec<_>>()
            } else {
                graph_node_kinds(graph).into_iter().map(|name| (name, "nodeKind".into(), None)).collect::<Vec<_>>()
            };
            return filter_completions(kinds, &prefix);
        }
        if matches!(last.token, Token::Dot) {
            let props = graph_property_names(graph).into_iter().map(|name| (name, "property".into(), None)).collect::<Vec<_>>();
            return filter_completions(props, &prefix);
        }
    }

    if in_where_clause(before) {
        let logic = filter_completions(LOGIC_KEYWORDS.iter().map(|kw| (kw.to_string(), "keyword".into(), None)), &prefix);
        if !logic.is_empty() {
            return logic;
        }
    }

    let vars = collect_bound_vars(before);
    if !vars.is_empty() && prefix.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_') {
        let var_items = vars.into_iter().map(|name| (name, "variable".into(), None)).collect::<Vec<_>>();
        let filtered = filter_completions(var_items, &prefix);
        if !filtered.is_empty() {
            return filtered;
        }
    }

    filter_completions(CLAUSE_KEYWORDS.iter().map(|kw| (kw.to_string(), "keyword".into(), None)), &prefix)
}
// #endregion 🔖️Language

// #region 🔖️LanguageService
#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Info,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Diagnostic {
    pub start: usize,
    pub end: usize,
    pub severity: DiagnosticSeverity,
    pub message: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Hover {
    pub start: usize,
    pub end: usize,
    pub contents: String,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemanticToken {
    pub start: usize,
    pub end: usize,
    pub class: String,
}

fn collect_pattern_vars(pattern: &Pattern, out: &mut BTreeSet<String>) {
    for node in &pattern.nodes {
        out.insert(node.var.clone());
    }
    if let Some(edge) = &pattern.edge {
        if let Some(var) = &edge.var {
            out.insert(var.clone());
        }
        out.insert(edge.right.var.clone());
    }
}

fn collect_clause_bound_vars(clauses: &[Clause]) -> BTreeSet<String> {
    let mut vars = BTreeSet::new();
    for clause in clauses {
        match clause {
            Clause::Match(patterns) => {
                for pattern in patterns {
                    collect_pattern_vars(pattern, &mut vars);
                }
            }
            Clause::Create(pattern) | Clause::Merge(pattern) => collect_pattern_vars(pattern, &mut vars),
            _ => {}
        }
    }
    vars
}

fn collect_referenced_vars(clauses: &[Clause]) -> Vec<(String, usize, usize)> {
    let mut refs = Vec::new();
    for clause in clauses {
        match clause {
            Clause::Return(items) => {
                for item in items {
                    match item {
                        ReturnItem::Var(v) => refs.push((v.clone(), 0, v.len())),
                        ReturnItem::Property { var, .. } => refs.push((var.clone(), 0, var.len())),
                    }
                }
            }
            Clause::Delete(vars) => {
                for var in vars {
                    refs.push((var.clone(), 0, var.len()));
                }
            }
            Clause::Set(assignments) => {
                for assignment in assignments {
                    refs.push((assignment.var.clone(), 0, assignment.var.len()));
                }
            }
            Clause::Where(expr) => collect_expr_vars(expr, &mut refs),
            _ => {}
        }
    }
    refs
}

fn collect_expr_vars(expr: &Expr, refs: &mut Vec<(String, usize, usize)>) {
    match expr {
        Expr::Eq { var, .. } | Expr::Ne { var, .. } => refs.push((var.clone(), 0, var.len())),
        Expr::And(a, b) | Expr::Or(a, b) => {
            collect_expr_vars(a, refs);
            collect_expr_vars(b, refs);
        }
    }
}

fn semantic_lints<G: QueryableGraph>(graph: &G, query: &Query, source: &str) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    let node_kinds = graph_node_kinds(graph).into_iter().collect::<BTreeSet<_>>();
    let edge_kinds = graph_edge_kinds(graph).into_iter().collect::<BTreeSet<_>>();
    let bound = collect_clause_bound_vars(&query.clauses);
    for clause in &query.clauses {
        match clause {
            Clause::Match(patterns) => {
                for pattern in patterns {
                    for node in &pattern.nodes {
                        if !node_kinds.contains(&node.kind) {
                            if let Some((start, end)) = find_kind_span(source, &node.kind) {
                                out.push(Diagnostic { start, end, severity: DiagnosticSeverity::Error, message: format!("unknown node kind '{}'", node.kind), code: Some("jack/unknown-node-kind".into()) });
                            }
                        }
                    }
                    if let Some(edge) = &pattern.edge {
                        if let Some(kind) = &edge.kind {
                            if !edge_kinds.contains(kind) {
                                if let Some((start, end)) = find_kind_span(source, kind) {
                                    out.push(Diagnostic { start, end, severity: DiagnosticSeverity::Error, message: format!("unknown edge kind '{}'", kind), code: Some("jack/unknown-edge-kind".into()) });
                                }
                            }
                        }
                    }
                }
            }
            Clause::Create(pattern) | Clause::Merge(pattern) => {
                for node in &pattern.nodes {
                    if !node_kinds.contains(&node.kind) {
                        if let Some((start, end)) = find_kind_span(source, &node.kind) {
                            out.push(Diagnostic { start, end, severity: DiagnosticSeverity::Error, message: format!("unknown node kind '{}'", node.kind), code: Some("jack/unknown-node-kind".into()) });
                        }
                    }
                }
            }
            _ => {}
        }
    }
    for (var, _, _) in collect_referenced_vars(&query.clauses) {
        if !bound.contains(&var) {
            if let Some((start, end)) = find_ident_span(source, &var) {
                out.push(Diagnostic { start, end, severity: DiagnosticSeverity::Error, message: format!("variable '{var}' is not bound by MATCH"), code: Some("jack/unbound-variable".into()) });
            }
        }
    }
    out
}

fn find_kind_span(source: &str, kind: &str) -> Option<(usize, usize)> {
    let needle = format!(":{kind}");
    let start = source.find(&needle)?;
    Some((start + 1, start + needle.len()))
}

fn find_ident_span(source: &str, ident: &str) -> Option<(usize, usize)> {
    let mut from = 0;
    while let Some(rel) = source[from..].find(ident) {
        let start = from + rel;
        let end = start + ident.len();
        let before = source.as_bytes().get(start.wrapping_sub(1));
        let after = source.as_bytes().get(end);
        let boundary_before = before.is_none_or(|c| !c.is_ascii_alphanumeric() && *c != b'_');
        let boundary_after = after.is_none_or(|c| !c.is_ascii_alphanumeric() && *c != b'_');
        if boundary_before && boundary_after {
            return Some((start, end));
        }
        from = end;
    }
    None
}

/// 🩺️ Lint jack source with syntax and semantic diagnostics.
pub fn lint<G: QueryableGraph>(graph: &G, source: &str) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    for span in tokenize(source) {
        if span.class == TokenClass::Error {
            out.push(Diagnostic { start: span.start, end: span.end, severity: DiagnosticSeverity::Error, message: "unterminated string literal".into(), code: Some("jack/unterminated-string".into()) });
        }
    }
    match parse(source) {
        Ok(query) => out.extend(semantic_lints(graph, &query, source)),
        Err(err) => {
            let end = source.len().max(1);
            out.push(Diagnostic { start: 0, end, severity: DiagnosticSeverity::Error, message: err.to_string(), code: Some("jack/parse-error".into()) });
        }
    }
    out
}

fn format_token(tok: &Token) -> String {
    match tok {
        Token::KwMatch => "MATCH".into(),
        Token::KwWhere => "WHERE".into(),
        Token::KwReturn => "RETURN".into(),
        Token::KwCreate => "CREATE".into(),
        Token::KwDelete => "DELETE".into(),
        Token::KwSet => "SET".into(),
        Token::KwMerge => "MERGE".into(),
        Token::KwWith => "WITH".into(),
        Token::KwUnwind => "UNWIND".into(),
        Token::KwCall => "CALL".into(),
        Token::KwAs => "AS".into(),
        Token::And => "AND".into(),
        Token::Or => "OR".into(),
        Token::Ident(s) => s.clone(),
        Token::Number(n) => {
            if n.fract() == 0.0 {
                format!("{}", *n as i64)
            } else {
                n.to_string()
            }
        }
        Token::StringLit(s) => format!("\"{}\"", semio_framework_dsl::escape_text(s)),
        Token::LParen => "(".into(),
        Token::RParen => ")".into(),
        Token::LBracket => "[".into(),
        Token::RBracket => "]".into(),
        Token::Colon => ":".into(),
        Token::Comma => ",".into(),
        Token::Dot => ".".into(),
        Token::Eq => "=".into(),
        Token::Ne => "!=".into(),
        Token::Arrow => "->".into(),
        Token::Dash => "-".into(),
        Token::DashArrow => "--".into(),
        Token::BackArrow => "<-".into(),
        Token::At => "@".into(),
        Token::Eof => String::new(),
    }
}

/// 🪞️ Format jack source canonically (idempotent).
pub fn format(source: &str) -> Result<String, GraphDslError> {
    let tokens = lex_spanned(source, false)?;
    let mut out = String::new();
    let mut line_open = false;
    let mut i = 0;
    while i < tokens.len() {
        let row = &tokens[i];
        if matches!(row.token, Token::Eof) {
            break;
        }
        match &row.token {
            Token::KwMatch | Token::KwWhere | Token::KwReturn | Token::KwCreate | Token::KwDelete | Token::KwSet | Token::KwMerge | Token::KwWith | Token::KwUnwind | Token::KwCall => {
                if !out.is_empty() {
                    out.push('\n');
                }
                out.push_str(&format_token(&row.token));
                out.push(' ');
                line_open = true;
            }
            Token::Comma => {
                out.push_str(", ");
            }
            Token::Arrow => {
                out.push_str("->");
            }
            Token::Dash => {
                out.push_str("-");
            }
            Token::DashArrow => {
                out.push_str("--");
            }
            Token::BackArrow => {
                out.push_str("<-");
            }
            Token::And | Token::Or | Token::KwAs => {
                out.push(' ');
                out.push_str(&format_token(&row.token));
                out.push(' ');
            }
            Token::Eq | Token::Ne => {
                out.push(' ');
                out.push_str(&format_token(&row.token));
                out.push(' ');
            }
            _ => {
                if line_open && !out.ends_with(' ') && !out.ends_with('\n') && !matches!(row.token, Token::RParen | Token::RBracket | Token::Comma | Token::Dot) {
                    let prev = tokens.get(i.saturating_sub(1)).map(|t| &t.token);
                    if !matches!(prev, Some(Token::LParen | Token::LBracket | Token::Colon | Token::Dot | Token::Arrow | Token::Dash | Token::DashArrow | Token::BackArrow)) {
                        out.push(' ');
                    }
                }
                out.push_str(&format_token(&row.token));
            }
        }
        i += 1;
    }
    Ok(out.trim().to_string())
}

fn hover_word_at(source: &str, cursor: usize) -> Option<(usize, usize, String)> {
    let cursor = cursor.min(source.len());
    if cursor > source.len() {
        return None;
    }
    let bytes = source.as_bytes();
    let mut start = cursor;
    while start > 0 {
        let c = bytes[start - 1];
        if c.is_ascii_alphanumeric() || c == b'_' || c == b':' || c == b'.' {
            start -= 1;
        } else {
            break;
        }
    }
    let mut end = cursor;
    while end < bytes.len() {
        let c = bytes[end];
        if c.is_ascii_alphanumeric() || c == b'_' || c == b':' || c == b'.' {
            end += 1;
        } else {
            break;
        }
    }
    if start == end {
        return None;
    }
    Some((start, end, source[start..end].to_string()))
}

/// 💬️ Hover information at cursor.
pub fn hover<G: QueryableGraph>(graph: &G, source: &str, cursor: usize) -> Option<Hover> {
    let (start, end, word) = hover_word_at(source, cursor)?;
    let upper = word.to_ascii_uppercase();
    if CLAUSE_KEYWORDS.iter().any(|kw| *kw == upper) || LOGIC_KEYWORDS.iter().any(|kw| *kw == upper) {
        return Some(Hover { start, end, contents: format!("Jack keyword `{upper}`") });
    }
    if graph_node_kinds(graph).iter().any(|kind| kind == &word) {
        return Some(Hover { start, end, contents: format!("Node kind `{word}`") });
    }
    if graph_edge_kinds(graph).iter().any(|kind| kind == &word) {
        return Some(Hover { start, end, contents: format!("Edge kind `{word}`") });
    }
    if graph_property_names(graph).iter().any(|prop| prop == &word) {
        return Some(Hover { start, end, contents: format!("Property `{word}`") });
    }
    if collect_bound_vars(&lex_spanned(source, true).unwrap_or_default()).contains(&word) {
        return Some(Hover { start, end, contents: format!("Bound variable `{word}`") });
    }
    None
}

/// 🎨️ Semantic token classes for LSP highlighting.
pub fn semantic_tokens(source: &str) -> Vec<SemanticToken> {
    tokenize(source)
        .into_iter()
        .map(|span| SemanticToken {
            start: span.start,
            end: span.end,
            class: match span.class {
                TokenClass::Keyword => "keyword",
                TokenClass::Ident => "ident",
                TokenClass::Number => "number",
                TokenClass::String => "string",
                TokenClass::Operator => "operator",
                TokenClass::Punctuation => "punctuation",
                TokenClass::Error => "error",
            }
            .into(),
        })
        .collect()
}
// #endregion 🔖️LanguageService

// #region 🔖️DslIdiom
/// 🔌️ Registers Jack as a `semio_framework_dsl::IdiomHooks` entry (the `DslIdiom` seam's Route B — an EMBEDDED
/// idiom hosted inside another document's `Shape::Embed("jack")` field) so `canonicalize`
/// normalizes embedded Jack text through this crate's own `format`/`tokenize`. Hand-built rather
/// than `semio_framework_dsl::hooks_for::<I: DslIdiom>()`: that helper needs `DslIdiom::print(ast) -> String`, and
/// Jack has no AST-to-text printer (`format` re-derives canonical text token-by-token from SOURCE,
/// not from a `Query`) — `IdiomHooks` itself only needs function pointers, so it's built directly
/// from the language-service surface Jack already has, no printer required.
// 🚫️async: E4 fn-pointer slot — builds an `IdiomHooks` whose fields are plain `fn` pointers; an
// `async fn`'s value cannot coerce to `fn`, so this stays sync. See R2 E4, mirrors
// `semio_framework_dsl::hooks_for`/`semio_framework_dsl::passthrough_hooks`.
pub fn idiom_hooks() -> semio_framework_dsl::IdiomHooks {
    semio_framework_dsl::IdiomHooks { lang: "jack", canonicalize: idiom_canonicalize, classify: idiom_classify, complete: idiom_complete }
}

// 🚫️async: E4 fn-pointer slot (`IdiomHooks.canonicalize`) — see R2 E4.
fn idiom_canonicalize(text: &str) -> Result<String, semio_framework_diagnostic::TextError> {
    format(text).map_err(|error|match error{GraphDslError::Lex(error)=>error,GraphDslError::Json(error)=>semio_framework_diagnostic::TextError::new(error.kind(),error.to_string(),semio_framework_diagnostic::TextSpan::at(1,1)),GraphDslError::UnsupportedMutation=>semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"mutating jack clauses are not supported on this graph domain",semio_framework_diagnostic::TextSpan::at(1,1)),error @ (GraphDslError::UnterminatedString|GraphDslError::UnexpectedChar(_)|GraphDslError::NumberFormat(_)|GraphDslError::UnexpectedToken{..}|GraphDslError::EdgeTargetMissingPort|GraphDslError::EmptyPattern|GraphDslError::UnknownProcedure(_)|GraphDslError::ProcedureArity{..}|GraphDslError::ManifestIdentity{..})=>semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string(),semio_framework_diagnostic::TextSpan::at(1,1))})
}

// 🚫️async: E4 fn-pointer slot (`IdiomHooks.classify`) — see R2 E4.
fn idiom_classify(text: &str) -> Vec<(semio_framework_dsl::TokenClass, semio_framework_diagnostic::TextSpan)> {
    tokenize(text)
        .into_iter()
        .map(|span| {
            let class = match span.class {
                TokenClass::Keyword => semio_framework_dsl::TokenClass::Keyword,
                TokenClass::Ident => semio_framework_dsl::TokenClass::Ident,
                TokenClass::Number => semio_framework_dsl::TokenClass::Number,
                TokenClass::String => semio_framework_dsl::TokenClass::String,
                TokenClass::Operator => semio_framework_dsl::TokenClass::Operator,
                TokenClass::Punctuation => semio_framework_dsl::TokenClass::Punctuation,
                TokenClass::Error => semio_framework_dsl::TokenClass::Error,
            };
            (class, byte_range_to_span(text, span.start, span.end))
        })
        .collect()
}

// 🚫️async: E4 fn-pointer slot (`IdiomHooks.complete`) — see R2 E4.
fn idiom_complete(text: &str, offset: usize) -> Vec<semio_framework_dsl::CompletionItem> {
    struct EmptyGraph;
    impl QueryableGraph for EmptyGraph {
        fn manifest(&self) -> Option<&semio_framework_graph_manifest::GraphManifest> {
            None
        }
        fn node_ids(&self) -> Vec<String> {
            Vec::new()
        }
        fn node_kind(&self, _id: &str) -> Option<String> {
            None
        }
        fn node_name(&self, _id: &str) -> Option<String> {
            None
        }
        fn node_property(&self, _id: &str, _key: &str) -> Option<PropertyValue> {
            None
        }
        fn edges(&self) -> Vec<QueryableEdge> {
            Vec::new()
        }
        fn subgraph_snapshot_json(&self, _node_ids: &BTreeSet<String>, _edge_ids: &BTreeSet<String>) -> Option<String> {
            None
        }
    }
    complete(&EmptyGraph, text, offset).into_iter().map(|c| semio_framework_dsl::CompletionItem { label: c.label, detail: c.detail }).collect()
}

/// 📍️ Converts a byte-offset half-open range into `os_dsl::TextSpan`'s 1-based line/column/
/// length form — Jack's own spans are byte offsets (`TokenSpan`/`SemanticToken`), `dsl_core`'s are
/// line/column, so this is the one place that needs the source text to translate between them.
fn byte_range_to_span(text: &str, start: usize, end: usize) -> semio_framework_diagnostic::TextSpan {
    let mut line = 1u32;
    let mut column = 1u32;
    for (i, ch) in text.char_indices() {
        if i >= start {
            break;
        }
        if ch == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    let length = text.get(start..end).map_or(0, |s| s.chars().count()) as u32;
    semio_framework_diagnostic::TextSpan::with_length(line, column, length)
}
// #endregion 🔖️DslIdiom
