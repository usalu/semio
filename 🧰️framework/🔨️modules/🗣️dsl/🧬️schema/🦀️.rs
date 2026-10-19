//! 🧬️ `dsl_schema` — the data-driven declarative grammar engine: technologies describe their
//! document/op grammar as `RecordSpec`/`Shape` DATA (not code), and this crate parses text against
//! that data into a generic `Cst` (walked by typed binders that `dsl_derive` will generate) and
//! prints it back via a chunk `Writer` that structurally guarantees the newline law: every
//! grammar renders both as multi-line canonical `Document` text and as one space-joined `Inline`
//! line, and both re-parse to the same value.

#[cfg(test)]
extern crate self as semio_framework_dsl_record;
use semio_framework_dsl::format_f64;
use semio_framework_dsl::lex;
use semio_framework_dsl::parse_f64;
#[path="🛬️receive/🦀️.rs"]mod native_receive;
pub use semio_framework_diagnostic::{Limits, TextError, TextSpan};
use semio_framework_dsl::SpannedToken;

use semio_framework_dsl::TokenClass;
use semio_framework_dsl::TokenKind;
use std::collections::{HashSet};
#[cfg(test)] use std::collections::{HashMap};
use crate::notation as dsl_notation;
pub use semio_framework_dsl_notation as notation;
use semio_framework_value::{DslValue, NativeEncodeControl, Number};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_framework_value::{ValueError,ValueRefusalKind};

#[path = "🛬️decoding/🦀️.rs"]
mod controlled_decoding;
pub use controlled_decoding::{parse_exact_controlled,parse_expr_text_controlled};

pub use semio_framework_dsl_record_emission::*;

pub use semio_framework_dsl_record_model::*;

#[path = "🧭️cursor-and-expressions/🦀️.rs"]
mod cursor_and_expressions;
pub use cursor_and_expressions::*;

#[path = "🗺️shape-parsing/🦀️.rs"]
mod shape_parsing;
use shape_parsing::*;

#[path = "📜️dsl-value-and-wire/🦀️.rs"]
mod dsl_value_and_wire;
pub use dsl_value_and_wire::*;

#[path = "📒️records-and-tables/🦀️.rs"]
mod records_and_tables;
use records_and_tables::*;

#[path = "🔎️canonicalize-and-language-service/🦀️.rs"]
mod canonicalize_and_language_service;
pub use canonicalize_and_language_service::*;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🪆️refusal/🦀️.rs"]
mod field_refusal_tests;

#[path="🪆️binding/🎮️operation/🦀️.rs"]
pub mod variants_text;
