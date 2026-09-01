//! 🔮️ Mutation oracle for `s.note.note@1/✳️any` — the reader half of the THREE registered
//! third-party carriers (`dxf-crate-note-ink-reader`, `quick-xml-note-drawing-reader`,
//! `lopdf-note-text-reader`, all declared in `../🧪️oracle/🔣️.json`). This subset's own domain
//! vocabulary (33 kinds) is note-native, not DXF/SVG/PDF-native — unlike `s.stdio.dxf@r12/✳️any`,
//! whose vocabulary IS the format — so this module's job is to READ what `NoteIntoDxf`/`NoteIntoSvg`/
//! `NoteIntoPdf` (`../🚪️io/📤️export/🧵️serializers/🗿️artifacts/**/🦀️component.rs`) actually wrote,
//! independently of this subset's own codec, exactly the role `three-carrier-reader` plays for
//! `s.stdio.semio@v1/✳️mesh`.
//!
//! Every function below DELEGATES to an already-registered, already-oracle-qualified projector this
//! crate carries for OTHER subsets, rather than re-implementing a DXF/XML/PDF reader a third time:
//! `crate::artifacts::dxf::standards::v_r12::subsets::any::project_dxf_r12` is the same qualifying
//! `dxf` 0.6 reader `s.stdio.dxf@r12/✳️any` registers under `dxf-crate-r12-mutate`; the DXF bytes
//! `NoteIntoDxf` writes are ordinary DXF R12 (only ever containing `LINE` entities), so the reader
//! that already qualifies against the full grammar reads this narrower subset of it for free.
//! `crate::markup::live::{parse_markup, project_markup}` is the `quick-xml` 0.42 tree reader/semantic
//! projector this crate's `📰markup` family module already carries for the `🎨️svg` subsets — SVG is
//! XML, so nothing note-specific is needed to read it. `crate::document::project_pdf` is the `lopdf`
//! 0.44 reader already registered under `pdf-edit`/`pdf-parse`.
//!
//! WHAT THE PROJECTIONS DO AND DO NOT WITNESS, per `../🧪️oracle/🔣️.json`'s `mutationManifests`
//! `carriers`/`oracleRequirements`: `project_note_dxf` sees only `LINE` entities built from an Ink
//! block's raw `points` (no block `x`/`y`/`rotation`, no visibility filter, no width — read straight
//! from `NoteIntoDxf::serialize`'s body). `project_note_svg` sees every visible block's `<g
//! transform="matrix(…)">` (position+rotation) and, per kind, a `<path>` (Ink: geometry+stroke-width;
//! Table/Math/Group/image-fallback: an outline rectangle keyed to width/height only), a `<text>`
//! (Text: joined paragraph content; `font_size` is wired to `y`, never to a size attribute), or an
//! `<image>` (real bytes when the referenced asset exists). `project_note_pdf` sees the title and
//! every Text block's content flattened onto one page's text stream — no position, no other kind.
//!
//! @see `../🧪️oracle/🔣️.json` — the three oracle registrations and the per-mutation carrier list.
//! @see ../🚪️io/📤️export/🧵️serializers/🗿️artifacts/**/✳️any/🦀️component.rs — what is projected.

use semio_repo_test_host::Json;

//#region 🔖️Dispatch
/// 🖊️ Independent semantic projection of the `LINE` entities `NoteIntoDxf` wrote, via the same
/// qualifying `dxf` reader `s.stdio.dxf@r12/✳️any` registers.
#[cfg(feature = "oracles")]
pub fn project_note_dxf(bytes: &[u8]) -> Result<Json, String> {
    crate::artifacts::dxf::standards::v_r12::subsets::any::project_dxf_r12(bytes)
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
mod smoke_tests {
    use super::{project_note_dxf, project_note_pdf, project_note_svg};

    /// 🖊️ A minimal DXF R12 `ENTITIES` section holding exactly the shape `NoteIntoDxf::serialize`
    /// emits for one Ink block's `points.windows(2)` pair: one `LINE` on layer `"0"`.
    const DXF_ONE_LINE: &str = "0\nSECTION\n2\nENTITIES\n0\nLINE\n8\n0\n10\n0.0\n20\n0.0\n30\n0.0\n11\n10.0\n21\n20.0\n31\n0.0\n0\nENDSEC\n0\nEOF\n";

    #[test]
    fn project_note_dxf_reads_the_line_entity_note_would_have_written() {
        let projected = project_note_dxf(DXF_ONE_LINE.as_bytes()).expect("dxf crate parses a minimal ENTITIES section");
        let entities = projected.array("entities");
        assert_eq!(entities.len(), 1, "expected exactly the one LINE entity");
        assert_eq!(entities[0].str("kind"), "line");
    }

    /// 🎨️ The exact `<g transform="matrix(a,b,c,d,e,f)"><path d="…"/></g>` shape
    /// `svg_element_from_draw_node` (the semio/drawing→svg composer note's real bridge dispatches
    /// through) writes for one block: a translate-by-(5,10) group wrapping an ink path.
    const SVG_ONE_GROUP: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"100\" height=\"100\"><g id=\"layer-0\"><g transform=\"matrix(1,0,0,1,5,10)\"><path d=\"M0,0 L1,1\"/></g></g></svg>";

    #[test]
    fn project_note_svg_decomposes_the_group_transform_and_reaches_the_path() {
        let projected = project_note_svg(SVG_ONE_GROUP.as_bytes()).expect("quick-xml parses well-formed SVG");
        let root = projected.get("root").expect("root present");
        assert_eq!(root.str("name"), "svg");
        let outer_layer_group = &root.array("children")[0];
        let block_group = &outer_layer_group.array("children")[0];
        let transform = block_group.get("transform").expect("transform decomposed, not left as a raw string");
        // 🔺 `MarkupTransformOp::Matrix{a,b,c,d,e,f}` projected positionally — e/f are the translation
        // this subject's `note_block_transform` would have written for `x: 5.0, y: 10.0`.
        assert!(format!("{transform:?}").contains('5'), "expected the translate-x component 5 somewhere in {transform:?}");
        let path = &block_group.array("children")[0];
        assert_eq!(path.str("name"), "path");
    }

    #[test]
    fn project_note_pdf_reads_the_text_lopdf_itself_wrote() {
        use lopdf::{dictionary, Document, Object, Stream};
        let mut document = Document::with_version("1.4");
        let pages_id = document.new_object_id();
        let content = lopdf::content::Content { operations: vec![lopdf::content::Operation::new("Tj", vec![Object::string_literal("hello from note")])] };
        let content_id = document.add_object(Stream::new(dictionary! {}, content.encode().expect("encode content stream")));
        let page_id = document.add_object(dictionary! { "Type" => "Page", "Parent" => pages_id, "Contents" => content_id, "MediaBox" => vec![0.into(), 0.into(), 200.into(), 100.into()] });
        let pages = dictionary! { "Type" => "Pages", "Kids" => vec![page_id.into()], "Count" => 1 };
        document.objects.insert(pages_id, Object::Dictionary(pages));
        let catalog_id = document.add_object(dictionary! { "Type" => "Catalog", "Pages" => pages_id });
        document.trailer.set("Root", catalog_id);
        let mut bytes = Vec::new();
        document.save_to(&mut bytes).expect("lopdf saves the document it just built");

        let projected = project_note_pdf(&bytes).expect("lopdf reads back its own document");
        let pages_json = projected.array("pages");
        assert_eq!(pages_json.len(), 1);
        let text = pages_json[0].array("text");
        assert!(text.iter().any(|value| matches!(value, semio_repo_test_host::Json::String(s) if s == "hello from note")), "expected the Tj text operand to surface, got {text:?}");
    }
}
//#endregion 🧪️SmokeTests
