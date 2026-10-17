//! 📜️ DSL value and wire parsing of the schema parser: the keyword probe, the DSL value parser and the wire parsers. See [`super`].

use super::*;

//#region 🔖️Parser
pub(super) fn current_keyword(cursor: &Cursor) -> Option<String> {
    if cursor.peek().kind == TokenKind::Ident && cursor.at_attr_key().is_none() {
        Some(cursor.peek().text.as_str().to_string())
    } else {
        None
    }
}

pub(super) fn parse_dsl_value(cursor: &mut Cursor, depth: usize) -> Result<DslValue, TextError> {
    cursor.limits.check_depth(depth, cursor.span())?;
    match cursor.peek().kind {
        TokenKind::LBrace => {
            cursor.advance();
            let mut entries = Vec::new();
            while let Some(key) = cursor.dynamic_attr_key()? {
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
            let text = semio_framework_dsl::unescape_text(&token.text.as_str(), false).map_err(|e| TextError::new(ValueRefusalKind::InvalidValue,e,token.span))?;
            Ok(DslValue::String(text))
        }
        TokenKind::Int => {
            let token = cursor.advance();
            let text = token.text.as_str();
            if let Ok(v) = text.parse::<u64>() {
                Ok(DslValue::Number(Number::UInt(v)))
            } else if let Ok(v) = text.parse::<i64>() {
                Ok(DslValue::Number(Number::Int(v)))
            } else {
                let value = parse_f64(&text).map_err(|e| TextError::new(ValueRefusalKind::InvalidValue,e,token.span))?;
                Ok(DslValue::Number(Number::Float(value)))
            }
        }
        TokenKind::Float => {
            let token = cursor.advance();
            let value = parse_f64(&token.text.as_str()).map_err(|e| TextError::new(ValueRefusalKind::InvalidValue,e,token.span))?;
            Ok(DslValue::Number(Number::Float(value)))
        }
        TokenKind::Ident => {
            let token = cursor.advance();
            match token.text.as_str().as_ref() {
                "int64"=>{cursor.expect(TokenKind::LParen)?;let token=cursor.expect(TokenKind::Int)?;let value=token.text.as_str().parse::<i64>().map_err(|error|TextError::new(ValueRefusalKind::InvalidValue,error.to_string(),token.span))?;cursor.expect(TokenKind::RParen)?;Ok(DslValue::Number(Number::Int(value)))},
                "bytes64"=>{cursor.expect(TokenKind::LParen)?;let token=cursor.expect(TokenKind::Text)?;let bytes=base64_decode(&token.text.as_str()).map_err(|error|TextError::new(ValueRefusalKind::InvalidValue,error,token.span))?;cursor.expect(TokenKind::RParen)?;Ok(DslValue::Bytes(bytes))},
                "null" => Ok(DslValue::Null),
                "true" => Ok(DslValue::Bool(true)),
                "false" => Ok(DslValue::Bool(false)),
                "nan"|"inf"=>Ok(DslValue::Number(Number::Float(parse_f64(&token.text.as_str()).map_err(|error|TextError::new(ValueRefusalKind::InvalidValue,error,token.span))?))),
                other => Err(TextError::new(ValueRefusalKind::InvalidValue,format!("expected a value literal, found ident '{other}'"),token.span)),
            }
        }
        other => Err(TextError::new(ValueRefusalKind::InvalidValue,format!("expected a value literal, found {other:?}"),cursor.span())),
    }
}

/// 🕸️ Parses one wire literal. `<-` is accepted sugar only: normalized here by swapping
/// the two endpoints, so the stored `WireValue` (and everything reprinted from it) only ever
/// holds `->`/`--` or fused labeled arrows — `b<-a` and `a->b` parse to the identical value.
pub(super) fn parse_wire(cursor: &mut Cursor) -> Result<WireValue, TextError> {
    fn parse_wire_label(cursor: &mut Cursor) -> Result<WireEdgeLabel, TextError> {
        cursor.expect(TokenKind::LBracket)?;
        let id = if cursor.peek().kind == TokenKind::Ident { Some(ident_like_text(&cursor.advance())) } else { None };
        let kind = if cursor.peek().kind == TokenKind::Colon {
            cursor.advance();
            Some(ident_like_text(&cursor.expect(TokenKind::Ident)?))
        } else {
            None
        };
        let label = WireEdgeLabel { id, kind };
        if label.is_empty() {
            return Err(TextError::new(ValueRefusalKind::InvalidValue,"edge label `[...]` must name an id and/or a `:kind`",cursor.span()));
        }
        cursor.expect(TokenKind::RBracket)?;
        Ok(label)
    }

    let mut from = parse_wire_node(cursor)?;
    let mut edge_label = WireEdgeLabel::default();
    let edge = match cursor.peek().kind {
        TokenKind::Arrow => {
            cursor.advance();
            let to = parse_wire_node(cursor)?;
            Some((true, to))
        }
        TokenKind::DashArrow => {
            cursor.advance();
            let to = parse_wire_node(cursor)?;
            Some((false, to))
        }
        TokenKind::BackArrow => {
            cursor.advance();
            if cursor.peek().kind == TokenKind::LBracket {
                edge_label = parse_wire_label(cursor)?;
                cursor.expect(TokenKind::Minus)?;
            }
            let to = parse_wire_node(cursor)?;
            let swapped_to = std::mem::replace(&mut from, to);
            Some((true, swapped_to))
        }
        TokenKind::Minus if cursor.peek_at(1).kind == TokenKind::LBracket => {
            cursor.advance();
            edge_label = parse_wire_label(cursor)?;
            let directed = match cursor.peek().kind {
                TokenKind::Arrow => {
                    cursor.advance();
                    true
                }
                TokenKind::DashArrow => {
                    cursor.advance();
                    false
                }
                other => {
                    return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("expected `->` or `--` to close a labeled edge, found {other:?}"),cursor.span()));
                }
            };
            let to = parse_wire_node(cursor)?;
            Some((directed, to))
        }
        TokenKind::EdgeArrow => {
            let token = cursor.advance();
            let (directed, label) = dsl_notation::decode_fused_edge_arrow(&token.text.as_str())?;
            edge_label = WireEdgeLabel { id: label.id, kind: label.kind };
            let to = parse_wire_node(cursor)?;
            Some((directed, to))
        }
        _ => None,
    };
    let properties = if cursor.peek().kind == TokenKind::LBrace { parse_dsl_value(cursor, 0)? } else { DslValue::Object(Vec::new()) };
    Ok(WireValue { from, edge, edge_label, properties })
}

/// 🔌️ Small public entry point other crates (the graph wire module, trinity) can call
/// directly to lex + parse one standalone wire literal, without needing a `RecordSpec` around it.
pub fn parse_wire_text(text: &str) -> Result<WireValue, TextError> {
    let limits = Limits::default();
    let tokens = lex(text, &limits, false)?;
    let mut cursor = Cursor::new(tokens, limits);
    parse_wire(&mut cursor)
}

fn parse_wire_node(cursor: &mut Cursor) -> Result<WireNode, TextError> {
    let FieldValue::Text(id) = parse_scalar_text(cursor)? else { unreachable!() };
    let kind = if cursor.peek().kind == TokenKind::Colon {
        cursor.advance();
        Some(match parse_scalar_text(cursor)? { FieldValue::Text(value) => value, _ => unreachable!() })
    } else {
        None
    };
    let port = if cursor.peek().kind == TokenKind::At {
        cursor.advance();
        Some(match parse_scalar_text(cursor)? { FieldValue::Text(value) => value, _ => unreachable!() })
    } else {
        None
    };
    Ok(WireNode { id, kind, port })
}
//#endregion 🔖️Parser
