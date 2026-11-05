//! 🖊️ Real board drawing formats are readable by owned codecs and an independent DWG reader.
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::dwg::v_ac1024::any as dwg_out;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::dxf::v_r12::any as dxf_out;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::pdf::v1_4::any as pdf_out;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::png::v1_2::any as png_out;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::svg::v1_1::any as svg_out;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::txt::v_utf_8::any as txt_out;
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::any as txt_in;
use crate::Puzzle2dSnapshot;
use serde_json::{json, Value};

fn cases() -> Value {
    serde_json::from_str(include_str!("📁️cases.json")).expect("authored format witness")
}

struct BoardOwner(Option<Puzzle2dSnapshot>);
impl BoardOwner {
    fn get(&self) -> &Puzzle2dSnapshot {
        self.0.as_ref().expect("live board")
    }
}
impl Drop for BoardOwner {
    fn drop(&mut self) {
        if let Some(snapshot) = self.0.take() {
            let mut source = crate::standards::v1::subsets::any::schema::testkit::admit_source(std::sync::Arc::new(snapshot));
            crate::standards::v1::subsets::any::schema::testkit::close_source(&mut source);
        }
    }
}
fn board() -> BoardOwner {
    BoardOwner(Some(semio_framework_pack_json::from_json_str(
        &cases()["snapshot"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject,
    ).expect("board json")))
}

fn svg_witness(source: &str) -> (usize, Vec<String>) {
    use quick_xml::events::Event;
    let mut reader = quick_xml::Reader::from_str(source);
    let mut stack = Vec::new();
    let mut roots = 0;
    let mut paths = 0;
    let mut labels = Vec::new();
    loop {
        match reader.read_event().expect("independent XML parse") {
            Event::Start(element) => {
                let name = element.name().as_ref().to_string();
                if stack.is_empty() {
                    roots += 1;
                    assert_eq!(name, "svg", "real SVG root");
                }
                if name == "path" { paths += 1; }
                stack.push(name);
            }
            Event::Empty(element) => {
                let name = element.name().as_ref().to_string();
                if stack.is_empty() {
                    roots += 1;
                    assert_eq!(name, "svg", "real SVG root");
                }
                if name == "path" { paths += 1; }
            }
            Event::End(element) => assert_eq!(stack.pop().as_deref(), Some(element.name().as_ref()), "balanced XML"),
            Event::Text(text) if stack.last().is_some_and(|name| name == "text") => labels.push(text.as_ref().to_string()),
            Event::Eof => break,
            _ => {}
        }
    }
    assert_eq!(roots, 1);
    assert!(stack.is_empty(), "closed SVG document");
    labels.sort();
    (paths, labels)
}

fn pdf_witness(source: &[u8]) -> (usize, String) {
    let document = lopdf::Document::load_mem(source).expect("independent PDF parse");
    let pages: Vec<u32> = document.get_pages().keys().copied().collect();
    let text = document.extract_text(&pages).expect("independent PDF page text");
    (pages.len(), text)
}

#[test]
fn svg_draws_the_visible_board() {
    let board = board();
    let svg = String::from_utf8(svg_out::serialize_bytes(board.get()).expect("svg")).expect("utf-8");
    assert_eq!(svg.matches("<path").count(), 4, "frame, link and two nodes: {svg}");
    assert!(svg.contains(">root<") && svg.contains(">goal<"), "{svg}");
    let (paths, labels) = svg_witness(&svg);
    assert_eq!(paths, cases()["page"]["paths"].as_u64().unwrap() as usize);
    assert_eq!(json!(labels), cases()["page"]["labels"], "independently parsed visible SVG labels");
}

#[test]
fn page_formats_are_real_files() {
    let board = board();
    let pdf = pdf_out::serialize_bytes(board.get()).expect("pdf");
    assert!(pdf.starts_with(b"%PDF-1.4"));
    let (pages, text) = pdf_witness(&pdf);
    assert_eq!(pages, cases()["page"]["pages"].as_u64().unwrap() as usize);
    for label in cases()["page"]["labels"].as_array().unwrap() {
        assert!(text.contains(label.as_str().unwrap()), "PDF page is missing visible label {label}: {text}");
    }
    let png = semio_s_artifact_stdio_png::standards::v1_2::subsets::any::io::png_layout(&semio_s_artifact_stdio_png::standards::v1_2::subsets::any::io::decode_png(&png_out::serialize_bytes(board.get()).expect("png")).expect("decodes as png")).expect("png header");
    assert_eq!((png.width, png.height), (240 + 64, 120 + 64));
    let dxf = String::from_utf8(dxf_out::serialize_bytes(board.get()).expect("dxf")).expect("dxf text");
    assert_eq!(dxf.matches("CIRCLE").count(), 1, "{dxf}");
}

fn drawing_witness(standard: &str, circles: Vec<Value>, mut texts: Vec<Value>) -> Value {
    texts.sort_by(|a, b| a["value"].as_str().unwrap().cmp(b["value"].as_str().unwrap()));
    json!({"standard": standard, "circles": circles, "texts": texts})
}

#[test]
fn dwg_draws_the_visible_board_for_both_independent_readers() {
    use acadrust::entities::EntityType;
    use semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry;
    let routes: Vec<_> = super::routes::entries().iter().filter(|entry| entry.into.artifact_kind == "s.stdio.dwg").collect();
    assert_eq!(routes.len(), 1, "one actual Puzzle DWG output route");
    assert_eq!(routes[0].into.standard.0, "ac1024", "the registered dialect matches the real writer");
    let board = board();
    let bytes = dwg_out::serialize_bytes(board.get()).expect("DWG export");
    let standard = std::str::from_utf8(bytes.get(..6).expect("DWG version")).expect("version text");
    let own = semio_s_artifact_stdio_dwg_read::standards::v_ac1024::subsets::any::io::import::dwg_from_bytes(&bytes).expect("owned DWG reader");
    let circles = own.entities.iter().filter_map(|entity| match &entity.geometry {
        DwgGeometry::Circle { center, radius, .. } => Some(json!({"center": center, "radius": radius})),
        _ => None,
    }).collect();
    let texts = own.entities.iter().filter_map(|entity| match &entity.geometry {
        DwgGeometry::Text { at, content, .. } => Some(json!({"at": at, "value": content})),
        _ => None,
    }).collect();
    let expected = cases()["dwg"].clone();
    assert_eq!(drawing_witness(standard, circles, texts), expected, "owned reader preserves the visible circle and both labels; the hidden ghost adds no circle");
    let document = acadrust::DwgReader::from_stream(std::io::Cursor::new(bytes)).read().expect("independent acadrust DWG reader");
    assert_eq!(document.version, acadrust::types::DxfVersion::AC1024);
    let circles = document.entities().filter_map(|entity| match entity {
        EntityType::Circle(circle) => Some(json!({"center": [circle.center.x, circle.center.y, circle.center.z], "radius": circle.radius})),
        _ => None,
    }).collect();
    let texts = document.entities().filter_map(|entity| match entity {
        EntityType::Text(text) => Some(json!({"at": [text.insertion_point.x, text.insertion_point.y, text.insertion_point.z], "value": text.value})),
        _ => None,
    }).collect();
    assert_eq!(drawing_witness("AC1024", circles, texts), expected, "independent reader preserves the same authored visible board");
}

#[test]
fn txt_is_the_exact_dsl_carrier() {
    let board = board();
    let bytes = txt_out::serialize_bytes(board.get()).expect("txt export");
    let restored = BoardOwner(Some(txt_in::deserialize_bytes(&bytes).expect("txt import")));
    assert_eq!(restored.get(), board.get());
}
