//! 🗣️ Trinity jack language service — parse, complete, lint, hover.
#![allow(dead_code)]

use crate::artifacts::jack::{port_node_id, port_port_id, Camera, Edge, Graph, JackSnapshot, Manifest, Node, Port, PortDirection, PropertyBag, PropertyValue};
use crate::lexer::{lex, lex_spanned, SpannedToken, Token};
use graph::dsl::{QueryableEdge, QueryableGraph};
use std::collections::BTreeSet;

pub mod queryable {
    use super::*;

    fn trinity_jack_manifest() -> &'static graph::manifest::GraphManifest {
        use std::sync::OnceLock;
        static MANIFEST: OnceLock<graph::manifest::GraphManifest> = OnceLock::new();
        MANIFEST.get_or_init(|| graph::manifest::manifest_by_id("nakagin").expect("nakagin manifest").clone())
    }

    fn trinity_queryable_edges(graph: &Graph) -> Vec<QueryableEdge> {
        graph
            .edges
            .values()
            .filter_map(|edge| {
                let source_node_id = port_node_id(&edge.source)?.to_string();
                let target_node_id = port_node_id(&edge.target)?.to_string();
                Some(QueryableEdge {
                    id: edge.id.clone(),
                    kind: edge.kind.clone(),
                    source_node_id,
                    target_node_id,
                    source_port: port_port_id(&edge.source).map(str::to_string),
                    target_port: port_port_id(&edge.target).map(str::to_string),
                    properties: edge.properties.clone(),
                })
            })
            .collect()
    }

    pub struct TrinityQueryableGraph<'a>(pub &'a Graph);

    impl QueryableGraph for TrinityQueryableGraph<'_> {
        fn manifest(&self) -> Option<&graph::manifest::GraphManifest> {
            Some(trinity_jack_manifest())
        }

        fn node_ids(&self) -> Vec<String> {
            self.0.nodes.keys().cloned().collect()
        }

        fn node_kind(&self, id: &str) -> Option<String> {
            self.0.nodes.get(id).map(|node| node.kind.clone())
        }

        fn node_name(&self, id: &str) -> Option<String> {
            self.0.nodes.get(id).map(|node| node.name.clone())
        }

        fn node_property(&self, id: &str, key: &str) -> Option<PropertyValue> {
            let node = self.0.nodes.get(id)?;
            match key {
                "id" => Some(PropertyValue::String(id.to_string())),
                "name" | "label" | "text" => Some(PropertyValue::String(node.name.clone())),
                "kind" => Some(PropertyValue::String(node.kind.clone())),
                "__all" => Some(PropertyValue::Object(node.properties.clone())),
                _ => node.properties.get(key).cloned(),
            }
        }

        fn edges(&self) -> Vec<QueryableEdge> {
            trinity_queryable_edges(self.0)
        }

        fn subgraph_fixture_json(&self, node_ids: &BTreeSet<String>, edge_ids: &BTreeSet<String>) -> Option<String> {
            self.0.subgraph_fixture(node_ids, edge_ids).to_json().ok()
        }
    }

    pub struct OwnedTrinityQueryableGraph(pub Graph);

    impl QueryableGraph for OwnedTrinityQueryableGraph {
        fn manifest(&self) -> Option<&graph::manifest::GraphManifest> {
            Some(trinity_jack_manifest())
        }

        fn node_ids(&self) -> Vec<String> {
            self.0.nodes.keys().cloned().collect()
        }

        fn node_kind(&self, id: &str) -> Option<String> {
            self.0.nodes.get(id).map(|node| node.kind.clone())
        }

        fn node_name(&self, id: &str) -> Option<String> {
            self.0.nodes.get(id).map(|node| node.name.clone())
        }

        fn node_property(&self, id: &str, key: &str) -> Option<PropertyValue> {
            let node = self.0.nodes.get(id)?;
            match key {
                "id" => Some(PropertyValue::String(id.to_string())),
                "name" | "label" | "text" => Some(PropertyValue::String(node.name.clone())),
                "kind" => Some(PropertyValue::String(node.kind.clone())),
                "__all" => Some(PropertyValue::Object(node.properties.clone())),
                _ => node.properties.get(key).cloned(),
            }
        }

        fn edges(&self) -> Vec<QueryableEdge> {
            trinity_queryable_edges(&self.0)
        }

        fn subgraph_fixture_json(&self, node_ids: &BTreeSet<String>, edge_ids: &BTreeSet<String>) -> Option<String> {
            self.0.subgraph_fixture(node_ids, edge_ids).to_json().ok()
        }
    }
}

use crate::ast::{Assignment, Clause, Expr, Pattern, PatternEdge, PatternNode, Query, ReturnItem};
use graph::dsl::{Completion, Diagnostic, DiagnosticSeverity, Hover, SemanticToken};
pub use queryable::{OwnedTrinityQueryableGraph, TrinityQueryableGraph};

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
    graph::dsl::complete(&TrinityQueryableGraph(graph), source, cursor)
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
    graph::dsl::lint(&TrinityQueryableGraph(graph), source)
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
    graph::dsl::format(source).map_err(|err| err.to_string())
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
    graph::dsl::hover(&TrinityQueryableGraph(graph), source, cursor)
}

/// 🎨️ Semantic token classes for LSP highlighting.
pub fn semantic_tokens(source: &str) -> Vec<SemanticToken> {
    graph::dsl::semantic_tokens(source)
}
// #endregion 🔖️LanguageService
/// 🧩️ Demo `Piece`/`Connection` fixture shared by the jack language server default session
/// and playgrounds that need a non-empty graph for completions, hover and lint.
pub fn example_graph_fixture() -> JackSnapshot {
    JackSnapshot::with_content(
        JackSnapshot::SCHEMA.into(),
        "jack-example".into(),
        Some("nakagin".into()),
        Manifest::nakagin_default(),
        Camera::default(),
        vec![
            Node {
                id: "root".into(),
                kind: "Piece".into(),
                name: "core".into(),
                x: 0.0,
                y: 0.0,
                width: 80.0,
                height: 40.0,
                properties: PropertyBag::new(),
                ports: vec![Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }],
            },
            Node {
                id: "child".into(),
                kind: "Piece".into(),
                name: "leaf".into(),
                x: 160.0,
                y: 0.0,
                width: 80.0,
                height: 40.0,
                properties: PropertyBag::new(),
                ports: vec![Port { id: "in".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }],
            },
        ],
        vec![Edge { id: "e1".into(), kind: "Connection".into(), source: "root@out".into(), target: "child@in".into(), properties: PropertyBag::new() }],
        Some("root".into()),
    )
}

/// 🧩️ [`example_graph_fixture`] as a resolved in-memory [`Graph`].
pub fn example_graph() -> Graph {
    Graph::from_fixture(example_graph_fixture()).expect("jack example fixture")
}

/// 🧩️ [`example_graph_fixture`] serialized as fixture JSON.
pub fn example_graph_fixture_json() -> String {
    serde_json::to_string(&example_graph_fixture()).unwrap_or_else(|_| "{}".into())
}
// #endregion 🔖️ExampleFixture
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn bump(&mut self) -> Token {
        let t = self.peek().clone();
        if !matches!(t, Token::Eof) {
            self.pos += 1;
        }
        t
    }

    fn expect_ident(&mut self) -> Result<String, String> {
        match self.bump() {
            Token::Ident(s) => Ok(s),
            other => Err(format!("expected ident, got {other:?}")),
        }
    }

    fn parse_query(&mut self) -> Result<Query, String> {
        let mut clauses = Vec::new();
        while !matches!(self.peek(), Token::Eof) {
            clauses.push(self.parse_clause()?);
        }
        Ok(Query { clauses })
    }

    fn parse_clause(&mut self) -> Result<Clause, String> {
        match self.peek() {
            Token::KwMatch => {
                self.bump();
                let mut patterns = vec![self.parse_pattern()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    patterns.push(self.parse_pattern()?);
                }
                Ok(Clause::Match(patterns))
            }
            Token::KwWhere => {
                self.bump();
                Ok(Clause::Where(self.parse_expr()?))
            }
            Token::KwReturn => {
                self.bump();
                let mut items = vec![self.parse_return_item()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    items.push(self.parse_return_item()?);
                }
                Ok(Clause::Return(items))
            }
            Token::KwCreate => {
                self.bump();
                Ok(Clause::Create(self.parse_pattern()?))
            }
            Token::KwDelete => {
                self.bump();
                let mut vars = vec![self.expect_ident()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    vars.push(self.expect_ident()?);
                }
                Ok(Clause::Delete(vars))
            }
            Token::KwSet => {
                self.bump();
                let mut items = vec![self.parse_assignment()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    items.push(self.parse_assignment()?);
                }
                Ok(Clause::Set(items))
            }
            Token::KwMerge => {
                self.bump();
                Ok(Clause::Merge(self.parse_pattern()?))
            }
            other => Err(format!("unexpected clause start {other:?}")),
        }
    }

    fn parse_pattern(&mut self) -> Result<Pattern, String> {
        self.expect(&Token::LParen)?;
        let left = self.parse_pattern_node()?;
        self.expect(&Token::RParen)?;
        if matches!(self.peek(), Token::Dash) {
            self.bump();
            self.expect(&Token::LBracket)?;
            let edge_var = if matches!(self.peek(), Token::Ident(_)) { Some(self.expect_ident()?) } else { None };
            let edge_kind = if matches!(self.peek(), Token::Colon) {
                self.bump();
                Some(self.expect_ident()?)
            } else {
                None
            };
            self.expect(&Token::RBracket)?;
            self.expect(&Token::Arrow)?;
            self.expect(&Token::LParen)?;
            let right = self.parse_pattern_node()?;
            self.expect(&Token::RParen)?;
            Ok(Pattern { nodes: vec![left], edge: Some(PatternEdge { var: edge_var, kind: edge_kind, directed: true, right }) })
        } else {
            Ok(Pattern { nodes: vec![left], edge: None })
        }
    }

    fn parse_pattern_node(&mut self) -> Result<PatternNode, String> {
        let var = self.expect_ident()?;
        self.expect(&Token::Colon)?;
        let kind = self.expect_ident()?;
        Ok(PatternNode { var, kind })
    }

    fn parse_return_item(&mut self) -> Result<ReturnItem, String> {
        let var = self.expect_ident()?;
        if matches!(self.peek(), Token::Dot) {
            self.bump();
            let prop = self.expect_ident()?;
            Ok(ReturnItem::Property { var, prop })
        } else {
            Ok(ReturnItem::Var(var))
        }
    }

    fn parse_assignment(&mut self) -> Result<Assignment, String> {
        let var = self.expect_ident()?;
        self.expect(&Token::Dot)?;
        let prop = self.expect_ident()?;
        self.expect(&Token::Eq)?;
        let value = self.parse_value()?;
        Ok(Assignment { var, prop, value })
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_or_expr()
    }

    fn parse_or_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and_expr()?;
        while matches!(self.peek(), Token::Or) {
            self.bump();
            let right = self.parse_and_expr()?;
            left = Expr::Or(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_and_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_cmp_expr()?;
        while matches!(self.peek(), Token::And) {
            self.bump();
            let right = self.parse_cmp_expr()?;
            left = Expr::And(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_cmp_expr(&mut self) -> Result<Expr, String> {
        let var = self.expect_ident()?;
        self.expect(&Token::Dot)?;
        let prop = self.expect_ident()?;
        match self.bump() {
            Token::Eq => Ok(Expr::Eq { var, prop, value: self.parse_value()? }),
            Token::Ne => Ok(Expr::Ne { var, prop, value: self.parse_value()? }),
            other => Err(format!("expected = or !=, got {other:?}")),
        }
    }

    fn parse_value(&mut self) -> Result<PropertyValue, String> {
        match self.bump() {
            Token::Number(n) => Ok(PropertyValue::Number(n)),
            Token::StringLit(s) => Ok(PropertyValue::String(s)),
            Token::Ident(s) if s.eq_ignore_ascii_case("true") => Ok(PropertyValue::Bool(true)),
            Token::Ident(s) if s.eq_ignore_ascii_case("false") => Ok(PropertyValue::Bool(false)),
            Token::Ident(s) if s.eq_ignore_ascii_case("null") => Ok(PropertyValue::Null),
            other => Err(format!("expected value, got {other:?}")),
        }
    }

    fn expect(&mut self, want: &Token) -> Result<(), String> {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(want) {
            self.bump();
            Ok(())
        } else {
            Err(format!("expected {want:?}, got {:?}", self.peek()))
        }
    }
}

/// 🔍️ Parse a jack query string.
pub fn parse(query: &str) -> Result<Query, String> {
    let tokens = lex(query)?;
    Parser::new(tokens).parse_query()
}
// #endregion 🔖️Parser
/// 🌳️ Span-tracked jack AST node for editor tree/hover/outline surfaces, which need source positions
/// the semantic [`Query`] AST from [`parse`] discards.
#[derive(Clone, Debug, PartialEq)]
pub struct SpannedNode {
    pub kind: String,
    pub label: String,
    pub start: usize,
    pub end: usize,
    pub children: Vec<SpannedNode>,
}

/// 🌊️ Collapses runs of whitespace to a single space, for deriving a node's default display label.
fn collapse_spanned_whitespace(text: &str) -> String {
    let mut out = String::new();
    let mut last_was_space = false;
    for ch in text.chars() {
        if ch.is_whitespace() {
            if !last_was_space {
                out.push(' ');
                last_was_space = true;
            }
        } else {
            out.push(ch);
            last_was_space = false;
        }
    }
    out.trim().to_string()
}

fn spanned_node(kind: &str, start: usize, end: usize, source: &str, children: Vec<SpannedNode>, label: Option<&str>) -> SpannedNode {
    let slice = collapse_spanned_whitespace(source.get(start..end).unwrap_or(""));
    let label = label.map_or_else(|| if slice.is_empty() { kind.to_string() } else { slice }, str::to_string);
    SpannedNode { kind: kind.into(), label, start, end, children }
}

struct SpannedParser<'a> {
    tokens: &'a [SpannedToken],
    source: &'a str,
    pos: usize,
}

impl<'a> SpannedParser<'a> {
    fn new(tokens: &'a [SpannedToken], source: &'a str) -> Self {
        Self { tokens, source, pos: 0 }
    }

    fn peek(&self) -> &SpannedToken {
        &self.tokens[self.pos.min(self.tokens.len() - 1)]
    }

    fn bump(&mut self) -> SpannedToken {
        let token = self.peek().clone();
        if !matches!(token.token, Token::Eof) {
            self.pos += 1;
        }
        token
    }

    fn expect_ident(&mut self) -> Result<(String, usize, usize), String> {
        let token = self.bump();
        match token.token {
            Token::Ident(text) => Ok((text, token.start, token.end)),
            other => Err(format!("expected ident at {}, got {other:?}", token.start)),
        }
    }

    fn expect(&mut self, want: &Token) -> Result<SpannedToken, String> {
        let token = self.bump();
        if std::mem::discriminant(&token.token) == std::mem::discriminant(want) {
            Ok(token)
        } else {
            Err(format!("expected {want:?} at {}, got {:?}", token.start, token.token))
        }
    }

    fn parse_query(&mut self) -> Result<SpannedNode, String> {
        let mut children = Vec::new();
        while !matches!(self.peek().token, Token::Eof) {
            children.push(self.parse_clause()?);
        }
        Ok(spanned_node("query", 0, self.source.len(), self.source, children, Some("Query")))
    }

    fn parse_clause(&mut self) -> Result<SpannedNode, String> {
        let start = self.peek().start;
        match self.peek().token {
            Token::KwMatch => {
                self.bump();
                let mut patterns = vec![self.parse_pattern()?];
                while matches!(self.peek().token, Token::Comma) {
                    self.bump();
                    patterns.push(self.parse_pattern()?);
                }
                let end = patterns.last().map_or(start, |p| p.end);
                Ok(spanned_node("match", start, end, self.source, patterns, Some("MATCH")))
            }
            Token::KwWhere => {
                self.bump();
                let expr = self.parse_expr()?;
                let end = expr.end;
                Ok(spanned_node("where", start, end, self.source, vec![expr], Some("WHERE")))
            }
            Token::KwReturn => {
                self.bump();
                let mut items = vec![self.parse_return_item()?];
                while matches!(self.peek().token, Token::Comma) {
                    self.bump();
                    items.push(self.parse_return_item()?);
                }
                let end = items.last().map_or(start, |i| i.end);
                Ok(spanned_node("return", start, end, self.source, items, Some("RETURN")))
            }
            Token::KwCreate => {
                self.bump();
                let pattern = self.parse_pattern()?;
                let end = pattern.end;
                Ok(spanned_node("create", start, end, self.source, vec![pattern], Some("CREATE")))
            }
            Token::KwDelete => {
                self.bump();
                let mut vars = vec![self.expect_ident()?];
                while matches!(self.peek().token, Token::Comma) {
                    self.bump();
                    vars.push(self.expect_ident()?);
                }
                let end = vars.last().map_or(start, |(_, _, end)| *end);
                let children: Vec<SpannedNode> = vars.iter().map(|(text, vstart, vend)| spanned_node("var", *vstart, *vend, self.source, Vec::new(), Some(text.as_str()))).collect();
                Ok(spanned_node("delete", start, end, self.source, children, Some("DELETE")))
            }
            Token::KwSet => {
                self.bump();
                let mut items = vec![self.parse_assignment()?];
                while matches!(self.peek().token, Token::Comma) {
                    self.bump();
                    items.push(self.parse_assignment()?);
                }
                let end = items.last().map_or(start, |i| i.end);
                Ok(spanned_node("set", start, end, self.source, items, Some("SET")))
            }
            Token::KwMerge => {
                self.bump();
                let pattern = self.parse_pattern()?;
                let end = pattern.end;
                Ok(spanned_node("merge", start, end, self.source, vec![pattern], Some("MERGE")))
            }
            ref other => Err(format!("unexpected clause at {start}, got {other:?}")),
        }
    }

    fn parse_pattern(&mut self) -> Result<SpannedNode, String> {
        let start = self.expect(&Token::LParen)?.start;
        let left = self.parse_pattern_node()?;
        self.expect(&Token::RParen)?;
        if matches!(self.peek().token, Token::Dash) {
            let edge_start = self.bump().start;
            self.expect(&Token::LBracket)?;
            let mut edge_children = Vec::new();
            if matches!(self.peek().token, Token::Ident(_)) {
                let (text, s, e) = self.expect_ident()?;
                edge_children.push(spanned_node("edgeVar", s, e, self.source, Vec::new(), Some(text.as_str())));
            }
            if matches!(self.peek().token, Token::Colon) {
                self.bump();
                let (text, s, e) = self.expect_ident()?;
                edge_children.push(spanned_node("edgeKind", s, e, self.source, Vec::new(), Some(text.as_str())));
            }
            self.expect(&Token::RBracket)?;
            self.expect(&Token::Arrow)?;
            self.expect(&Token::LParen)?;
            let right = self.parse_pattern_node()?;
            self.expect(&Token::RParen)?;
            let edge_end = right.end;
            let edge = spanned_node("edge", edge_start, edge_end, self.source, edge_children, Some("edge"));
            return Ok(spanned_node("pattern", start, edge_end, self.source, vec![left, edge, right], None));
        }
        let end = left.end;
        Ok(spanned_node("pattern", start, end, self.source, vec![left], None))
    }

    fn parse_pattern_node(&mut self) -> Result<SpannedNode, String> {
        let start = self.peek().start;
        let (var_text, _, var_end) = self.expect_ident()?;
        self.expect(&Token::Colon)?;
        let (kind_text, kind_start, kind_end) = self.expect_ident()?;
        let var_node = spanned_node("var", start, var_end, self.source, Vec::new(), Some(var_text.as_str()));
        let kind_node = spanned_node("label", kind_start, kind_end, self.source, Vec::new(), Some(kind_text.as_str()));
        let label = format!("{var_text}:{kind_text}");
        Ok(spanned_node("patternNode", start, kind_end, self.source, vec![var_node, kind_node], Some(label.as_str())))
    }

    fn parse_return_item(&mut self) -> Result<SpannedNode, String> {
        let start = self.peek().start;
        let (var_text, var_start, var_end) = self.expect_ident()?;
        if matches!(self.peek().token, Token::Dot) {
            self.bump();
            let (prop_text, prop_start, prop_end) = self.expect_ident()?;
            let var_node = spanned_node("var", var_start, var_end, self.source, Vec::new(), Some(var_text.as_str()));
            let prop_node = spanned_node("property", prop_start, prop_end, self.source, Vec::new(), Some(prop_text.as_str()));
            let label = format!("{var_text}.{prop_text}");
            return Ok(spanned_node("returnItem", start, prop_end, self.source, vec![var_node, prop_node], Some(label.as_str())));
        }
        let var_node = spanned_node("var", var_start, var_end, self.source, Vec::new(), Some(var_text.as_str()));
        Ok(spanned_node("returnItem", start, var_end, self.source, vec![var_node], Some(var_text.as_str())))
    }

    fn parse_assignment(&mut self) -> Result<SpannedNode, String> {
        let start = self.peek().start;
        let (var_text, var_start, var_end) = self.expect_ident()?;
        self.expect(&Token::Dot)?;
        let (prop_text, prop_start, prop_end) = self.expect_ident()?;
        self.expect(&Token::Eq)?;
        let value = self.parse_value()?;
        let var_node = spanned_node("var", var_start, var_end, self.source, Vec::new(), Some(var_text.as_str()));
        let prop_node = spanned_node("property", prop_start, prop_end, self.source, Vec::new(), Some(prop_text.as_str()));
        let end = value.end;
        Ok(spanned_node("assignment", start, end, self.source, vec![var_node, prop_node, value], None))
    }

    fn parse_expr(&mut self) -> Result<SpannedNode, String> {
        self.parse_or_expr()
    }

    fn parse_or_expr(&mut self) -> Result<SpannedNode, String> {
        let mut left = self.parse_and_expr()?;
        while matches!(self.peek().token, Token::Or) {
            let op_start = self.bump().start;
            let right = self.parse_and_expr()?;
            let end = right.end;
            left = spanned_node("or", op_start, end, self.source, vec![left, right], Some("OR"));
        }
        Ok(left)
    }

    fn parse_and_expr(&mut self) -> Result<SpannedNode, String> {
        let mut left = self.parse_cmp_expr()?;
        while matches!(self.peek().token, Token::And) {
            let op_start = self.bump().start;
            let right = self.parse_cmp_expr()?;
            let end = right.end;
            left = spanned_node("and", op_start, end, self.source, vec![left, right], Some("AND"));
        }
        Ok(left)
    }

    fn parse_cmp_expr(&mut self) -> Result<SpannedNode, String> {
        let start = self.peek().start;
        let (var_text, var_start, var_end) = self.expect_ident()?;
        self.expect(&Token::Dot)?;
        let (prop_text, prop_start, prop_end) = self.expect_ident()?;
        let operation = self.bump();
        if !matches!(operation.token, Token::Eq | Token::Ne) {
            return Err(format!("expected comparison at {}", operation.start));
        }
        let value = self.parse_value()?;
        let var_node = spanned_node("var", var_start, var_end, self.source, Vec::new(), Some(var_text.as_str()));
        let prop_node = spanned_node("property", prop_start, prop_end, self.source, Vec::new(), Some(prop_text.as_str()));
        let kind = if operation.token == Token::Eq { "eq" } else { "ne" };
        let end = value.end;
        Ok(spanned_node(kind, start, end, self.source, vec![var_node, prop_node, value], None))
    }

    fn parse_value(&mut self) -> Result<SpannedNode, String> {
        let token = self.bump();
        match token.token {
            Token::Number(_) => Ok(spanned_node("number", token.start, token.end, self.source, Vec::new(), None)),
            Token::StringLit(text) => Ok(spanned_node("string", token.start, token.end, self.source, Vec::new(), Some(text.as_str()))),
            Token::Ident(text) => {
                let lower = text.to_lowercase();
                if lower == "true" || lower == "false" {
                    Ok(spanned_node("bool", token.start, token.end, self.source, Vec::new(), Some(text.as_str())))
                } else if lower == "null" {
                    Ok(spanned_node("null", token.start, token.end, self.source, Vec::new(), Some("null")))
                } else {
                    Err(format!("expected value at {}", token.start))
                }
            }
            other => Err(format!("expected value at {}, got {other:?}", token.start)),
        }
    }
}

/// 🌳️ Parse jack source into a span-tracked AST for hierarchy/outline panels; lexing never fails —
/// only parsing can, in which case a single `"error"` node spans the source.
pub fn parse_spanned(source: &str) -> SpannedNode {
    let tokens = lex_spanned(source, true).unwrap_or_else(|_| vec![SpannedToken { token: Token::Eof, start: source.len(), end: source.len() }]);
    let mut parser = SpannedParser::new(&tokens, source);
    match parser.parse_query() {
        Ok(node) => node,
        Err(message) => spanned_node("error", 0, source.len(), source, Vec::new(), Some(message.as_str())),
    }
}
// #endregion 🔖️SpannedAst
