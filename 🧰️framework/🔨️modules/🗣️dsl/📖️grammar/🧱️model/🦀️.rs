//! 🧱️ The `.grammar` / `.protocol` data model: dialects, productions, symbols, framing, blocks, primitives and the walk trace.

use semio_framework_dsl_lexical::lexer::LexOptions;

//#region 🔖️Model
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemioDialect {
    Grammar,
    Protocol,
}

/// 📄️ One parsed `.grammar.semio` / `.protocol.semio` file: header directives + productions.
/// `lex` (P2-M1) is the per-grammar string-quote/escape + comment dialect declared by `string`/
/// `comment` header directives — `LexOptions::default()` when the grammar declares neither,
/// reproducing the fixed pre-M1 alphabet exactly.
#[derive(Clone, Debug, PartialEq)]
pub struct GrammarFile {
    pub dialect: SemioDialect,
    pub id: String,
    pub extension: Option<String>,
    pub uses: Vec<String>,
    pub start: String,
    pub productions: Vec<Production>,
    pub lex: LexOptions,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Production {
    pub name: String,
    pub alternatives: Vec<Alternative>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Alternative {
    pub symbols: Vec<Symbol>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MacroArg {
    Literal(String),
    Ident(String),
}

/// 🧩️ One symbol in a production's alternative. `Terminal` vs. `Ref` is decided lexically
/// at parse time (an all-uppercase bareword is a terminal token-class name; anything else is a
/// reference to another production, or a zero-arg macro — resolved later, not by this parser).
#[derive(Clone, Debug, PartialEq)]
pub enum Symbol {
    Literal(String),
    Terminal(String),
    Ref(String),
    Macro(String, Vec<MacroArg>),
    Group(Vec<Alternative>),
    Optional(Box<Symbol>),
    Star(Box<Symbol>),
    Plus(Box<Symbol>),
}
//#endregion 🔖️Model

//#region 📡️ProtocolModel
/// 📡️ One parsed `.protocol.semio` file: framing + typed body directives.
#[derive(Clone, Debug, PartialEq)]
pub struct ProtocolFile {
    pub id: String,
    pub version: u16,
    pub schema: String,
    pub start: String,
    pub uses: Vec<String>,
    pub framing: Framing,
    pub blocks: Vec<Block>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Framing {
    Magic([u8; 8]),
    Record,
    Chunked,
    Literal,
}

/// 🔀️ P2-M2 item 4: guard gating a field's (or a whole segment's) presence on an
/// EARLIER-decoded field's value — bmp's BITFIELDS masks (`if compression eq 3`), palette
/// (`if bits_per_pixel le 8`). Evaluated against the walk-wide field env (item 3), so the guarded
/// field can reference a value decoded in ANY earlier block, not just the same one.
/// `bits` requires a nonzero unsigned mask and admits a field when `(value & mask) == mask`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cond {
    pub field: String,
    pub op: CondOp,
    pub value: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CondOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    BitsSet,
}

/// 🔁️ P2-M2 item 1: one "repeated tag-dispatched block" — read a discriminator (+ optional
/// length), branch into a known arm's fields or skip an unrecognized discriminator's declared
/// length as opaque bytes, repeat until EOF or a declared sentinel discriminator value (`until`).
/// `order` controls whether `length` is read before or after `discriminator` each iteration (GLB/
/// PNG read length-then-tag; GIF/JPG read tag-first with no per-iteration length at all).
#[derive(Clone, Debug, PartialEq)]
pub struct RepeatDispatch {
    pub discriminator: Prim,
    pub length: Option<Prim>,
    pub order: DispatchOrder,
    pub trailer: Option<Prim>,
    pub until: Option<Vec<u8>>,
    pub arms: Vec<RepeatArm>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DispatchOrder {
    TagFirst,
    LengthFirst,
}

/// 🌿️ One recognized discriminator value's field-set. `nested` supports the GIF 89a
/// two-level case (an extension-introducer arm dispatches AGAIN on the label byte) — recursive via
/// `NestedDispatch`'s own `Vec<RepeatArm>`, so nesting depth is not artificially capped at two.
#[derive(Clone, Debug, PartialEq)]
pub struct RepeatArm {
    pub tag: Vec<u8>,
    pub fields: Vec<Field>,
    pub nested: Option<NestedDispatch>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NestedDispatch {
    pub name: String,
    pub discriminator: Prim,
    pub arms: Vec<RepeatArm>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Block {
    Header(Vec<Field>),
    Segment {
        name: String,
        kind: Option<u8>,
        fields: Vec<Field>,
        cond: Option<Cond>,
    },
    Record {
        name: String,
        tag: Option<u64>,
        fields: Vec<Field>,
    },
    Struct {
        name: String,
        fields: Vec<Field>,
    },
    Enum {
        name: String,
        variants: Vec<(String, u64)>,
    },
    Footer(usize),
    Chain(Prim),
    /// 🔂️ P2-M2 item 1.
    Repeat {
        name: String,
        dispatch: RepeatDispatch,
    },
    /// 🔎️ P2-M2 item 5a: scan BACKWARD from EOF for `magic`'s exact byte pattern (ZIP's EOCD, whose
    /// preceding comment field is 0-65535 bytes — its start is unknowable except by finding the
    /// EOCD itself first), jump `pos` directly to the match, then walk `fields` forward from there.
    BackwardScan {
        name: String,
        magic: Vec<u8>,
        fields: Vec<Field>,
    },
    /// 🦘️ P2-M2 item 5b: jump `pos` to the ABSOLUTE offset held in `offset_field` (decoded by an
    /// earlier block — ZIP's EOCD `cd_offset`), then walk `fields` forward from there. A genuine,
    /// deliberate exception to "position only increases" — see `walk_protocol`'s `jumped` handling.
    JumpTo {
        name: String,
        offset_field: String,
        fields: Vec<Field>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub name: String,
    pub ty: Prim,
    pub cond: Option<Cond>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Prim {
    U8,
    U16,
    U32,
    U64,
    I32,
    I64,
    F32,
    F64,
    /// 🐘️ P2-M2 item 2: ALWAYS big-endian regardless of the walker's runtime endian mode (png/jpg/
    /// deflate's trailer/ply's BE variant/pdf 1.7's xref rows — formats with a static, author-time-
    /// known byte order). Distinct from plain `U16`/`U32`/... which obey the walker's current mode
    /// (LE by default, flippable at runtime by a `Prim::Endian` field — P2-M2 item 6).
    U16Be,
    U32Be,
    U64Be,
    I32Be,
    I64Be,
    F32Be,
    F64Be,
    Varint,
    Zigzag,
    Bytes,
    Utf8,
    Fixed(usize),
    Array(Box<Prim>, Count),
    Ref(String),
    Tag,
    /// 🏷️ P2-M2 item 1c: scan forward past every `prefix` byte, then take the next byte as the
    /// discriminator — JPG's marker-prefix scan (`0xFF` fill bytes before a real marker code),
    /// distinct from a fixed-position tag read.
    MarkerScan(u8),
    /// 🖼️ P2-M2 item 6: TIFF-style runtime-selected endianness. Reads `key.len()` bytes (all keys
    /// must share one width — TIFF's `II`/`MM` are both 2), matches against the declared
    /// `(key, is_big_endian)` table, and MUTATES the walker's endian mode for every subsequent
    /// plain (non-`Be`-suffixed) `Prim` read for the REMAINDER of the walk. A walker-state mutation,
    /// not a value binding — distinct from `U16Be`/etc.'s static per-format choice (item 2).
    Endian(Vec<(String, bool)>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Count {
    Fixed(usize),
    Varint,
    Field(String),
}

/// ✅️ Successful byte walk: every declared wire slot consumed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProtocolTrace {
    pub consumed: usize,
}

/// ❌️ Spec/bytes disagreement at a concrete offset.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProtocolMismatch {
    pub offset: usize,
    pub message: String,
}
//#endregion 📡️ProtocolModel

/// 📏 The fixed byte width of a primitive, or none when its width depends on the data.
pub fn prim_fixed_width(prim: &Prim) -> Option<usize> {
    match prim {
        Prim::U8 => Some(1),
        Prim::U16 | Prim::U16Be => Some(2),
        Prim::U32 | Prim::I32 | Prim::F32 | Prim::U32Be | Prim::I32Be | Prim::F32Be => Some(4),
        Prim::U64 | Prim::I64 | Prim::F64 | Prim::U64Be | Prim::I64Be | Prim::F64Be => Some(8),
        Prim::Fixed(n) => Some(*n),
        Prim::MarkerScan(_) => Some(1),
        Prim::Endian(arms) => Some(arms.first().map_or(0, |(k, _)| k.len())),
        Prim::Varint | Prim::Zigzag | Prim::Tag | Prim::Bytes | Prim::Utf8 | Prim::Array(_, _) | Prim::Ref(_) => None,
    }
}
