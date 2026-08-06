//! 🧬️ `dsl` — facade for the token-native declarative DSL engine. Technologies depend on this one
//! crate (plus `vcs` for the `DocumentDsl`/`OpText` trait definitions themselves) to get the
//! derive macros, the `DslField` binding trait primitive Rust types implement, and the `__rt`
//! runtime the generated code calls into.

// The derive macros emit `::dsl::...` paths so generated code reads identically regardless of
// which technology crate invokes them. That only resolves for the crates that depend on `dsl` as
// an external crate — which is every real consumer, but NOT this crate's own tests (a crate is
// never its own dependency). `extern crate self as dsl;` is the standard fix: it makes `::dsl`
// resolve to this crate even when the derive is exercised in-crate, as the `🧪️Tests` region below does.
// Only needed for the in-crate tests, so it's cfg-gated to avoid an "unused extern crate" warning
// in ordinary (non-test) builds, where every real consumer already has `dsl` as a true dependency.
#[cfg(test)]
extern crate self as dsl;

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub use dsl_core::*;
pub use dsl_derive::{DslDiff, DslDocument, DslEnum, DslOps, DslRecord, DslScalar};
pub use dsl_schema::*;

pub use dsl_schema::{from_dsl_value, to_dsl_value};

//#region 🔖️Field
/// @emoji 🔗️ Bridges a concrete Rust field type to the engine's `Shape`/`FieldValue` — every
/// primitive implements it directly; `#[derive(DslRecord)]`/`#[derive(DslScalar)]` implement it
/// for technology-declared nested types, so composition (a record field whose type is another
/// derived record or enum) works transparently through the same trait.
pub trait DslField: Sized {
    fn shape() -> Shape;
    fn to_value(&self) -> FieldValue;
    fn from_value(value: &FieldValue) -> Result<Self, String>;
}

macro_rules! impl_dsl_field_int {
    ($ty:ty, $shape:expr, $variant:ident, $as_ty:ty) => {
        impl DslField for $ty {
            fn shape() -> Shape {
                $shape
            }
            fn to_value(&self) -> FieldValue {
                FieldValue::$variant(*self as $as_ty)
            }
            fn from_value(value: &FieldValue) -> Result<Self, String> {
                match value {
                    FieldValue::$variant(v) => Ok(*v as $ty),
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
/// (unquoted) whenever `dsl_core::is_bare_ident` holds for the value, quoted+escaped otherwise —
/// so bare-vs-quoted is entirely a printing decision now, not a separate shape a field opts into.
impl DslField for String {
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
/// `WIRE`-typed column) — thin `DslField` wrapper around `dsl_schema::WireValue` so adopter
/// technologies never need to hand-roll their own `Shape::Wire` binding.
#[derive(Clone, Debug, PartialEq)]
pub struct Wire(pub WireValue);

impl DslField for Wire {
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
    fn shape() -> Shape {
        Shape::List(Box::new(T::shape()))
    }
    fn to_value(&self) -> FieldValue {
        FieldValue::List(self.iter().map(DslField::to_value).collect())
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::List(items) => items.iter().map(T::from_value).collect(),
            other => Err(format!("expected List, found {other:?}")),
        }
    }
}

/// @emoji 🗺️ Same recursion seam as `Vec<T>`, for a `BTreeMap<String, T>` that's itself nested
/// (e.g. `Option<BTreeMap<String, T>>`) rather than a bare top-level field — `#[derive(DslRecord)]`
/// classifies a *bare* `BTreeMap<String, T>` field directly via its own dedicated `FieldKind`
/// (same `Shape::Map` this produces), so the two never conflict.
impl<T: DslField> DslField for std::collections::BTreeMap<String, T> {
    fn shape() -> Shape {
        Shape::Map(Box::new(T::shape()))
    }
    fn to_value(&self) -> FieldValue {
        FieldValue::Map(self.iter().map(|(k, v)| (k.clone(), v.to_value())).collect())
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Map(entries) => entries.iter().map(|(k, v)| Ok((k.clone(), T::from_value(v)?))).collect(),
            other => Err(format!("expected Map, found {other:?}")),
        }
    }
}

/// @emoji 📐️ Fixed-arity `Shape::Tuple(_, Some(N))` — a packed `x,y,z`-style literal for any `N`.
impl<T: DslField, const N: usize> DslField for [T; N] {
    fn shape() -> Shape {
        Shape::Tuple(Box::new(T::shape()), Some(N))
    }
    fn to_value(&self) -> FieldValue {
        FieldValue::Tuple(self.iter().map(DslField::to_value).collect())
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Tuple(items) if items.len() == N => {
                let converted: Vec<T> = items.iter().map(T::from_value).collect::<Result<_, _>>()?;
                converted.try_into().map_err(|_| format!("expected {N} items, got a length mismatch"))
            }
            other => Err(format!("expected a {N}-item Tuple, found {other:?}")),
        }
    }
}

/// @emoji 🌱️ Schema-less dynamic literal — binds as `Shape::Value`.
impl DslField for DslValue {
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
    fn variants() -> Vec<(String, fn() -> RecordSpec)>;
    fn to_named_record(&self) -> (String, RecordValue);
    /// @emoji ⚠️ Returns `TextError` (not `String`, unlike [`DslField::from_value`]) so
    /// generated bodies can `?`-propagate it directly — this is the same error type
    /// `protocol::OpText::parse_op`/`store::DocumentDsl::parse_dsl` already return, and the derive's
    /// `#[dsl(statements)]` field codegen composes it without any conversion at every nesting depth.
    fn from_named_record(keyword: &str, record: &RecordValue) -> Result<Self, TextError>;
}
//#endregion 🔖️Variants

//#region 🔖️Runtime
/// @emoji ⚙️ Thin wrappers the derive-generated `impl store::DocumentDsl`/`impl protocol::OpText`
/// bodies call into — kept as free functions (not methods) so generated code never has to name
/// this crate's internal types, only `dsl::__rt::*`.
pub mod __rt {
    use super::*;

    pub fn parse_document_record(text: &str, spec: &RecordSpec) -> Result<RecordValue, TextError> {
        parse(text, spec, &ParseOptions { limits: Limits::default(), mode: SourceMode::Document })
    }

    pub fn print_document_record(value: &RecordValue, spec: &RecordSpec) -> String {
        print(value, spec, JoinMode::Document)
    }

    pub fn parse_inline_record(text: &str, spec: &RecordSpec) -> Result<RecordValue, TextError> {
        parse(text, spec, &ParseOptions { limits: Limits::default(), mode: SourceMode::Inline })
    }

    pub fn print_inline_record(value: &RecordValue, spec: &RecordSpec) -> String {
        print(value, spec, JoinMode::Inline)
    }

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
    /// `#[derive(DslDocument)]`) — anything else is a derive-time misuse, hence the panic rather than
    /// a `Result` (there is no sensible recoverable path for a grammar that's wrong at compile time).
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

//#region 🔖️OpRt
/// @emoji 🎯️ Runtime behind `protocol::OpBinary`, resolved as `::dsl::op_rt::...` by
/// `#[derive(DslOps)]`'s emitted impl — the binary twin of `__rt`'s inline text path and the
/// op-level mirror of the `DocumentDsl`/`DocumentPack` pairing. An operation enum lowers to
/// `(variant keyword, RecordSpec, RecordValue)` via [`DslVariants`]; this module fixes the byte
/// layout `format u8 (=1) | variant ordinal varint | record body` where the ordinal indexes
/// `DslVariants::variants()` declaration order (reordering variants is a format break) and the
/// body is `pack::encode_record_body`'s container-less encoding. Lives here rather than in
/// `store` because the bound is this crate's own `DslVariants` — a `store`-hosted twin would be a
/// distinct trait instance inside this crate's own test build (dev-dependency cycle). LAW:
/// `decode_op(&encode_op(op)) == op == parse_op(&print_op(op))`, and encoding is deterministic.
pub mod op_rt {
    use super::DslVariants;
    use protocol::ProtocolError;

    /// @emoji 🎯️ Format byte every encoded operation starts with.
    pub const OP_BINARY_FORMAT: u8 = 1;

    /// @emoji 🎯️ Encodes one operation deterministically (byte-identical for equal ops).
    pub fn encode_op<T: DslVariants>(op: &T) -> Result<Vec<u8>, ProtocolError> {
        let (keyword, record) = op.to_named_record();
        let variants = T::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1)();
        let body = pack::encode_record_body(&spec, &record, &pack::EncodeOptions::default())?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        pack::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }

    /// @emoji 🎯️ Inverse of [`encode_op`].
    pub fn decode_op<T: DslVariants>(bytes: &[u8]) -> Result<T, ProtocolError> {
        let mut reader = pack::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = T::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = pack::decode_record_body(body, &spec, &pack::DecodeOptions::default())?;
        T::from_named_record(keyword, &record).map_err(|error| ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })
    }
}
//#endregion 🔖️OpRt

//#region 🔖️Idiom
/// @emoji 🗣️ A custom front-end language layered on this engine: its own lexer/parser/printer/AST,
/// sharing only the laws (round-trip, canonicalize idempotence) and — via `register_idiom` — the
/// editor plumbing (`LanguageService` fence delegation, semantic tokens). Formalizes the technique
/// Jack (`math_graph_dsl`) already used by hand: pre-scan tokens `dsl_core::lex`'s fixed alphabet
/// can't express, delegate every remaining run to `dsl_core::lex`, reuse `escape_text`/`Writer`/
/// `parse_wire_text` for anything already shared. Two integration routes:
/// - **Route A — whole-surface idiom** (a document/op language in its own right, e.g. CAD's
///   Construct): the crate hand-implements `store::DocumentDsl`/`protocol::OpText` by lowering its
///   own `Ast` to a `#[derive(DslRecord)]` semantic model, so `DocumentPack`/pack≡dsl hold through
///   that model without this trait needing to know about packing at all.
/// - **Route B — embedded idiom** (a `Shape::Embed(lang)` host field, e.g. a Jack query living
///   inside a `writer` document): `register_idiom` lets canonicalization normalize the embedded
///   text through the idiom's own canonical printer, so idempotence composes across the boundary.
pub trait DslIdiom {
    /// Stable registry id — the `lang` string a `#[dsl(lang = "...")]` field names.
    const LANG: &'static str;
    type Ast: Clone + PartialEq + Send + Sync;

    fn parse(text: &str) -> Result<Self::Ast, TextError>;
    /// LAW: `Self::parse(&Self::print(ast)) == Ok(ast)` for every `ast` the idiom can produce —
    /// the idiom's own round-trip law, the direct analogue of this engine's `parse ∘ print = id`
    /// for `RecordSpec` grammars.
    fn print(ast: &Self::Ast) -> String;
    fn classify(text: &str) -> Vec<(TokenClass, TextSpan)>;
    fn complete(_text: &str, _offset: usize) -> Vec<CompletionItem> {
        Vec::new()
    }
}

/// @emoji 🧩️ Placeholder until `dsl_schema::LanguageService` grows a real completion type — kept
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
pub fn hooks_for<I: DslIdiom>() -> IdiomHooks {
    IdiomHooks { lang: I::LANG, canonicalize: |text| I::parse(text).map(|ast| I::print(&ast)), classify: I::classify, complete: I::complete }
}

static IDIOM_REGISTRY: OnceLock<Mutex<HashMap<&'static str, IdiomHooks>>> = OnceLock::new();

fn idiom_registry() -> &'static Mutex<HashMap<&'static str, IdiomHooks>> {
    IDIOM_REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

/// @emoji 📌️ Registers an idiom's hooks under its `LANG` id — called once at host/plugin init.
/// Re-registering the same `lang` overwrites the previous hooks rather than erroring, so a
/// hot-reloaded dev build never deadlocks on itself.
pub fn register_idiom(hooks: IdiomHooks) {
    let mut registry = idiom_registry().lock().unwrap_or_else(|poison| poison.into_inner());
    registry.insert(hooks.lang, hooks);
}

/// @emoji 🔍️ Looks up a previously-registered idiom's hooks by `lang` id. `None` for an
/// unregistered (or not-yet-registered) lang — callers must treat that as "pass through verbatim",
/// never as an error, since `Shape::Embed` text must remain parseable before any plugin has run
/// its own registration.
pub fn idiom(lang: &str) -> Option<IdiomHooks> {
    let registry = idiom_registry().lock().unwrap_or_else(|poison| poison.into_inner());
    registry.get(lang).copied()
}

/// @emoji 🎭️ Which of an app's four grammars (per the `handcrafted-grammar-for-every-artifact`
/// program) a registered [`LanguageSpec`] describes. `Embedded` is `DslIdiom`'s existing Route B
/// (a `Shape::Embed(lang)` host field), kept as its own variant rather than folded into `Document`
/// since an embedded language has no `extension`/`DocumentCodec` of its own. `Diff` is the W1
/// addition (design ruling B-R4): a registered `#[derive(dsl::DslDiff)]` diff type, conventionally
/// registered under the id `"<doc-schema>#diff"` — additive alongside the other three roles, no
/// existing registration is affected by this variant's addition.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LanguageRole {
    Document,
    Config,
    Ops,
    Embedded,
    Diff,
}

/// @emoji 📖️ One app's grammar, registered once at plugin/idiom init: identity, the extension it
/// opens (documents/configs only — an `.ops` grammar and an embedded idiom have none), the
/// hand-authored `.grammar` spec text (once one exists for this language — most don't yet, hence
/// `Option`, not a hard requirement, so registering ahead of writing the spec file isn't blocked),
/// and the same [`IdiomHooks`] vtable `DslIdiom`-based registration already produces. This is
/// deliberately additive alongside `IdiomHooks`/`register_idiom`/`idiom`, not a replacement: no
/// existing registration (Jack's, or the `#[cfg(test)]` ones in this file) is touched by adding it.
#[derive(Clone, Copy)]
pub struct LanguageSpec {
    pub id: &'static str,
    pub extension: Option<&'static str>,
    pub role: LanguageRole,
    pub grammar: Option<&'static str>,
    pub grammar_path: Option<&'static str>,
    pub hooks: IdiomHooks,
}

impl LanguageSpec {
    /// @emoji 🧬️ Services a facet still on generic `RecordSpec`/`DocumentDsl` until its handcrafted
    /// `.semio` spec lands — same hooks as the parent document grammar, distinct registry id.
    pub fn derived(parent: LanguageSpec, id: &'static str, role: LanguageRole) -> Self {
        Self { id, extension: None, role, grammar: parent.grammar, grammar_path: parent.grammar_path, hooks: parent.hooks }
    }
}

static LANGUAGE_REGISTRY: OnceLock<Mutex<HashMap<&'static str, LanguageSpec>>> = OnceLock::new();

fn language_registry() -> &'static Mutex<HashMap<&'static str, LanguageSpec>> {
    LANGUAGE_REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

/// @emoji 📌️ Registers one grammar under its `id` — called once per grammar at plugin init,
/// alongside (not instead of) `register_document_codec_for_app`. Overwrites on re-registration,
/// matching `register_idiom`'s hot-reload-safe behavior.
pub fn register_language(spec: LanguageSpec) {
    let mut registry = language_registry().lock().unwrap_or_else(|poison| poison.into_inner());
    registry.insert(spec.id, spec);
}

/// @emoji 🔍️ Looks up a registered grammar by its `id` (e.g. `"fem2d"`, `"fem2dcfg"`, `"jack"`).
pub fn language(id: &str) -> Option<LanguageSpec> {
    let registry = language_registry().lock().unwrap_or_else(|poison| poison.into_inner());
    registry.get(id).copied()
}

/// @emoji 🔍️ Resolves a registered DSL grammar from `.semio` file bytes (content-derived envelope).
pub fn language_for_semio_content(bytes: &[u8]) -> Option<LanguageSpec> {
    let envelope = semio_format::sniff(bytes).ok()?;
    if envelope.component != semio_format::Component::Dsl {
        return None;
    }
    language(&envelope.envelope_id())
}
//#endregion 🔖️Idiom

//#region 🔖️TestSupport
/// @emoji 🧪️ Round-trip/property helpers every derived (or hand-declared) grammar's own tests
/// call — the facade-level analogue of `store::test_support`, scoped to the engine's own laws
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
mod tests {
    use super::*;

    #[test]
    fn primitive_dsl_field_impls_round_trip() {
        assert_eq!(i32::from_value(&42i32.to_value()), Ok(42));
        assert_eq!(u64::from_value(&7u64.to_value()), Ok(7));
        assert_eq!(bool::from_value(&true.to_value()), Ok(true));
        assert_eq!(f64::from_value(&1.5f64.to_value()), Ok(1.5));
        assert_eq!(String::from_value(&"hi".to_string().to_value()), Ok("hi".to_string()));
    }

    #[test]
    fn wire_field_dsl_field_impl_round_trips() {
        let literal = parse_wire_text("a:Kind@out->b:Kind2@in").expect("parse_wire_text");
        assert!(matches!(Wire::shape(), Shape::Wire));
        let wire = Wire(literal.clone());
        let value = wire.to_value();
        assert_eq!(value, FieldValue::Wire(literal));
        let restored = Wire::from_value(&value).expect("from_value");
        assert_eq!(restored, wire);
    }

    // --- DslIdiom: a toy "hello <name>" language exercising the whole trait + registry seam ---
    #[derive(Clone, Debug, PartialEq)]
    struct GreetAst {
        name: String,
    }

    struct GreetIdiom;

    impl DslIdiom for GreetIdiom {
        const LANG: &'static str = "greet";
        type Ast = GreetAst;

        fn parse(text: &str) -> Result<Self::Ast, TextError> {
            text.strip_prefix("hello ").map(|name| GreetAst { name: name.trim().to_string() }).ok_or_else(|| TextError::new("expected 'hello <name>'", TextSpan::at(1, 1)))
        }

        fn print(ast: &Self::Ast) -> String {
            format!("hello {}", ast.name)
        }

        fn classify(_text: &str) -> Vec<(TokenClass, TextSpan)> {
            Vec::new()
        }
    }

    #[test]
    fn dsl_idiom_round_trips_through_its_own_parse_and_print() {
        let ast = GreetIdiom::parse("hello world").expect("parse");
        assert_eq!(ast, GreetAst { name: "world".to_string() });
        assert_eq!(GreetIdiom::print(&ast), "hello world");
        assert_eq!(GreetIdiom::parse(&GreetIdiom::print(&ast)), Ok(ast), "idiom round trip law");
    }

    #[test]
    fn dsl_idiom_registry_resolves_by_lang_and_canonicalizes_through_the_hooks() {
        register_idiom(hooks_for::<GreetIdiom>());
        let hooks = idiom("greet").expect("registered idiom must be found by its LANG id");
        assert_eq!(hooks.lang, "greet");
        let canonical = (hooks.canonicalize)("hello   world").expect("canonicalize");
        assert_eq!(canonical, "hello world", "canonicalize normalizes through parse -> print");
        assert!((hooks.canonicalize)("not a greeting").is_err(), "a malformed idiom body must surface the idiom's own parse error");
        assert!(idiom("never-registered-lang").is_none(), "an unregistered lang must resolve to None, never a default/error");
    }

    #[test]
    fn language_registry_resolves_by_id_and_semio_content() {
        register_language(LanguageSpec {
            id: "greet-doc",
            extension: Some("greet"),
            role: LanguageRole::Document,
            grammar: None,
            grammar_path: None,
            hooks: hooks_for::<GreetIdiom>(),
        });
        let by_id = language("greet-doc").expect("registered language must be found by its id");
        assert_eq!(by_id.extension, Some("greet"));
        assert_eq!(by_id.role, LanguageRole::Document);
        let bytes = b"semio greet-doc.dsl v1\nhello world\n";
        let by_content = language_for_semio_content(bytes).expect("registered language must be found by sniffed envelope");
        assert_eq!(by_content.id, "greet-doc");
        assert!(language("never-registered-id").is_none());
        assert!(language_for_semio_content(b"semio missing.dsl v1\n").is_none());
    }

    #[test]
    fn dsl_value_dsl_field_round_trips_through_record_value() {
        let value = DslValue::object([("a".into(), DslValue::Number(1.0)), ("b".into(), DslValue::Array(vec![DslValue::Bool(true), DslValue::Null, DslValue::String("x".into())]))]);
        assert_eq!(DslValue::from_value(&value.to_value()), Ok(value));

        let map = DslValue::object([("curves".into(), DslValue::Array(vec![DslValue::Array(vec![DslValue::Number(0.0), DslValue::Number(0.0)]), DslValue::Array(vec![DslValue::Number(1.0), DslValue::Number(1.0)])]))]);
        assert_eq!(DslValue::from_value(&map.to_value()), Ok(map));
    }

    // --- end-to-end derive tests: mirrors the norm-family "flat scalar document" worked example ---

    #[derive(Clone, Debug, PartialEq, DslScalar, serde::Serialize, serde::Deserialize)]
    enum ClimateZone {
        Cold,
        Temperate,
        Warm,
    }

    #[derive(Clone, Debug, PartialEq, DslDocument, serde::Serialize, serde::Deserialize)]
    #[dsl(extension = "derivedoc")]
    struct DerivedDocument {
        category: String,
        climate: ClimateZone,
        airtightness_n50: f64,
        occupants: u32,
        note: Option<String>,
    }

    #[test]
    fn derived_document_round_trips_through_vcs_document_dsl() {
        let doc = DerivedDocument { category: "external_wall".to_string(), climate: ClimateZone::Cold, airtightness_n50: 0.6, occupants: 4, note: None };
        let printed = <DerivedDocument as store::DocumentDsl>::print_dsl(&doc);
        assert!(!printed.contains("note"), "absent optional field must be omitted: {printed}");
        let parsed = <DerivedDocument as store::DocumentDsl>::parse_dsl(&printed).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed}"));
        assert_eq!(parsed, doc, "derived DocumentDsl round trip diverged;\nprinted:\n{printed}");
    }

    #[test]
    fn derived_document_round_trips_with_optional_field_present() {
        let doc = DerivedDocument { category: "roof".to_string(), climate: ClimateZone::Warm, airtightness_n50: 1.2, occupants: 2, note: Some("re-inspect in 2027".to_string()) };
        let printed = <DerivedDocument as store::DocumentDsl>::print_dsl(&doc);
        let parsed = <DerivedDocument as store::DocumentDsl>::parse_dsl(&printed).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed}"));
        assert_eq!(parsed, doc);
    }

    // --- end-to-end derive test: an Operation enum via #[derive(DslOps)] ---

    #[derive(Clone, Debug, PartialEq, DslOps, serde::Serialize, serde::Deserialize)]
    enum DerivedOperation {
        #[dsl(key = "setCategory")]
        SetCategory { category: String },
        #[dsl(key = "setAirtightness")]
        SetAirtightness { n50: f64 },
        #[dsl(key = "reset")]
        Reset,
    }

    #[test]
    fn derived_op_text_round_trips_every_variant_as_one_line() {
        let ops = vec![DerivedOperation::SetCategory { category: "roof".to_string() }, DerivedOperation::SetAirtightness { n50: 0.9 }, DerivedOperation::Reset];
        for op in ops {
            let printed = <DerivedOperation as protocol::OpText>::print_op(&op);
            assert!(!printed.contains('\n'), "print_op must be one line: {printed:?}");
            let parsed = <DerivedOperation as protocol::OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed for {printed:?}: {e}"));
            assert_eq!(parsed, op, "OpText round trip diverged for {printed:?}");
        }
    }

    #[test]
    fn derived_document_dsl_satisfies_vcs_test_support_helpers() {
        let doc = DerivedDocument { category: "floor".to_string(), climate: ClimateZone::Temperate, airtightness_n50: 0.4, occupants: 3, note: None };
        store::test_support::assert_dsl_round_trip(&doc);
        store::test_support::assert_dsl_pack_equivalence(&doc);
    }

    #[test]
    fn derived_op_satisfies_vcs_test_support_helpers() {
        store::test_support::assert_op_line_round_trip(&DerivedOperation::SetCategory { category: "wall".to_string() });
    }

    #[test]
    fn derived_op_binary_round_trips_every_variant_and_matches_text() {
        let ops = vec![DerivedOperation::SetCategory { category: "roof".to_string() }, DerivedOperation::SetAirtightness { n50: 0.9 }, DerivedOperation::Reset];
        for op in ops {
            store::test_support::assert_op_text_binary_equivalence(&op);
        }
    }

    // --- end-to-end derive test: `#[derive(DslEnum)]` recursive block tree (the `note`/`draw`
    // pilots' hard case), `Vec<Vec<T>>`, `[T; N]`, and `BTreeMap<String, V>` fields ---

    #[derive(Clone, Debug, PartialEq, DslEnum, serde::Serialize, serde::Deserialize)]
    enum SceneNode {
        #[dsl(key = "point")]
        Point { pos: [f64; 3] },
        #[dsl(key = "grid")]
        Grid { rows: Vec<Vec<i32>> },
        #[dsl(key = "group")]
        Group {
            #[dsl(positional)]
            id: String,
            #[dsl(statements, block)]
            children: Vec<SceneNode>,
        },
    }

    #[derive(Clone, Debug, PartialEq, DslRecord, serde::Serialize, serde::Deserialize)]
    #[dsl(keyword = "camera")]
    struct SceneCamera {
        x: f64,
        y: f64,
    }

    #[derive(Clone, Debug, PartialEq, DslDocument, serde::Serialize, serde::Deserialize)]
    #[dsl(extension = "scenedoc")]
    struct SceneDocument {
        // `#[dsl(block)]` alone (no `statements`) wraps a plain nested-record scalar field so it
        // prints as a bare `camera { x=.. y=.. }` line instead of a `camera=...` attribute.
        #[dsl(block)]
        camera: SceneCamera,
        #[dsl(statements, block)]
        nodes: Vec<SceneNode>,
        tags: std::collections::BTreeMap<String, String>,
    }

    #[test]
    fn derived_enum_recursive_block_tree_and_map_and_nested_collections_round_trip() {
        let doc = SceneDocument {
            camera: SceneCamera { x: 1.0, y: 2.0 },
            nodes: vec![SceneNode::Point { pos: [1.0, 2.0, 3.0] }, SceneNode::Grid { rows: vec![vec![1, 2], vec![3, 4, 5]] }, SceneNode::Group { id: "g1".to_string(), children: vec![SceneNode::Point { pos: [0.0, 0.0, 0.0] }] }],
            tags: std::collections::BTreeMap::from([("author".to_string(), "semio".to_string()), ("version".to_string(), "1".to_string())]),
        };
        let printed = <SceneDocument as store::DocumentDsl>::print_dsl(&doc);
        let parsed = <SceneDocument as store::DocumentDsl>::parse_dsl(&printed).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed}"));
        assert_eq!(parsed, doc, "recursive/nested-collection round trip diverged;\nprinted:\n{printed}");
        store::test_support::assert_dsl_pack_equivalence(&doc);
    }

    // --- end-to-end derive test: single-field tuple ("newtype") variants (the `draw` pilot's
    // `LayerNode::Shape(ShapeBody)` shape) delegate entirely to the inner type's own spec/keyword ---

    #[derive(Clone, Debug, PartialEq, DslRecord, serde::Serialize, serde::Deserialize)]
    #[dsl(keyword = "circle")]
    struct CircleBody {
        #[dsl(positional)]
        id: String,
        r: f64,
    }

    #[derive(Clone, Debug, PartialEq, DslRecord, serde::Serialize, serde::Deserialize)]
    #[dsl(keyword = "square")]
    struct SquareBody {
        #[dsl(positional)]
        id: String,
        side: f64,
    }

    #[derive(Clone, Debug, PartialEq, DslEnum, serde::Serialize, serde::Deserialize)]
    enum ShapeNode {
        #[dsl(key = "circle")]
        Circle(CircleBody),
        #[dsl(key = "square")]
        Square(SquareBody),
    }

    #[derive(Clone, Debug, PartialEq, DslDocument, serde::Serialize, serde::Deserialize)]
    #[dsl(extension = "shapedoc")]
    struct ShapeDocument {
        #[dsl(statements, block)]
        shapes: Vec<ShapeNode>,
    }

    #[test]
    fn derived_newtype_tuple_variants_round_trip() {
        let doc = ShapeDocument { shapes: vec![ShapeNode::Circle(CircleBody { id: "c1".to_string(), r: 2.0 }), ShapeNode::Square(SquareBody { id: "s1".to_string(), side: 3.0 })] };
        let printed = <ShapeDocument as store::DocumentDsl>::print_dsl(&doc);
        // `"c1"` is bare-ident-shaped, so the unified "strings bare-preferred" law prints it
        // unquoted (`circle c1 r=2`, not `circle "c1" r=2`) — see `dsl_core::is_bare_ident`.
        assert!(printed.contains("circle c1 r=2"), "newtype variant must print via its own inner keyword/fields: {printed}");
        let parsed = <ShapeDocument as store::DocumentDsl>::parse_dsl(&printed).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed}"));
        assert_eq!(parsed, doc, "newtype tuple-variant round trip diverged;\nprinted:\n{printed}");
        store::test_support::assert_dsl_pack_equivalence(&doc);
    }

    // --- end-to-end derive test: `#[dsl(statements, block)] Option<T>` (the `draw` pilot's
    // `attributes.fill: Option<FillStyle>` shape) — a sum-type scalar field, not a collection ---

    #[derive(Clone, Debug, PartialEq, DslEnum, serde::Serialize, serde::Deserialize)]
    enum PaintStyle {
        #[dsl(key = "solid")]
        Solid { color: [f64; 4] },
        #[dsl(key = "gradient")]
        Gradient { stops: Vec<f64> },
    }

    #[derive(Clone, Debug, PartialEq, DslRecord, serde::Serialize, serde::Deserialize)]
    struct PaintAttributes {
        #[dsl(statements, block)]
        fill: Option<PaintStyle>,
    }

    #[derive(Clone, Debug, PartialEq, DslDocument, serde::Serialize, serde::Deserialize)]
    #[dsl(extension = "paintdoc")]
    struct PaintDocument {
        #[dsl(block)]
        attributes: PaintAttributes,
    }

    #[test]
    fn derived_option_statements_field_round_trips_present_and_absent() {
        let with_fill = PaintDocument { attributes: PaintAttributes { fill: Some(PaintStyle::Solid { color: [1.0, 0.0, 0.0, 1.0] }) } };
        let printed = <PaintDocument as store::DocumentDsl>::print_dsl(&with_fill);
        let parsed = <PaintDocument as store::DocumentDsl>::parse_dsl(&printed).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed}"));
        assert_eq!(parsed, with_fill, "Some(..) round trip diverged;\nprinted:\n{printed}");
        store::test_support::assert_dsl_pack_equivalence(&with_fill);

        let no_fill = PaintDocument { attributes: PaintAttributes { fill: None } };
        let printed_none = <PaintDocument as store::DocumentDsl>::print_dsl(&no_fill);
        let parsed_none = <PaintDocument as store::DocumentDsl>::parse_dsl(&printed_none).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed_none}"));
        assert_eq!(parsed_none, no_fill, "None round trip diverged;\nprinted:\n{printed_none}");
        store::test_support::assert_dsl_pack_equivalence(&no_fill);
    }

    // --- regression: `#[dsl(block)] Option<PlainRecord>` (the `draw` pilot's `attributes.stroke:
    // Option<StrokeStyle>` shape) — `None` must OMIT the field, not print empty `{ }` braces, since
    // reparsing empty braces would otherwise try to build a record whose required fields are absent ---

    #[derive(Clone, Debug, PartialEq, DslRecord, serde::Serialize, serde::Deserialize)]
    struct BrushStyle {
        color: [f64; 4],
        width: f64,
    }

    #[derive(Clone, Debug, PartialEq, DslDocument, serde::Serialize, serde::Deserialize)]
    #[dsl(extension = "brushdoc")]
    struct BrushDocument {
        #[dsl(block)]
        brush: Option<BrushStyle>,
    }

    #[test]
    fn derived_option_block_record_field_omits_rather_than_printing_empty_braces_when_absent() {
        let with_brush = BrushDocument { brush: Some(BrushStyle { color: [0.0, 0.0, 0.0, 1.0], width: 2.5 }) };
        let printed = <BrushDocument as store::DocumentDsl>::print_dsl(&with_brush);
        let parsed = <BrushDocument as store::DocumentDsl>::parse_dsl(&printed).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed}"));
        assert_eq!(parsed, with_brush, "Some(..) round trip diverged;\nprinted:\n{printed}");
        store::test_support::assert_dsl_pack_equivalence(&with_brush);

        let no_brush = BrushDocument { brush: None };
        let printed_none = <BrushDocument as store::DocumentDsl>::print_dsl(&no_brush);
        assert!(!printed_none.contains("brush"), "an absent block-wrapped Option<Record> must be omitted entirely, not printed as empty braces: {printed_none:?}");
        let parsed_none = <BrushDocument as store::DocumentDsl>::parse_dsl(&printed_none).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed_none}"));
        assert_eq!(parsed_none, no_brush, "None round trip diverged;\nprinted:\n{printed_none:?}");
        store::test_support::assert_dsl_pack_equivalence(&no_brush);
    }

    // --- end-to-end derive test: `#[dsl(statements)] Box<T>` (the `draw` pilot's
    // `AddLayer { layer: Box<DrawLayerNode> }` shape) — exactly one required tagged value ---

    #[derive(Clone, Debug, PartialEq, DslOps, serde::Serialize, serde::Deserialize)]
    enum PaintOp {
        #[dsl(key = "addShape")]
        AddShape {
            #[dsl(statements)]
            shape: Box<ShapeNode>,
        },
    }

    #[test]
    fn derived_required_statements_boxed_field_round_trips() {
        let op = PaintOp::AddShape { shape: Box::new(ShapeNode::Circle(CircleBody { id: "c1".to_string(), r: 2.0 })) };
        let printed = <PaintOp as protocol::OpText>::print_op(&op);
        assert!(!printed.contains('\n'), "print_op must be one line: {printed:?}");
        let parsed = <PaintOp as protocol::OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed for {printed:?}: {e}"));
        assert_eq!(parsed, op, "boxed required-statements round trip diverged for {printed:?}");
    }

    // --- regression: a genuinely self-referential `#[derive(DslRecord)]` STRUCT (not an enum) —
    // a field whose type recurses back to the struct itself, e.g. a dynamic-value type with a
    // nested-dictionary-of-itself field (the `imperative` pilot's `ValueDsl` shape). Unlike
    // `Shape::Statements` (already lazy), `Shape::Record` used to eagerly call `Self::__dsl_spec()`
    // to build its own shape, which itself built its "dict" field's shape by calling
    // `Self::__dsl_spec()` again — infinite recursion just constructing the spec, stack overflow
    // before a single byte of real data was ever touched. Now lazy (a `fn() -> RecordSpec` pointer,
    // mirroring `Statements`), so this must round trip a genuinely nested value correctly.

    #[derive(Clone, Debug, PartialEq, DslRecord, serde::Serialize, serde::Deserialize)]
    struct SelfRefValue {
        #[dsl(key = "n")]
        number: Option<i64>,
        #[dsl(key = "dict")]
        dictionary: Option<std::collections::BTreeMap<String, SelfRefValue>>,
    }

    #[derive(Clone, Debug, PartialEq, DslDocument, serde::Serialize, serde::Deserialize)]
    #[dsl(extension = "selfrefdoc")]
    struct SelfRefDocument {
        #[dsl(block)]
        root: SelfRefValue,
    }

    #[test]
    fn derived_self_referential_record_struct_round_trips_nested_values() {
        let doc = SelfRefDocument {
            root: SelfRefValue {
                number: None,
                dictionary: Some(std::collections::BTreeMap::from([("a".to_string(), SelfRefValue { number: Some(1), dictionary: Some(std::collections::BTreeMap::from([("b".to_string(), SelfRefValue { number: Some(2), dictionary: None })])) })])),
            },
        };
        let printed = <SelfRefDocument as store::DocumentDsl>::print_dsl(&doc);
        let parsed = <SelfRefDocument as store::DocumentDsl>::parse_dsl(&printed).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed}"));
        assert_eq!(parsed, doc, "self-referential record round trip diverged;\nprinted:\n{printed}");
        store::test_support::assert_dsl_pack_equivalence(&doc);
    }

    // --- end-to-end derive test: `#[dsl(table)] Vec<T>` (Structure-of-Arrays columnar field) ---

    #[derive(Clone, Debug, PartialEq, DslRecord, serde::Serialize, serde::Deserialize)]
    struct TableNodeRow {
        id: String,
        x: f64,
        y: f64,
    }

    #[derive(Clone, Debug, PartialEq, DslDocument, serde::Serialize, serde::Deserialize)]
    #[dsl(extension = "tabledoc")]
    struct TableDocument {
        #[dsl(table)]
        nodes: Vec<TableNodeRow>,
    }

    #[test]
    fn derived_table_field_prints_compact_soa_and_round_trips() {
        let doc = TableDocument { nodes: vec![TableNodeRow { id: "a".to_string(), x: 1.0, y: 2.0 }, TableNodeRow { id: "b".to_string(), x: 3.0, y: 4.0 }] };
        let printed = <TableDocument as store::DocumentDsl>::print_dsl(&doc);
        assert!(printed.contains("nodes [id:TEXT x:NUM y:NUM]"), "#[dsl(table)] field must print compact SoA: {printed}");
        let parsed = <TableDocument as store::DocumentDsl>::parse_dsl(&printed).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed}"));
        assert_eq!(parsed, doc, "table round trip diverged;\nprinted:\n{printed}");
        store::test_support::assert_dsl_pack_equivalence(&doc);
    }
}
//#endregion 🧪️Tests
