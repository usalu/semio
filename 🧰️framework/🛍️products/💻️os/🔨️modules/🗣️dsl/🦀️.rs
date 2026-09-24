//! 🧬️ `dsl` — facade for the token-native declarative DSL engine. Technologies depend on this one
//! crate (plus `vcs` for the `ArtifactDsl`/`OpText` trait definitions themselves) to get the
//! derive macros, the `DslField` binding trait primitive Rust types implement, and the `__rt`
//! runtime the generated code calls into.

// The derive macros emit `::crate::os_dsl::...` paths so generated code reads identically regardless of
// which technology crate invokes them. That only resolves for the crates that depend on `dsl` as
// an external crate — which is every real consumer, but NOT this crate's own tests (a crate is
// never its own dependency). `// extern crate self removed after merge` is the standard fix: it makes `::dsl`
// resolve to this crate even when the derive is exercised in-crate, as the `🧪️Tests` region below does.
// Only needed for the in-crate tests, so it's cfg-gated to avoid an "unused extern crate" warning
// in ordinary (non-test) builds, where every real consumer already has `dsl` as a true dependency.
// extern crate self removed after merge

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

#[path = "🪟️viewport/🦀️.rs"]
mod viewport;

pub use crate::os_dsl::schema::*;
pub use crate::os_dsl::{diagnostic::*, lexer::*, span::*, token::*, trust::*};
pub use dsl_derive::{DslArtifact, DslDiff, DslEnum, DslOps, DslRecord, DslScalar, MutationLeaf, Mutations};

pub use crate::os_dsl::grammar::{
    parse_grammar, parse_protocol, print_grammar, print_protocol, verify_protocol_bytes, verify_protocol_source, walk_protocol, Block, Count, Field, FragmentRegistry, Framing, GrammarFile, Prim, ProtocolFile, ProtocolMismatch, ProtocolTrace,
    Recognizer, SemioDialect,
};
pub use crate::os_dsl::schema::{from_dsl_value, to_dsl_value};

//#region 🔖️Field
/// @emoji 🔗️ Bridges a concrete Rust field type to the engine's `Shape`/`FieldValue` — every
/// primitive implements it directly; `#[derive(DslRecord)]`/`#[derive(DslScalar)]` implement it
/// for technology-declared nested types, so composition (a record field whose type is another
/// derived record or enum) works transparently through the same trait.
pub trait DslField: Sized {
    // 🚫️async: E4 fn-pointer transitivity — `Shape::Record`/`Table`/`Statements` hold
    // `fn() -> RecordSpec`; every `shape()` implementation ultimately feeds one, directly or
    // through a derived `__dsl_spec` — see R9.
    fn shape() -> Shape;
    fn to_value(&self) -> FieldValue;
    fn from_value(value: &FieldValue) -> Result<Self, String>;
}

/// 📦️ Boxed ownership preserves the inner field's schema, value, and decoding errors.
impl<T: DslField> DslField for Box<T> {
    fn shape() -> Shape {
        T::shape()
    }
    fn to_value(&self) -> FieldValue {
        T::to_value(self.as_ref())
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        T::from_value(value).map(Box::new)
    }
}

macro_rules! impl_dsl_field_int {
    ($ty:ty, $shape:expr, $variant:ident, $as_ty:ty) => {
        impl DslField for $ty {
            // 🚫️async: E4 — see `DslField::shape`'s tag above.
            fn shape() -> Shape {
                $shape
            }
            fn to_value(&self) -> FieldValue {
                FieldValue::$variant(*self as $as_ty)
            }
            fn from_value(value: &FieldValue) -> Result<Self, String> {
                match value {
                    FieldValue::$variant(v) => <$ty>::try_from(*v).map_err(|_| format!("integer {v} out of range for {}", stringify!($ty))),
                    other => Err(format!("expected {}, found {other:?}", stringify!($variant))),
                }
            }
        }
    };
}

impl_dsl_field_int!(i8, Shape::Int, Int, i64);
impl_dsl_field_int!(i16, Shape::Int, Int, i64);
impl_dsl_field_int!(i32, Shape::Int, Int, i64);
impl_dsl_field_int!(i64, Shape::Int, Int, i64);
impl_dsl_field_int!(isize, Shape::Int, Int, i64);
impl_dsl_field_int!(u8, Shape::UInt, UInt, u64);
impl_dsl_field_int!(u16, Shape::UInt, UInt, u64);
impl_dsl_field_int!(u32, Shape::UInt, UInt, u64);
impl_dsl_field_int!(u64, Shape::UInt, UInt, u64);
impl_dsl_field_int!(usize, Shape::UInt, UInt, u64);

impl DslField for bool {
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Bool
    }
    fn to_value(&self) -> FieldValue {
        FieldValue::Bool(*self)
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Bool(b) => Ok(*b),
            other => Err(format!("expected Bool, found {other:?}")),
        }
    }
}

impl DslField for f32 {
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Float
    }
    fn to_value(&self) -> FieldValue {
        FieldValue::Float(*self as f64)
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Float(f) => Ok(*f as f32),
            other => Err(format!("expected Float, found {other:?}")),
        }
    }
}

impl DslField for f64 {
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Float
    }
    fn to_value(&self) -> FieldValue {
        FieldValue::Float(*self)
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Float(f) => Ok(*f),
            other => Err(format!("expected Float, found {other:?}")),
        }
    }
}

/// @emoji 🔤️ `String` binds as `Shape::Text` — the one string shape. The parser accepts either a
/// bare `Ident` token or a quoted `Text` token wherever `Text` is expected; the printer emits bare
/// (unquoted) whenever `crate::os_dsl::is_bare_ident` holds for the value, quoted+escaped otherwise —
/// so bare-vs-quoted is entirely a printing decision now, not a separate shape a field opts into.
impl DslField for String {
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Text
    }
    fn to_value(&self) -> FieldValue {
        FieldValue::Text(self.clone())
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Text(s) => Ok(s.clone()),
            other => Err(format!("expected Text, found {other:?}")),
        }
    }
}

/// @emoji 🔌️ A wire literal as a plain struct field (or inside a `#[dsl(table)]` `Vec` as a
/// `WIRE`-typed column) — thin `DslField` wrapper around `crate::os_dsl::schema::WireValue` so adopter
/// technologies never need to hand-roll their own `Shape::Wire` binding.
#[derive(Clone, Debug, PartialEq)]
pub struct Wire(pub WireValue);

impl DslField for Wire {
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Wire
    }
    fn to_value(&self) -> FieldValue {
        FieldValue::Wire(self.0.clone())
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Wire(w) => Ok(Wire(w.clone())),
            other => Err(format!("expected Wire, found {other:?}")),
        }
    }
}
/// @emoji 📚️ General recursion seam: `#[derive(DslRecord)]`/`#[derive(DslScalar)]` fields classify
/// `Vec<T>`/`[T; N]` directly (so their own printed shape stays field-specific), but a NESTED
/// collection — `Vec<Vec<T>>`, a fixed-size array field, ... — needs its inner element type to
/// satisfy `DslField` itself. These two blanket impls close that gap generically instead of adding
/// a special-cased `FieldKind` for every depth of nesting.
impl<T: DslField> DslField for Vec<T> {
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::List(Box::new(T::shape()))
    }
    // 🔁 `Iterator::map` cannot await per-element (residue shape 1) and `T::to_value`/`from_value`
    // are AFIT over an arbitrary implementor, so — unlike a known-pure leaf fn — R9 does not apply;
    // the fix is a plain sequential loop that awaits each element in turn.
    fn to_value(&self) -> FieldValue {
        let mut items = Vec::with_capacity(self.len());
        for item in self {
            items.push(item.to_value());
        }
        FieldValue::List(items)
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::List(items) => {
                let mut out = Vec::with_capacity(items.len());
                for item in items {
                    out.push(T::from_value(item)?);
                }
                Ok(out)
            }
            other => Err(format!("expected List, found {other:?}")),
        }
    }
}

/// @emoji 🗺️ Same recursion seam as `Vec<T>`, for a `BTreeMap<String, T>` that's itself nested
/// (e.g. `Option<BTreeMap<String, T>>`) rather than a bare top-level field — `#[derive(DslRecord)]`
/// classifies a *bare* `BTreeMap<String, T>` field directly via its own dedicated `FieldKind`
/// (same `Shape::Map` this produces), so the two never conflict.
impl<T: DslField> DslField for std::collections::BTreeMap<String, T> {
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Map(Box::new(T::shape()))
    }
    // 🔁 Same R9-doesn't-apply reasoning as `Vec<T>` above: sequential loop, not `.map().collect()`.
    fn to_value(&self) -> FieldValue {
        let mut entries = Vec::with_capacity(self.len());
        for (k, v) in self {
            entries.push((k.clone(), v.to_value()));
        }
        FieldValue::Map(entries)
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Map(entries) => {
                let mut out = Self::new();
                for (k, v) in entries {
                    out.insert(k.clone(), T::from_value(v)?);
                }
                Ok(out)
            }
            other => Err(format!("expected Map, found {other:?}")),
        }
    }
}

/// @emoji 📐️ Fixed-arity `Shape::Tuple(_, Some(N))` — a packed `x,y,z`-style literal for any `N`.
impl<T: DslField, const N: usize> DslField for [T; N] {
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Tuple(Box::new(T::shape()), Some(N))
    }
    // 🔁 Same R9-doesn't-apply reasoning as `Vec<T>` above: sequential loop, not `.map().collect()`.
    fn to_value(&self) -> FieldValue {
        let mut items = Vec::with_capacity(N);
        for item in self {
            items.push(item.to_value());
        }
        FieldValue::Tuple(items)
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Tuple(items) if items.len() == N => {
                let mut converted: Vec<T> = Vec::with_capacity(N);
                for item in items {
                    converted.push(T::from_value(item)?);
                }
                converted.try_into().map_err(|_| format!("expected {N} items, got a length mismatch"))
            }
            other => Err(format!("expected a {N}-item Tuple, found {other:?}")),
        }
    }
}

/// @emoji 🌱️ Schema-less dynamic literal — binds as `Shape::Value`.
impl DslField for DslValue {
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Value
    }
    fn to_value(&self) -> FieldValue {
        FieldValue::Value(self.clone())
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Value(dsl_value) => Ok(dsl_value.clone()),
            other => Err(format!("expected Value, found {other:?}")),
        }
    }
}
//#endregion 🔖️Field

//#region 🔖️Variants
/// @emoji 🌿️ Bridges an enum whose variants are each their own keyword-tagged record — the type
/// bound for `#[dsl(statements)] Vec<T>` collection fields and for `#[derive(DslOps)]` operation
/// enums. `#[derive(DslEnum)]`-with-struct-variants and `#[derive(DslOps)]` both implement this.
pub trait DslVariants: Sized {
    /// @emoji 🐌️ Lazy: each entry is a zero-capture `fn` pointer, not an eagerly-built `RecordSpec`
    /// — a self-referential grammar's own `variants()` would otherwise need to recurse infinitely
    /// just to construct this list. See [`Shape::Statements`]'s doc comment for the full rationale.
    // 🚫️async: E4 — the returned `Vec<(String, fn() -> RecordSpec)>` IS a fn-pointer table, and
    // `Shape::Statements(<T>::variants())` is itself called from inside a sync `__dsl_spec` — see R9.
    fn variants() -> Vec<(String, fn() -> RecordSpec)>;
    fn to_named_record(&self) -> (String, RecordValue);
    /// @emoji ⚠️ Returns `TextError` (not `String`, unlike [`DslField::from_value`]) so
    /// generated bodies can `?`-propagate it directly — this is the same error type
    /// `crate::os_spr::OpText::parse_op`/`crate::os_store::ArtifactDsl::parse_dsl` already return, and the derive's
    /// `#[dsl(statements)]` field codegen composes it without any conversion at every nesting depth.
    fn from_named_record(keyword: &str, record: &RecordValue) -> Result<Self, TextError>;
}
//#endregion 🔖️Variants

//#region 🔖️Runtime
/// @emoji ⚙️ Helpers remaining after P6 flag day — DslField/DslVariants derive bodies only (codec paths deleted).
pub mod __rt {
    use super::*;

    // 🚫️async: E1 pure error constructor, consumed by `Option::ok_or_else` sync closures in every
    // `#[derive(DslRecord)]`-generated body (`✨️derive/🦀️.rs`'s `quote!{}` templates) — see R9
    pub fn field_error(message: impl Into<String>) -> TextError {
        TextError::new(message, TextSpan::at(1, 1))
    }

    /// @emoji 📐️ Resolves a `#[dsl(unit = "...")]`/`#[dsl(angle = "...")]` symbol at spec-build
    /// time. An unknown symbol is a derive-time misuse (a typo'd unit string, caught the first time
    /// the generated `__dsl_spec` runs — every RecordSpec-law test exercises this), so it panics
    /// rather than threading a `Result` through the whole spec-building call chain, matching
    /// `newtype_variant_spec`'s convention above.
    pub fn unit_for_derive(symbol: &'static str) -> &'static UnitSpec {
        unit_by_symbol(symbol).unwrap_or_else(|| panic!("dsl: unknown unit symbol '{symbol}' in #[dsl(unit = ...)]/#[dsl(angle = ...)]"))
    }

    /// @emoji 📦️ Single-field tuple ("newtype") enum variant support — `Variant(Body)` delegates its
    /// whole `RecordSpec`/value to `Body`'s own `DslField` impl rather than wrapping it in one
    /// positional field, so `Body` prints/parses identically whether reached through the enum or on
    /// its own. `Body` must have `Shape::Record` (i.e. itself come from `#[derive(DslRecord)]` or
    /// `#[derive(DslArtifact)]`) — anything else is a derive-time misuse, hence the panic rather than
    /// a `Result` (there is no sensible recoverable path for a grammar that's wrong at compile time).
    // 🚫️async: E4 — this fn's VALUE is cast `as fn() -> RecordSpec` at every newtype-variant call
    // site (`✨️derive/🦀️.rs`'s `dsl_variants_codegen`), and it calls the now-sync `DslField::shape`.
    pub fn newtype_variant_spec<T: DslField>() -> RecordSpec {
        match T::shape() {
            Shape::Record(spec_fn) => spec_fn(),
            other => panic!("newtype variant's inner type must have Record shape, found {other:?}"),
        }
    }

    pub fn newtype_variant_to_record<T: DslField>(inner: &T) -> RecordValue {
        match inner.to_value() {
            FieldValue::Record(record) => record,
            other => panic!("newtype variant's inner type must produce a Record value, found {other:?}"),
        }
    }

    pub fn newtype_variant_from_record<T: DslField>(record: &RecordValue) -> Result<T, TextError> {
        T::from_value(&FieldValue::Record(record.clone())).map_err(field_error)
    }
}

//#endregion 🔖️Runtime

//#region 🔖️OpTextRt
/// @emoji 🔤️ Handcrafted `OpText` helper — the text twin of [`variants_binary`].
///
/// An operation line is ONE terminal keyword-tagged record, so it parses through
/// [`parse_exact`], which rejects every token outside the variant's own schema body: a trailing
/// `unknown-field 1` is not a second statement, it is garbage the line must refuse. Plain
/// [`parse`] stops at the end of the record it recognises and silently drops the rest, which is
/// the document-mode contract, not the op-line one.
pub mod variants_text {
    use super::__rt::field_error;
    use super::{print, DslVariants, JoinMode, Limits, ParseOptions, SourceMode, TextError};

    pub fn parse_op<T: DslVariants>(line: &str) -> Result<T, TextError> {
        let variants = T::variants();
        for (keyword, spec_fn) in &variants {
            if line == keyword.as_str() || line.starts_with(&format!("{keyword} ")) {
                let record = super::parse_exact(line, &spec_fn(), &ParseOptions { limits: Limits::default(), mode: SourceMode::Inline })?;
                return T::from_named_record(keyword, &record);
            }
        }
        Err(field_error(format!("unknown operation line '{line}'")))
    }

    pub fn print_op<T: DslVariants>(op: &T) -> String {
        let (keyword, record) = op.to_named_record();
        let variants = T::variants();
        let spec_fn = variants.iter().find(|(key, _)| key == &keyword).map(|(_, spec)| *spec).expect("variant spec must exist for its own keyword");
        print(&record, &spec_fn(), JoinMode::Inline)
    }
}
//#endregion 🔖️OpTextRt

//#region 🏷️ProtocolRecord
/// @emoji 🏷️ The one source of a mutation vocabulary's op tags: the `record <kind> tag=<n>` lines of its
/// `💾️binary/📡️.protocol.semio`. Every codec derives its tags from here at compile time, so a kind whose
/// record is missing or duplicated fails the build instead of drifting from the wire.
/// See [`crate::os_dsl::grammar::parse_protocol`] for the full dialect this scanner agrees with.
pub mod protocol_record {
    const fn is_space(byte: u8) -> bool {
        byte == b' ' || byte == b'\t'
    }

    const fn is_name(byte: u8) -> bool {
        byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_' || byte == b'.'
    }

    const fn skip_space(bytes: &[u8], mut at: usize) -> usize {
        while at < bytes.len() && is_space(bytes[at]) {
            at += 1;
        }
        at
    }

    const fn line_end(bytes: &[u8], mut at: usize) -> usize {
        while at < bytes.len() && bytes[at] != b'\n' {
            at += 1;
        }
        at
    }

    const fn starts_with(bytes: &[u8], at: usize, prefix: &[u8]) -> bool {
        if at + prefix.len() > bytes.len() {
            return false;
        }
        let mut index = 0;
        while index < prefix.len() {
            if bytes[at + index] != prefix[index] {
                return false;
            }
            index += 1;
        }
        true
    }

    /// 🔎️ Parses the record header at `line`: `(name start, name end, tag)`, or `None` for any other line.
    const fn record_at(bytes: &[u8], line: usize) -> Option<(usize, usize, u64)> {
        let at = skip_space(bytes, line);
        if !starts_with(bytes, at, b"record") || at + 6 >= bytes.len() || !is_space(bytes[at + 6]) {
            return None;
        }
        let name_start = skip_space(bytes, at + 6);
        let mut name_end = name_start;
        while name_end < bytes.len() && is_name(bytes[name_end]) {
            name_end += 1;
        }
        let at = skip_space(bytes, name_end);
        if name_end == name_start || !starts_with(bytes, at, b"tag=") {
            return None;
        }
        let mut at = at + 4;
        let digits = at;
        let mut tag: u64 = 0;
        while at < bytes.len() && bytes[at].is_ascii_digit() {
            tag = tag * 10 + (bytes[at] - b'0') as u64;
            at += 1;
        }
        if at == digits {
            return None;
        }
        Some((name_start, name_end, tag))
    }

    const fn name_equals(bytes: &[u8], start: usize, end: usize, kind: &[u8]) -> bool {
        end - start == kind.len() && starts_with(bytes, start, kind)
    }

    /// 🔢️ `(tag, occurrences)` of `kind` across every record line.
    const fn scan(protocol: &str, kind: &str) -> (u64, usize) {
        let bytes = protocol.as_bytes();
        let kind = kind.as_bytes();
        let mut line = 0;
        let mut found = 0;
        let mut tag = 0;
        while line < bytes.len() {
            if let Some((start, end, value)) = record_at(bytes, line) {
                if name_equals(bytes, start, end, kind) {
                    found += 1;
                    tag = value;
                }
            }
            line = line_end(bytes, line) + 1;
        }
        (tag, found)
    }

    /// 🏷️ The tag `kind`'s record declares; a missing or duplicated record is a const-evaluation error.
    pub const fn tag(protocol: &str, kind: &str) -> u64 {
        match scan(protocol, kind) {
            (tag, 1) => tag,
            (_, 0) => panic!("📡️.protocol.semio declares no `record <kind> tag=<n>` for this mutation kind"),
            _ => panic!("📡️.protocol.semio declares this mutation kind more than once"),
        }
    }

    /// 🏷️ [`tag`] for a codec whose wire tag is one byte; a tag above 255 is a const-evaluation error.
    pub const fn tag_u8(protocol: &str, kind: &str) -> u8 {
        let tag = tag(protocol, kind);
        assert!(tag <= u8::MAX as u64, "📡️.protocol.semio record tag does not fit the codec's u8 tag field");
        tag as u8
    }

    /// 🏷️ [`tag`] for a codec whose wire tag is a `u32`; a tag above `u32::MAX` is a const-evaluation error.
    pub const fn tag_u32(protocol: &str, kind: &str) -> u32 {
        let tag = tag(protocol, kind);
        assert!(tag <= u32::MAX as u64, "📡️.protocol.semio record tag does not fit the codec's u32 tag field");
        tag as u32
    }

    /// 📇️ Every `(kind, tag)` record, in file order.
    pub fn records(protocol: &str) -> impl Iterator<Item = (&str, u64)> {
        let bytes = protocol.as_bytes();
        let mut line = 0;
        std::iter::from_fn(move || {
            while line < bytes.len() {
                let current = line;
                line = line_end(bytes, line) + 1;
                if let Some((start, end, tag)) = record_at(bytes, current) {
                    return Some((&protocol[start..end], tag));
                }
            }
            None
        })
    }

    /// 🔁️ The kind whose record declares `tag`.
    pub fn kind(protocol: &str, tag: u64) -> Option<&str> {
        records(protocol).find(|(_, value)| *value == tag).map(|(kind, _)| kind)
    }
}
//#endregion 🏷️ProtocolRecord

//#region 🔖️OpRt
/// @emoji 🎯️ Handcrafted OpBinary helper (P6): layout `format u8 (=1) | tag varint | record body`.
/// `encode_tagged_op`/`decode_tagged_op` take the tag from the vocabulary's `📡️.protocol.semio` record
/// ([`super::protocol_record`]); `encode_op`/`decode_op` serve the ephemeral layers that carry no wire
/// protocol facet, whose tag is the variant ordinal.
/// Called explicitly from handcrafted `protocol::OpBinary` impls — never re-emitted by derive.
pub mod variants_binary {
    use super::{protocol_record, DslVariants};
    use crate::os_pack::{decode_record_body_exact, encode_record_body, write_varint_u64, ByteReader, DecodeOptions, EncodeOptions};
    use crate::os_spr::ProtocolError;

    pub const OP_BINARY_FORMAT: u8 = 1;

    fn encode_with<T: DslVariants>(op: &T, tag_of: impl Fn(&str, usize) -> Result<u64, ProtocolError>) -> Result<Vec<u8>, ProtocolError> {
        let (keyword, record) = op.to_named_record();
        let variants = T::variants();
        let ordinal = variants.iter().position(|(k, _)| k == &keyword).ok_or(ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword '{keyword}' missing from variants()") })?;
        let tag = tag_of(&keyword, ordinal)?;
        let spec = variants[ordinal].1();
        let body = encode_record_body(&spec, &record, &EncodeOptions::default()).map_err(ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        write_varint_u64(&mut out, tag);
        out.extend_from_slice(&body);
        Ok(out)
    }

    fn decode_with<T: DslVariants>(bytes: &[u8], index_of: impl Fn(u64, &[(String, fn() -> super::RecordSpec)]) -> Result<usize, ProtocolError>, reencode: impl Fn(&T) -> Result<Vec<u8>, ProtocolError>) -> Result<T, ProtocolError> {
        let mut reader = ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let tag = reader.read_varint_u64()?;
        let variants = T::variants();
        let index = index_of(tag, &variants)?;
        let (keyword, spec_fn) = &variants[index];
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let record = decode_record_body_exact(body, &spec, &DecodeOptions::default()).map_err(ProtocolError::from)?;
        let record_offset = reader.position() as u64;
        let decoded = T::from_named_record(keyword, &record).map_err(|error| ProtocolError::Malformed { what: "op record", offset: record_offset, detail: error.to_string() })?;
        if reencode(&decoded)?.as_slice() != bytes {
            return Err(ProtocolError::Malformed { what: "op encoding", offset: 0, detail: "operation bytes are not canonical".into() });
        }
        Ok(decoded)
    }

    /// 🏷️ Encodes `op` with the tag its kind's record declares in `protocol`.
    pub fn encode_tagged_op<T: DslVariants>(protocol: &str, op: &T) -> Result<Vec<u8>, ProtocolError> {
        encode_with(op, |keyword, _| protocol_record::records(protocol).find(|(kind, _)| *kind == keyword).map(|(_, tag)| tag).ok_or(ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("📡️.protocol.semio declares no record for '{keyword}'") }))
    }

    /// 🏷️ Decodes an op whose tag names its kind's record in `protocol`.
    pub fn decode_tagged_op<T: DslVariants>(protocol: &str, bytes: &[u8]) -> Result<T, ProtocolError> {
        decode_with(
            bytes,
            |tag, variants| {
                let kind = protocol_record::kind(protocol, tag).ok_or(ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("📡️.protocol.semio declares no record with tag {tag}") })?;
                variants.iter().position(|(keyword, _)| keyword == kind).ok_or(ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("record '{kind}' names no variant") })
            },
            |decoded| encode_tagged_op(protocol, decoded),
        )
    }

    pub fn encode_op<T: DslVariants>(op: &T) -> Result<Vec<u8>, ProtocolError> {
        encode_with(op, |_, ordinal| u64::try_from(ordinal).map_err(|_| ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} exceeds the u64 wire range") }))
    }

    pub fn decode_op<T: DslVariants>(bytes: &[u8]) -> Result<T, ProtocolError> {
        decode_with(
            bytes,
            |ordinal, variants| {
                let index = usize::try_from(ordinal).map_err(|_| ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} exceeds the native index range") })?;
                if index < variants.len() { Ok(index) } else { Err(ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) }) }
            },
            |decoded| encode_op(decoded),
        )
    }
}
//#endregion 🔖️OpRt

//#region 🏷️TaggedValueRt
/// @emoji 🏷️ Op frame for a mutation aggregate whose payload is its `ToValue` tree: `format u8 (=1) | tag varint
/// | wire value (`pack_rt::encode_wire_value`) of the variant's value with its variant name removed`. The tag is
/// the variant kind's `record <kind> tag=<n>` in the vocabulary's `📡️.protocol.semio` ([`super::protocol_record`]),
/// so the wire never spells the variant name and the protocol file is the only source of tags.
pub mod tagged_value_binary {
    use super::protocol_record;
    use crate::os_dsl::schema::{DslValue, FromValue, ToValue};
    use crate::os_pack::{write_varint_u64, ByteReader};
    use crate::os_spr::ProtocolError;
    use crate::os_store::pack_rt::{decode_wire_value, encode_wire_value};

    pub const OP_BINARY_FORMAT: u8 = 1;

    /// 🧭️ Where the aggregate's `ToValue` tree names its variant.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum VariantTag {
        /// `#[value(tag = "…")]`, camelCase variant names, with or without `content`.
        Field(&'static str),
        /// Externally tagged, PascalCase variant names: `{"Variant": payload}`.
        Key,
    }

    fn malformed(what: &'static str, offset: u64, detail: String) -> ProtocolError {
        ProtocolError::Malformed { what, offset, detail }
    }

    fn kebab(name: &str) -> String {
        let mut out = String::with_capacity(name.len() + 4);
        for (index, ch) in name.char_indices() {
            if ch.is_ascii_uppercase() {
                if index > 0 {
                    out.push('-');
                }
                out.push(ch.to_ascii_lowercase());
            } else {
                out.push(ch);
            }
        }
        out
    }

    fn cased(kind: &str, pascal: bool) -> String {
        let mut out = String::with_capacity(kind.len());
        let mut upper = pascal;
        for ch in kind.chars() {
            if ch == '-' {
                upper = true;
            } else if upper {
                out.push(ch.to_ascii_uppercase());
                upper = false;
            } else {
                out.push(ch);
            }
        }
        out
    }

    /// 🏷️ Encodes `op` under its kind's record tag.
    pub fn encode_op<T: ToValue>(protocol: &str, tagging: VariantTag, op: &T) -> Result<Vec<u8>, ProtocolError> {
        let DslValue::Object(mut entries) = op.to_value() else { return Err(malformed("op value", 0, "a mutation aggregate's value must be an object".into())) };
        let (variant, payload) = match tagging {
            VariantTag::Field(key) => {
                let position = entries.iter().position(|(name, _)| name == key).ok_or_else(|| malformed("op value", 0, format!("value carries no `{key}` variant field")))?;
                let (_, variant) = entries.remove(position);
                let DslValue::String(variant) = variant else { return Err(malformed("op value", 0, format!("`{key}` is not a string"))) };
                (variant, DslValue::Object(entries))
            }
            VariantTag::Key => {
                let mut entries = entries.into_iter();
                let (Some((variant, payload)), None) = (entries.next(), entries.next()) else { return Err(malformed("op value", 0, "an externally tagged value must hold exactly one variant".into())) };
                (variant, payload)
            }
        };
        let kind = kebab(&variant);
        let tag = protocol_record::records(protocol).find(|(record, _)| *record == kind).map(|(_, tag)| tag).ok_or_else(|| malformed("op tag", 1, format!("📡️.protocol.semio declares no record for '{kind}'")))?;
        let body = encode_wire_value(&payload);
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        write_varint_u64(&mut out, tag);
        out.extend_from_slice(&body);
        Ok(out)
    }

    /// 🏷️ Decodes an op whose tag names its kind's record.
    pub fn decode_op<T: FromValue>(protocol: &str, tagging: VariantTag, bytes: &[u8]) -> Result<T, ProtocolError> {
        let mut reader = ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(malformed("op format", 0, format!("unsupported op format {format}")));
        }
        let tag = reader.read_varint_u64()?;
        let kind = protocol_record::kind(protocol, tag).ok_or_else(|| malformed("op tag", 1, format!("📡️.protocol.semio declares no record with tag {tag}")))?;
        let offset = reader.position();
        let payload = decode_wire_value(&bytes[offset..]).map_err(|error| malformed("op payload", offset as u64, error.to_string()))?;
        let value = match tagging {
            VariantTag::Field(key) => {
                let DslValue::Object(mut entries) = payload else { return Err(malformed("op payload", offset as u64, "payload must be an object".into())) };
                entries.insert(0, (key.to_string(), DslValue::String(cased(kind, false))));
                DslValue::Object(entries)
            }
            VariantTag::Key => DslValue::Object(vec![(cased(kind, true), payload)]),
        };
        T::from_value(value).map_err(|error| malformed("op value", offset as u64, error.to_string()))
    }
}
//#endregion 🏷️TaggedValueRt

//#region 🏷️TaggedTextRt
/// @emoji 🏷️ Op frame for a mutation aggregate whose canonical payload is its own `OpText` line `<kind> <args>`:
/// `format u8 (=1) | tag varint | args utf-8`. The tag is the kind's `record <kind> tag=<n>`, so the keyword never
/// travels and the protocol file stays the only source of tags. Used where the `ToValue` tree is lossy.
pub mod tagged_text_binary {
    use super::protocol_record;
    use crate::os_pack::{write_varint_u64, ByteReader};
    use crate::os_spr::ProtocolError;

    pub const OP_BINARY_FORMAT: u8 = 1;

    /// 🏷️ Encodes one printed op line under its keyword's record tag.
    pub fn encode_line(protocol: &str, line: &str) -> Result<Vec<u8>, ProtocolError> {
        let (keyword, args) = line.split_once(' ').unwrap_or((line, ""));
        let tag = protocol_record::records(protocol).find(|(kind, _)| *kind == keyword).map(|(_, tag)| tag).ok_or_else(|| ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("📡️.protocol.semio declares no record for '{keyword}'") })?;
        let mut out = Vec::with_capacity(args.len() + 3);
        out.push(OP_BINARY_FORMAT);
        write_varint_u64(&mut out, tag);
        out.extend_from_slice(args.as_bytes());
        Ok(out)
    }

    /// 🏷️ Restores the op line whose keyword the tag's record names.
    pub fn decode_line(protocol: &str, bytes: &[u8]) -> Result<String, ProtocolError> {
        let mut reader = ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let tag = reader.read_varint_u64()?;
        let kind = protocol_record::kind(protocol, tag).ok_or_else(|| ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("📡️.protocol.semio declares no record with tag {tag}") })?;
        let offset = reader.position();
        let args = std::str::from_utf8(&bytes[offset..]).map_err(|error| ProtocolError::Malformed { what: "op args", offset: offset as u64, detail: error.to_string() })?;
        Ok(if args.is_empty() { kind.to_string() } else { format!("{kind} {args}") })
    }
}
//#endregion 🏷️TaggedTextRt

//#region 🔖️Idiom
/// @emoji 🗣️ A custom front-end language layered on this engine: its own lexer/parser/printer/AST,
/// sharing only the laws (round-trip, canonicalize idempotence) and — via `register_idiom` — the
/// editor plumbing (`LanguageService` fence delegation, semantic tokens). Formalizes the technique
/// Jack (`math_graph_dsl`) already used by hand: pre-scan tokens `crate::os_dsl::lex`'s fixed alphabet
/// can't express, delegate every remaining run to `crate::os_dsl::lex`, reuse `escape_text`/`Writer`/
/// `parse_wire_text` for anything already shared. Two integration routes:
/// - **Route A — whole-surface idiom** (a document/op language in its own right, e.g. CAD's
///   Construct): the crate hand-implements `crate::os_store::ArtifactDsl`/`crate::os_spr::OpText` by lowering its
///   own `Ast` to a `#[derive(DslRecord)]` semantic model, so `ArtifactPack`/pack≡dsl hold through
///   that model without this trait needing to know about packing at all.
/// - **Route B — embedded idiom** (a `Shape::Embed(lang)` host field, e.g. a Jack query living
///   inside a `writer` document): `register_idiom` lets canonicalization normalize the embedded
///   text through the idiom's own canonical printer, so idempotence composes across the boundary.
pub trait DslIdiom {
    /// Stable registry id — the `lang` string a `#[dsl(lang = "...")]` field names.
    const LANG: &'static str;
    type Ast: Clone + PartialEq + Send + Sync;

    // 🚫️async: E4 fn-pointer slot — every method here is coerced into `IdiomHooks`'s plain `fn`
    // fields (`canonicalize`/`classify`/`complete`) in `hooks_for` below; an `fn` item's
    // pointer type is unnameable, so this whole trait must stay sync. See R2 E4.
    fn parse(text: &str) -> Result<Self::Ast, TextError>;
    /// LAW: `Self::parse(&Self::print(ast)) == Ok(ast)` for every `ast` the idiom can produce —
    /// the idiom's own round-trip law, the direct analogue of this engine's `parse ∘ print = id`
    /// for `RecordSpec` grammars.
    // 🚫️async: E4 fn-pointer slot — see `parse` above
    fn print(ast: &Self::Ast) -> String;
    // 🚫️async: E4 fn-pointer slot — see `parse` above
    fn classify(text: &str) -> Vec<(TokenClass, TextSpan)>;
    // 🚫️async: E4 fn-pointer slot — see `parse` above
    fn complete(_text: &str, _offset: usize) -> Vec<CompletionItem> {
        Vec::new()
    }
}

/// @emoji 🧩️ Placeholder until `crate::os_dsl::schema::LanguageService` grows a real completion type — kept
/// as a named type now so `DslIdiom::complete`'s signature doesn't need to change when it does.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompletionItem {
    pub label: String,
    pub detail: Option<String>,
}

/// @emoji 📇️ Type-erased vtable for one registered idiom — what `Shape::Embed` canonicalization
/// and `LanguageService` fence delegation call through, without depending on the idiom's own crate
/// (which would be a dependency cycle: the idiom depends on `dsl`, not the reverse).
#[derive(Clone, Copy)]
pub struct IdiomHooks {
    pub lang: &'static str,
    /// `print ∘ parse` — `Err` propagates the idiom's own parse diagnostic unchanged.
    pub canonicalize: fn(&str) -> Result<String, TextError>,
    pub classify: fn(&str) -> Vec<(TokenClass, TextSpan)>,
    pub complete: fn(&str, usize) -> Vec<CompletionItem>,
}

/// @emoji 🏗️ Derives an `IdiomHooks` vtable from a `DslIdiom` impl — the one place `Self::Ast`
/// needs to be named, so every other caller works with the type-erased `IdiomHooks` instead.
// 🚫️async: E4 fn-pointer slot — builds an `IdiomHooks` whose fields are plain `fn` pointers; an
// `fn`'s captured closure cannot coerce to `fn`, so this stays sync. See R2 E4.
pub fn hooks_for<I: DslIdiom>() -> IdiomHooks {
    IdiomHooks { lang: I::LANG, canonicalize: |text| I::parse(text).map(|ast| I::print(&ast)), classify: I::classify, complete: I::complete }
}

/// @emoji 🪞 Minimal hooks for binary/text facets that register a [`LanguageSpec`] without a custom
/// [`DslIdiom`] front-end — canonicalize is identity; classify/complete are empty.
// 🚫️async: E4 fn-pointer slot — see `hooks_for` above
pub fn passthrough_hooks(lang: &'static str) -> IdiomHooks {
    IdiomHooks { lang, canonicalize: |text| Ok(text.to_string()), classify: |_| Vec::new(), complete: |_, _| Vec::new() }
}

static IDIOM_REGISTRY: OnceLock<Mutex<HashMap<&'static str, IdiomHooks>>> = OnceLock::new();

// 🚫️async: E1 pure accessor consumed by the E4 `IdiomHooks` cluster — see R9
fn idiom_registry() -> &'static Mutex<HashMap<&'static str, IdiomHooks>> {
    IDIOM_REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

/// @emoji 📌️ Registers an idiom's hooks under its `LANG` id — called once at host/plugin init.
/// Re-registering the same `lang` overwrites the previous hooks rather than erroring, so a
/// hot-reloaded dev build never deadlocks on itself.
// 🚫️async: E1 pure accessor consumed by the E4 `IdiomHooks` cluster — see R9
pub fn register_idiom(hooks: IdiomHooks) {
    let mut registry = idiom_registry().lock().unwrap_or_else(|poison| poison.into_inner());
    registry.insert(hooks.lang, hooks);
}

/// @emoji 🔍️ Looks up a previously-registered idiom's hooks by `lang` id. `None` for an
/// unregistered (or not-yet-registered) lang — callers must treat that as "pass through verbatim",
/// never as an error, since `Shape::Embed` text must remain parseable before any plugin has run
/// its own registration.
// 🚫️async: E1 pure accessor consumed by the E4 `IdiomHooks` cluster — see R9
pub fn idiom(lang: &str) -> Option<IdiomHooks> {
    let registry = idiom_registry().lock().unwrap_or_else(|poison| poison.into_inner());
    registry.get(lang).copied()
}

/// @emoji 🎭️ Which surface a registered [`LanguageSpec`] describes for the
/// `handcrafted-grammar-for-every-artifact` program.
///
/// Text roles carry a `.grammar.semio` (`grammar` / `grammar_path`): `Document` (`🗣️dsl`),
/// `Config`, `Ops` (`🔧️op`), `Embedded` (`Shape::Embed` idiom), and `Diff` (`🔺️diff`).
/// Binary roles carry a `.protocol.semio` (`protocol` / `protocol_path`): `Pack` (`🎒️pack`)
/// and `Spr` (`📡️spr`). Never put grammar files on pack/spr or protocol files on dsl/op/diff.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LanguageRole {
    Document,
    Config,
    Ops,
    Embedded,
    Diff,
    Pack,
    Spr,
}

/// @emoji 📖️ One artifact facet language, registered once at plugin init: identity, the extension
/// it opens (documents/configs only), optional hand-authored **grammar** text for text surfaces
/// (`🗣️dsl` / `🔧️op` / `🔺️diff`, `dialect grammar`), optional hand-authored **protocol** text for
/// binary surfaces (`🎒️pack` / `📡️spr`, `dialect protocol`), and the [`IdiomHooks`] vtable used by
/// text hosts (`LanguageSession`, writer). Additive alongside `IdiomHooks`/`register_idiom`.
#[derive(Clone, Copy)]
pub struct LanguageSpec {
    pub id: &'static str,
    pub extension: Option<&'static str>,
    pub role: LanguageRole,
    pub grammar: Option<&'static str>,
    pub grammar_path: Option<&'static str>,
    pub protocol: Option<&'static str>,
    pub protocol_path: Option<&'static str>,
    pub hooks: IdiomHooks,
}

impl LanguageSpec {
    /// @emoji 📝 Whether this role is a text grammar surface (dsl/op/diff/config/embed).
    // 🚫️async: E1 pure accessor — trivial enum match, no suspension point — see R9
    pub fn is_text_role(self) -> bool {
        matches!(self.role, LanguageRole::Document | LanguageRole::Config | LanguageRole::Ops | LanguageRole::Embedded | LanguageRole::Diff)
    }

    /// @emoji 📡️ Whether this role is a binary protocol surface (pack/spr).
    // 🚫️async: E1 pure accessor — trivial enum match, no suspension point — see R9
    pub fn is_binary_role(self) -> bool {
        matches!(self.role, LanguageRole::Pack | LanguageRole::Spr)
    }

    /// @emoji 📖️ Parses `grammar` via [`parse_grammar`], requiring [`SemioDialect::Grammar`].
    pub fn parsed_grammar(&self) -> Result<Option<GrammarFile>, TextError> {
        let Some(text) = self.grammar else {
            return Ok(None);
        };
        let file = parse_grammar(text)?;
        if file.dialect != SemioDialect::Grammar {
            return Err(TextError::new("LanguageSpec.grammar requires dialect grammar", TextSpan::at(1, 1)));
        }
        Ok(Some(file))
    }

    /// @emoji 📡️ Parses `protocol` via [`parse_protocol`].
    pub fn parsed_protocol(&self) -> Result<Option<ProtocolFile>, TextError> {
        let Some(text) = self.protocol else {
            return Ok(None);
        };
        Ok(Some(parse_protocol(text)?))
    }

    /// @emoji ✅ Verifies encoded bytes against this language's protocol when protocol text is present.
    pub fn verify_protocol(&self, bytes: &[u8]) -> Result<(), String> {
        let Some(text) = self.protocol else {
            return Ok(());
        };
        verify_protocol_source(text, bytes)
    }
}

/// @emoji 🪪 Pass-through [`IdiomHooks`] for binary facets (pack/spr) and text facets without a
/// dedicated `DslIdiom` yet — canonicalize is identity; classify/complete are empty.

static LANGUAGE_REGISTRY: OnceLock<Mutex<HashMap<&'static str, LanguageSpec>>> = OnceLock::new();

// 🚫️async: E1 pure accessor — plain `OnceLock`/`Mutex` init, no suspension point — see R9. Its two
// PUBLIC callers that cross into `🔌️plugin/🦀️.rs` (a live ATOMIC packet's file, not mine to
// touch) stay `fn` themselves — see `preflight_languages`/`register_languages` below — so that
// external `` call shape needs no change; only this private accessor and the purely-local
// lookups (`language`/`language_for_extension`/…) revert to sync.
fn language_registry() -> &'static Mutex<HashMap<&'static str, LanguageSpec>> {
    LANGUAGE_REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

/// ⚠️ Language registration rejects a distinct owner for an established language id.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LanguageRegistryError {
    pub id: String,
}

impl std::fmt::Display for LanguageRegistryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "language registration conflicts for {}", self.id)
    }
}

impl std::error::Error for LanguageRegistryError {}

fn same_language(left: LanguageSpec, right: LanguageSpec) -> bool {
    left.id == right.id
        && left.extension == right.extension
        && left.role == right.role
        && left.grammar == right.grammar
        && left.grammar_path == right.grammar_path
        && left.protocol == right.protocol
        && left.protocol_path == right.protocol_path
        && left.hooks.lang == right.hooks.lang
        && std::ptr::fn_addr_eq(left.hooks.canonicalize, right.hooks.canonicalize)
        && std::ptr::fn_addr_eq(left.hooks.classify, right.hooks.classify)
        && std::ptr::fn_addr_eq(left.hooks.complete, right.hooks.complete)
}

/// 🔬️ Verifies language specifications against established and intra-batch identities without mutation.
/// Stays `fn` — no internal suspension point (`language_registry` is sync, R9), but
/// `🔌️plugin/🦀️.rs` (a live ATOMIC packet's file, not mine to touch) already awaits this,
/// so per R9 rule 3 the external `` call shape wins over reverting it to match a leaf helper.
#[must_use]
pub fn preflight_languages(specs: &[LanguageSpec]) -> Result<(), LanguageRegistryError> {
    let mut proposed = HashMap::new();
    for spec in specs {
        match proposed.insert(spec.id, *spec) {
            Some(existing) if same_language(existing, *spec) => {}
            Some(_) => return Err(LanguageRegistryError { id: spec.id.to_string() }),
            None => {}
        }
    }
    let registry = language_registry().lock().unwrap_or_else(|poison| poison.into_inner());
    for spec in specs {
        if let Some(existing) = registry.get(spec.id) {
            if !same_language(*existing, *spec) {
                return Err(LanguageRegistryError { id: spec.id.to_string() });
            }
        }
    }
    Ok(())
}

/// 📌️ Registers language specifications only after the whole candidate set is conflict-free.
/// Stays `fn` — see `preflight_languages` above, same external-caller reason.
#[must_use]
pub fn register_languages(specs: Vec<LanguageSpec>) -> Result<(), LanguageRegistryError> {
    preflight_languages(&specs)?;
    let mut registry = language_registry().lock().unwrap_or_else(|poison| poison.into_inner());
    for spec in specs {
        registry.entry(spec.id).or_insert(spec);
    }
    Ok(())
}

/// @emoji 📌️ Registers one grammar under its `id` — called once per grammar at plugin init,
/// alongside (not instead of) `register_document_codec_for_app`. Overwrites on re-registration,
/// matching `register_idiom`'s hot-reload-safe behavior.
// 🚫️async: E1 pure accessor — see `language_registry` above
pub fn register_language(spec: LanguageSpec) {
    let mut registry = language_registry().lock().unwrap_or_else(|poison| poison.into_inner());
    registry.insert(spec.id, spec);
}

/// @emoji 🔍️ Looks up a registered grammar by its `id` (e.g. `"fem2d"`, `"fem2dcfg"`, `"jack"`).
// 🚫️async: E1 pure accessor — see `language_registry` above
pub fn language(id: &str) -> Option<LanguageSpec> {
    let registry = language_registry().lock().unwrap_or_else(|poison| poison.into_inner());
    registry.get(id).copied()
}

/// @emoji 🔍️ Looks up a registered grammar by legacy file-extension suffix (e.g. `"note"`, `"jack"`).
// 🚫️async: E1 pure accessor — see `language_registry` above
pub fn language_for_extension(extension: &str) -> Option<LanguageSpec> {
    let suffix = extension.strip_prefix('.').unwrap_or(extension);
    let registry = language_registry().lock().unwrap_or_else(|poison| poison.into_inner());
    registry.values().find(|spec| spec.extension == Some(suffix)).copied()
}

/// @emoji 🔍️ Resolves a registered language from `.semio` file bytes (content-derived envelope).
/// Text components (`dsl`/`op`) prefer grammar registrations; binary components (`pack`/`spr`)
/// prefer protocol registrations.
// 🚫️async: E1 pure accessor — see `language_registry` above
pub fn language_for_semio_content(bytes: &[u8]) -> Option<LanguageSpec> {
    let envelope = semio_format::sniff(bytes).ok()?;
    let base = envelope.envelope_id();
    let plugin = envelope.plugin.as_str();
    let artifact = envelope.artifact.as_str();
    match envelope.component {
        semio_format::Component::Dsl => language(&base).or_else(|| language_for_extension(artifact)).or_else(|| language_for_extension(plugin)),
        semio_format::Component::Op => language_for_suffix_candidates(&base, plugin, artifact, "op").or_else(|| {
            let registry = language_registry().lock().unwrap_or_else(|poison| poison.into_inner());
            registry.values().find(|s| s.role == LanguageRole::Ops && s.extension == Some(artifact)).copied()
        }),
        semio_format::Component::Pack => language_for_suffix_candidates(&base, plugin, artifact, "pack").or_else(|| language(&base).filter(|s| s.protocol.is_some())),
        semio_format::Component::Spr => language_for_suffix_candidates(&base, plugin, artifact, "spr"),
        _ => None,
    }
}

// 🚫️async: E1 pure accessor — see `language_registry` above
fn language_for_suffix_candidates(base: &str, plugin: &str, artifact: &str, suffix: &str) -> Option<LanguageSpec> {
    language(&format!("{base}.{suffix}")).or_else(|| language(&format!("{plugin}.{suffix}"))).or_else(|| language(&format!("{artifact}.{suffix}"))).or_else(|| language(&format!("{plugin}.{artifact}.{suffix}")))
}
//#endregion 🔖️Idiom

//#region 🔖️TestSupport
/// @emoji 🧪️ Round-trip/property helpers every derived (or hand-declared) grammar's own tests
/// call — the facade-level analogue of `crate::os_store::test_support`, scoped to the engine's own laws
/// rather than the VCS store's.
pub mod test_support {
    use super::*;

    /// @emoji 🔁️ `parse(print(value)) == value` for a `RecordSpec` and an already-built `RecordValue`.
    pub fn assert_schema_round_trip(value: &RecordValue, spec: &RecordSpec) {
        let printed = print(value, spec, JoinMode::Document);
        let opts = ParseOptions::default();
        let reparsed = parse(&printed, spec, &opts).unwrap_or_else(|e| panic!("reparse failed: {e}\nprinted:\n{printed}"));
        assert_eq!(value, &reparsed, "schema round trip diverged;\nprinted:\n{printed}");
    }

    /// @emoji ♻️ `canonicalize(canonicalize(x)) == canonicalize(x)`.
    pub fn assert_idempotent(text: &str, spec: &RecordSpec) {
        let once = canonicalize(text, spec, &ParseOptions::default()).unwrap_or_else(|e| panic!("canonicalize failed: {e}"));
        let twice = canonicalize(&once, spec, &ParseOptions::default()).unwrap_or_else(|e| panic!("second canonicalize failed: {e}"));
        assert_eq!(once, twice, "canonicalization must be idempotent");
    }

    /// @emoji 📏️ Document and Inline renders of the same value must parse back to equal values,
    /// and the Inline render must be exactly one line — the newline law, checked generically.
    pub fn assert_document_inline_agree(value: &RecordValue, spec: &RecordSpec) {
        let inline_text = print(value, spec, JoinMode::Inline);
        assert!(!inline_text.contains('\n'), "inline render must be one line: {inline_text:?}");
        let inline_opts = ParseOptions { limits: Limits::default(), mode: SourceMode::Inline };
        let reparsed = parse(&inline_text, spec, &inline_opts).unwrap_or_else(|e| panic!("inline reparse failed: {e}\ninline:\n{inline_text}"));
        assert_eq!(value, &reparsed, "Document and Inline renders must parse to the same value");
    }
}
//#endregion 🔖️TestSupport

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/📦️boxed-fields/🦀️.rs"]
mod boxed_field_tests;

#[cfg(test)]
#[path = "🧪️tests/🔢️checked-integers/🦀️.rs"]
mod checked_integer_tests;

#[cfg(test)]
#[path = "🧪️tests/🏷️protocol-record/🦀️.rs"]
mod protocol_record_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
