//! 🔤️ XML lexemes shared by the document model and the text codec: whitespace, names, quoted values, entity references and the document type literal.

use super::{XmlDoctype, XmlDtdDeclaration, XmlExternalId, XmlQuote};

/// 🔓️ Decode the five predefined XML entities plus numeric character references (`&#NNN;`,
/// `&#xHHHH;`/`&#XHHHH;`) into their literal characters. This is the read-side half that was
/// entirely missing before: without it, `&amp;` in a source document is kept as the 5 literal
/// characters `&`,`a`,`m`,`p`,`;` in the `Text` node, and the NEXT write-side escape turns that
/// lone `&` into `&amp;` again -- so every decode->encode cycle grows `&amp;` into `&amp;amp;`
/// into `&amp;amp;amp;`, permanently corrupting the document. An unrecognized/malformed entity is
/// a hard parse error (never silently dropped or passed through raw).
pub fn xml_unescape_text(s: &str) -> Result<String, String> {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.char_indices().peekable();
    while let Some((i, ch)) = chars.next() {
        if ch != '&' {
            out.push(ch);
            continue;
        }
        let rest = &s[i..];
        let end = rest.find(';').ok_or_else(|| format!("unterminated entity reference at byte {i}"))?;
        let entity = &rest[1..end];
        let decoded = if let Some(numeric) = entity.strip_prefix('#') {
            let code = if let Some(hex) = numeric.strip_prefix('x').or_else(|| numeric.strip_prefix('X')) {
                u32::from_str_radix(hex, 16).map_err(|_| format!("invalid hex character reference &{entity};"))?
            } else {
                numeric.parse::<u32>().map_err(|_| format!("invalid decimal character reference &{entity};"))?
            };
            char::from_u32(code).ok_or_else(|| format!("invalid unicode scalar &{entity};"))?
        } else {
            match entity {
                "amp" => '&',
                "lt" => '<',
                "gt" => '>',
                "quot" => '"',
                "apos" => '\'',
                other => return Err(format!("unknown entity &{other};")),
            }
        };
        out.push(decoded);
        let consume_to = i + end + 1;
        while let Some(&(j, _)) = chars.peek() {
            if j < consume_to {
                chars.next();
            } else {
                break;
            }
        }
    }
    Ok(out)
}

pub fn skip_ws(s: &str, pos: &mut usize) {
    while *pos < s.len() && s.as_bytes()[*pos].is_ascii_whitespace() {
        *pos += 1;
    }
}

pub fn parse_name(s: &str, pos: &mut usize) -> Result<String, String> {
    let start = *pos;
    if *pos >= s.len() || !is_name_start(s[*pos..].chars().next().unwrap()) {
        return Err("expected XML name".into());
    }
    *pos += 1;
    while *pos < s.len() {
        let ch = s[*pos..].chars().next().unwrap();
        if is_name_char(ch) {
            *pos += ch.len_utf8();
        } else {
            break;
        }
    }
    Ok(s[start..*pos].to_string())
}

pub fn parse_attr_value(s: &str, pos: &mut usize) -> Result<String, String> {
    parse_attr_value_quoted(s, pos).map(|(value, _)| value)
}

/// 🗣️ `parse_attr_value` plus the delimiter it consumed — the XML declaration is the one place
/// whose quoting is modeled state ([`XmlQuote`]), so only that caller needs the second half.
pub fn parse_attr_value_quoted(s: &str, pos: &mut usize) -> Result<(String, XmlQuote), String> {
    skip_ws(s, pos);
    let quote = s[*pos..].chars().next().ok_or("expected attribute value")?;
    if quote != '"' && quote != '\'' {
        return Err("attribute value must be quoted".into());
    }
    *pos += 1;
    let start = *pos;
    while *pos < s.len() {
        let ch = s[*pos..].chars().next().unwrap();
        if ch == quote {
            let value = s[start..*pos].to_string();
            *pos += 1;
            return Ok((value, XmlQuote::from_char(quote)));
        }
        *pos += ch.len_utf8();
    }
    Err("unclosed attribute value".into())
}

pub fn is_name_start(ch: char) -> bool {
    ch == ':' || ch.is_ascii_alphabetic() || ch == '_'
}

pub fn is_name_char(ch: char) -> bool {
    is_name_start(ch) || ch.is_ascii_digit() || ch == '-' || ch == '.'
}

pub fn parse_doctype(text: &str) -> Result<XmlDoctype, String> {
    let mut pos = "<!DOCTYPE".len();
    skip_ws(text, &mut pos);
    let name = parse_name(text, &mut pos)?;
    skip_ws(text, &mut pos);
    let external_id = if text[pos..].starts_with("SYSTEM") {
        pos += "SYSTEM".len();
        Some(XmlExternalId::System { system_id: xml_unescape_text(&parse_attr_value(text, &mut pos)?)? })
    } else if text[pos..].starts_with("PUBLIC") {
        pos += "PUBLIC".len();
        let public_id = xml_unescape_text(&parse_attr_value(text, &mut pos)?)?;
        let system_id = xml_unescape_text(&parse_attr_value(text, &mut pos)?)?;
        Some(XmlExternalId::Public { public_id, system_id })
    } else {
        None
    };
    skip_ws(text, &mut pos);
    let mut declarations = Vec::new();
    if text[pos..].starts_with('[') {
        pos += 1;
        loop {
            skip_ws(text, &mut pos);
            if text[pos..].starts_with(']') {
                pos += 1;
                break;
            }
            if !text[pos..].starts_with("<!ENTITY") {
                return Err("unsupported XML DTD declaration; only typed ENTITY declarations are modeled".into());
            }
            pos += "<!ENTITY".len();
            skip_ws(text, &mut pos);
            let parameter = text[pos..].starts_with('%');
            if parameter {
                pos += 1;
                skip_ws(text, &mut pos);
            }
            let entity_name = parse_name(text, &mut pos)?;
            let value = xml_unescape_text(&parse_attr_value(text, &mut pos)?)?;
            skip_ws(text, &mut pos);
            if !text[pos..].starts_with('>') {
                return Err("expected > after XML entity declaration".into());
            }
            pos += 1;
            declarations.push(XmlDtdDeclaration::Entity { parameter, name: entity_name, value });
        }
    }
    skip_ws(text, &mut pos);
    if !text[pos..].starts_with('>') {
        return Err("expected > after XML document type declaration".into());
    }
    pos += 1;
    if pos != text.len() {
        return Err("trailing content in XML document type declaration".into());
    }
    Ok(XmlDoctype { prolog_position: 0, name, external_id, declarations })
}

impl From<&str> for XmlDoctype {
    fn from(value: &str) -> Self {
        parse_doctype(value).expect("valid XML document type literal")
    }
}
