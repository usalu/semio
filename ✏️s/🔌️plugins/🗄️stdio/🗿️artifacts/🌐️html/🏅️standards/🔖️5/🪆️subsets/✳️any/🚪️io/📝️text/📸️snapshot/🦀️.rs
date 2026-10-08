//! 📝️ Text representation codec surface for `stdio.html` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type HtmlSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v5::subsets::any::schema::snapshot::*;
use semio_framework_diagnostic::TextSpan;
use framework_schema::ArtifactSchema;
use semio_framework_diagnostic::TextError;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn is_void_element(name: &str) -> bool {
    VOID_ELEMENTS.iter().any(|v| v.eq_ignore_ascii_case(name))
}

/// 🔒️ Escapes text-node content for re-serialization (`&`, `<`, `>`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            c => out.push(c),
        }
    }
    out
}

/// 🔒️ Escapes an attribute value for re-serialization. Attribute values are ALWAYS re-emitted
/// double-quoted (see module doc comment on quote-style normalization), so only `&` and `"` need
/// escaping.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_attr_value(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            c => out.push(c),
        }
    }
    out
}

/// 🧽 Whitespace-only character data, per the WHATWG definition (TAB, LF, FF, CR, SPACE).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn is_whitespace_text(node: &HtmlNode) -> bool {
    matches!(node, HtmlNode::Text { text } if text.chars().all(|c| matches!(c, '\t' | '\n' | '\u{000C}' | '\r' | ' ')))
}

/// 🧹 The three places where a well-formed HTML5 document's SOURCE whitespace is not where the
/// document's TREE carries it. All three are normative WHATWG tree construction, not error recovery
/// (which stays out of scope per the module header): §13.2.6.4.3 "before head" *ignores*
/// whitespace-only character tokens, so `<html>\n  <head>` has no text node before `<head>` in any
/// conformant DOM; §13.2.6.4.20 "after body" and §13.2.6.4.22 "after after body" both process them
/// with the "in body" rules, whose insertion point is still the `body` element (`</body>` and
/// `</html>` switch the insertion mode but never pop the stack), so the newlines in
/// `</body>\n</html>\n` both belong to `body`, merged onto its last text node by the tree
/// construction's own "append to existing text node" rule — not to `html`, and not discarded.
/// `after_root` carries the document-level tail the caller consumed after `</html>`.
///
/// Reading them literally is what this parser used to do, and it put every path index inside `<html>`
/// one place off from every other HTML5 implementation's: `[2]` addressed a whitespace text node here
/// where `html5ever`, every browser, and this subset's own mutation oracle all address `<body>`.
/// Found by `../../../../../🧪️tests/🌐️mutate-html-5`'s parity phase the first time it ran
/// (ticket 26/08/23/END-TO-END-TESTING-REFACTOR), where all seven path-addressed kinds were refused
/// with `mutation.apply.conflicting-target` — "element diff targets a non-element node".
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn normalize_html_root_whitespace(root: &mut HtmlNode, after_root: &str) {
    let HtmlNode::Element { name, children, .. } = root else { return };
    if !name.eq_ignore_ascii_case("html") {
        return;
    }
    let Some(first_element) = children.iter().position(|child| matches!(child, HtmlNode::Element { .. })) else { return };
    let mut seen = 0;
    children.retain(|child| {
        let keep = seen >= first_element || !is_whitespace_text(child);
        seen += 1;
        keep
    });
    let Some(body) = children.iter().position(|child| matches!(child, HtmlNode::Element { name, .. } if name.eq_ignore_ascii_case("body"))) else { return };
    let mut trailing = String::new();
    let mut seen = 0;
    children.retain(|child| {
        let keep = seen <= body || !is_whitespace_text(child);
        if !keep {
            if let HtmlNode::Text { text } = child {
                trailing.push_str(text);
            }
        }
        seen += 1;
        keep
    });
    trailing.push_str(after_root);
    if trailing.is_empty() {
        return;
    }
    let HtmlNode::Element { children: body_children, .. } = &mut children[body] else { return };
    match body_children.last_mut() {
        Some(HtmlNode::Text { text }) => text.push_str(&trailing),
        _ => body_children.push(HtmlNode::Text { text: trailing }),
    }
}

/// 🔓️ Parses a complete well-formed HTML5 document (optional leading `<!DOCTYPE ...>` + exactly
/// one root element + only whitespace before/after). See the module doc comment for the "honest
/// boundary" this subset draws, and [`normalize_html_root_whitespace`] for the two normative
/// tree-construction whitespace placements applied to an `<html>` root.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_html_document(text: &str) -> Result<HtmlSnapshot, TextError> {
    let mut p = Parser::new(text);
    p.skip_ws();

    let doctype = if p.peek_str_ci("<!doctype") {
        p.expect(b'<')?;
        p.expect(b'!')?;
        let start = p.pos;
        while p.peek() != Some(b'>') {
            if p.advance().is_none() {
                return Err(p.err("unterminated <!DOCTYPE ...> declaration, expected '>'"));
            }
        }
        let content = p.slice(start, p.pos).to_string();
        p.advance();
        Some(content)
    } else if p.peek_str("<!") && !p.peek_str("<!--") {
        return Err(p.err("unsupported '<!' construct at document top level (only <!DOCTYPE ...> and <!-- comments --> are supported)"));
    } else {
        None
    };

    p.skip_ws();
    let mut root = p.parse_element()?;
    let tail_start = p.pos;
    p.skip_ws();
    if p.pos != p.bytes.len() {
        return Err(p.err("trailing content after the root element"));
    }
    let after_root = p.slice(tail_start, p.pos).to_string();
    normalize_html_root_whitespace(&mut root, &after_root);
    Ok(HtmlSnapshot { schema: STDIO_HTML_DOCUMENT_SCHEMA.into(), doctype, root })
}

/// 🖊️ Serializes a snapshot back to HTML5 text. Canonical/normalized form (documented per the
/// ticket brief's "documented-honest normalization" allowance — the fixed `{doctype, root}`
/// snapshot shape has no slot for the raw bytes between the doctype and the root element, so those
/// are normalized to a single `\n`): `<!doctype>\n` (if present) + the root element, verbatim inside
/// its own subtree (all inter-tag whitespace INSIDE the root IS a real `Text` node and round-trips
/// exactly). Attribute values are always re-emitted double-quoted regardless of the source's
/// original quote style (a second documented normalization — the `HtmlAttr{name,value}` shape has no
/// slot to remember which quote character was used).
///
/// ⚠️ Nothing follows the root element — not even a courtesy `\n`. Whitespace after `</html>` is NOT
/// inert in HTML: WHATWG §13.2.6.4.22 "after after body" processes it with the "in body" rules, so a
/// trailing newline re-enters `<body>`'s last text node on the very next read by any conformant
/// parser. Emitting one made `write` → `html5ever::parse` grow a newline inside `body` on every
/// cycle (found by `🌐️mutate-html-5`'s parity row, ticket
/// 26/08/23/END-TO-END-TESTING-REFACTOR); [`parse_html_document`] carries that whitespace into the
/// model instead, where it round-trips as the real text node it is.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_html_document(snapshot: &HtmlSnapshot) -> String {
    let mut out = String::new();
    if let Some(doctype) = &snapshot.doctype {
        out.push_str("<!");
        out.push_str(doctype);
        out.push_str(">\n");
    }
    write_node(&snapshot.root, &mut out);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_node(node: &HtmlNode, out: &mut String) {
    enum Action<'a> { Node(&'a HtmlNode), Close(&'a str) }
    let mut stack = vec![Action::Node(node)];
    while let Some(action) = stack.pop() {
        match action {
            Action::Close(name) => { out.push_str("</"); out.push_str(name); out.push('>'); },
            Action::Node(node) => match node {
                HtmlNode::Text { text } => out.push_str(&encode_text(text)),
                HtmlNode::Comment { text } => { out.push_str("<!--"); out.push_str(text); out.push_str("-->"); },
                HtmlNode::RawText { text, .. } => out.push_str(text),
                HtmlNode::Element { name, attributes, children } => {
                    out.push('<'); out.push_str(name);
                    for attr in attributes {
                        out.push(' '); out.push_str(&attr.name);
                        if let Some(value) = &attr.value { out.push_str("=\""); out.push_str(&encode_attr_value(value)); out.push('"'); }
                    }
                    out.push('>');
                    if !is_void_element(name) { stack.push(Action::Close(name)); for child in children.iter().rev() { stack.push(Action::Node(child)); } }
                },
            },
        }
    }
}

impl store::ArtifactDsl for HtmlSnapshot {
    const EXTENSION: &'static str = "html";
    fn envelope_id() -> &'static str {
        STDIO_HTML_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_html_document(body)
    }

    fn print_dsl(&self) -> String {
        let body = write_html_document(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v5::subsets::any::schema::snapshot::*;
use semio_framework_diagnostic::TextSpan;
use framework_schema::ArtifactSchema;
use semio_framework_diagnostic::TextError;

/// 🔓️ Decodes the small honest entity subset (see module doc comment) inside text/attribute
/// content. Any `&`-sequence outside that subset (malformed, or a real named reference like
/// `&nbsp;` this subset doesn't model) is passed through byte-for-byte, never dropped or errored.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_entities(raw: &str) -> String {
    let bytes = raw.as_bytes();
    let mut out = String::with_capacity(raw.len());
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] != b'&' {
            // 🧭️ Advance by one CHAR (not byte) to stay on UTF-8 boundaries.
            let ch_len = utf8_char_len(bytes[i]);
            out.push_str(&raw[i..i + ch_len]);
            i += ch_len;
            continue;
        }
        if let Some((decoded, consumed)) = try_decode_entity(&raw[i..]) {
            out.push(decoded);
            i += consumed;
        } else {
            out.push('&');
            i += 1;
        }
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn utf8_char_len(lead: u8) -> usize {
    if lead >= 0xF0 {
        4
    } else if lead >= 0xE0 {
        3
    } else if lead >= 0xC0 {
        2
    } else {
        1
    }
}

/// 🔓️ Attempts to decode ONE entity starting at `s[0] == '&'`. Returns `(char, bytes_consumed)`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn try_decode_entity(s: &str) -> Option<(char, usize)> {
    let named: &[(&str, char)] = &[("&amp;", '&'), ("&lt;", '<'), ("&gt;", '>'), ("&quot;", '"'), ("&apos;", '\'')];
    for (lit, ch) in named {
        if s.starts_with(lit) {
            return Some((*ch, lit.len()));
        }
    }
    if let Some(rest) = s.strip_prefix("&#x").or_else(|| s.strip_prefix("&#X")) {
        let hex: String = rest.chars().take_while(|c| c.is_ascii_hexdigit()).collect();
        if !hex.is_empty() && rest[hex.len()..].starts_with(';') {
            let code = u32::from_str_radix(&hex, 16).ok()?;
            let ch = char::from_u32(code)?;
            return Some((ch, 3 + hex.len() + 1));
        }
        return None;
    }
    if let Some(rest) = s.strip_prefix("&#") {
        let dec: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
        if !dec.is_empty() && rest[dec.len()..].starts_with(';') {
            let code: u32 = dec.parse().ok()?;
            let ch = char::from_u32(code)?;
            return Some((ch, 2 + dec.len() + 1));
        }
    }
    None
}

/// 🚶️ Byte-cursor recursive-descent parser with 1-based line/column tracking for `TextError`
/// spans, same shape as `stdio.json`'s `Parser`. Operates on the UTF-8 byte slice of a valid
/// `&str` — multi-byte characters are never mistaken for an ASCII delimiter (continuation bytes
/// are always `0x80..=0xBF`), so every slice point found by scanning for `<`/`>`/`&`/quotes/etc.
/// is a valid UTF-8 boundary.
pub(crate) struct Parser<'a> {
    src: &'a str,
    pub(crate) bytes: &'a [u8],
    pub(crate) pos: usize,
    line: u32,
    col: u32,
}

impl<'a> Parser<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn new(text: &'a str) -> Self {
        Self { src: text, bytes: text.as_bytes(), pos: 0, line: 1, col: 1 }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn peek_at(&self, offset: usize) -> Option<u8> {
        self.bytes.get(self.pos + offset).copied()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn advance(&mut self) -> Option<u8> {
        let byte = self.peek()?;
        self.pos += 1;
        if byte == b'\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(byte)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn span(&self) -> TextSpan {
        TextSpan::at(self.line, self.col)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn err(&self, message: impl Into<String>) -> TextError {
        TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message, self.span())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\n' | b'\r' | 0x0C)) {
            self.advance();
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn expect(&mut self, byte: u8) -> Result<(), TextError> {
        match self.peek() {
            Some(b) if b == byte => {
                self.advance();
                Ok(())
            }
            Some(other) => Err(self.err(format!("expected '{}', found '{}'", byte as char, other as char))),
            None => Err(self.err(format!("expected '{}', found end of input", byte as char))),
        }
    }

    /// 🔎 Literal, case-sensitive prefix check at the current position (no consumption).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn peek_str(&self, lit: &str) -> bool {
        self.src[self.pos..].starts_with(lit)
    }

    /// 🔎 ASCII case-insensitive prefix check at the current position (no consumption). Compares
    /// raw BYTES (not a `&str` slice) — `pos + lit.len()` is an arithmetically-computed end offset
    /// with no guarantee of landing on a UTF-8 char boundary, and `&str` slicing at a non-boundary
    /// offset panics where `&[u8]` slicing does not (same rationale as
    /// `read_raw_text_until_close`'s probe).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn peek_str_ci(&self, lit: &str) -> bool {
        let end = self.pos + lit.len();
        end <= self.bytes.len() && self.bytes[self.pos..end].eq_ignore_ascii_case(lit.as_bytes())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn slice(&self, start: usize, end: usize) -> &'a str {
        &self.src[start..end]
    }

    /// 🔎 Whether the byte at `pos + offset` is a "tag boundary" (whitespace, `>`, `/`) — used to
    /// avoid matching `</script2>` when looking for `</script>`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn is_boundary_at(&self, offset: usize) -> bool {
        match self.peek_at(offset) {
            None => true,
            Some(b) => matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0C | b'>' | b'/'),
        }
    }
}

/// 🔤️ Valid HTML5 tag-name / attribute-name continuation character (permissive superset: ASCII
/// alnum plus the common `-`/`:`/`_`/`.` seen in custom elements and `data-*`/namespaced attrs).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn is_name_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b':' | b'.')
}

impl<'a> Parser<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn parse_name(&mut self) -> Result<String, TextError> {
        let start = self.pos;
        while matches!(self.peek(), Some(b) if is_name_byte(b)) {
            self.advance();
        }
        if self.pos == start {
            return Err(self.err("expected a name"));
        }
        Ok(self.slice(start, self.pos).to_string())
    }

    /// 🏷️ `name` / `name=value` / `name="value"` / `name='value'`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn parse_attribute(&mut self) -> Result<HtmlAttr, TextError> {
        let name = self.parse_name()?;
        let save = self.pos;
        self.skip_ws();
        if self.peek() == Some(b'=') {
            self.advance();
            self.skip_ws();
            let raw = match self.peek() {
                Some(q @ (b'"' | b'\'')) => {
                    self.advance();
                    let start = self.pos;
                    while self.peek() != Some(q) {
                        if self.advance().is_none() {
                            return Err(self.err(format!("unterminated attribute value for '{name}'")));
                        }
                    }
                    let raw = self.slice(start, self.pos).to_string();
                    self.advance(); // closing quote
                    raw
                }
                Some(_) => {
                    let start = self.pos;
                    while matches!(self.peek(), Some(b) if !matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0C | b'>')) {
                        self.advance();
                    }
                    self.slice(start, self.pos).to_string()
                }
                None => return Err(self.err(format!("unterminated tag: expected value for attribute '{name}'"))),
            };
            Ok(HtmlAttr { name, value: Some(decode_entities(&raw)) })
        } else {
            // ↩️ No '=' -- rewind past any whitespace we speculatively skipped; the caller's own
            // loop re-does `skip_ws()` before deciding what comes next.
            self.pos = save;
            Ok(HtmlAttr { name, value: None })
        }
    }

    /// 🏗️ `<name attr...>` or `<name attr.../>`, returning `(name, attributes, self_closed)`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn parse_start_tag(&mut self) -> Result<(String, Vec<HtmlAttr>, bool), TextError> {
        self.expect(b'<')?;
        let name = self.parse_name()?;
        let mut attributes = Vec::new();
        let self_closed = loop {
            self.skip_ws();
            match self.peek() {
                Some(b'>') => {
                    self.advance();
                    break false;
                }
                Some(b'/') => {
                    self.advance();
                    self.expect(b'>').map_err(|_| self.err(format!("expected '>' after '/' in tag '<{name}'")))?;
                    break true;
                }
                Some(_) => attributes.push(self.parse_attribute()?),
                None => return Err(self.err(format!("unterminated start tag '<{name}'"))),
            }
        };
        Ok((name, attributes, self_closed))
    }

    /// 🚪️ `</name>` (whitespace before `>` tolerated). Returns the closing tag's own name (NOT
    /// forced to match the caller's expectation — the caller compares case-insensitively).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn parse_end_tag(&mut self) -> Result<String, TextError> {
        self.expect(b'<')?;
        self.expect(b'/')?;
        let name = self.parse_name()?;
        self.skip_ws();
        self.expect(b'>').map_err(|_| self.err(format!("expected '>' to close end tag '</{name}'")))?;
        Ok(name)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn read_comment(&mut self) -> Result<String, TextError> {
        // 🎯 Assumes "<!--" already consumed by the caller.
        let start = self.pos;
        while !self.peek_str("-->") {
            if self.advance().is_none() {
                return Err(self.err("unterminated comment, expected '-->'"));
            }
        }
        let content = self.slice(start, self.pos).to_string();
        self.advance();
        self.advance();
        self.advance();
        Ok(content)
    }

    /// 📄️ Reads RAWTEXT content verbatim (no entity decoding, no nested-markup parsing — matches
    /// HTML5's RAWTEXT content model for `<script>`/`<style>`) up to (not including) the matching
    /// case-insensitive `</tag` close-tag boundary. Compares raw BYTES (not `&str` slices) for the
    /// probe — `probe_end` is an arithmetically-computed offset (`pos + 2 + tag.len()`) with no
    /// guarantee of landing on a UTF-8 char boundary when the source contains multi-byte content
    /// right after a stray `</`, and `&str` slicing at a non-boundary offset panics; `&[u8]`
    /// slicing never does, so byte comparison keeps this parser panic-free on adversarial input
    /// (falls through to "not a match, keep scanning" instead of crashing).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn read_raw_text_until_close(&mut self, tag: &str) -> Result<String, TextError> {
        let start = self.pos;
        loop {
            if self.peek() == Some(b'<') && self.peek_at(1) == Some(b'/') {
                let probe_start = self.pos + 2;
                let probe_end = probe_start + tag.len();
                if probe_end <= self.bytes.len() && self.bytes[probe_start..probe_end].eq_ignore_ascii_case(tag.as_bytes()) {
                    let saved_offset = probe_end - self.pos;
                    if self.is_boundary_at(saved_offset) {
                        break;
                    }
                }
            }
            if self.advance().is_none() {
                return Err(self.err(format!("unterminated raw text content, expected '</{tag}>'")));
            }
        }
        Ok(self.slice(start, self.pos).to_string())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn read_text_until_lt(&mut self) -> String {
        let start = self.pos;
        while !matches!(self.peek(), Some(b'<') | None) {
            self.advance();
        }
        decode_entities(self.slice(start, self.pos))
    }

    /// 🌳 Parses one element and its full subtree, starting at `<`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn parse_element(&mut self) -> Result<HtmlNode, TextError> {
        let open_span = self.span();
        let (name, attributes, self_closed) = self.parse_start_tag()?;

        if is_void_element(&name) {
            return Ok(HtmlNode::Element { name, attributes, children: Vec::new() });
        }
        if self_closed {
            return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("'/>' self-closing syntax is only supported on void elements, found on non-void '<{name}/>'"), open_span));
        }

        if let Some(kind) = RawTextKind::from_tag_name(&name) {
            let raw = self.read_raw_text_until_close(&name)?;
            let close_name = self.parse_end_tag()?;
            if !close_name.eq_ignore_ascii_case(&name) {
                return Err(self.err(format!("mismatched close tag: expected '</{name}>', found '</{close_name}>'")));
            }
            let children = if raw.is_empty() { Vec::new() } else { vec![HtmlNode::RawText { parent_kind: kind, text: raw }] };
            return Ok(HtmlNode::Element { name, attributes, children });
        }

        let mut children = Vec::new();
        loop {
            match self.peek() {
                None => return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unterminated element '<{name}>', expected '</{name}>'"), open_span)),
                Some(b'<') => {
                    if self.peek_str("<!--") {
                        self.pos += 4;
                        self.col += 4;
                        children.push(HtmlNode::Comment { text: self.read_comment()? });
                    } else if self.peek_at(1) == Some(b'/') {
                        let close_name = self.parse_end_tag()?;
                        if !close_name.eq_ignore_ascii_case(&name) {
                            return Err(self.err(format!("mismatched close tag: expected '</{name}>', found '</{close_name}>'")));
                        }
                        break;
                    } else {
                        children.push(self.parse_element()?);
                    }
                }
                Some(_) => children.push(HtmlNode::Text { text: self.read_text_until_lt() }),
            }
        }
        Ok(HtmlNode::Element { name, attributes, children })
    }
}
}
pub use snapshot_wire_codec::*;
