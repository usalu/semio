//! 📝️ Text representation codec surface for `stdio.semio.value` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::value::schema::snapshot::*;
use framework_schema::ArtifactSchema;

/// 🌳️ `SemioValueSnapshot`'s own real recursive text encoding — `[hex(schema),<value>,[<node>,...]]`
/// — genuinely walked/parsed field-by-field (never a hex dump of a `serde_json` blob). Reuses the
/// SAME tag-prefixed `SemioValue` grammar (`enc_semio_value`/`dec_semio_value`) the sibling
/// `🔺️diff`/`🧬️mutations` facets already define for their own text codecs — this subset has no
/// natural "on-disk file format" of its own the way `json`/`csv` do (`SemioValueSnapshot` is a
/// NEUTRAL semio type), so reusing one already-real, already-hand-rolled grammar as the single
/// source of truth for every facet is the honest choice, not a shortcut (`json`'s own `JsonValue`
/// text codec is likewise shared verbatim by its diff/mutations facets' `value=` token). Single
/// source of truth: `🧬️mutations/🦀️.rs`'s `SetSnapshot` argument encoding calls THESE
/// functions directly rather than keeping its own second copy.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_semio_value_snapshot(s: &SemioValueSnapshot) -> String {
    let nodes = s.nodes.iter().map(crate::standards::v1::subsets::value::io::text::diff::enc_semio_value_node).collect::<Vec<_>>().join(",");
    format!("[{},{},[{}]]", crate::standards::v1::subsets::value::io::text::diff::enc_str(&s.schema), crate::standards::v1::subsets::value::io::text::diff::enc_semio_value(&s.root), nodes)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_semio_value_snapshot(s: &str) -> Result<SemioValueSnapshot, String> {
    use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
    use crate::standards::v1::subsets::value::io::text::diff::{dec_semio_value_node};
    use crate::standards::v1::subsets::value::io::text::diff::{dec_semio_value};
    use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_str};
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [schema_s, root_s, nodes_s] = parts.as_slice() else {
        return Err(format!("semio value snapshot: expected 3 top-level fields, got {}", parts.len()));
    };
    let nodes_inner = strip_brackets(nodes_s)?;
    let nodes = split_top_level(nodes_inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_semio_value_node).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioValueSnapshot { schema: dec_str(schema_s)?, root: dec_semio_value(root_s)?, nodes })
}

/// 🎁️ Real recursive text/binary round trip — NOT a per-on-disk-file-format codec (this subset's
/// snapshot is a NEUTRAL semio type, like `json`'s own `JsonSnapshot`, not a real-world file
/// format), so — mirroring `json`'s own text-native precedent exactly (`🔣️json/…/📸️snapshot/
/// 🦀️.rs`'s `ArtifactPack::encode_pack_with`: `write_json_text(&self.value).into_bytes()`
/// passed straight to `wrap_binary`, no distinct "binary JsonValue" layout) — the PACK bytes are
/// the SAME real compact text this facet's DSL emits, wrapped in the semio envelope. No
/// `serde_json` anywhere in this impl block.
impl store::ArtifactDsl for SemioValueSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIOVALUE_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        dec_semio_value_snapshot(body.trim()).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = enc_semio_value_snapshot(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📤️ This subset's own `#[value(rename_all = "camelCase")]` structural JSON projection of
/// `stdio.semio.value` — the shape `🔢️mutate-semio-value` compares under `ordered-json-v1`, derived
/// from the snapshot type itself rather than hand-written a second time in the adapter, where it
/// could drift away from the type it claims to project. A thin `pack::to_json_string` wrapper
/// (first-party, over `ToValue`/`DslValue`). Mirrors `📊️table`'s and `🌊️flow`'s own bridges.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_value_snapshot_json(snapshot: &SemioValueSnapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ The `pack::from_json_str` inverse of [`encode_semio_value_snapshot_json`] — decodes a committed
/// `(before, mutation, after)` specification vector into a real [`SemioValueSnapshot`], so the case
/// adapter reads the committed fixture instead of re-declaring it as a Rust literal beside it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_value_snapshot_json(text: &str) -> Result<SemioValueSnapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📝️ Parses `stdio.semio.value` DSL text into a [`SemioValueSnapshot`] — a named pass-through of
/// this snapshot's own `store::ArtifactDsl` impl above, whose trait and error type are both
/// unnameable outside this crate, so `🔢️mutate-semio-value`'s `identity-round-trip` scenario reaches
/// the real committed artifact (`../../../../✉️base/📚️examples/🕸️graph/🖼️assets/🗣️.dsl.semio`)
/// through this instead.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_semio_value_dsl(text: &str) -> Result<SemioValueSnapshot, String> {
    <SemioValueSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 📝️ Renders a [`SemioValueSnapshot`] back as `stdio.semio.value` DSL text — the inverse of
/// [`parse_semio_value_dsl`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_semio_value_dsl(snapshot: &SemioValueSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::value::schema::snapshot::*;
use framework_schema::ArtifactSchema;




}
pub use snapshot_wire_codec::*;
