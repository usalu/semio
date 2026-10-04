//! 🗣️ Product-neutral lexical, grammar and idiom interfaces.
extern crate self as semio_framework_dsl;
pub use semio_framework_diagnostic::{TextError,Limits,TextSpan,FaultCode,DiagnosticCode,Severity,ExpectedSet,Diagnostic,FaultOrigin,FaultScope,FaultCause,Fault};
pub use semio_framework_value::native_decoding::{NativeDecodeControl,NativeDecodeProgress};
#[path = "../../🔤️token/🦀️.rs"]
pub mod token;
#[path = "../../🔍️lexer/🦀️.rs"]
pub mod lexer;
#[path = "../../🎖️trust/🦀️.rs"]
pub mod trust;
#[path = "../../📖️grammar/🦀️.rs"]
pub mod grammar;
#[path = "../../🗣️idiom/🦀️.rs"]
pub mod idiom;
pub use token::*;
pub use lexer::*;
pub use trust::*;
pub use idiom::*;
pub use grammar::{parse_grammar,parse_protocol,print_grammar,print_protocol,verify_protocol_source,walk_protocol,walk_literal_protocol_controlled,Block,Count,Field,FragmentRegistry,Framing,GrammarFile,Prim,ProtocolFile,ProtocolMismatch,ProtocolTrace,Recognizer,SemioDialect,MacroMatcher};
#[cfg(test)]
#[path = "../../📖️grammar/📡️literal/🧪️tests/🦀️.rs"]
mod literal_protocol_tests;
