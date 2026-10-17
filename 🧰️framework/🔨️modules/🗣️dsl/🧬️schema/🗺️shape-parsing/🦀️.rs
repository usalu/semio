//! 🗺️ Shape parsing of the schema parser: the embed, field shape, quantity, number, dimension and shape parsers. See [`super`].

use super::*;

//#region 🔖️Parser
/// 🗣️ `Shape::Embed`'s parse: a `Fence` token (Document mode — see `dsl_core`'s lexer for
/// the `lang\u{0}content` encoding) with an empty or matching lang tag, OR anything
/// `parse_scalar_text` already accepts (Inline mode's escaped-quoted fallback) — both converge on
/// the same `FieldValue::Text`, which is what makes Document/Inline renders agree.
pub(super) fn parse_embed(cursor: &mut Cursor, declared_lang: &str) -> Result<FieldValue, TextError> {
    if cursor.peek().kind == TokenKind::Fence {
        let token = cursor.advance();
        let raw = token.text.as_str();
        let (lang, content) = raw.split_once('\u{0}').ok_or_else(|| TextError::new(ValueRefusalKind::InvalidValue,"malformed fence token (missing separator)",token.span))?;
        if !lang.is_empty() && !declared_lang.is_empty() && lang != declared_lang {
            return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("fence declares lang '{lang}', field expects '{declared_lang}'"),token.span));
        }
        return Ok(FieldValue::Text(content.to_string()));
    }
    parse_scalar_text(cursor)
}

pub(super) fn sibling_text_field<'a>(record: &'a RecordValue, spec: &RecordSpec, lang_key: &str) -> Option<&'a str> {
    let field = spec.fields.iter().find(|f| f.key == lang_key)?;
    match record.get(field.id)? {
        FieldValue::Text(text) => Some(text.as_str()),
        _ => None,
    }
}

pub(super) fn parse_field_shape(cursor: &mut Cursor, field: &FieldSpec, spec: &RecordSpec, record: &RecordValue, depth: usize) -> Result<FieldValue, TextError> {
    if let Shape::EmbedFrom(lang_key) = &field.shape {
        let declared = sibling_text_field(record, spec, lang_key).unwrap_or("");
        return parse_embed(cursor, declared);
    }
    parse_shape(cursor, &field.shape, depth)
}

/// 📐️ Shared parse for `Shape::Quantity`/`Shape::Angle`: a number, optionally followed by a
/// GLUED (no whitespace between — the lexer already ends a numeric token exactly where the next
/// `Ident` token begins for input like `210GPa`) unit-symbol ident. No suffix means the number is
/// already expressed in `declared`'s unit; a suffix converts, erroring if the dimensions differ.
pub(super) fn parse_quantity(cursor: &mut Cursor, declared: &'static semio_framework_dsl::UnitSpec) -> Result<FieldValue, TextError> {
    let is_number_token = matches!(cursor.peek().kind, TokenKind::Float | TokenKind::Int) || (cursor.peek().kind == TokenKind::Ident && matches!(cursor.peek().text.as_str().as_ref(), "nan" | "inf" | "-inf"));
    if !is_number_token {
        return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("expected a quantity, found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
    }
    let number_token = cursor.advance();
    let value = parse_f64(&number_token.text.as_str()).map_err(|e| TextError::new(ValueRefusalKind::InvalidValue,e,number_token.span))?;
    let suffix = cursor.peek();
    if suffix.kind == TokenKind::Ident && suffix.byte_range.0 == number_token.byte_range.1 {
        let suffix_token = cursor.advance();
        let symbol = suffix_token.text.as_str().to_string();
        let suffix_unit = semio_framework_dsl::unit_by_symbol(&symbol).ok_or_else(|| TextError::new(ValueRefusalKind::InvalidValue,format!("unknown unit '{symbol}'"),suffix_token.span))?;
        let converted = if value.is_nan(){if suffix_unit.dimension!=declared.dimension{return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("unit '{symbol}' is not compatible with expected unit '{}'",declared.symbol),suffix_token.span));}value}else{semio_framework_dsl::convert(value,suffix_unit,declared).ok_or_else(||TextError::new(ValueRefusalKind::InvalidValue,format!("unit '{symbol}' is not compatible with expected unit '{}'",declared.symbol),suffix_token.span))?};
        Ok(FieldValue::Float(converted))
    } else {
        Ok(FieldValue::Float(value))
    }
}

/// 🔢️ Reads a complete numeric token for coordinate, direction, dimension and range components.
fn parse_plain_number(cursor: &mut Cursor) -> Result<f64, TextError> {
    if !(matches!(cursor.peek().kind,TokenKind::Float|TokenKind::Int)||(cursor.peek().kind==TokenKind::Ident&&matches!(cursor.peek().text.as_str().as_ref(),"nan"|"inf"|"-inf"))) {
        return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("expected a number, found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
    }
    let token = cursor.advance();
    parse_f64(&token.text.as_str()).map_err(|e| TextError::new(ValueRefusalKind::InvalidValue,e,token.span))
}

/// 📍️ Shared body for `Shape::Coord`/`Shape::Dir`: a fixed-arity comma-separated run of
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
        return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("{what} literal expects exactly {arity} components"),cursor.span()));
    }
    Ok(FieldValue::Tuple(items))
}

/// 📏️ `Shape::Dim`'s `WxHxD` grammar: the FIRST number is an ordinary `Float|Int` token;
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
            return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("dimension literal expects {dims} components glued with 'x' (e.g. '2x3'), found only one"),cursor.span()));
        }
        let suffix_token = cursor.advance();
        let suffix_text = suffix_token.text.as_str();
        let parts: Vec<&str> = suffix_text.split('x').collect();
        // `"x0.12x0.24".split('x')` yields `["", "0.12", "0.24"]` — the leading empty piece is the
        // text before the first `x`, which is always empty since the suffix itself starts with it.
        if parts.first() != Some(&"") || parts.len() != dims {
            return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("dimension literal expects {dims} components glued with 'x', found '{}{}'", format_f64(first), suffix_text),suffix_token.span));
        }
        for part in &parts[1..] {
            let value = parse_f64(part).map_err(|_| TextError::new(ValueRefusalKind::InvalidValue,format!("invalid dimension component '{part}'"),suffix_token.span))?;
            items.push(FieldValue::Float(value));
        }
    }
    Ok(FieldValue::Tuple(items))
}

pub(super) fn parse_shape(cursor: &mut Cursor, shape: &Shape, depth: usize) -> Result<FieldValue, TextError> {
    cursor.limits.check_depth(depth, cursor.span())?;
    match shape {
        Shape::Bool | Shape::Int | Shape::UInt | Shape::Float | Shape::Text | Shape::Bytes64 | Shape::Enum(_) | Shape::Quantity(_) | Shape::Angle(_) | Shape::Ref(_) | Shape::Count | Shape::Embed(_) => parse_scalar(cursor, shape),
        Shape::EmbedFrom(_) => Err(TextError::new(ValueRefusalKind::UnsupportedOwner,"EmbedFrom field must be parsed in record context",cursor.span())),
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
                    return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("tuple expects {} elements, found {}", expected_len, items.len()),cursor.span()));
                }
            }
            Ok(FieldValue::Tuple(items))
        }
        Shape::List(elem) => {
            cursor.expect(TokenKind::LBracket)?;
            let mut items = Vec::new();
            while cursor.peek().kind != TokenKind::RBracket {
                let pos_before = cursor.pos;
                let value = if let Shape::Record(make) = elem.as_ref() {
                    cursor.expect(TokenKind::LBrace)?;
                    let record = parse_record_body(cursor, &(make.ordinary)(), depth + 1)?;
                    cursor.expect(TokenKind::RBrace)?;
                    FieldValue::Record(record)
                } else {
                    parse_shape(cursor, elem, depth + 1)?
                };
                items.push(value);
                if cursor.pos == pos_before {
                    return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("list element made no progress at {:?} '{}' — likely an unrecognized field key", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
                }
                cursor.limits.check_nodes(items.len(), cursor.span())?;
            }
            cursor.expect(TokenKind::RBracket)?;
            Ok(FieldValue::List(items))
        }
        Shape::Record(spec_fn) => Ok(FieldValue::Record(parse_record_body(cursor, &(spec_fn.ordinary)(), depth + 1)?)),
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
                let record = parse_record_body(cursor, &(spec_fn.ordinary)(), depth + 1)?;
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
            while let Some(key) = cursor.dynamic_attr_key()? {
                cursor.advance();
                cursor.expect(TokenKind::Equals)?;
                let record = matches!(inner.as_ref(), Shape::Record(_));
                if record { cursor.expect(TokenKind::LBrace)?; }
                let value = parse_shape(cursor, inner, depth + 1)?;
                if record { cursor.expect(TokenKind::RBrace)?; }
                entries.push((key, value));
            }
            cursor.expect(TokenKind::RBrace)?;
            Ok(FieldValue::Map(entries))
        }
        Shape::Value => Ok(FieldValue::Value(parse_dsl_value(cursor, depth + 1)?)),
        Shape::Table(spec_fn) => {
            validate_table_columns(&(spec_fn.ordinary)())?;
            parse_table_list(cursor, *spec_fn, depth)
        }
        Shape::Wire => Ok(FieldValue::Wire(parse_wire(cursor)?)),
    }
}
//#endregion 🔖️Parser
