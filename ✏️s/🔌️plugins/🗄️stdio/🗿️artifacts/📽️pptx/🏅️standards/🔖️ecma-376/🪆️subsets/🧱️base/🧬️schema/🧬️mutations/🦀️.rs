//! 🧬️ PptxMutation — document mutation dispatch. Every variant's `diff()` is handcrafted (never
//! apply-and-capture) and every variant's `inverse()` is handcrafted, index-aware.

use crate::schema::diff::{diff_insert_shape, diff_insert_slide, diff_move_slide, diff_remove_shape, diff_remove_slide, diff_set_shape_position, diff_set_shape_text, diff_set_snapshot, PptxDiff};
use crate::schema::snapshot::{PptxParagraph, PptxShape, PptxSlide, PptxSnapshotRecord, PptxTransform};
use crate::PptxSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, OpText};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.pptx`. Addresses `presentation.slides` by index
/// (slide order matters -- see `MoveSlide`) and, within a slide, `shapes` by
/// `(slide_index, shape_index)` -- a flat two-level address is sufficient since PresentationML
/// slides don't nest arbitrarily (grouped shapes fall back to `PptxShape::Other`, see the
/// snapshot module's doc comment).
/// 🧪️ F6 CONFIRMED: `#[derive(dsl::DslOps)]` on this enum fails to compile — real `cargo check`
/// error: `the trait bound PptxSnapshot: DslField is not satisfied` at `SetSnapshot{snapshot}`
/// (`PptxSnapshot` embeds `PptxShape`, a data-carrying enum, and the generic `IndexedTripleDiff`/
/// `NamedTripleDiff` collection engine `PptxPresentation`/`OpcPackage` route through — see the diff
/// file's `HandcraftedDiffCodec` doc comment for the full three-reason citation), plus a SECOND,
/// independent hit at `InsertShape{shape: PptxShape}`/`InsertSlide{slide: PptxSlide}` (`PptxShape`/
/// `PptxSlide` carry the same enum-shaped payload DIRECTLY as a variant field, mirroring
/// `SvgMutation::InsertElement`'s `node: XmlNode` blocker). `OpText`/`OpBinary` hand-rolled below,
/// reusing `PptxDiff`'s `pub(crate)` grammar primitives.
//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "➕insert-slide/🦀️.rs"]
pub mod insert_slide;
#[path = "➖remove-slide/🦀️.rs"]
pub mod remove_slide;
#[path = "🔀move-slide/🦀️.rs"]
pub mod move_slide;
#[path = "🔷insert-shape/🦀️.rs"]
pub mod insert_shape;
#[path = "🔶remove-shape/🦀️.rs"]
pub mod remove_shape;
#[path = "✍️set-shape-text/🦀️.rs"]
pub mod set_shape_text;
#[path = "📐set-shape-position/🦀️.rs"]
pub mod set_shape_position;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = PptxSnapshot, diff = PptxDiff, schema = "PptxMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum PptxMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    /// ➕️ Inserts `🎞️slide` at `index` (FINAL state).
    InsertSlide(insert_slide::InsertSlide),
    /// ➖️ Removes the slide at `index` (BASE-state index).
    RemoveSlide(remove_slide::RemoveSlide),
    /// 🔀️ Moves the slide at BASE-state index `from` to FINAL-state index `to`.
    MoveSlide(move_slide::MoveSlide),
    /// ➕️ Inserts `shape` at `shape_index` (FINAL state) on the slide at `slide_index`.
    InsertShape(insert_shape::InsertShape),
    /// ➖️ Removes the shape at `shape_index` (BASE-state index) on the slide at `slide_index`.
    RemoveShape(remove_shape::RemoveShape),
    /// ✍️ Replaces a `TextBox`/`Placeholder` shape's `text_frame` (no-op on `Picture`/`Other`).
    SetShapeText(set_shape_text::SetShapeText),
    /// 📐️ Sets a shape's `position` (no-op on `Other`, which has none).
    SetShapePosition(set_shape_position::SetShapePosition),
}
//#endregion 🔖️Mutations

//#region 🔖️Kinds
/// 🏷️ The kebab-case spelling of every `PptxMutation` variant, in the same order the enum
/// declares them — this repository's mutation oracle registrations never parse this enum;
/// `kinds_matches_enum_variants_and_manifest` below is what keeps the two declarations honest
/// against each other.
pub const KINDS: &[&str] = &["set-snapshot", "insert-slide", "remove-slide", "move-slide", "insert-shape", "remove-shape", "set-shape-text", "set-shape-position"];

/// 🏷️ The `KINDS` spelling of one mutation's own variant. An exhaustive match (no wildcard arm),
/// so a new variant that forgets its kebab spelling here fails to compile rather than failing
/// silently.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn kind_of(mutation: &PptxMutation) -> &'static str {
    match mutation {
        PptxMutation::SetSnapshot(_) => "set-snapshot",
        PptxMutation::InsertSlide(_) => "insert-slide",
        PptxMutation::RemoveSlide(_) => "remove-slide",
        PptxMutation::MoveSlide(_) => "move-slide",
        PptxMutation::InsertShape(_) => "insert-shape",
        PptxMutation::RemoveShape(_) => "remove-shape",
        PptxMutation::SetShapeText(_) => "set-shape-text",
        PptxMutation::SetShapePosition(_) => "set-shape-position",
    }
}
//#endregion 🔖️Kinds

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` -- the diff is the single semantics source, never a separate imperative
/// apply path (apply-and-capture is banned).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_pptx_mutation(snapshot: &mut PptxSnapshot, mutation: &PptxMutation) -> protocol::MutationOutcome<PptxDiff> {
    let outcome = Mutation::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn slide_at(base: &PptxSnapshot, index: usize) -> Option<&PptxSlide> {
    base.presentation.slides.get(index)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn shape_at(base: &PptxSnapshot, slide_index: usize, shape_index: usize) -> Option<&PptxShape> {
    base.presentation.slides.get(slide_index)?.shapes.get(shape_index)
}
//#endregion 🔖️Helpers

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &PptxMutation, base: &PptxSnapshot) -> protocol::MutationOutcome<PptxDiff> {
    protocol::MutationOutcome::new(match this {
        PptxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        PptxMutation::InsertSlide(insert_slide::InsertSlide { index, slide }) => diff_insert_slide(*index, slide.clone()),
        PptxMutation::RemoveSlide(remove_slide::RemoveSlide { index }) => diff_remove_slide(*index),
        PptxMutation::MoveSlide(move_slide::MoveSlide { from, to }) => diff_move_slide(&base.presentation, *from, *to),
        PptxMutation::InsertShape(insert_shape::InsertShape { slide_index, shape_index, shape }) => diff_insert_shape(*slide_index, *shape_index, shape.clone()),
        PptxMutation::RemoveShape(remove_shape::RemoveShape { slide_index, shape_index }) => diff_remove_shape(*slide_index, *shape_index),
        PptxMutation::SetShapeText(set_shape_text::SetShapeText { slide_index, shape_index, text_frame }) => diff_set_shape_text(&base.presentation, *slide_index, *shape_index, text_frame),
        PptxMutation::SetShapePosition(set_shape_position::SetShapePosition { slide_index, shape_index, position }) => diff_set_shape_position(&base.presentation, *slide_index, *shape_index, *position),
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
// 🧭️ `NoMutation` was dropped; the four "restore the pre-state" branches that fell back to it (no
// slide/shape/text/position to restore) now return the EMPTY inverse (`Vec::new()`) instead of a
// synthetic no-op mutation, the same replacement tiff's own migration made for its structural axes.
pub(crate) fn agg_inverse(this: &PptxMutation, base: &PptxSnapshot) -> Vec<PptxMutation> {
    match this {
        PptxMutation::SetSnapshot(_) => vec![PptxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        PptxMutation::InsertSlide(insert_slide::InsertSlide { index, .. }) => vec![PptxMutation::RemoveSlide(remove_slide::RemoveSlide { index: *index })],
        PptxMutation::RemoveSlide(remove_slide::RemoveSlide { index }) => match slide_at(base, *index) {
            Some(slide) => vec![PptxMutation::InsertSlide(insert_slide::InsertSlide { index: *index, slide: slide.clone() })],
            None => Vec::new(),
        },
        PptxMutation::MoveSlide(move_slide::MoveSlide { from, to }) => {
            // 🧭️ After moving `from -> to`, the slide ends up at `min(to, len-1)` (per
            // `apply_indexed`'s own remove-then-insert semantics: one item shorter after the
            // removal, then inserted at `min(to, that_shorter_len)`) -- moving it back to
            // `from` restores the original order exactly.
            let len = base.presentation.slides.len();
            let final_pos = (*to).min(len.saturating_sub(1));
            vec![PptxMutation::MoveSlide(move_slide::MoveSlide { from: final_pos, to: *from })]
        }
        PptxMutation::InsertShape(insert_shape::InsertShape { slide_index, shape_index, .. }) => vec![PptxMutation::RemoveShape(remove_shape::RemoveShape { slide_index: *slide_index, shape_index: *shape_index })],
        PptxMutation::RemoveShape(remove_shape::RemoveShape { slide_index, shape_index }) => match shape_at(base, *slide_index, *shape_index) {
            Some(shape) => vec![PptxMutation::InsertShape(insert_shape::InsertShape { slide_index: *slide_index, shape_index: *shape_index, shape: shape.clone() })],
            None => Vec::new(),
        },
        PptxMutation::SetShapeText(set_shape_text::SetShapeText { slide_index, shape_index, .. }) => {
            let old = shape_at(base, *slide_index, *shape_index).and_then(|s| match s {
                PptxShape::TextBox { text_frame, .. } | PptxShape::Placeholder { text_frame, .. } => Some(text_frame.clone()),
                _ => None,
            });
            match old {
                Some(text_frame) => vec![PptxMutation::SetShapeText(set_shape_text::SetShapeText { slide_index: *slide_index, shape_index: *shape_index, text_frame })],
                None => Vec::new(),
            }
        }
        PptxMutation::SetShapePosition(set_shape_position::SetShapePosition { slide_index, shape_index, .. }) => {
            let old = shape_at(base, *slide_index, *shape_index).and_then(|s| match s {
                PptxShape::TextBox { position, .. } | PptxShape::Picture { position, .. } | PptxShape::Placeholder { position, .. } => Some(*position),
                PptxShape::Other { .. } => None,
            });
            match old {
                Some(position) => vec![PptxMutation::SetShapePosition(set_shape_position::SetShapePosition { slide_index: *slide_index, shape_index: *shape_index, position })],
                None => Vec::new(),
            }
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ F6: **hand-rolled** `OpText`/`OpBinary` for `PptxMutation` (`#[derive(dsl::DslOps)]`
/// confirmed rejected above) — reuses `PptxDiff`'s `pub(crate)` grammar primitives
/// (`enc_shape`/`enc_slide`/`enc_transform`/`split_top_level`/`encode_option`/...) rather than
/// duplicating them a second time in this file (same intra-artifact reuse `SvgMutation` established
/// for `SvgDiff`'s primitives). Grammar: `keyword arg=value ...` (space-separated, same shape the
/// derive's own handcrafted-wrapper convention uses), one match arm per variant.
//#region 🔖️SnapshotCodec
/// 🌳 Full `OpcPackage` (used only by `SetSnapshot`'s `snapshot` payload, never by `PptxDiff` --
/// diffs only ever carry the sparse `PptxOpcDiff`): `[parts,defaults,overrides,relationships]`.
/// `relationships` (a `HashMap`) is sorted by owner key first for deterministic `encode_op` output
/// (`OpBinary`'s own LAW: "encoding is deterministic -- byte-identical output for equal operations").
/// 🌳 Full `PptxSnapshot`: `[schema,opc,xml-parts,slides]` -- `presentation` collapses to its own single
/// field (`slides: Vec<PptxSlide>`), same convention `enc_slide`/`enc_paragraph` use.
//#endregion 🔖️SnapshotCodec

#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct PptxMutationRecord {
    kind: String,
    value: dsl::DslValue,
    snapshot: Option<PptxSnapshotRecord>,
}

impl OpText for PptxMutation {
    fn print_op(&self) -> String {
        let record = match self {
            PptxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => PptxMutationRecord { kind: "setSnapshot".into(), value: dsl::DslValue::Null, snapshot: Some(PptxSnapshotRecord::from_snapshot(snapshot).expect("serializable logical pptx snapshot")) },
            mutation => PptxMutationRecord { kind: "mutation".into(), value: dsl::ToValue::to_value(mutation), snapshot: None },
        };
        dsl::print(&record.__dsl_to_record(), &PptxMutationRecord::__dsl_spec(), dsl::JoinMode::Inline)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let record = dsl::parse(line, &PptxMutationRecord::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits { max_bytes: 64 * 1024 * 1024, ..dsl::Limits::default() }, mode: dsl::SourceMode::Inline })?;
        let model = PptxMutationRecord::__dsl_from_record(&record)?;
        match (model.kind.as_str(), model.snapshot) {
            ("setSnapshot", Some(snapshot)) => snapshot.into_snapshot().map(|snapshot| PptxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot })).map_err(|error| store::TextError::new(error, dsl::TextSpan::at(1, 1))),
            ("mutation", None) => dsl::FromValue::from_value(model.value).map_err(|error| store::TextError::new(error.to_string(), dsl::TextSpan::at(1, 1))),
            _ => Err(store::TextError::new("PPTX mutation record kind/payload mismatch", dsl::TextSpan::at(1, 1))),
        }
    }
}

//#region 🔖️OpBinaryCodec
/// 🧪️ FG-wave: real recursive binary primitives backing the upgraded `OpBinary` impl below --
/// mirrors `📜️docx/…/🧬️mutations/🦀️.rs`'s own `enc_docx_snapshot_bin`/`enc_opc_package_bin`
/// shape, reusing `store::pack_rt::write_varint_u64`/`store::ByteReader` plus `PptxDiff`'s own
/// `write_str_lp`/`read_str_lp`/`enc_shape_bin`/`dec_shape_bin`/`enc_slide_bin`/`dec_slide_bin`/
/// `enc_transform_bin`/`dec_transform_bin`/`enc_part_bin`/`dec_part_bin`/`enc_rel_bin`/`dec_rel_bin`
/// (`../🔺️diff/🦀️.rs`, `pub(crate)` to this artifact).
///
/// 🌱 Full (non-diff) `OpcPackage`/`PptxSnapshot` binary codecs -- only `SetSnapshot`'s
/// whole-payload encoding needs these, mirroring this file's own `enc_opc_package`/`enc_snapshot`
/// text forms above. Relationship owners sorted for a deterministic encoding, same `HashMap`
/// -iteration-order caveat those text forms document.
/// 🌳 Full `PptxSnapshot`: `[schema,opc,xml-parts,slides]`, mirroring `enc_snapshot`'s text form above.
//#endregion 🔖️OpBinaryCodec
/// 🧪️ FG-wave: REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape --
/// upgraded from F1's `print_op().into_bytes()` text-as-binary shortcut. `tag` is the
/// `PptxMutation` variant ordinal, in the same 0-8 order `print_pptx_mutation`'s own keyword
/// match uses.
impl OpBinary for PptxMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let value = dsl::ToValue::to_value(self);
        Ok(store::pack_rt::encode_wire_value(&value))
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "pptx mutation", offset: 0, detail: error.to_string() })?;
        dsl::FromValue::from_value(value).map_err(|error| protocol::ProtocolError::Malformed { what: "pptx mutation", offset: 0, detail: error.to_string() })
    }
}
//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🧪️ FG-wave: representative `PptxMutation` values -- one per variant -- the single source of
/// truth reused by this file's own `mutation_diff_law`/`inverse_law`/`op_text_binary_roundtrip_law`
/// tests AND by `⚙️engine/🦀️.rs`'s `ops_grammar_conformance_law`/`protocol_walk_law`
/// conformance tests, same shape `📜️docx/…/🧬️mutations/🦀️.rs`'s own
/// `demo_mutation_cases()` establishes.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_fixture() -> PptxSnapshot {
    crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_pptx(crate::schema::snapshot::PptxPresentation {
        slides: vec![
            PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("first")], position: PptxTransform { x: 0, y: 0, cx: 100, cy: 100 } }] },
            PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("second")], position: PptxTransform::default() }] },
        ],
    })
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<PptxMutation> {
    vec![
        PptxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: demo_fixture() }),
        PptxMutation::InsertSlide(insert_slide::InsertSlide { index: 1, slide: PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("x")], position: PptxTransform::default() }] } }),
        PptxMutation::RemoveSlide(remove_slide::RemoveSlide { index: 0 }),
        PptxMutation::MoveSlide(move_slide::MoveSlide { from: 0, to: 1 }),
        PptxMutation::InsertShape(insert_shape::InsertShape { slide_index: 0, shape_index: 1, shape: PptxShape::Picture { blip_rel_id: "rId7".into(), position: PptxTransform { x: 1, y: 2, cx: 3, cy: 4 } } }),
        PptxMutation::RemoveShape(remove_shape::RemoveShape { slide_index: 0, shape_index: 0 }),
        PptxMutation::SetShapeText(set_shape_text::SetShapeText { slide_index: 0, shape_index: 0, text_frame: vec![PptxParagraph::text("z")] }),
        PptxMutation::SetShapePosition(set_shape_position::SetShapePosition { slide_index: 0, shape_index: 0, position: PptxTransform { x: 5, y: 6, cx: 7, cy: 8 } }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `🦀️.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧪️FixtureTests
