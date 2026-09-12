
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
    use lopdf::{Document, Object, Stream, dictionary};
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
