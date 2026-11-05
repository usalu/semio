//! 🧾️ `pack_json` — an owned, spec-correct streaming JSON reader and writer: the replacement for

use std::fmt;

pub use semio_framework_pack_json_canonical::{ArtifactCanonicalJsonText, ArtifactCanonicalJsonTextCursor, ArtifactCanonicalJsonTextStep, canonical_escape, canonical_escaped_byte, ArtifactCanonicalJsonNode, ArtifactCanonicalJsonScalarBytes, ArtifactCanonicalDecimalI64, ArtifactCanonicalDecimalU64, ArtifactCanonicalHexWordArray, ArtifactCanonicalHexWordF32, ArtifactCanonicalHexWordList, ArtifactCanonicalJsonTree, ArtifactCanonicalJsonTreeCursor, ArtifactCanonicalJsonTreeStep, ArtifactCanonicalJsonTreeWireCursor, ArtifactCanonicalJsonTreeWireStep};

#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;

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


use semio_framework_value::{RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};





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

#[cfg(feature = "testkit")]
#[doc(hidden)]
pub mod testkit {
    pub use semio_framework_pack_json_canonical::testkit::tree;
    pub type JsonLexeme = super::JsonLexeme;
}

/// 🛬️ Native JSON schema recipients use the actual first-party decoding owner.
pub use semio_framework_value_native_owned::native_decoding;
pub use semio_framework_value_native_owned::native_decoding::OriginalReceivingPrimary;
