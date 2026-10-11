//! 🌳️ Trinity jack query AST.

use crate::{JackSnapshot, PropertyValue};

/// 🌳️ Jack query abstract syntax tree.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct Query {
    pub clauses: Vec<Clause>,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub enum Clause {
    Match(Vec<Pattern>),
    Where(Expr),
    Return(Vec<ReturnItem>),
    Create(Pattern),
    Delete(Vec<String>),
    Set(Vec<Assignment>),
    Merge(Pattern),
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct Pattern {
    pub nodes: Vec<PatternNode>,
    pub edge: Option<PatternEdge>,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct PatternNode {
    pub var: String,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct PatternEdge {
    pub var: Option<String>,
    pub kind: Option<String>,
    pub directed: bool,
    pub right: PatternNode,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub enum ReturnItem {
    Var(String),
    Property { var: String, prop: String },
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct Assignment {
    pub var: String,
    pub prop: String,
    pub value: PropertyValue,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub enum Expr {
    Eq { var: String, prop: String, value: PropertyValue },
    Ne { var: String, prop: String, value: PropertyValue },
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub enum QueryResultKind {
    #[default]
    Table,
    Graph,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct QueryResult {
    #[value(default)]
    pub kind: QueryResultKind,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<PropertyValue>>,
    /// 📦️ Boxed: a `JackSnapshot` is ~330 inline bytes, and the results-window transient that
    /// carries a `QueryResult` moves through the ephemeral transfer lane, whose admission bounds
    /// inline owner metadata at 256 bytes (`ARTIFACT_EPHEMERAL_TRANSFER_MAXIMUM_INLINE_BYTES`) —
    /// the heap payload stays owned either way, only the inline footprint must fit.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub graph_snapshot: Option<Box<JackSnapshot>>,
}

impl semio_framework_dsl_record::DslField for QueryResult {
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Value
    }

    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Value(semio_framework_value::ToValue::to_value(self))
    }

    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Value(value) => semio_framework_value::FromValue::from_value(value.clone()).map_err(|error| error.to_string()),
            other => Err(format!("expected Value, found {other:?}")),
        }
    }
}

impl semio_framework_dsl_record::BorrowedDslField for QueryResult {
    const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Value;
}

impl QueryResult {
    pub fn table(columns: Vec<String>, rows: Vec<Vec<PropertyValue>>) -> Self {
        Self { kind: QueryResultKind::Table, columns, rows, graph_snapshot: None }
    }

    pub fn graph(columns: Vec<String>, graph_snapshot: JackSnapshot) -> Self {
        Self { kind: QueryResultKind::Graph, columns, rows: vec![], graph_snapshot: Some(Box::new(graph_snapshot)) }
    }
}
// #endregion 🔖️Ast
