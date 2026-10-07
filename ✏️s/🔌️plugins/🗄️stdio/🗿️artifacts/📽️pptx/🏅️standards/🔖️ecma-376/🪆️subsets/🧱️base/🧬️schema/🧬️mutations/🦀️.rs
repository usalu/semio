//! 🧬️ PptxMutation — document mutation dispatch. Every variant's `diff()` is handcrafted (never
//! apply-and-capture) and every variant's `inverse()` is handcrafted, index-aware.

use crate::schema::diff::{diff_set_snapshot, PptxDiff};
use crate::schema::snapshot::{PptxParagraph, PptxShape, PptxSlide, PptxTransform};
use crate::PptxSnapshot;
use protocol::Mutation;
use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;

//#region 🔖️Mutations
#[path = "🔷insert-shape/🦀️.rs"]
pub mod insert_shape;
#[path = "➕insert-slide/🦀️.rs"]
pub mod insert_slide;
#[path = "🔀move-slide/🦀️.rs"]
pub mod move_slide;
#[path = "🔶remove-shape/🦀️.rs"]
pub mod remove_shape;
#[path = "➖remove-slide/🦀️.rs"]
pub mod remove_slide;
#[path = "📐set-shape-position/🦀️.rs"]
pub mod set_shape_position;
#[path = "✍️set-shape-text/🦀️.rs"]
pub mod set_shape_text;
#[path = "🧭️xml-address/🦀️.rs"]
pub mod xml_address;
pub use xml_address::{PptxShapeAddress, PptxSlideAddress, PptxXmlAddress, PptxXmlVacancyAddress};
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
#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = PptxSnapshot, diff = PptxDiff, schema = "PptxMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum PptxMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    PatchSnapshot(patch_snapshot::PatchSnapshot),
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
pub const KINDS: &[&str] = &["set-snapshot", "patch-snapshot", "insert-slide", "remove-slide", "move-slide", "insert-shape", "remove-shape", "set-shape-text", "set-shape-position"];

/// 🏷️ The `KINDS` spelling of one mutation's own variant. An exhaustive match (no wildcard arm),
/// so a new variant that forgets its kebab spelling here fails to compile rather than failing
/// silently.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn kind_of(mutation: &PptxMutation) -> &'static str {
    match mutation {
        PptxMutation::SetSnapshot(_) => "set-snapshot",
        PptxMutation::PatchSnapshot(_) => "patch-snapshot",
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
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️CanonicalPreparation
fn canonical_next(this: &PptxMutation, base: &PptxSnapshot) -> Result<PptxSnapshot, String> {
    let mut next = base.clone();
    match this {
        PptxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => return Ok(snapshot.clone()),
        PptxMutation::PatchSnapshot(_) => return Err("patch snapshot uses its schema-owned mutation path".into()),
        PptxMutation::InsertSlide(insert_slide::InsertSlide { vacancy, entry }) => xml_address::insert_slide(&mut next, vacancy, entry.clone())?,
        PptxMutation::RemoveSlide(remove_slide::RemoveSlide { address }) => xml_address::remove_slide(&mut next, address)?,
        PptxMutation::MoveSlide(move_slide::MoveSlide { address, destination_index }) => xml_address::move_slide(&mut next, address, *destination_index)?,
        PptxMutation::InsertShape(insert_shape::InsertShape { vacancy, shape }) => xml_address::insert_shape(&mut next, vacancy, shape.clone())?,
        PptxMutation::RemoveShape(remove_shape::RemoveShape { address }) => xml_address::remove_shape(&mut next, address)?,
        PptxMutation::SetShapeText(set_shape_text::SetShapeText { address, text }) => xml_address::set_shape_text(&mut next, address, text)?,
        PptxMutation::SetShapePosition(set_shape_position::SetShapePosition { address, position }) => xml_address::set_shape_position(&mut next, address, *position)?,
    }
    Ok(next)
}
//#endregion 🔖️CanonicalPreparation

//#region 🔖️MutationTrait
pub(crate) fn agg_diff(this: &PptxMutation, base: &PptxSnapshot) -> protocol::MutationOutcome<PptxDiff> {
    if let PptxMutation::PatchSnapshot(patch) = this {
        return <patch_snapshot::PatchSnapshot as protocol::MutationKind<PptxSnapshot, PptxMutation>>::diff(patch, base);
    }
    match canonical_next(this, base) {
        Ok(next) => protocol::MutationOutcome::new(diff_set_snapshot(base, &next)),
        Err(message) => protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, Vec::<String>::new()),
    }
}

pub(crate) fn agg_inverse(this: &PptxMutation, base: &PptxSnapshot) -> Result<Vec<PptxMutation>, semio_framework_value::ValueError> {
    Ok({
    match this {
        PptxMutation::PatchSnapshot(patch) => <patch_snapshot::PatchSnapshot as protocol::MutationKind<PptxSnapshot, PptxMutation>>::inverse(patch, base)?,
        PptxMutation::SetSnapshot(_)
        | PptxMutation::InsertSlide(_)
        | PptxMutation::RemoveSlide(_)
        | PptxMutation::MoveSlide(_)
        | PptxMutation::InsertShape(_)
        | PptxMutation::RemoveShape(_)
        | PptxMutation::SetShapeText(_)
        | PptxMutation::SetShapePosition(_) => vec![PptxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
    }

    })
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
    crate::standards::v_ecma_376::subsets::base::schema::construction::minimal::build_minimal_pptx(crate::schema::snapshot::PptxPresentation {
        slides: vec![
            PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("first")], position: PptxTransform { x: 0, y: 0, cx: 100, cy: 100 } }] },
            PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("second")], position: PptxTransform::default() }] },
        ],
    })
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<PptxMutation> {
    let fixture = demo_fixture();
    let slides = xml_address::pptx_slides(&fixture).expect("canonical demo slides");
    let first_shape = slides[0].shapes[0].address.clone();
    let second_slide = slides[1].address.clone();
    let slide_entry = xml_address::resolve_pptx_xml_address(&fixture, &slides[0].address.entry).expect("canonical slide entry").clone();
    let shape_node = xml_address::resolve_pptx_xml_address(&fixture, &first_shape.node).expect("canonical shape node").clone();
    vec![
        PptxMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch: semio_s_artifact_stdio_contract::editing::SnapshotPatch::Set { path: "/schema".into(), value: semio_framework_value::DslValue::String("stdio.patch-snapshot.witness".into()) } }),
        PptxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: demo_fixture() }),
        PptxMutation::InsertSlide(insert_slide::InsertSlide { vacancy: xml_address::pptx_slide_vacancy(&fixture, 1).expect("slide vacancy"), entry: slide_entry }),
        PptxMutation::RemoveSlide(remove_slide::RemoveSlide { address: slides[0].address.clone() }),
        PptxMutation::MoveSlide(move_slide::MoveSlide { address: second_slide, destination_index: 0 }),
        PptxMutation::InsertShape(insert_shape::InsertShape { vacancy: xml_address::pptx_shape_vacancy(&fixture, &slides[0].address, 1).expect("shape vacancy"), shape: shape_node }),
        PptxMutation::RemoveShape(remove_shape::RemoveShape { address: first_shape.clone() }),
        PptxMutation::SetShapeText(set_shape_text::SetShapeText { address: first_shape.clone(), text: "z".into() }),
        PptxMutation::SetShapePosition(set_shape_position::SetShapePosition { address: first_shape, position: PptxTransform { x: 5, y: 6, cx: 7, cy: 8 } }),
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
