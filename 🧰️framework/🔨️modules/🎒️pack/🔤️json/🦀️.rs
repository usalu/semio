//! 🧾️ `pack_json` — an owned, spec-correct streaming JSON reader and writer: the replacement for
//! `serde_json` inside framework crates (see `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/
//! INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/📓️p9b-owned-serialization.md`).
//!
//! Whole-document parsing drains the same retained [`JsonParseCursor`] used by interactive owners.
//! The cursor borrows unchanged source text on each grant and keeps partial candidate ownership;
//! [`JsonValueProjection`] moves that candidate into domain-neutral values without payload clones.
//! The token-at-a-time [`Lexer`] shares the same string and number grammar. Arbitrary-precision
//! integers are excluded (JSON numbers outside `[i64::MIN, u64::MAX]` fall back to `f64`, exactly like
//! `serde_json` without its `arbitrary_precision` feature — the only configuration this repo ever
//! built with).
//!
//! Number formatting is byte-identical to `serde_json`'s own (`zmij`-based) writer for every
//! `f64`: the fixed/exponential split follows `zmij`'s rule (`-5 <= exponent <= 15` stays fixed),
//! not ECMA-262's `Number::toString` (`-6 <= e < 21`) — the two differ, and only `zmij`'s matches
//! this repo's actual `serde_json` dependency — and the digits themselves are recomputed by exact
//! round-half-to-even arithmetic so a genuine last-digit tie always resolves the same way `zmij`
//! resolves it (proof: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/
//! RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/🔍️research/📓️float-format-parity.md`).
//! A float and an integer of the same magnitude are still never spelled the same way (`42.0`
//! always keeps its `.0`). NaN/±Infinity — not representable in JSON — encode as `null`, matching
//! `serde_json`'s own behaviour (verified by the differential tests below).

use std::fmt;

#[path = "🛫️encode/🔤️text/🦀️.rs"]
mod native_text;
pub use native_text::{ArtifactCanonicalJsonText, ArtifactCanonicalJsonTextCursor, ArtifactCanonicalJsonTextStep, canonical_escape, canonical_escaped_byte};
#[path = "🛫️encode/🔣️scalar/🦀️.rs"]
mod native_scalar;
pub use native_scalar::{ArtifactCanonicalJsonNode, ArtifactCanonicalJsonScalarBytes};
#[path = "🛫️encode/🧭️tree/🦀️.rs"]
mod native_tree;
pub use native_tree::{ArtifactCanonicalDecimalI64, ArtifactCanonicalDecimalU64, ArtifactCanonicalHexWordArray, ArtifactCanonicalHexWordF32, ArtifactCanonicalHexWordList, ArtifactCanonicalJsonTree, ArtifactCanonicalJsonTreeCursor, ArtifactCanonicalJsonTreeStep, ArtifactCanonicalJsonTreeWireCursor, ArtifactCanonicalJsonTreeWireStep};

use semio_framework_value::{DslValue, FromValue, ValueError, ValueRefusalKind};
pub use semio_framework_value::ToValue;

#[path = "🧩️members/🦀️.rs"]
mod members;
pub use members::JsonMemberPolicy;

#[path = "🚨️json-error/🦀️.rs"]
mod json_error;
pub use json_error::*;

#[path = "🔢️number/🦀️.rs"]
mod number;
pub use number::*;

#[path = "🧱️object-and-value/🦀️.rs"]
mod object_and_value;
pub use object_and_value::*;

#[path = "🌉️dsl-value-bridge/🦀️.rs"]
mod dsl_value_bridge;
pub use dsl_value_bridge::*;

#[path = "🔤️lexer/🦀️.rs"]
mod lexer;
pub use lexer::*;

#[path = "🧮️parser-and-grammar-cursor/🦀️.rs"]
mod parser_and_grammar_cursor;
pub use parser_and_grammar_cursor::*;

#[path = "✍️text-writer/🦀️.rs"]
mod text_writer;
pub use text_writer::*;

use semio_framework_pack_json_float::{write_float_to, ScalarText};

#[path = "🎛️controlled-and-borrowed-bridge/🦀️.rs"]
mod controlled_and_borrowed_bridge;
pub use controlled_and_borrowed_bridge::*;

#[path = "📥️decode/🫳️borrowed/🦀️.rs"]
mod borrowed_read_source;
pub use borrowed_read_source::{JsonReadSource,JsonBorrowedParseCursor,JsonBorrowedDslCursor,JsonParsedValue,JsonReadLimits,JsonSourceCursor};
#[path="📥️decode/🧳️receiving/🦀️.rs"]
pub mod original_receiving;
pub use original_receiving::OriginalJsonReceiving;
#[path="📥️decode/🧵️operation/🦀️.rs"]
mod original_read_operation;
pub use original_read_operation::{JsonReadOperation,JsonReadPolicy};

//#region 🔖️Parser

use semio_framework_value::{RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};

//#endregion 🔖️Parser

//#region 🔖️ToFromValueBridge


//#endregion 🔖️ToFromValueBridge

//#region 🔖️Macro
/// 🧩️ `serde_json::json!` replacement — an object/array literal builder over [`Value`], expanded
/// via the standard TT-muncher recursion (see `json_object_internal!`/`json_array_internal!`,
/// `#[doc(hidden)]`, exported only so this macro's own expansion can call them from any crate).
/// Object keys are string literals (`"key": value`); leaf expressions are borrowed and encoded
/// through [`ToValue`], preserving access to records and fields after constructing the JSON tree.
#[macro_export]
macro_rules! json {
    (null) => { $crate::Value::Null };
    (true) => { $crate::Value::Bool(true) };
    (false) => { $crate::Value::Bool(false) };
    ([]) => { $crate::Value::Array(::std::vec::Vec::new()) };
    ([ $($tt:tt)+ ]) => {
        $crate::Value::Array($crate::json_array_internal!(@collect [] $($tt)+))
    };
    ({}) => { $crate::Value::Object($crate::Object::new()) };
    ({ $($tt:tt)+ }) => {
        $crate::Value::Object({
            let mut __object = $crate::Object::new();
            $crate::json_object_internal!(__object $($tt)+);
            __object
        })
    };
    ($other:expr) => {{
        use $crate::ToValue as _;
        $crate::from_dsl_value(&(&$other).to_value())
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! json_array_internal {
    (@collect [$($elems:expr,)*]) => {
        ::std::vec![$($elems),*]
    };
    (@collect [$($elems:expr,)*] null $(, $($rest:tt)*)?) => {
        $crate::json_array_internal!(@collect [$($elems,)* $crate::json!(null),] $($($rest)*)?)
    };
    (@collect [$($elems:expr,)*] [$($array:tt)*] $(, $($rest:tt)*)?) => {
        $crate::json_array_internal!(@collect [$($elems,)* $crate::json!([$($array)*]),] $($($rest)*)?)
    };
    (@collect [$($elems:expr,)*] {$($object:tt)*} $(, $($rest:tt)*)?) => {
        $crate::json_array_internal!(@collect [$($elems,)* $crate::json!({$($object)*}),] $($($rest)*)?)
    };
    (@collect [$($elems:expr,)*] $next:expr $(, $($rest:tt)*)?) => {
        $crate::json_array_internal!(@collect [$($elems,)* $crate::json!($next),] $($($rest)*)?)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! json_object_internal {
    ($object:ident) => {};
    ($object:ident $key:literal : null $(, $($rest:tt)*)?) => {
        $object.insert($key, $crate::json!(null));
        $crate::json_object_internal!($object $($($rest)*)?);
    };
    ($object:ident $key:literal : [$($array:tt)*] $(, $($rest:tt)*)?) => {
        $object.insert($key, $crate::json!([$($array)*]));
        $crate::json_object_internal!($object $($($rest)*)?);
    };
    ($object:ident $key:literal : {$($inner:tt)*} $(, $($rest:tt)*)?) => {
        $object.insert($key, $crate::json!({$($inner)*}));
        $crate::json_object_internal!($object $($($rest)*)?);
    };
    ($object:ident $key:literal : $value:expr $(, $($rest:tt)*)?) => {
        $object.insert($key, $crate::json!($value));
        $crate::json_object_internal!($object $($($rest)*)?);
    };
}
//#endregion 🔖️Macro

#[cfg(feature = "testkit")]
#[doc(hidden)]
pub mod testkit {
    pub use super::native_tree::testkit as tree;
    pub type JsonLexeme = super::JsonLexeme;
}
