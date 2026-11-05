//! 🧳️ Canonical recursive Record values, shape declarations and their native ownership ports.
pub use semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind};
pub use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
#[path="../../🏭️producer/🦀️.rs"]
pub mod producer;
pub use producer::{NativeSchemaControl,RecordSpecProducer};
#[path="../../♻️retirement/🦀️.rs"]
pub mod retirement;
#[path="../🫳️borrowed/🦀️.rs"]
mod borrowed_schema;
pub use borrowed_schema::{BorrowedShape,BorrowedFieldSpec,BorrowedRecordSpec};
#[path="🛬️source.rs"]
mod native_source;

/// 📦️ Canonical record octets retain separately admitted backing pages.
pub type RecordBytes=semio_framework_value::paged::PagedBytes<{isize::MAX as usize}>;

//#region 🔖️Shape
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecordLayout {
    /// 📎️ All fields printed as space-joined `key=value` tokens on one logical unit.
    Inline,
    /// 📏️ Each field printed as its own line/statement (norm-family style) in Document mode;
    /// collapses to the same space-joined form as `Inline` when rendered in `JoinMode::Inline`.
    Lines,
    /// 📨️ `<name> = <keyword>(arg1=val1 arg2=val2)` — a named call assignment (the graph-family
    /// construction-chain notation, e.g. `extrude = brep.solid.extrude(profile=w1 axis=v1)`).
    /// Requires exactly one field marked [`FieldSpec::call_name`] (printed before `=`, must have
    /// `Shape::Text`) and `RecordSpec.keyword` set (the dotted call target after `=`, e.g.
    /// `"brep.solid.extrude"` — printed and matched as one token since `.` is `dsl_core`
    /// ident-continue). Every other field prints/parses exactly as it would under `Inline`
    /// (positional bare, keyed as `key=value`), just inside the parens instead of bare after the
    /// keyword.
    Call,
}

/// 🧩️ What one field's value looks like, textually. Covers all 16 grammar-shape
/// primitives found across the 32 hand-rolled implementations this engine replaces.
#[derive(Clone, Debug)]
pub enum Shape {
    Bool,
    Int,
    UInt,
    Float,
    Text,
    Bytes64,
    /// 🔀️ Unit-variant keyword table: `(tag, ordinal)` pairs.
    Enum(Vec<(String, u32)>),
    /// 👫️ Packed `x,y,z` — `len = Some(n)` enforces arity.
    Tuple(Box<Shape>, Option<usize>),
    /// 🧾️ Bracketed scalar lists `[a b c]`; each direct record item is braced `[ { fields } { } ]`.
    List(Box<Shape>),
    /// 📄️ Inline nested `key=value` run using another record's fields. Direct list items are braced. Lazy for the same
    /// reason `Statements` is: a self-referential `#[derive(DslRecord)]` struct (a field whose type
    /// recurses back to the struct itself, e.g. a dynamic-value type with a nested-dictionary-of-
    /// itself field) would otherwise recurse infinitely just building its own `RecordSpec`.
    Record(RecordSpecProducer),
    /// 🧱️ Wraps the inner shape in `{ ... }`.
    Block(Box<Shape>),
    /// 📡️ Keyword-dispatched, order-preserving repeated records: `(keyword, spec_fn)` per variant.
    /// `spec_fn` is a zero-capture `fn` pointer, not an eagerly-built `RecordSpec` — a genuinely
    /// self-referential grammar (a recursive block tree whose own variant table contains itself)
    /// would otherwise recurse infinitely just building the table. Calling `(spec_fn.ordinary)()` one level at
    /// a time bottoms out naturally at real documents' finite depth instead.
    Statements(Vec<(String, RecordSpecProducer)>),
    /// 🔳️ `{ key=value ... }` block, keys sorted on canonical print.
    Map(Box<Shape>),
    /// 📰️ Dynamic JSON-equivalent literal.
    Value,
    /// 🗺️ Structure-of-Arrays columnar table: `key [col:TYPE ...] { v11 v12 ...  v21 v22 ... }`.
    /// `fn() -> RecordSpec` is the SAME lazy self-referential seam `Record`/`Statements` use.
    /// Parses to `FieldValue::List(Vec<FieldValue::Record>)` — identical to `List(Record)` — so
    /// no binder/diff/derive path needs to know a field is a table rather than a verbose AoS list.
    /// Only a record's OWN keyword-prefixed field prints/parses the compact bare SoA form above
    /// (`print_record`/`parse_record_body`'s dedicated lookahead); a `Table` reached any other way
    /// (a table row's own column, a list element, the generic `key=` keyed dispatch) prints/parses
    /// as the bracketed AoS list `[ {...} {...} ]` instead — the bare form has no bracket of its
    /// own to mark where it ends, so it's only safe directly after a record's leading keyword.
    Table(RecordSpecProducer),
    /// 🔗️ Graph endpoint literal: `id[:kind][@port][->|--id2[:kind2][@port2]]{props}`.
    Wire,
    /// 📦️ A `Shape::Float` refinement: prints/parses with a glued unit suffix (`210GPa`). The value
    /// is stored in `unit`'s declared unit; a compatible alien suffix on parse (`210000MPa`)
    /// converts into it, an incompatible one (wrong dimension) is a parse error. No suffix at all
    /// means the bare number is already in the declared unit.
    Quantity(&'static semio_framework_dsl::UnitSpec),
    /// 🔄️ A `Shape::Quantity` restricted to angle units (`deg`/`rad`/`turn`) — kept as its own variant
    /// (rather than reusing `Quantity` with an angle unit) so `shape_type_name`/table headers can
    /// tell a length from a rotation at a glance (`NUM` vs `QTY` vs `ANG`).
    Angle(&'static semio_framework_dsl::UnitSpec),
    /// 🧷️ A `Shape::Text` refinement: a checked reference to an entity of the named kind (e.g.
    /// `"material"`). Prints/parses identically to `Text` (bare-preferred) — the only difference
    /// is semantic (a paired `FieldSpec.defines` anchor lets `LanguageService::validate` flag a
    /// dangling reference), so it needs no dedicated parse/print arm, only a distinct type name.
    Ref(&'static str),
    /// 📍️ `@x,y[,z,...]` — a placement/position literal, `dims` coordinates. Value is
    /// `FieldValue::Tuple` (same representation `Shape::Tuple` uses) with exactly `dims` floats.
    Coord(u8),
    /// 📁️ `^x,y,z` — a unit direction/axis vector, always exactly 3 floats. Value is
    /// `FieldValue::Tuple` — distinct from `Coord(3)` only by its `^` sigil and `DIR` type tag,
    /// so a reader never confuses "where" from "which way".
    Dir,
    /// ➖️ `WxHxD` (glued, no separator token — see `parse_dim`) — `dims` size components. Value is
    /// `FieldValue::Tuple` with exactly `dims` floats.
    Dim(u8),
    /// ↔️ `(lo..hi)` or `(lo..hi,step)` — value is `FieldValue::Tuple` of 2 or 3 floats (no dedicated
    /// `RangeValue` type: a range IS a small tuple, just printed with `..` instead of `,` between
    /// the first two elements).
    Range,
    /// 📜️ `xN` — a bare count/multiplicity literal. Value is `FieldValue::UInt`.
    Count,
    /// ➗️ `(expr)` — an arithmetic formula literal, always outer-parenthesized. Value is the ONE
    /// genuinely new `FieldValue` variant this engine adds (`FieldValue::Expr`) — everything else
    /// in this Shape reuses an existing representation.
    Expr,
    /// 🏠️ Fenced verbatim text in Document mode (`` ```lang\ncontent\n``` ``), escaped-quoted `Text`
    /// in Inline mode — both parse to the same `FieldValue::Text`, the "Document/Inline agree" law
    /// applied to a shape whose Document form needs raw multi-line content. `lang` is this field's
    /// DECLARED embedded language (e.g. `"jack"`); an authored fence's own lang tag must be empty
    /// or match it.
    Embed(&'static str),
    /// 🏨️ Fence language taken from a sibling Text field named by this key (see `#[dsl(lang_from)]`).
    EmbedFrom(&'static str),
}

#[derive(Clone, Debug)]
pub struct FieldSpec {
    pub id: u16,
    /// 🫥️ Empty for positional-only fields.
    pub key: String,
    /// 🧭️ `Some(n)` = nth positional token right after the keyword, in declaration order among
    /// positional fields.
    pub position: Option<u8>,
    pub shape: Shape,
    pub optional: bool,
    /// 🪆️ Splice a nested record's fields directly into this record (shared doc/op field schemas).
    pub flatten: bool,
    /// ⚖️ Paired with a sibling field's `Shape::Ref(kind)`: this field's value is the canonical id of
    /// an entity of kind `kind`. `None` for every field that isn't such an anchor. Not wire/hash
    /// relevant (LanguageService-only, see `Shape::Ref`'s doc comment) — purely an authoring aid.
    pub defines: Option<&'static str>,
    /// 📞️ The one field a `RecordLayout::Call` spec prints before `=` and parses as the assignment
    /// target — see [`RecordLayout::Call`]. Always `false` outside a `Call`-layout spec; ignored
    /// (never printed/parsed specially) for any other layout.
    pub is_call_name: bool,
}

impl FieldSpec {
    pub fn new(id: u16, key: &str, shape: Shape) -> Self {
        Self { id, key: key.to_string(), position: None, shape, optional: false, flatten: false, defines: None, is_call_name: false }
    }

    pub fn positional(mut self, index: u8) -> Self {
        self.position = Some(index);
        self
    }

    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    pub fn flatten(mut self) -> Self {
        self.flatten = true;
        self
    }

    pub fn defines(mut self, kind: &'static str) -> Self {
        self.defines = Some(kind);
        self
    }

    /// 📛️ Marks this field as the one printed before `=` / parsed as the assignment target
    /// in a `RecordLayout::Call` spec. See [`RecordLayout::Call`].
    pub fn call_name(mut self) -> Self {
        self.is_call_name = true;
        self
    }
}

#[derive(Clone, Debug)]
pub struct RecordSpec {
    pub keyword: Option<String>,
    pub layout: RecordLayout,
    pub fields: Vec<FieldSpec>,
}

impl RecordSpec {
    pub fn new(keyword: Option<&str>, layout: RecordLayout, fields: Vec<FieldSpec>) -> Self {
        Self { keyword: keyword.map(|k| k.to_string()), layout, fields }
    }

    /// 🏗️ Same as [`Self::new`] but takes an already-owned keyword — what
    /// `dsl_derive`-generated code builds from a spliced `String` literal.
    pub fn new_owned(keyword: Option<String>, layout: RecordLayout, fields: Vec<FieldSpec>) -> Self {
        Self { keyword, layout, fields }
    }
}

pub struct GrammarSpec {
    pub name: String,
    pub root: RecordSpec,
}
//#endregion 🔖️Shape

//#region 🔖️Value
use semio_framework_value::{DslValue,Number};

/// 🕸️ One endpoint (and optional edge) of a wire-literal.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct WireNode {
    pub id: String,
    pub kind: Option<String>,
    pub port: Option<String>,
}

/// 🏷️ Optional id/kind label on a wire edge (`-[e1:Connection]->` / fused `-e1:Connection>`).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct WireEdgeLabel {
    pub id: Option<String>,
    pub kind: Option<String>,
}

impl WireEdgeLabel {
    pub fn is_empty(&self) -> bool {
        self.id.is_none() && self.kind.is_none()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WireValue {
    pub from: WireNode,
    /// ⛓️ `Some((directed, to))` if this line describes an edge, `None` for a bare node declaration.
    pub edge: Option<(bool, WireNode)>,
    pub edge_label: WireEdgeLabel,
    pub properties: DslValue,
}

/// 🌳️ The parsed representation of one field's value — what a typed binder converts
/// to/from a concrete Rust value. Doubles as this v1 engine's "Cst": simplified (semantic, not a
/// full lossless syntax tree) but sufficient for round-tripping, diagnostics, and highlighting;
/// a real green/red tree can replace it later behind the same `parse`/`Writer` API.
#[derive(Clone, Debug, PartialEq)]
pub enum FieldValue {
    Bool(bool),
    Int(i64),
    UInt(u64),
    Float(f64),
    Text(String),
    Bytes64(RecordBytes),
    Enum(u32),
    Tuple(Vec<FieldValue>),
    List(Vec<FieldValue>),
    Record(RecordValue),
    Block(Box<FieldValue>),
    Statements(Vec<(String, RecordValue)>),
    Map(Vec<(String, FieldValue)>),
    Value(DslValue),
    Wire(WireValue),
    Expr(ExprValue),
    Absent,
}

#[path = "../../🧩️record/🗂️fields/🦀️.rs"]
mod record_fields;
pub use record_fields::RecordFields;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct RecordValue {
    pub fields: RecordFields,
}

impl RecordValue {
    pub fn get(&self, id: u16) -> Option<&FieldValue> {
        self.fields.get(&id)
    }
}

/// ⚙️ Alias naming the parse product per the engine's design vocabulary.
pub type Cst = RecordValue;
//#endregion 🔖️Value

//#region 🔖️Expr
/// ➕️ Arithmetic operators `Shape::Expr` supports — standard left-associative precedence
/// (`*`/`/` bind tighter than `+`/`-`), plus a call form for named functions (`min(a, b)`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExprOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl ExprOp {
    pub fn precedence(self) -> u8 {
        match self {
            ExprOp::Add | ExprOp::Sub => 1,
            ExprOp::Mul | ExprOp::Div => 2,
        }
    }

    pub fn symbol(self) -> &'static str {
        match self {
            ExprOp::Add => "+",
            ExprOp::Sub => "-",
            ExprOp::Mul => "*",
            ExprOp::Div => "/",
        }
    }
}

/// 🧮️ The parsed body of a `Shape::Expr` field — a small formula AST, e.g.
/// `1.35*G + 1.5*Q` parses to `Binary(Add, Binary(Mul, Num(1.35), Var("G")), Binary(Mul,
/// Num(1.5), Var("Q")))`. Deliberately NOT a general-purpose scripting language (no assignment, no
/// control flow, no boolean logic) — it's a formula literal, one notch above a bare number.
#[derive(Clone, Debug, PartialEq)]
pub enum ExprValue {
    Num(f64),
    /// 🔣️ A snake_case reference to a sibling field/symbol, resolved by the consuming technology
    /// (e.g. a norm calc-sheet's own `given`/prior `clause` definitions) — this engine only
    /// parses/prints the name, it never evaluates it.
    Var(String),
    Neg(Box<ExprValue>),
    Binary(ExprOp, Box<ExprValue>, Box<ExprValue>),
    Call(String, Vec<ExprValue>),
}
//#endregion 🔖️Expr


/// 🟢️ Original canonical physical shape vocabulary shared by parser and emitter.
pub fn shape_type_name(shape: &Shape) -> &'static str {
    match shape {
        Shape::Bool => "BOOL",
        Shape::Int => "INT",
        Shape::UInt => "UINT",
        Shape::Float => "NUM",
        Shape::Text => "TEXT",
        Shape::Bytes64 => "BYTES",
        Shape::Enum(_) => "ENUM",
        Shape::Tuple(_, _) => "TUPLE",
        Shape::List(_) => "LIST",
        Shape::Record(_) => "REC",
        Shape::Block(_) => "BLOCK",
        Shape::Statements(_) => "STMT",
        Shape::Map(_) => "MAP",
        Shape::Value => "VAL",
        Shape::Table(_) => "TABLE",
        Shape::Wire => "WIRE",
        Shape::Quantity(_) => "QTY",
        Shape::Angle(_) => "ANG",
        Shape::Ref(_) => "REF",
        Shape::Coord(_) => "CRD",
        Shape::Dir => "DIR",
        Shape::Dim(_) => "DIM",
        Shape::Range => "RNG",
        Shape::Count => "CNT",
        Shape::Expr => "EXPR",
        Shape::Embed(_) | Shape::EmbedFrom(_) => "EMBED",
    }
}

