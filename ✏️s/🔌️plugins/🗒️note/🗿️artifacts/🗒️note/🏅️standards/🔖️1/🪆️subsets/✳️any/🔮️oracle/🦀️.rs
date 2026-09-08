//! 🔮️ Mutation oracle for `s.note.note@1/✳️any` — the reader half of the THREE registered
//! third-party carriers (`dxf-crate-note-ink-reader`, `quick-xml-note-drawing-reader`,
//! `lopdf-note-text-reader`, all declared in `./🔣️.json`). This subset's own domain
//! vocabulary (33 kinds) is note-native, not DXF/SVG/PDF-native — unlike `s.stdio.dxf@r12/✳️any`,
//! whose vocabulary IS the format — so this module's job is to READ what `NoteIntoDxf`/`NoteIntoSvg`/
//! `NoteIntoPdf` (`../🚪️io/📤️export/🧵️serializers/🗿️artifacts/**/🦀️.rs`) actually wrote,
//! independently of this subset's own codec, exactly the role `three-carrier-reader` plays for
//! `s.stdio.semio@v1/✳️mesh`.
//!
//! Every function below DELEGATES to an already-registered, already-oracle-qualified projector this
//! crate carries for OTHER subsets, rather than re-implementing a DXF/XML/PDF reader a third time:
//! `semio_s_artifact_stdio_dxf::standards::v_r12::subsets::any::project_dxf_r12` is the same qualifying
//! `dxf` 0.6 reader `s.stdio.dxf@r12/✳️any` registers under `dxf-crate-r12-mutate`; the DXF bytes
//! `NoteIntoDxf` writes are ordinary DXF R12 (only ever containing `LINE` entities), so the reader
//! that already qualifies against the full grammar reads this narrower subset of it for free.
//! `crate::markup::live::{parse_markup, project_markup}` is the `quick-xml` 0.42 tree reader/semantic
//! projector this crate's `📰markup` family module already carries for the `🎨️svg` subsets — SVG is
//! XML, so nothing note-specific is needed to read it. `crate::document::project_pdf` is the `lopdf`
//! 0.44 reader already registered under `pdf-edit`/`pdf-parse`.
//!
//! WHAT THE PROJECTIONS DO AND DO NOT WITNESS, per `./🔣️.json`'s `mutationManifests`
//! `carriers`/`oracleRequirements`: `project_note_dxf` sees only `LINE` entities built from an Ink
//! block's raw `points` (no block `x`/`y`/`rotation`, no visibility filter, no width — read straight
//! from `NoteIntoDxf::serialize`'s body). `project_note_svg` sees every visible block's `<g
//! transform="matrix(…)">` (position+rotation) and, per kind, a `<path>` (Ink: geometry+stroke-width;
//! Table/Math/Group/image-fallback: an outline rectangle keyed to width/height only), a `<text>`
//! (Text: joined paragraph content; `font_size` is wired to `y`, never to a size attribute), or an
//! `<image>` (real bytes when the referenced asset exists). `project_note_pdf` sees the title and
//! every Text block's content flattened onto one page's text stream — no position, no other kind.
//!
//! @see `./🔣️.json` — the three oracle registrations and the per-mutation carrier list.
//! @see ../🚪️io/📤️export/🧵️serializers/🗿️artifacts/**/✳️any/🦀️.rs — what is projected.

use semio_repo_test_host::Json;

//#region 🔖️Dispatch
/// 🖊️ Independent semantic projection of the `LINE` entities `NoteIntoDxf` wrote, via the same
/// qualifying `dxf` reader `s.stdio.dxf@r12/✳️any` registers.
#[cfg(feature = "oracles")]
pub fn project_note_dxf(bytes: &[u8]) -> Result<Json, String> {
    semio_s_artifact_stdio_dxf::standards::v_r12::subsets::any::project_dxf_r12(bytes)
}

/// 🎨️ Independent semantic projection of the SVG XML `NoteIntoSvg` wrote (via the real semio/drawing
/// bridge), using this crate's shared `quick-xml` tree reader/projector — SVG is XML, so nothing
/// note-specific is needed to read it.
#[cfg(feature = "oracles")]
pub fn project_note_svg(bytes: &[u8]) -> Result<Json, String> {
    let doc = crate::markup::live::parse_markup(bytes)?;
    Ok(crate::markup::live::project_markup(&doc))
}

/// 📄️ Independent semantic projection (media box + `Tj` text operands per page) of the PDF
/// `NoteIntoPdf` wrote, via this crate's shared `lopdf` reader.
#[cfg(feature = "oracles")]
pub fn project_note_pdf(bytes: &[u8]) -> Result<Json, String> {
    crate::document::project_pdf(bytes)
}

/// 🚫️ Without the `oracles` feature no reference implementation is linked at all.
#[cfg(not(feature = "oracles"))]
pub fn project_note_dxf(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
#[cfg(not(feature = "oracles"))]
pub fn project_note_svg(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
#[cfg(not(feature = "oracles"))]
pub fn project_note_pdf(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🧪️SmokeTests
/// 🧪️ Runtime proof (not merely compilation) that each delegated projector actually reads bytes
/// shaped the way `NoteIntoDxf`/`NoteIntoSvg`/`NoteIntoPdf` produce them. This crate carries no
/// dependency on `semio_s_plugin_stdio` (the production codec that plugin belongs to — an oracle
/// crate must never link the subject it is evidence for), so each fixture below is built from the
/// FORMAT'S OWN minimal grammar rather than by calling note's real serializer.
#[cfg(all(test, feature = "oracles"))]
#[path = "🧪️tests/🔬️smoke/🦀️.rs"]
mod smoke_tests;
//#endregion 🧪️SmokeTests
