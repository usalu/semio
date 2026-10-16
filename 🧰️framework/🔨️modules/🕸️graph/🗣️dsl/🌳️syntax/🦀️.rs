//! 🧩️ Jack syntax: the query abstract syntax tree and the lexer that turns text into tokens and token spans.

use super::*;

// #region 🔖️Ast
/// 🌳️ Jack query abstract syntax tree.
#[derive(Clone, Debug, PartialEq)]
pub struct Query {
    pub clauses: Vec<Clause>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Clause {
    Match(Vec<Pattern>),
    Where(Expr),
    /// 🔭️ `WITH <items>` — projects the current bindings down to just the named vars (dropping
    /// everything else out of scope), no more than [`ReturnItem`] itself models: no aliasing, no
    /// `DISTINCT`, no `ORDER BY`/`SKIP`/`LIMIT`. A trailing filter is just the next
    /// [`Clause::Where`] in sequence — it needs no special-casing here.
    With(Vec<ReturnItem>),
    /// 🌀️ `UNWIND <source> AS <var>` — see [`UnwindClause`].
    Unwind(UnwindClause),
    /// 📞️ `CALL <name>(<args>...)` — see [`CallClause`] and [`call_procedure`].
    Call(CallClause),
    Return(Vec<ReturnItem>),
    Create(Pattern),
    Delete(Vec<String>),
    Set(Vec<Assignment>),
    Merge(Pattern),
}

/// 🌀️ `UNWIND <source> AS <var>` — flattens a list-valued source into per-row bindings of `var`.
#[derive(Clone, Debug, PartialEq)]
pub struct UnwindClause {
    pub source: ReturnItem,
    pub var: String,
}

/// 📞️ `CALL <name>(<args>...)` — a named procedure invocation with positional scalar arguments.
#[derive(Clone, Debug, PartialEq)]
pub struct CallClause {
    pub name: String,
    pub args: Vec<PropertyValue>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pattern {
    pub nodes: Vec<PatternNode>,
    pub edge: Option<PatternEdge>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternNode {
    pub var: String,
    pub kind: String,
    pub port: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternEdge {
    pub var: Option<String>,
    pub kind: Option<String>,
    pub directed: bool,
    pub right: PatternNode,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ReturnItem {
    Var(String),
    Property { var: String, prop: String },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Assignment {
    pub var: String,
    pub prop: String,
    pub value: PropertyValue,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Eq { var: String, prop: String, value: PropertyValue },
    Ne { var: String, prop: String, value: PropertyValue },
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum QueryResultKind {
    #[default]
    Table,
    Graph,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct QueryResult {
    #[value(default)]
    pub kind: QueryResultKind,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<PropertyValue>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub graph_snapshot_json: Option<String>,
}

impl QueryResult {
    pub fn table(columns: Vec<String>, rows: Vec<Vec<PropertyValue>>) -> Self {
        Self { kind: QueryResultKind::Table, columns, rows, graph_snapshot_json: None }
    }

    pub fn graph(columns: Vec<String>, graph_snapshot_json: String) -> Self {
        Self { kind: QueryResultKind::Graph, columns, rows: vec![], graph_snapshot_json: Some(graph_snapshot_json) }
    }
}
// #endregion 🔖️Ast

// #region 🔖️Lexer
#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum TokenClass {
    Keyword,
    Ident,
    Number,
    String,
    Operator,
    Punctuation,
    Error,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct TokenSpan {
    pub class: TokenClass,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Token {
    KwMatch,
    KwWhere,
    KwReturn,
    KwCreate,
    KwDelete,
    KwSet,
    KwMerge,
    KwWith,
    KwUnwind,
    KwCall,
    KwAs,
    Ident(String),
    Number(f64),
    StringLit(String),
    LParen,
    RParen,
    LBracket,
    RBracket,
    Colon,
    Comma,
    Dot,
    Eq,
    Ne,
    Arrow,
    /// ➖️ A bare `-`, the Cypher spelling of the pattern connector in `(a)-[r]->(b)`. `DashArrow`
    /// (`--`) is this grammar's own equal-status spelling; both reach the same `PatternEdge`.
    Dash,
    DashArrow,
    BackArrow,
    At,
    And,
    Or,
    Eof,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SpannedToken {
    pub(crate) token: Token,
    pub(crate) start: usize,
    pub(crate) end: usize,
}

fn token_class(token: &Token) -> TokenClass {
    match token {
        Token::KwMatch | Token::KwWhere | Token::KwReturn | Token::KwCreate | Token::KwDelete | Token::KwSet | Token::KwMerge | Token::KwWith | Token::KwUnwind | Token::KwCall | Token::KwAs | Token::And | Token::Or => TokenClass::Keyword,
        Token::Ident(_) => TokenClass::Ident,
        Token::Number(_) => TokenClass::Number,
        Token::StringLit(_) => TokenClass::String,
        Token::Eq | Token::Ne | Token::Arrow | Token::Dash | Token::DashArrow | Token::BackArrow | Token::At => TokenClass::Operator,
        Token::LParen | Token::RParen | Token::LBracket | Token::RBracket | Token::Colon | Token::Comma | Token::Dot => TokenClass::Punctuation,
        Token::Eof => TokenClass::Punctuation,
    }
}

fn push_spanned(tokens: &mut Vec<SpannedToken>, token: Token, start: usize, end: usize) {
    tokens.push(SpannedToken { token, start, end });
}

/// 🔑️ Uppercases and matches against Jack's clause/logic keyword table; anything else stays a
/// plain variable/property/kind identifier. Case-insensitive (Cypher heritage — `match`, `Match`,
/// `MATCH` are all the same token), unlike `dsl_core`'s own grammars which are case-sensitive.
fn keyword_or_ident(text: String) -> Token {
    match text.to_ascii_uppercase().as_str() {
        "MATCH" => Token::KwMatch,
        "WHERE" => Token::KwWhere,
        "RETURN" => Token::KwReturn,
        "CREATE" => Token::KwCreate,
        "DELETE" => Token::KwDelete,
        "SET" => Token::KwSet,
        "MERGE" => Token::KwMerge,
        "WITH" => Token::KwWith,
        "UNWIND" => Token::KwUnwind,
        "CALL" => Token::KwCall,
        "AS" => Token::KwAs,
        "AND" => Token::And,
        "OR" => Token::Or,
        _ => Token::Ident(text),
    }
}

/// 🪚️ `dsl_core` treats `.` as ident-continue (so `a.name` lexes as ONE ident there), but Jack's
/// `var.prop` property-access grammar needs `.` as its own token — splits it back apart here,
/// checking each piece against the keyword table too (defensive; keywords never legitimately
/// contain a dot, but this keeps the one keyword-recognition path authoritative).
fn push_ident_or_keyword_with_dots(text: &str, start: usize, out: &mut Vec<SpannedToken>) {
    let mut offset = 0usize;
    for (idx, part) in text.split('.').enumerate() {
        if idx > 0 {
            push_spanned(out, Token::Dot, start + offset, start + offset + 1);
            offset += 1;
        }
        if !part.is_empty() {
            let end = start + offset + part.len();
            push_spanned(out, keyword_or_ident(part.to_string()), start + offset, end);
        }
        offset += part.len();
    }
}

/// 🔬️ Converts one already-lexed `dsl_core` segment (containing no quotes or `!=` — those are
/// scanned by [`lex_spanned`] itself, ahead of delegating everything else) into Jack's own
/// richer, grammar-aware token stream.
fn push_dsl_core_segment(segment: &str, base_offset: usize, forgiving: bool, out: &mut Vec<SpannedToken>) -> Result<(), GraphDslError> {
    if segment.is_empty() {
        return Ok(());
    }
    let raw = semio_framework_dsl::lex(segment, &semio_framework_diagnostic::Limits::default(), forgiving).map_err(GraphDslError::Lex)?;
    for token in raw {
        if token.kind.is_trivia() || token.kind == semio_framework_dsl::TokenKind::Eof {
            continue;
        }
        let start = base_offset + token.byte_range.0 as usize;
        let end = base_offset + token.byte_range.1 as usize;
        let text = token.text.as_str().to_string();
        match token.kind {
            semio_framework_dsl::TokenKind::Ident => push_ident_or_keyword_with_dots(&text, start, out),
            // A lone `_` is `dsl_core`'s placeholder sigil; Jack has no placeholder concept of its
            // own, so it round-trips as an ordinary one-character identifier.
            semio_framework_dsl::TokenKind::Placeholder => push_spanned(out, Token::Ident(text), start, end),
            semio_framework_dsl::TokenKind::Int | semio_framework_dsl::TokenKind::Float => {
                let n: f64 = text.parse().map_err(GraphDslError::NumberFormat)?;
                push_spanned(out, Token::Number(n), start, end);
            }
            semio_framework_dsl::TokenKind::LParen => push_spanned(out, Token::LParen, start, end),
            semio_framework_dsl::TokenKind::RParen => push_spanned(out, Token::RParen, start, end),
            semio_framework_dsl::TokenKind::LBracket => push_spanned(out, Token::LBracket, start, end),
            semio_framework_dsl::TokenKind::RBracket => push_spanned(out, Token::RBracket, start, end),
            semio_framework_dsl::TokenKind::Colon => push_spanned(out, Token::Colon, start, end),
            semio_framework_dsl::TokenKind::Comma => push_spanned(out, Token::Comma, start, end),
            semio_framework_dsl::TokenKind::Equals => push_spanned(out, Token::Eq, start, end),
            semio_framework_dsl::TokenKind::At => push_spanned(out, Token::At, start, end),
            semio_framework_dsl::TokenKind::Arrow => push_spanned(out, Token::Arrow, start, end),
            // ➖️ A bare `-` is the pattern connector every Cypher-shaped query writes
            // (`(a)-[r:Kind]->(b)`), which is also what this dialect's own executor parser accepts
            // and what every committed example query in the repo uses. It used to fall into the
            // stray-character bucket below and every such query was reported
            // `unexpected character '-'` by lint/complete/hover/format while running perfectly.
            semio_framework_dsl::TokenKind::Minus => push_spanned(out, Token::Dash, start, end),
            semio_framework_dsl::TokenKind::DashArrow => push_spanned(out, Token::DashArrow, start, end),
            semio_framework_dsl::TokenKind::BackArrow => push_spanned(out, Token::BackArrow, start, end),
            // Double-quoted text delegated straight through `dsl_core` — unreachable in practice
            // since `lex_spanned` pre-scans and consumes every quote itself before ever
            // delegating a segment, kept only for defensive completeness.
            semio_framework_dsl::TokenKind::Text => push_spanned(out, Token::StringLit(text), start, end),
            // `{`/`}` aren't part of Jack's grammar (no map/object literals) — same "stray
            // character" treatment as an outright `os_dsl::TokenKind::Error` below. P2-M1's
            // promoted `< > & $ ;` tokens and STEP's `DotEnum` literal join this bucket too —
            // Jack has no grammar concept for any of them either.
            semio_framework_dsl::TokenKind::EdgeArrow
            | semio_framework_dsl::TokenKind::LBrace
            | semio_framework_dsl::TokenKind::RBrace
            | semio_framework_dsl::TokenKind::Caret
            | semio_framework_dsl::TokenKind::DotDot
            | semio_framework_dsl::TokenKind::Plus
            | semio_framework_dsl::TokenKind::Star
            | semio_framework_dsl::TokenKind::Slash
            | semio_framework_dsl::TokenKind::Fence
            | semio_framework_dsl::TokenKind::Lt
            | semio_framework_dsl::TokenKind::Gt
            | semio_framework_dsl::TokenKind::Amp
            | semio_framework_dsl::TokenKind::Dollar
            | semio_framework_dsl::TokenKind::Semicolon
            | semio_framework_dsl::TokenKind::DotEnum
            | semio_framework_dsl::TokenKind::Error => {
                if forgiving {
                    push_spanned(out, Token::Ident(text), start, end);
                } else {
                    return Err(GraphDslError::UnexpectedChar(text.chars().next().unwrap_or('?')));
                }
            }
            semio_framework_dsl::TokenKind::Whitespace | semio_framework_dsl::TokenKind::Newline | semio_framework_dsl::TokenKind::Comment | semio_framework_dsl::TokenKind::Eof => {
                unreachable!("trivia/Eof filtered above")
            }
        }
    }
    Ok(())
}

/// 🔬️ Jack's own lexer: unifies on `os_dsl::lex` for the shared token alphabet (idents,
/// numbers, punctuation, `(`/`)`/`[`/`]`, `->`/`--`/`<-`) but keeps two genuinely Cypher-specific
/// pieces local, since neither fits `dsl_core`'s grammar-independent alphabet: dual-quote strings
/// (`'x'`/`"x"` — Cypher heritage; `dsl_core` only ever lexes `"..."`) and the `!=` comparison
/// operator (`dsl_core` has no relational operators at all — it's a structural DSL alphabet, not
/// an expression language). Both are pre-scanned as their own tokens; every remaining run of
/// characters is delegated whole to `os_dsl::lex` and converted via [`push_dsl_core_segment`].
pub(crate) fn lex_spanned(input: &str, forgiving: bool) -> Result<Vec<SpannedToken>, GraphDslError> {
    let bytes = input.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 0usize;
    let mut seg_start = 0usize;

    while i < bytes.len() {
        let c = bytes[i];
        if c == b'\'' || c == b'"' {
            push_dsl_core_segment(&input[seg_start..i], seg_start, forgiving, &mut tokens)?;
            let quote = c;
            let start = i;
            i += 1;
            let content_start = i;
            let mut closed = false;
            while i < bytes.len() {
                if bytes[i] == b'\\' && i + 1 < bytes.len() {
                    i += 2;
                    continue;
                }
                if bytes[i] == quote {
                    closed = true;
                    break;
                }
                i += 1;
            }
            let raw = String::from_utf8_lossy(&bytes[content_start..i]).into_owned();
            if !closed {
                if forgiving {
                    push_spanned(&mut tokens, Token::StringLit(raw), start, i);
                    seg_start = i;
                    break;
                }
                return Err(GraphDslError::UnterminatedString);
            }
            i += 1;
            let text = semio_framework_dsl::unescape_text(&raw, forgiving).unwrap_or(raw);
            push_spanned(&mut tokens, Token::StringLit(text), start, i);
            seg_start = i;
            continue;
        }
        if c == b'!' && i + 1 < bytes.len() && bytes[i + 1] == b'=' {
            push_dsl_core_segment(&input[seg_start..i], seg_start, forgiving, &mut tokens)?;
            push_spanned(&mut tokens, Token::Ne, i, i + 2);
            i += 2;
            seg_start = i;
            continue;
        }
        i += 1;
    }
    push_dsl_core_segment(&input[seg_start..bytes.len()], seg_start, forgiving, &mut tokens)?;
    push_spanned(&mut tokens, Token::Eof, input.len(), input.len());
    Ok(tokens)
}

pub(crate) fn lex(input: &str) -> Result<Vec<Token>, GraphDslError> {
    lex_spanned(input, false).map(|spanned| spanned.into_iter().map(|row| row.token).collect())
}

/// 🎨️ Tokenize jack source for editor highlighting (never fails).
pub fn tokenize(input: &str) -> Vec<TokenSpan> {
    // 🔀️ Rewritten from `.map(..)` — `token_class` is async and cannot be called inside the sync
    // closure that used to build each `TokenSpan` (R10 residue shape #1).
    let rows = lex_spanned(input, true).unwrap_or_default();
    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        if matches!(row.token, Token::Eof) {
            continue;
        }
        let mut class = token_class(&row.token);
        if matches!(row.token, Token::StringLit(_)) {
            let quote = input.as_bytes().get(row.start);
            if quote == Some(&b'\'') || quote == Some(&b'"') {
                let closed = input.as_bytes().get(row.end.saturating_sub(1)) == quote;
                if !closed {
                    class = TokenClass::Error;
                }
            }
        }
        out.push(TokenSpan { class, start: row.start, end: row.end });
    }
    out
}
// #endregion 🔖️Lexer
