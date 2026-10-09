//! 🧮️ The semio expression framework module: a closed, dimension-aware expression tree, kind inference, a deterministic evaluator, cycle-safe parameter sets and a text syntax.
//!
//! Each domain is a `🦀️.rs` in the owner tree; this entry file is pure wiring.

#[path = "../../🌳️tree/🦀️.rs"]
pub mod tree;

#[path = "../../⚠️errors/🦀️.rs"]
pub mod errors;

#[path = "../../📏️kinds/🦀️.rs"]
pub mod kinds;

#[path = "../../🧮️evaluation/🦀️.rs"]
pub mod evaluation;

#[path = "../../🕸️parameters/🦀️.rs"]
pub mod parameters;

#[path = "../../🔤️syntax/🦀️.rs"]
pub mod syntax;

pub use errors::{ErrorKind, ExprError};
pub use evaluation::evaluate;
pub use kinds::{infer, Kind};
pub use parameters::{dependencies, evaluate_all, evaluate_all_declared, plan, Plan, Resolved};
pub use syntax::{parse, print, print_with_spans, ParseError, ParseErrorKind, Span};
pub use tree::{AngleUnit, AreaUnit, BinaryOp, CompareOp, Expr, Function, LengthUnit, UnaryOp, Value, VolumeUnit};
