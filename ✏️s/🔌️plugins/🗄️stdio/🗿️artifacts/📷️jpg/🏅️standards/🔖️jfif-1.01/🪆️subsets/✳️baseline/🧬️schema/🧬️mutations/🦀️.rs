//! 🧬️ `JpgBaselineMutation` — the ITU-T T.81 / ISO 10918-1 BASELINE SEQUENTIAL DCT
//! conformance-class vocabulary of `stdio.jpg`, in a JFIF 1.01 container. Every variant's `diff()`
//! is handcrafted (it constructs the sparse `JpgDiff` directly — apply-and-capture is banned) and
//! every variant's `inverse()` is handcrafted, reading whatever pre-state it needs out of the base.
//!
//! # Why this subset needs a vocabulary of its own
//!
//! `✳️any` owns the DOCUMENT vocabulary — the JFIF header, the quantization and Huffman tables by
//! id, the restart interval, the retained segments, the raster and the re-encode quality. Not one
//! of those kinds addresses whether the document IS baseline: that is a property of the frame
//! header and the entropy-coding mode, and `check_baseline_conformance` (`../🦀️component.rs`) reads
//! exactly five axes of it:
//!
//! | Axis | Diagnostic | Restriction |
//! |---|---|---|
//! | `sof_marker` | `CODE_SOF_MARKER` (hard) | SOF0 (0xC0) — T.81 Annex F admits no other SOFn |
//! | `frame.precision` | `CODE_PRECISION` (hard) | 8 bits — T.81 §4.2 |
//! | `arithmetic` | `CODE_ARITHMETIC` (hard) | absent — Annex F is Huffman-entropy-coded only |
//! | `huffman_tables` count per class | `CODE_HUFFMAN_TABLE_COUNT` (soft) | at most 2 DC and 2 AC |
//! | `frame.components` count and sampling | `CODE_COMPONENT_SAMPLING` (soft) | at most 4 components, each sampling factor in 1..=4 |
//!
//! One variant per axis, plus the two baseline variants every vocabulary carries, plus the
//! insert/remove pairings the two counting axes need to be reachable in both directions — the same
//! one-kind-per-axis derivation the OOXML conformance-class subsets make from their own
//! `check_strict_conformance`.
//!
//! Two of these axes overlap the `✳️any` vocabulary by NAME and not by meaning.
//! `JpgMutation::SetHuffmanTable` sets the CONTENT of one table (its bit-length counts and value
//! bytes) for a document that already has the table it wants; `InsertHuffmanTable`/
//! `RemoveHuffmanTable` here exist to move the table COUNT across the ≤2-per-class line, which is
//! the only thing the conformance check reads. The same distinction separates
//! `SetComponentSampling` from any content edit: it moves a component's sampling factors across the
//! 1..=4 line, and touches nothing else.
//!
//! `Diff` is `JpgDiff`, the SAME diff type `✳️any` uses — the two subsets share one snapshot type,
//! so they share its diff. What differs is the vocabulary that produces it, which is what a subset
//! is.
//!
//! # Where this vocabulary is observable, and where it is not
//!
//! `encode_jpg` (`../../✳️any/🚪️io/🦀️component.rs`) writes a baseline file and nothing else, by
//! construction: `out.extend_from_slice(&[0xFF, 0xC0])` for the frame marker, `precision: 8`, a
//! fixed three-component 4:2:0 `comps` array with sampling factors 2x2/1x1/1x1, exactly four
//! `write_dht` calls, and no DAC segment anywhere. Every axis this vocabulary addresses is
//! therefore normalized away by the encoder — correctly, because each of those fields DESCRIBES the
//! entropy-coded scan that follows it and any other value would describe bytes the encoder did not
//! write, exactly as PNG's IHDR and TIFF's strip tags are constrained.
//!
//! That is a real and reportable property of this repository's JPEG encoder: it can serialize a
//! conforming baseline JPEG and no other kind of JPEG at all. What follows from it is NOT that this
//! vocabulary is untestable — it is that a BYTE-level exhaustive case built on this catalog would
//! report every kind as green while the mutation never reached a byte, which is the precise shape of
//! shallow green ticket 26/08/23/END-TO-END-TESTING-REFACTOR exists to remove.
//!
//! The catalog `jpg-jfif-1-01-baseline` (`../../🧪️oracle/🔣️.json`) is therefore declared
//! and claimed by `mutate-jpg-jfif-1-01-baseline`, and that case measures this vocabulary where its
//! axes actually live: on the DECODED SNAPSHOT, against [`check_baseline_conformance`]'s verdict.
//! Each kind must move its own axis and raise its own diagnostic; each inverse must restore the
//! snapshot exactly. The case states in as many words that it makes no byte-level claim, and its
//! `identity-round-trip` is the one scenario that does touch bytes — decode, re-encode, and read
//! both through the INDEPENDENT `image` reader the sibling `✳️any` subset registers.
//!
//! @see ../🦀️component.rs — this subset's conformance check, one axis per variant below.
//! @see ../../✳️any/🧬️schema/🧬️mutations/🦀️component.rs — the DOCUMENT vocabulary this one is disjoint from.

use crate::artifacts::jpg::standards::v_jfif_1_01::subsets::document::schema::diff::{JpgComponentDiff, JpgComponentModified, JpgComponentsDiff, JpgDiff, JpgFrameChange, JpgFrameFieldsDiff, JpgHuffmanTableAdded, JpgHuffmanTableKey, JpgHuffmanTablesDiff};
use crate::artifacts::jpg::standards::v_jfif_1_01::subsets::document::schema::snapshot::{JpgFrameComponent, JpgHuffmanTable, JpgSnapshot};
use protocol::{Mutation, MutationDiff};

//#region 🔖️Dialect
/// 🏷️ SOF0 — the baseline sequential DCT frame marker, T.81 Table B.1. The one value
/// `CODE_SOF_MARKER` accepts.
pub const SOF0: u8 = 0xC0;
/// 🏷️ Baseline sequential DCT's mandated sample precision, T.81 §4.2.
pub const BASELINE_PRECISION: u8 = 8;
/// 🏷️ The per-class Huffman table budget `CODE_HUFFMAN_TABLE_COUNT` reports against.
pub const BASELINE_TABLES_PER_CLASS: usize = 2;
//#endregion 🔖️Dialect

//#region 🔖️Mutations
/// 📐️ Typed conformance-class mutation for `stdio.jpg` under T.81 baseline sequential DCT. Every
/// variant addresses ONE axis of the class; none addresses document content.
//#region 🔖️Leaves
#[path = "🔧set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🔩set-sof-marker/🦀️.rs"]
pub mod set_sof_marker;
#[path = "⚙set-sample-precision/🦀️.rs"]
pub mod set_sample_precision;
#[path = "🧩set-arithmetic/🦀️.rs"]
pub mod set_arithmetic;
#[path = "🔖insert-huffman-table/🦀️.rs"]
pub mod insert_huffman_table;
#[path = "🏷remove-huffman-table/🦀️.rs"]
pub mod remove_huffman_table;
#[path = "📐insert-frame-component/🦀️.rs"]
pub mod insert_frame_component;
#[path = "📏remove-frame-component/🦀️.rs"]
pub mod remove_frame_component;
#[path = "🧮set-component-sampling/🦀️.rs"]
pub mod set_component_sampling;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = JpgSnapshot, diff = JpgDiff, schema = "JpgBaselineMutation")]
pub enum JpgBaselineMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    SetSofMarker(set_sof_marker::SetSofMarker),
    SetSamplePrecision(set_sample_precision::SetSamplePrecision),
    SetArithmetic(set_arithmetic::SetArithmetic),
    InsertHuffmanTable(insert_huffman_table::InsertHuffmanTable),
    RemoveHuffmanTable(remove_huffman_table::RemoveHuffmanTable),
    InsertFrameComponent(insert_frame_component::InsertFrameComponent),
    RemoveFrameComponent(remove_frame_component::RemoveFrameComponent),
    SetComponentSampling(set_component_sampling::SetComponentSampling),
}

/// 🏷️ Kebab-case spelling of every `JpgBaselineMutation` variant, in declaration order — the
/// vocabulary the `jpg-jfif-1-01-baseline` mutation catalog (`../../🧪️oracle/🔣️.json`)
/// declares and `mutate-jpg-jfif-1-01-baseline` measures itself against.
/// `kinds_match_enum_variants_in_declaration_order` below is what keeps the two honest against the
/// enum, and `kinds_match_the_committed_catalog` against the manifest.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-sof-marker", "set-sample-precision", "set-arithmetic", "insert-huffman-table", "remove-huffman-table", "insert-frame-component", "remove-frame-component", "set-component-sampling"];

crate::impl_serde_op_codec!(JpgBaselineMutation, "jpg-baseline-mutation");

//#region 🌉️ConformanceProjection
/// 👁️ The comparison surface `mutate-jpg-jfif-1-01-baseline` measures this vocabulary through: the
/// five T.81 Annex F axes as they stand on the DECODED snapshot, plus
/// [`check_baseline_conformance`]'s verdict over them. It carries no pixels and no quantization
/// tables on purpose — this is a conformance-class vocabulary, and a class is a property of the
/// frame header and the entropy-coding mode, not of the raster.
///
/// Rendered by hand rather than through `serde` because it is a PROJECTION, not the snapshot: the
/// snapshot's own serialization carries a multi-megabyte raster that no comparison here should ever
/// have to walk, and the axes are exactly the ten this subset's own checker reads.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_jpg_baseline_projection_json(snapshot: &JpgSnapshot) -> String {
    let quoted = |values: Vec<String>| format!("[{}]", values.into_iter().map(|value| format!("\"{value}\"")).collect::<Vec<_>>().join(","));
    let tables = quoted(snapshot.huffman_tables.iter().map(|table| format!("{:?}:{}", table.class, table.id).to_lowercase()).collect());
    let components = quoted(snapshot.frame.as_ref().map(|frame| frame.components.iter().map(|component| format!("{}:{}x{}", component.id, component.h_sampling, component.v_sampling)).collect()).unwrap_or_default());
    let verdict = quoted(crate::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::schema::check_baseline_conformance(snapshot).into_iter().map(|finding| finding.code.0.to_string()).collect());
    format!(
        "{{\"format\":\"jpg-baseline\",\"sofMarker\":\"{:02x}\",\"precision\":{},\"arithmetic\":{},\"componentCount\":{},\"huffmanTables\":{tables},\"components\":{components},\"conformance\":{verdict}}}",
        snapshot.sof_marker,
        snapshot.frame.as_ref().map(|frame| frame.precision).unwrap_or(0),
        snapshot.arithmetic,
        snapshot.frame.as_ref().map(|frame| frame.components.len()).unwrap_or(0)
    )
}

/// 🛡️ [`check_baseline_conformance`]'s verdict as bare diagnostic codes — what a `mutate-<kind>`
/// scenario names when it claims a kind leaves the class by its own axis.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn jpg_baseline_conformance_codes(snapshot: &JpgSnapshot) -> Vec<String> {
    crate::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::schema::check_baseline_conformance(snapshot).into_iter().map(|finding| finding.code.0.to_string()).collect()
}
//#endregion 🌉️ConformanceProjection
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot` through its own diff — the diff is the single semantics
/// source, never a separate imperative apply path.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_jpg_baseline_mutation(snapshot: &mut JpgSnapshot, mutation: &JpgBaselineMutation) -> protocol::MutationOutcome<JpgDiff> {
    let outcome = <JpgBaselineMutation as Mutation<JpgSnapshot>>::diff(mutation, snapshot);
    match MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

/// ↩️ This subset's own inverse algebra as a free function, so a caller that legitimately drives the
/// vocabulary from outside the crate reaches it without naming the `protocol::Mutation` trait.
pub fn inverse_jpg_baseline_mutation(mutation: &JpgBaselineMutation, base: &JpgSnapshot) -> Vec<JpgBaselineMutation> {
    Mutation::inverse(mutation, base)
}
//#endregion 🔖️Apply

//#region 🔖️Axes
/// 🧾️ A `frame`-only diff: every axis on the frame header funnels through this, so the diff a kind
/// produces is visibly the diff its axis calls for and nothing else. A document with no retained
/// frame has no frame axis to move, and produces the empty diff rather than fabricating one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn frame_diff(base: &JpgSnapshot, fields: JpgFrameFieldsDiff) -> JpgDiff {
    if base.frame.is_none() || fields == JpgFrameFieldsDiff::default() {
        return JpgDiff::default();
    }
    JpgDiff { frame: Some(JpgFrameChange::Modify(fields)), ..Default::default() }
}

/// 🔎️ The component carrying `id`, or `None`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn component(base: &JpgSnapshot, id: u8) -> Option<&JpgFrameComponent> {
    base.frame.as_ref().and_then(|frame| frame.components.iter().find(|found| found.id == id))
}

/// 🔎️ The Huffman table at `(class, id)`, or `None`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn huffman<'a>(base: &'a JpgSnapshot, key: &JpgHuffmanTableKey) -> Option<&'a JpgHuffmanTable> {
    base.huffman_tables.iter().find(|table| table.class == key.class && table.id == key.id)
}
//#endregion 🔖️Axes

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &JpgBaselineMutation, base: &JpgSnapshot) -> protocol::MutationOutcome<JpgDiff> {
        protocol::MutationOutcome::new(match this {
            JpgBaselineMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => crate::artifacts::jpg::standards::v_jfif_1_01::subsets::document::schema::diff::diff_set_snapshot(base, snapshot),
            JpgBaselineMutation::SetSofMarker(set_sof_marker::SetSofMarker { marker }) => JpgDiff { sof_marker: (base.sof_marker != *marker).then_some(*marker), ..Default::default() },
            JpgBaselineMutation::SetSamplePrecision(set_sample_precision::SetSamplePrecision { precision }) => {
                let unchanged = base.frame.as_ref().is_some_and(|frame| frame.precision == *precision);
                frame_diff(base, JpgFrameFieldsDiff { precision: (!unchanged).then_some(*precision), ..Default::default() })
            }
            JpgBaselineMutation::SetArithmetic(set_arithmetic::SetArithmetic { arithmetic }) => JpgDiff { arithmetic: (base.arithmetic != *arithmetic).then_some(*arithmetic), ..Default::default() },
            JpgBaselineMutation::InsertHuffmanTable(insert_huffman_table::InsertHuffmanTable { index, table }) => {
                let key = JpgHuffmanTableKey { class: table.class, id: table.id };
                if huffman(base, &key).is_some() {
                    JpgDiff::default()
                } else {
                    JpgDiff { huffman_tables: Some(JpgHuffmanTablesDiff { removed: Vec::new(), modified: Vec::new(), added: vec![JpgHuffmanTableAdded { index: (*index).min(base.huffman_tables.len()), item: table.clone() }] }), ..Default::default() }
                }
            }
            JpgBaselineMutation::RemoveHuffmanTable(remove_huffman_table::RemoveHuffmanTable { key }) => {
                if huffman(base, key).is_none() {
                    JpgDiff::default()
                } else {
                    JpgDiff { huffman_tables: Some(JpgHuffmanTablesDiff { removed: vec![*key], modified: Vec::new(), added: Vec::new() }), ..Default::default() }
                }
            }
            JpgBaselineMutation::InsertFrameComponent(insert_frame_component::InsertFrameComponent { index, component: added }) => {
                if component(base, added.id).is_some() {
                    JpgDiff::default()
                } else {
                    let at = (*index).min(base.frame.as_ref().map_or(0, |frame| frame.components.len()));
                    frame_diff(base, JpgFrameFieldsDiff { components: Some(JpgComponentsDiff { removed: Vec::new(), modified: Vec::new(), added: vec![crate::artifacts::jpg::standards::v_jfif_1_01::subsets::document::schema::diff::JpgComponentAdded { index: at, item: *added }] }), ..Default::default() })
                }
            }
            JpgBaselineMutation::RemoveFrameComponent(remove_frame_component::RemoveFrameComponent { id }) => {
                if component(base, *id).is_none() {
                    JpgDiff::default()
                } else {
                    frame_diff(base, JpgFrameFieldsDiff { components: Some(JpgComponentsDiff { removed: vec![*id], modified: Vec::new(), added: Vec::new() }), ..Default::default() })
                }
            }
            JpgBaselineMutation::SetComponentSampling(set_component_sampling::SetComponentSampling { id, h_sampling, v_sampling }) => match component(base, *id) {
                Some(found) if found.h_sampling == *h_sampling && found.v_sampling == *v_sampling => JpgDiff::default(),
                Some(_) => frame_diff(
                    base,
                    JpgFrameFieldsDiff {
                        components: Some(JpgComponentsDiff { removed: Vec::new(), modified: vec![JpgComponentModified { id: *id, diff: JpgComponentDiff { h_sampling: Some(*h_sampling), v_sampling: Some(*v_sampling), ..Default::default() } }], added: Vec::new() }),
                        ..Default::default()
                    },
                ),
                None => JpgDiff::default(),
            },
        })
    }

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &JpgBaselineMutation, base: &JpgSnapshot) -> Vec<JpgBaselineMutation> {
        vec![match this {
            JpgBaselineMutation::SetSnapshot(_) => JpgBaselineMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
            JpgBaselineMutation::SetSofMarker(_) => JpgBaselineMutation::SetSofMarker(set_sof_marker::SetSofMarker { marker: base.sof_marker }),
            JpgBaselineMutation::SetSamplePrecision(_) => match &base.frame {
                Some(frame) => JpgBaselineMutation::SetSamplePrecision(set_sample_precision::SetSamplePrecision { precision: frame.precision }),
                None => return Vec::new(),
            },
            JpgBaselineMutation::SetArithmetic(_) => JpgBaselineMutation::SetArithmetic(set_arithmetic::SetArithmetic { arithmetic: base.arithmetic }),
            JpgBaselineMutation::InsertHuffmanTable(insert_huffman_table::InsertHuffmanTable { table, .. }) => {
                let key = JpgHuffmanTableKey { class: table.class, id: table.id };
                match huffman(base, &key) {
                    Some(_) => return Vec::new(),
                    None => JpgBaselineMutation::RemoveHuffmanTable(remove_huffman_table::RemoveHuffmanTable { key }),
                }
            }
            // ↩️ The removed table goes back at the position it held, not at the end: `index` exists
            // on the insertion kind precisely so this inverse can name it.
            JpgBaselineMutation::RemoveHuffmanTable(remove_huffman_table::RemoveHuffmanTable { key }) => match base.huffman_tables.iter().position(|table| table.class == key.class && table.id == key.id) {
                Some(at) => JpgBaselineMutation::InsertHuffmanTable(insert_huffman_table::InsertHuffmanTable { index: at, table: base.huffman_tables[at].clone() }),
                None => return Vec::new(),
            },
            JpgBaselineMutation::InsertFrameComponent(insert_frame_component::InsertFrameComponent { component: added, .. }) => match component(base, added.id) {
                Some(_) => return Vec::new(),
                None => JpgBaselineMutation::RemoveFrameComponent(remove_frame_component::RemoveFrameComponent { id: added.id }),
            },
            JpgBaselineMutation::RemoveFrameComponent(remove_frame_component::RemoveFrameComponent { id }) => match base.frame.as_ref().and_then(|frame| frame.components.iter().position(|found| found.id == *id)) {
                Some(at) => JpgBaselineMutation::InsertFrameComponent(insert_frame_component::InsertFrameComponent { index: at, component: base.frame.as_ref().expect("the frame was just read").components[at] }),
                None => return Vec::new(),
            },
            JpgBaselineMutation::SetComponentSampling(set_component_sampling::SetComponentSampling { id, .. }) => match component(base, *id) {
                Some(found) => JpgBaselineMutation::SetComponentSampling(set_component_sampling::SetComponentSampling { id: *id, h_sampling: found.h_sampling, v_sampling: found.v_sampling }),
                None => return Vec::new(),
            },
        }]
    }
//#endregion 🔖️MutationTrait

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::jpg::standards::v_jfif_1_01::subsets::document::schema::snapshot::{JpgFrameHeader, JpgHuffmanClass};
    use crate::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::schema::{check_baseline_conformance, CODE_ARITHMETIC, CODE_COMPONENT_SAMPLING, CODE_HUFFMAN_TABLE_COUNT, CODE_PRECISION, CODE_SOF_MARKER};

    fn table(class: JpgHuffmanClass, id: u8) -> JpgHuffmanTable {
        JpgHuffmanTable { id, class, bits: [0u8; 16], values: vec![id] }
    }

    /// 🧫️ A conforming 16x16 YCbCr baseline snapshot: SOF0, 8-bit, three components at 2x2/1x1/1x1,
    /// two DC and two AC Huffman tables, no arithmetic conditioning.
    fn conforming() -> JpgSnapshot {
        JpgSnapshot {
            frame: Some(JpgFrameHeader {
                precision: BASELINE_PRECISION,
                width: 16,
                height: 16,
                components: vec![
                    JpgFrameComponent { id: 1, h_sampling: 2, v_sampling: 2, quant_table_id: 0 },
                    JpgFrameComponent { id: 2, h_sampling: 1, v_sampling: 1, quant_table_id: 1 },
                    JpgFrameComponent { id: 3, h_sampling: 1, v_sampling: 1, quant_table_id: 1 },
                ],
            }),
            sof_marker: SOF0,
            arithmetic: false,
            huffman_tables: vec![table(JpgHuffmanClass::Dc, 0), table(JpgHuffmanClass::Ac, 0), table(JpgHuffmanClass::Dc, 1), table(JpgHuffmanClass::Ac, 1)],
            ..JpgSnapshot::default()
        }
    }

    fn codes(snapshot: &JpgSnapshot) -> Vec<String> {
        check_baseline_conformance(snapshot).into_iter().map(|finding| finding.code.0.to_string()).collect()
    }

    /// 🏷️ [`KINDS`] against the committed catalog. The framework never parses Rust, so without this
    /// the manifest could keep measuring `mutate-jpg-jfif-1-01-baseline` against a vocabulary this
    /// subset no longer has — which is exactly the gap that left this vocabulary with no catalog at
    /// all until the completeness gate learned to see an unregistered one.
    #[test]
    fn kinds_match_the_committed_catalog() {
        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
        assert!(manifest.contains("jpg-jfif-1-01-baseline-mutate"), "the manifest must declare this subset's OWN capability, not the ✳️any subset's");
    }

    #[test]
    fn kinds_match_enum_variants_in_declaration_order() {
        let variants = [
            JpgBaselineMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: JpgSnapshot::default() }),
            JpgBaselineMutation::SetSofMarker(set_sof_marker::SetSofMarker { marker: SOF0 }),
            JpgBaselineMutation::SetSamplePrecision(set_sample_precision::SetSamplePrecision { precision: BASELINE_PRECISION }),
            JpgBaselineMutation::SetArithmetic(set_arithmetic::SetArithmetic { arithmetic: false }),
            JpgBaselineMutation::InsertHuffmanTable(insert_huffman_table::InsertHuffmanTable { index: 4, table: table(JpgHuffmanClass::Dc, 2) }),
            JpgBaselineMutation::RemoveHuffmanTable(remove_huffman_table::RemoveHuffmanTable { key: JpgHuffmanTableKey { class: JpgHuffmanClass::Dc, id: 0 } }),
            JpgBaselineMutation::InsertFrameComponent(insert_frame_component::InsertFrameComponent { index: 3, component: JpgFrameComponent { id: 4, h_sampling: 1, v_sampling: 1, quant_table_id: 0 } }),
            JpgBaselineMutation::RemoveFrameComponent(remove_frame_component::RemoveFrameComponent { id: 3 }),
            JpgBaselineMutation::SetComponentSampling(set_component_sampling::SetComponentSampling { id: 1, h_sampling: 2, v_sampling: 2 }),
        ];
        assert_eq!(variants.len(), KINDS.len(), "every variant needs exactly one KINDS entry");
        for (variant, kind) in variants.iter().zip(KINDS) {
            let tag = match serde_json::to_value(variant).expect("serialize") {
                serde_json::Value::Object(members) => members.get("mutation").and_then(|value| value.as_str()).expect("tagged enum carries its own discriminant").to_string(),
                other => panic!("a tagged enum must serialize as an object, got {other:?}"),
            };
            assert_eq!(&tag.as_str(), kind, "declaration order must match KINDS");
        }
    }

    /// 🛡️ The point of the whole vocabulary: every kind moves the document across the axis its own
    /// diagnostic reports, and only that axis.
    #[test]
    fn each_kind_moves_exactly_the_axis_its_diagnostic_reports() {
        assert!(codes(&conforming()).is_empty(), "the fixture must start conforming, got {:?}", codes(&conforming()));

        let mut snapshot = conforming();
        apply_jpg_baseline_mutation(&mut snapshot, &JpgBaselineMutation::SetSofMarker(set_sof_marker::SetSofMarker { marker: 0xC2 }));
        assert_eq!(codes(&snapshot), vec![CODE_SOF_MARKER.to_string()]);

        let mut snapshot = conforming();
        apply_jpg_baseline_mutation(&mut snapshot, &JpgBaselineMutation::SetSamplePrecision(set_sample_precision::SetSamplePrecision { precision: 12 }));
        assert_eq!(codes(&snapshot), vec![CODE_PRECISION.to_string()]);

        let mut snapshot = conforming();
        apply_jpg_baseline_mutation(&mut snapshot, &JpgBaselineMutation::SetArithmetic(set_arithmetic::SetArithmetic { arithmetic: true }));
        assert_eq!(codes(&snapshot), vec![CODE_ARITHMETIC.to_string()]);

        let mut snapshot = conforming();
        apply_jpg_baseline_mutation(&mut snapshot, &JpgBaselineMutation::InsertHuffmanTable(insert_huffman_table::InsertHuffmanTable { index: 4, table: table(JpgHuffmanClass::Dc, 2) }));
        assert_eq!(codes(&snapshot), vec![CODE_HUFFMAN_TABLE_COUNT.to_string()]);

        let mut snapshot = conforming();
        apply_jpg_baseline_mutation(&mut snapshot, &JpgBaselineMutation::SetComponentSampling(set_component_sampling::SetComponentSampling { id: 1, h_sampling: 5, v_sampling: 2 }));
        assert_eq!(codes(&snapshot), vec![CODE_COMPONENT_SAMPLING.to_string()]);
    }

    /// 🔢️ The component-count axis needs TWO insertions to cross its line (the check reports more
    /// than four), which is exactly why the kind counts rather than replaces.
    #[test]
    fn a_fifth_frame_component_is_what_crosses_the_component_count_line() {
        let mut snapshot = conforming();
        apply_jpg_baseline_mutation(&mut snapshot, &JpgBaselineMutation::InsertFrameComponent(insert_frame_component::InsertFrameComponent { index: 3, component: JpgFrameComponent { id: 4, h_sampling: 1, v_sampling: 1, quant_table_id: 1 } }));
        assert!(codes(&snapshot).is_empty(), "four components is still inside the line");
        apply_jpg_baseline_mutation(&mut snapshot, &JpgBaselineMutation::InsertFrameComponent(insert_frame_component::InsertFrameComponent { index: 4, component: JpgFrameComponent { id: 5, h_sampling: 1, v_sampling: 1, quant_table_id: 1 } }));
        assert_eq!(codes(&snapshot), vec![CODE_COMPONENT_SAMPLING.to_string()]);
    }

    /// ↩️ `apply(inverse(m), apply(m, base))` must land back on `base` for every kind.
    #[test]
    fn every_kind_is_inverted_by_its_own_inverse() {
        let cases = [
            JpgBaselineMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: JpgSnapshot::default() }),
            JpgBaselineMutation::SetSofMarker(set_sof_marker::SetSofMarker { marker: 0xC1 }),
            JpgBaselineMutation::SetSamplePrecision(set_sample_precision::SetSamplePrecision { precision: 12 }),
            JpgBaselineMutation::SetArithmetic(set_arithmetic::SetArithmetic { arithmetic: true }),
            JpgBaselineMutation::InsertHuffmanTable(insert_huffman_table::InsertHuffmanTable { index: 4, table: table(JpgHuffmanClass::Ac, 3) }),
            JpgBaselineMutation::RemoveHuffmanTable(remove_huffman_table::RemoveHuffmanTable { key: JpgHuffmanTableKey { class: JpgHuffmanClass::Dc, id: 1 } }),
            JpgBaselineMutation::InsertFrameComponent(insert_frame_component::InsertFrameComponent { index: 3, component: JpgFrameComponent { id: 9, h_sampling: 1, v_sampling: 1, quant_table_id: 0 } }),
            JpgBaselineMutation::RemoveFrameComponent(remove_frame_component::RemoveFrameComponent { id: 2 }),
            JpgBaselineMutation::SetComponentSampling(set_component_sampling::SetComponentSampling { id: 1, h_sampling: 4, v_sampling: 4 }),
        ];
        for mutation in cases {
            let base = conforming();
            let mut snapshot = base.clone();
            apply_jpg_baseline_mutation(&mut snapshot, &mutation);
            for undo in inverse_jpg_baseline_mutation(&mutation, &base) {
                apply_jpg_baseline_mutation(&mut snapshot, &undo);
            }
            assert_eq!(snapshot.sof_marker, base.sof_marker, "inverse of {mutation:?} left sof_marker moved");
            assert_eq!(snapshot.arithmetic, base.arithmetic, "inverse of {mutation:?} left arithmetic moved");
            assert_eq!(snapshot.frame, base.frame, "inverse of {mutation:?} left the frame moved");
            assert_eq!(snapshot.huffman_tables, base.huffman_tables, "inverse of {mutation:?} left the Huffman tables moved");
        }
    }

    /// 🚫️ An addition whose target already exists, and a removal whose target does not, are both
    /// no-ops — and must produce the EMPTY diff rather than a change nothing made.
    #[test]
    fn an_insertion_that_finds_its_target_present_produces_an_empty_diff() {
        let base = conforming();
        let already = JpgBaselineMutation::InsertHuffmanTable(insert_huffman_table::InsertHuffmanTable { index: 0, table: table(JpgHuffmanClass::Dc, 0) });
        assert_eq!(<JpgBaselineMutation as Mutation<JpgSnapshot>>::diff(&already, &base).diff(), &JpgDiff::default());
        assert_eq!(inverse_jpg_baseline_mutation(&already, &base), Vec::new());

        let absent = JpgBaselineMutation::RemoveFrameComponent(remove_frame_component::RemoveFrameComponent { id: 42 });
        assert_eq!(<JpgBaselineMutation as Mutation<JpgSnapshot>>::diff(&absent, &base).diff(), &JpgDiff::default());
        assert_eq!(inverse_jpg_baseline_mutation(&absent, &base), Vec::new());
    }
}
//#endregion 🧪️Tests
