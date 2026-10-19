//! 🧭️ Cursor and expressions of the schema parser: the source mode, the token cursor, the parse options, the entry points and the scalar and expression parsers. See [`super`].

use super::*;

//#region 🔖️Cursor
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceMode {
    Document,
    Inline,
}

pub(super) struct Cursor {
    pub(super) tokens: Vec<SpannedToken>,
    pub(super) pos: usize,
    pub(super) limits: Limits,
}

impl Cursor {
    pub(super) fn new(tokens: Vec<SpannedToken>, limits: Limits) -> Self {
        let tokens: Vec<SpannedToken> = tokens.into_iter().filter(|t| !t.kind.is_trivia()).collect();
        Self { tokens, pos: 0, limits }
    }

    pub(super) fn peek(&self) -> &SpannedToken {
        &self.tokens[self.pos.min(self.tokens.len() - 1)]
    }

    pub(super) fn peek_at(&self, offset: usize) -> &SpannedToken {
        let idx = (self.pos + offset).min(self.tokens.len() - 1);
        &self.tokens[idx]
    }

    pub(super) fn span(&self) -> TextSpan {
        self.peek().span
    }

    pub(super) fn advance(&mut self) -> SpannedToken {
        let token = self.tokens[self.pos.min(self.tokens.len() - 1)].clone();
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        token
    }

    pub(super) fn expect(&mut self, kind: TokenKind) -> Result<SpannedToken, TextError> {
        if self.peek().kind == kind {
            Ok(self.advance())
        } else {
            Err(TextError::new(ValueRefusalKind::InvalidValue,format!("expected {:?}, found {:?} '{}'", kind, self.peek().kind, self.peek().text.as_str()),self.span()))
        }
    }

    /// 🔎️ Whether the next token is an `Ident` that is followed by `=` — the LL(2)
    /// lookahead that makes the grammar newline-insensitive: a bare ident followed by `=` is
    /// always a `key=value` attribute, never the start of a new statement.
    pub(super) fn at_attr_key(&self) -> Option<String> {
        if self.peek().kind == TokenKind::Ident && self.peek_at(1).kind == TokenKind::Equals {
            Some(self.peek().text.as_str().to_string())
        } else {
            None
        }
    }

    pub(super) fn at_keyword(&self, keyword: &str) -> bool {
        self.peek().kind == TokenKind::Ident && self.peek().text.as_str().as_ref() == keyword
    }

    pub(super) fn dynamic_attr_key(&self) -> Result<Option<String>, TextError> {
        if self.peek().kind == TokenKind::Text && self.peek_at(1).kind == TokenKind::Equals {
            semio_framework_dsl::unescape_text(&self.peek().text.as_str(), false).map(Some).map_err(|message| TextError::new(ValueRefusalKind::InvalidValue,message,self.span()))
        } else {
            Ok(self.at_attr_key())
        }
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

/// ✂️ The structural seam between lexing and parsing: everything downstream of a token
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

/// 🛑️ Parses one terminal record and rejects every token outside its schema-owned body.
pub fn parse_exact(text: &str, spec: &RecordSpec, opts: &ParseOptions) -> Result<Cst, TextError> {
    let mut cursor = Cursor::new(lex(text, &opts.limits, false)?, opts.limits);
    let record = parse_record_body(&mut cursor, spec, 0)?;
    cursor.expect(TokenKind::Eof)?;
    Ok(record)
}

pub(super) fn ident_like_text(token: &SpannedToken) -> String {
    token.text.as_str().to_string()
}

pub(super) fn parse_scalar(cursor: &mut Cursor, shape: &Shape) -> Result<FieldValue, TextError> {
    match shape {
        Shape::Bool => {
            let token = cursor.expect(TokenKind::Ident)?;
            match token.text.as_str().as_ref() {
                "true" => Ok(FieldValue::Bool(true)),
                "false" => Ok(FieldValue::Bool(false)),
                other => Err(TextError::new(ValueRefusalKind::InvalidValue,format!("expected 'true' or 'false', found '{other}'"),token.span)),
            }
        }
        Shape::Int => {
            let token = cursor.expect(TokenKind::Int)?;
            let value: i64 = token.text.as_str().parse().map_err(|_| TextError::new(ValueRefusalKind::InvalidValue,format!("invalid integer '{}'", token.text.as_str()),token.span))?;
            Ok(FieldValue::Int(value))
        }
        Shape::UInt => {
            let token = cursor.expect(TokenKind::Int)?;
            let value: u64 = token.text.as_str().parse().map_err(|_| TextError::new(ValueRefusalKind::InvalidValue,format!("invalid unsigned integer '{}'", token.text.as_str()),token.span))?;
            Ok(FieldValue::UInt(value))
        }
        Shape::Float => {
            let is_float_token = matches!(cursor.peek().kind, TokenKind::Float | TokenKind::Int) || (cursor.peek().kind == TokenKind::Ident && matches!(cursor.peek().text.as_str().as_ref(), "nan" | "inf" | "-inf"));
            if !is_float_token {
                return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("expected a float, found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
            }
            let token = cursor.advance();
            let value = parse_f64(&token.text.as_str()).map_err(|e| TextError::new(ValueRefusalKind::InvalidValue,e,token.span))?;
            Ok(FieldValue::Float(value))
        }
        Shape::Text => parse_scalar_text(cursor),
        Shape::Bytes64 => {
            let token = cursor.expect(TokenKind::Text)?;
            let bytes = base64_decode(&token.text.as_str()).map_err(|e| TextError::new(ValueRefusalKind::InvalidValue,e,token.span))?;
            Ok(FieldValue::Bytes64(bytes))
        }
        Shape::Enum(variants) => {
            let token = cursor.expect(TokenKind::Ident)?;
            let text = token.text.as_str();
            variants.iter().find(|(tag, _)| tag == text.as_ref()).map(|(_, ordinal)| FieldValue::Enum(*ordinal)).ok_or_else(|| TextError::new(ValueRefusalKind::InvalidValue,format!("unknown enum tag '{text}'"),token.span))
        }
        Shape::Quantity(declared) | Shape::Angle(declared) => parse_quantity(cursor, declared),
        Shape::Ref(_) => parse_scalar_text(cursor),
        Shape::Embed(declared_lang) => parse_embed(cursor, declared_lang),
        Shape::EmbedFrom(_) => Err(TextError::new(ValueRefusalKind::UnsupportedOwner,"EmbedFrom field must be parsed in record context",cursor.span())),
        Shape::Count => {
            if cursor.peek().kind != TokenKind::Ident {
                return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("expected a count literal like 'x24', found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
            }
            let token = cursor.advance();
            let text = token.text.as_str();
            let digits = text.strip_prefix('x').ok_or_else(|| TextError::new(ValueRefusalKind::InvalidValue,format!("expected a count literal like 'x24', found '{text}'"),token.span))?;
            let value: u64 = digits.parse().map_err(|_| TextError::new(ValueRefusalKind::InvalidValue,format!("invalid count literal 'x{digits}'"),token.span))?;
            Ok(FieldValue::UInt(value))
        }
        other => Err(TextError::new(ValueRefusalKind::UnsupportedOwner,format!("shape {other:?} is not a scalar"),cursor.span())),
    }
}

/// 🧮️ Precedence-climbing entry point for `Shape::Expr`'s body (called with the caller's
/// outer `(`/`)` already consumed). `min_prec` is the lowest operator precedence this call is
/// willing to keep consuming at — the standard technique for turning a flat token stream into a
/// precedence-correct tree without a separate tokenize-then-shunting-yard pass.
pub(super) fn parse_expr(cursor: &mut Cursor, min_prec: u8) -> Result<ExprValue, TextError> {
    let lhs = parse_expr_unary(cursor)?;
    parse_expr_continue(cursor, min_prec, lhs)
}

/// 🧮️ The loop body of `parse_expr`, factored out so the glued-negative-number case below
/// can re-enter it with an ALREADY-PARSED left operand instead of calling `parse_expr_unary` again
/// (which would re-consume nothing, since the token was already consumed to build that operand).
fn parse_expr_continue(cursor: &mut Cursor, min_prec: u8, mut lhs: ExprValue) -> Result<ExprValue, TextError> {
    loop {
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
            let value = parse_f64(&token.text.as_str()).map_err(|e| TextError::new(ValueRefusalKind::InvalidValue,e,token.span))?;
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
            let value = parse_f64(&token.text.as_str()).map_err(|e| TextError::new(ValueRefusalKind::InvalidValue,e,token.span))?;
            Ok(ExprValue::Num(value))
        }
        TokenKind::Ident | TokenKind::Text => {
            let token = cursor.advance();
            let name = if token.kind==TokenKind::Text{semio_framework_dsl::unescape_text(&token.text.as_str(),false).map_err(|error|TextError::new(ValueRefusalKind::InvalidValue,error,token.span))?}else{ident_like_text(&token)};
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
        other => Err(TextError::new(ValueRefusalKind::InvalidValue,format!("expected a number, variable, or '(', found {other:?} '{}'", cursor.peek().text.as_str()),cursor.span())),
    }
}

/// 🧮️ Standalone entry point for parsing a bare expression body (no surrounding `(`/`)`,
/// unlike `Shape::Expr`'s own field-value grammar) — what `pack_value`'s decoder calls to turn the
/// canonical string it stored back into an `ExprValue`, since decode has no `Cursor` of its own.
pub fn parse_expr_text(text: &str) -> Result<ExprValue, TextError> {
    let tokens = lex(text, &Limits::default(), false)?;
    let mut cursor = Cursor::new(tokens, Limits::default());
    let value = parse_expr(&mut cursor, 0)?;
    cursor.expect(TokenKind::Eof)?;
    Ok(value)
}

pub(super) fn parse_scalar_text(cursor: &mut Cursor) -> Result<FieldValue, TextError> {
    match cursor.peek().kind {
        TokenKind::Text => {
            let token = cursor.advance();
            let text = semio_framework_dsl::unescape_text(&token.text.as_str(), false).map_err(|e| TextError::new(ValueRefusalKind::InvalidValue,e,token.span))?;
            Ok(FieldValue::Text(text))
        }
        TokenKind::Ident => {
            let token = cursor.advance();
            Ok(FieldValue::Text(ident_like_text(&token)))
        }
        other => Err(TextError::new(ValueRefusalKind::InvalidValue,format!("expected Text, found {other:?} '{}'", cursor.peek().text.as_str()),cursor.span())),
    }
}
//#endregion 🔖️Parser
