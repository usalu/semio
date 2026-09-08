//! 📖️ `compiler_syntax` — the semio compiler's math notation: lexer, AST, recursive-descent
//! parser, and canonical printer for the semio-native math snippet syntax (`x^2`, `frac(a, b)`,
//! `mat(1, 2; 3, 4)`, `:rocket:`, …) that replaces Typst math markup. Follows the same "own
//! pre-scan lexer for extras outside `dsl_core`'s shared token alphabet, delegate every other run
//! of characters to `os_dsl::lex`" pattern `dsl_grammar` and `mathematical_graph_dsl` already
//! established — this EXTENDS the shared grammar infrastructure, it does not fork it. The
//! normative spec ships alongside as `📖️.grammar.semio` (see `math_grammar_parses_under_dsl_grammar`
//! in the `🧪️Tests` region, a dev-dependency-only check since `dsl_grammar`'s `Recognizer` matches
//! `os_dsl::lex` tokens directly and can't see this crate's own pre-scanned extras).
//!
//! Scope (Wave 1 of the compiler plan): parses/prints math snippets only. Layout, fonts, and SVG
//! emission are later waves — this crate produces an AST, nothing renders yet.

use crate::os_dsl::{escape_text, lex as core_lex, unescape_text, Limits, TextError, TextSpan, TokenKind as CoreKind};

//#region 🔖️Model
/// @emoji 🌳️ One parsed math expression. Function/structure names (`frac`, `sqrt`, `mat`, `hat`, …)
/// are NOT baked into the grammar — every `name(...)` call parses generically as [`MathNode::Call`];
/// resolving what a given name means is a layout-layer (Wave 2) concern, matching how `dsl_grammar`
/// itself treats macro names as opaque until a matcher is registered.
#[derive(Clone, Debug, PartialEq)]
pub enum MathNode {
    /// An integer or float literal, canonical text as lexed (e.g. `"42"`, `"3.14"`).
    Number(String),
    /// A bare identifier: a variable (`x`), a named symbol (`alpha`, `pi`, `sum`), or direct
    /// Unicode (`α`, `∞`) — `dsl_core`'s ident rule already accepts alphabetic Unicode.
    Symbol(String),
    /// `:name:` — an emoji shortcode atom.
    Emoji(String),
    /// A quoted text-in-math atom (`"if"`), unescaped.
    Text(String),
    /// `{ ... }` — invisible grouping, the only way to script a multi-token expression.
    Group(Box<MathNode>),
    /// A stretchy auto-sized delimiter pair: `(` paired with `)`, or `[` paired with `]`. The
    /// stored `char` is the opening delimiter.
    Paren(char, Box<MathNode>),
    /// `base^exponent`.
    Sup(Box<MathNode>, Box<MathNode>),
    /// `base_subscript`.
    Sub(Box<MathNode>, Box<MathNode>),
    /// `name(row1cell1, row1cell2; row2cell1, ...)` — rows separated by `;`, cells by `,`. A plain
    /// call like `frac(a, b)` is one row of two cells; `mat(1, 2; 3, 4)` is two rows.
    Call(String, Vec<Vec<MathNode>>),
    /// A binary relation/arithmetic operator.
    BinOp(BinOp, Box<MathNode>, Box<MathNode>),
    /// A horizontal run of two or more terms with no single operator joining them all — bare
    /// juxtaposition (`2x`) and explicit-`*` products (`2 * x`) both land here; [`SeqItem::dot`]
    /// distinguishes which, since only the explicit form renders a centered dot.
    Sequence(Vec<SeqItem>),
}

/// @emoji 🧵️ One item in a [`MathNode::Sequence`] — `dot` is `true` when this item was joined to
/// the previous one by an explicit `*` (renders `⋅`), `false` for bare juxtaposition (no glyph).
#[derive(Clone, Debug, PartialEq)]
pub struct SeqItem {
    pub node: MathNode,
    pub dot: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Div,
    Eq,
    Ne,
    Le,
    Ge,
    Lt,
    Gt,
    Arrow,
}

impl BinOp {
    fn symbol(self) -> &'static str {
        match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Div => "/",
            BinOp::Eq => "=",
            BinOp::Ne => "!=",
            BinOp::Le => "<=",
            BinOp::Ge => ">=",
            BinOp::Lt => "<",
            BinOp::Gt => ">",
            BinOp::Arrow => "->",
        }
    }
}
//#endregion 🔖️Model

//#region 🔖️Lexer
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MKind {
    Ident,
    Int,
    Float,
    Text,
    Caret,
    Underscore,
    Semicolon,
    Comma,
    Colon,
    Equals,
    NotEquals,
    LessEqual,
    GreaterEqual,
    Less,
    Greater,
    Arrow,
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Eof,
}

#[derive(Clone, Debug, PartialEq)]
struct MToken {
    kind: MKind,
    text: String,
    span: TextSpan,
}

fn line_col_at(text: &str, byte_pos: usize) -> (u32, u32) {
    let line = text[..byte_pos].matches('\n').count() as u32 + 1;
    let col = (byte_pos - text[..byte_pos].rfind('\n').map_or(0, |p| p + 1)) as u32 + 1;
    (line, col)
}

/// @emoji 🔬️ Pre-scans `_ ; < > !` — none in `dsl_core`'s shared alphabet, and `_` in particular
/// would otherwise glue into a preceding ident (`x_1` lexing as one `Ident("x_1")`) since
/// `os_dsl::lex`'s `is_ident_continue` accepts `_` — and delegates every other run of characters
/// whole to `os_dsl::lex`, exactly like `os_dsl::grammar::lex` does for its own `? |` extras.
fn lex(text: &str) -> Result<Vec<MToken>, TextError> {
    let bytes = text.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 0usize;
    let mut seg_start = 0usize;

    let push_segment = |seg_start: usize, seg_end: usize, tokens: &mut Vec<MToken>| -> Result<(), TextError> {
        if seg_end <= seg_start {
            return Ok(());
        }
        let segment = &text[seg_start..seg_end];
        let raw = core_lex(segment, &Limits::default(), false)?;
        for token in raw {
            if matches!(token.kind, CoreKind::Whitespace | CoreKind::Newline | CoreKind::Comment | CoreKind::Eof) {
                continue;
            }
            let kind = match token.kind {
                CoreKind::Ident => MKind::Ident,
                CoreKind::Int => MKind::Int,
                CoreKind::Float => MKind::Float,
                CoreKind::Text => MKind::Text,
                CoreKind::Caret => MKind::Caret,
                CoreKind::Comma => MKind::Comma,
                CoreKind::Colon => MKind::Colon,
                CoreKind::Equals => MKind::Equals,
                CoreKind::Arrow => MKind::Arrow,
                CoreKind::Plus => MKind::Plus,
                CoreKind::Minus => MKind::Minus,
                CoreKind::Star => MKind::Star,
                CoreKind::Slash => MKind::Slash,
                CoreKind::LParen => MKind::LParen,
                CoreKind::RParen => MKind::RParen,
                CoreKind::LBrace => MKind::LBrace,
                CoreKind::RBrace => MKind::RBrace,
                CoreKind::LBracket => MKind::LBracket,
                CoreKind::RBracket => MKind::RBracket,
                other => return Err(TextError::new(format!("math notation cannot contain a {other:?} token here"), token.span)),
            };
            tokens.push(MToken { kind, text: token.text.as_str().to_string(), span: token.span });
        }
        Ok(())
    };

    while i < bytes.len() {
        let c = bytes[i];
        // A quoted `"..."` literal is skipped whole here (dsl_core's own escape scheme) so an
        // extra character inside a string is never mistaken for this lexer's own operators.
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
        if c == b'_' || c == b';' {
            push_segment(seg_start, i, &mut tokens)?;
            let (line, col) = line_col_at(text, i);
            let kind = if c == b'_' { MKind::Underscore } else { MKind::Semicolon };
            tokens.push(MToken { kind, text: (c as char).to_string(), span: TextSpan::with_length(line, col, 1) });
            i += 1;
            seg_start = i;
            continue;
        }
        if c == b'!' {
            push_segment(seg_start, i, &mut tokens)?;
            let (line, col) = line_col_at(text, i);
            if bytes.get(i + 1) == Some(&b'=') {
                tokens.push(MToken { kind: MKind::NotEquals, text: "!=".to_string(), span: TextSpan::with_length(line, col, 2) });
                i += 2;
                seg_start = i;
                continue;
            }
            return Err(TextError::new("expected `!=` (bare `!` is not a math notation operator)", TextSpan::at(line, col)));
        }
        // `->` is already `Arrow` in `dsl_core`'s alphabet — when this `>` is the second half of
        // one (immediately preceded by `-`), leave it in the ambient segment instead of cutting it
        // off here, so `core_lex` sees the whole `->` and tokenizes it as a single `Arrow`.
        if c == b'>' && i > 0 && bytes[i - 1] == b'-' {
            i += 1;
            continue;
        }
        if c == b'<' || c == b'>' {
            push_segment(seg_start, i, &mut tokens)?;
            let (line, col) = line_col_at(text, i);
            let has_eq = bytes.get(i + 1) == Some(&b'=');
            let (kind, len): (MKind, usize) = match (c, has_eq) {
                (b'<', true) => (MKind::LessEqual, 2),
                (b'<', false) => (MKind::Less, 1),
                (b'>', true) => (MKind::GreaterEqual, 2),
                (_, false) => (MKind::Greater, 1),
                _ => unreachable!("only `<`/`>` reach this branch"),
            };
            tokens.push(MToken { kind, text: text[i..i + len].to_string(), span: TextSpan::with_length(line, col, len as u32) });
            i += len;
            seg_start = i;
            continue;
        }
        i += 1;
    }
    push_segment(seg_start, bytes.len(), &mut tokens)?;
    let eof_span = tokens.last().map_or(TextSpan::at(1, 1), |t| t.span);
    tokens.push(MToken { kind: MKind::Eof, text: String::new(), span: eof_span });
    Ok(tokens)
}
//#endregion 🔖️Lexer

//#region 🔖️Parser
struct Cursor {
    tokens: Vec<MToken>,
    pos: usize,
}

impl Cursor {
    fn peek(&self) -> &MToken {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> MToken {
        let token = self.tokens[self.pos].clone();
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        token
    }

    fn expect(&mut self, kind: MKind) -> Result<MToken, TextError> {
        if self.peek().kind == kind {
            Ok(self.advance())
        } else {
            Err(TextError::new(format!("expected {kind:?}, found {:?} {:?}", self.peek().kind, self.peek().text), self.peek().span))
        }
    }
}

/// @emoji 🚪️ Parses one complete math snippet — the crate's main entry point.
pub fn parse_formula(text: &str) -> Result<MathNode, TextError> {
    let tokens = lex(text)?;
    let mut cursor = Cursor { tokens, pos: 0 };
    let node = parse_relation(&mut cursor)?;
    if cursor.peek().kind != MKind::Eof {
        return Err(TextError::new(format!("unexpected trailing token {:?} {:?}", cursor.peek().kind, cursor.peek().text), cursor.peek().span));
    }
    Ok(node)
}

fn relop(kind: MKind) -> Option<BinOp> {
    match kind {
        MKind::Equals => Some(BinOp::Eq),
        MKind::NotEquals => Some(BinOp::Ne),
        MKind::LessEqual => Some(BinOp::Le),
        MKind::GreaterEqual => Some(BinOp::Ge),
        MKind::Less => Some(BinOp::Lt),
        MKind::Greater => Some(BinOp::Gt),
        MKind::Arrow => Some(BinOp::Arrow),
        _ => None,
    }
}

fn parse_relation(cursor: &mut Cursor) -> Result<MathNode, TextError> {
    let mut node = parse_expr(cursor)?;
    while let Some(op) = relop(cursor.peek().kind) {
        cursor.advance();
        let rhs = parse_expr(cursor)?;
        node = MathNode::BinOp(op, Box::new(node), Box::new(rhs));
    }
    Ok(node)
}

fn parse_expr(cursor: &mut Cursor) -> Result<MathNode, TextError> {
    let mut node = parse_run(cursor)?;
    loop {
        let op = match cursor.peek().kind {
            MKind::Plus => BinOp::Add,
            MKind::Minus => BinOp::Sub,
            _ => break,
        };
        cursor.advance();
        let rhs = parse_run(cursor)?;
        node = MathNode::BinOp(op, Box::new(node), Box::new(rhs));
    }
    Ok(node)
}

fn starts_atom(kind: MKind) -> bool {
    matches!(kind, MKind::Ident | MKind::Int | MKind::Float | MKind::Text | MKind::Colon | MKind::LBrace | MKind::LParen | MKind::LBracket)
}

/// @emoji 🏃️ A "run" is one or more postfix terms joined by explicit `*`/`/` or bare juxtaposition.
/// `/` binds tightly to its immediately preceding term (so `a * b / c` prints as `a * (b / c)`,
/// documented left-to-right behavior — this notation has no `*`-vs-`/` precedence distinction, and
/// current usage (icons, matrix cells) never chains the two).
fn parse_run(cursor: &mut Cursor) -> Result<MathNode, TextError> {
    let first = parse_postfix(cursor)?;
    let mut items = vec![SeqItem { node: first, dot: false }];
    loop {
        match cursor.peek().kind {
            MKind::Star => {
                cursor.advance();
                let node = parse_postfix(cursor)?;
                items.push(SeqItem { node, dot: true });
            }
            MKind::Slash => {
                cursor.advance();
                let rhs = parse_postfix(cursor)?;
                let lhs_item = items.pop().expect("items always has at least one entry");
                items.push(SeqItem { node: MathNode::BinOp(BinOp::Div, Box::new(lhs_item.node), Box::new(rhs)), dot: lhs_item.dot });
            }
            kind if starts_atom(kind) => {
                let node = parse_postfix(cursor)?;
                items.push(SeqItem { node, dot: false });
            }
            _ => break,
        }
    }
    if items.len() == 1 {
        Ok(items.pop().expect("checked len == 1").node)
    } else {
        Ok(MathNode::Sequence(items))
    }
}

fn parse_postfix(cursor: &mut Cursor) -> Result<MathNode, TextError> {
    let mut node = parse_atom(cursor)?;
    loop {
        match cursor.peek().kind {
            MKind::Caret => {
                cursor.advance();
                let exponent = parse_atom(cursor)?;
                node = MathNode::Sup(Box::new(node), Box::new(exponent));
            }
            MKind::Underscore => {
                cursor.advance();
                let subscript = parse_atom(cursor)?;
                node = MathNode::Sub(Box::new(node), Box::new(subscript));
            }
            _ => break,
        }
    }
    Ok(node)
}

fn parse_atom(cursor: &mut Cursor) -> Result<MathNode, TextError> {
    match cursor.peek().kind {
        MKind::Int | MKind::Float => Ok(MathNode::Number(cursor.advance().text)),
        MKind::Text => {
            let token = cursor.advance();
            let text = unescape_text(&token.text, false).map_err(|message| TextError::new(message, token.span))?;
            Ok(MathNode::Text(text))
        }
        MKind::Colon => {
            cursor.advance();
            let name = cursor.expect(MKind::Ident)?.text;
            cursor.expect(MKind::Colon)?;
            Ok(MathNode::Emoji(name))
        }
        MKind::LBrace => {
            cursor.advance();
            let inner = parse_relation(cursor)?;
            cursor.expect(MKind::RBrace)?;
            Ok(MathNode::Group(Box::new(inner)))
        }
        MKind::LParen => {
            cursor.advance();
            let inner = parse_relation(cursor)?;
            cursor.expect(MKind::RParen)?;
            Ok(MathNode::Paren('(', Box::new(inner)))
        }
        MKind::LBracket => {
            cursor.advance();
            let inner = parse_relation(cursor)?;
            cursor.expect(MKind::RBracket)?;
            Ok(MathNode::Paren('[', Box::new(inner)))
        }
        MKind::Ident => {
            let name = cursor.advance().text;
            if cursor.peek().kind == MKind::LParen {
                let rows = parse_call_args(cursor)?;
                Ok(MathNode::Call(name, rows))
            } else {
                Ok(MathNode::Symbol(name))
            }
        }
        other => Err(TextError::new(format!("expected a math atom, found {other:?} {:?}", cursor.peek().text), cursor.peek().span)),
    }
}

/// @emoji 📦️ `(` ROW {`;` ROW}* `)` — parens are macro-call syntax exclusively when they directly
/// follow an `Ident` (see [`parse_atom`]'s `MKind::Ident` arm); a bare, non-ident-preceded `(`/`[`
/// is a stretchy delimiter group instead. Same resolution `dsl_grammar` uses for `name(args)` vs
/// `name (group)` — whitespace is trivia, so the token stream alone can't otherwise disambiguate.
fn parse_call_args(cursor: &mut Cursor) -> Result<Vec<Vec<MathNode>>, TextError> {
    cursor.expect(MKind::LParen)?;
    let mut rows = vec![parse_row(cursor)?];
    while cursor.peek().kind == MKind::Semicolon {
        cursor.advance();
        rows.push(parse_row(cursor)?);
    }
    cursor.expect(MKind::RParen)?;
    Ok(rows)
}

fn parse_row(cursor: &mut Cursor) -> Result<Vec<MathNode>, TextError> {
    if matches!(cursor.peek().kind, MKind::RParen | MKind::Semicolon) {
        return Ok(Vec::new());
    }
    let mut cells = vec![parse_relation(cursor)?];
    while cursor.peek().kind == MKind::Comma {
        cursor.advance();
        cells.push(parse_relation(cursor)?);
    }
    Ok(cells)
}
//#endregion 🔖️Parser

//#region 🔖️Printer
fn print_node(node: &MathNode, out: &mut String) {
    match node {
        MathNode::Number(text) | MathNode::Symbol(text) => out.push_str(text),
        MathNode::Emoji(name) => {
            out.push(':');
            out.push_str(name);
            out.push(':');
        }
        MathNode::Text(value) => {
            out.push('"');
            out.push_str(&escape_text(value));
            out.push('"');
        }
        MathNode::Group(inner) => {
            out.push('{');
            print_node(inner, out);
            out.push('}');
        }
        MathNode::Paren(open, inner) => {
            let close = if *open == '(' { ')' } else { ']' };
            out.push(*open);
            print_node(inner, out);
            out.push(close);
        }
        MathNode::Sup(base, exponent) => {
            print_node(base, out);
            out.push('^');
            print_node(exponent, out);
        }
        MathNode::Sub(base, subscript) => {
            print_node(base, out);
            out.push('_');
            print_node(subscript, out);
        }
        MathNode::Call(name, rows) => {
            out.push_str(name);
            out.push('(');
            for (i, row) in rows.iter().enumerate() {
                if i > 0 {
                    out.push_str("; ");
                }
                for (j, cell) in row.iter().enumerate() {
                    if j > 0 {
                        out.push_str(", ");
                    }
                    print_node(cell, out);
                }
            }
            out.push(')');
        }
        MathNode::BinOp(op, lhs, rhs) => {
            print_node(lhs, out);
            out.push(' ');
            out.push_str(op.symbol());
            out.push(' ');
            print_node(rhs, out);
        }
        MathNode::Sequence(items) => {
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push_str(if item.dot { " * " } else { " " });
                }
                print_node(&item.node, out);
            }
        }
    }
}

/// @emoji 🖨️ Canonical printer — `parse_formula(print(&parse_formula(x)?)) == parse_formula(x)` is
/// this crate's round-trip law, checked in `🧪️Tests` over representative formulas.
pub fn print(node: &MathNode) -> String {
    let mut out = String::new();
    print_node(node, &mut out);
    out
}

/// @emoji ♻️ `canonicalize(canonicalize(x)) == canonicalize(x)`.
pub fn canonicalize(text: &str) -> Result<String, TextError> {
    Ok(print(&parse_formula(text)?))
}
//#endregion 🔖️Printer

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
