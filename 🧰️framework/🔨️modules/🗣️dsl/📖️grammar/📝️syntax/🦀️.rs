//! 📝️ Syntax layer of the self-hosted `.grammar` and protocol formats: the grammar and protocol model, the grammar lexer and the two parsers.
//! The recognizer, the protocol walker and the printer live in `semio_framework_dsl` and build on this crate.

#[path = "../🧱️model/🦀️.rs"]
mod model;
pub use model::{Alternative, Block, Cond, CondOp, Count, DispatchOrder, Field, Framing, GrammarFile, MacroArg, NestedDispatch, Prim, Production, ProtocolFile, ProtocolMismatch, ProtocolTrace, RepeatArm, RepeatDispatch, SemioDialect, Symbol, prim_fixed_width};

#[path = "../🔡️lexer/🦀️.rs"]
mod lexer;
pub use lexer::{GKind, GToken, lex};

#[path = "../🌳️parser/🦀️.rs"]
mod parser;
pub use parser::{is_protocol_source, parse_grammar, parse_protocol};
