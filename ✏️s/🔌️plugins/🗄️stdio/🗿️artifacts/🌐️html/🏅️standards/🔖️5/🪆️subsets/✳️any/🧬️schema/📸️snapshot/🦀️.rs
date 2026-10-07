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

use semio_framework_diagnostic::TextSpan;
use framework_schema::ArtifactSchema;
use semio_framework_diagnostic::TextError;

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
    pub(crate) fn from_tag_name(name: &str) -> Option<Self> {
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
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
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
pub(crate) const VOID_ELEMENTS: &[&str] = &["area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source", "track", "wbr"];


//#endregion 🔖️VoidElements

//#region 🔖️Entities









//#endregion 🔖️Entities

//#region 🔖️Parser













//#endregion 🔖️Parser

//#region 🔖️Writer



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



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests



#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests



