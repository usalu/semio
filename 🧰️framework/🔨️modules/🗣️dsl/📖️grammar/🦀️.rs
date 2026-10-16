//! 📖️ `dsl_grammar` — the self-hosted `.grammar` spec format: a hand-authorable,
//! EBNF-style description of one language's productions, used as the *normative* artifact every
//! handcrafted grammar in the repo ships alongside its parser/printer. This crate parses and
//! prints the format itself (this crate's own `📖️grammar/📖️grammar.grammar` is written in it and
//! parses cleanly under its own parser — see the `self_hosting` test), and provides a recognizer
//! that can check a target document's tokens against a compiled grammar for the subset of
//! productions this v1 supports (see the Recognizer region's doc comment for exactly what that
//! covers today and what it does not yet).
//!
//! Depends on `dsl_core` only, following the same "own pre-scan lexer delegating the shared
//! alphabet to `crate::lex`" pattern `math::graph::dsl` (Jack) established — `?` and `|`
//! aren't in the shared token alphabet (a structural-DSL alphabet has no need for them), so this
//! crate's lexer pre-scans those two characters itself and hands every other run of characters to
//! `crate::lex` unchanged.

use crate::{lex_with as core_lex_with, CommentDialect, LexOptions, Limits, StringEscape, StringMode, TextError, TextSpan, TokenKind as CoreKind};

#[path="📡️literal/🦀️.rs"]
mod literal;
pub use literal::walk_literal_protocol_controlled;

#[path = "🧱️model/🦀️.rs"]
mod model;
pub use model::{Alternative, Block, Cond, CondOp, Count, DispatchOrder, Field, Framing, GrammarFile, MacroArg, NestedDispatch, Prim, Production, ProtocolFile, ProtocolMismatch, ProtocolTrace, RepeatArm, RepeatDispatch, SemioDialect, Symbol};

#[path = "🔡️lexer/🦀️.rs"]
mod lexer;
use lexer::{GKind, GToken, lex};

#[path = "🌳️parser/🦀️.rs"]
mod parser;
pub use parser::{parse_grammar, parse_protocol};
use parser::is_protocol_source;

#[path = "🖨️writer/🦀️.rs"]
mod writer;
pub use writer::{canonicalize, print_grammar, print_protocol};

#[path = "🧭️recognizer/🦀️.rs"]
mod recognizer;
pub use recognizer::{FragmentRegistry, MacroMatcher, Recognizer};

#[path = "🧮️walker/🦀️.rs"]
mod walker;
use walker::{mismatch, prim_fixed_width};
pub use walker::{verify_protocol_source, walk_protocol};

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
