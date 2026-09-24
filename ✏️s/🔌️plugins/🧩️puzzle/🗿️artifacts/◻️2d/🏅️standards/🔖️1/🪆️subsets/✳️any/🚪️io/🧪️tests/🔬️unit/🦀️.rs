use crate::standards::v1::subsets::any::io::export::serializers::artifacts::{dxf::v_r12::any as dxf_out, pdf::v1_4::any as pdf_out, png::v1_2::any as png_out, svg::v1_1::any as svg_out, txt::v_utf_8::any as txt_out};
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::any as txt_in;
use crate::Puzzle2dSnapshot;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn board() -> Puzzle2dSnapshot {
    let mut board = Puzzle2dSnapshot::default();
    let extra: Puzzle2dSnapshot = dsl::os_pack::json::from_json_str(
        r#"{"schema":"puzzle.2d","camera":{"x":0,"y":0,"zoom":1},"nodes":[{"id":"a","shape":"circle","x":0,"y":0,"radius":20,"text":"root"},{"id":"b","shape":"rectangle","x":120,"y":40,"width":60,"height":30},{"id":"ghost","x":500,"y":500,"visible":false}],"edges":[{"id":"e","source":"a","target":"b"}],"targetRegions":[{"id":"t","x":-40,"y":-40,"width":240,"height":120,"label":"goal","hidden":false,"locked":false}],"meta":{}}"#,
    )
    .expect("board json");
    board.nodes = extra.nodes;
    board.edges = extra.edges;
    board.target_regions = extra.target_regions;
    board
}

#[test]
fn svg_draws_the_visible_board() {
    let svg = String::from_utf8(svg_out::serialize_bytes(&board()).expect("svg")).expect("utf-8");
    assert_eq!(svg.matches("<path").count(), 4, "frame, link and two nodes: {svg}");
    assert!(svg.contains(">root<") && svg.contains(">goal<"), "{svg}");
}

#[test]
fn page_formats_are_real_files() {
    assert!(pdf_out::serialize_bytes(&board()).expect("pdf").starts_with(b"%PDF-1.4"));
    let png = semio_s_artifact_stdio_png::io::decode_png(&png_out::serialize_bytes(&board()).expect("png")).expect("decodes as png");
    assert_eq!((png.width, png.height), (240 + 64, 120 + 64));
    let dxf = String::from_utf8(dxf_out::serialize_bytes(&board()).expect("dxf")).expect("dxf text");
    assert_eq!(dxf.matches("CIRCLE").count(), 1, "{dxf}");
}

#[test]
fn txt_is_the_exact_dsl_carrier() {
    let bytes = txt_out::serialize_bytes(&board()).expect("txt export");
    assert_eq!(txt_in::deserialize_bytes(&bytes).expect("txt import"), board());
}
