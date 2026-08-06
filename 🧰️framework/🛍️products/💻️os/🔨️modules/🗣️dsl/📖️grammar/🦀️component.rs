//! @emoji 📖️ `dsl_grammar` — the self-hosted `.grammar` spec format: a hand-authorable,
//! EBNF-style description of one language's productions, used as the *normative* artifact every
//! handcrafted grammar in the repo ships alongside its parser/printer. This crate parses and
//! prints the format itself (this crate's own `📖️grammar/📖️grammar.grammar` is written in it and
//! parses cleanly under its own parser — see the `self_hosting` test), and provides a recognizer
//! that can check a target document's tokens against a compiled grammar for the subset of
//! productions this v1 supports (see `//#region 🔖️Recognizer`'s doc comment for exactly what that
//! covers today and what it does not yet).
//!
//! Depends on `dsl_core` only, following the same "own pre-scan lexer delegating the shared
//! alphabet to `crate::os_dsl::core::lex`" pattern `math::graph::dsl` (Jack) established — `?` and `|`
//! aren't in the shared token alphabet (a structural-DSL alphabet has no need for them), so this
//! crate's lexer pre-scans those two characters itself and hands every other run of characters to
//! `crate::os_dsl::core::lex` unchanged.

use crate::os_dsl::core::{lex as core_lex, Limits, TextError, TextSpan, TokenKind as CoreKind};

//#region 🔖️Model
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemioDialect {
    Grammar,
    Protocol,
}

/// @emoji 📄️ One parsed `.grammar.semio` / `.protocol.semio` file: header directives + productions.
#[derive(Clone, Debug, PartialEq)]
pub struct GrammarFile {
    pub dialect: SemioDialect,
    pub id: String,
    pub extension: Option<String>,
    pub uses: Vec<String>,
    pub start: String,
    pub productions: Vec<Production>,
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

//#region 🔖️Lexer
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GKind {
    Ident,
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
/// characters whole to `crate::os_dsl::core::lex`, exactly like `math::graph::crate::os_dsl::lex_spanned` does
/// for its own two Cypher-specific extras.
fn lex(text: &str) -> Result<Vec<GToken>, TextError> {
    let bytes = text.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 0usize;
    let mut seg_start = 0usize;

    let push_segment = |seg_start: usize, seg_end: usize, tokens: &mut Vec<GToken>| -> Result<(), TextError> {
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
    };

    while i < bytes.len() {
        let c = bytes[i];
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
            push_segment(seg_start, i, &mut tokens)?;
            let line = text[..i].matches('\n').count() as u32 + 1;
            let col = (i - text[..i].rfind('\n').map(|p| p + 1).unwrap_or(0)) as u32 + 1;
            let span = TextSpan::with_length(line, col, 1);
            tokens.push(GToken { kind: if c == b'?' { GKind::Question } else { GKind::Pipe }, text: (c as char).to_string(), span });
            i += 1;
            seg_start = i;
            continue;
        }
        i += 1;
    }
    push_segment(seg_start, bytes.len(), &mut tokens)?;
    let eof_span = tokens.last().map(|t| t.span).unwrap_or(TextSpan::at(1, 1));
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
            Err(TextError::new(format!("expected {kind:?}, found {:?} {:?}", self.peek().kind, self.peek().text), self.peek().span.clone()))
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
                other => return Err(TextError::new(format!("expected a macro argument, found {other:?}"), cursor.peek().span.clone())),
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
        other => return Err(TextError::new(format!("expected a symbol, found {other:?}"), cursor.peek().span.clone())),
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
        return Err(TextError::new("a production alternative must have at least one symbol", cursor.peek().span.clone()));
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

/// @emoji 📖️ Parses one `.grammar` file. v1 requires every header directive and every production
/// on its own line — no line continuation. This is a real, currently-enforced limitation (not yet
/// a gap that's silently wrong): a production too long for one line is a signal to factor it into
/// named sub-productions, which the format already supports via `Ref`.
pub fn parse_grammar(text: &str) -> Result<GrammarFile, TextError> {
    let tokens = lex(text)?;
    let mut cursor = Cursor { tokens, pos: 0 };
    cursor.skip_newlines();

    cursor.skip_newlines();

    let dialect = if cursor.peek_ident("dialect") {
        cursor.expect_ident("dialect")?;
        let name = cursor.expect(GKind::Ident)?.text;
        cursor.skip_newlines();
        match name.as_str() {
            "grammar" => SemioDialect::Grammar,
            "protocol" => SemioDialect::Protocol,
            other => return Err(TextError::new(format!("unknown semio dialect `{other}`"), cursor.peek().span.clone())),
        }
    } else {
        SemioDialect::Grammar
    };

    cursor.expect_ident(match dialect {
        SemioDialect::Grammar => "grammar",
        SemioDialect::Protocol => "protocol",
    })?;
    let id = cursor.expect(GKind::Ident)?.text;
    cursor.skip_newlines();

    let mut extension = None;
    let mut uses = Vec::new();
    let mut start = None;
    let mut productions = Vec::new();

    loop {
        if cursor.peek().kind == GKind::Eof {
            break;
        }
        let head = cursor.expect(GKind::Ident)?;
        match head.text.as_str() {
            "extension" => {
                extension = Some(cursor.expect(GKind::Ident)?.text);
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
            _ => {
                cursor.pos -= 1; // un-consume; re-read as a production name
                productions.push(parse_production_line(&mut cursor)?);
                cursor.skip_newlines();
            }
        }
    }

    let start = start.ok_or_else(|| TextError::new("`.grammar` file is missing a `start` directive", cursor.peek().span.clone()))?;
    Ok(GrammarFile { dialect, id, extension, uses, start, productions })
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

/// @emoji ♻️ `canonicalize(canonicalize(x)) == canonicalize(x)` — the idempotence law every
/// technology's canonical form must satisfy.
pub fn canonicalize(text: &str) -> Result<String, TextError> {
    Ok(print_grammar(&parse_grammar(text)?))
}
//#endregion 🔖️Writer

//#region 🔖️FromRecordSpec
/// @emoji 🏗️ W1 skeleton of design ruling B-R2 ("`.grammar` = generated normative artifact"):
/// mechanically lowers a `crate::os_dsl::schema::RecordSpec` — the same value every `#[derive(crate::os_dsl::Dsl...)]`
/// macro already builds — into a [`GrammarFile`] a [`Recognizer`] can check real printed text
/// against. v1 scope (real, not a stub, but genuinely partial): a flat `RecordLayout::Inline`/
/// `RecordLayout::Lines` record whose fields are all `Shape::Bool`/`Int`/`UInt`/`Float`/`Text`/
/// `Bytes64`/`List` of one of those — exactly the shape `dsl_derive`'s `record_codegen` already
/// covers for the common case, and the one this crate's own `round_trip_matrix_over_representative_
/// grammars` test exercises by hand today. Any other `Shape` (`Record`/`Statements`/`Block`/`Map`/
/// `Table`/`Wire`/…) is a genuine gap, not silently approximated — `Err` names the unsupported shape
/// so a caller knows exactly what's missing rather than shipping a grammar that would falsely reject
/// (or worse, falsely accept) real documents. Full coverage — including nested records/statements —
/// is deferred to a later wave (`POLICY_GRAMMAR_FILE_ALLOWLIST` in the root `📜️script.ts` tracks
/// which apps still lack a committed `.grammar` file; every app is seeded there today since no app
/// has adopted B-R2 yet).
pub fn from_record_spec(id: &str, spec: &crate::os_dsl::schema::RecordSpec) -> Result<GrammarFile, String> {
    let mut symbols = Vec::new();
    if let Some(keyword) = &spec.keyword {
        symbols.push(Symbol::Literal(keyword.clone()));
    }

    let mut positional_fields: Vec<&crate::os_dsl::schema::FieldSpec> = spec.fields.iter().filter(|f| f.position.is_some()).collect();
    positional_fields.sort_by_key(|f| f.position.expect("filtered on position.is_some()"));
    for field in positional_fields {
        let symbol = terminal_for_shape(&field.shape)?;
        symbols.push(if field.optional { Symbol::Optional(Box::new(symbol)) } else { symbol });
    }

    let mut keyed_fields: Vec<&crate::os_dsl::schema::FieldSpec> = spec.fields.iter().filter(|f| f.position.is_none()).collect();
    keyed_fields.sort_by_key(|f| f.id);
    for field in keyed_fields {
        let value_symbol = terminal_for_shape(&field.shape)?;
        let entry = Symbol::Group(vec![Alternative { symbols: vec![Symbol::Literal(field.key.clone()), Symbol::Terminal("EQUALS".to_string()), value_symbol] }]);
        symbols.push(if field.optional { Symbol::Optional(Box::new(entry)) } else { entry });
    }

    let production = Production { name: "document".to_string(), alternatives: vec![Alternative { symbols }] };
    Ok(GrammarFile { dialect: SemioDialect::Grammar, id: id.to_string(), extension: None, uses: Vec::new(), start: "document".to_string(), productions: vec![production] })
}

/// @emoji 🧭️ [`from_record_spec`]'s per-field shape lowering — see that function's doc comment for
/// exactly which `Shape` variants v1 covers. `Text` matches `{IDENT | TEXT}` (a `Group` alternation)
/// since the engine's "bare-preferred" printing law (`crate::os_dsl::core::is_bare_ident`) means an identical
/// field can print as either token kind depending on its value.
fn terminal_for_shape(shape: &crate::os_dsl::schema::Shape) -> Result<Symbol, String> {
    match shape {
        crate::os_dsl::schema::Shape::Bool => Ok(Symbol::Terminal("IDENT".to_string())),
        crate::os_dsl::schema::Shape::Int | crate::os_dsl::schema::Shape::UInt => Ok(Symbol::Terminal("INT".to_string())),
        crate::os_dsl::schema::Shape::Float => Ok(Symbol::Terminal("FLOAT".to_string())),
        crate::os_dsl::schema::Shape::Text => Ok(Symbol::Group(vec![Alternative { symbols: vec![Symbol::Terminal("IDENT".to_string())] }, Alternative { symbols: vec![Symbol::Terminal("TEXT".to_string())] }])),
        crate::os_dsl::schema::Shape::Bytes64 => Ok(Symbol::Terminal("TEXT".to_string())),
        crate::os_dsl::schema::Shape::List(inner) => Ok(Symbol::Star(Box::new(terminal_for_shape(inner)?))),
        other => Err(format!("crate::os_dsl::grammar::from_record_spec: v1 lowering does not yet cover {other:?} — see this function's doc comment")),
    }
}
//#endregion 🔖️FromRecordSpec

//#region 🔖️Recognizer
/// @emoji 🧭️ What the recognizer can check TODAY: `Literal`/`Terminal`/`Ref`/`Group`/quantifiers
/// against a real `dsl_core`-lexed token stream, plus macros that have a registered matcher. Only
/// the `edge` macro has one so far (backed by `crate::os_dsl::notation::parse_edge_text`) — every other
/// macro name (`table`, `quantity`, `props`, …) is accepted syntactically by the parser above but
/// has NO recognizer support yet, because the shared `dsl_notation` piece-parser library those
/// macros are supposed to delegate to doesn't exist yet (tracked in the ticket's progress.md).
/// Corpus-agreement / production-coverage / generative-sampling sweeps (the fuller conformance
/// story in the architecture plan) are NOT implemented here yet — this is a real but partial v1.
pub struct MacroMatcher {
    pub name: &'static str,
    pub try_match: fn(&str) -> bool,
}

pub struct Recognizer<'g> {
    grammar: &'g GrammarFile,
    macros: Vec<MacroMatcher>,
}

impl<'g> Recognizer<'g> {
    pub fn compile(grammar: &'g GrammarFile) -> Self {
        Self { grammar, macros: default_macros() }
    }

    fn find_production(&self, name: &str) -> Option<&Production> {
        self.grammar.productions.iter().find(|p| p.name == name)
    }

    /// @emoji ✅️ Recognizes `text` against the grammar's `start` production. PEG-style: the first
    /// alternative that matches wins, quantifiers are greedy, no left recursion is supported (a
    /// left-recursive production will simply fail to terminate the intended way — not detected or
    /// guarded against in this v1). Matches on `crate::os_dsl::core::lex` tokens directly (the same lexer real
    /// app documents are lexed with), skipping all trivia.
    pub fn recognize(&self, text: &str) -> Result<bool, TextError> {
        let raw = core_lex(text, &Limits::default(), false)?;
        let tokens: Vec<_> = raw.into_iter().filter(|t| !t.kind.is_trivia() && t.kind != CoreKind::Eof).collect();
        let start = self
            .find_production(&self.grammar.start)
            .ok_or_else(|| TextError::new(format!("start production `{}` not found", self.grammar.start), TextSpan::at(1, 1)))?;
        match self.match_production(start, &tokens, 0) {
            Some(pos) => Ok(pos == tokens.len()),
            None => Ok(false),
        }
    }

    fn match_production(&self, production: &Production, tokens: &[crate::os_dsl::core::SpannedToken], pos: usize) -> Option<usize> {
        for alt in &production.alternatives {
            if let Some(next) = self.match_sequence(&alt.symbols, tokens, pos) {
                return Some(next);
            }
        }
        None
    }

    fn match_sequence(&self, symbols: &[Symbol], tokens: &[crate::os_dsl::core::SpannedToken], mut pos: usize) -> Option<usize> {
        for symbol in symbols {
            pos = self.match_symbol(symbol, tokens, pos)?;
        }
        Some(pos)
    }

    fn match_symbol(&self, symbol: &Symbol, tokens: &[crate::os_dsl::core::SpannedToken], pos: usize) -> Option<usize> {
        match symbol {
            Symbol::Literal(text) => {
                let token = tokens.get(pos)?;
                (token.text.as_str().as_ref() == text.as_str()).then_some(pos + 1)
            }
            Symbol::Terminal(name) => {
                let token = tokens.get(pos)?;
                (token_kind_name(token.kind) == name.to_uppercase()).then_some(pos + 1)
            }
            Symbol::Ref(name) => {
                if let Some(production) = self.find_production(name) {
                    self.match_production(production, tokens, pos)
                } else if let Some(matcher) = self.macros.iter().find(|m| m.name == name) {
                    self.match_macro_span(matcher, tokens, pos)
                } else {
                    None
                }
            }
            Symbol::Macro(name, _args) => {
                let matcher = self.macros.iter().find(|m| &m.name == name)?;
                self.match_macro_span(matcher, tokens, pos)
            }
            Symbol::Group(alts) => alts.iter().find_map(|alt| self.match_sequence(&alt.symbols, tokens, pos)),
            Symbol::Optional(inner) => Some(self.match_symbol(inner, tokens, pos).unwrap_or(pos)),
            Symbol::Star(inner) => {
                let mut cur = pos;
                while let Some(next) = self.match_symbol(inner, tokens, cur) {
                    if next == cur {
                        break;
                    }
                    cur = next;
                }
                Some(cur)
            }
            Symbol::Plus(inner) => {
                let first = self.match_symbol(inner, tokens, pos)?;
                let mut cur = first;
                loop {
                    match self.match_symbol(inner, tokens, cur) {
                        Some(next) if next != cur => cur = next,
                        _ => break,
                    }
                }
                Some(cur)
            }
        }
    }

    /// @emoji 🧩️ A macro consumes a variable-length run of tokens it alone understands; since this
    /// v1 has no token-level matcher API for macros yet (only a whole-remaining-text `try_match`
    /// probe), it greedily tries matching against every possible remaining-token count from
    /// longest to shortest and re-lexes that slice's text back out. This is correct but not
    /// efficient — fine for fixture-sized documents, not for production use at scale.
    fn match_macro_span(&self, matcher: &MacroMatcher, tokens: &[crate::os_dsl::core::SpannedToken], pos: usize) -> Option<usize> {
        for end in (pos + 1..=tokens.len()).rev() {
            let slice_text = slice_source_text(&tokens[pos..end]);
            if (matcher.try_match)(&slice_text) {
                return Some(end);
            }
        }
        None
    }
}

fn slice_source_text(tokens: &[crate::os_dsl::core::SpannedToken]) -> String {
    tokens.iter().map(|t| t.text.as_str().to_string()).collect::<Vec<_>>().join(" ")
}

fn token_kind_name(kind: CoreKind) -> String {
    format!("{kind:?}").to_uppercase()
}

fn default_macros() -> Vec<MacroMatcher> {
    vec![MacroMatcher { name: "edge", try_match: |text| crate::os_dsl::notation::parse_edge_text(text).is_ok() }]
}
//#endregion 🔖️Recognizer

//#region 🔖️ProtocolVerify
/// @emoji 📡️ Byte-level protocol conformance: walks a parsed `.protocol.semio` spec against encoded
/// bytes. v1 checks container magic and minimum header span for `dag.pack`-shaped fixtures.
pub fn verify_protocol_bytes(spec: &GrammarFile, bytes: &[u8]) -> Result<(), String> {
    if spec.dialect != SemioDialect::Protocol {
        return Err("verify_protocol_bytes requires dialect protocol".to_string());
    }
    if bytes.len() < 8 {
        return Err("protocol bytes shorter than magic".to_string());
    }
    if &bytes[..8] != &[0x89, b'S', b'P', b'K', 0x0D, 0x0A, 0x1A, 0x0A] {
        return Err("SPK magic mismatch".to_string());
    }
    if bytes.len() < 32 && spec.id.contains("pack") {
        return Err("pack header requires 32 bytes".to_string());
    }
    Ok(())
}
//#endregion 🔖️ProtocolVerify

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_minimal_grammar_header() {
        let g = parse_grammar("grammar demo\nstart doc\ndoc = \"hello\"\n").expect("parse_grammar");
        assert_eq!(g.id, "demo");
        assert_eq!(g.start, "doc");
        assert_eq!(g.productions.len(), 1);
        assert_eq!(g.productions[0].alternatives[0].symbols, vec![Symbol::Literal("hello".to_string())]);
    }

    #[test]
    fn parses_extension_and_uses() {
        let g = parse_grammar("grammar fem2d\nextension fem2d\nuse core\nuse family-sheet\nstart document\ndocument = header\nheader = \"fem2d\" TEXT\n")
            .expect("parse_grammar");
        assert_eq!(g.extension, Some("fem2d".to_string()));
        assert_eq!(g.uses, vec!["core".to_string(), "family-sheet".to_string()]);
        assert_eq!(g.productions.len(), 2);
    }

    #[test]
    fn parses_terminal_vs_ref_vs_macro() {
        let g = parse_grammar("grammar demo\nstart doc\ndoc = TEXT node table(\"rows\", row)\nrow = IDENT\n").expect("parse_grammar");
        let symbols = &g.productions[0].alternatives[0].symbols;
        assert_eq!(symbols[0], Symbol::Terminal("TEXT".to_string()));
        assert_eq!(symbols[1], Symbol::Ref("node".to_string()));
        assert_eq!(symbols[2], Symbol::Macro("table".to_string(), vec![MacroArg::Literal("rows".to_string()), MacroArg::Ident("row".to_string())]));
    }

    #[test]
    fn parses_alternation_group_and_quantifiers() {
        let g = parse_grammar("grammar demo\nstart doc\ndoc = {\"a\" | \"b\"}? node* row+\nnode = IDENT\nrow = IDENT\n").expect("parse_grammar");
        let symbols = &g.productions[0].alternatives[0].symbols;
        match &symbols[0] {
            Symbol::Optional(inner) => match inner.as_ref() {
                Symbol::Group(alts) => {
                    assert_eq!(alts.len(), 2);
                    assert_eq!(alts[0].symbols, vec![Symbol::Literal("a".to_string())]);
                    assert_eq!(alts[1].symbols, vec![Symbol::Literal("b".to_string())]);
                }
                other => panic!("expected Group, got {other:?}"),
            },
            other => panic!("expected Optional, got {other:?}"),
        }
        assert!(matches!(&symbols[1], Symbol::Star(_)));
        assert!(matches!(&symbols[2], Symbol::Plus(_)));
    }

    #[test]
    fn round_trip_matrix_over_representative_grammars() {
        let sources = vec![
            "grammar demo\nstart doc\ndoc = \"hello\"\n",
            "grammar fem2d\nextension fem2d\nuse core\nstart document\ndocument = header body\nheader = \"fem2d\" TEXT\nbody = row*\nrow = IDENT FLOAT?\n",
            "grammar demo\nstart doc\ndoc = {\"a\" | \"b\"} node+\nnode = IDENT\n",
        ];
        for source in sources {
            let parsed = parse_grammar(source).unwrap_or_else(|e| panic!("parse of {source:?} failed: {e:?}"));
            let printed = print_grammar(&parsed);
            let reparsed = parse_grammar(&printed).unwrap_or_else(|e| panic!("reparse of canonical {printed:?} failed: {e:?}"));
            assert_eq!(reparsed, parsed, "round trip mismatch for {source:?} -> {printed:?}");
            let canonical_twice = canonicalize(&printed).expect("canonicalize");
            assert_eq!(canonical_twice, printed, "canonicalize is not idempotent for {printed:?}");
        }
    }

    #[test]
    fn missing_start_directive_is_an_error() {
        let err = parse_grammar("grammar demo\ndoc = \"hello\"\n").unwrap_err();
        assert!(err.message.contains("start"), "unexpected message: {}", err.message);
    }

    /// @emoji 🪞️ This crate's own format description parses under the parser it defines — the
    /// self-hosting proof the architecture plan calls for.
    #[test]
    fn self_hosting_grammar_grammar_parses_and_round_trips() {
        let source = include_str!("../../📖️grammar.grammar.semio");
        let parsed = parse_grammar(source).expect("dsl_grammar's own grammar.grammar must parse under its own parser");
        assert_eq!(parsed.id, "grammar");
        let printed = print_grammar(&parsed);
        let reparsed = parse_grammar(&printed).expect("canonical print of grammar.grammar must reparse");
        assert_eq!(reparsed, parsed);
    }

    #[test]
    fn recognizer_matches_plain_arrow_via_registered_edge_macro() {
        let grammar = parse_grammar("grammar demo\nstart doc\ndoc = edge\n").expect("parse_grammar");
        let recognizer = Recognizer::compile(&grammar);
        assert!(recognizer.recognize("a->b").expect("recognize"));
        assert!(recognizer.recognize("a -[e1:Connection]->b").expect("recognize"));
        assert!(!recognizer.recognize("a-> ->").expect("recognize"));
    }

    #[test]
    fn recognizer_matches_literals_terminals_and_quantifiers() {
        let grammar = parse_grammar("grammar demo\nstart doc\ndoc = \"beam\" IDENT node*\nnode = IDENT\n").expect("parse_grammar");
        let recognizer = Recognizer::compile(&grammar);
        assert!(recognizer.recognize("beam e3 n1 n2").expect("recognize"));
        assert!(recognizer.recognize("beam e3").expect("recognize"));
        assert!(!recognizer.recognize("beam").expect("recognize"));
    }

    //#region 🔖️FromRecordSpecTests
    /// 🧬️ Mirrors `pack_cli`'s own `sample_spec()` (three keyed scalar fields, no keyword) — the
    /// "smallest possible schema" the pack CLI already uses as its own demonstration fixture.
    fn sample_record_spec() -> crate::os_dsl::schema::RecordSpec {
        crate::os_dsl::schema::RecordSpec::new(
            None,
            crate::os_dsl::schema::RecordLayout::Lines,
            vec![crate::os_dsl::schema::FieldSpec::new(1, "name", crate::os_dsl::schema::Shape::Text), crate::os_dsl::schema::FieldSpec::new(2, "age", crate::os_dsl::schema::Shape::UInt), crate::os_dsl::schema::FieldSpec::new(3, "active", crate::os_dsl::schema::Shape::Bool)],
        )
    }

    #[test]
    fn from_record_spec_lowers_a_flat_scalar_record_and_recognizes_its_own_printed_text() {
        let spec = sample_record_spec();
        let grammar = from_record_spec("sample", &spec).expect("from_record_spec");
        assert_eq!(grammar.id, "sample");
        assert_eq!(grammar.start, "document");
        // The generated grammar must itself be a well-formed `.grammar` file: round trips through
        // this crate's own parser/printer exactly like a hand-authored one.
        let printed = print_grammar(&grammar);
        let reparsed = parse_grammar(&printed).unwrap_or_else(|e| panic!("generated grammar failed to reparse: {e}\n{printed}"));
        assert_eq!(reparsed, grammar);

        // The generated grammar accepts REAL printed text from the record it was lowered from — not
        // hand-typed fixture text — proving the mechanism against the actual printer, not a guess.
        let mut record = crate::os_dsl::schema::RecordValue::default();
        record.fields.insert(1, crate::os_dsl::schema::FieldValue::Text("Ada Lovelace".to_string()));
        record.fields.insert(2, crate::os_dsl::schema::FieldValue::UInt(42));
        record.fields.insert(3, crate::os_dsl::schema::FieldValue::Bool(true));
        let mut writer = crate::os_dsl::schema::Writer::new();
        crate::os_dsl::schema::print_record(&record, &spec, &mut writer);
        let text = writer.render(crate::os_dsl::schema::JoinMode::Inline);

        let recognizer = Recognizer::compile(&grammar);
        assert!(recognizer.recognize(&text).expect("recognize"), "generated grammar must accept its own record's real printed text: {text:?}");
        assert!(!recognizer.recognize("this is not a valid record at all").expect("recognize"), "generated grammar must reject unrelated text");
    }

    #[test]
    fn from_record_spec_reports_unsupported_shapes_instead_of_silently_approximating() {
        let spec = crate::os_dsl::schema::RecordSpec::new(None, crate::os_dsl::schema::RecordLayout::Inline, vec![crate::os_dsl::schema::FieldSpec::new(1, "nested", crate::os_dsl::schema::Shape::Map(Box::new(crate::os_dsl::schema::Shape::Text)))]);
        let error = from_record_spec("unsupported", &spec).expect_err("Shape::Map must be a reported gap, not a silent approximation");
        assert!(error.contains("Map"), "error should name the unsupported shape: {error}");
    }
    //#endregion 🔖️FromRecordSpecTests
}
//#endregion 🔖️Tests
