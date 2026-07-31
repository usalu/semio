//! 🧬️ `dsl_schema` — the data-driven declarative grammar engine: technologies describe their
//! document/op grammar as `RecordSpec`/`Shape` DATA (not code), and this crate parses text against
//! that data into a generic `Cst` (walked by typed binders that `dsl_derive` will generate) and
//! prints it back via a chunk `Writer` that structurally guarantees the newline law: every
//! grammar renders both as multi-line canonical `Document` text and as one space-joined `Inline`
//! line, and both re-parse to the same value.

use dsl_core::{format_f64, lex, parse_f64, Limits, SpannedToken, TextError, TextSpan, TokenClass, TokenKind};
use std::collections::{HashMap, HashSet};

//#region 🔖️Shape
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecordLayout {
    /// All fields printed as space-joined `key=value` tokens on one logical unit.
    Inline,
    /// Each field printed as its own line/statement (norm-family style) in Document mode;
    /// collapses to the same space-joined form as `Inline` when rendered in `JoinMode::Inline`.
    Lines,
}

/// @emoji 🧩️ What one field's value looks like, textually. Covers all 16 grammar-shape
/// primitives found across the 32 hand-rolled implementations this engine replaces.
#[derive(Clone, Debug)]
pub enum Shape {
    Bool,
    Int,
    UInt,
    Float,
    Text,
    Bytes64,
    /// Unit-variant keyword table: `(tag, ordinal)` pairs.
    Enum(Vec<(String, u32)>),
    /// Packed `x,y,z` — `len = Some(n)` enforces arity.
    Tuple(Box<Shape>, Option<usize>),
    /// Bracketed `[a b c]`.
    List(Box<Shape>),
    /// Inline nested `key=value` run using another record's fields, unwrapped. Lazy for the same
    /// reason `Statements` is: a self-referential `#[derive(DslRecord)]` struct (a field whose type
    /// recurses back to the struct itself, e.g. a dynamic-value type with a nested-dictionary-of-
    /// itself field) would otherwise recurse infinitely just building its own `RecordSpec`.
    Record(fn() -> RecordSpec),
    /// Wraps the inner shape in `{ ... }`.
    Block(Box<Shape>),
    /// Keyword-dispatched, order-preserving repeated records: `(keyword, spec_fn)` per variant.
    /// `spec_fn` is a zero-capture `fn` pointer, not an eagerly-built `RecordSpec` — a genuinely
    /// self-referential grammar (a recursive block tree whose own variant table contains itself)
    /// would otherwise recurse infinitely just building the table. Calling `spec_fn()` one level at
    /// a time bottoms out naturally at real documents' finite depth instead.
    Statements(Vec<(String, fn() -> RecordSpec)>),
    /// `{ key=value ... }` block, keys sorted on canonical print.
    Map(Box<Shape>),
    /// Dynamic JSON-equivalent literal.
    Value,
    /// Structure-of-Arrays columnar table: `key [col:TYPE ...] { v11 v12 ...  v21 v22 ... }`.
    /// `fn() -> RecordSpec` is the SAME lazy self-referential seam `Record`/`Statements` use.
    /// Parses to `FieldValue::List(Vec<FieldValue::Record>)` — identical to `List(Record)` — so
    /// no binder/diff/derive path needs to know a field is a table rather than a verbose AoS list.
    /// Only a record's OWN keyword-prefixed field prints/parses the compact bare SoA form above
    /// (`print_record`/`parse_record_body`'s dedicated lookahead); a `Table` reached any other way
    /// (a table row's own column, a list element, the generic `key=` keyed dispatch) prints/parses
    /// as the bracketed AoS list `[ {...} {...} ]` instead — the bare form has no bracket of its
    /// own to mark where it ends, so it's only safe directly after a record's leading keyword.
    Table(fn() -> RecordSpec),
    /// Graph endpoint literal: `id[:kind][@port][->|--id2[:kind2][@port2]]{props}`.
    Wire,
    /// A `Shape::Float` refinement: prints/parses with a glued unit suffix (`210GPa`). The value
    /// is stored in `unit`'s declared unit; a compatible alien suffix on parse (`210000MPa`)
    /// converts into it, an incompatible one (wrong dimension) is a parse error. No suffix at all
    /// means the bare number is already in the declared unit.
    Quantity(&'static dsl_core::UnitSpec),
    /// A `Shape::Quantity` restricted to angle units (`deg`/`rad`/`turn`) — kept as its own variant
    /// (rather than reusing `Quantity` with an angle unit) so `shape_type_name`/table headers can
    /// tell a length from a rotation at a glance (`NUM` vs `QTY` vs `ANG`).
    Angle(&'static dsl_core::UnitSpec),
    /// A `Shape::Text` refinement: a checked reference to an entity of the named kind (e.g.
    /// `"material"`). Prints/parses identically to `Text` (bare-preferred) — the only difference
    /// is semantic (a paired `FieldSpec.defines` anchor lets `LanguageService::validate` flag a
    /// dangling reference), so it needs no dedicated parse/print arm, only a distinct type name.
    Ref(&'static str),
    /// `@x,y[,z,...]` — a placement/position literal, `dims` coordinates. Value is
    /// `FieldValue::Tuple` (same representation `Shape::Tuple` uses) with exactly `dims` floats.
    Coord(u8),
    /// `^x,y,z` — a unit direction/axis vector, always exactly 3 floats. Value is
    /// `FieldValue::Tuple` — distinct from `Coord(3)` only by its `^` sigil and `DIR` type tag,
    /// so a reader never confuses "where" from "which way".
    Dir,
    /// `WxHxD` (glued, no separator token — see `parse_dim`) — `dims` size components. Value is
    /// `FieldValue::Tuple` with exactly `dims` floats.
    Dim(u8),
    /// `(lo..hi)` or `(lo..hi,step)` — value is `FieldValue::Tuple` of 2 or 3 floats (no dedicated
    /// `RangeValue` type: a range IS a small tuple, just printed with `..` instead of `,` between
    /// the first two elements).
    Range,
    /// `xN` — a bare count/multiplicity literal. Value is `FieldValue::UInt`.
    Count,
    /// `(expr)` — an arithmetic formula literal, always outer-parenthesized. Value is the ONE
    /// genuinely new `FieldValue` variant this engine adds (`FieldValue::Expr`) — everything else
    /// in this Shape reuses an existing representation.
    Expr,
    /// Fenced verbatim text in Document mode (`` ```lang\ncontent\n``` ``), escaped-quoted `Text`
    /// in Inline mode — both parse to the same `FieldValue::Text`, the "Document/Inline agree" law
    /// applied to a shape whose Document form needs raw multi-line content. `lang` is this field's
    /// DECLARED embedded language (e.g. `"jack"`); an authored fence's own lang tag must be empty
    /// or match it.
    Embed(&'static str),
}

#[derive(Clone, Debug)]
pub struct FieldSpec {
    pub id: u16,
    /// Empty for positional-only fields.
    pub key: String,
    /// `Some(n)` = nth positional token right after the keyword, in declaration order among
    /// positional fields.
    pub position: Option<u8>,
    pub shape: Shape,
    pub optional: bool,
    /// Splice a nested record's fields directly into this record (shared doc/op field schemas).
    pub flatten: bool,
    /// Paired with a sibling field's `Shape::Ref(kind)`: this field's value is the canonical id of
    /// an entity of kind `kind`. `None` for every field that isn't such an anchor. Not wire/hash
    /// relevant (LanguageService-only, see `Shape::Ref`'s doc comment) — purely an authoring aid.
    pub defines: Option<&'static str>,
}

impl FieldSpec {
    pub fn new(id: u16, key: &str, shape: Shape) -> Self {
        Self { id, key: key.to_string(), position: None, shape, optional: false, flatten: false, defines: None }
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

    /// @emoji 🏗️ Same as [`Self::new`] but takes an already-owned keyword — what
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
/// @emoji 🌱️ Dynamic JSON-equivalent literal for schema-less fields (`Shape::Value`).
#[derive(Clone, Debug, PartialEq)]
pub enum DslValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<DslValue>),
    Object(Vec<(String, DslValue)>),
}

/// @emoji 🕸️ One endpoint (and optional edge) of a wire-literal.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct WireNode {
    pub id: String,
    pub kind: Option<String>,
    pub port: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WireValue {
    pub from: WireNode,
    /// `Some((directed, to))` if this line describes an edge, `None` for a bare node declaration.
    pub edge: Option<(bool, WireNode)>,
    pub properties: DslValue,
}

/// @emoji 🌳️ The parsed representation of one field's value — what a typed binder converts
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
    Bytes64(Vec<u8>),
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

#[derive(Clone, Debug, PartialEq, Default)]
pub struct RecordValue {
    pub fields: HashMap<u16, FieldValue>,
}

impl RecordValue {
    pub fn get(&self, id: u16) -> Option<&FieldValue> {
        self.fields.get(&id)
    }
}

/// @emoji 🌳️ Alias naming the parse product per the engine's design vocabulary.
pub type Cst = RecordValue;
//#endregion 🔖️Value

//#region 🔖️Expr
/// @emoji ➕️ Arithmetic operators `Shape::Expr` supports — standard left-associative precedence
/// (`*`/`/` bind tighter than `+`/`-`), plus a call form for named functions (`min(a, b)`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExprOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl ExprOp {
    fn precedence(self) -> u8 {
        match self {
            ExprOp::Add | ExprOp::Sub => 1,
            ExprOp::Mul | ExprOp::Div => 2,
        }
    }

    fn symbol(self) -> &'static str {
        match self {
            ExprOp::Add => "+",
            ExprOp::Sub => "-",
            ExprOp::Mul => "*",
            ExprOp::Div => "/",
        }
    }
}

/// @emoji 🧮️ The parsed body of a `Shape::Expr` field — a small formula AST, e.g.
/// `1.35*G + 1.5*Q` parses to `Binary(Add, Binary(Mul, Num(1.35), Var("G")), Binary(Mul,
/// Num(1.5), Var("Q")))`. Deliberately NOT a general-purpose scripting language (no assignment, no
/// control flow, no boolean logic) — it's a formula literal, one notch above a bare number.
#[derive(Clone, Debug, PartialEq)]
pub enum ExprValue {
    Num(f64),
    /// A snake_case reference to a sibling field/symbol, resolved by the consuming technology
    /// (e.g. a norm calc-sheet's own `given`/prior `clause` definitions) — this engine only
    /// parses/prints the name, it never evaluates it.
    Var(String),
    Neg(Box<ExprValue>),
    Binary(ExprOp, Box<ExprValue>, Box<ExprValue>),
    Call(String, Vec<ExprValue>),
}
//#endregion 🔖️Expr

//#region 🔖️Cursor
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceMode {
    Document,
    Inline,
}

struct Cursor {
    tokens: Vec<SpannedToken>,
    pos: usize,
    limits: Limits,
}

impl Cursor {
    // `SourceMode` no longer participates in parsing (its only consumer, `RawLines`, is gone —
    // `Shape::Text` now accepts `Ident|Text` identically regardless of Document/Inline); it stays
    // a `ParseOptions`/`parse` public-API distinction only, still meaningful to callers choosing
    // between `dsl::__rt::parse_document_record`/`parse_inline_record`.
    fn new(tokens: Vec<SpannedToken>, limits: Limits) -> Self {
        let tokens: Vec<SpannedToken> = tokens.into_iter().filter(|t| !t.kind.is_trivia()).collect();
        Self { tokens, pos: 0, limits }
    }

    fn peek(&self) -> &SpannedToken {
        &self.tokens[self.pos.min(self.tokens.len() - 1)]
    }

    fn peek_at(&self, offset: usize) -> &SpannedToken {
        let idx = (self.pos + offset).min(self.tokens.len() - 1);
        &self.tokens[idx]
    }

    fn span(&self) -> TextSpan {
        self.peek().span
    }

    fn advance(&mut self) -> SpannedToken {
        let token = self.tokens[self.pos.min(self.tokens.len() - 1)].clone();
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        token
    }

    fn expect(&mut self, kind: TokenKind) -> Result<SpannedToken, TextError> {
        if self.peek().kind == kind {
            Ok(self.advance())
        } else {
            Err(TextError::new(format!("expected {:?}, found {:?} '{}'", kind, self.peek().kind, self.peek().text.as_str()), self.span()))
        }
    }

    /// @emoji 🔎️ Whether the next token is an `Ident` that is followed by `=` — the LL(2)
    /// lookahead that makes the grammar newline-insensitive: a bare ident followed by `=` is
    /// always a `key=value` attribute, never the start of a new statement.
    fn at_attr_key(&self) -> Option<String> {
        if self.peek().kind == TokenKind::Ident && self.peek_at(1).kind == TokenKind::Equals {
            Some(self.peek().text.as_str().to_string())
        } else {
            None
        }
    }

    fn at_keyword(&self, keyword: &str) -> bool {
        self.peek().kind == TokenKind::Ident && self.peek().text.as_str().as_ref() == keyword
    }
}
//#endregion 🔖️Cursor

//#region 🔖️Parser
pub struct ParseOptions {
    pub limits: Limits,
    pub mode: SourceMode,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self { limits: Limits::default(), mode: SourceMode::Document }
    }
}

/// @emoji ✂️ The structural seam between lexing and parsing: everything downstream of a token
/// vector is grammar-only and needs no raw source bytes (the parser is token-only — no shape
/// still consumes verbatim source text the way the deleted `RawLines` shape once did). Exists so
/// a caller that already has tokens (e.g. an incremental relexer) can skip `parse`'s own lex pass.
pub fn parse_tokens(tokens: Vec<SpannedToken>, spec: &RecordSpec, opts: &ParseOptions) -> Result<Cst, TextError> {
    let mut cursor = Cursor::new(tokens, opts.limits);
    parse_record_body(&mut cursor, spec, 0)
}

pub fn parse(text: &str, spec: &RecordSpec, opts: &ParseOptions) -> Result<Cst, TextError> {
    let tokens = lex(text, &opts.limits, false)?;
    parse_tokens(tokens, spec, opts)
}

fn ident_like_text(token: &SpannedToken) -> String {
    token.text.as_str().to_string()
}

fn parse_scalar(cursor: &mut Cursor, shape: &Shape) -> Result<FieldValue, TextError> {
    match shape {
        Shape::Bool => {
            let token = cursor.expect(TokenKind::Ident)?;
            match token.text.as_str().as_ref() {
                "true" => Ok(FieldValue::Bool(true)),
                "false" => Ok(FieldValue::Bool(false)),
                other => Err(TextError::new(format!("expected 'true' or 'false', found '{other}'"), token.span)),
            }
        }
        Shape::Int => {
            let token = cursor.expect(TokenKind::Int)?;
            let value: i64 = token.text.as_str().parse().map_err(|_| TextError::new(format!("invalid integer '{}'", token.text.as_str()), token.span))?;
            Ok(FieldValue::Int(value))
        }
        Shape::UInt => {
            let token = cursor.expect(TokenKind::Int)?;
            let value: u64 = token.text.as_str().parse().map_err(|_| TextError::new(format!("invalid unsigned integer '{}'", token.text.as_str()), token.span))?;
            Ok(FieldValue::UInt(value))
        }
        Shape::Float => {
            let is_float_token = matches!(cursor.peek().kind, TokenKind::Float | TokenKind::Int)
                || (cursor.peek().kind == TokenKind::Ident && matches!(cursor.peek().text.as_str().as_ref(), "nan" | "inf" | "-inf"));
            if !is_float_token {
                return Err(TextError::new(format!("expected a float, found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()), cursor.span()));
            }
            let token = cursor.advance();
            let value = parse_f64(&token.text.as_str()).map_err(|e| TextError::new(e, token.span))?;
            Ok(FieldValue::Float(value))
        }
        Shape::Text => parse_scalar_text(cursor),
        Shape::Bytes64 => {
            let token = cursor.expect(TokenKind::Text)?;
            let bytes = base64_decode(&token.text.as_str()).map_err(|e| TextError::new(e, token.span))?;
            Ok(FieldValue::Bytes64(bytes))
        }
        Shape::Enum(variants) => {
            let token = cursor.expect(TokenKind::Ident)?;
            let text = token.text.as_str();
            variants
                .iter()
                .find(|(tag, _)| tag == text.as_ref())
                .map(|(_, ordinal)| FieldValue::Enum(*ordinal))
                .ok_or_else(|| TextError::new(format!("unknown enum tag '{text}'"), token.span))
        }
        Shape::Quantity(declared) | Shape::Angle(declared) => parse_quantity(cursor, declared),
        Shape::Ref(_) => parse_scalar_text(cursor),
        Shape::Embed(declared_lang) => parse_embed(cursor, declared_lang),
        Shape::Count => {
            if cursor.peek().kind != TokenKind::Ident {
                return Err(TextError::new(format!("expected a count literal like 'x24', found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()), cursor.span()));
            }
            let token = cursor.advance();
            let text = token.text.as_str();
            let digits = text.strip_prefix('x').ok_or_else(|| TextError::new(format!("expected a count literal like 'x24', found '{text}'"), token.span))?;
            let value: u64 = digits.parse().map_err(|_| TextError::new(format!("invalid count literal 'x{digits}'"), token.span))?;
            Ok(FieldValue::UInt(value))
        }
        other => Err(TextError::new(format!("shape {other:?} is not a scalar"), cursor.span())),
    }
}

/// @emoji 🧮️ Precedence-climbing entry point for `Shape::Expr`'s body (called with the caller's
/// outer `(`/`)` already consumed). `min_prec` is the lowest operator precedence this call is
/// willing to keep consuming at — the standard technique for turning a flat token stream into a
/// precedence-correct tree without a separate tokenize-then-shunting-yard pass.
fn parse_expr(cursor: &mut Cursor, min_prec: u8) -> Result<ExprValue, TextError> {
    let lhs = parse_expr_unary(cursor)?;
    parse_expr_continue(cursor, min_prec, lhs)
}

/// @emoji 🧮️ The loop body of `parse_expr`, factored out so the glued-negative-number case below
/// can re-enter it with an ALREADY-PARSED left operand instead of calling `parse_expr_unary` again
/// (which would re-consume nothing, since the token was already consumed to build that operand).
fn parse_expr_continue(cursor: &mut Cursor, min_prec: u8, mut lhs: ExprValue) -> Result<ExprValue, TextError> {
    loop {
        // The shared lexer glues a leading `-` onto an immediately-following digit as ONE negative
        // number token (`y=-2`'s existing, load-bearing behavior — see dsl_core's lexer) — so
        // `10-2` lexes as `Int(10), Int(-2)`, not `Int(10), Minus, Int(2)`. Detect that shape here
        // and reinterpret it as `Sub` with a positive right operand, rather than requiring authors
        // to always space out `-` (canonical PRINT output always does; hand-written input may not).
        let glued_negative = matches!(cursor.peek().kind, TokenKind::Float | TokenKind::Int) && cursor.peek().text.as_str().starts_with('-');
        let (op, prec) = if glued_negative {
            (ExprOp::Sub, ExprOp::Sub.precedence())
        } else {
            match cursor.peek().kind {
                TokenKind::Plus => (ExprOp::Add, ExprOp::Add.precedence()),
                TokenKind::Minus => (ExprOp::Sub, ExprOp::Sub.precedence()),
                TokenKind::Star => (ExprOp::Mul, ExprOp::Mul.precedence()),
                TokenKind::Slash => (ExprOp::Div, ExprOp::Div.precedence()),
                _ => break,
            }
        };
        if prec < min_prec {
            break;
        }
        let rhs = if glued_negative {
            let token = cursor.advance();
            let value = parse_f64(&token.text.as_str()).map_err(|e| TextError::new(e, token.span))?;
            parse_expr_continue(cursor, prec + 1, ExprValue::Num(-value))?
        } else {
            cursor.advance();
            parse_expr(cursor, prec + 1)?
        };
        lhs = ExprValue::Binary(op, Box::new(lhs), Box::new(rhs));
    }
    Ok(lhs)
}

fn parse_expr_unary(cursor: &mut Cursor) -> Result<ExprValue, TextError> {
    if cursor.peek().kind == TokenKind::Minus {
        cursor.advance();
        return Ok(ExprValue::Neg(Box::new(parse_expr_unary(cursor)?)));
    }
    parse_expr_primary(cursor)
}

fn parse_expr_primary(cursor: &mut Cursor) -> Result<ExprValue, TextError> {
    match cursor.peek().kind {
        TokenKind::Float | TokenKind::Int => {
            let token = cursor.advance();
            let value = parse_f64(&token.text.as_str()).map_err(|e| TextError::new(e, token.span))?;
            Ok(ExprValue::Num(value))
        }
        TokenKind::Ident => {
            let token = cursor.advance();
            let name = ident_like_text(&token);
            if cursor.peek().kind == TokenKind::LParen {
                cursor.advance();
                let mut args = Vec::new();
                if cursor.peek().kind != TokenKind::RParen {
                    loop {
                        args.push(parse_expr(cursor, 0)?);
                        if cursor.peek().kind == TokenKind::Comma {
                            cursor.advance();
                            continue;
                        }
                        break;
                    }
                }
                cursor.expect(TokenKind::RParen)?;
                Ok(ExprValue::Call(name, args))
            } else {
                Ok(ExprValue::Var(name))
            }
        }
        TokenKind::LParen => {
            cursor.advance();
            let inner = parse_expr(cursor, 0)?;
            cursor.expect(TokenKind::RParen)?;
            Ok(inner)
        }
        other => Err(TextError::new(format!("expected a number, variable, or '(', found {other:?} '{}'", cursor.peek().text.as_str()), cursor.span())),
    }
}

/// @emoji 🧮️ Standalone entry point for parsing a bare expression body (no surrounding `(`/`)`,
/// unlike `Shape::Expr`'s own field-value grammar) — what `pack_value`'s decoder calls to turn the
/// canonical string it stored back into an `ExprValue`, since decode has no `Cursor` of its own.
pub fn parse_expr_text(text: &str) -> Result<ExprValue, TextError> {
    let tokens = lex(text, &Limits::default(), false)?;
    let mut cursor = Cursor::new(tokens, Limits::default());
    let value = parse_expr(&mut cursor, 0)?;
    cursor.expect(TokenKind::Eof)?;
    Ok(value)
}

/// @emoji 🎨️ Canonical `Shape::Expr` printer. Parenthesizes the minimum necessary to guarantee
/// `parse_expr(print_expr(e)) == e` for EVERY tree shape (not just canonically-left-nested ones):
/// a `Binary` right operand is parenthesized whenever its own precedence isn't STRICTLY higher
/// than the parent's (so even a commutative `a+(b+c)` keeps its parens — losing them would
/// reparse as the structurally different `(a+b)+c`), and a left operand only when strictly lower
/// (left-associativity already makes equal precedence safe there).
pub fn print_expr(expr: &ExprValue) -> String {
    print_expr_prec(expr, 0)
}

fn print_expr_prec(expr: &ExprValue, min_prec: u8) -> String {
    let (body, own_prec) = match expr {
        ExprValue::Num(v) => (format_f64(*v), 255),
        ExprValue::Var(name) => (name.clone(), 255),
        ExprValue::Call(name, args) => {
            let joined = args.iter().map(|a| print_expr_prec(a, 0)).collect::<Vec<_>>().join(", ");
            (format!("{name}({joined})"), 255)
        }
        // min_prec=4 is higher than every binary op (max 2) and Neg's own rank (3), so a nested
        // Binary OR another Neg always gets parenthesized — the latter specifically avoids ever
        // printing adjacent `--`, which would relex as `DashArrow`, not two `Minus` tokens.
        ExprValue::Neg(inner) => (format!("-{}", print_expr_prec(inner, 4)), 3),
        ExprValue::Binary(op, l, r) => {
            let prec = op.precedence();
            let l_text = print_expr_prec(l, prec);
            let r_text = print_expr_prec(r, prec + 1);
            (format!("{l_text} {} {r_text}", op.symbol()), prec)
        }
    };
    if own_prec < min_prec {
        format!("({body})")
    } else {
        body
    }
}

/// @emoji 📛️ `Shape::Text`'s own body, factored out so `Shape::Ref` (identical grammar, distinct
/// type only) can share it without a redundant match arm duplicating both branches.
fn parse_scalar_text(cursor: &mut Cursor) -> Result<FieldValue, TextError> {
    match cursor.peek().kind {
        TokenKind::Text => {
            let token = cursor.advance();
            let text = dsl_core::unescape_text(&token.text.as_str(), false).map_err(|e| TextError::new(e, token.span))?;
            Ok(FieldValue::Text(text))
        }
        TokenKind::Ident => {
            let token = cursor.advance();
            Ok(FieldValue::Text(ident_like_text(&token)))
        }
        other => Err(TextError::new(format!("expected Text, found {other:?} '{}'", cursor.peek().text.as_str()), cursor.span())),
    }
}

/// @emoji 🗣️ `Shape::Embed`'s parse: a `Fence` token (Document mode — see `dsl_core`'s lexer for
/// the `lang\u{0}content` encoding) with an empty or matching lang tag, OR anything
/// `parse_scalar_text` already accepts (Inline mode's escaped-quoted fallback) — both converge on
/// the same `FieldValue::Text`, which is what makes Document/Inline renders agree.
fn parse_embed(cursor: &mut Cursor, declared_lang: &'static str) -> Result<FieldValue, TextError> {
    if cursor.peek().kind == TokenKind::Fence {
        let token = cursor.advance();
        let raw = token.text.as_str();
        let (lang, content) = raw.split_once('\u{0}').ok_or_else(|| TextError::new("malformed fence token (missing separator)", token.span))?;
        if !lang.is_empty() && lang != declared_lang {
            return Err(TextError::new(format!("fence declares lang '{lang}', field expects '{declared_lang}'"), token.span));
        }
        return Ok(FieldValue::Text(content.to_string()));
    }
    parse_scalar_text(cursor)
}

/// @emoji 📐️ Shared parse for `Shape::Quantity`/`Shape::Angle`: a number, optionally followed by a
/// GLUED (no whitespace between — the lexer already ends a numeric token exactly where the next
/// `Ident` token begins for input like `210GPa`) unit-symbol ident. No suffix means the number is
/// already expressed in `declared`'s unit; a suffix converts, erroring if the dimensions differ.
fn parse_quantity(cursor: &mut Cursor, declared: &'static dsl_core::UnitSpec) -> Result<FieldValue, TextError> {
    let is_number_token = matches!(cursor.peek().kind, TokenKind::Float | TokenKind::Int)
        || (cursor.peek().kind == TokenKind::Ident && matches!(cursor.peek().text.as_str().as_ref(), "nan" | "inf" | "-inf"));
    if !is_number_token {
        return Err(TextError::new(format!("expected a quantity, found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()), cursor.span()));
    }
    let number_token = cursor.advance();
    let value = parse_f64(&number_token.text.as_str()).map_err(|e| TextError::new(e, number_token.span))?;
    let suffix = cursor.peek();
    if suffix.kind == TokenKind::Ident && suffix.byte_range.0 == number_token.byte_range.1 {
        let suffix_token = cursor.advance();
        let symbol = suffix_token.text.as_str().to_string();
        let suffix_unit = dsl_core::unit_by_symbol(&symbol).ok_or_else(|| TextError::new(format!("unknown unit '{symbol}'"), suffix_token.span))?;
        let converted = dsl_core::convert(value, suffix_unit, declared)
            .ok_or_else(|| TextError::new(format!("unit '{symbol}' is not compatible with expected unit '{}'", declared.symbol), suffix_token.span))?;
        Ok(FieldValue::Float(converted))
    } else {
        Ok(FieldValue::Float(value))
    }
}

/// @emoji 🔢️ Reads one `Float|Int` token as `f64` — the plain-number leaf `Shape::Coord`/`Dir`/
/// `Dim`/`Range` compose from (unlike `parse_quantity`, no unit-suffix consumption: these shapes'
/// components are always dimensionless numbers or already-declared-unit numbers).
fn parse_plain_number(cursor: &mut Cursor) -> Result<f64, TextError> {
    if !matches!(cursor.peek().kind, TokenKind::Float | TokenKind::Int) {
        return Err(TextError::new(format!("expected a number, found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()), cursor.span()));
    }
    let token = cursor.advance();
    parse_f64(&token.text.as_str()).map_err(|e| TextError::new(e, token.span))
}

/// @emoji 📍️ Shared body for `Shape::Coord`/`Shape::Dir`: a fixed-arity comma-separated run of
/// plain numbers, with no delimiter of its own (the caller already consumed the `@`/`^` sigil).
fn parse_fixed_number_tuple(cursor: &mut Cursor, arity: usize, what: &str) -> Result<FieldValue, TextError> {
    let mut items = Vec::with_capacity(arity);
    loop {
        items.push(FieldValue::Float(parse_plain_number(cursor)?));
        if items.len() == arity {
            break;
        }
        cursor.expect(TokenKind::Comma)?;
    }
    if cursor.peek().kind == TokenKind::Comma {
        return Err(TextError::new(format!("{what} literal expects exactly {arity} components"), cursor.span()));
    }
    Ok(FieldValue::Tuple(items))
}

/// @emoji 📏️ `Shape::Dim`'s `WxHxD` grammar: the FIRST number is an ordinary `Float|Int` token;
/// every number after it is glued (no whitespace, no comma) onto an `x`-prefixed ident — the
/// lexer has no notion of a bare `x` operator (digits/`.` are ident-continue, so `x0.12x0.24`
/// lexes as ONE `Ident` token), so this splits that single glued token on `x` itself rather than
/// looping token-by-token the way `parse_fixed_number_tuple` does.
fn parse_dim(cursor: &mut Cursor, dims: usize) -> Result<FieldValue, TextError> {
    let first_token = cursor.peek().clone();
    let first = parse_plain_number(cursor)?;
    let mut items = vec![FieldValue::Float(first)];
    if dims > 1 {
        let suffix = cursor.peek();
        if suffix.kind != TokenKind::Ident || suffix.byte_range.0 != first_token.byte_range.1 {
            return Err(TextError::new(format!("dimension literal expects {dims} components glued with 'x' (e.g. '2x3'), found only one"), cursor.span()));
        }
        let suffix_token = cursor.advance();
        let suffix_text = suffix_token.text.as_str();
        let parts: Vec<&str> = suffix_text.split('x').collect();
        // `"x0.12x0.24".split('x')` yields `["", "0.12", "0.24"]` — the leading empty piece is the
        // text before the first `x`, which is always empty since the suffix itself starts with it.
        if parts.first() != Some(&"") || parts.len() != dims {
            return Err(TextError::new(format!("dimension literal expects {dims} components glued with 'x', found '{}{}'", format_f64(first), suffix_text), suffix_token.span));
        }
        for part in &parts[1..] {
            let value = parse_f64(part).map_err(|_| TextError::new(format!("invalid dimension component '{part}'"), suffix_token.span))?;
            items.push(FieldValue::Float(value));
        }
    }
    Ok(FieldValue::Tuple(items))
}

fn parse_shape(cursor: &mut Cursor, shape: &Shape, depth: usize) -> Result<FieldValue, TextError> {
    cursor.limits.check_depth(depth, cursor.span())?;
    match shape {
        Shape::Bool | Shape::Int | Shape::UInt | Shape::Float | Shape::Text | Shape::Bytes64 | Shape::Enum(_) | Shape::Quantity(_) | Shape::Angle(_) | Shape::Ref(_) | Shape::Count | Shape::Embed(_) => parse_scalar(cursor, shape),
        Shape::Coord(dims) => {
            cursor.expect(TokenKind::At)?;
            parse_fixed_number_tuple(cursor, *dims as usize, "coordinate")
        }
        Shape::Dir => {
            cursor.expect(TokenKind::Caret)?;
            parse_fixed_number_tuple(cursor, 3, "direction")
        }
        Shape::Dim(dims) => parse_dim(cursor, *dims as usize),
        Shape::Range => {
            cursor.expect(TokenKind::LParen)?;
            let lo = parse_plain_number(cursor)?;
            cursor.expect(TokenKind::DotDot)?;
            let hi = parse_plain_number(cursor)?;
            let mut items = vec![FieldValue::Float(lo), FieldValue::Float(hi)];
            if cursor.peek().kind == TokenKind::Comma {
                cursor.advance();
                items.push(FieldValue::Float(parse_plain_number(cursor)?));
            }
            cursor.expect(TokenKind::RParen)?;
            Ok(FieldValue::Tuple(items))
        }
        Shape::Expr => {
            cursor.expect(TokenKind::LParen)?;
            let value = parse_expr(cursor, 0)?;
            cursor.expect(TokenKind::RParen)?;
            Ok(FieldValue::Expr(value))
        }
        Shape::Tuple(elem, len) => {
            let mut items = Vec::new();
            loop {
                items.push(parse_shape(cursor, elem, depth + 1)?);
                if cursor.peek().kind == TokenKind::Comma {
                    cursor.advance();
                    continue;
                }
                break;
            }
            if let Some(expected_len) = len {
                if items.len() != *expected_len {
                    return Err(TextError::new(format!("tuple expects {} elements, found {}", expected_len, items.len()), cursor.span()));
                }
            }
            Ok(FieldValue::Tuple(items))
        }
        Shape::List(elem) => {
            cursor.expect(TokenKind::LBracket)?;
            let mut items = Vec::new();
            while cursor.peek().kind != TokenKind::RBracket {
                let pos_before = cursor.pos;
                items.push(parse_shape(cursor, elem, depth + 1)?);
                // A bare `Shape::Record` element (no keyword, no brackets of its own — e.g. a
                // list of `key=value` port records) can legitimately parse to an empty record
                // consuming zero tokens once its remaining keys stop matching whatever comes
                // next: an unrecognized key (typo, wrong field name) looks identical to "this
                // record legitimately ended" from `parse_record_body`'s point of view. Detecting
                // it here — the one place with cursor before/after to compare — turns what would
                // otherwise be an infinite zero-progress loop (silently bottoming out at the
                // `check_nodes` safety limit, far from the actual offending token) into an
                // immediate, correctly-spanned parse error.
                if cursor.pos == pos_before {
                    return Err(TextError::new(format!("list element made no progress at {:?} '{}' — likely an unrecognized field key", cursor.peek().kind, cursor.peek().text.as_str()), cursor.span()));
                }
                cursor.limits.check_nodes(items.len(), cursor.span())?;
            }
            cursor.expect(TokenKind::RBracket)?;
            Ok(FieldValue::List(items))
        }
        Shape::Record(spec_fn) => Ok(FieldValue::Record(parse_record_body(cursor, &spec_fn(), depth + 1)?)),
        Shape::Block(inner) => {
            cursor.expect(TokenKind::LBrace)?;
            let value = parse_shape(cursor, inner, depth + 1)?;
            cursor.expect(TokenKind::RBrace)?;
            Ok(FieldValue::Block(Box::new(value)))
        }
        Shape::Statements(variants) => {
            let mut out = Vec::new();
            while let Some(keyword) = current_keyword(cursor) {
                let Some((_, spec_fn)) = variants.iter().find(|(kw, _)| kw == &keyword) else { break };
                // `parse_record_body` consumes the keyword itself (see its own check below); we
                // only peek here to decide whether this token starts a known variant at all.
                let record = parse_record_body(cursor, &spec_fn(), depth + 1)?;
                out.push((keyword, record));
                cursor.limits.check_nodes(out.len(), cursor.span())?;
                if cursor.peek().kind == TokenKind::RBrace || cursor.peek().kind == TokenKind::Eof {
                    break;
                }
            }
            Ok(FieldValue::Statements(out))
        }
        Shape::Map(inner) => {
            cursor.expect(TokenKind::LBrace)?;
            let mut entries = Vec::new();
            while let Some(key) = cursor.at_attr_key() {
                cursor.advance();
                cursor.expect(TokenKind::Equals)?;
                let value = parse_shape(cursor, inner, depth + 1)?;
                entries.push((key, value));
            }
            cursor.expect(TokenKind::RBrace)?;
            Ok(FieldValue::Map(entries))
        }
        Shape::Value => Ok(FieldValue::Value(parse_dsl_value(cursor, depth + 1)?)),
        // Reached whenever a `Table` shape is parsed via the generic `key=` dispatch (the
        // AoS-verbose alternate input, `name=[ {row} {row} ... ]`) or nested inside another shape
        // (a table row's own column, a list element). Delegates to `parse_table_list`, NOT to
        // plain `List(Record)`: a table row type is commonly declared with no keyword of its own
        // (a header already gives every row its column order, so SoA rows don't need one), and a
        // bare `Shape::Record` with no keyword and no brace has nothing marking where one row's
        // fields end and the next row's begin — the exact ambiguity `parse_table_cell` guards
        // against for table COLUMNS applies identically to table ROWS printed as a bare list. The
        // bare SoA form (`name [col:TYPE ...] { rows }`) is recognized earlier, in
        // `parse_record_body`, and calls `parse_table_soa` directly since its grammar (a header,
        // then count-delimited rows) isn't reachable through `parse_shape` at all.
        Shape::Table(spec_fn) => {
            validate_table_columns(&spec_fn())?;
            parse_table_list(cursor, *spec_fn, depth)
        }
        Shape::Wire => Ok(FieldValue::Wire(parse_wire(cursor)?)),
    }
}

fn current_keyword(cursor: &Cursor) -> Option<String> {
    if cursor.peek().kind == TokenKind::Ident && cursor.at_attr_key().is_none() {
        Some(cursor.peek().text.as_str().to_string())
    } else {
        None
    }
}

fn parse_dsl_value(cursor: &mut Cursor, depth: usize) -> Result<DslValue, TextError> {
    cursor.limits.check_depth(depth, cursor.span())?;
    match cursor.peek().kind {
        TokenKind::LBrace => {
            cursor.advance();
            let mut entries = Vec::new();
            while let Some(key) = cursor.at_attr_key() {
                cursor.advance();
                cursor.expect(TokenKind::Equals)?;
                entries.push((key, parse_dsl_value(cursor, depth + 1)?));
            }
            cursor.expect(TokenKind::RBrace)?;
            Ok(DslValue::Object(entries))
        }
        TokenKind::LBracket => {
            cursor.advance();
            let mut items = Vec::new();
            while cursor.peek().kind != TokenKind::RBracket {
                items.push(parse_dsl_value(cursor, depth + 1)?);
            }
            cursor.expect(TokenKind::RBracket)?;
            Ok(DslValue::Array(items))
        }
        TokenKind::Text => {
            let token = cursor.advance();
            let text = dsl_core::unescape_text(&token.text.as_str(), false).map_err(|e| TextError::new(e, token.span))?;
            Ok(DslValue::String(text))
        }
        TokenKind::Int | TokenKind::Float => {
            let token = cursor.advance();
            let value = parse_f64(&token.text.as_str()).map_err(|e| TextError::new(e, token.span))?;
            Ok(DslValue::Number(value))
        }
        TokenKind::Ident => {
            let token = cursor.advance();
            match token.text.as_str().as_ref() {
                "null" => Ok(DslValue::Null),
                "true" => Ok(DslValue::Bool(true)),
                "false" => Ok(DslValue::Bool(false)),
                other => Err(TextError::new(format!("expected a value literal, found ident '{other}'"), token.span)),
            }
        }
        other => Err(TextError::new(format!("expected a value literal, found {other:?}"), cursor.span())),
    }
}

/// @emoji 🕸️ Parses one wire literal. `<-` is accepted sugar only: normalized here by swapping
/// the two endpoints, so the stored `WireValue` (and everything reprinted from it) only ever
/// holds `->`/`--` — `b<-a` and `a->b` parse to the identical value and print identically.
fn parse_wire(cursor: &mut Cursor) -> Result<WireValue, TextError> {
    let mut from = parse_wire_node(cursor)?;
    let edge = if matches!(cursor.peek().kind, TokenKind::Arrow | TokenKind::DashArrow | TokenKind::BackArrow) {
        let arrow_kind = cursor.advance().kind;
        let to = parse_wire_node(cursor)?;
        match arrow_kind {
            TokenKind::BackArrow => {
                let swapped_to = std::mem::replace(&mut from, to);
                Some((true, swapped_to))
            }
            other => Some((other == TokenKind::Arrow, to)),
        }
    } else {
        None
    };
    let properties = if cursor.peek().kind == TokenKind::LBrace { parse_dsl_value(cursor, 0)? } else { DslValue::Object(Vec::new()) };
    Ok(WireValue { from, edge, properties })
}

/// @emoji 🔌️ Small public entry point other crates (the graph wire module, trinity) can call
/// directly to lex + parse one standalone wire literal, without needing a `RecordSpec` around it.
pub fn parse_wire_text(text: &str) -> Result<WireValue, TextError> {
    let limits = Limits::default();
    let tokens = lex(text, &limits, false)?;
    let mut cursor = Cursor::new(tokens, limits);
    parse_wire(&mut cursor)
}

fn parse_wire_node(cursor: &mut Cursor) -> Result<WireNode, TextError> {
    let id = ident_like_text(&cursor.expect(TokenKind::Ident)?);
    let kind = if cursor.peek().kind == TokenKind::Colon {
        cursor.advance();
        Some(ident_like_text(&cursor.expect(TokenKind::Ident)?))
    } else {
        None
    };
    let port = if cursor.peek().kind == TokenKind::At {
        cursor.advance();
        Some(ident_like_text(&cursor.expect(TokenKind::Ident)?))
    } else {
        None
    };
    Ok(WireNode { id, kind, port })
}

/// @emoji 🧾️ Parses one record: its own leading keyword if `spec.keyword` declares one (the
/// `Statements` dispatcher only peeks to choose a variant — consuming it is always this
/// function's job, so a spec is self-contained regardless of whether it's reached via `parse`
/// directly, `Shape::Record`, or a `Statements` variant), positional fields in declaration order,
/// then order-independent `key=value` attributes (LL(2): an `Ident` followed by `=` is always a
/// key), until a token that is neither a known key nor an unfilled positional slot — which ends
/// the record (it belongs to whatever comes next: a new statement, a closing brace, or EOF).
fn parse_record_body(cursor: &mut Cursor, spec: &RecordSpec, depth: usize) -> Result<RecordValue, TextError> {
    cursor.limits.check_depth(depth, cursor.span())?;
    if let Some(keyword) = &spec.keyword {
        if cursor.at_keyword(keyword) {
            cursor.advance();
        } else {
            return Err(TextError::new(format!("expected keyword '{keyword}', found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()), cursor.span()));
        }
    }
    let mut record = RecordValue::default();
    let positional: Vec<&FieldSpec> = {
        let mut p: Vec<&FieldSpec> = spec.fields.iter().filter(|f| f.position.is_some()).collect();
        p.sort_by_key(|f| f.position.unwrap());
        p
    };
    for field in &positional {
        if field.optional {
            // An explicit `_` placeholder always means "absent, but consume the slot" — this is
            // what keeps LATER positionals aligned when an earlier optional one is skipped (see
            // `print_record`'s matching print-side logic). Only positional contexts ever see a
            // `Placeholder` token; keyed optionals are simply omitted instead.
            if cursor.peek().kind == TokenKind::Placeholder {
                cursor.advance();
                record.fields.insert(field.id, FieldValue::Absent);
                continue;
            }
            if !can_start_positional(cursor, &field.shape) {
                record.fields.insert(field.id, FieldValue::Absent);
                continue;
            }
        }
        let value = parse_shape(cursor, &field.shape, depth + 1)?;
        record.fields.insert(field.id, value);
    }

    // `Statements` fields have no field-level key at all — they're recognized purely by matching
    // one of their own variants' keywords, so at most one such field may appear per record.
    // `Block` fields are also excluded from the `key=value` loop below: their own key acts as a
    // bare leading keyword (`children { ... }`, no `=`) — `Table` fields (bare `key [...] {...}`
    // SoA form) are handled the same way, via their own lookahead branch below.
    let statements_field = spec.fields.iter().find(|f| f.position.is_none() && matches!(f.shape, Shape::Statements(_)));
    let mut keyed: Vec<&FieldSpec> = spec.fields.iter().filter(|f| f.position.is_none() && !f.key.is_empty() && !matches!(f.shape, Shape::Statements(_))).collect();

    loop {
        if let Some(key) = cursor.at_attr_key() {
            let Some(index) = keyed.iter().position(|f| !matches!(f.shape, Shape::Block(_)) && f.key == key) else { break };
            let field = keyed.remove(index);
            cursor.advance();
            cursor.expect(TokenKind::Equals)?;
            let value = parse_shape(cursor, &field.shape, depth + 1)?;
            record.fields.insert(field.id, value);
            continue;
        }
        // `Table`'s bare SoA form: the keyword directly followed by `[` (no `=`) — distinct from
        // the AoS-verbose `key=[...]` form already handled by the `at_attr_key` branch above.
        if let Some(index) = keyed.iter().position(|f| matches!(f.shape, Shape::Table(_)) && cursor.at_keyword(&f.key) && cursor.peek_at(1).kind == TokenKind::LBracket) {
            let field = keyed.remove(index);
            let Shape::Table(spec_fn) = &field.shape else { unreachable!() };
            let spec_fn = *spec_fn;
            cursor.advance();
            let value = parse_table_soa(cursor, spec_fn, depth + 1)?;
            record.fields.insert(field.id, value);
            continue;
        }
        let Some(index) = keyed.iter().position(|f| matches!(f.shape, Shape::Block(_)) && cursor.at_keyword(&f.key)) else { break };
        let field = keyed.remove(index);
        cursor.advance();
        let value = parse_shape(cursor, &field.shape, depth + 1)?;
        record.fields.insert(field.id, value);
    }
    for field in keyed {
        record.fields.entry(field.id).or_insert(FieldValue::Absent);
    }

    if let Some(field) = statements_field {
        let value = parse_shape(cursor, &field.shape, depth + 1)?;
        record.fields.insert(field.id, value);
    }

    Ok(record)
}

fn can_start_positional(cursor: &Cursor, shape: &Shape) -> bool {
    match shape {
        Shape::Bool | Shape::Enum(_) => cursor.peek().kind == TokenKind::Ident,
        Shape::Int | Shape::UInt => cursor.peek().kind == TokenKind::Int,
        Shape::Float | Shape::Quantity(_) | Shape::Angle(_) | Shape::Dim(_) => matches!(cursor.peek().kind, TokenKind::Float | TokenKind::Int),
        Shape::Ref(_) => matches!(cursor.peek().kind, TokenKind::Text | TokenKind::Placeholder),
        Shape::Count => cursor.peek().kind == TokenKind::Ident,
        Shape::Coord(_) => cursor.peek().kind == TokenKind::At,
        Shape::Dir => cursor.peek().kind == TokenKind::Caret,
        Shape::Range | Shape::Expr => cursor.peek().kind == TokenKind::LParen,
        Shape::Embed(_) => matches!(cursor.peek().kind, TokenKind::Fence | TokenKind::Text | TokenKind::Placeholder),
        // Only `Text|Placeholder` — NOT bare `Ident` — may start an optional positional `Text`
        // field: an unquoted bare-ident value here would be indistinguishable from the next
        // statement's leading keyword, so this deliberately narrower check (versus `Shape::Text`
        // parsing `Ident|Text` everywhere else) resolves that ambiguity.
        Shape::Text => matches!(cursor.peek().kind, TokenKind::Text | TokenKind::Placeholder),
        Shape::Bytes64 => cursor.peek().kind == TokenKind::Text,
        Shape::List(_) => cursor.peek().kind == TokenKind::LBracket,
        Shape::Block(_) | Shape::Map(_) => cursor.peek().kind == TokenKind::LBrace,
        _ => true,
    }
}

//#region 🔖️Table
/// @emoji 🚧️ Which shapes have a fixed/bounded token extent and may therefore be a `Table`
/// column: an unbounded `Tuple` (`len: None`, comma-separated until... forever) and `Statements`
/// (repeats until a non-matching keyword) both need an external delimiter to know where they end
/// — fine inside `[ ]`/`{ }` brackets, fatal inside a table row where the ONLY thing marking a
/// row boundary is "we've now read exactly `columns.len()` values".
fn shape_is_self_delimiting(shape: &Shape) -> bool {
    !matches!(shape, Shape::Statements(_) | Shape::Tuple(_, None))
}

/// @emoji 🚧️ Spec-build-time validation for a `Table`'s element `RecordSpec` — called wherever a
/// `Shape::Table(spec_fn)` is first evaluated (both parse paths, and printing), since `spec_fn` is
/// a lazy pointer rather than an eagerly-built value there is no earlier moment to check it at.
fn validate_table_columns(spec: &RecordSpec) -> Result<(), TextError> {
    for field in &spec.fields {
        if !shape_is_self_delimiting(&field.shape) {
            return Err(TextError::new(
                format!("table column '{}' has a non-self-delimiting shape ({}) and cannot be a table column", field.key, shape_type_name(&field.shape)),
                TextSpan::at(1, 1),
            ));
        }
    }
    Ok(())
}

/// @emoji 🏷️ UPPERCASE schema type tag for a `Shape` — what a `Table` header prints per column
/// (`id:TEXT`), per the unified syntax law (`UPPERCASE` for engine shapes, `PascalCase` reserved
/// for technology-declared domain kinds).
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
        Shape::Embed(_) => "EMBED",
    }
}

/// @emoji 📊️ Parses the bare SoA form of a `Table` field: `[col:TYPE ...] { v11 v12 ...  v21 v22
/// ... }`, cursor positioned right after the field's own keyword has already been consumed. The
/// header names columns (in the order values then appear per row); a `:TYPE` suffix is accepted
/// but not required to resolve a column (it's a human/printer-facing tag, not load-bearing for
/// parsing — the column's real shape always comes from the element `RecordSpec`), which is what
/// lets a hand-written header omit types the engine can already infer. Rows have NO separator —
/// reading exactly `columns.len()` values per row is what makes a row self-delimiting, which is
/// also why every column shape must itself be self-delimiting (`validate_table_columns`).
fn parse_table_soa(cursor: &mut Cursor, spec_fn: fn() -> RecordSpec, depth: usize) -> Result<FieldValue, TextError> {
    let element_spec = spec_fn();
    validate_table_columns(&element_spec)?;
    cursor.expect(TokenKind::LBracket)?;
    let mut columns: Vec<&FieldSpec> = Vec::new();
    while cursor.peek().kind != TokenKind::RBracket {
        let key_token = cursor.expect(TokenKind::Ident)?;
        let key = key_token.text.as_str().to_string();
        if cursor.peek().kind == TokenKind::Colon {
            cursor.advance();
            cursor.expect(TokenKind::Ident)?; // type tag — documentation only, not re-validated here
        }
        let field_spec = element_spec.fields.iter().find(|f| f.key == key).ok_or_else(|| TextError::new(format!("unknown table column '{key}'"), key_token.span))?;
        columns.push(field_spec);
    }
    cursor.expect(TokenKind::RBracket)?;
    cursor.expect(TokenKind::LBrace)?;
    let mut rows = Vec::new();
    while cursor.peek().kind != TokenKind::RBrace {
        let mut record = RecordValue::default();
        for field_spec in &columns {
            if cursor.peek().kind == TokenKind::Placeholder {
                cursor.advance();
                record.fields.insert(field_spec.id, FieldValue::Absent);
                continue;
            }
            let value = parse_table_cell(cursor, &field_spec.shape, depth + 1)?;
            record.fields.insert(field_spec.id, value);
        }
        for field_spec in &element_spec.fields {
            record.fields.entry(field_spec.id).or_insert(FieldValue::Absent);
        }
        rows.push(FieldValue::Record(record));
        cursor.limits.check_nodes(rows.len(), cursor.span())?;
    }
    cursor.expect(TokenKind::RBrace)?;
    Ok(FieldValue::List(rows))
}

/// @emoji 🧱️ Reads one table cell's value. Every table-safe shape is bounded by its own bracket or
/// a fixed token count (`validate_table_columns`/`shape_is_self_delimiting`) — EXCEPT a bare
/// `Shape::Record` column, which prints as a flat run of `key=value` tokens with no bracket of its
/// own (a table row has no `field=` prefix to give it one, unlike a Record-shaped field elsewhere).
/// Two adjacent columns of the SAME record type (or any two types sharing a field name) are then
/// genuinely ambiguous: `parse_record_body`'s keyed loop for column N keeps matching `key=value`
/// tokens for as long as the key is one of ITS OWN not-yet-filled fields, so a column-N field left
/// absent (never printed) silently lets column N's parse run on and swallow column N+1's
/// same-named token instead of stopping at the column boundary. Braced here for exactly that
/// reason — every other shape already round-trips through the ordinary `parse_shape`.
fn parse_table_cell(cursor: &mut Cursor, shape: &Shape, depth: usize) -> Result<FieldValue, TextError> {
    if let Shape::Record(spec_fn) = shape {
        cursor.expect(TokenKind::LBrace)?;
        let record = parse_record_body(cursor, &spec_fn(), depth + 1)?;
        cursor.expect(TokenKind::RBrace)?;
        return Ok(FieldValue::Record(record));
    }
    parse_shape(cursor, shape, depth)
}

/// @emoji 📋️ The AoS-list form for a `Table` value reached anywhere other than a record's own
/// leading keyword-prefixed field: `[ {row-fields} {row-fields} ... ]`. Each row is brace-wrapped
/// for the same reason a `Shape::Record` table COLUMN is (`parse_table_cell` above) — a table row
/// type is commonly declared with no keyword of its own (a header already gives every row its
/// column order, so SoA rows don't need one), so without a bracket of its own, one row's absent
/// field could let its parse run on into the next row's same-named token exactly like the
/// column-vs-column case. Bracing every row here removes that ambiguity regardless of whether the
/// row type happens to declare a keyword or not.
fn parse_table_list(cursor: &mut Cursor, spec_fn: fn() -> RecordSpec, depth: usize) -> Result<FieldValue, TextError> {
    cursor.expect(TokenKind::LBracket)?;
    let mut items = Vec::new();
    while cursor.peek().kind != TokenKind::RBracket {
        cursor.expect(TokenKind::LBrace)?;
        let record = parse_record_body(cursor, &spec_fn(), depth + 1)?;
        cursor.expect(TokenKind::RBrace)?;
        items.push(FieldValue::Record(record));
        cursor.limits.check_nodes(items.len(), cursor.span())?;
    }
    cursor.expect(TokenKind::RBracket)?;
    Ok(FieldValue::List(items))
}

/// @emoji 📋️ Prints the braced AoS-list form `parse_table_list` reads back. Ordinary `[ ]` spacing
/// (a space just inside, per the general list rule — NOT the header's own tight-glued exception).
fn print_table_list(spec_fn: fn() -> RecordSpec, items: &[FieldValue], writer: &mut Writer) {
    writer.atom("[");
    for item in items {
        let FieldValue::Record(record) = item else { continue };
        writer.atom("{");
        writer.glue();
        print_record(record, &spec_fn(), writer);
        writer.glue();
        writer.atom("}");
    }
    writer.atom("]");
}
//#endregion 🔖️Table

fn base64_decode(text: &str) -> Result<Vec<u8>, String> {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut lut = [255u8; 256];
    for (i, &c) in ALPHABET.iter().enumerate() {
        lut[c as usize] = i as u8;
    }
    let clean: Vec<u8> = text.bytes().filter(|b| *b != b'=').collect();
    let mut out = Vec::with_capacity(clean.len() * 3 / 4);
    let mut buffer = 0u32;
    let mut bits = 0u32;
    for b in clean {
        let value = lut[b as usize];
        if value == 255 {
            return Err(format!("invalid base64 byte '{}'", b as char));
        }
        buffer = (buffer << 6) | value as u32;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buffer >> bits) as u8);
        }
    }
    Ok(out)
}

fn base64_encode(bytes: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = *chunk.get(1).unwrap_or(&0);
        let b2 = *chunk.get(2).unwrap_or(&0);
        out.push(ALPHABET[(b0 >> 2) as usize] as char);
        out.push(ALPHABET[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        out.push(if chunk.len() > 1 { ALPHABET[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { ALPHABET[(b2 & 0x3f) as usize] as char } else { '=' });
    }
    out
}
//#endregion 🔖️Parser

//#region 🔖️Writer
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinMode {
    Document,
    Inline,
}

/// @emoji ✍️ A chunk tree that renders in either join mode — the structural half of the newline
/// law. `atom` asserts its argument contains no raw `\n` (Document mode still separates atoms with
/// synthesized whitespace, never embeds one inside an atom), so `render(Inline)` joining every
/// chunk with a single space can never produce an embedded newline.
///
/// @emoji 📏️ Canonical spacing rules (both join modes, structurally guaranteed — never hand-tuned
/// per callsite): never a space adjacent to `=` (`key=[ a b ]`, not `key= [ a b ]` — the printer
/// achieves this by pushing a bare `key=` atom, calling [`Writer::glue`], then printing the
/// value); exactly one space between sibling atoms; exactly one space just inside `[ ]`/`{ }` when
/// rendered inline (`[ a b ]`, not `[a b]`) — EXCEPT a `Table` header's `[ ]`, which is glued
/// tight on both sides (`[id:TEXT x:NUM]`) since it's a fixed one-shot header, not a
/// space-joined element list; a space appears before a keyword-led block's `{` (`children {
/// ... }`) but never before a glued composite's `{` (`data={ ... }`).
pub struct Writer {
    chunks: Vec<Chunk>,
    indent: usize,
}

enum Chunk {
    Atom(String),
    OpenBlock,
    CloseBlock,
    NewRecord,
    /// @emoji 🧲️ One-shot marker: the very next `Atom`/`OpenBlock` chunk renders with NO
    /// preceding separator (space in Inline mode, space-or-newline-continuation in Document mode)
    /// — consumed by that one chunk, then normal spacing resumes. See [`Writer::glue`].
    Glue,
    /// @emoji 📜️ `Shape::Embed`'s payload — the one chunk kind whose Document and Inline renders
    /// genuinely differ in FORM (fenced block vs. escaped quoted string), not just spacing.
    Verbatim { lang: String, content: String },
}

impl Default for Writer {
    fn default() -> Self {
        Self::new()
    }
}

impl Writer {
    pub fn new() -> Self {
        Self { chunks: Vec::new(), indent: 0 }
    }

    pub fn atom(&mut self, s: impl AsRef<str>) {
        let s = s.as_ref();
        debug_assert!(!s.contains('\n'), "Writer::atom must not contain a raw newline: {s:?}");
        self.chunks.push(Chunk::Atom(s.to_string()));
    }

    pub fn key_value(&mut self, key: &str, value: impl AsRef<str>) {
        self.atom(format!("{key}={}", value.as_ref()));
    }

    pub fn open_block(&mut self) {
        self.chunks.push(Chunk::OpenBlock);
        self.indent += 1;
    }

    pub fn close_block(&mut self) {
        self.indent = self.indent.saturating_sub(1);
        self.chunks.push(Chunk::CloseBlock);
    }

    pub fn new_record(&mut self) {
        self.chunks.push(Chunk::NewRecord);
    }

    /// @emoji 🧲️ Fuses the next pushed chunk onto whatever precedes it, with no separator, in
    /// BOTH join modes — the mechanism behind every `key=value`/`key=[...]`/`key={...}` fusion in
    /// this printer. Replaces the old approach of mutating an already-pushed atom's string in
    /// place (which only worked for single-atom scalar values): `glue()` composes with arbitrarily
    /// structured values (nested blocks, lists, whole sub-records) since it's a rendering-time
    /// join, not a string-splice.
    pub fn glue(&mut self) {
        self.chunks.push(Chunk::Glue);
    }

    /// @emoji 📜️ Pushes a `Shape::Embed` payload — content MAY contain raw newlines (unlike
    /// [`Self::atom`], which forbids them), since Document mode renders it as a fence.
    pub fn verbatim(&mut self, lang: &str, content: &str) {
        self.chunks.push(Chunk::Verbatim { lang: lang.to_string(), content: content.to_string() });
    }

    pub fn render(&self, mode: JoinMode) -> String {
        match mode {
            JoinMode::Inline => {
                let mut parts: Vec<String> = Vec::new();
                let mut glued = false;
                let mut push = |piece: String, glued: &mut bool| {
                    if *glued {
                        if let Some(last) = parts.last_mut() {
                            last.push_str(&piece);
                        } else {
                            parts.push(piece);
                        }
                    } else {
                        parts.push(piece);
                    }
                    *glued = false;
                };
                for chunk in &self.chunks {
                    match chunk {
                        Chunk::Glue => glued = true,
                        Chunk::Atom(s) => push(s.clone(), &mut glued),
                        Chunk::OpenBlock => push("{".to_string(), &mut glued),
                        Chunk::CloseBlock => push("}".to_string(), &mut glued),
                        Chunk::NewRecord => {}
                        Chunk::Verbatim { content, .. } => push(format!("\"{}\"", dsl_core::escape_text(content)), &mut glued),
                    }
                }
                parts.join(" ")
            }
            JoinMode::Document => {
                let mut out = String::new();
                let mut indent = 0usize;
                let mut line_open = false;
                let mut glued = false;
                let push_indent = |out: &mut String, indent: usize| {
                    for _ in 0..indent {
                        out.push_str("  ");
                    }
                };
                for chunk in &self.chunks {
                    match chunk {
                        Chunk::Glue => glued = true,
                        Chunk::Atom(s) => {
                            if !line_open {
                                push_indent(&mut out, indent);
                                line_open = true;
                            } else if !glued {
                                out.push(' ');
                            }
                            out.push_str(s);
                            glued = false;
                        }
                        Chunk::OpenBlock => {
                            if glued {
                                out.push('{');
                            } else {
                                out.push_str(" {");
                            }
                            out.push('\n');
                            line_open = false;
                            indent += 1;
                            glued = false;
                        }
                        Chunk::CloseBlock => {
                            if line_open {
                                out.push('\n');
                                line_open = false;
                            }
                            indent = indent.saturating_sub(1);
                            push_indent(&mut out, indent);
                            out.push('}');
                            out.push('\n');
                        }
                        Chunk::NewRecord => {
                            if line_open {
                                out.push('\n');
                                line_open = false;
                            }
                        }
                        Chunk::Verbatim { lang, content } => {
                            if !line_open {
                                push_indent(&mut out, indent);
                            } else if !glued {
                                out.push(' ');
                            }
                            out.push_str("```");
                            out.push_str(lang);
                            out.push('\n');
                            out.push_str(content);
                            if !content.is_empty() {
                                out.push('\n');
                            }
                            out.push_str("```");
                            line_open = true;
                            glued = false;
                        }
                    }
                }
                if line_open {
                    out.push('\n');
                }
                out
            }
        }
    }
}

/// @emoji 🥇️ Field print order within one record — NOT declaration order: keyword, then
/// positionals (unchanged), then keyed fields grouped scalar-before-composite-before-table-
/// before-statements, ties broken by original declaration order (a stable sort over an
/// already-declaration-order slice achieves this for free). Metadata/scalars land before large
/// nested/tabular blocks, which is friendlier to lazy loading/streaming readers — parsing stays
/// completely order-independent, so this is a print-only change.
fn keyed_field_rank(shape: &Shape) -> u8 {
    match shape {
        Shape::Bool
        | Shape::Int
        | Shape::UInt
        | Shape::Float
        | Shape::Text
        | Shape::Bytes64
        | Shape::Enum(_)
        | Shape::Tuple(_, _)
        | Shape::Quantity(_)
        | Shape::Angle(_)
        | Shape::Ref(_)
        | Shape::Coord(_)
        | Shape::Dir
        | Shape::Dim(_)
        | Shape::Range
        | Shape::Count
        | Shape::Expr => 0,
        Shape::List(_) | Shape::Map(_) | Shape::Record(_) | Shape::Block(_) | Shape::Value | Shape::Wire => 1,
        Shape::Table(_) => 2,
        Shape::Statements(_) => 3,
        // Ranks LAST of all: a multi-line fence dwarfs everything else in a record, so it should
        // print after every scalar/composite/table field, not interleaved among them.
        Shape::Embed(_) => 4,
    }
}

pub fn print_record(value: &RecordValue, spec: &RecordSpec, writer: &mut Writer) {
    if let Some(keyword) = &spec.keyword {
        writer.atom(keyword);
    }
    let mut positional: Vec<&FieldSpec> = spec.fields.iter().filter(|f| f.position.is_some()).collect();
    positional.sort_by_key(|f| f.position.unwrap());
    for (index, field) in positional.iter().enumerate() {
        match value.get(field.id) {
            Some(fv) if !matches!(fv, FieldValue::Absent) => print_shape(fv, &field.shape, writer),
            _ => {
                // An absent OPTIONAL positional prints as `_` only if some LATER positional in
                // this same record is actually present — that's what keeps slots aligned for the
                // reader (and reparse). A run of trailing absents needs no placeholder at all.
                let later_present = positional[index + 1..].iter().any(|f| matches!(value.get(f.id), Some(fv) if !matches!(fv, FieldValue::Absent)));
                if later_present {
                    writer.atom("_");
                }
            }
        }
    }

    let mut keyed: Vec<&FieldSpec> = spec.fields.iter().filter(|f| f.position.is_none() && !f.key.is_empty()).collect();
    keyed.sort_by_key(|f| keyed_field_rank(&f.shape));
    for field in keyed {
        match value.get(field.id) {
            Some(FieldValue::Absent) | None => continue,
            Some(fv) => match &field.shape {
                // `Statements` items each carry their own leading keyword — no field-level key at
                // all is ever printed for this shape.
                Shape::Statements(_) => print_shape(fv, &field.shape, writer),
                // `Block`'s own key is a bare leading keyword, not a `key=value` attribute
                // (`children { ... }`, never `children={...}`).
                Shape::Block(_) => {
                    writer.new_record();
                    writer.atom(&field.key);
                    print_shape(fv, &field.shape, writer);
                }
                // `Table`'s own key is likewise a bare leading keyword, but — unlike `Block` —
                // it must always go through the dedicated SoA writer (`print_table`), never the
                // generic `print_shape` dispatch: that dispatch renders `Table` as the bracketed
                // AoS list (see its `Shape::Table` arm below) so a `Table` value reached any OTHER
                // way (nested inside a table row, a list, ...) stays self-delimiting. Only here,
                // directly after a record's own leading keyword, is the bare `[col:TYPE ...]
                // {rows}` form reachable on the parse side (`parse_record_body`'s dedicated
                // bare-SoA lookahead) — printing it via `print_shape` here would silently regress
                // to the AoS form for every top-level table field.
                Shape::Table(spec_fn) => {
                    writer.new_record();
                    writer.atom(&field.key);
                    if let FieldValue::List(items) = fv {
                        print_table(*spec_fn, items, writer);
                    }
                }
                _ => {
                    writer.atom(format!("{}=", field.key));
                    print_key_value(field, fv, writer);
                }
            },
        }
    }
}

/// @emoji 🧲️ `key=` was just pushed by the caller — glue the value onto it with no separator,
/// then print it normally (composed, not string-spliced, so this handles arbitrarily structured
/// values exactly like a bare `print_shape` call would).
fn print_key_value(field: &FieldSpec, value: &FieldValue, writer: &mut Writer) {
    writer.glue();
    match (&field.shape, value) {
        (Shape::Enum(variants), FieldValue::Enum(ordinal)) => {
            if let Some((tag, _)) = variants.iter().find(|(_, o)| o == ordinal) {
                writer.atom(tag);
            }
        }
        _ => print_shape(value, &field.shape, writer),
    }
}

fn scalar_to_text(value: &FieldValue) -> String {
    match value {
        FieldValue::Bool(b) => b.to_string(),
        FieldValue::Int(i) => i.to_string(),
        FieldValue::UInt(u) => u.to_string(),
        FieldValue::Float(f) => format_f64(*f),
        // Bare (unquoted) whenever the text lexes back as exactly this one ident — the printer's
        // half of the "strings bare-preferred" law; `is_bare_ident` also excludes reserved literal
        // idents (`_`/`true`/`false`/`null`/`nan`/`inf`) and number-shaped text, which always fall
        // through to the quoted+escaped form instead.
        FieldValue::Text(s) => {
            if dsl_core::is_bare_ident(s) {
                s.clone()
            } else {
                format!("\"{}\"", dsl_core::escape_text(s))
            }
        }
        FieldValue::Bytes64(bytes) => format!("\"{}\"", base64_encode(bytes)),
        FieldValue::Enum(_) => String::new(), // resolved by caller via variants table when needed
        _ => String::new(),
    }
}

/// @emoji 🔢️ Renders one `FieldValue::Tuple` element as bare text for the `Coord`/`Dir`/`Dim`/
/// `Range` printers above — every element of those tuples is always `FieldValue::Float` by
/// construction (their parsers only ever push `FieldValue::Float`), so this panics rather than
/// falling back on a malformed value, matching the rest of this module's "trust the parser built
/// this" convention for shapes whose `FieldValue` invariant is enforced entirely at parse time.
fn number_tuple_component(value: &FieldValue) -> String {
    match value {
        FieldValue::Float(v) => format_f64(*v),
        other => panic!("Coord/Dir/Dim/Range tuple element must be Float, found {other:?}"),
    }
}

pub fn print_shape(value: &FieldValue, shape: &Shape, writer: &mut Writer) {
    match (value, shape) {
        // Must precede the generic scalar arm below: that arm's shape pattern is `_` and would
        // otherwise swallow every `FieldValue::Float` regardless of shape, printing a bare number
        // with no unit suffix even for a `Quantity`/`Angle` field.
        (FieldValue::Float(v), Shape::Quantity(unit) | Shape::Angle(unit)) => {
            writer.atom(format!("{}{}", format_f64(*v), unit.symbol));
        }
        (FieldValue::UInt(v), Shape::Count) => {
            writer.atom(format!("x{v}"));
        }
        (FieldValue::Tuple(items), Shape::Coord(_)) => {
            writer.atom(format!("@{}", items.iter().map(number_tuple_component).collect::<Vec<_>>().join(",")));
        }
        (FieldValue::Tuple(items), Shape::Dir) => {
            writer.atom(format!("^{}", items.iter().map(number_tuple_component).collect::<Vec<_>>().join(",")));
        }
        (FieldValue::Tuple(items), Shape::Dim(_)) => {
            writer.atom(items.iter().map(number_tuple_component).collect::<Vec<_>>().join("x"));
        }
        (FieldValue::Tuple(items), Shape::Range) => {
            let parts: Vec<String> = items.iter().map(number_tuple_component).collect();
            let body = match parts.as_slice() {
                [lo, hi] => format!("{lo}..{hi}"),
                [lo, hi, step] => format!("{lo}..{hi},{step}"),
                _ => parts.join(","),
            };
            writer.atom(format!("({body})"));
        }
        (FieldValue::Expr(expr), Shape::Expr) => {
            writer.atom(format!("({})", print_expr(expr)));
        }
        (FieldValue::Text(content), Shape::Embed(lang)) => {
            writer.verbatim(lang, content);
        }
        (FieldValue::Bool(_) | FieldValue::Int(_) | FieldValue::UInt(_) | FieldValue::Float(_) | FieldValue::Text(_) | FieldValue::Bytes64(_), _) => {
            writer.atom(scalar_to_text(value));
        }
        (FieldValue::Enum(ordinal), Shape::Enum(variants)) => {
            if let Some((tag, _)) = variants.iter().find(|(_, o)| o == ordinal) {
                writer.atom(tag);
            }
        }
        (FieldValue::Tuple(items), Shape::Tuple(elem, _)) => {
            let rendered: Vec<String> = items
                .iter()
                .map(|item| {
                    let mut sub = Writer::new();
                    print_shape(item, elem, &mut sub);
                    sub.render(JoinMode::Inline)
                })
                .collect();
            writer.atom(rendered.join(","));
        }
        // A `Table` reached here (NOT via `print_record`'s own keyed-field dispatch, which calls
        // `print_table` directly) is nested inside another shape — a table row's own column, a
        // list element, ... — where the bare `key [col:TYPE ...] {rows}` form has no bracket of
        // its own to mark where it ends. Render the braced-row AoS list instead (see
        // `print_table_list`), matching what `parse_shape`'s own `Shape::Table` arm parses in
        // every one of these same contexts.
        (FieldValue::List(items), Shape::Table(spec_fn)) => print_table_list(*spec_fn, items, writer),
        (FieldValue::List(items), Shape::List(elem)) => {
            writer.atom("[");
            for item in items {
                print_shape(item, elem, writer);
            }
            writer.atom("]");
        }
        (FieldValue::Record(record), Shape::Record(spec_fn)) => {
            print_record(record, &spec_fn(), writer);
        }
        (FieldValue::Block(inner_value), Shape::Block(inner_shape)) => {
            writer.open_block();
            print_shape(inner_value, inner_shape, writer);
            writer.close_block();
        }
        (FieldValue::Statements(items), Shape::Statements(variants)) => {
            for (keyword, record) in items {
                writer.new_record();
                if let Some((_, spec_fn)) = variants.iter().find(|(kw, _)| kw == keyword) {
                    print_record(record, &spec_fn(), writer);
                }
            }
        }
        (FieldValue::Map(entries), Shape::Map(inner)) => {
            writer.open_block();
            let mut sorted = entries.clone();
            sorted.sort_by(|a, b| a.0.cmp(&b.0));
            for (key, value) in &sorted {
                writer.atom(format!("{key}="));
                writer.glue();
                print_shape(value, inner, writer);
            }
            writer.close_block();
        }
        (FieldValue::Value(dsl_value), Shape::Value) => print_dsl_value(dsl_value, writer),
        (FieldValue::Wire(wire), Shape::Wire) => print_wire(wire, writer),
        _ => {}
    }
}

/// @emoji 📊️ Always prints the compact SoA form — this (not the parser, which still accepts the
/// verbose AoS form too) is what makes `canonicalize` migrate old AoS documents to SoA
/// automatically. Header `[ ]` is glued tight on both sides (`[id:TEXT x:NUM]`); rows have no
/// separator, one row per line in Document mode purely for readability (`new_record` is a no-op
/// in Inline mode).
fn print_table(spec_fn: fn() -> RecordSpec, items: &[FieldValue], writer: &mut Writer) {
    let element_spec = spec_fn();
    writer.atom("[");
    writer.glue();
    for field in &element_spec.fields {
        writer.atom(format!("{}:{}", field.key, shape_type_name(&field.shape)));
    }
    writer.glue();
    writer.atom("]");
    writer.open_block();
    for item in items {
        writer.new_record();
        let FieldValue::Record(record) = item else { continue };
        for field in &element_spec.fields {
            match record.get(field.id) {
                Some(fv) if !matches!(fv, FieldValue::Absent) => print_table_cell(fv, &field.shape, writer),
                _ => writer.atom("_"),
            }
        }
    }
    writer.close_block();
}

/// @emoji 🧱️ Prints one table cell's value. See `parse_table_cell` for why a bare `Shape::Record`
/// column is brace-wrapped here — `{ }` glued tight on both sides, the same technique the header's
/// own `[ ]` uses, so bracing never disturbs the "no space just inside" canonical spacing rule for
/// a one-shot wrapper — and every other shape is left to the ordinary `print_shape`, already
/// self-delimiting.
fn print_table_cell(value: &FieldValue, shape: &Shape, writer: &mut Writer) {
    if let (FieldValue::Record(record), Shape::Record(spec_fn)) = (value, shape) {
        writer.atom("{");
        writer.glue();
        print_record(record, &spec_fn(), writer);
        writer.glue();
        writer.atom("}");
        return;
    }
    print_shape(value, shape, writer);
}

fn print_dsl_value(value: &DslValue, writer: &mut Writer) {
    match value {
        DslValue::Null => writer.atom("null"),
        DslValue::Bool(b) => writer.atom(b.to_string()),
        DslValue::Number(n) => writer.atom(format_f64(*n)),
        DslValue::String(s) => writer.atom(format!("\"{}\"", dsl_core::escape_text(s))),
        DslValue::Array(items) => {
            writer.atom("[");
            for item in items {
                print_dsl_value(item, writer);
            }
            writer.atom("]");
        }
        DslValue::Object(entries) => {
            let mut sorted = entries.clone();
            sorted.sort_by(|a, b| a.0.cmp(&b.0));
            writer.open_block();
            for (key, value) in &sorted {
                writer.atom(format!("{key}="));
                writer.glue();
                print_dsl_value(value, writer);
            }
            writer.close_block();
        }
    }
}

fn print_wire(wire: &WireValue, writer: &mut Writer) {
    let node_text = |node: &WireNode| -> String {
        let mut s = node.id.clone();
        if let Some(kind) = &node.kind {
            s.push(':');
            s.push_str(kind);
        }
        if let Some(port) = &node.port {
            s.push('@');
            s.push_str(port);
        }
        s
    };
    let mut atom = node_text(&wire.from);
    if let Some((directed, to)) = &wire.edge {
        atom.push_str(if *directed { "->" } else { "--" });
        atom.push_str(&node_text(to));
    }
    writer.atom(atom);
    if !matches!(&wire.properties, DslValue::Object(entries) if entries.is_empty()) {
        print_dsl_value(&wire.properties, writer);
    }
}

/// @emoji 🔁️ Prints `value` against `spec` in the given join mode — the top-level entry point
/// `dsl_derive`-generated code calls from `DocumentDsl::print_dsl`/`OpText::print_op`.
pub fn print(value: &RecordValue, spec: &RecordSpec, mode: JoinMode) -> String {
    let mut writer = Writer::new();
    print_record(value, spec, &mut writer);
    writer.render(mode)
}
//#endregion 🔖️Writer

//#region 🔖️Canonicalize
/// @emoji ♻️ `canonicalize(canonicalize(x)) == canonicalize(x)`: reprints whatever `parse`
/// produces from `text`, which is the fixpoint every technology's `print_dsl` output must already
/// be at (the round-trip law), so this doubles as the idempotence check.
pub fn canonicalize(text: &str, spec: &RecordSpec, opts: &ParseOptions) -> Result<String, TextError> {
    let value = parse(text, spec, opts)?;
    Ok(print(&value, spec, JoinMode::Document))
}
//#endregion 🔖️Canonicalize

//#region 🔖️Serde
impl From<serde_json::Value> for DslValue {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => DslValue::Null,
            serde_json::Value::Bool(b) => DslValue::Bool(b),
            serde_json::Value::Number(n) => DslValue::Number(n.as_f64().unwrap_or(0.0)),
            serde_json::Value::String(s) => DslValue::String(s),
            serde_json::Value::Array(items) => DslValue::Array(items.into_iter().map(DslValue::from).collect()),
            serde_json::Value::Object(map) => DslValue::Object(map.into_iter().map(|(k, v)| (k, DslValue::from(v))).collect()),
        }
    }
}

impl From<DslValue> for serde_json::Value {
    fn from(value: DslValue) -> Self {
        match value {
            DslValue::Null => serde_json::Value::Null,
            DslValue::Bool(b) => serde_json::Value::Bool(b),
            DslValue::Number(n) => serde_json::json!(n),
            DslValue::String(s) => serde_json::Value::String(s),
            DslValue::Array(items) => serde_json::Value::Array(items.into_iter().map(serde_json::Value::from).collect()),
            DslValue::Object(entries) => serde_json::Value::Object(entries.into_iter().map(|(k, v)| (k, serde_json::Value::from(v))).collect()),
        }
    }
}
//#endregion 🔖️Serde

//#region 🔖️Language
/// @emoji 🎨️ Generic editor surface over any `RecordSpec` — the generalization of
/// `mathematical_graph_dsl`'s hand-rolled `LanguageService`.
pub struct LanguageService<'g> {
    pub spec: &'g RecordSpec,
}

pub struct CompletionItem {
    pub label: String,
    pub detail: Option<String>,
}

impl<'g> LanguageService<'g> {
    pub fn new(spec: &'g RecordSpec) -> Self {
        Self { spec }
    }

    fn keywords(&self) -> Vec<String> {
        let mut out = Vec::new();
        collect_keywords(self.spec, &mut out, &mut HashSet::new(), &mut HashSet::new());
        out
    }

    pub fn semantic_tokens(&self, text: &str) -> Vec<(TokenClass, TextSpan)> {
        let limits = Limits::default();
        let tokens = lex(text, &limits, true).unwrap_or_default();
        let keywords = self.keywords();
        let keyword_refs: Vec<&str> = keywords.iter().map(String::as_str).collect();
        dsl_core::token_classes(&tokens, &keyword_refs)
    }

    pub fn diagnostics(&self, text: &str) -> Vec<TextError> {
        match parse(text, self.spec, &ParseOptions::default()) {
            Ok(_) => Vec::new(),
            Err(e) => vec![e],
        }
    }

    /// @emoji 💡️ Completions at `offset`: every key not yet used in the record enclosing the
    /// cursor, plus every keyword reachable from the root. A simple, always-available baseline —
    /// full context-sensitive narrowing is a natural follow-up once `Cst` gains node addressing.
    pub fn completions(&self, _text: &str, _offset: usize) -> Vec<CompletionItem> {
        let mut items: Vec<CompletionItem> = self
            .spec
            .fields
            .iter()
            .filter(|f| !f.key.is_empty())
            .map(|f| CompletionItem { label: f.key.clone(), detail: Some(format!("{:?}", f.shape)) })
            .collect();
        for keyword in self.keywords() {
            items.push(CompletionItem { label: keyword, detail: None });
        }
        items
    }
}

fn collect_keywords(spec: &RecordSpec, out: &mut Vec<String>, seen: &mut HashSet<String>, seen_records: &mut HashSet<usize>) {
    if let Some(kw) = &spec.keyword {
        out.push(kw.clone());
    }
    for field in &spec.fields {
        collect_shape_keywords(&field.shape, out, seen, seen_records);
    }
}

/// @emoji 🔁️ `seen` guards against a genuinely self-referential `Statements` table (a recursive
/// block tree whose own variant list contains itself): each `spec_fn()` call is only expanded the
/// first time its keyword is reached, so the keyword set — which is always finite, even when the
/// grammar's real nesting isn't — is collected exactly once instead of infinitely. `seen_records`
/// is the same guard for a self-referential `Shape::Record` (a `#[derive(DslRecord)]` struct field
/// whose type recurses back to itself, e.g. a dynamic-value type nesting a map of itself) — a bare
/// Record has no keyword to key on, so this tracks the `fn() -> RecordSpec` pointer's own address
/// instead (two calls to the same generated `__dsl_spec` always share one code address).
fn collect_shape_keywords(shape: &Shape, out: &mut Vec<String>, seen: &mut HashSet<String>, seen_records: &mut HashSet<usize>) {
    match shape {
        Shape::Record(spec_fn) => {
            if seen_records.insert(*spec_fn as usize) {
                collect_keywords(&spec_fn(), out, seen, seen_records);
            }
        }
        Shape::Block(inner) => collect_shape_keywords(inner, out, seen, seen_records),
        Shape::Statements(variants) => {
            for (kw, spec_fn) in variants {
                out.push(kw.clone());
                if seen.insert(kw.clone()) {
                    collect_keywords(&spec_fn(), out, seen, seen_records);
                }
            }
        }
        Shape::List(inner) | Shape::Tuple(inner, _) | Shape::Map(inner) => collect_shape_keywords(inner, out, seen, seen_records),
        _ => {}
    }
}
//#endregion 🔖️Language

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn assert_round_trip(text: &str, spec: &RecordSpec) {
        let opts = ParseOptions::default();
        let value = parse(text, spec, &opts).unwrap_or_else(|e| panic!("parse failed for {text:?}: {e}"));
        let printed = print(&value, spec, JoinMode::Document);
        let reparsed = parse(&printed, spec, &opts).unwrap_or_else(|e| panic!("reparse of printed output failed: {e}\nprinted:\n{printed}"));
        assert_eq!(value, reparsed, "round trip diverged;\noriginal print:\n{printed}");
    }

    fn assert_document_inline_agree(text: &str, spec: &RecordSpec) {
        let doc_opts = ParseOptions { limits: Limits::default(), mode: SourceMode::Document };
        let value = parse(text, spec, &doc_opts).expect("parse document");
        let inline_text = print(&value, spec, JoinMode::Inline);
        assert!(!inline_text.contains('\n'), "inline render must be one line: {inline_text:?}");
        let inline_opts = ParseOptions { limits: Limits::default(), mode: SourceMode::Inline };
        let reparsed = parse(&inline_text, spec, &inline_opts).unwrap_or_else(|e| panic!("inline reparse failed: {e}\ninline:\n{inline_text}"));
        assert_eq!(value, reparsed, "Document and Inline renders must parse to the same value");
    }

    // --- primitive 1: record with typed scalar fields, order-independent key=value ---
    fn camera_spec() -> RecordSpec {
        RecordSpec::new(
            Some("camera"),
            RecordLayout::Inline,
            vec![
                FieldSpec::new(0, "x", Shape::Float),
                FieldSpec::new(1, "y", Shape::Float),
                FieldSpec::new(2, "zoom", Shape::Float),
                FieldSpec::new(3, "label", Shape::Text).optional(),
            ],
        )
    }

    #[test]
    fn primitive_scalar_record_round_trips_and_is_order_independent() {
        let spec = camera_spec();
        assert_round_trip("camera x=1 y=2 zoom=3", &spec);
        assert_round_trip("camera zoom=3 x=1 y=2", &spec);
        assert_round_trip("camera x=-1.5 y=0 zoom=2.25 label=\"hi \\\"there\\\"\"", &spec);
        assert_document_inline_agree("camera x=1 y=2 zoom=3", &spec);
    }

    #[test]
    fn primitive_optional_field_omits_on_print_and_absent_on_parse() {
        let spec = camera_spec();
        let value = parse("camera x=1 y=2 zoom=1", &spec, &ParseOptions::default()).expect("parse");
        assert_eq!(value.get(3), Some(&FieldValue::Absent));
        let printed = print(&value, &spec, JoinMode::Document);
        assert!(!printed.contains("label"), "optional absent field must be omitted: {printed}");
    }

    // --- primitive: embed — fenced verbatim text (Document) / escaped Text (Inline) ---
    fn writer_note_spec() -> RecordSpec {
        RecordSpec::new(Some("query"), RecordLayout::Inline, vec![FieldSpec::new(0, "id", Shape::Text).positional(0), FieldSpec::new(1, "body", Shape::Embed("jack"))])
    }

    #[test]
    fn embed_round_trips_multiline_fenced_content_in_document_mode() {
        let spec = writer_note_spec();
        assert_round_trip("query q1 body=```jack\nMATCH (a) RETURN a\nWHERE a.x > 1\n```", &spec);
    }

    #[test]
    fn embed_document_and_inline_renders_agree() {
        let spec = writer_note_spec();
        assert_document_inline_agree("query q1 body=```jack\nMATCH (a) RETURN a\n```", &spec);
    }

    #[test]
    fn embed_empty_lang_tag_is_accepted_and_canonicalizes_to_the_declared_lang() {
        let spec = writer_note_spec();
        let value = parse("query q1 body=```\nMATCH (a) RETURN a\n```", &spec, &ParseOptions::default()).expect("parse with empty lang tag");
        let printed = print(&value, &spec, JoinMode::Document);
        assert!(printed.contains("```jack"), "empty lang tag must canonicalize to the field's declared lang: {printed}");
    }

    #[test]
    fn embed_rejects_a_mismatched_lang_tag() {
        let spec = writer_note_spec();
        let error = parse("query q1 body=```python\nprint(1)\n```", &spec, &ParseOptions::default()).unwrap_err();
        assert!(error.message.contains("jack"), "{error}");
    }

    #[test]
    fn embed_inline_mode_accepts_a_quoted_escaped_string_directly() {
        let spec = writer_note_spec();
        let value = parse("query q1 body=\"MATCH (a) RETURN a\"", &spec, &ParseOptions { limits: Limits::default(), mode: SourceMode::Inline }).expect("inline parse");
        let FieldValue::Text(body) = value.get(1).expect("body field") else { panic!("expected Text") };
        assert_eq!(body, "MATCH (a) RETURN a");
    }

    #[test]
    fn embed_empty_content_round_trips() {
        let spec = writer_note_spec();
        assert_round_trip("query q1 body=```jack\n```", &spec);
    }

    // --- primitive: expr — an arithmetic formula literal ---
    fn formula_spec() -> RecordSpec {
        RecordSpec::new(Some("combine"), RecordLayout::Inline, vec![FieldSpec::new(0, "id", Shape::Text).positional(0), FieldSpec::new(1, "value", Shape::Expr)])
    }

    #[test]
    fn expr_round_trips_a_load_combination_formula() {
        let spec = formula_spec();
        assert_round_trip("combine ULS value=(1.35*G + 1.5*Q)", &spec);
        assert_document_inline_agree("combine ULS value=(1.35*G + 1.5*Q)", &spec);
    }

    #[test]
    fn expr_parses_with_correct_precedence() {
        let spec = formula_spec();
        let value = parse("combine c value=(10-2*3)", &spec, &ParseOptions::default()).expect("parse");
        let FieldValue::Expr(expr) = value.get(1).expect("value field") else { panic!("expected Expr") };
        assert_eq!(
            *expr,
            ExprValue::Binary(
                ExprOp::Sub,
                Box::new(ExprValue::Num(10.0)),
                Box::new(ExprValue::Binary(ExprOp::Mul, Box::new(ExprValue::Num(2.0)), Box::new(ExprValue::Num(3.0)))),
            ),
            "10-2*3 must parse as 10-(2*3), not (10-2)*3"
        );
    }

    #[test]
    fn expr_right_nested_addition_round_trips_through_parens() {
        // a+(b+c) is structurally distinct from (a+b)+c; canonical print must keep the parens.
        let spec = formula_spec();
        let value = parse("combine c value=(a+(b+c))", &spec, &ParseOptions::default()).expect("parse");
        let printed = print(&value, &spec, JoinMode::Document);
        assert!(printed.contains("(b + c)"), "right-nested addition must keep disambiguating parens: {printed}");
        let reparsed = parse(&printed, &spec, &ParseOptions::default()).expect("reparse");
        assert_eq!(value, reparsed);
    }

    #[test]
    fn expr_supports_unary_minus_and_function_calls() {
        let spec = formula_spec();
        assert_round_trip("combine c value=(min(a, b) + -1)", &spec);
    }

    #[test]
    fn expr_glued_negative_number_after_operand_canonicalizes_to_spaced_subtraction() {
        // Hand-written "10-2" (no space) hits the lexer's negative-number-literal rule, not a bare
        // Minus token; the Expr parser must still interpret it as subtraction, and re-print it with
        // real spacing so the ambiguity never reappears in canonical output.
        let spec = formula_spec();
        let value = parse("combine c value=(10-2)", &spec, &ParseOptions::default()).expect("parse");
        let FieldValue::Expr(expr) = value.get(1).expect("value field") else { panic!("expected Expr") };
        assert_eq!(*expr, ExprValue::Binary(ExprOp::Sub, Box::new(ExprValue::Num(10.0)), Box::new(ExprValue::Num(2.0))));
        let printed = print(&value, &spec, JoinMode::Document);
        assert!(printed.contains("10 - 2"), "canonical print must space out the operator: {printed}");
    }

    // --- primitive: quantity/angle — a Shape::Float refinement that prints/parses a glued unit suffix ---
    fn material_spec() -> RecordSpec {
        RecordSpec::new(
            Some("material"),
            RecordLayout::Inline,
            vec![
                FieldSpec::new(0, "e", Shape::Quantity(dsl_core::unit_by_symbol("GPa").unwrap())),
                FieldSpec::new(1, "rho", Shape::Quantity(dsl_core::unit_by_symbol("kg/m3").unwrap())),
                FieldSpec::new(2, "rotation", Shape::Angle(dsl_core::unit_by_symbol("deg").unwrap())),
            ],
        )
    }

    #[test]
    fn quantity_and_angle_round_trip_in_their_declared_unit() {
        let spec = material_spec();
        assert_round_trip("material e=210GPa rho=7850kg/m3 rotation=45deg", &spec);
        assert_document_inline_agree("material e=210GPa rho=7850kg/m3 rotation=45deg", &spec);
    }

    #[test]
    fn quantity_accepts_a_compatible_alien_unit_and_canonicalizes_to_the_declared_one() {
        let spec = material_spec();
        let value = parse("material e=210000MPa rho=7850kg/m3 rotation=45deg", &spec, &ParseOptions::default()).expect("parse alien unit");
        let printed = print(&value, &spec, JoinMode::Document);
        assert!(printed.contains("e=210GPa"), "alien-unit input must canonicalize to the declared unit: {printed}");
    }

    #[test]
    fn quantity_with_no_suffix_is_already_in_the_declared_unit() {
        let spec = material_spec();
        let with_suffix = parse("material e=210GPa rho=7850kg/m3 rotation=45deg", &spec, &ParseOptions::default()).expect("parse with suffix");
        let bare = parse("material e=210 rho=7850kg/m3 rotation=45deg", &spec, &ParseOptions::default()).expect("parse bare number");
        assert_eq!(with_suffix.get(0), bare.get(0), "a bare number must equal the same value spelled with its declared unit's suffix");
    }

    // --- primitive: coord/dir/dim/range/count/ref — sigil-and-glyph notation literals ---
    fn placement_spec() -> RecordSpec {
        RecordSpec::new(
            Some("object"),
            RecordLayout::Inline,
            vec![
                FieldSpec::new(0, "id", Shape::Text).positional(0),
                FieldSpec::new(1, "material", Shape::Ref("material")),
                FieldSpec::new(2, "position", Shape::Coord(3)),
                FieldSpec::new(3, "axis", Shape::Dir),
                FieldSpec::new(4, "size", Shape::Dim(3)),
                FieldSpec::new(5, "slider", Shape::Range),
                FieldSpec::new(6, "count", Shape::Count),
            ],
        )
    }

    #[test]
    fn coord_dir_dim_range_count_ref_round_trip() {
        let spec = placement_spec();
        assert_round_trip("object col-a material=s355 position=@1.35,0,0 axis=^0,1,0 size=2.4x0.12x0.24 slider=(0..10,0.5) count=x24", &spec);
        assert_document_inline_agree("object col-a material=s355 position=@1.35,0,0 axis=^0,1,0 size=2.4x0.12x0.24 slider=(0..10,0.5) count=x24", &spec);
    }

    #[test]
    fn range_without_step_round_trips_with_two_elements() {
        let spec = RecordSpec::new(Some("slot"), RecordLayout::Inline, vec![FieldSpec::new(0, "window", Shape::Range)]);
        assert_round_trip("slot window=(0..10)", &spec);
    }

    #[test]
    fn coord_dir_dim_range_count_reject_wrong_arity_or_form() {
        let spec = placement_spec();
        // Coord declared as 3 components; only 2 given.
        let err = parse("object col-a material=s355 position=@1.35,0 axis=^0,1,0 size=2.4x0.12x0.24 slider=(0..10) count=x1", &spec, &ParseOptions::default()).unwrap_err();
        assert!(err.message.contains("coordinate") || err.message.contains("expected"), "{err}");
        // Dim declared as 3 components; only one number, no glued 'x' suffix at all.
        let err2 = parse("object col-a material=s355 position=@1,2,3 axis=^0,1,0 size=2.4 slider=(0..10) count=x1", &spec, &ParseOptions::default()).unwrap_err();
        assert!(err2.message.contains("dimension"), "{err2}");
        // Count without the 'x' prefix is not a valid count literal.
        let err3 = parse("object col-a material=s355 position=@1,2,3 axis=^0,1,0 size=2.4x0.12x0.24 slider=(0..10) count=24", &spec, &ParseOptions::default()).unwrap_err();
        assert!(err3.message.contains("count"), "{err3}");
    }

        #[test]
    fn quantity_rejects_an_incompatible_unit() {
        let spec = material_spec();
        let error = parse("material e=210kg rho=7850kg/m3 rotation=45deg", &spec, &ParseOptions::default()).unwrap_err();
        assert!(error.message.contains("not compatible"), "wrong-dimension suffix must be a parse error, got: {error}");
    }

    // --- primitive 2 + 3: keyword-led statements, homogeneous ordered collection ---
    fn layer_variant_spec() -> RecordSpec {
        RecordSpec::new(Some("layer"), RecordLayout::Inline, vec![FieldSpec::new(0, "id", Shape::Text).positional(0), FieldSpec::new(1, "opacity", Shape::Float)])
    }

    fn document_with_layers_spec() -> RecordSpec {
        RecordSpec::new(
            None,
            RecordLayout::Inline,
            vec![FieldSpec::new(0, "schema", Shape::Text), FieldSpec::new(1, "layers", Shape::Statements(vec![("layer".to_string(), layer_variant_spec)]))],
        )
    }

    #[test]
    fn primitive_statements_collection_preserves_order_and_round_trips() {
        let spec = document_with_layers_spec();
        assert_round_trip("schema=doc layer a opacity=1 layer b opacity=0.5 layer c opacity=1", &spec);
        let value = parse("schema=doc layer a opacity=1 layer b opacity=0.5", &spec, &ParseOptions::default()).expect("parse");
        let FieldValue::Statements(items) = value.get(1).unwrap() else { panic!("expected statements") };
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].0, "layer");
    }

    // --- primitive 4: recursive sub-blocks ---
    /// @emoji 🌳️ Genuinely self-referential: `children`'s own variant table names `group_spec`
    /// itself. Lazy `fn() -> RecordSpec` entries make this sound — `group_spec()` doesn't recurse
    /// just to build the table, only `parse`/`print` calling the stored fn pointer one level at a
    /// time (as deep as real input actually nests) ever evaluates it again.
    fn group_spec() -> RecordSpec {
        RecordSpec::new(
            Some("group"),
            RecordLayout::Inline,
            vec![
                FieldSpec::new(0, "id", Shape::Text).positional(0),
                FieldSpec::new(1, "children", Shape::Block(Box::new(Shape::Statements(vec![("group".to_string(), group_spec)])))).optional(),
            ],
        )
    }

    #[test]
    fn primitive_recursive_blocks_round_trip() {
        let spec = group_spec();
        assert_round_trip("group root children { group a group b }", &spec);
        assert_round_trip("group leaf", &spec);
    }

    // --- primitive 6/7: escaped inline text, formerly-trailing free text ---
    #[test]
    fn primitive_escaped_text_handles_quotes_newlines_and_trailing_position() {
        let spec = camera_spec();
        let value = parse("camera x=1 y=1 zoom=1 label=\"line1\\nline2 with \\\"quotes\\\"\"", &spec, &ParseOptions::default()).expect("parse");
        assert_eq!(value.get(3), Some(&FieldValue::Text("line1\nline2 with \"quotes\"".to_string())));
    }

    // --- primitive 9: graph endpoints (wire literal) ---
    fn wire_spec() -> RecordSpec {
        RecordSpec::new(Some("edge"), RecordLayout::Inline, vec![FieldSpec::new(0, "link", Shape::Wire).positional(0)])
    }

    #[test]
    fn primitive_wire_literal_directed_and_undirected_round_trip() {
        let spec = wire_spec();
        assert_round_trip("edge a:Kind@out->b:Kind2@in", &spec);
        assert_round_trip("edge a--b", &spec);
        assert_round_trip("edge solo", &spec);
    }

    // --- primitive 10: packed tuples / lists / base64 ---
    fn geometry_spec() -> RecordSpec {
        RecordSpec::new(
            Some("vertex"),
            RecordLayout::Inline,
            vec![
                FieldSpec::new(0, "pos", Shape::Tuple(Box::new(Shape::Float), Some(3))).positional(0),
                FieldSpec::new(1, "tags", Shape::List(Box::new(Shape::Text))).optional(),
                FieldSpec::new(2, "blob", Shape::Bytes64).optional(),
            ],
        )
    }

    #[test]
    fn primitive_tuple_list_and_base64_round_trip() {
        let spec = geometry_spec();
        assert_round_trip("vertex 1,2,3", &spec);
        assert_round_trip("vertex 1,2,3 tags=[a b c]", &spec);
        assert_round_trip("vertex 0,0,0 blob=\"aGVsbG8=\"", &spec);
        let value = parse("vertex 0,0,0 blob=\"aGVsbG8=\"", &spec, &ParseOptions::default()).expect("parse");
        assert_eq!(value.get(2), Some(&FieldValue::Bytes64(b"hello".to_vec())));
    }

    // --- primitive 11: dynamic value literal ---
    fn value_spec() -> RecordSpec {
        RecordSpec::new(Some("payload"), RecordLayout::Inline, vec![FieldSpec::new(0, "data", Shape::Value)])
    }

    #[test]
    fn primitive_dynamic_value_round_trips_and_bridges_to_serde_json() {
        let spec = value_spec();
        assert_round_trip("payload data={a=1 b=[1 2 3] c=\"x\"}", &spec);
        let value = parse("payload data={a=1}", &spec, &ParseOptions::default()).expect("parse");
        let FieldValue::Value(dsl_value) = value.get(0).unwrap().clone() else { panic!() };
        let json: serde_json::Value = dsl_value.into();
        assert_eq!(json["a"], serde_json::json!(1.0));
    }

    // --- primitive 12: sparse patch records (Option<T> absent != null) ---
    #[test]
    fn primitive_sparse_patch_distinguishes_absent_from_present() {
        let spec = camera_spec();
        let with = parse("camera x=1 y=1 zoom=1 label=\"x\"", &spec, &ParseOptions::default()).expect("parse with");
        let without = parse("camera x=1 y=1 zoom=1", &spec, &ParseOptions::default()).expect("parse without");
        assert_ne!(with.get(3), without.get(3));
        assert_eq!(without.get(3), Some(&FieldValue::Absent));
    }

    // --- primitive 15: comments ---
    #[test]
    fn primitive_comments_are_skipped_as_trivia() {
        let spec = camera_spec();
        let value = parse("# a comment\ncamera x=1 y=2 zoom=3 # trailing comment", &spec, &ParseOptions::default()).expect("parse with comments");
        assert_eq!(value.get(0), Some(&FieldValue::Float(1.0)));
    }

    // --- primitive 16: real spans ---
    #[test]
    fn primitive_spans_are_real_on_parse_error() {
        let spec = camera_spec();
        let error = parse("camera x=1\ny=notanumber zoom=1", &spec, &ParseOptions::default()).unwrap_err();
        assert_eq!(error.span.line, 2, "error span must point at the real line, not (1,1)");
    }

    // --- bare-string printing: `is_bare_ident` values print unquoted, reserved/number-shaped/
    // multi-word values stay quoted (unified syntax law: strings bare-preferred) ---
    #[test]
    fn bare_strings_print_unquoted_and_reserved_or_number_shaped_values_stay_quoted() {
        let spec = camera_spec();
        let value = parse("camera x=1 y=2 zoom=3 label=alpha", &spec, &ParseOptions::default()).expect("parse");
        let printed = print(&value, &spec, JoinMode::Document);
        assert!(printed.contains("label=alpha"), "a bare-ident-shaped value must print unquoted: {printed}");
        assert!(!printed.contains("\"alpha\""), "must not quote a value that already lexes as a bare ident: {printed}");

        for reserved in ["_", "true", "3", "two words"] {
            let mut writer = Writer::new();
            print_shape(&FieldValue::Text(reserved.to_string()), &Shape::Text, &mut writer);
            let out = writer.render(JoinMode::Inline);
            assert!(out.starts_with('"') && out.ends_with('"'), "{reserved:?} must print quoted, got {out:?}");
        }
    }

    // --- `Writer::glue()`: exact-string spacing assertions for every composite shape's
    // `key=value` fusion (the "key= value" bug this replaces) ---
    fn nested_point_spec() -> RecordSpec {
        RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "x", Shape::Float), FieldSpec::new(1, "y", Shape::Float)])
    }
    fn marker_spec() -> RecordSpec {
        RecordSpec::new(Some("marker"), RecordLayout::Inline, vec![FieldSpec::new(0, "at", Shape::Record(nested_point_spec))])
    }
    fn edge_keyed_wire_spec() -> RecordSpec {
        RecordSpec::new(Some("edge2"), RecordLayout::Inline, vec![FieldSpec::new(0, "link", Shape::Wire)])
    }
    fn tags_map_spec() -> RecordSpec {
        RecordSpec::new(Some("meta"), RecordLayout::Inline, vec![FieldSpec::new(0, "props", Shape::Map(Box::new(Shape::Text)))])
    }

    #[test]
    fn glue_removes_the_key_equals_space_for_every_composite_shape() {
        // List
        let spec = geometry_spec();
        let value = parse("vertex 1,2,3 tags=[a b c]", &spec, &ParseOptions::default()).expect("parse list");
        let printed = print(&value, &spec, JoinMode::Document);
        assert!(printed.contains("tags=[ a b c ]"), "List field must glue key= directly onto '[': {printed}");
        assert!(!printed.contains("tags= ["), "must never leave a stray space after 'key=': {printed}");

        // Value (dynamic)
        let spec = value_spec();
        let value = parse("payload data={a=1}", &spec, &ParseOptions::default()).expect("parse value");
        let printed = print(&value, &spec, JoinMode::Document);
        assert!(printed.contains("data={"), "Value field must glue key= directly onto '{{': {printed}");
        assert!(!printed.contains("data= {"), "must never leave a stray space before the glued brace: {printed}");

        // Map
        let spec = tags_map_spec();
        let value = parse("meta props={a=\"x\" b=\"y\"}", &spec, &ParseOptions::default()).expect("parse map");
        let printed = print(&value, &spec, JoinMode::Document);
        assert!(printed.contains("props={"), "Map field must glue key= directly onto '{{': {printed}");
        assert_round_trip("meta props={a=\"x\" b=\"y\"}", &spec);

        // Record (nested, un-blocked — prints inline without its own keyword)
        let spec = marker_spec();
        let value = parse("marker at=x=1 y=2", &spec, &ParseOptions::default()).expect("parse record");
        let printed = print(&value, &spec, JoinMode::Document);
        assert!(printed.contains("at=x=1"), "Record field must glue key= directly onto its first field: {printed}");
        assert!(!printed.contains("at= x=1"), "must never leave a stray space before a nested record: {printed}");

        // Wire (keyed, not positional)
        let spec = edge_keyed_wire_spec();
        let value = parse("edge2 link=a->b", &spec, &ParseOptions::default()).expect("parse wire");
        let printed = print(&value, &spec, JoinMode::Document);
        assert!(printed.contains("link=a->b"), "Wire field must glue key= directly onto the wire literal: {printed}");
        assert!(!printed.contains("link= a"), "must never leave a stray space before a keyed wire literal: {printed}");
    }

    // --- wire `<-` normalization: accepted sugar only, always stored/printed as `->` with
    // endpoints swapped ---
    #[test]
    fn wire_back_arrow_normalizes_to_forward_arrow_with_swapped_endpoints() {
        let spec = wire_spec();
        let backward = parse("edge b<-a", &spec, &ParseOptions::default()).expect("parse backward");
        let forward = parse("edge a->b", &spec, &ParseOptions::default()).expect("parse forward");
        assert_eq!(backward, forward, "'b<-a' must parse to the same value as 'a->b'");
        let printed = print(&backward, &spec, JoinMode::Document);
        assert!(printed.contains("a->b"), "must print using '->': {printed}");
        assert!(!printed.contains("<-"), "must never print '<-': {printed}");
    }

    #[test]
    fn parse_wire_text_parses_a_standalone_wire_literal_with_back_arrow() {
        let value = parse_wire_text("b<-a").expect("parse_wire_text");
        assert_eq!(value.from.id, "a");
        let (directed, to) = value.edge.expect("edge");
        assert!(directed);
        assert_eq!(to.id, "b");
    }

    // --- primitive 17: `Shape::Table` — SoA columnar collection ---
    fn table_row_spec() -> RecordSpec {
        RecordSpec::new(
            None,
            RecordLayout::Inline,
            vec![
                FieldSpec::new(0, "id", Shape::Text),
                FieldSpec::new(1, "x", Shape::Float),
                FieldSpec::new(2, "y", Shape::Float),
                FieldSpec::new(3, "link", Shape::Wire).optional(),
            ],
        )
    }
    fn table_doc_spec() -> RecordSpec {
        RecordSpec::new(Some("scene"), RecordLayout::Inline, vec![FieldSpec::new(0, "nodes", Shape::Table(table_row_spec))])
    }

    #[test]
    fn table_soa_round_trips_with_underscore_absent_cell_and_a_wire_column() {
        let spec = table_doc_spec();
        let text = "scene nodes [id:TEXT x:NUM y:NUM link:WIRE] { a 1 2 _  b 3 4 a@out->b@in }";
        assert_round_trip(text, &spec);
        let value = parse(text, &spec, &ParseOptions::default()).expect("parse");
        let FieldValue::List(rows) = value.get(0).unwrap() else { panic!("expected a table (List) value") };
        assert_eq!(rows.len(), 2);
        let FieldValue::Record(row0) = &rows[0] else { panic!("expected a Record row") };
        assert_eq!(row0.get(3), Some(&FieldValue::Absent), "the '_' cell must parse as Absent");
        let FieldValue::Record(row1) = &rows[1] else { panic!("expected a Record row") };
        assert!(matches!(row1.get(3), Some(FieldValue::Wire(_))), "the wire-typed column must parse as FieldValue::Wire");

        let printed = print(&value, &spec, JoinMode::Document);
        assert!(printed.contains("nodes [id:TEXT x:NUM y:NUM link:WIRE]"), "header must print tight SoA, no inner spaces: {printed}");
    }

    #[test]
    fn table_accepts_verbose_aos_input_and_canonicalizes_to_soa_output() {
        let spec = table_doc_spec();
        let aos_text = "scene nodes=[ {id=a x=1 y=2} {id=b x=3 y=4} ]";
        let value = parse(aos_text, &spec, &ParseOptions::default()).expect("parse AoS-verbose");
        let printed = print(&value, &spec, JoinMode::Document);
        assert!(printed.contains("nodes [id:TEXT x:NUM y:NUM link:WIRE]"), "AoS input must canonicalize to the SoA header on print: {printed}");
        assert!(!printed.contains("nodes="), "must never print the old AoS '=' form: {printed}");
        let reparsed = parse(&printed, &spec, &ParseOptions::default()).expect("reparse canonicalized SoA");
        assert_eq!(value, reparsed, "AoS-in/SoA-out must still round trip to the same value");
    }

    #[test]
    fn table_header_without_explicit_type_tags_is_still_parseable() {
        let spec = table_doc_spec();
        let text = "scene nodes [id x y link] { a 1 2 _  b 3 4 a@out->b@in }";
        let value = parse(text, &spec, &ParseOptions::default()).expect("parse header without explicit types");
        let FieldValue::List(rows) = value.get(0).unwrap() else { panic!("expected a table (List) value") };
        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn table_document_and_inline_renders_agree() {
        let spec = table_doc_spec();
        assert_document_inline_agree("scene nodes [id:TEXT x:NUM y:NUM link:WIRE] { a 1 2 _  b 3 4 a@out->b@in }", &spec);
    }

    #[test]
    fn table_rejects_non_self_delimiting_column_shapes_at_spec_build_time() {
        fn unbounded_tuple_row_spec() -> RecordSpec {
            RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "vals", Shape::Tuple(Box::new(Shape::Float), None))])
        }
        fn bad_table_doc_spec() -> RecordSpec {
            RecordSpec::new(Some("bad"), RecordLayout::Inline, vec![FieldSpec::new(0, "rows", Shape::Table(unbounded_tuple_row_spec))])
        }
        let spec = bad_table_doc_spec();
        let result = parse("bad rows [vals:TUPLE] { 1,2,3 }", &spec, &ParseOptions::default());
        assert!(result.is_err(), "an unbounded Tuple column must be rejected, not silently accepted");
    }

    // --- regression: a table row whose own field is ITSELF a `#[dsl(table)]` (nested SoA output
    // used to break the parser's row-boundary counting; see `print_table_list`/`parse_table_list`) ---
    fn nested_inner_row_spec() -> RecordSpec {
        RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "id", Shape::Text), FieldSpec::new(1, "val", Shape::Float).optional()])
    }
    fn nested_outer_row_spec() -> RecordSpec {
        RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "id", Shape::Text), FieldSpec::new(1, "children", Shape::Table(nested_inner_row_spec))])
    }
    fn nested_table_doc_spec() -> RecordSpec {
        RecordSpec::new(Some("doc"), RecordLayout::Inline, vec![FieldSpec::new(0, "items", Shape::Table(nested_outer_row_spec))])
    }

    #[test]
    fn table_row_containing_its_own_table_field_round_trips_without_desync() {
        let spec = nested_table_doc_spec();
        let text = "doc items [id:TEXT children:TABLE] { p1 [ {id=c1 val=1.5} {id=c2} ]  p2 [ {id=c3 val=2} ] }";
        assert_round_trip(text, &spec);
        assert_document_inline_agree(text, &spec);

        let value = parse(text, &spec, &ParseOptions::default()).expect("parse nested table");
        let FieldValue::List(outer_rows) = value.get(0).unwrap() else { panic!("expected outer table (List)") };
        assert_eq!(outer_rows.len(), 2);
        let FieldValue::Record(row0) = &outer_rows[0] else { panic!("expected outer Record row") };
        let FieldValue::List(inner_rows) = row0.get(1).unwrap() else { panic!("expected nested table (List)") };
        assert_eq!(inner_rows.len(), 2, "the inner table's own row count must not desync from its header");
        let FieldValue::Record(inner_row1) = &inner_rows[1] else { panic!("expected inner Record row") };
        assert_eq!(inner_row1.get(1), Some(&FieldValue::Absent), "the second inner row's absent 'val' must round trip as Absent, not corrupt later parsing");
    }

    // --- regression: a table row with 2+ columns of the exact same nested `DslRecord` type (the
    // greedy same-key consumption bug: an unset field on column N used to silently eat a later
    // column's same-named present value; see `print_table_cell`/`parse_table_cell`) ---
    fn quantity_spec() -> RecordSpec {
        RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(0, "target", Shape::Float).optional(), FieldSpec::new(1, "actual", Shape::Float).optional()])
    }
    fn duplicate_type_row_spec() -> RecordSpec {
        RecordSpec::new(
            None,
            RecordLayout::Inline,
            vec![
                FieldSpec::new(0, "id", Shape::Text),
                FieldSpec::new(1, "area", Shape::Record(quantity_spec)),
                FieldSpec::new(2, "volume", Shape::Record(quantity_spec)),
            ],
        )
    }
    fn duplicate_type_table_doc_spec() -> RecordSpec {
        RecordSpec::new(Some("doc"), RecordLayout::Inline, vec![FieldSpec::new(0, "rows", Shape::Table(duplicate_type_row_spec))])
    }

    #[test]
    fn table_row_with_two_columns_of_the_same_record_type_does_not_cross_contaminate() {
        let spec = duplicate_type_table_doc_spec();
        // `area`'s `target` is left absent (never printed) while `volume`'s `target=4` is present
        // right after it — the exact shape of the reported corruption, since both columns share
        // the identical field name.
        let text = "doc rows [id:TEXT area:REC volume:REC] { r1 {actual=3} {target=4 actual=1} }";
        assert_round_trip(text, &spec);
        assert_document_inline_agree(text, &spec);

        let value = parse(text, &spec, &ParseOptions::default()).expect("parse duplicate-type columns");
        let FieldValue::List(rows) = value.get(0).unwrap() else { panic!("expected table (List)") };
        let FieldValue::Record(row0) = &rows[0] else { panic!("expected Record row") };
        let FieldValue::Record(area) = row0.get(1).unwrap() else { panic!("expected area Record") };
        let FieldValue::Record(volume) = row0.get(2).unwrap() else { panic!("expected volume Record") };
        assert_eq!(area.get(0), Some(&FieldValue::Absent), "area.target must stay absent, not stolen from volume's column");
        assert_eq!(area.get(1), Some(&FieldValue::Float(3.0)), "area.actual must be area's own value");
        assert_eq!(volume.get(0), Some(&FieldValue::Float(4.0)), "volume.target must not be consumed by area's parse");
        assert_eq!(volume.get(1), Some(&FieldValue::Float(1.0)), "volume.actual must be volume's own value");
    }

    // --- idempotent canonicalization ---
    #[test]
    fn canonicalization_is_idempotent() {
        let spec = camera_spec();
        let once = canonicalize("camera   zoom=3   x=1 y=2", &spec, &ParseOptions::default()).expect("canonicalize once");
        let twice = canonicalize(&once, &spec, &ParseOptions::default()).expect("canonicalize twice");
        assert_eq!(once, twice, "canonicalize(canonicalize(x)) must equal canonicalize(x)");
    }

    // --- limits enforced, not panicking ---

    #[test]
    fn deeply_nested_blocks_hit_the_depth_limit_as_a_diagnostic() {
        // `group_spec()` (primitive 4, above) is already genuinely self-referential, so it needs no
        // pre-unrolling to exercise real depth this many levels deep — `parse` only ever expands one
        // level of its lazy `Statements` fn pointer at a time, following the actual input text.
        let levels = 20;
        let spec = group_spec();
        let mut nested = String::from("group root");
        for _ in 0..levels {
            nested.push_str(" children { group a");
        }
        for _ in 0..levels {
            nested.push('}');
        }
        let tiny_limits = Limits { max_depth: 10, ..Limits::default() };
        let opts = ParseOptions { limits: tiny_limits, mode: SourceMode::Document };
        let result = parse(&nested, &spec, &opts);
        assert!(result.is_err(), "exceeding max_depth must produce an error, not a stack overflow");

        let generous_limits = Limits { max_depth: 100, ..Limits::default() };
        let generous_opts = ParseOptions { limits: generous_limits, mode: SourceMode::Document };
        assert!(parse(&nested, &spec, &generous_opts).is_ok(), "the same nesting must parse fine under a generous depth limit");
    }

    // --- LanguageService ---
    #[test]
    fn language_service_reports_semantic_tokens_and_diagnostics() {
        let spec = camera_spec();
        let service = LanguageService::new(&spec);
        let classes = service.semantic_tokens("camera x=1 y=2 zoom=3");
        assert!(classes.iter().any(|(class, _)| *class == TokenClass::Keyword));
        assert!(service.diagnostics("camera x=1 y=2 zoom=3").is_empty());
        assert!(!service.diagnostics("camera x=notanumber").is_empty());
    }

    #[test]
    fn language_service_completions_include_every_declared_key() {
        let spec = camera_spec();
        let service = LanguageService::new(&spec);
        let labels: Vec<String> = service.completions("", 0).into_iter().map(|c| c.label).collect();
        assert!(labels.contains(&"x".to_string()));
        assert!(labels.contains(&"zoom".to_string()));
        assert!(labels.contains(&"label".to_string()));
    }

    // --- 10k-iteration generative round trip over the flat-scalar shape ---
    #[test]
    fn generative_round_trip_over_scalar_records() {
        let spec = camera_spec();
        let mut state: u64 = 0xD1B54A32D192ED03;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        for _ in 0..2_000 {
            let x = (next() % 2000) as i64 - 1000;
            let y = (next() % 2000) as i64 - 1000;
            let zoom = (next() % 2000) as i64 - 1000;
            let text = format!("camera x={x} y={y} zoom={zoom}");
            assert_round_trip(&text, &spec);
        }
    }
}
//#endregion 🧪️Tests
