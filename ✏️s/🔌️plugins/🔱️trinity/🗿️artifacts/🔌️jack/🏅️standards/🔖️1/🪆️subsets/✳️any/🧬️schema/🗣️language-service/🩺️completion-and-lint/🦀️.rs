//! 🩺️ Jack completion and lint: the completion context, the semantic lints, the formatting, the hover and the semantic tokens of the jack language service.

use super::*;

// #region 🔖️Language
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
    if after.chars().any(|c| c.is_whitespace() || matches!(c, '(' | ')' | '[' | ']' | ',')) {
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

fn graph_node_kinds(graph: &Graph) -> Vec<String> {
    let mut kinds = BTreeSet::new();
    for node in graph.nodes.values() {
        kinds.insert(node.kind.clone());
    }
    for def in &graph.manifest.node_kinds {
        kinds.insert(def.name.clone());
    }
    kinds.into_iter().collect()
}

fn graph_edge_kinds(graph: &Graph) -> Vec<String> {
    let mut kinds = BTreeSet::new();
    for edge in graph.edges.values() {
        kinds.insert(edge.kind.clone());
    }
    for def in &graph.manifest.edge_kinds {
        kinds.insert(def.name.clone());
    }
    kinds.into_iter().collect()
}

fn graph_property_names(graph: &Graph) -> Vec<String> {
    let mut props = BTreeSet::from(["id".to_string(), "name".to_string(), "kind".to_string()]);
    for node in graph.nodes.values() {
        for key in node.properties.keys() {
            props.insert(key.clone());
        }
    }
    props.into_iter().collect()
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
pub fn complete(graph: &Graph, source: &str, cursor: usize) -> Vec<Completion> {
    semio_framework_graph_dsl::complete(&TrinityQueryableGraph(graph), source, cursor)
}

// #endregion 🔖️Language
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

fn semantic_lints(graph: &Graph, query: &Query, source: &str) -> Vec<Diagnostic> {
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
pub fn lint(graph: &Graph, source: &str) -> Vec<Diagnostic> {
    semio_framework_graph_dsl::lint(&TrinityQueryableGraph(graph), source)
}

#[allow(dead_code)]
fn format_token(tok: &Token) -> String {
    match tok {
        Token::KwMatch => "MATCH".into(),
        Token::KwWhere => "WHERE".into(),
        Token::KwReturn => "RETURN".into(),
        Token::KwCreate => "CREATE".into(),
        Token::KwDelete => "DELETE".into(),
        Token::KwSet => "SET".into(),
        Token::KwMerge => "MERGE".into(),
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
        Token::StringLit(s) => format!("'{s}'"),
        Token::LParen => "(".into(),
        Token::RParen => ")".into(),
        Token::LBracket => "[".into(),
        Token::RBracket => "]".into(),
        Token::Colon => ":".into(),
        Token::Comma => ",".into(),
        Token::Dot => ".".into(),
        Token::Eq => "=".into(),
        Token::Ne => "!=".into(),
        Token::Dash => "-".into(),
        Token::Arrow => "->".into(),
        Token::Eof => String::new(),
    }
}

/// 🪞️ Format jack source canonically (idempotent).
pub fn format(source: &str) -> Result<String, String> {
    semio_framework_graph_dsl::format(source).map_err(|err| err.to_string())
}

#[allow(dead_code)]
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
pub fn hover(graph: &Graph, source: &str, cursor: usize) -> Option<Hover> {
    semio_framework_graph_dsl::hover(&TrinityQueryableGraph(graph), source, cursor)
}

/// 🎨️ Semantic token classes for LSP highlighting.
pub fn semantic_tokens(source: &str) -> Vec<SemanticToken> {
    semio_framework_graph_dsl::semantic_tokens(source)
}
