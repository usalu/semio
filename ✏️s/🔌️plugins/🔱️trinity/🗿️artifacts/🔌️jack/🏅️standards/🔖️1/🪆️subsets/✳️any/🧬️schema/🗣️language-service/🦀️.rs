//! 🗣️ Trinity jack language service — parse, complete, lint, hover.
#![allow(dead_code)]

use crate::standards::v1::subsets::any::schema::content::JackWorkingScene;
use crate::standards::v1::subsets::any::schema::lexer::{lex, lex_spanned, SpannedToken, Token};
use crate::standards::v1::subsets::any::schema::graph::{port_node_id, port_port_id, Camera, Edge, Node, Port};
use crate::standards::v1::subsets::any::schema::memory::Graph;
use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
use crate::standards::v1::subsets::any::schema::manifest::Manifest;
use semio_framework_graph_manifest::{PortDirection, PropertyBag, PropertyValue};
use semio_framework_graph_dsl::{QueryableEdge, QueryableGraph};
use std::collections::BTreeSet;

#[path = "🔗️queryable-graph/🦀️.rs"]
pub mod queryable;

use crate::standards::v1::subsets::any::schema::ast::{Assignment, Clause, Expr, Pattern, PatternEdge, PatternNode, Query, ReturnItem};
pub use queryable::{OwnedTrinityQueryableGraph, TrinityQueryableGraph};
use semio_framework_graph_dsl::{Completion, Diagnostic, DiagnosticSeverity, Hover, SemanticToken};

#[path = "🩺️completion-and-lint/🦀️.rs"]
mod completion_and_lint;
pub use completion_and_lint::*;

#[path = "🧩️example-fixture/🦀️.rs"]
mod example_fixture;
pub use example_fixture::*;

#[path = "🔍️query-parser/🦀️.rs"]
mod query_parser;
pub use query_parser::*;

#[path = "🌳️spanned-parser/🦀️.rs"]
mod spanned_parser;
pub use spanned_parser::*;
