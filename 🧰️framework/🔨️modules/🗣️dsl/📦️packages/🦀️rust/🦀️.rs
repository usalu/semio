//! 🗣️ Product-neutral lexical, grammar and idiom interfaces.
extern crate self as semio_framework_dsl;
pub use semio_framework_diagnostic::{TextError,Limits,TextSpan,FaultCode,DiagnosticCode,Severity,ExpectedSet,Diagnostic,FaultOrigin,FaultScope,FaultCause,Fault};
pub use semio_framework_value_native_owned::{native_decoding::NativeDecodeControl, native_decoding::NativeDecodeProgress};
pub use semio_framework_dsl_token as token;
pub use semio_framework_dsl_lexical::{lexer,trust};
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
