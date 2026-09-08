//! @emoji 📖️ `dsl_grammar` — the self-hosted `.grammar` spec format: a hand-authorable,
//! EBNF-style description of one language's productions, used as the *normative* artifact every
//! handcrafted grammar in the repo ships alongside its parser/printer. This crate parses and
//! prints the format itself (this crate's own `📖️grammar/📖️grammar.grammar` is written in it and
//! parses cleanly under its own parser — see the `self_hosting` test), and provides a recognizer
//! that can check a target document's tokens against a compiled grammar for the subset of
//! productions this v1 supports (see the Recognizer region's doc comment for exactly what that
//! covers today and what it does not yet).
//!
//! Depends on `dsl_core` only, following the same "own pre-scan lexer delegating the shared
//! alphabet to `crate::os_dsl::lex`" pattern `math::graph::dsl` (Jack) established — `?` and `|`
//! aren't in the shared token alphabet (a structural-DSL alphabet has no need for them), so this
//! crate's lexer pre-scans those two characters itself and hands every other run of characters to
//! `crate::os_dsl::lex` unchanged.

use crate::os_dsl::{lex as core_lex, lex_with as core_lex_with, CommentDialect, LexOptions, Limits, StringEscape, StringMode, TextError, TextSpan, TokenKind as CoreKind};

//#region 🔖️Model
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemioDialect {
    Grammar,
    Protocol,
}

/// @emoji 📄️ One parsed `.grammar.semio` / `.protocol.semio` file: header directives + productions.
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

/// @emoji 🧩️ One symbol in a production's alternative. `Terminal` vs. `Ref` is decided lexically
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
/// @emoji 📡️ One parsed `.protocol.semio` file: framing + typed body directives.
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
}

/// @emoji 🔀️ P2-M2 item 4: guard gating a field's (or a whole segment's) presence on an
/// EARLIER-decoded field's value — bmp's BITFIELDS masks (`if compression eq 3`), palette
/// (`if bits_per_pixel le 8`). Evaluated against the walk-wide field env (item 3), so the guarded
/// field can reference a value decoded in ANY earlier block, not just the same one.
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
}

/// @emoji 🔁️ P2-M2 item 1: one "repeated tag-dispatched block" — read a discriminator (+ optional
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

/// @emoji 🌿️ One recognized discriminator value's field-set. `nested` supports the GIF 89a
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
    /// P2-M2 item 1.
    Repeat {
        name: String,
        dispatch: RepeatDispatch,
    },
    /// P2-M2 item 5a: scan BACKWARD from EOF for `magic`'s exact byte pattern (ZIP's EOCD, whose
    /// preceding comment field is 0-65535 bytes — its start is unknowable except by finding the
    /// EOCD itself first), jump `pos` directly to the match, then walk `fields` forward from there.
    BackwardScan {
        name: String,
        magic: Vec<u8>,
        fields: Vec<Field>,
    },
    /// P2-M2 item 5b: jump `pos` to the ABSOLUTE offset held in `offset_field` (decoded by an
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
    /// P2-M2 item 2: ALWAYS big-endian regardless of the walker's runtime endian mode (png/jpg/
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
    /// P2-M2 item 1c: scan forward past every `prefix` byte, then take the next byte as the
    /// discriminator — JPG's marker-prefix scan (`0xFF` fill bytes before a real marker code),
    /// distinct from a fixed-position tag read.
    MarkerScan(u8),
    /// P2-M2 item 6: TIFF-style runtime-selected endianness. Reads `key.len()` bytes (all keys
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

/// @emoji ✅️ Successful byte walk: every declared wire slot consumed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProtocolTrace {
    pub consumed: usize,
}

/// @emoji ❌️ Spec/bytes disagreement at a concrete offset.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProtocolMismatch {
    pub offset: usize,
    pub message: String,
}
//#endregion 📡️ProtocolModel

//#region 🔖️Lexer
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GKind {
    Ident,
    Int,
    Text,
    Star,
    Plus,
    Question,
    Pipe,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Equals,
    Newline,
    Eof,
}

#[derive(Clone, Debug, PartialEq)]
struct GToken {
    kind: GKind,
    text: String,
    span: TextSpan,
}

/// @emoji 🔬️ Pre-scans `?`/`|` (not in `dsl_core`'s alphabet) and delegates every other run of
/// characters whole to `crate::os_dsl::lex`, exactly like `math::graph::crate::os_dsl::lex_spanned` does
/// for its own two Cypher-specific extras.
// 🚫️async: E1 pure — `core_lex`/`core_lex_with` (`🗣️dsl/🔍️lexer/🦀️.rs`) were reverted to
// sync per R9 (0 await, 0 io, measured directly); this wrapper's only suspension source is gone —
// see R9.
fn lex(text: &str) -> Result<Vec<GToken>, TextError> {
    let bytes = text.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 0usize;
    let mut seg_start = 0usize;

    // 🔁 Was a local closure; a sync closure can't declare a nested fn that borrows `tokens`
    // mutably across multiple call sites as cleanly, so this stays a nested fn (now sync — see
    // the outer `lex`'s own R9 tag) taking `text` explicitly (residue shape 1 lineage note kept
    // for history: this used to be a nested ASYNC fn to bridge a since-reverted `core_lex`).
    fn push_segment(text: &str, seg_start: usize, seg_end: usize, tokens: &mut Vec<GToken>) -> Result<(), TextError> {
        if seg_end <= seg_start {
            return Ok(());
        }
        let segment = &text[seg_start..seg_end];
        let raw = core_lex(segment, &Limits::default(), false)?;
        for token in raw {
            if matches!(token.kind, CoreKind::Whitespace | CoreKind::Comment | CoreKind::Eof) {
                continue;
            }
            let kind = match token.kind {
                CoreKind::Ident | CoreKind::Placeholder => GKind::Ident,
                CoreKind::Int => GKind::Int,
                CoreKind::Text => GKind::Text,
                CoreKind::Star => GKind::Star,
                CoreKind::Plus => GKind::Plus,
                CoreKind::LParen => GKind::LParen,
                CoreKind::RParen => GKind::RParen,
                CoreKind::LBrace => GKind::LBrace,
                CoreKind::RBrace => GKind::RBrace,
                CoreKind::Comma => GKind::Comma,
                CoreKind::Equals => GKind::Equals,
                CoreKind::Newline => GKind::Newline,
                other => return Err(TextError::new(format!("`.grammar` files cannot contain a {other:?} token here"), token.span)),
            };
            tokens.push(GToken { kind, text: token.text.as_str().to_string(), span: token.span });
        }
        Ok(())
    }

    while i < bytes.len() {
        let c = bytes[i];
        // P2-P1 regression fix: `#`-to-end-of-line comments (the fixed default marker every
        // `.grammar`/`.protocol` file's own meta-syntax uses to tokenize its OWN header/productions,
        // independent of whatever `comment` header directive the file declares for the RECOGNIZER —
        // see `push_segment`'s `core_lex(segment, &Limits::default(), false)` call, which always
        // uses `CommentDialect::default()`) are skipped whole here too, matching `dsl_core`'s own
        // line-comment scan exactly. Without this, a `"`, `?`, or `|` character inside comment PROSE
        // (illustrating an escaped quote like `\"`, or an EBNF alternation like `a | b`, in a doc
        // comment — routine in a hand-authored grammar's own commentary) is misread by the quote/
        // operator checks below as this *lexer's* own quote-open/Pipe/Question token, corrupting
        // every segment boundary from that point on (confirmed root cause of the P2-P1 pilot
        // conformance-test parse failures — every one of them traces back to a comment-embedded `"`,
        // `?`, or `|`, not to any actual defect in the pilots' own grammar syntax).
        if c == b'#' {
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        // A quoted `TEXT` literal is skipped whole here (matching dsl_core's own `"..."` escape
        // scheme, backslash-escapes included) so a `?` or `|` *inside* a string — needed to write
        // e.g. `"|"` as a literal pipe token — is never mistaken for this lexer's own operators.
        // The segment (quotes included) is left for `push_segment`'s `core_lex` call to tokenize.
        if c == b'"' {
            i += 1;
            while i < bytes.len() {
                if bytes[i] == b'\\' && i + 1 < bytes.len() {
                    i += 2;
                    continue;
                }
                if bytes[i] == b'"' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            continue;
        }
        if c == b'?' || c == b'|' {
            push_segment(text, seg_start, i, &mut tokens)?;
            let line = text[..i].matches('\n').count() as u32 + 1;
            let col = (i - text[..i].rfind('\n').map_or(0, |p| p + 1)) as u32 + 1;
            let span = TextSpan::with_length(line, col, 1);
            tokens.push(GToken { kind: if c == b'?' { GKind::Question } else { GKind::Pipe }, text: (c as char).to_string(), span });
            i += 1;
            seg_start = i;
            continue;
        }
        i += 1;
    }
    push_segment(text, seg_start, bytes.len(), &mut tokens)?;
    let eof_span = tokens.last().map_or(TextSpan::at(1, 1), |t| t.span);
    tokens.push(GToken { kind: GKind::Eof, text: String::new(), span: eof_span });
    Ok(tokens)
}
//#endregion 🔖️Lexer

//#region 🔖️Parser
struct Cursor {
    tokens: Vec<GToken>,
    pos: usize,
}

impl Cursor {
    fn peek(&self) -> &GToken {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> GToken {
        let token = self.tokens[self.pos].clone();
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        token
    }

    fn skip_newlines(&mut self) {
        while self.peek().kind == GKind::Newline {
            self.advance();
        }
    }

    fn expect(&mut self, kind: GKind) -> Result<GToken, TextError> {
        if self.peek().kind == kind {
            Ok(self.advance())
        } else {
            Err(TextError::new(format!("expected {kind:?}, found {:?} {:?}", self.peek().kind, self.peek().text), self.peek().span))
        }
    }

    fn peek_ident(&self, expected_text: &str) -> bool {
        self.peek().kind == GKind::Ident && self.peek().text == expected_text
    }

    fn expect_ident(&mut self, expected_text: &str) -> Result<(), TextError> {
        let token = self.expect(GKind::Ident)?;
        if token.text == expected_text {
            Ok(())
        } else {
            Err(TextError::new(format!("expected keyword `{expected_text}`, found `{}`", token.text), token.span))
        }
    }

    fn expect_ident_or_int(&mut self) -> Result<GToken, TextError> {
        match self.peek().kind {
            GKind::Ident | GKind::Int => Ok(self.advance()),
            other => Err(TextError::new(format!("expected ident or int, found {other:?}"), self.peek().span)),
        }
    }
}

fn is_all_upper(text: &str) -> bool {
    text.chars().any(|c| c.is_alphabetic()) && text.chars().all(|c| c.is_uppercase() || c == '_' || c == '-' || c.is_ascii_digit())
}

fn parse_macro_args(cursor: &mut Cursor) -> Result<Vec<MacroArg>, TextError> {
    cursor.expect(GKind::LParen)?;
    let mut args = Vec::new();
    if cursor.peek().kind != GKind::RParen {
        loop {
            let arg = match cursor.peek().kind {
                GKind::Text => MacroArg::Literal(cursor.advance().text),
                GKind::Ident => MacroArg::Ident(cursor.advance().text),
                other => return Err(TextError::new(format!("expected a macro argument, found {other:?}"), cursor.peek().span)),
            };
            args.push(arg);
            if cursor.peek().kind == GKind::Comma {
                cursor.advance();
                continue;
            }
            break;
        }
    }
    cursor.expect(GKind::RParen)?;
    Ok(args)
}

fn parse_atom(cursor: &mut Cursor) -> Result<Symbol, TextError> {
    let base = match cursor.peek().kind {
        GKind::Text => Symbol::Literal(cursor.advance().text),
        // Grouping uses `{ }`, never `( )`: whitespace is discarded before parsing (trivia is
        // dropped at lex time), so a token stream alone can't distinguish `name (group)` — a
        // bareword reference followed by a separate grouped alternative — from `name(args)`, a
        // macro call. Reserving `( )` exclusively for macro-call argument lists keeps that
        // distinction unambiguous without needing whitespace-sensitive parsing.
        GKind::LBrace => {
            cursor.advance();
            // 🔁 `parse_atom` -> `parse_alternatives` -> `parse_sequence` -> `parse_atom` is a
            // recursion cycle through fns (E0733: an fn's future can't be infinite-
            // sized) — `Box::pin` breaks it at this one edge, per R10 residue shape 3.
            let alts = parse_alternatives(cursor)?;
            cursor.expect(GKind::RBrace)?;
            Symbol::Group(alts)
        }
        GKind::Ident => {
            let name = cursor.advance().text;
            if cursor.peek().kind == GKind::LParen {
                Symbol::Macro(name, parse_macro_args(cursor)?)
            } else if is_all_upper(&name) {
                Symbol::Terminal(name)
            } else {
                Symbol::Ref(name)
            }
        }
        other => return Err(TextError::new(format!("expected a symbol, found {other:?}"), cursor.peek().span)),
    };
    let quantified = match cursor.peek().kind {
        GKind::Question => {
            cursor.advance();
            Symbol::Optional(Box::new(base))
        }
        GKind::Star => {
            cursor.advance();
            Symbol::Star(Box::new(base))
        }
        GKind::Plus => {
            cursor.advance();
            Symbol::Plus(Box::new(base))
        }
        _ => base,
    };
    Ok(quantified)
}

fn parse_sequence(cursor: &mut Cursor) -> Result<Alternative, TextError> {
    let mut symbols = Vec::new();
    loop {
        match cursor.peek().kind {
            GKind::Pipe | GKind::Newline | GKind::Eof | GKind::RBrace => break,
            _ => symbols.push(parse_atom(cursor)?),
        }
    }
    if symbols.is_empty() {
        return Err(TextError::new("a production alternative must have at least one symbol", cursor.peek().span));
    }
    Ok(Alternative { symbols })
}

fn parse_alternatives(cursor: &mut Cursor) -> Result<Vec<Alternative>, TextError> {
    let mut alts = vec![parse_sequence(cursor)?];
    while cursor.peek().kind == GKind::Pipe {
        cursor.advance();
        alts.push(parse_sequence(cursor)?);
    }
    Ok(alts)
}

fn parse_production_line(cursor: &mut Cursor) -> Result<Production, TextError> {
    let name = cursor.expect(GKind::Ident)?.text;
    cursor.expect(GKind::Equals)?;
    let alternatives = parse_alternatives(cursor)?;
    Ok(Production { name, alternatives })
}

/// @emoji 📖️ Parses one `.grammar` file. Protocol dialect sources project through
/// [`parse_protocol`] into a shallow [`GrammarFile`] (empty productions).
fn parse_grammar_id(cursor: &mut Cursor) -> Result<String, TextError> {
    let first = cursor.expect_ident_or_int()?;
    let mut id = first.text;
    if cursor.peek().kind == GKind::Ident {
        id.push_str(&cursor.advance().text);
    }
    Ok(id)
}

pub fn parse_grammar(text: &str) -> Result<GrammarFile, TextError> {
    if is_protocol_source(text) {
        return Ok(project_protocol(parse_protocol(text)?));
    }

    let tokens = lex(text)?;
    let mut cursor = Cursor { tokens, pos: 0 };
    cursor.skip_newlines();

    let dialect = if cursor.peek_ident("dialect") {
        cursor.expect_ident("dialect")?;
        let name = cursor.expect(GKind::Ident)?.text;
        cursor.skip_newlines();
        match name.as_str() {
            "grammar" => SemioDialect::Grammar,
            "protocol" => return Ok(project_protocol(parse_protocol(text)?)),
            other => return Err(TextError::new(format!("unknown semio dialect `{other}`"), cursor.peek().span)),
        }
    } else {
        SemioDialect::Grammar
    };

    cursor.expect_ident("grammar")?;
    let id = parse_grammar_id(&mut cursor)?;
    cursor.skip_newlines();

    let mut extension = None;
    let mut uses = Vec::new();
    let mut start = None;
    let mut productions = Vec::new();
    // P2-M1 items 1 & 4: per-grammar string quote+escape modes and comment dialect, declared via
    // optional `string`/`comment` header directives. Defaults (`comment_line = Some("#")`, no
    // block comment, no `string` overrides) reproduce `LexOptions::default()` exactly, so a
    // grammar that never declares either directive lexes byte-identically to before P2-M1.
    let mut comment_line: Option<String> = CommentDialect::default().line;
    let mut comment_block: Option<(String, String)> = None;
    let mut strings: Vec<StringMode> = Vec::new();

    loop {
        if cursor.peek().kind == GKind::Eof {
            break;
        }
        let head = cursor.expect(GKind::Ident)?;
        match head.text.as_str() {
            "extension" => {
                extension = Some(parse_grammar_id(&mut cursor)?);
                cursor.skip_newlines();
            }
            "use" => {
                uses.push(cursor.expect(GKind::Ident)?.text);
                cursor.skip_newlines();
            }
            "start" => {
                start = Some(cursor.expect(GKind::Ident)?.text);
                cursor.skip_newlines();
            }
            // `comment none` / `comment line "MARKER"` / `comment line none` / `comment block "OPEN" "CLOSE"`
            "comment" => {
                let sub = cursor.expect(GKind::Ident)?.text;
                match sub.as_str() {
                    "none" => {
                        comment_line = None;
                        comment_block = None;
                    }
                    "line" => {
                        if cursor.peek_ident("none") {
                            cursor.advance();
                            comment_line = None;
                        } else {
                            comment_line = Some(cursor.expect(GKind::Text)?.text);
                        }
                    }
                    "block" => {
                        let open = cursor.expect(GKind::Text)?.text;
                        let close = cursor.expect(GKind::Text)?.text;
                        comment_block = Some((open, close));
                    }
                    other => return Err(TextError::new(format!("unknown `comment` directive `{other}` (expected `none`/`line`/`block`)"), head.span)),
                }
                cursor.skip_newlines();
            }
            // `string double|single raw|backslash|doubled` — declaring ANY `string` directive
            // replaces the default double-quote-Raw quote set entirely (a grammar that wants both
            // delimiters, e.g. xml, declares both explicitly).
            "string" => {
                let which = cursor.expect(GKind::Ident)?.text;
                let quote = match which.as_str() {
                    "double" => '"',
                    "single" => '\'',
                    other => return Err(TextError::new(format!("unknown `string` quote `{other}` (expected `double`/`single`)"), head.span)),
                };
                let mode = cursor.expect(GKind::Ident)?.text;
                let escape = match mode.as_str() {
                    "raw" => StringEscape::Raw,
                    "backslash" => StringEscape::Backslash,
                    "doubled" => StringEscape::Doubled,
                    other => return Err(TextError::new(format!("unknown `string` escape mode `{other}` (expected `raw`/`backslash`/`doubled`)"), head.span)),
                };
                strings.push(StringMode { quote, escape });
                cursor.skip_newlines();
            }
            _ => {
                cursor.pos -= 1;
                productions.push(parse_production_line(&mut cursor)?);
                cursor.skip_newlines();
            }
        }
    }

    let _ = dialect;
    let start = match start {
        Some(s) => s,
        None => return Err(TextError::new("`.grammar` file is missing a `start` directive", cursor.peek().span)),
    };
    let lex = LexOptions { strings, comment: CommentDialect { line: comment_line, block: comment_block } };
    Ok(GrammarFile { dialect: SemioDialect::Grammar, id, extension, uses, start, productions, lex })
}

fn is_protocol_source(text: &str) -> bool {
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
            continue;
        }
        if trimmed.starts_with("dialect") {
            return trimmed.split_whitespace().nth(1) == Some("protocol");
        }
        return trimmed.starts_with("protocol");
    }
    false
}

fn project_protocol(protocol: ProtocolFile) -> GrammarFile {
    GrammarFile { dialect: SemioDialect::Protocol, id: protocol.id, extension: None, uses: protocol.uses, start: protocol.start, productions: Vec::new(), lex: LexOptions::default() }
}

fn parse_usize_token(token: &GToken) -> Result<usize, TextError> {
    token.text.parse::<usize>().map_err(|_| TextError::new(format!("expected unsigned integer, found `{}`", token.text), token.span))
}

fn parse_u64_literal(cursor: &mut Cursor) -> Result<u64, TextError> {
    let first = cursor.expect_ident_or_int()?;
    if first.kind == GKind::Int && first.text == "0" && cursor.peek().kind == GKind::Ident {
        let rest = cursor.peek().text.clone();
        if let Some(hex) = rest.strip_prefix('x').or_else(|| rest.strip_prefix('X')) {
            cursor.advance();
            return u64::from_str_radix(hex, 16).map_err(|_| TextError::new(format!("invalid hex literal `0{rest}`"), first.span));
        }
    }
    if let Some(hex) = first.text.strip_prefix("0x").or_else(|| first.text.strip_prefix("0X")) {
        return u64::from_str_radix(hex, 16).map_err(|_| TextError::new(format!("invalid hex literal `{}`", first.text), first.span));
    }
    first.text.parse::<u64>().map_err(|_| TextError::new(format!("expected unsigned integer, found `{}`", first.text), first.span))
}

fn parse_count(cursor: &mut Cursor) -> Result<Count, TextError> {
    match cursor.peek().kind {
        GKind::Ident if cursor.peek().text == "Varint" || cursor.peek().text == "varint" => {
            cursor.advance();
            Ok(Count::Varint)
        }
        GKind::Ident if cursor.peek().text == "Fixed" || cursor.peek().text == "fixed" => {
            cursor.advance();
            cursor.expect(GKind::LParen)?;
            let n = parse_usize_token(&cursor.expect_ident_or_int()?)?;
            cursor.expect(GKind::RParen)?;
            Ok(Count::Fixed(n))
        }
        GKind::Ident if cursor.peek().text == "Field" => {
            cursor.advance();
            cursor.expect(GKind::LParen)?;
            let name = cursor.expect(GKind::Ident)?.text;
            cursor.expect(GKind::RParen)?;
            Ok(Count::Field(name))
        }
        GKind::Int => Ok(Count::Fixed(parse_usize_token(&cursor.advance())?)),
        _ => Err(TextError::new("expected Array count (Fixed/Varint/Field)", cursor.peek().span)),
    }
}

fn parse_prim(cursor: &mut Cursor) -> Result<Prim, TextError> {
    match cursor.peek().kind {
        GKind::Ident => {
            let name = cursor.advance().text;
            match name.as_str() {
                "u8" => Ok(Prim::U8),
                "u16" => Ok(Prim::U16),
                "u32" => Ok(Prim::U32),
                "u64" => Ok(Prim::U64),
                "i32" => Ok(Prim::I32),
                "i64" => Ok(Prim::I64),
                "f32" => Ok(Prim::F32),
                "f64" => Ok(Prim::F64),
                "varint" => {
                    if cursor.peek_ident("bytes") {
                        cursor.advance();
                        Ok(Prim::Array(Box::new(Prim::U8), Count::Varint))
                    } else {
                        Ok(Prim::Varint)
                    }
                }
                "zigzag" => Ok(Prim::Zigzag),
                "bytes" => Ok(Prim::Bytes),
                "utf8" => Ok(Prim::Utf8),
                "tag" => Ok(Prim::Tag),
                "fixed" => {
                    let n = parse_usize_token(&cursor.expect_ident_or_int()?)?;
                    Ok(Prim::Fixed(n))
                }
                "Fixed" => {
                    cursor.expect(GKind::LParen)?;
                    let n = parse_usize_token(&cursor.expect_ident_or_int()?)?;
                    cursor.expect(GKind::RParen)?;
                    Ok(Prim::Fixed(n))
                }
                "array" => {
                    let inner = parse_prim(cursor)?;
                    Ok(Prim::Array(Box::new(inner), Count::Varint))
                }
                "Array" => {
                    cursor.expect(GKind::LParen)?;
                    let inner = parse_prim(cursor)?;
                    cursor.expect(GKind::Comma)?;
                    let count = parse_count(cursor)?;
                    cursor.expect(GKind::RParen)?;
                    Ok(Prim::Array(Box::new(inner), count))
                }
                "Ref" => {
                    cursor.expect(GKind::LParen)?;
                    let target = cursor.expect(GKind::Ident)?.text;
                    cursor.expect(GKind::RParen)?;
                    Ok(Prim::Ref(target))
                }
                // P2-M2 item 2: BE counterparts of the LE-hardcoded fixed-width numerics.
                "u16be" => Ok(Prim::U16Be),
                "u32be" => Ok(Prim::U32Be),
                "u64be" => Ok(Prim::U64Be),
                "i32be" => Ok(Prim::I32Be),
                "i64be" => Ok(Prim::I64Be),
                "f32be" => Ok(Prim::F32Be),
                "f64be" => Ok(Prim::F64Be),
                // P2-M2 item 1c: `marker(0xFF)` — scan past every 0xFF fill byte, then read the
                // next byte as the discriminator (JPG's marker-prefix scan variant mode).
                "marker" => {
                    cursor.expect(GKind::LParen)?;
                    let value = parse_u64_literal(cursor)?;
                    cursor.expect(GKind::RParen)?;
                    Ok(Prim::MarkerScan(value as u8))
                }
                // P2-M2 item 6: `endian { "II"=le "MM"=be }` — TIFF-style runtime endian marker.
                "endian" => {
                    cursor.expect(GKind::LBrace)?;
                    cursor.skip_newlines();
                    let mut arms = Vec::new();
                    while cursor.peek().kind != GKind::RBrace && cursor.peek().kind != GKind::Eof {
                        let key = cursor.expect(GKind::Text)?.text;
                        cursor.expect(GKind::Equals)?;
                        let mode = cursor.expect(GKind::Ident)?.text;
                        let be = match mode.as_str() {
                            "be" => true,
                            "le" => false,
                            other => return Err(TextError::new(format!("unknown `endian` mode `{other}` (expected `le`/`be`)"), cursor.peek().span)),
                        };
                        arms.push((key, be));
                        cursor.skip_newlines();
                    }
                    cursor.expect(GKind::RBrace)?;
                    Ok(Prim::Endian(arms))
                }
                other => Ok(Prim::Ref(other.to_string())),
            }
        }
        other => Err(TextError::new(format!("expected a protocol type, found {other:?}"), cursor.peek().span)),
    }
}

/// @emoji 🔀️ P2-M2 item 4: `if <field> <op> <value>` guard, `<op>` one of `eq|ne|lt|le|gt|ge`
/// (word-keyword operators rather than symbolic `==`/`<=` — this file's own local protocol lexer
/// only ever whitelists a small fixed token set, see the module doc; word keywords need zero lexer
/// changes and stay entirely inside this parser).
fn parse_cond(cursor: &mut Cursor) -> Result<Cond, TextError> {
    let field = cursor.expect(GKind::Ident)?.text;
    let op_word = cursor.expect(GKind::Ident)?.text;
    let op = match op_word.as_str() {
        "eq" => CondOp::Eq,
        "ne" => CondOp::Ne,
        "lt" => CondOp::Lt,
        "le" => CondOp::Le,
        "gt" => CondOp::Gt,
        "ge" => CondOp::Ge,
        other => return Err(TextError::new(format!("unknown condition operator `{other}` (expected eq/ne/lt/le/gt/ge)"), cursor.peek().span)),
    };
    let value = parse_u64_literal(cursor)?;
    Ok(Cond { field, op, value })
}

fn parse_field_pair(cursor: &mut Cursor) -> Result<Field, TextError> {
    let name = cursor.expect(GKind::Ident)?.text;
    let ty = parse_prim(cursor)?;
    let cond = if cursor.peek_ident("if") {
        cursor.advance();
        Some(parse_cond(cursor)?)
    } else {
        None
    };
    Ok(Field { name, ty, cond })
}

/// @emoji 🏷️ P2-M2 item 1: an arm/`until` tag literal — a `TEXT` literal encodes to its raw ASCII
/// bytes (PNG/GLB's 4-char chunk/type tags), an int/hex literal encodes big-endian, trimmed to the
/// discriminator prim's own byte width (GIF/JPG's single-byte introducer/marker codes).
fn parse_tag_value(cursor: &mut Cursor, discriminator: &Prim) -> Result<Vec<u8>, TextError> {
    if cursor.peek().kind == GKind::Text {
        Ok(cursor.advance().text.into_bytes())
    } else {
        let value = parse_u64_literal(cursor)?;
        let width = prim_fixed_width(discriminator).unwrap_or(1).clamp(1, 8);
        let be = value.to_be_bytes();
        Ok(be[8 - width..].to_vec())
    }
}

/// @emoji ✂️ Trims leading zero bytes off a `u64`'s big-endian representation (keeping at least one
/// byte) — used for `backward <name> magic 0x...` directives, matching `framing magic`'s existing
/// literal-to-bytes convention but without forcing a fixed 8-byte width.
fn trim_be_bytes(value: u64) -> Vec<u8> {
    let be = value.to_be_bytes();
    let first_nonzero = be.iter().position(|&b| b != 0).unwrap_or(7);
    be[first_nonzero..].to_vec()
}

/// @emoji 🌿️ Parses one `arm <tag> { field... | nested <name> <prim> { arm ... } }` body.
fn parse_arm_body(cursor: &mut Cursor) -> Result<(Vec<Field>, Option<NestedDispatch>), TextError> {
    cursor.expect(GKind::LBrace)?;
    cursor.skip_newlines();
    let mut fields = Vec::new();
    let mut nested = None;
    while cursor.peek().kind != GKind::RBrace && cursor.peek().kind != GKind::Eof {
        if cursor.peek_ident("nested") {
            cursor.advance();
            let name = cursor.expect(GKind::Ident)?.text;
            let discriminator = parse_prim(cursor)?;
            cursor.expect(GKind::LBrace)?;
            cursor.skip_newlines();
            let mut arms = Vec::new();
            while cursor.peek().kind != GKind::RBrace && cursor.peek().kind != GKind::Eof {
                cursor.expect_ident("arm")?;
                let tag = parse_tag_value(cursor, &discriminator)?;
                let (afields, anested) = parse_arm_body(cursor)?;
                arms.push(RepeatArm { tag, fields: afields, nested: anested });
                cursor.skip_newlines();
            }
            cursor.expect(GKind::RBrace)?;
            nested = Some(NestedDispatch { name, discriminator, arms });
        } else {
            fields.push(parse_field_pair(cursor)?);
        }
        cursor.skip_newlines();
    }
    cursor.expect(GKind::RBrace)?;
    Ok((fields, nested))
}

/// @emoji 🔁️ P2-M2 item 1: `repeat <name> { tag <prim> length <prim>? order length-first?
/// trailer <prim>? until <tag>? arm <tag> {...}* }`.
fn parse_repeat_dispatch(cursor: &mut Cursor) -> Result<RepeatDispatch, TextError> {
    cursor.expect(GKind::LBrace)?;
    cursor.skip_newlines();
    let mut discriminator: Option<Prim> = None;
    let mut length: Option<Prim> = None;
    let mut order = DispatchOrder::TagFirst;
    let mut trailer: Option<Prim> = None;
    let mut until: Option<Vec<u8>> = None;
    let mut arms = Vec::new();
    while cursor.peek().kind != GKind::RBrace && cursor.peek().kind != GKind::Eof {
        let head = cursor.expect(GKind::Ident)?;
        match head.text.as_str() {
            "tag" => discriminator = Some(parse_prim(cursor)?),
            "length" => length = Some(parse_prim(cursor)?),
            "order" => {
                let word = cursor.expect(GKind::Ident)?.text;
                order = match word.as_str() {
                    "tag-first" => DispatchOrder::TagFirst,
                    "length-first" => DispatchOrder::LengthFirst,
                    other => return Err(TextError::new(format!("unknown `repeat` order `{other}` (expected `tag-first`/`length-first`)"), head.span)),
                };
            }
            "trailer" => trailer = Some(parse_prim(cursor)?),
            "until" => {
                let disc = discriminator.as_ref().ok_or_else(|| TextError::new("`until` must follow `tag` in a `repeat` block", head.span))?;
                until = Some(parse_tag_value(cursor, disc)?);
            }
            "arm" => {
                let disc = discriminator.as_ref().ok_or_else(|| TextError::new("`arm` must follow `tag` in a `repeat` block", head.span))?;
                let tag = parse_tag_value(cursor, disc)?;
                let (fields, nested) = parse_arm_body(cursor)?;
                arms.push(RepeatArm { tag, fields, nested });
            }
            other => return Err(TextError::new(format!("unknown `repeat` directive `{other}`"), head.span)),
        }
        cursor.skip_newlines();
    }
    cursor.expect(GKind::RBrace)?;
    let discriminator = match discriminator {
        Some(d) => d,
        None => return Err(TextError::new("`repeat` block is missing a `tag` directive", cursor.peek().span)),
    };
    Ok(RepeatDispatch { discriminator, length, order, trailer, until, arms })
}

fn parse_fields_until_break(cursor: &mut Cursor) -> Result<Vec<Field>, TextError> {
    let mut fields = Vec::new();
    while matches!(cursor.peek().kind, GKind::Ident) {
        fields.push(parse_field_pair(cursor)?);
    }
    Ok(fields)
}

fn parse_braced_fields(cursor: &mut Cursor) -> Result<Vec<Field>, TextError> {
    cursor.expect(GKind::LBrace)?;
    cursor.skip_newlines();
    let mut fields = Vec::new();
    while cursor.peek().kind != GKind::RBrace && cursor.peek().kind != GKind::Eof {
        fields.push(parse_field_pair(cursor)?);
        cursor.skip_newlines();
    }
    cursor.expect(GKind::RBrace)?;
    Ok(fields)
}

fn parse_enum_variants(cursor: &mut Cursor) -> Result<Vec<(String, u64)>, TextError> {
    cursor.expect(GKind::LBrace)?;
    cursor.skip_newlines();
    let mut variants = Vec::new();
    while cursor.peek().kind != GKind::RBrace && cursor.peek().kind != GKind::Eof {
        let name = cursor.expect(GKind::Ident)?.text;
        cursor.expect(GKind::Equals)?;
        let value = parse_u64_literal(cursor)?;
        variants.push((name, value));
        if cursor.peek().kind == GKind::Comma {
            cursor.advance();
        }
        cursor.skip_newlines();
    }
    cursor.expect(GKind::RBrace)?;
    Ok(variants)
}

fn magic_bytes(value: u64) -> [u8; 8] {
    value.to_be_bytes()
}

fn flush_open_segment(blocks: &mut Vec<Block>, open: &mut Option<Block>) {
    if let Some(block) = open.take() {
        blocks.push(block);
    }
}

/// @emoji 📡️ Parses one `.protocol.semio` file into a typed [`ProtocolFile`] — retains every body
/// directive (`header`/`field`/`segment`/`record`/`struct`/`enum`/`footer`/`chain`).
pub fn parse_protocol(text: &str) -> Result<ProtocolFile, TextError> {
    let tokens = lex(text)?;
    let mut cursor = Cursor { tokens, pos: 0 };
    cursor.skip_newlines();

    if cursor.peek_ident("dialect") {
        cursor.expect_ident("dialect")?;
        cursor.expect_ident("protocol")?;
        cursor.skip_newlines();
    }

    cursor.expect_ident("protocol")?;
    let id = cursor.expect(GKind::Ident)?.text;
    cursor.skip_newlines();

    let mut version = 1u16;
    let mut schema = String::new();
    let mut start = None;
    let mut uses = Vec::new();
    let mut framing = None;
    let mut blocks = Vec::new();
    let mut open_segment: Option<Block> = None;
    let mut open_record: Option<Block> = None;
    let mut open_header: Option<Vec<Field>> = None;

    let close_header = |blocks: &mut Vec<Block>, open_header: &mut Option<Vec<Field>>| {
        if let Some(fields) = open_header.take() {
            blocks.push(Block::Header(fields));
        }
    };
    let close_record = |blocks: &mut Vec<Block>, open_record: &mut Option<Block>| {
        if let Some(block) = open_record.take() {
            blocks.push(block);
        }
    };

    loop {
        if cursor.peek().kind == GKind::Eof {
            break;
        }
        let head = cursor.expect(GKind::Ident)?;
        match head.text.as_str() {
            "version" => {
                version = parse_u64_literal(&mut cursor)? as u16;
                cursor.skip_newlines();
            }
            "schema" => {
                schema = cursor.expect(GKind::Ident)?.text;
                cursor.skip_newlines();
            }
            "use" => {
                uses.push(cursor.expect(GKind::Ident)?.text);
                cursor.skip_newlines();
            }
            "start" => {
                start = Some(cursor.expect(GKind::Ident)?.text);
                cursor.skip_newlines();
            }
            "framing" => {
                let mode = cursor.expect(GKind::Ident)?.text;
                framing = Some(match mode.as_str() {
                    "magic" => Framing::Magic(magic_bytes(parse_u64_literal(&mut cursor)?)),
                    "record" => Framing::Record,
                    "chunked" => Framing::Chunked,
                    other => return Err(TextError::new(format!("unknown framing `{other}`"), head.span)),
                });
                cursor.skip_newlines();
            }
            "header" => {
                flush_open_segment(&mut blocks, &mut open_segment);
                close_record(&mut blocks, &mut open_record);
                close_header(&mut blocks, &mut open_header);
                cursor.expect_ident("fixed")?;
                let _size = parse_u64_literal(&mut cursor)?;
                open_header = Some(Vec::new());
                cursor.skip_newlines();
            }
            "field" => {
                let field = parse_field_pair(&mut cursor)?;
                if let Some(fields) = open_header.as_mut() {
                    fields.push(field);
                } else if let Some(Block::Record { fields, .. }) = open_record.as_mut() {
                    fields.push(field);
                } else if let Some(Block::Segment { fields, .. }) = open_segment.as_mut() {
                    fields.push(field);
                } else if matches!(framing, Some(Framing::Record)) {
                    open_record = Some(Block::Record { name: String::new(), tag: None, fields: vec![field] });
                } else {
                    open_header.get_or_insert_with(Vec::new).push(field);
                }
                cursor.skip_newlines();
            }
            "segment" => {
                close_header(&mut blocks, &mut open_header);
                close_record(&mut blocks, &mut open_record);
                let name = cursor.expect(GKind::Ident)?.text;
                // P2-M2 item 4: whole-segment presence guard — `segment palette if bpp le 8 {...}`.
                let cond = if cursor.peek_ident("if") {
                    cursor.advance();
                    Some(parse_cond(&mut cursor)?)
                } else {
                    None
                };
                if cursor.peek_ident("kind") && cursor.tokens.get(cursor.pos + 1).is_some_and(|t| t.kind == GKind::Equals) {
                    flush_open_segment(&mut blocks, &mut open_segment);
                    cursor.expect_ident("kind")?;
                    cursor.expect(GKind::Equals)?;
                    let kind = parse_u64_literal(&mut cursor)? as u8;
                    let fields = if cursor.peek().kind == GKind::LBrace { parse_braced_fields(&mut cursor)? } else { Vec::new() };
                    blocks.push(Block::Segment { name, kind: Some(kind), fields, cond });
                } else if cursor.peek().kind == GKind::LBrace {
                    flush_open_segment(&mut blocks, &mut open_segment);
                    let fields = parse_braced_fields(&mut cursor)?;
                    blocks.push(Block::Segment { name, kind: None, fields, cond });
                } else {
                    let ty = parse_prim(&mut cursor)?;
                    match open_segment.as_mut() {
                        Some(Block::Segment { fields, .. }) => fields.push(Field { name, ty, cond: None }),
                        _ => {
                            open_segment = Some(Block::Segment { name: String::new(), kind: None, fields: vec![Field { name, ty, cond: None }], cond: None });
                        }
                    }
                }
                cursor.skip_newlines();
            }
            "record" => {
                close_header(&mut blocks, &mut open_header);
                flush_open_segment(&mut blocks, &mut open_segment);
                close_record(&mut blocks, &mut open_record);
                let name = cursor.expect(GKind::Ident)?.text;
                let mut tag = None;
                if cursor.peek_ident("tag") && cursor.tokens.get(cursor.pos + 1).is_some_and(|t| t.kind == GKind::Equals) {
                    cursor.expect_ident("tag")?;
                    cursor.expect(GKind::Equals)?;
                    tag = Some(parse_u64_literal(&mut cursor)?);
                }
                let fields = if cursor.peek().kind == GKind::LBrace { parse_braced_fields(&mut cursor)? } else { parse_fields_until_break(&mut cursor)? };
                open_record = Some(Block::Record { name, tag, fields });
                cursor.skip_newlines();
            }
            "struct" => {
                close_header(&mut blocks, &mut open_header);
                flush_open_segment(&mut blocks, &mut open_segment);
                close_record(&mut blocks, &mut open_record);
                let name = cursor.expect(GKind::Ident)?.text;
                let fields = parse_braced_fields(&mut cursor)?;
                blocks.push(Block::Struct { name, fields });
                cursor.skip_newlines();
            }
            "enum" => {
                close_header(&mut blocks, &mut open_header);
                flush_open_segment(&mut blocks, &mut open_segment);
                close_record(&mut blocks, &mut open_record);
                let name = cursor.expect(GKind::Ident)?.text;
                let variants = parse_enum_variants(&mut cursor)?;
                blocks.push(Block::Enum { name, variants });
                cursor.skip_newlines();
            }
            "footer" => {
                close_header(&mut blocks, &mut open_header);
                flush_open_segment(&mut blocks, &mut open_segment);
                close_record(&mut blocks, &mut open_record);
                cursor.expect_ident("fixed")?;
                let size = parse_u64_literal(&mut cursor)? as usize;
                blocks.push(Block::Footer(size));
                cursor.skip_newlines();
            }
            "chain" => {
                close_header(&mut blocks, &mut open_header);
                flush_open_segment(&mut blocks, &mut open_segment);
                close_record(&mut blocks, &mut open_record);
                if cursor.peek().kind == GKind::Ident {
                    let maybe_name = cursor.peek().text.clone();
                    let prim_names = ["u8", "u16", "u32", "u64", "i32", "i64", "f32", "f64", "varint", "zigzag", "bytes", "utf8", "tag", "fixed", "Fixed", "Array", "array", "Ref"];
                    if !prim_names.contains(&maybe_name.as_str()) {
                        cursor.advance();
                    }
                }
                let ty = parse_prim(&mut cursor)?;
                blocks.push(Block::Chain(ty));
                cursor.skip_newlines();
            }
            // P2-M2 item 1: `repeat <name> { tag <prim> ... arm <tag> {...} }`.
            "repeat" => {
                close_header(&mut blocks, &mut open_header);
                flush_open_segment(&mut blocks, &mut open_segment);
                close_record(&mut blocks, &mut open_record);
                let name = cursor.expect(GKind::Ident)?.text;
                let dispatch = parse_repeat_dispatch(&mut cursor)?;
                blocks.push(Block::Repeat { name, dispatch });
                cursor.skip_newlines();
            }
            // P2-M2 item 5a: `backward <name> magic 0x... {...}` — scan backward from EOF.
            "backward" => {
                close_header(&mut blocks, &mut open_header);
                flush_open_segment(&mut blocks, &mut open_segment);
                close_record(&mut blocks, &mut open_record);
                let name = cursor.expect(GKind::Ident)?.text;
                cursor.expect_ident("magic")?;
                let magic = trim_be_bytes(parse_u64_literal(&mut cursor)?);
                let fields = parse_braced_fields(&mut cursor)?;
                blocks.push(Block::BackwardScan { name, magic, fields });
                cursor.skip_newlines();
            }
            // P2-M2 item 5b: `jump <name> from <field> {...}` — absolute-offset jump.
            "jump" => {
                close_header(&mut blocks, &mut open_header);
                flush_open_segment(&mut blocks, &mut open_segment);
                close_record(&mut blocks, &mut open_record);
                let name = cursor.expect(GKind::Ident)?.text;
                cursor.expect_ident("from")?;
                let offset_field = cursor.expect(GKind::Ident)?.text;
                let fields = parse_braced_fields(&mut cursor)?;
                blocks.push(Block::JumpTo { name, offset_field, fields });
                cursor.skip_newlines();
            }
            other => return Err(TextError::new(format!("unknown protocol directive `{other}`"), head.span)),
        }
    }

    close_header(&mut blocks, &mut open_header);
    flush_open_segment(&mut blocks, &mut open_segment);
    close_record(&mut blocks, &mut open_record);

    let start = match start {
        Some(s) => s,
        None => return Err(TextError::new("`.protocol` file is missing a `start` directive", cursor.peek().span)),
    };
    let framing = match framing {
        Some(f) => f,
        None => return Err(TextError::new("`.protocol` file is missing a `framing` directive", cursor.peek().span)),
    };
    if schema.is_empty() {
        return Err(TextError::new("`.protocol` file is missing a `schema` directive", cursor.peek().span));
    }
    Ok(ProtocolFile { id, version, schema, start, uses, framing, blocks })
}
//#endregion 🔖️Parser

//#region 🔖️Writer
fn print_symbol(symbol: &Symbol, out: &mut String) {
    match symbol {
        Symbol::Literal(text) => {
            out.push('"');
            out.push_str(text);
            out.push('"');
        }
        Symbol::Terminal(name) | Symbol::Ref(name) => out.push_str(name),
        Symbol::Macro(name, args) => {
            out.push_str(name);
            out.push('(');
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                match arg {
                    MacroArg::Literal(text) => {
                        out.push('"');
                        out.push_str(text);
                        out.push('"');
                    }
                    MacroArg::Ident(name) => out.push_str(name),
                }
            }
            out.push(')');
        }
        Symbol::Group(alts) => {
            out.push('{');
            // 🔁️ Box::pin: print_symbol <-> print_alternatives is a mutual-recursion cycle, and
            // an fn's own opaque Future type cannot embed a cycle-partner's opaque type at
            // an unboxed, unbounded size (R10 residue shape 3).
            print_alternatives(alts, out);
            out.push('}');
        }
        Symbol::Optional(inner) => {
            print_symbol(inner, out);
            out.push('?');
        }
        Symbol::Star(inner) => {
            print_symbol(inner, out);
            out.push('*');
        }
        Symbol::Plus(inner) => {
            print_symbol(inner, out);
            out.push('+');
        }
    }
}

fn print_alternatives(alts: &[Alternative], out: &mut String) {
    for (i, alt) in alts.iter().enumerate() {
        if i > 0 {
            out.push_str(" | ");
        }
        for (j, symbol) in alt.symbols.iter().enumerate() {
            if j > 0 {
                out.push(' ');
            }
            print_symbol(symbol, out);
        }
    }
}

/// @emoji 🖨️ Canonical printer — `parse_grammar(print_grammar(g)) == g` is this crate's own
/// round-trip law, checked by the `self_hosting` test below over this crate's own grammar file.
pub fn print_grammar(grammar: &GrammarFile) -> String {
    let mut out = String::new();
    out.push_str("dialect ");
    out.push_str(match grammar.dialect {
        SemioDialect::Grammar => "grammar",
        SemioDialect::Protocol => "protocol",
    });
    out.push('\n');
    out.push_str(match grammar.dialect {
        SemioDialect::Grammar => "grammar ",
        SemioDialect::Protocol => "protocol ",
    });
    out.push_str(&grammar.id);
    out.push('\n');
    if let Some(extension) = &grammar.extension {
        out.push_str("extension ");
        out.push_str(extension);
        out.push('\n');
    }
    // P2-M1 items 1 & 4: only emitted when they differ from `LexOptions::default()`, so a grammar
    // that never declared `comment`/`string` directives prints byte-identically to before P2-M1.
    let default_comment = CommentDialect::default();
    if grammar.lex.comment.line != default_comment.line {
        match &grammar.lex.comment.line {
            Some(marker) => {
                out.push_str("comment line \"");
                out.push_str(marker);
                out.push_str("\"\n");
            }
            None => out.push_str("comment line none\n"),
        }
    }
    if let Some((open, close)) = &grammar.lex.comment.block {
        out.push_str("comment block \"");
        out.push_str(open);
        out.push_str("\" \"");
        out.push_str(close);
        out.push_str("\"\n");
    }
    for mode in &grammar.lex.strings {
        let which = match mode.quote {
            '"' => "double",
            '\'' => "single",
            _ => continue,
        };
        let escape = match mode.escape {
            StringEscape::Raw => "raw",
            StringEscape::Backslash => "backslash",
            StringEscape::Doubled => "doubled",
        };
        out.push_str("string ");
        out.push_str(which);
        out.push(' ');
        out.push_str(escape);
        out.push('\n');
    }
    for fragment in &grammar.uses {
        out.push_str("use ");
        out.push_str(fragment);
        out.push('\n');
    }
    out.push_str("start ");
    out.push_str(&grammar.start);
    out.push('\n');
    for production in &grammar.productions {
        out.push_str(&production.name);
        out.push_str(" = ");
        print_alternatives(&production.alternatives, &mut out);
        out.push('\n');
    }
    out
}

fn print_count(count: &Count, out: &mut String) {
    match count {
        Count::Fixed(n) => {
            out.push_str("Fixed(");
            out.push_str(&n.to_string());
            out.push(')');
        }
        Count::Varint => out.push_str("Varint"),
        Count::Field(name) => {
            out.push_str("Field(");
            out.push_str(name);
            out.push(')');
        }
    }
}

fn print_prim(prim: &Prim, out: &mut String) {
    match prim {
        Prim::U8 => out.push_str("u8"),
        Prim::U16 => out.push_str("u16"),
        Prim::U32 => out.push_str("u32"),
        Prim::U64 => out.push_str("u64"),
        Prim::I32 => out.push_str("i32"),
        Prim::I64 => out.push_str("i64"),
        Prim::F32 => out.push_str("f32"),
        Prim::F64 => out.push_str("f64"),
        Prim::Varint => out.push_str("varint"),
        Prim::Zigzag => out.push_str("zigzag"),
        Prim::Bytes => out.push_str("bytes"),
        Prim::Utf8 => out.push_str("utf8"),
        Prim::Tag => out.push_str("tag"),
        Prim::Fixed(n) => {
            out.push_str("fixed ");
            out.push_str(&n.to_string());
        }
        Prim::Array(inner, count) => {
            if matches!(inner.as_ref(), Prim::U8) && matches!(count, Count::Varint) {
                out.push_str("varint bytes");
            } else if matches!(count, Count::Varint) {
                out.push_str("array ");
                // 🔁️ Box::pin: self-recursive fn (R10 residue shape 3).
                print_prim(inner, out);
            } else {
                out.push_str("Array(");
                // 🔁️ Box::pin: self-recursive fn (R10 residue shape 3).
                print_prim(inner, out);
                out.push_str(", ");
                print_count(count, out);
                out.push(')');
            }
        }
        Prim::Ref(name) => out.push_str(name),
        Prim::U16Be => out.push_str("u16be"),
        Prim::U32Be => out.push_str("u32be"),
        Prim::U64Be => out.push_str("u64be"),
        Prim::I32Be => out.push_str("i32be"),
        Prim::I64Be => out.push_str("i64be"),
        Prim::F32Be => out.push_str("f32be"),
        Prim::F64Be => out.push_str("f64be"),
        Prim::MarkerScan(prefix) => out.push_str(&format!("marker(0x{prefix:02X})")),
        Prim::Endian(arms) => {
            out.push_str("endian {");
            for (i, (key, be)) in arms.iter().enumerate() {
                if i > 0 {
                    out.push(' ');
                }
                out.push('"');
                out.push_str(key);
                out.push_str("\"=");
                out.push_str(if *be { "be" } else { "le" });
            }
            out.push('}');
        }
    }
}

fn print_cond(cond: &Cond, out: &mut String) {
    out.push_str(" if ");
    out.push_str(&cond.field);
    out.push(' ');
    out.push_str(match cond.op {
        CondOp::Eq => "eq",
        CondOp::Ne => "ne",
        CondOp::Lt => "lt",
        CondOp::Le => "le",
        CondOp::Gt => "gt",
        CondOp::Ge => "ge",
    });
    out.push(' ');
    out.push_str(&cond.value.to_string());
}

/// @emoji 🏷️ Prints raw discriminator/magic bytes back to source: printable multi-byte ASCII
/// round-trips as a `TEXT` literal (PNG/GLB's 4-char tags stay readable), anything else as a hex
/// integer literal — matches [`parse_tag_value`]'s two accepted input forms exactly.
fn print_tag_bytes(tag: &[u8], out: &mut String) {
    let printable = tag.len() > 1 && tag.iter().all(|b| b.is_ascii_graphic() || *b == b' ');
    if printable {
        out.push('"');
        out.push_str(&String::from_utf8_lossy(tag));
        out.push('"');
    } else {
        out.push_str("0x");
        for b in tag {
            out.push_str(&format!("{b:02X}"));
        }
    }
}

fn print_repeat_arm(arm: &RepeatArm, out: &mut String) {
    out.push_str("arm ");
    print_tag_bytes(&arm.tag, out);
    out.push_str(" {");
    for (i, field) in arm.fields.iter().enumerate() {
        if i > 0 {
            out.push(' ');
        }
        print_field(field, out);
    }
    if let Some(nested) = &arm.nested {
        if !arm.fields.is_empty() {
            out.push(' ');
        }
        out.push_str("nested ");
        out.push_str(&nested.name);
        out.push(' ');
        print_prim(&nested.discriminator, out);
        out.push_str(" {");
        for (i, narm) in nested.arms.iter().enumerate() {
            if i > 0 {
                out.push(' ');
            }
            // 🔁️ Box::pin: self-recursive fn (R10 residue shape 3).
            print_repeat_arm(narm, out);
        }
        out.push('}');
    }
    out.push_str("}\n");
}

fn print_field(field: &Field, out: &mut String) {
    out.push_str(&field.name);
    out.push(' ');
    print_prim(&field.ty, out);
    if let Some(cond) = &field.cond {
        print_cond(cond, out);
    }
}

fn header_fixed_size(fields: &[Field]) -> usize {
    let mut total = 0;
    for f in fields {
        total += prim_fixed_width(&f.ty).unwrap_or(0);
    }
    total
}

/// @emoji 🖨️ Lossless protocol printer — `parse_protocol(print_protocol(p)) == p`.
pub fn print_protocol(protocol: &ProtocolFile) -> String {
    let mut out = String::new();
    out.push_str("dialect protocol\nprotocol ");
    out.push_str(&protocol.id);
    out.push('\n');
    out.push_str("version ");
    out.push_str(&protocol.version.to_string());
    out.push('\n');
    out.push_str("schema ");
    out.push_str(&protocol.schema);
    out.push('\n');
    out.push_str("start ");
    out.push_str(&protocol.start);
    out.push('\n');
    for fragment in &protocol.uses {
        out.push_str("use ");
        out.push_str(fragment);
        out.push('\n');
    }
    match &protocol.framing {
        Framing::Magic(bytes) => {
            let value = u64::from_be_bytes(*bytes);
            out.push_str(&format!("framing magic 0x{value:016X}\n"));
        }
        Framing::Record => out.push_str("framing record\n"),
        Framing::Chunked => out.push_str("framing chunked\n"),
    }
    for block in &protocol.blocks {
        match block {
            Block::Header(fields) => {
                let size = header_fixed_size(fields);
                out.push_str("header fixed ");
                out.push_str(&size.to_string());
                out.push('\n');
                for field in fields {
                    out.push_str("field ");
                    print_field(field, &mut out);
                    out.push('\n');
                }
            }
            Block::Segment { name, kind, fields, cond } => {
                if name.is_empty() && kind.is_none() && cond.is_none() {
                    for field in fields {
                        out.push_str("segment ");
                        print_field(field, &mut out);
                        out.push('\n');
                    }
                } else {
                    out.push_str("segment ");
                    out.push_str(name);
                    if let Some(k) = kind {
                        out.push_str(" kind=");
                        out.push_str(&k.to_string());
                    }
                    if let Some(c) = cond {
                        print_cond(c, &mut out);
                    }
                    out.push_str(" {");
                    for (i, field) in fields.iter().enumerate() {
                        if i > 0 {
                            out.push(' ');
                        }
                        print_field(field, &mut out);
                    }
                    out.push_str("}\n");
                }
            }
            Block::Record { name, tag, fields } => {
                if name.is_empty() && tag.is_none() {
                    for field in fields {
                        out.push_str("field ");
                        print_field(field, &mut out);
                        out.push('\n');
                    }
                } else {
                    out.push_str("record ");
                    out.push_str(name);
                    if let Some(t) = tag {
                        out.push_str(" tag=");
                        out.push_str(&t.to_string());
                    }
                    out.push(' ');
                    for (i, field) in fields.iter().enumerate() {
                        if i > 0 {
                            out.push(' ');
                        }
                        print_field(field, &mut out);
                    }
                    out.push('\n');
                }
            }
            Block::Struct { name, fields } => {
                out.push_str("struct ");
                out.push_str(name);
                out.push_str(" {");
                for (i, field) in fields.iter().enumerate() {
                    if i > 0 {
                        out.push(' ');
                    }
                    print_field(field, &mut out);
                }
                out.push_str("}\n");
            }
            Block::Enum { name, variants } => {
                out.push_str("enum ");
                out.push_str(name);
                out.push_str(" {");
                for (i, (vname, value)) in variants.iter().enumerate() {
                    if i > 0 {
                        out.push(' ');
                    }
                    out.push_str(vname);
                    out.push('=');
                    out.push_str(&value.to_string());
                }
                out.push_str("}\n");
            }
            Block::Footer(size) => {
                out.push_str("footer fixed ");
                out.push_str(&size.to_string());
                out.push('\n');
            }
            Block::Chain(prim) => {
                out.push_str("chain ");
                print_prim(prim, &mut out);
                out.push('\n');
            }
            Block::Repeat { name, dispatch } => {
                out.push_str("repeat ");
                out.push_str(name);
                out.push_str(" {\n");
                out.push_str("tag ");
                print_prim(&dispatch.discriminator, &mut out);
                out.push('\n');
                if let Some(length) = &dispatch.length {
                    out.push_str("length ");
                    print_prim(length, &mut out);
                    out.push('\n');
                }
                if matches!(dispatch.order, DispatchOrder::LengthFirst) {
                    out.push_str("order length-first\n");
                }
                if let Some(trailer) = &dispatch.trailer {
                    out.push_str("trailer ");
                    print_prim(trailer, &mut out);
                    out.push('\n');
                }
                if let Some(until) = &dispatch.until {
                    out.push_str("until ");
                    print_tag_bytes(until, &mut out);
                    out.push('\n');
                }
                for arm in &dispatch.arms {
                    print_repeat_arm(arm, &mut out);
                }
                out.push_str("}\n");
            }
            Block::BackwardScan { name, magic, fields } => {
                out.push_str("backward ");
                out.push_str(name);
                out.push_str(" magic ");
                print_tag_bytes(magic, &mut out);
                out.push_str(" {");
                for (i, field) in fields.iter().enumerate() {
                    if i > 0 {
                        out.push(' ');
                    }
                    print_field(field, &mut out);
                }
                out.push_str("}\n");
            }
            Block::JumpTo { name, offset_field, fields } => {
                out.push_str("jump ");
                out.push_str(name);
                out.push_str(" from ");
                out.push_str(offset_field);
                out.push_str(" {");
                for (i, field) in fields.iter().enumerate() {
                    if i > 0 {
                        out.push(' ');
                    }
                    print_field(field, &mut out);
                }
                out.push_str("}\n");
            }
        }
    }
    out
}

/// @emoji ♻️ `canonicalize(canonicalize(x)) == canonicalize(x)` — the idempotence law every
/// technology's canonical form must satisfy.
pub fn canonicalize(text: &str) -> Result<String, TextError> {
    if is_protocol_source(text) {
        Ok(print_protocol(&parse_protocol(text)?))
    } else {
        Ok(print_grammar(&parse_grammar(text)?))
    }
}
//#endregion 🔖️Writer

//#region 🔖️FromRecordSpec
// 🗑️ `from_record_spec` hatch deleted — handcrafted `.grammar.semio` / `.protocol.semio` are normative.
//#endregion 🔖️FromRecordSpec

//#region 🔖️Recognizer
/// @emoji 🧭️ Recognizer with explicit terminal predicates, family fragment merge, and macro matchers.
pub struct MacroMatcher {
    pub name: &'static str,
    pub try_match: fn(&str) -> bool,
}

/// @emoji 🧩️ Named grammar fragments (family kits) merged into Recognizer::compile_with.
#[derive(Default, Clone)]
pub struct FragmentRegistry {
    fragments: std::collections::HashMap<String, GrammarFile>,
}

impl FragmentRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builtin() -> Self {
        let mut reg = Self::new();
        if let Ok(g) = parse_grammar(include_str!("../👪️family/🌍️geo/📖️.grammar.semio")) {
            reg.insert("family-geo", g);
        }
        if let Ok(g) = parse_grammar(include_str!("../👪️family/📎️embed/📖️.grammar.semio")) {
            reg.insert("family-embed", g);
        }
        if let Ok(g) = parse_grammar(include_str!("../👪️family/🕸️graph/📖️.grammar.semio")) {
            reg.insert("family-graph", g);
        }
        if let Ok(g) = parse_grammar(include_str!("../👪️family/📊️sheet/📖️.grammar.semio")) {
            reg.insert("family-sheet", g);
        }
        if let Ok(g) = parse_grammar(include_str!("../👪️family/🗂️catalog/📖️.grammar.semio")) {
            reg.insert("family-catalog", g);
        }
        if let Ok(g) = parse_grammar(include_str!("../👪️family/🎬️scene/📖️.grammar.semio")) {
            reg.insert("family-scene", g);
        }
        if let Ok(g) = parse_grammar(include_str!("../👪️family/🧑‍🍳recipe/📖️.grammar.semio")) {
            reg.insert("family-recipe", g);
        }
        reg
    }

    pub fn insert(&mut self, name: impl Into<String>, grammar: GrammarFile) {
        self.fragments.insert(name.into(), grammar);
    }

    pub fn get(&self, name: &str) -> Option<&GrammarFile> {
        self.fragments.get(name)
    }
}

pub struct Recognizer {
    grammar: GrammarFile,
    macros: Vec<MacroMatcher>,
}

impl Recognizer {
    pub fn compile(grammar: &GrammarFile) -> Self {
        let registry = FragmentRegistry::builtin();
        Self::compile_with(grammar, &registry)
    }

    /// @emoji 🔗️ Compile grammar, merging productions from each use via registry.
    pub fn compile_with(grammar: &GrammarFile, registry: &FragmentRegistry) -> Self {
        let mut merged = grammar.clone();
        let mut seen = std::collections::HashSet::<String>::new();
        for p in &grammar.productions {
            seen.insert(p.name.clone());
        }
        for use_name in &grammar.uses {
            if let Some(frag) = registry.get(use_name) {
                for prod in &frag.productions {
                    if seen.insert(prod.name.clone()) {
                        merged.productions.push(prod.clone());
                    }
                }
            }
        }
        Self { grammar: merged, macros: default_macros() }
    }

    fn find_production(&self, name: &str) -> Option<&Production> {
        self.grammar.productions.iter().find(|p| p.name == name)
    }

    /// @emoji ✅️ Recognizes text against the grammar start production. Lexes with this grammar's
    /// own `lex` dialect (P2-M1: per-grammar string/comment configuration), so a grammar that
    /// declared `comment`/`string` header directives recognizes text under ITS OWN alphabet.
    /// Lexes forgivingly (P2-M1 item 2): a raw-span terminal (`LINE`/`REST`) exists precisely to
    /// swallow content outside the fixed token alphabet (txt's arbitrary prose body) without the
    /// WHOLE document failing to lex before the Recognizer ever sees a token stream to walk over —
    /// resource-limit violations (`Limits`) still surface as a real `Err`, only lexical-shape
    /// errors degrade to `Error` tokens. Behavior-identical to strict mode for any text that
    /// already lexed cleanly (every pre-M1 pilot fixture), since forgiving vs. strict only differ
    /// on inputs that would otherwise abort with a lex error.
    pub fn recognize(&self, text: &str) -> Result<bool, TextError> {
        let raw = core_lex_with(text, &Limits::default(), true, &self.grammar.lex)?;
        let tokens: Vec<_> = raw.into_iter().filter(|t| !t.kind.is_trivia() && t.kind != CoreKind::Eof).collect();
        let start = self.find_production(&self.grammar.start).ok_or_else(|| TextError::new(format!("start production `{}` not found", self.grammar.start), TextSpan::at(1, 1)))?;
        match self.match_production(start, &tokens, 0, text) {
            Some(pos) => Ok(pos == tokens.len()),
            None => Ok(false),
        }
    }

    /// @emoji 📊️ Productions never reached while recognizing text. Forgiving for the same reason
    /// as [`Recognizer::recognize`] (P2-M1 raw-span support).
    pub fn uncovered_productions(&self, text: &str) -> Result<Vec<String>, TextError> {
        let raw = core_lex_with(text, &Limits::default(), true, &self.grammar.lex)?;
        let tokens: Vec<_> = raw.into_iter().filter(|t| !t.kind.is_trivia() && t.kind != CoreKind::Eof).collect();
        let mut covered = std::collections::HashSet::<String>::new();
        let start = self.find_production(&self.grammar.start).ok_or_else(|| TextError::new(format!("start production `{}` not found", self.grammar.start), TextSpan::at(1, 1)))?;
        let _ = self.match_production_tracked(start, &tokens, 0, &mut covered, text);
        Ok(self.grammar.productions.iter().map(|p| p.name.clone()).filter(|n| !covered.contains(n)).collect())
    }

    fn match_production(&self, production: &Production, tokens: &[crate::os_dsl::SpannedToken], pos: usize, text: &str) -> Option<usize> {
        let mut covered = std::collections::HashSet::new();
        self.match_production_tracked(production, tokens, pos, &mut covered, text)
    }

    fn match_production_tracked(&self, production: &Production, tokens: &[crate::os_dsl::SpannedToken], pos: usize, covered: &mut std::collections::HashSet<String>, text: &str) -> Option<usize> {
        for alt in &production.alternatives {
            if let Some(next) = self.match_sequence_tracked(&alt.symbols, tokens, pos, covered, text) {
                covered.insert(production.name.clone());
                return Some(next);
            }
        }
        None
    }

    fn match_sequence_tracked(&self, symbols: &[Symbol], tokens: &[crate::os_dsl::SpannedToken], mut pos: usize, covered: &mut std::collections::HashSet<String>, text: &str) -> Option<usize> {
        for symbol in symbols {
            pos = self.match_symbol_tracked(symbol, tokens, pos, covered, text)?;
        }
        Some(pos)
    }

    fn match_symbol_tracked(&self, symbol: &Symbol, tokens: &[crate::os_dsl::SpannedToken], pos: usize, covered: &mut std::collections::HashSet<String>, text: &str) -> Option<usize> {
        match symbol {
            Symbol::Literal(literal) => {
                let token = tokens.get(pos)?;
                (token.text.as_str().as_ref() == literal.as_str()).then_some(pos + 1)
            }
            // P2-M1 item 2: the "raw span" terminal. `LINE`/`REST` don't match ONE token via
            // `terminal_matches` — they consume a byte SPAN of the original source (read straight
            // from `text`, not reassembled from tokens, so interior whitespace/punctuation the
            // shared lexer would otherwise fragment is preserved verbatim) and skip forward past
            // every already-lexed token that span swallowed, without attempting to re-tokenize it.
            Symbol::Terminal(name) if matches!(name.to_uppercase().as_str(), "LINE" | "REST") => {
                let end = match name.to_uppercase().as_str() {
                    "LINE" => RawSpanEnd::Newline,
                    _ => RawSpanEnd::Eof,
                };
                Some(match_raw_span(tokens, pos, text, end))
            }
            Symbol::Terminal(name) => {
                let token = tokens.get(pos)?;
                terminal_matches(name, token).then_some(pos + 1)
            }
            Symbol::Ref(name) => {
                if let Some(production) = self.find_production(name) {
                    // 🔁 Breaks the `match_symbol_tracked` -> `match_production_tracked` ->
                    // `match_sequence_tracked` -> `match_symbol_tracked` async recursion cycle
                    // (E0733: an fn's future can't be infinite-sized) at this one edge.
                    self.match_production_tracked(production, tokens, pos, covered, text)
                } else if let Some(matcher) = self.macros.iter().find(|m| m.name == name) {
                    self.match_macro_span(matcher, tokens, pos)
                } else {
                    None
                }
            }
            Symbol::Macro(name, _args) => {
                let matcher = self.macros.iter().find(|m| m.name == name)?;
                self.match_macro_span(matcher, tokens, pos)
            }
            Symbol::Group(alts) => {
                // 🔁 Was `.find_map(|alt| self.match_sequence_tracked(...))` — `Iterator::find_map`'s
                // closure can't `` (residue shape 1), so this is a manual loop instead.
                let mut found = None;
                for alt in alts {
                    if let Some(next) = self.match_sequence_tracked(&alt.symbols, tokens, pos, covered, text) {
                        found = Some(next);
                        break;
                    }
                }
                found
            }
            Symbol::Optional(inner) => Some(self.match_symbol_tracked(inner, tokens, pos, covered, text).unwrap_or(pos)),
            Symbol::Star(inner) => {
                let mut cur = pos;
                while let Some(next) = self.match_symbol_tracked(inner, tokens, cur, covered, text) {
                    if next == cur {
                        break;
                    }
                    cur = next;
                }
                Some(cur)
            }
            Symbol::Plus(inner) => {
                let first = self.match_symbol_tracked(inner, tokens, pos, covered, text)?;
                let mut cur = first;
                loop {
                    match self.match_symbol_tracked(inner, tokens, cur, covered, text) {
                        Some(next) if next != cur => cur = next,
                        _ => break,
                    }
                }
                Some(cur)
            }
        }
    }

    /// @emoji 🔙️ Tries the LARGEST token span first, shrinking down to (and including, P2-P1) a
    /// zero-width match — real backtracking, unlike `Symbol::Star`'s single-pass greedy loop, which
    /// is why a macro is the right escape hatch for a Ref that must stop short of a following
    /// literal it would otherwise swallow (see the `hex` macro's own doc comment). The zero-width
    /// floor (`pos..` rather than `pos + 1..`) only ever fires for a matcher whose `try_match`
    /// accepts the empty string — `hex`'s does (an empty hex-encoded value is valid) — so this is a
    /// strict widening: every pre-existing macro's behavior on non-empty spans is unchanged.
    fn match_macro_span(&self, matcher: &MacroMatcher, tokens: &[crate::os_dsl::SpannedToken], pos: usize) -> Option<usize> {
        for end in (pos..=tokens.len()).rev() {
            let slice_text = slice_source_text(&tokens[pos..end]);
            if (matcher.try_match)(&slice_text) {
                return Some(end);
            }
        }
        None
    }
}

/// @emoji 📏️ Which delimiter ends a P2-M1 "raw span" terminal capture.
enum RawSpanEnd {
    /// `LINE` — rest of the current physical line (up to the next `\n`, or EOF if none) — obj's
    /// `o`/`g` names, stl's `solid <name>`/`endsolid <name>`, dxf's opaque group-code value lines.
    Newline,
    /// `REST` — everything remaining to end-of-document — txt's whole prose body.
    Eof,
}

/// @emoji ✂️ Captures a raw byte span of the ORIGINAL source starting at `tokens[pos]`'s byte
/// offset (or end-of-text if `pos` is already past the last token) through `end`, then returns the
/// token index just past every token that span swallowed — the span's interior is never
/// re-tokenized, matching the shared lexer's own token boundaries only at the far edge.
fn match_raw_span(tokens: &[crate::os_dsl::SpannedToken], pos: usize, text: &str, end: RawSpanEnd) -> usize {
    let start_byte = tokens.get(pos).map_or(text.len(), |t| t.byte_range.0 as usize);
    let end_byte = match end {
        RawSpanEnd::Newline => text.get(start_byte..).and_then(|rest| rest.find('\n')).map_or(text.len(), |off| start_byte + off),
        RawSpanEnd::Eof => text.len(),
    };
    let mut new_pos = pos;
    while new_pos < tokens.len() && (tokens[new_pos].byte_range.0 as usize) < end_byte {
        new_pos += 1;
    }
    new_pos
}

fn slice_source_text(tokens: &[crate::os_dsl::SpannedToken]) -> String {
    tokens.iter().map(|t| t.text.as_str().to_string()).collect::<Vec<_>>().join(" ")
}

/// @emoji 🏷️ Explicit terminal predicates — BOOL is Ident true|false.
fn terminal_matches(name: &str, token: &crate::os_dsl::SpannedToken) -> bool {
    let upper = name.to_uppercase();
    let text = token.text.as_str();
    let text = text.as_ref();
    match upper.as_str() {
        "BOOL" => matches!(token.kind, CoreKind::Ident) && (text == "true" || text == "false"),
        "IDENT" | "PLACEHOLDER" => matches!(token.kind, CoreKind::Ident | CoreKind::Placeholder),
        "INT" => matches!(token.kind, CoreKind::Int),
        "FLOAT" => matches!(token.kind, CoreKind::Float),
        "TEXT" | "STRING" => matches!(token.kind, CoreKind::Text),
        "STAR" => matches!(token.kind, CoreKind::Star),
        "PLUS" => matches!(token.kind, CoreKind::Plus),
        "EQUALS" | "EQ" => matches!(token.kind, CoreKind::Equals) || text == "=",
        "ARROW" => text == "->" || text == "→",
        "DASHARROW" => text == "-->" || text == "⟶",
        "BACKARROW" => text == "<-" || text == "←",
        "EDGEARROW" => text == "<->" || text == "<-->" || text == "↔",
        "QUANTITY" => matches!(token.kind, CoreKind::Float | CoreKind::Int),
        "VEC3" | "COLOR" | "POINT" | "UNIT" => {
            matches!(token.kind, CoreKind::Ident | CoreKind::Float | CoreKind::Int | CoreKind::Text)
        }
        // P2-M1 item 3/5: the promoted single-char tokens + STEP's leading-dot enum literal each
        // get an explicit named terminal so a grammar can require them positionally (not just via
        // the generic `other` token-kind-name fallback below, though that still works too since
        // e.g. `Symbol::Terminal("LT")` already matches `format!("{:?}", TokenKind::Lt)`).
        "DOTENUM" => matches!(token.kind, CoreKind::DotEnum),
        other => format!("{:?}", token.kind).to_uppercase() == other,
    }
}

// 🚫️async: E4 fn-pointer slot — `MacroMatcher.try_match` is a bare `fn(&str) -> bool` — see R2 E4.
fn macro_table_ok(text: &str) -> bool {
    let t = text.trim();
    t.contains('|') || t.starts_with("table")
}

// 🚫️async: E4 fn-pointer slot — see `macro_table_ok` above.
fn macro_quantity_ok(text: &str) -> bool {
    let parts: Vec<_> = text.split_whitespace().collect();
    !parts.is_empty() && parts[0].chars().next().is_some_and(|c| c.is_ascii_digit() || c == '-' || c == '.')
}

// 🚫️async: E4 fn-pointer slot — see `macro_table_ok` above.
fn macro_props_ok(text: &str) -> bool {
    text.contains('=')
}

/// @emoji 🔠️ P2-P1: a run of lowercase hex digits (`enc_str`/`hex_encode`'s own alphabet), incl. the
/// empty string (an empty hex-encoded value is valid — see `match_macro_span`'s zero-width floor).
/// Exists as a MACRO, not a `hex = {INT | IDENT | FLOAT}*` production, because a Star is a single
/// greedy pass with no backtracking (`Symbol::Star`'s doc comment): a *production*-modeled `hex`
/// immediately followed by an unrelated bareword literal it happens to share a token KIND with
/// (`set-member`'s real wire shape is `key=<hex> value=<value>` — `value` tokenizes as an ordinary
/// `IDENT`, exactly like a stray hex letter run would) gets silently swallowed INTO the greedy `hex`
/// Star, desyncing everything after it — a real, silent (no parse error, just `recognize() == false`
/// or worse, a wrong match) trap for every future FG-wave author who reaches for `{INT|IDENT}*` to
/// model a generic opaque/hex content field placed before another keyword. `match_macro_span`
/// already tries the largest token span first and shrinks until `try_match` accepts, so `hex` here
/// naturally backtracks off `value` (spelled with `v`/`l`/`u`, none of them valid hex digits) without
/// any grammar-file change beyond referencing the bare `hex` ident (no production named `hex` is
/// defined — `Symbol::Ref`'s existing production-then-macro fallback routes it here automatically).
// 🚫️async: E4 fn-pointer slot — see `macro_table_ok` above.
fn macro_hex_ok(text: &str) -> bool {
    // `slice_source_text` joins multi-token spans with a synthetic `" "` (no such space exists in
    // the real source — whitespace is trivia, stripped before tokens ever reach the recognizer, so
    // e.g. hex `"6b"` lexing as two adjacent tokens `Int("6")`/`Ident("b")` joins to `"6 b"` here);
    // filter it back out before validating, or every multi-token hex value would spuriously fail.
    text.bytes().filter(|b| !b.is_ascii_whitespace()).all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

fn default_macros() -> Vec<MacroMatcher> {
    vec![
        MacroMatcher {
            name: "edge",
            // 🚫️async: E4 fn-pointer slot — `MacroMatcher.try_match` is a bare `fn`; `parse_edge_text`
            // `crate::os_dsl::notation::parse_edge_text` was reverted to sync per R9 (whole-module,
            // 0 await/0 io measured), so the `resolve_ready` bridge this thunk used is gone — a
            // direct sync call now.
            try_match: |text| crate::os_dsl::notation::parse_edge_text(text).is_ok(),
        },
        MacroMatcher { name: "table", try_match: macro_table_ok },
        MacroMatcher { name: "quantity", try_match: macro_quantity_ok },
        MacroMatcher { name: "props", try_match: macro_props_ok },
        MacroMatcher { name: "hex", try_match: macro_hex_ok },
    ]
}
//#endregion 🔖️Recognizer

//#region 📡️ProtocolWalk
/// @emoji 🧮️ P2-M2 items 3+6: walk-wide state threaded through the ENTIRE `walk_protocol` pass
/// (every block, in order) — not reset per block/call as the pre-M2 `walk_fields`-local `HashMap`
/// was. `env` makes a LATER block's `Count::Field`/`Cond` resolve against a value decoded by any
/// EARLIER block (las's VLR/point-record repeat counts from the header; gif89a's GCE state carried
/// to the next block; bmp's palette/row counts). `big_endian` is the runtime endian mode a
/// `Prim::Endian` field can flip for the remainder of the walk (TIFF's `II`/`MM`); plain `U16`/
/// `U32`/`U64`/`I32`/`I64`/`F32`/`F64` obey it, `*Be` variants (item 2) always ignore it.
#[derive(Default)]
struct WalkState {
    env: std::collections::HashMap<String, u64>,
    big_endian: bool,
}

fn prim_fixed_width(prim: &Prim) -> Option<usize> {
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

fn decode_u16(slice: &[u8], big_endian: bool) -> u16 {
    let bytes = [slice[0], slice[1]];
    if big_endian {
        u16::from_be_bytes(bytes)
    } else {
        u16::from_le_bytes(bytes)
    }
}

fn decode_u32(slice: &[u8], big_endian: bool) -> u32 {
    let bytes: [u8; 4] = slice.try_into().unwrap();
    if big_endian {
        u32::from_be_bytes(bytes)
    } else {
        u32::from_le_bytes(bytes)
    }
}

fn decode_u64(slice: &[u8], big_endian: bool) -> u64 {
    let bytes: [u8; 8] = slice.try_into().unwrap();
    if big_endian {
        u64::from_be_bytes(bytes)
    } else {
        u64::from_le_bytes(bytes)
    }
}

// 🚫️async: E1 pure error constructor, consumed by `Option::ok_or_else` sync closures at several
// call sites across this file's protocol-walk decoder — see R9
fn mismatch(offset: usize, message: impl Into<String>) -> ProtocolMismatch {
    ProtocolMismatch { offset, message: message.into() }
}

fn read_varint_u64(bytes: &[u8], pos: &mut usize) -> Result<u64, ProtocolMismatch> {
    let mut result = 0u64;
    let mut shift = 0u32;
    loop {
        if *pos >= bytes.len() {
            return Err(mismatch(*pos, "truncated varint"));
        }
        let byte = bytes[*pos];
        *pos += 1;
        result |= u64::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            return Ok(result);
        }
        shift += 7;
        if shift > 63 {
            return Err(mismatch(*pos, "varint overflow"));
        }
    }
}

fn need<'a>(bytes: &'a [u8], pos: usize, n: usize, what: &str) -> Result<&'a [u8], ProtocolMismatch> {
    if pos + n > bytes.len() {
        return Err(mismatch(pos, format!("truncated {what}: need {n} bytes, have {}", bytes.len().saturating_sub(pos))));
    }
    Ok(&bytes[pos..pos + n])
}

fn trailing_reserved(blocks: &[Block], from: usize) -> usize {
    let mut reserved = 0usize;
    for block in &blocks[from..] {
        match block {
            Block::Footer(n) => reserved += *n,
            Block::Chain(prim) => reserved += prim_fixed_width(prim).unwrap_or(0),
            Block::Struct { .. } | Block::Enum { .. } => {}
            Block::Header(_) | Block::Segment { .. } | Block::Record { .. } | Block::Repeat { .. } | Block::BackwardScan { .. } | Block::JumpTo { .. } => break,
        }
    }
    reserved
}

fn resolve_count(count: &Count, env: &std::collections::HashMap<String, u64>, offset: usize) -> Result<usize, ProtocolMismatch> {
    match count {
        Count::Fixed(n) => Ok(*n),
        Count::Varint => Err(mismatch(offset, "Count::Varint must be read from the byte stream, not resolved from env")),
        Count::Field(name) => env.get(name).map(|v| *v as usize).ok_or_else(|| mismatch(offset, format!("unknown count field `{name}`"))),
    }
}

/// @emoji 🔀️ P2-M2 item 4: evaluate a field/segment presence guard against the walk-wide env.
fn eval_cond(cond: &Cond, env: &std::collections::HashMap<String, u64>, offset: usize) -> Result<bool, ProtocolMismatch> {
    let actual = *env.get(&cond.field).ok_or_else(|| mismatch(offset, format!("condition references unknown field `{}`", cond.field)))?;
    Ok(match cond.op {
        CondOp::Eq => actual == cond.value,
        CondOp::Ne => actual != cond.value,
        CondOp::Lt => actual < cond.value,
        CondOp::Le => actual <= cond.value,
        CondOp::Gt => actual > cond.value,
        CondOp::Ge => actual >= cond.value,
    })
}

fn walk_prim(prim: &Prim, bytes: &[u8], pos: &mut usize, state: &mut WalkState, reserved_tail: usize) -> Result<(), ProtocolMismatch> {
    match prim {
        Prim::U8 => {
            need(bytes, *pos, 1, "u8")?;
            *pos += 1;
        }
        Prim::U16 | Prim::U16Be => {
            need(bytes, *pos, 2, "u16")?;
            *pos += 2;
        }
        Prim::U32 | Prim::I32 | Prim::F32 | Prim::U32Be | Prim::I32Be | Prim::F32Be => {
            need(bytes, *pos, 4, "u32/i32/f32")?;
            *pos += 4;
        }
        Prim::U64 | Prim::I64 | Prim::F64 | Prim::U64Be | Prim::I64Be | Prim::F64Be => {
            need(bytes, *pos, 8, "u64/i64/f64")?;
            *pos += 8;
        }
        Prim::Fixed(n) => {
            need(bytes, *pos, *n, "fixed")?;
            *pos += *n;
        }
        Prim::Varint | Prim::Tag | Prim::Zigzag => {
            let _ = read_varint_u64(bytes, pos)?;
        }
        Prim::Bytes | Prim::Utf8 => {
            let end = bytes.len().saturating_sub(reserved_tail);
            if *pos > end {
                return Err(mismatch(*pos, "bytes field overlaps trailing reserved region"));
            }
            *pos = end;
        }
        Prim::Array(inner, count) => {
            let n = match count {
                Count::Varint => read_varint_u64(bytes, pos)? as usize,
                other => resolve_count(other, &state.env, *pos)?,
            };
            if matches!(inner.as_ref(), Prim::U8) {
                need(bytes, *pos, n, "byte array")?;
                *pos += n;
            } else {
                for _ in 0..n {
                    walk_prim(inner, bytes, pos, state, reserved_tail)?;
                }
            }
        }
        Prim::Ref(name) => return Err(mismatch(*pos, format!("unresolved protocol Ref({name}) during walk"))),
        // P2-M2 item 1c.
        Prim::MarkerScan(prefix) => {
            while *pos < bytes.len() && bytes[*pos] == *prefix {
                *pos += 1;
            }
            need(bytes, *pos, 1, "marker byte")?;
            *pos += 1;
        }
        // P2-M2 item 6.
        Prim::Endian(arms) => {
            let width = arms.first().map_or(0, |(k, _)| k.len());
            let slice = need(bytes, *pos, width, "endian marker")?;
            match arms.iter().find(|(k, _)| k.as_bytes() == slice) {
                Some((_, be)) => state.big_endian = *be,
                None => return Err(mismatch(*pos, format!("unrecognized endianness marker bytes {slice:?}"))),
            }
            *pos += width;
        }
    }
    Ok(())
}

/// @emoji 🏷️ P2-M2 item 1: reads a discriminator's exact raw bytes (no numeric decode — arm tags
/// are compared byte-for-byte, see [`parse_tag_value`]) and advances `pos` past them.
fn read_raw_prim_bytes(prim: &Prim, bytes: &[u8], pos: &mut usize) -> Result<Vec<u8>, ProtocolMismatch> {
    match prim {
        Prim::MarkerScan(prefix) => {
            while *pos < bytes.len() && bytes[*pos] == *prefix {
                *pos += 1;
            }
            let slice = need(bytes, *pos, 1, "marker byte")?.to_vec();
            *pos += 1;
            Ok(slice)
        }
        Prim::Fixed(n) => {
            let slice = need(bytes, *pos, *n, "fixed discriminator")?.to_vec();
            *pos += n;
            Ok(slice)
        }
        other => {
            let width = prim_fixed_width(other).ok_or_else(|| mismatch(*pos, format!("{other:?} has no fixed width, cannot be used as a discriminator")))?;
            let slice = need(bytes, *pos, width, "discriminator")?.to_vec();
            *pos += width;
            Ok(slice)
        }
    }
}

/// @emoji 🔢️ Reads a numeric scalar (length/count field) honoring [`WalkState::big_endian`] for
/// the plain (non-`Be`) variants — used by `repeat`'s `length` directive and, via [`walk_fields`],
/// for ordinary `Count::Field`-producing fields.
fn read_scalar_prim(prim: &Prim, bytes: &[u8], pos: &mut usize, state: &WalkState) -> Result<u64, ProtocolMismatch> {
    match prim {
        Prim::U8 => {
            let slice = need(bytes, *pos, 1, "u8")?;
            let v = u64::from(slice[0]);
            *pos += 1;
            Ok(v)
        }
        Prim::U16 => {
            let slice = need(bytes, *pos, 2, "u16")?;
            let v = u64::from(decode_u16(slice, state.big_endian));
            *pos += 2;
            Ok(v)
        }
        Prim::U16Be => {
            let slice = need(bytes, *pos, 2, "u16be")?;
            let v = u64::from(decode_u16(slice, true));
            *pos += 2;
            Ok(v)
        }
        Prim::U32 => {
            let slice = need(bytes, *pos, 4, "u32")?;
            let v = u64::from(decode_u32(slice, state.big_endian));
            *pos += 4;
            Ok(v)
        }
        Prim::U32Be => {
            let slice = need(bytes, *pos, 4, "u32be")?;
            let v = u64::from(decode_u32(slice, true));
            *pos += 4;
            Ok(v)
        }
        Prim::U64 => {
            let slice = need(bytes, *pos, 8, "u64")?;
            let v = decode_u64(slice, state.big_endian);
            *pos += 8;
            Ok(v)
        }
        Prim::U64Be => {
            let slice = need(bytes, *pos, 8, "u64be")?;
            let v = decode_u64(slice, true);
            *pos += 8;
            Ok(v)
        }
        Prim::Varint | Prim::Tag | Prim::Zigzag => read_varint_u64(bytes, pos),
        other => Err(mismatch(*pos, format!("{other:?} cannot be read as a scalar length/count"))),
    }
}

/// @emoji 🔎️ P2-M2 item 5a: the rightmost occurrence of `pattern` in `bytes` — ZIP's EOCD is
/// located by scanning BACKWARD from EOF because its preceding comment field is 0-65535 bytes, so
/// its start is unknowable except by finding the EOCD's own magic first.
fn find_last_occurrence(bytes: &[u8], pattern: &[u8]) -> Option<usize> {
    if pattern.is_empty() || pattern.len() > bytes.len() {
        return None;
    }
    (0..=bytes.len() - pattern.len()).rev().find(|&i| &bytes[i..i + pattern.len()] == pattern)
}

fn walk_fields(fields: &[Field], bytes: &[u8], pos: &mut usize, state: &mut WalkState, reserved_tail: usize) -> Result<(), ProtocolMismatch> {
    for (index, field) in fields.iter().enumerate() {
        // P2-M2 item 4: a field absent under its guard is simply skipped — not read at all.
        if let Some(cond) = &field.cond {
            if !eval_cond(cond, &state.env, *pos)? {
                continue;
            }
        }
        let field_reserved = if index + 1 == fields.len() {
            reserved_tail
        } else {
            let mut tail_width = 0;
            for f in &fields[index + 1..] {
                tail_width += prim_fixed_width(&f.ty).unwrap_or(0);
            }
            reserved_tail + tail_width
        };
        match &field.ty {
            Prim::U8 => {
                let slice = need(bytes, *pos, 1, &field.name)?;
                state.env.insert(field.name.clone(), u64::from(slice[0]));
                *pos += 1;
            }
            // P2-M2 item 6: plain U16/U32/U64 honor the walker's current runtime endian mode.
            Prim::U16 => {
                let slice = need(bytes, *pos, 2, &field.name)?;
                let value = u64::from(decode_u16(slice, state.big_endian));
                state.env.insert(field.name.clone(), value);
                *pos += 2;
            }
            Prim::U32 => {
                let slice = need(bytes, *pos, 4, &field.name)?;
                let value = u64::from(decode_u32(slice, state.big_endian));
                state.env.insert(field.name.clone(), value);
                *pos += 4;
            }
            Prim::U64 => {
                let slice = need(bytes, *pos, 8, &field.name)?;
                let value = decode_u64(slice, state.big_endian);
                state.env.insert(field.name.clone(), value);
                *pos += 8;
            }
            // P2-M2 item 2: *Be variants are ALWAYS big-endian, regardless of the runtime mode.
            Prim::U16Be => {
                let slice = need(bytes, *pos, 2, &field.name)?;
                state.env.insert(field.name.clone(), u64::from(decode_u16(slice, true)));
                *pos += 2;
            }
            Prim::U32Be => {
                let slice = need(bytes, *pos, 4, &field.name)?;
                state.env.insert(field.name.clone(), u64::from(decode_u32(slice, true)));
                *pos += 4;
            }
            Prim::U64Be => {
                let slice = need(bytes, *pos, 8, &field.name)?;
                state.env.insert(field.name.clone(), decode_u64(slice, true));
                *pos += 8;
            }
            Prim::Varint | Prim::Tag | Prim::Zigzag => {
                let value = read_varint_u64(bytes, pos)?;
                state.env.insert(field.name.clone(), value);
            }
            // P2-M2 item 6: the endian-marker field itself — mutates walker state, binds no value.
            Prim::Endian(arms) => {
                let width = arms.first().map_or(0, |(k, _)| k.len());
                let slice = need(bytes, *pos, width, "endian marker")?;
                match arms.iter().find(|(k, _)| k.as_bytes() == slice) {
                    Some((_, be)) => state.big_endian = *be,
                    None => return Err(mismatch(*pos, format!("unrecognized endianness marker bytes {slice:?}"))),
                }
                *pos += width;
            }
            other => walk_prim(other, bytes, pos, state, field_reserved)?,
        }
    }
    Ok(())
}

/// @emoji 🌿️ P2-M2 item 1b: single-shot (non-repeating) second-level dispatch — GIF 89a's
/// extension-introducer arm dispatches again on the label byte.
fn walk_nested_dispatch(nested: &NestedDispatch, bytes: &[u8], pos: &mut usize, state: &mut WalkState) -> Result<(), ProtocolMismatch> {
    let tag = read_raw_prim_bytes(&nested.discriminator, bytes, pos)?;
    match nested.arms.iter().find(|arm| arm.tag == tag) {
        Some(arm) => {
            walk_fields(&arm.fields, bytes, pos, state, 0)?;
            if let Some(deeper) = &arm.nested {
                walk_nested_dispatch(deeper, bytes, pos, state)?;
            }
            Ok(())
        }
        None => Err(mismatch(*pos, format!("unrecognized nested discriminator {tag:?} in `{}`", nested.name))),
    }
}

/// @emoji 🔁️ P2-M2 item 1: read discriminator (+ optional length, per `order`), dispatch into a
/// known arm's fields or skip an unrecognized discriminator's declared `length` as opaque bytes,
/// repeat until EOF or `until`'s sentinel discriminator value is seen.
fn walk_repeat(dispatch: &RepeatDispatch, bytes: &[u8], pos: &mut usize, state: &mut WalkState) -> Result<(), ProtocolMismatch> {
    loop {
        if *pos >= bytes.len() {
            break;
        }
        let iter_start = *pos;
        let (tag, length_value) = match dispatch.order {
            DispatchOrder::LengthFirst => {
                let len = match &dispatch.length {
                    Some(p) => Some(read_scalar_prim(p, bytes, pos, state)?),
                    None => None,
                };
                let tag = read_raw_prim_bytes(&dispatch.discriminator, bytes, pos)?;
                (tag, len)
            }
            DispatchOrder::TagFirst => {
                let tag = read_raw_prim_bytes(&dispatch.discriminator, bytes, pos)?;
                let len = match &dispatch.length {
                    Some(p) => Some(read_scalar_prim(p, bytes, pos, state)?),
                    None => None,
                };
                (tag, len)
            }
        };
        let is_sentinel = dispatch.until.as_deref() == Some(tag.as_slice());
        let body_start = *pos;
        match dispatch.arms.iter().find(|arm| arm.tag == tag) {
            Some(arm) => {
                walk_fields(&arm.fields, bytes, pos, state, 0)?;
                if let Some(nested) = &arm.nested {
                    walk_nested_dispatch(nested, bytes, pos, state)?;
                }
                if let Some(len) = length_value {
                    let expected_end = body_start + len as usize;
                    if expected_end > bytes.len() {
                        return Err(mismatch(*pos, "repeat arm declared length exceeds buffer"));
                    }
                    if *pos > expected_end {
                        return Err(mismatch(*pos, format!("repeat arm fields overran declared length ({} > {})", *pos, expected_end)));
                    }
                    *pos = expected_end;
                }
            }
            // P2-M2 item 1a: unrecognized discriminator — skip its declared length as opaque bytes.
            None => match length_value {
                Some(len) => {
                    need(bytes, *pos, len as usize, "repeat skip")?;
                    *pos += len as usize;
                }
                None => return Err(mismatch(*pos, format!("unrecognized discriminator {tag:?} with no declared `length` to skip"))),
            },
        }
        if let Some(trailer_prim) = &dispatch.trailer {
            walk_prim(trailer_prim, bytes, pos, state, 0)?;
        }
        if is_sentinel {
            break;
        }
        if *pos == iter_start {
            return Err(mismatch(*pos, "repeat block made no forward progress"));
        }
    }
    Ok(())
}

fn definitions_only(block: &Block) -> bool {
    matches!(block, Block::Struct { .. } | Block::Enum { .. })
}

/// @emoji 🧭️ Spec-driven byte walker — consumes every declared wire slot and must finish at
/// exactly `bytes.len()`, else returns [`ProtocolMismatch`] with the failing offset. **Exception**
/// (P2-M2 item 5, documented precisely per the plan's own instruction): once ANY block has
/// explicitly JUMPED `pos` (`Block::BackwardScan`/`Block::JumpTo`), the walk is no longer a pure
/// linear forward accounting of every byte in the buffer — that is the whole point of a
/// backward-scan/offset-jump (ZIP's EOCD sits at a position determined only by scanning backward
/// from EOF, and its `cd_offset` field points at a position determined only by decoding the EOCD
/// first; the bytes physically between a jump target and EOF, or before it, are validly described
/// by OTHER blocks the walk already visited, not by "whatever's left after this one"). The final
/// `pos == bytes.len()` law is therefore skipped for any walk that performed at least one jump; it
/// holds EXACTLY as before for every protocol that declares neither block (the overwhelming
/// majority). The walker still only ever reads FORWARD from a jump's landing point — jumps move
/// `pos` directly, they never make the walker itself search or backtrack mid-block.
pub fn walk_protocol(spec: &ProtocolFile, bytes: &[u8]) -> Result<ProtocolTrace, ProtocolMismatch> {
    let mut pos = 0usize;
    match &spec.framing {
        Framing::Magic(magic) => {
            let got = need(bytes, 0, 8, "magic")?;
            if got != magic {
                return Err(mismatch(0, format!("magic mismatch: expected {magic:?}, got {got:?}")));
            }
            pos = 8;
        }
        Framing::Record | Framing::Chunked => {}
    }

    let skip_named_records = matches!(spec.framing, Framing::Magic(_) | Framing::Chunked);
    let record_body_as_rest = matches!(spec.framing, Framing::Record);
    let mut consumed_record_body = false;
    // P2-M2 items 3+6: ONE state, shared across every block in this walk (see `WalkState` doc).
    let mut state = WalkState::default();
    // P2-M2 item 5: set once a BackwardScan/JumpTo block executes — relaxes the final EOF check.
    let mut jumped = false;

    for (index, block) in spec.blocks.iter().enumerate() {
        if definitions_only(block) {
            continue;
        }
        let reserved = trailing_reserved(&spec.blocks, index + 1);
        match block {
            Block::Header(fields) => walk_fields(fields, bytes, &mut pos, &mut state, reserved)?,
            Block::Segment { fields, cond, .. } => {
                let present = match cond {
                    Some(c) => eval_cond(c, &state.env, pos)?,
                    None => true,
                };
                if present {
                    walk_fields(fields, bytes, &mut pos, &mut state, reserved)?;
                }
            }
            Block::Record { name, fields, .. } => {
                if skip_named_records && !name.is_empty() {
                    continue;
                }
                if record_body_as_rest && !name.is_empty() {
                    if !consumed_record_body {
                        pos = bytes.len();
                        consumed_record_body = true;
                    }
                    continue;
                }
                walk_fields(fields, bytes, &mut pos, &mut state, reserved)?;
            }
            Block::Footer(size) => {
                need(bytes, pos, *size, "footer")?;
                pos += *size;
            }
            Block::Chain(prim) => walk_prim(prim, bytes, &mut pos, &mut state, 0)?,
            Block::Repeat { dispatch, .. } => walk_repeat(dispatch, bytes, &mut pos, &mut state)?,
            Block::BackwardScan { magic, fields, .. } => {
                let found = find_last_occurrence(bytes, magic).ok_or_else(|| mismatch(bytes.len(), "backward-scan magic not found in buffer"))?;
                // `fields` describe what comes AFTER the magic pattern itself, not the magic bytes.
                pos = found + magic.len();
                jumped = true;
                walk_fields(fields, bytes, &mut pos, &mut state, 0)?;
            }
            Block::JumpTo { offset_field, fields, .. } => {
                let target = *state.env.get(offset_field).ok_or_else(|| mismatch(pos, format!("jump target field `{offset_field}` was not decoded by an earlier block")))?;
                let target = target as usize;
                if target > bytes.len() {
                    return Err(mismatch(target, "jump target offset exceeds buffer length"));
                }
                pos = target;
                jumped = true;
                walk_fields(fields, bytes, &mut pos, &mut state, 0)?;
            }
            Block::Struct { .. } | Block::Enum { .. } => {}
        }
    }

    if !jumped && pos != bytes.len() {
        return Err(mismatch(pos, format!("trailing {} bytes after protocol walk", bytes.len() - pos)));
    }
    Ok(ProtocolTrace { consumed: pos })
}

/// @emoji 📡️ Shallow [`GrammarFile`] back-compat check: pack requires leading `0x89` magic
/// (any family) and ≥32 bytes; spr requires non-empty bytes. Deep walks use [`verify_protocol_source`].
pub fn verify_protocol_bytes(spec: &GrammarFile, bytes: &[u8]) -> Result<(), String> {
    let id = spec.id.to_ascii_lowercase();
    let start = spec.start.to_ascii_lowercase();
    let is_spr = start == "record" || id.contains("spr");
    let is_pack = start == "frame" || id.contains("pack") || (matches!(spec.dialect, SemioDialect::Protocol) && !is_spr);
    if is_spr {
        if bytes.is_empty() {
            return Err("spr envelope rejects empty bytes".into());
        }
        return Ok(());
    }
    if is_pack || bytes.first() == Some(&0x89) {
        if bytes.len() < 32 {
            return Err(format!("pack envelope requires ≥32 bytes, got {}", bytes.len()));
        }
        if bytes[0] != 0x89 {
            return Err("pack magic must start with 0x89".into());
        }
        return Ok(());
    }
    Err(format!("verify_protocol_bytes: cannot classify protocol id='{}' start='{}'", spec.id, spec.start))
}

/// @emoji 📡️ Parses handcrafted `.protocol.semio` source then deep-walks bytes via [`walk_protocol`].
pub fn verify_protocol_source(source: &str, bytes: &[u8]) -> Result<(), String> {
    let spec = parse_protocol(source).map_err(|error| error.message)?;
    walk_protocol(&spec, bytes).map(|_| ()).map_err(|e| format!("offset {}: {}", e.offset, e.message))
}

//#endregion 📡️ProtocolWalk

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
