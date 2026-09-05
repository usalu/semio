//! 🧬️ HtmlSnapshot schema — own `HtmlNode` recursive tree model + a from-scratch WHATWG-inspired
//! HTML5 tokenizer/parser and serializer. HTML is NOT XML: no shared types with `📰️xml`/`🎨️svg`
//! (only the general "recursive node tree" *structural pattern* is borrowed, per the ticket brief)
//! — own element/text/comment/raw-text node kinds, own void-element handling, own (deliberately
//! small) entity table.
//!
//! ## Honest boundary (documented per the ticket brief)
//! `✳️any` accepts **well-formed HTML5 documents only**. Full HTML5 "error recovery" parsing (the
//! WHATWG parsing algorithm's tree-construction insertion modes, implied end tags, the adoption
//! agency algorithm, foster parenting, etc.) is genuinely out of scope for a from-scratch
//! implementation — malformed/tag-soup markup is rejected with a `TextError`, never silently
//! "fixed up". A second, small honest boundary: only the five XML-equivalent named character
//! references (`&amp; &lt; &gt; &quot; &apos;`) plus numeric character references (`&#DD;` /
//! `&#xHH;`) are decoded — the full WHATWG named-character-reference table (~2200 entries, e.g.
//! `&nbsp;`) is not reproduced here; any other `&name;`-shaped sequence is passed through literally
//! as raw text (never an error, never silently corrupted).
//!
//! Out of scope is not the same as free: for the WELL-FORMED documents this subset does accept, the
//! tree it builds must be the tree every other HTML5 implementation builds, because the mutation
//! vocabulary addresses nodes by child index. The two normative placements a purely literal reader
//! gets wrong are applied by [`normalize_html_root_whitespace`] — see its own doc comment.

use dsl::TextSpan;
use schema::ArtifactSchema;
use store::TextError;

//#region 🔖️Ids
pub const STDIO_HTML_DOCUMENT_SCHEMA: &str = "stdio.html";
//#endregion 🔖️Ids

//#region 🔖️Model
/// 🏷️ One element attribute. `value: None` is a valueless boolean attribute (`<p disabled>`),
/// distinct from an attribute that isn't present at all.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct HtmlAttr {
    pub name: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl HtmlAttr {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self { name: name.into(), value: Some(value.into()) }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn boolean(name: impl Into<String>) -> Self {
        Self { name: name.into(), value: None }
    }
}

/// 🍃️ Which RAWTEXT element a [`HtmlNode::RawText`] node's content belongs to — `<script>` and
/// `<style>` are the only two RAWTEXT-content-model elements this subset models (HTML5 also gives
/// `<textarea>`/`<title>` a related-but-distinct RCDATA content model, out of scope here: their
/// content is parsed as plain `Text`, entity-decoded like everywhere else).
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum RawTextKind {
    Script,
    Style,
}

impl RawTextKind {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn tag_name(self) -> &'static str {
        match self {
            RawTextKind::Script => "script",
            RawTextKind::Style => "style",
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn from_tag_name(name: &str) -> Option<Self> {
        if name.eq_ignore_ascii_case("script") {
            Some(RawTextKind::Script)
        } else if name.eq_ignore_ascii_case("style") {
            Some(RawTextKind::Style)
        } else {
            None
        }
    }
}

/// 🌳 A node in the HTML5 document tree.
// NOTE: every non-unit variant MUST be a struct variant (named field), never a bare tuple variant
// -- serde's internally-tagged (`tag = "kind"`) representation can only merge the tag into
// map-shaped content; a tuple variant wrapping a non-map type compiles but fails at RUNTIME
// serialization ("can only flatten structs and maps"). Same real finding already on record for
// `stdio.json`'s `JsonValue` (see that file's identical NOTE) -- `Text`/`Comment` are therefore
// `{ text: String }` struct variants, not the bare-tuple `Text(String)`/`Comment(String)` shorthand
// used in the ticket brief's conceptual shape.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum HtmlNode {
    Element {
        name: String,
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        attributes: Vec<HtmlAttr>,
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        children: Vec<HtmlNode>,
    },
    Text {
        text: String,
    },
    Comment {
        text: String,
    },
    RawText {
        parent_kind: RawTextKind,
        text: String,
    },
}

impl HtmlNode {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn element(name: impl Into<String>) -> Self {
        HtmlNode::Element { name: name.into(), attributes: Vec::new(), children: Vec::new() }
    }
}

/// 🧭️ Path from the document root to a node: chain of child indices at each nesting level. `[]`
/// addresses the root itself.
pub type NodePath = Vec<usize>;

/// 📸️ Persisted `stdio.html` snapshot: `doctype` (raw content between `<!` and `>`, e.g.
/// `"DOCTYPE html"`, `None` if the document has no doctype declaration) + the recursive `root`
/// element tree.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.html")]
pub struct HtmlSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub doctype: Option<String>,
    #[state(artifact)]
    pub root: HtmlNode,
}

impl Default for HtmlSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_HTML_DOCUMENT_SCHEMA.into(), doctype: Some("DOCTYPE html".into()), root: HtmlNode::element("html") }
    }
}
//#endregion 🔖️Model

//#region 🔖️VoidElements
/// 🚪️ The HTML5/WHATWG void-element set (14 elements) — these never have a closing tag and never
/// carry children; the encoder must not emit `</tag>` (or self-close `/>`) for them.
const VOID_ELEMENTS: &[&str] = &["area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source", "track", "wbr"];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn is_void_element(name: &str) -> bool {
    VOID_ELEMENTS.iter().any(|v| v.eq_ignore_ascii_case(name))
}
//#endregion 🔖️VoidElements

//#region 🔖️Entities
/// 🔓️ Decodes the small honest entity subset (see module doc comment) inside text/attribute
/// content. Any `&`-sequence outside that subset (malformed, or a real named reference like
/// `&nbsp;` this subset doesn't model) is passed through byte-for-byte, never dropped or errored.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_entities(raw: &str) -> String {
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
fn utf8_char_len(lead: u8) -> usize {
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
fn try_decode_entity(s: &str) -> Option<(char, usize)> {
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

/// 🔒️ Escapes text-node content for re-serialization (`&`, `<`, `>`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_text(s: &str) -> String {
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
fn encode_attr_value(s: &str) -> String {
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
//#endregion 🔖️Entities

//#region 🔖️Parser
/// 🚶️ Byte-cursor recursive-descent parser with 1-based line/column tracking for `TextError`
/// spans, same shape as `stdio.json`'s `Parser`. Operates on the UTF-8 byte slice of a valid
/// `&str` — multi-byte characters are never mistaken for an ASCII delimiter (continuation bytes
/// are always `0x80..=0xBF`), so every slice point found by scanning for `<`/`>`/`&`/quotes/etc.
/// is a valid UTF-8 boundary.
struct Parser<'a> {
    src: &'a str,
    bytes: &'a [u8],
    pos: usize,
    line: u32,
    col: u32,
}

impl<'a> Parser<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn new(text: &'a str) -> Self {
        Self { src: text, bytes: text.as_bytes(), pos: 0, line: 1, col: 1 }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn peek_at(&self, offset: usize) -> Option<u8> {
        self.bytes.get(self.pos + offset).copied()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn advance(&mut self) -> Option<u8> {
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
    fn span(&self) -> TextSpan {
        TextSpan::at(self.line, self.col)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn err(&self, message: impl Into<String>) -> TextError {
        TextError::new(message, self.span())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\n' | b'\r' | 0x0C)) {
            self.advance();
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn expect(&mut self, byte: u8) -> Result<(), TextError> {
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
    fn peek_str(&self, lit: &str) -> bool {
        self.src[self.pos..].starts_with(lit)
    }

    /// 🔎 ASCII case-insensitive prefix check at the current position (no consumption). Compares
    /// raw BYTES (not a `&str` slice) — `pos + lit.len()` is an arithmetically-computed end offset
    /// with no guarantee of landing on a UTF-8 char boundary, and `&str` slicing at a non-boundary
    /// offset panics where `&[u8]` slicing does not (same rationale as
    /// `read_raw_text_until_close`'s probe).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn peek_str_ci(&self, lit: &str) -> bool {
        let end = self.pos + lit.len();
        end <= self.bytes.len() && self.bytes[self.pos..end].eq_ignore_ascii_case(lit.as_bytes())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn slice(&self, start: usize, end: usize) -> &'a str {
        &self.src[start..end]
    }

    /// 🔎 Whether the byte at `pos + offset` is a "tag boundary" (whitespace, `>`, `/`) — used to
    /// avoid matching `</script2>` when looking for `</script>`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn is_boundary_at(&self, offset: usize) -> bool {
        match self.peek_at(offset) {
            None => true,
            Some(b) => matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0C | b'>' | b'/'),
        }
    }
}

/// 🔤️ Valid HTML5 tag-name / attribute-name continuation character (permissive superset: ASCII
/// alnum plus the common `-`/`:`/`_`/`.` seen in custom elements and `data-*`/namespaced attrs).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_name_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b':' | b'.')
}

impl<'a> Parser<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn parse_name(&mut self) -> Result<String, TextError> {
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
    fn parse_attribute(&mut self) -> Result<HtmlAttr, TextError> {
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
    fn parse_start_tag(&mut self) -> Result<(String, Vec<HtmlAttr>, bool), TextError> {
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
    fn parse_end_tag(&mut self) -> Result<String, TextError> {
        self.expect(b'<')?;
        self.expect(b'/')?;
        let name = self.parse_name()?;
        self.skip_ws();
        self.expect(b'>').map_err(|_| self.err(format!("expected '>' to close end tag '</{name}'")))?;
        Ok(name)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn read_comment(&mut self) -> Result<String, TextError> {
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
    fn read_raw_text_until_close(&mut self, tag: &str) -> Result<String, TextError> {
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
    fn read_text_until_lt(&mut self) -> Result<String, TextError> {
        let start = self.pos;
        while !matches!(self.peek(), Some(b'<') | None) {
            self.advance();
        }
        Ok(decode_entities(self.slice(start, self.pos)))
    }

    /// 🌳 Parses one element and its full subtree, starting at `<`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn parse_element(&mut self) -> Result<HtmlNode, TextError> {
        let open_span = self.span();
        let (name, attributes, self_closed) = self.parse_start_tag()?;

        if is_void_element(&name) {
            return Ok(HtmlNode::Element { name, attributes, children: Vec::new() });
        }
        if self_closed {
            return Err(TextError::new(format!("'/>' self-closing syntax is only supported on void elements, found on non-void '<{name}/>'"), open_span));
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
                None => return Err(TextError::new(format!("unterminated element '<{name}>', expected '</{name}>'"), open_span)),
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
                Some(_) => children.push(HtmlNode::Text { text: self.read_text_until_lt()? }),
            }
        }
        Ok(HtmlNode::Element { name, attributes, children })
    }
}

/// 🧽 Whitespace-only character data, per the WHATWG definition (TAB, LF, FF, CR, SPACE).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_whitespace_text(node: &HtmlNode) -> bool {
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
/// Found by `../../../../../🧪️tests/🟠️mutate-html-5`'s parity phase the first time it ran
/// (ticket 26/08/23/END-TO-END-TESTING-REFACTOR), where all seven path-addressed kinds were refused
/// with `mutation.apply.conflicting-target` — "element diff targets a non-element node".
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn normalize_html_root_whitespace(root: &mut HtmlNode, after_root: &str) {
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
//#endregion 🔖️Parser

//#region 🔖️Writer
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
/// cycle (found by `🟠️mutate-html-5`'s `set-snapshot` parity row, ticket
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
fn write_node(node: &HtmlNode, out: &mut String) {
    match node {
        HtmlNode::Text { text } => out.push_str(&encode_text(text)),
        HtmlNode::Comment { text } => {
            out.push_str("<!--");
            out.push_str(text);
            out.push_str("-->");
        }
        HtmlNode::RawText { text, .. } => out.push_str(text),
        HtmlNode::Element { name, attributes, children } => {
            out.push('<');
            out.push_str(name);
            for a in attributes {
                out.push(' ');
                out.push_str(&a.name);
                if let Some(v) = &a.value {
                    out.push_str("=\"");
                    out.push_str(&encode_attr_value(v));
                    out.push('"');
                }
            }
            out.push('>');
            if is_void_element(name) {
                return;
            }
            for child in children {
                write_node(child, out);
            }
            out.push_str("</");
            out.push_str(name);
            out.push('>');
        }
    }
}
//#endregion 🔖️Writer

//#region 🔖️Navigation
/// 🧭️ Resolves `path` (a chain of child indices from the document root) against `snapshot`,
/// erroring on any non-`Element` intermediate node or out-of-range index.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn node_at<'a>(snapshot: &'a HtmlSnapshot, path: &[usize]) -> Result<&'a HtmlNode, String> {
    let mut current = &snapshot.root;
    for &index in path {
        match current {
            HtmlNode::Element { children, .. } => {
                current = children.get(index).ok_or_else(|| format!("node path index {index} out of range"))?;
            }
            other => return Err(format!("node path descends into a non-element node: {other:?}")),
        }
    }
    Ok(current)
}

/// 🔎 Reads attribute `name`'s value from an `Element` node — `None` both when the attribute is
/// absent and when `node` isn't an `Element` (callers that need to distinguish those already have
/// `node_at`'s own `Result`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn element_attr<'a>(node: &'a HtmlNode, name: &str) -> Option<&'a Option<String>> {
    match node {
        HtmlNode::Element { attributes, .. } => attributes.iter().find(|a| a.name == name).map(|a| &a.value),
        _ => None,
    }
}
//#endregion 🔖️Navigation

//#region 🔖️HandcraftedArtifactCodecs
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

impl store::ArtifactPack for HtmlSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = write_html_document(self).into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        let text = std::str::from_utf8(&inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        parse_html_document(text).map_err(|e| store::PackError::Schema(e.to_string()))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = include_str!("../../📚️examples/🎬️demo/🖼️assets/🧪️example/🌐️.html");

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn el(name: &str, attrs: Vec<HtmlAttr>, children: Vec<HtmlNode>) -> HtmlNode {
        HtmlNode::Element { name: name.into(), attributes: attrs, children }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn text(s: &str) -> HtmlNode {
        HtmlNode::Text { text: s.into() }
    }

    /// 🧪 Pins WHATWG §13.2.6.4.3 "before head": a whitespace-only character token before `<head>`
    /// is IGNORED, so `<html>`'s first child is the `<head>` element itself. Without this the whole
    /// path space inside `<html>` is shifted one place away from every conformant HTML5 DOM.
    #[test]
    fn whitespace_before_head_is_not_a_node() {
        let snap = parse_html_document("<html lang=\"de\">\n  <head></head>\n  <body></body>\n</html>").unwrap();
        let HtmlNode::Element { children, .. } = &snap.root else { panic!("root not element") };
        assert!(matches!(&children[0], HtmlNode::Element { name, .. } if name == "head"), "got {:?}", children[0]);
        assert!(matches!(&children[1], HtmlNode::Text { text } if text == "\n  "));
        assert!(matches!(&children[2], HtmlNode::Element { name, .. } if name == "body"), "got {:?}", children[2]);
        assert_eq!(children.len(), 3, "nothing may follow <body>: {children:?}");
    }

    /// 🧪 Pins WHATWG §13.2.6.4.20 "after body" and §13.2.6.4.22 "after after body": whitespace
    /// between `</body>` and `</html>` AND after `</html>` is processed with the "in body" rules,
    /// whose insertion point is still `body`, and merges onto `body`'s last text node rather than
    /// becoming a child of `html` or being discarded.
    #[test]
    fn whitespace_after_body_belongs_to_body() {
        let snap = parse_html_document("<html><head></head><body><p>x</p>\n  </body>\n</html>\n").unwrap();
        let HtmlNode::Element { children, .. } = &snap.root else { panic!("root not element") };
        assert_eq!(children.len(), 2);
        let HtmlNode::Element { children: body, .. } = &children[1] else { panic!("body not element") };
        assert_eq!(body.len(), 2);
        assert!(matches!(&body[1], HtmlNode::Text { text } if text == "\n  \n\n"), "got {:?}", body[1]);
    }

    /// 🧪 Nothing may follow `</html>` on the way out: whatever came after it on the way in is
    /// already inside `body`, so a courtesy trailing newline would be a NEW character the next
    /// conformant read puts inside `body` again. `write` after `parse` is a true fixpoint.
    #[test]
    fn writing_never_emits_anything_after_the_root_element() {
        let source = "<!DOCTYPE html>\n<html><head></head><body><p>x</p>\n</body></html>";
        let printed = write_html_document(&parse_html_document(source).unwrap());
        assert_eq!(printed, source);
        assert_eq!(write_html_document(&parse_html_document(&printed).unwrap()), printed);
    }

    /// 🧪 The normalization is scoped to an `<html>` root: a fragment-shaped document keeps every
    /// text node exactly where the source put it.
    #[test]
    fn non_html_root_keeps_its_whitespace_verbatim() {
        let snap = parse_html_document("<section>\n  <p>x</p>\n</section>").unwrap();
        let HtmlNode::Element { children, .. } = &snap.root else { panic!("root not element") };
        assert_eq!(children.len(), 3);
        assert!(matches!(&children[0], HtmlNode::Text { text } if text == "\n  "));
    }

    #[semio_framework_async_macros::async_test]
    async fn parses_void_elements_without_children_or_close_tag() {
        let snap = parse_html_document("<html><br><img src=\"x.png\"></html>").unwrap();
        match &snap.root {
            HtmlNode::Element { children, .. } => {
                assert_eq!(children.len(), 2);
                assert!(matches!(&children[0], HtmlNode::Element { name, children, .. } if name == "br" && children.is_empty()));
                assert!(matches!(&children[1], HtmlNode::Element { name, children, .. } if name == "img" && children.is_empty()));
            }
            other => panic!("expected element root, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn parses_valueless_boolean_attribute() {
        let snap = parse_html_document("<html><p disabled>hi</p></html>").unwrap();
        match &snap.root {
            HtmlNode::Element { children, .. } => match &children[0] {
                HtmlNode::Element { attributes, .. } => {
                    assert_eq!(attributes.len(), 1);
                    assert_eq!(attributes[0].name, "disabled");
                    assert_eq!(attributes[0].value, None);
                }
                other => panic!("expected element, got {other:?}"),
            },
            other => panic!("expected element root, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn parses_comment_and_script_style_rawtext() {
        let snap = parse_html_document("<html><!-- hi --><style>.a { color: red; }</style><script>if (1 < 2) { console.log(\"</not-a-tag>\"); }</script></html>").unwrap();
        match &snap.root {
            HtmlNode::Element { children, .. } => {
                assert!(matches!(&children[0], HtmlNode::Comment { text } if text == " hi "));
                match &children[1] {
                    HtmlNode::Element { name, children, .. } => {
                        assert_eq!(name, "style");
                        assert!(matches!(&children[0], HtmlNode::RawText { parent_kind: RawTextKind::Style, text } if text == ".a { color: red; }"));
                    }
                    other => panic!("expected style element, got {other:?}"),
                }
                match &children[2] {
                    HtmlNode::Element { name, children, .. } => {
                        assert_eq!(name, "script");
                        assert!(matches!(&children[0], HtmlNode::RawText { parent_kind: RawTextKind::Script, text } if text.contains("</not-a-tag>")));
                    }
                    other => panic!("expected script element, got {other:?}"),
                }
            }
            other => panic!("expected element root, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn rejects_mismatched_close_tag() {
        assert!(parse_html_document("<html><div></span></html>").is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn rejects_self_closing_syntax_on_non_void_element() {
        assert!(parse_html_document("<html><div/></html>").is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn accepts_self_closing_syntax_on_void_element() {
        let snap = parse_html_document("<html><br/></html>").unwrap();
        match &snap.root {
            HtmlNode::Element { children, .. } => assert!(matches!(&children[0], HtmlNode::Element { name, .. } if name == "br")),
            other => panic!("expected element root, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn decodes_and_encodes_small_entity_subset_only() {
        let snap = parse_html_document("<html><p>a &amp; b &lt;3 &#65; &#x42; &nbsp;</p></html>").unwrap();
        match &snap.root {
            HtmlNode::Element { children, .. } => match &children[0] {
                HtmlNode::Element { children, .. } => {
                    assert!(matches!(&children[0], HtmlNode::Text { text } if text == "a & b <3 A B &nbsp;"));
                }
                other => panic!("expected p element, got {other:?}"),
            },
            other => panic!("expected element root, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_default_snapshot_has_html_root() {
        let snap = HtmlSnapshot::default();
        assert!(matches!(&snap.root, HtmlNode::Element { name, .. } if name == "html"));
    }

    #[semio_framework_async_macros::async_test]
    async fn nested_structure_round_trips_synthetic() {
        let snap = HtmlSnapshot {
            schema: STDIO_HTML_DOCUMENT_SCHEMA.into(),
            doctype: Some("DOCTYPE html".into()),
            root: el("html", vec![HtmlAttr::new("lang", "en")], vec![el("body", vec![], vec![el("p", vec![HtmlAttr::boolean("disabled")], vec![text("hi "), el("br", vec![], vec![]), text(" there")])])]),
        };
        let printed = write_html_document(&snap);
        let reparsed = parse_html_document(&printed).unwrap();
        assert_eq!(reparsed, snap);
    }

    //#region 🔖️CodecRetentionLaw
    /// 🎯️ codec_retention_law: byte-preserving round trip of the real W0 fixture. Exact, not just
    /// "documented-honest normalization" — the fixture's actual bytes follow this codec's canonical
    /// top-level-whitespace convention (a single `\n` after the doctype, `<head>` opening
    /// immediately after `<html …>` and `</html>` closing immediately after `</body>`, because
    /// WHATWG tree construction gives whitespace in those two places to nothing and to `body`
    /// respectively — see [`normalize_html_root_whitespace`]), and every attribute value is already
    /// double-quoted, so `decode -> re-encode` matches the source byte-for-byte.
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law() {
        let snap = parse_html_document(FIXTURE).expect("fixture parses");
        let re_encoded = write_html_document(&snap);
        assert_eq!(re_encoded, FIXTURE, "fixture must round-trip byte-for-byte");

        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <HtmlSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }
    //#endregion 🔖️CodecRetentionLaw

    #[semio_framework_async_macros::async_test]
    async fn snapshot_dsl_and_pack_round_trip() {
        let snap = parse_html_document(FIXTURE).expect("fixture parses");
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <HtmlSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed, snap);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <HtmlSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }
}
//#endregion 🧪️Tests
