//! 🧭️ Token-stream `Recognizer` with explicit terminal predicates, family fragment merge and macro matchers.

use crate::{lex_with as core_lex_with, Limits, TextError, TextSpan, TokenKind as CoreKind};
use super::{GrammarFile, Production, Symbol};

//#region 🔖️Recognizer
/// 🧭️ Recognizer with explicit terminal predicates, family fragment merge, and macro matchers.
pub struct MacroMatcher {
    pub name: &'static str,
    pub try_match: fn(&str) -> bool,
}

/// 🧩️ Named grammar fragments (family kits) merged into Recognizer::compile.
#[derive(Default, Clone)]
pub struct FragmentRegistry {
    fragments: std::collections::HashMap<String, GrammarFile>,
}

impl FragmentRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, name: impl Into<String>, grammar: GrammarFile) {
        self.fragments.insert(name.into(), grammar);
    }

    pub fn get(&self, name: &str) -> Option<&GrammarFile> {
        self.fragments.get(name)
    }
}

pub struct Recognizer {
    pub(super) grammar: GrammarFile,
    macros: Vec<MacroMatcher>,
}

impl Recognizer {
    /// 🔗️ Compiles only the selected fragments and matchers, refusing unresolved declarations.
    pub fn compile(grammar: &GrammarFile, registry: &FragmentRegistry, macros: Vec<MacroMatcher>) -> Result<Self, TextError> {
        let mut merged = grammar.clone();
        let mut seen = std::collections::HashSet::<String>::new();
        for p in &grammar.productions {
            seen.insert(p.name.clone());
        }
        for use_name in &grammar.uses {
            let Some(frag) = registry.get(use_name) else { return Err(TextError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner, format!("unregistered grammar fragment `{use_name}`"), TextSpan::at(1, 1))); };
            for prod in &frag.productions {
                if seen.insert(prod.name.clone()) {
                    merged.productions.push(prod.clone());
                }
            }
        }
        Ok(Self { grammar: merged, macros })
    }

    fn find_production(&self, name: &str) -> Option<&Production> {
        self.grammar.productions.iter().find(|p| p.name == name)
    }

    /// ✅️ Recognizes text against the grammar start production. Lexes with this grammar's
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
        self.recognize_with_limits(text,&Limits::default())
    }

    /// 📏️ Recognizes an authored grammar within an explicit caller-owned lexical ceiling.
    pub fn recognize_with_limits(&self,text:&str,limits:&Limits)->Result<bool,TextError>{
        let raw = core_lex_with(text, limits, true, &self.grammar.lex)?;
        let tokens: Vec<_> = raw.into_iter().filter(|t| !t.kind.is_trivia() && t.kind != CoreKind::Eof).collect();
        let start = self.find_production(&self.grammar.start).ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("start production `{}` not found", self.grammar.start), TextSpan::at(1, 1)))?;
        match self.match_production(start, &tokens, 0, text) {
            Some(pos) => Ok(pos == tokens.len()),
            None => Ok(false),
        }
    }

    /// 📊️ Productions never reached while recognizing text. Forgiving for the same reason
    /// as [`Recognizer::recognize`] (P2-M1 raw-span support).
    pub fn uncovered_productions(&self, text: &str) -> Result<Vec<String>, TextError> {
        let raw = core_lex_with(text, &Limits::default(), true, &self.grammar.lex)?;
        let tokens: Vec<_> = raw.into_iter().filter(|t| !t.kind.is_trivia() && t.kind != CoreKind::Eof).collect();
        let mut covered = std::collections::HashSet::<String>::new();
        let start = self.find_production(&self.grammar.start).ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("start production `{}` not found", self.grammar.start), TextSpan::at(1, 1)))?;
        let _ = self.match_production_tracked(start, &tokens, 0, &mut covered, text);
        Ok(self.grammar.productions.iter().map(|p| p.name.clone()).filter(|n| !covered.contains(n)).collect())
    }

    fn match_production(&self, production: &Production, tokens: &[crate::SpannedToken], pos: usize, text: &str) -> Option<usize> {
        let mut covered = std::collections::HashSet::new();
        self.match_production_tracked(production, tokens, pos, &mut covered, text)
    }

    fn match_production_tracked(&self, production: &Production, tokens: &[crate::SpannedToken], pos: usize, covered: &mut std::collections::HashSet<String>, text: &str) -> Option<usize> {
        for alt in &production.alternatives {
            if let Some(next) = self.match_sequence_tracked(&alt.symbols, tokens, pos, covered, text) {
                covered.insert(production.name.clone());
                return Some(next);
            }
        }
        None
    }

    fn match_sequence_tracked(&self, symbols: &[Symbol], tokens: &[crate::SpannedToken], mut pos: usize, covered: &mut std::collections::HashSet<String>, text: &str) -> Option<usize> {
        for symbol in symbols {
            pos = self.match_symbol_tracked(symbol, tokens, pos, covered, text)?;
        }
        Some(pos)
    }

    fn match_symbol_tracked(&self, symbol: &Symbol, tokens: &[crate::SpannedToken], pos: usize, covered: &mut std::collections::HashSet<String>, text: &str) -> Option<usize> {
        match symbol {
            Symbol::Literal(literal) => {
                let token = tokens.get(pos)?;
                (token.text.as_str().as_ref() == literal.as_str()).then_some(pos + 1)
            }
            Symbol::Terminal(name) if matches!(name.to_uppercase().as_str(), "LINE" | "REST") => {
                let end = match name.to_uppercase().as_str() {
                    "LINE" => RawSpanEnd::Newline,
                    _ => RawSpanEnd::Eof,
                };
                Some(match_raw_span(tokens, pos, text, end))
            }
            Symbol::Terminal(name) if name.eq_ignore_ascii_case("HEX_BYTES") => match_hex_bytes(tokens, pos, text),
            Symbol::Terminal(name) => {
                let token = tokens.get(pos)?;
                terminal_matches(name, token).then_some(pos + 1)
            }
            Symbol::Ref(name) => {
                if let Some(production) = self.find_production(name) {
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

    /// 🔙️ Tries the LARGEST token span first, shrinking down to (and including, P2-P1) a
    /// zero-width match — real backtracking, unlike `Symbol::Star`'s single-pass greedy loop, which
    /// is why a macro is the right escape hatch for a Ref that must stop short of a following
    /// literal it would otherwise swallow (see the `hex` macro's own doc comment). The zero-width
    /// floor (`pos..` rather than `pos + 1..`) only ever fires for a matcher whose `try_match`
    /// accepts the empty string — `hex`'s does (an empty hex-encoded value is valid) — so this is a
    /// strict widening: every pre-existing macro's behavior on non-empty spans is unchanged.
    fn match_macro_span(&self, matcher: &MacroMatcher, tokens: &[crate::SpannedToken], pos: usize) -> Option<usize> {
        for end in (pos..=tokens.len()).rev() {
            let slice_text = slice_source_text(&tokens[pos..end]);
            if (matcher.try_match)(&slice_text) {
                return Some(end);
            }
        }
        None
    }
}

/// 📏️ Which delimiter ends a P2-M1 "raw span" terminal capture.
enum RawSpanEnd {
    /// `LINE` — rest of the current physical line (up to the next `\n`, or EOF if none) — obj's
    /// `o`/`g` names, stl's `solid <name>`/`endsolid <name>`, dxf's opaque group-code value lines.
    Newline,
    /// `REST` — everything remaining to end-of-document — txt's whole prose body.
    Eof,
}

/// ✂️ Captures a raw byte span of the ORIGINAL source starting at `tokens[pos]`'s byte
/// offset (or end-of-text if `pos` is already past the last token) through `end`, then returns the
/// token index just past every token that span swallowed — the span's interior is never
/// re-tokenized, matching the shared lexer's own token boundaries only at the far edge.
fn match_raw_span(tokens: &[crate::SpannedToken], pos: usize, text: &str, end: RawSpanEnd) -> usize {
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

/// 🧬️ Matches one contiguous hexadecimal octet literal across numeric and identifier token boundaries.
fn match_hex_bytes(tokens: &[crate::SpannedToken], pos: usize, text: &str) -> Option<usize> {
    let start = tokens.get(pos)?.byte_range.0 as usize;
    let bytes = text.as_bytes();
    let mut end = start;
    while bytes.get(end).is_some_and(u8::is_ascii_alphanumeric) { end += 1; }
    if end == start || (end-start)%2 != 0 || !bytes[start..end].iter().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte)) { return None; }
    let mut next = pos;
    while let Some(token) = tokens.get(next).filter(|token| (token.byte_range.0 as usize) < end) {
        if token.byte_range.1 as usize > end { return None; }
        next += 1;
    }
    (tokens.get(next-1)?.byte_range.1 as usize == end).then_some(next)
}

fn slice_source_text(tokens: &[crate::SpannedToken]) -> String {
    tokens.iter().map(|t| t.text.as_str().to_string()).collect::<Vec<_>>().join(" ")
}

/// 🏷️ Explicit terminal predicates — BOOL is Ident true|false.
fn terminal_matches(name: &str, token: &crate::SpannedToken) -> bool {
    let upper = name.to_uppercase();
    let text = token.text.as_str();
    let text = text.as_ref();
    match upper.as_str() {
        "BOOL" => matches!(token.kind, CoreKind::Ident) && (text == "true" || text == "false"),
        "IDENT" | "PLACEHOLDER" => matches!(token.kind, CoreKind::Ident | CoreKind::Placeholder),
        "INT" => matches!(token.kind, CoreKind::Int),
        "U64" => matches!(token.kind, CoreKind::Int) && (text=="0"||!text.starts_with('0')) && text.bytes().all(|byte|byte.is_ascii_digit()) && text.parse::<u64>().is_ok(),
        "U32" => matches!(token.kind, CoreKind::Int) && (text=="0"||!text.starts_with('0')) && text.bytes().all(|byte|byte.is_ascii_digit()) && text.parse::<u32>().is_ok(),
        "I64" => {let digits=text.strip_prefix('-').unwrap_or(text);matches!(token.kind,CoreKind::Int)&&text!="-0"&&(digits=="0"||!digits.starts_with('0'))&&digits.bytes().all(|byte|byte.is_ascii_digit())&&text.parse::<i64>().is_ok()},
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
        "DOTENUM" => matches!(token.kind, CoreKind::DotEnum),
        other => format!("{:?}", token.kind).to_uppercase() == other,
    }
}

//#endregion 🔖️Recognizer
