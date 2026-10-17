//! 🌳️ Recursive-descent parsers for `.grammar.semio` and `.protocol.semio` sources over one shared token cursor.

use semio_framework_diagnostic::TextError;
use semio_framework_dsl_lexical::lexer::{CommentDialect, LexOptions, StringEscape, StringMode};
use super::{Alternative, Block, Cond, CondOp, Count, DispatchOrder, Field, Framing, GKind, GToken, GrammarFile, MacroArg, NestedDispatch, Prim, Production, ProtocolFile, RepeatArm, RepeatDispatch, SemioDialect, Symbol, lex, prim_fixed_width};

//#region 🔖️Parser
pub(super) struct Cursor {
    pub(super) tokens: Vec<GToken>,
    pub(super) pos: usize,
}

impl Cursor {
    fn peek(&self) -> &GToken {
        &self.tokens[self.pos]
    }

    pub(super) fn advance(&mut self) -> GToken {
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

    pub(super) fn expect(&mut self, kind: GKind) -> Result<GToken, TextError> {
        if self.peek().kind == kind {
            Ok(self.advance())
        } else {
            Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected {kind:?}, found {:?} {:?}", self.peek().kind, self.peek().text), self.peek().span))
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
            Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected keyword `{expected_text}`, found `{}`", token.text), token.span))
        }
    }

    fn expect_ident_or_int(&mut self) -> Result<GToken, TextError> {
        match self.peek().kind {
            GKind::Ident | GKind::Int => Ok(self.advance()),
            other => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected ident or int, found {other:?}"), self.peek().span)),
        }
    }
}

/// 🔢 The largest exact repetition `n(…)` a grammar may state. Exact repetition expands to
/// the copies it names, so the bound is what keeps a typo from building an enormous production.
const EXACT_REPETITION_MAXIMUM: usize = 1_024;

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
                other => return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a macro argument, found {other:?}"), cursor.peek().span)),
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
        // 🧺 `name (group)` and `name(args)` are told apart by ADJACENCY: a macro call's `(`
        // touches its name, a grouped alternative's does not. Spans carry each token's line,
        // column and length, so the distinction survives trivia being dropped at lex time — which
        // is what lets `( )` mean a group everywhere else without stealing the macro-call form.
        GKind::LBrace => {
            cursor.advance();
            // 🔁 `parse_atom` -> `parse_alternatives` -> `parse_sequence` -> `parse_atom` is a
            // recursion cycle through fns (E0733: an fn's future can't be infinite-
            // sized) — `Box::pin` breaks it at this one edge, per R10 residue shape 3.
            let alts = parse_alternatives(cursor)?;
            cursor.expect(GKind::RBrace)?;
            Symbol::Group(alts)
        }
        // 🧺 `( … )` is a grouped alternation, the ABNF spelling of `{ … }`. It is unambiguous
        // against a macro-call argument list because a macro call is only ever read when the
        // `(` FOLLOWS an ident in the same atom, which the `GKind::Ident` arm below handles.
        GKind::LParen => {
            cursor.advance();
            let alts = parse_alternatives(cursor)?;
            cursor.expect(GKind::RParen)?;
            Symbol::Group(alts)
        }
        // 🧺 `[ … ]` is an optional group — `{ … }?` in this language's own spelling.
        GKind::LBracket => {
            cursor.advance();
            let alts = parse_alternatives(cursor)?;
            cursor.expect(GKind::RBracket)?;
            Symbol::Optional(Box::new(Symbol::Group(alts)))
        }
        // 🔁 `*x`, `1*x` and `0*x` are PREFIX repetition — the ABNF spelling of this language's
        // own postfix `x*` and `x+`. A valid postfix quantifier is consumed by the quantifier
        // match below, so a repetition operator can only reach an atom's head as a prefix one.
        GKind::Star => {
            cursor.advance();
            Symbol::Star(Box::new(parse_atom(cursor)?))
        }
        GKind::Int if cursor.tokens.get(cursor.pos + 1).is_some_and(|token| token.kind == GKind::Star) => {
            let count = cursor.advance();
            cursor.advance();
            let inner = Box::new(parse_atom(cursor)?);
            match count.text.as_str() {
                "0" => Symbol::Star(inner),
                "1" => Symbol::Plus(inner),
                other => return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("a repetition count of `{other}` has no symbol in this grammar dialect; write the repeated symbol out"), count.span)),
            }
        }
        // 🔢 `n(…)` is EXACT repetition — `9(COMMA field)` is nine of that group, the shape a
        // fixed-column record format states directly. It expands to the n copies it means, so the
        // recogniser needs no counted-repetition node of its own.
        GKind::Int => {
            let count = cursor.advance();
            let times = match count.text.parse::<usize>() {
                Ok(0) => return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "an exact repetition count must be positive", count.span)),
                Ok(times) if times <= EXACT_REPETITION_MAXIMUM => times,
                _ => return Err(TextError::new(semio_framework_value::ValueRefusalKind::WorkLimit, format!("a repetition count of `{}` exceeds {EXACT_REPETITION_MAXIMUM}", count.text), count.span)),
            };
            let inner = parse_atom(cursor)?;
            Symbol::Group(vec![Alternative { symbols: std::iter::repeat_n(inner, times).collect() }])
        }
        GKind::Ident => {
            let token = cursor.advance();
            let adjacent = cursor.peek().kind == GKind::LParen && cursor.peek().span.line == token.span.line && cursor.peek().span.column == token.span.column.saturating_add(token.span.length);
            let name = token.text;
            if adjacent {
                Symbol::Macro(name, parse_macro_args(cursor)?)
            } else if is_all_upper(&name) {
                Symbol::Terminal(name)
            } else {
                Symbol::Ref(name)
            }
        }
        other => return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a symbol, found {other:?}"), cursor.peek().span)),
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
            // ↩️ An INDENTED line continues the sequence above it: a long rule is routinely wrapped
            // that way. The run of newlines is only consumed when what follows is indented AND is
            // not a production head (`IDENT "="`), so a rule that simply ended still ends here.
            GKind::Newline => {
                let mut lookahead = cursor.pos;
                while cursor.tokens[lookahead].kind == GKind::Newline {
                    lookahead += 1;
                }
                let next = &cursor.tokens[lookahead];
                let heads_production = next.kind == GKind::Ident && cursor.tokens.get(lookahead + 1).is_some_and(|token| token.kind == GKind::Equals);
                if next.kind == GKind::Eof || next.span.column <= 1 || heads_production || symbols.is_empty() {
                    break;
                }
                cursor.pos = lookahead;
            }
            GKind::Pipe | GKind::Eof | GKind::RBrace | GKind::RParen | GKind::RBracket => break,
            _ => symbols.push(parse_atom(cursor)?),
        }
    }
    if symbols.is_empty() {
        return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "a production alternative must have at least one symbol", cursor.peek().span));
    }
    Ok(Alternative { symbols })
}

fn parse_alternatives(cursor: &mut Cursor) -> Result<Vec<Alternative>, TextError> {
    let mut alts = vec![parse_sequence(cursor)?];
    loop {
        // ↩️ A long alternation is routinely wrapped across lines with the operator leading the
        // continuation line. A newline run is only consumed when an alternation operator is what
        // follows it, so a production that simply ended still ends at its own newline.
        if cursor.peek().kind == GKind::Newline {
            let mut lookahead = cursor.pos;
            while cursor.tokens[lookahead].kind == GKind::Newline {
                lookahead += 1;
            }
            if cursor.tokens[lookahead].kind != GKind::Pipe {
                break;
            }
            cursor.pos = lookahead;
        }
        if cursor.peek().kind != GKind::Pipe {
            break;
        }
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

/// 📖️ Parses one `.grammar` file. Protocol dialect sources project through
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

    // 🪪 `dialect grammar <id>` names the dialect and the grammar on ONE line — the compact
    // spelling the whole stdio family uses. A third ident on the dialect line IS the grammar id,
    // and no separate `grammar` line follows it.
    let mut inline_id = None;
    let dialect = if cursor.peek_ident("dialect") {
        let keyword = cursor.expect(GKind::Ident)?;
        let name = cursor.expect(GKind::Ident)?;
        if cursor.peek().kind == GKind::Ident && cursor.peek().span.line == keyword.span.line {
            inline_id = Some(cursor.advance().text);
        }
        cursor.skip_newlines();
        match name.text.as_str() {
            "grammar" => SemioDialect::Grammar,
            "protocol" => return Ok(project_protocol(parse_protocol(text)?)),
            other => return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown semio dialect `{other}`"), name.span)),
        }
    } else {
        SemioDialect::Grammar
    };

    let id = match inline_id {
        Some(id) => id,
        None => {
            cursor.expect_ident("grammar")?;
            let id = parse_grammar_id(&mut cursor)?;
            cursor.skip_newlines();
            id
        }
    };

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
        // 🏷️ A header keyword immediately followed by `=` is a PRODUCTION NAME. `comment`, `start`
        // and `string` are ordinary words a grammar is entitled to define a rule for — the html
        // source defines `comment` for HTML's own `<!-- -->` — and a directive never carries an
        // `=`, so the two can never be confused.
        let directive = if cursor.peek().kind == GKind::Equals { "" } else { head.text.as_str() };
        match directive {
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
                    other => return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown `comment` directive `{other}` (expected `none`/`line`/`block`)"), head.span)),
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
                    other => return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown `string` quote `{other}` (expected `double`/`single`)"), head.span)),
                };
                let mode = cursor.expect(GKind::Ident)?.text;
                let escape = match mode.as_str() {
                    "raw" => StringEscape::Raw,
                    "backslash" => StringEscape::Backslash,
                    "doubled" => StringEscape::Doubled,
                    other => return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown `string` escape mode `{other}` (expected `raw`/`backslash`/`doubled`)"), head.span)),
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
    // 🎬 A file with no `start` directive starts at the production named `root` — the spelling the
    // whole stdio family uses. Nothing else is inferred: a file with neither is still refused.
    let start = match start.or_else(|| productions.iter().any(|production| production.name == "root").then(|| "root".to_string())) {
        Some(s) => s,
        None => return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "`.grammar` file is missing a `start` directive and defines no `root` production", cursor.peek().span)),
    };
    let lex = LexOptions { strings, comment: CommentDialect { line: comment_line, block: comment_block } };
    Ok(GrammarFile { dialect: SemioDialect::Grammar, id, extension, uses, start, productions, lex })
}

pub fn is_protocol_source(text: &str) -> bool {
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
    token.text.parse::<usize>().map_err(|_| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected unsigned integer, found `{}`", token.text), token.span))
}

fn parse_u64_literal(cursor: &mut Cursor) -> Result<u64, TextError> {
    let first = cursor.expect_ident_or_int()?;
    if first.kind == GKind::Int && first.text == "0" && cursor.peek().kind == GKind::Ident {
        let rest = cursor.peek().text.clone();
        if let Some(hex) = rest.strip_prefix('x').or_else(|| rest.strip_prefix('X')) {
            cursor.advance();
            return u64::from_str_radix(hex, 16).map_err(|_| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("invalid hex literal `0{rest}`"), first.span));
        }
    }
    if let Some(hex) = first.text.strip_prefix("0x").or_else(|| first.text.strip_prefix("0X")) {
        return u64::from_str_radix(hex, 16).map_err(|_| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("invalid hex literal `{}`", first.text), first.span));
    }
    first.text.parse::<u64>().map_err(|_| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected unsigned integer, found `{}`", first.text), first.span))
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
        _ => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected Array count (Fixed/Varint/Field)", cursor.peek().span)),
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
                            other => return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown `endian` mode `{other}` (expected `le`/`be`)"), cursor.peek().span)),
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
        other => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a protocol type, found {other:?}"), cursor.peek().span)),
    }
}

/// 🔀️ P2-M2 item 4: `if <field> <op> <value>` guard, `<op>` one of `eq|ne|lt|le|gt|ge`
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
        "bits" => CondOp::BitsSet,
        other => return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown condition operator `{other}` (expected eq/ne/lt/le/gt/ge/bits)"), cursor.peek().span)),
    };
    let value = parse_u64_literal(cursor)?;
    if matches!(op,CondOp::BitsSet)&&value==0{return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"bit condition requires a nonzero mask",cursor.peek().span));}
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

/// 🏷️ P2-M2 item 1: an arm/`until` tag literal — a `TEXT` literal encodes to its raw ASCII
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

/// ✂️ Trims leading zero bytes off a `u64`'s big-endian representation (keeping at least one
/// byte) — used for `backward <name> magic 0x...` directives, matching `framing magic`'s existing
/// literal-to-bytes convention but without forcing a fixed 8-byte width.
fn trim_be_bytes(value: u64) -> Vec<u8> {
    let be = value.to_be_bytes();
    let first_nonzero = be.iter().position(|&b| b != 0).unwrap_or(7);
    be[first_nonzero..].to_vec()
}

/// 🌿️ Parses one `arm <tag> { field... | nested <name> <prim> { arm ... } }` body.
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

/// 🔁️ P2-M2 item 1: `repeat <name> { tag <prim> length <prim>? order length-first?
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
                    other => return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown `repeat` order `{other}` (expected `tag-first`/`length-first`)"), head.span)),
                };
            }
            "trailer" => trailer = Some(parse_prim(cursor)?),
            "until" => {
                let disc = discriminator.as_ref().ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "`until` must follow `tag` in a `repeat` block", head.span))?;
                until = Some(parse_tag_value(cursor, disc)?);
            }
            "arm" => {
                let disc = discriminator.as_ref().ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "`arm` must follow `tag` in a `repeat` block", head.span))?;
                let tag = parse_tag_value(cursor, disc)?;
                let (fields, nested) = parse_arm_body(cursor)?;
                arms.push(RepeatArm { tag, fields, nested });
            }
            other => return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown `repeat` directive `{other}`"), head.span)),
        }
        cursor.skip_newlines();
    }
    cursor.expect(GKind::RBrace)?;
    let discriminator = match discriminator {
        Some(d) => d,
        None => return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "`repeat` block is missing a `tag` directive", cursor.peek().span)),
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

/// 📡️ Parses one `.protocol.semio` file into a typed [`ProtocolFile`] — retains every body
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
                    "literal" => Framing::Literal,
                    other => return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown framing `{other}`"), head.span)),
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
            other => return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown protocol directive `{other}`"), head.span)),
        }
    }

    close_header(&mut blocks, &mut open_header);
    flush_open_segment(&mut blocks, &mut open_segment);
    close_record(&mut blocks, &mut open_record);

    let start = match start {
        Some(s) => s,
        None => return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "`.protocol` file is missing a `start` directive", cursor.peek().span)),
    };
    let framing = match framing {
        Some(f) => f,
        None => return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "`.protocol` file is missing a `framing` directive", cursor.peek().span)),
    };
    if schema.is_empty() {
        return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "`.protocol` file is missing a `schema` directive", cursor.peek().span));
    }
    Ok(ProtocolFile { id, version, schema, start, uses, framing, blocks })
}
//#endregion 🔖️Parser
