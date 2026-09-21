//! 🔬️ drawing → pdf painter laws: the bytes are a PDF 1.4 that `s.stdio.pdf`'s own 1.4 reader
//! opens (classic xref, page tree, MediaBox = artboard), and the inflated content stream carries the
//! document's outlines, fills, strokes, shadings, text and images as real painting operators.

use super::*;
use crate::schema::{create_drawing_image_layer, create_drawing_path_layer, create_drawing_shape_layer_rect, create_drawing_text_layer, default_drawing_document, layer_base_mut};
use crate::{DrawingArtboard, DrawingImageAsset, DrawingLayerNode};

fn find(haystack: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    haystack[from..].windows(needle.len()).position(|window| window == needle).map(|index| index + from)
}

fn inflate_content(pdf: &[u8]) -> String {
    // 📦️ The page content is the one `<< /Filter /FlateDecode /Length N >>` stream (images and
    // masks carry `/Type /XObject` first); offsets are byte offsets — the binary header comment
    // would drift a lossy-UTF-8 search.
    let marker = b"<< /Filter /FlateDecode /Length ";
    let start = find(pdf, marker, 0).expect("a content stream");
    let length_start = start + marker.len();
    let length_end = find(pdf, b" ", length_start).unwrap();
    let length: usize = std::str::from_utf8(&pdf[length_start..length_end]).unwrap().parse().unwrap();
    let data_start = find(pdf, b"stream\n", start).unwrap() + b"stream\n".len();
    let data = &pdf[data_start..data_start + length];
    String::from_utf8(semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::zlib_decompress(data).expect("content inflates")).expect("content is text")
}

fn demo() -> DrawingSnapshot {
    let source = crate::standards::v1::subsets::any::examples().iter().find(|source| source.id() == "demo").expect("demo example");
    <DrawingSnapshot as crate::ArtifactDsl>::parse_dsl(&source.document_json()).expect("demo parses")
}

#[test]
fn demo_exports_a_pdf_the_stdio_reader_opens_with_the_artboard_page_and_every_outline() {
    let pdf = drawing_document_to_pdf(&demo()).expect("demo exports");
    assert!(pdf.starts_with(b"%PDF-1.4\n"));
    assert!(pdf.ends_with(b"%%EOF\n"));
    let read = semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::io::decode_pdf(&pdf).expect("stdio's pdf 1.4 reader opens the export");
    assert_eq!(read.pages.len(), 1);
    assert_eq!((read.pages[0].width, read.pages[0].height), (200.0, 200.0), "the artboard is the page");
    let content = inflate_content(&pdf);
    assert!(content.starts_with("q 1 0 0 -1 0 200 cm\n"), "the base CTM flips drawing space onto the page: {content}");
    assert_eq!(content.matches(" rg\n").count(), 3, "three solid fills (orange, red, teal)");
    assert!(content.contains("0.98 0.584 0 rg"), "the orange wedge's fill colour");
    assert!(content.contains("0 0.067 0.09 RG 2.5 w 0 J 0 j"), "the shared miter stroke");
    assert!(content.contains("1.25 196.933 m\n36.25 161.125 l\n"), "path coordinates are verbatim drawing coordinates");
    assert_eq!(content.matches("B*\n").count(), 3, "fill+stroke paints even-odd, as the canvas does");
    assert!(!content.contains("Do"), "the emblem's PNG is a trace SOURCE asset, not an image layer");
}

#[test]
fn gradients_text_images_opacity_and_arcs_paint_through_their_pdf_constructs() {
    let mut doc = default_drawing_document("laws", None);
    doc.artboard = Some(DrawingArtboard { width: 300.0, height: 150.0 });
    doc.layers.clear();
    let mut rect = create_drawing_shape_layer_rect("Gradient");
    {
        let base = layer_base_mut(&mut rect);
        base.opacity = 0.5;
        base.blend_mode = "multiply".into();
        base.attributes.fill = Some(FillStyle::LinearGradient { x1: 0.0, y1: 0.0, x2: 100.0, y2: 0.0, stops: vec![GradientStop { offset: 0.2, color: [1.0, 0.0, 0.0, 1.0] }, GradientStop { offset: 0.5, color: [0.0, 1.0, 0.0, 1.0] }, GradientStop { offset: 1.0, color: [0.0, 0.0, 1.0, 1.0] }] });
        base.attributes.stroke = Some(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 1.5, cap: "round".into(), join: "bevel".into(), dash: Some(vec![4.0, 2.0]) });
    }
    doc.layers.push(rect);
    let mut text = create_drawing_text_layer("Caption");
    if let DrawingLayerNode::Text(body) = &mut text {
        body.content = "Hi (there) \\ café".into();
        body.size = 14.0;
        body.base.transform.x = 20.0;
        body.base.transform.y = 120.0;
        body.base.attributes.fill = Some(FillStyle::Solid { color: [0.1, 0.2, 0.3, 1.0] });
    }
    doc.layers.push(text);
    // 🖼️ A 2×2 RGBA PNG with one transparent pixel.
    let png = semio_framework_pixels::encode_png(&semio_framework_pixels::RasterImage { width: 2, height: 2, pixels: vec![255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 0, 0, 0, 0] }).expect("png encodes");
    doc.assets.insert("pic".into(), DrawingImageAsset { mime: "image/png".into(), data: base64_codec::base64_standard_encode(&png), width: None, height: None });
    let mut image = create_drawing_image_layer("Picture", "pic");
    if let DrawingLayerNode::Image(body) = &mut image {
        body.width = 40.0;
        body.height = 30.0;
    }
    doc.layers.push(image);
    let mut arc = create_drawing_path_layer("Arc", vec![PathSegment::Move { to: [10.0, 10.0] }, PathSegment::Arc { rx: 20.0, ry: 20.0, rotation: 0.0, large_arc: false, sweep: true, to: [50.0, 10.0] }, PathSegment::Quad { ctrl: [30.0, 60.0], to: [10.0, 10.0] }, PathSegment::Close]);
    layer_base_mut(&mut arc).attributes.fill = Some(FillStyle::RadialGradient { cx: 30.0, cy: 20.0, r: 25.0, stops: vec![GradientStop { offset: 0.0, color: [1.0, 1.0, 1.0, 1.0] }, GradientStop { offset: 1.0, color: [0.0, 0.0, 0.0, 1.0] }] });
    doc.layers.push(arc);

    let pdf = drawing_document_to_pdf(&doc).expect("exports");
    let read = semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::io::decode_pdf(&pdf).expect("reader opens the export");
    assert_eq!((read.pages[0].width, read.pages[0].height), (300.0, 150.0));
    let text = String::from_utf8_lossy(&pdf).into_owned();
    assert!(text.contains("/ShadingType 2 /ColorSpace /DeviceRGB /Coords [0 0 100 0]"), "axial shading");
    assert!(text.contains("/ShadingType 3 /ColorSpace /DeviceRGB /Coords [30 20 0 30 20 25]"), "radial shading");
    assert!(text.contains("/FunctionType 3 /Domain [0 1] /Functions ["), "a three-stop gradient stitches");
    assert!(text.contains("/ExtGState << /GS1 << /Type /ExtGState /ca 0.5 /CA 0.5 /BM /Multiply >> >>"), "opacity and blend mode become one ExtGState");
    assert!(text.contains("/Font << /F1 "), "Helvetica is declared once text is painted");
    assert!(text.contains("/Subtype /Image /Width 2 /Height 2 /ColorSpace /DeviceRGB /BitsPerComponent 8 /Filter /FlateDecode /SMask "), "the RGBA image carries an alpha SMask");
    let content = inflate_content(&pdf);
    assert!(content.contains("/GS1 gs"));
    assert!(content.contains("0 0 0 RG 1.5 w 1 J 2 j\n[4 2] 0 d"), "round caps, bevel joins, dash: {content}");
    assert!(content.contains("W* n\n/Sh1 sh"), "the gradient fill clips through the outline");
    assert!(content.contains("BT /F1 14 Tf 0.1 0.2 0.3 rg 1 0 0 -1 0 0 Tm (Hi \\(there\\) \\\\ caf\\351) Tj ET"), "text escapes and WinAnsi: {content}");
    assert!(content.contains("q 40 0 0 -30 0 30 cm /Im1 Do Q"), "the image is drawn upright at the node origin: {content}");
    assert!(content.contains(" c\n"), "the arc and the quad became cubics: {content}");
    assert!(content.matches(" c\n").count() >= 2);
}

#[test]
fn arc_to_cubics_lands_on_the_endpoint_and_degrades_honestly() {
    let pieces = arc_to_cubics([0.0, 0.0], 10.0, 10.0, 0.0, false, true, [20.0, 0.0]);
    assert_eq!(pieces.len(), 2, "a half turn is two quarter-turn cubics");
    assert_eq!(pieces[pieces.len() - 1][2], [20.0, 0.0]);
    let mid = pieces[0][2];
    assert!((mid[0] - 10.0).abs() < 1e-9 && (mid[1].abs() - 10.0).abs() < 1e-9, "the quarter point sits on the circle: {mid:?}");
    assert_eq!(arc_to_cubics([0.0, 0.0], 0.0, 10.0, 0.0, false, true, [20.0, 0.0]), vec![[[0.0, 0.0], [20.0, 0.0], [20.0, 0.0]]], "a zero radius is a line");
    assert!(arc_to_cubics([5.0, 5.0], 10.0, 10.0, 0.0, false, true, [5.0, 5.0]).is_empty(), "coincident endpoints paint nothing");
}

#[test]
fn an_empty_document_still_exports_a_page() {
    let mut doc = default_drawing_document("blank", None);
    doc.layers.clear();
    doc.artboard = None;
    let pdf = drawing_document_to_pdf(&doc).expect("blank exports");
    let read = semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::io::decode_pdf(&pdf).expect("opens");
    assert_eq!((read.pages[0].width, read.pages[0].height), DEFAULT_PAGE);
}

#[test]
fn numbers_and_strings_follow_the_pdf_lexicon() {
    assert_eq!(num(1.0), "1");
    assert_eq!(num(0.5), "0.5");
    assert_eq!(num(-0.00001), "0");
    assert_eq!(num(123.456789), "123.4568");
    assert_eq!(num(f64::NAN), "0");
    assert_eq!(pdf_string("a(b)c\\d\né"), "a\\(b\\)c\\\\d\\n\\351");
    assert_eq!(pdf_string("日本"), "??");
}
