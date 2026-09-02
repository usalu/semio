//! 🌳️ Trinity jack query AST.

use crate::artifacts::jack::{JackSnapshot, PropertyValue};

/// 🌳️ Jack query abstract syntax tree.
#[derive(Clone, Debug, PartialEq)]
pub struct Query {
    pub clauses: Vec<Clause>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Clause {
    Match(Vec<Pattern>),
    Where(Expr),
    Return(Vec<ReturnItem>),
    Create(Pattern),
    Delete(Vec<String>),
    Set(Vec<Assignment>),
    Merge(Pattern),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pattern {
    pub nodes: Vec<PatternNode>,
    pub edge: Option<PatternEdge>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternNode {
    pub var: String,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternEdge {
    pub var: Option<String>,
    pub kind: Option<String>,
    pub directed: bool,
    pub right: PatternNode,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ReturnItem {
    Var(String),
    Property { var: String, prop: String },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Assignment {
    pub var: String,
    pub prop: String,
    pub value: PropertyValue,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Eq { var: String, prop: String, value: PropertyValue },
    Ne { var: String, prop: String, value: PropertyValue },
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum QueryResultKind {
    #[default]
    Table,
    Graph,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct QueryResult {
    #[value(default)]
    pub kind: QueryResultKind,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<PropertyValue>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub graph_fixture: Option<JackSnapshot>,
}

impl QueryResult {
    pub fn table(columns: Vec<String>, rows: Vec<Vec<PropertyValue>>) -> Self {
        Self { kind: QueryResultKind::Table, columns, rows, graph_fixture: None }
    }

    pub fn graph(columns: Vec<String>, graph_fixture: JackSnapshot) -> Self {
        Self { kind: QueryResultKind::Graph, columns, rows: vec![], graph_fixture: Some(graph_fixture) }
    }
}
// #endregion 🔖️Ast
