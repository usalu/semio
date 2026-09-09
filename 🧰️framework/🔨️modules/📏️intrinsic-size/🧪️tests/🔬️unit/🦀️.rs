
use super::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Corpus {
    #[serde(rename = "svgCases")]
    svg_cases: Vec<SvgCase>,
    #[serde(rename = "rasterCases")]
    raster_cases: Vec<RasterCase>,
}

#[derive(Deserialize)]
struct SvgCase {
    name: String,
    svg: String,
    width: Option<f64>,
    height: Option<f64>,
    #[serde(default)]
    error: bool,
}

#[derive(Deserialize)]
struct RasterCase {
    name: String,
    width: u32,
    height: u32,
    bytes: Vec<u8>,
}

fn corpus() -> Corpus {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("valid intrinsic-size fixture corpus")
}

#[test]
fn svg_fixture_corpus_matches_recorded_usvg_derived_expectations() {
    let c = corpus();
    assert!(!c.svg_cases.is_empty());
    for case in &c.svg_cases {
        let got = svg_intrinsic_size(&case.svg);
        if case.error {
            assert!(got.is_err(), "{}: expected error, got {got:?}", case.name);
        } else {
            let (w, h) = got.unwrap_or_else(|e| panic!("{}: expected Ok, got {e:?}", case.name));
            assert!((w - case.width.unwrap()).abs() < 1e-6, "{}: width {w} != {:?}", case.name, case.width);
            assert!((h - case.height.unwrap()).abs() < 1e-6, "{}: height {h} != {:?}", case.name, case.height);
        }
    }
    println!("svg fixture corpus: {}/{} matched", c.svg_cases.len(), c.svg_cases.len());
}

#[test]
fn raster_fixture_corpus_matches_recorded_image_crate_derived_expectations() {
    let c = corpus();
    assert!(!c.raster_cases.is_empty());
    for case in &c.raster_cases {
        let (w, h) = raster_dimensions(&case.bytes).unwrap_or_else(|e| panic!("{}: {e:?}", case.name));
        assert_eq!((w, h), (case.width, case.height), "{}", case.name);
    }
    println!("raster fixture corpus: {}/{} matched", c.raster_cases.len(), c.raster_cases.len());
}

fn fixture_image(w: u32, h: u32) -> image::RgbaImage {
    image::RgbaImage::from_fn(w, h, |x, y| image::Rgba([(x % 256) as u8, (y % 256) as u8, ((x + y) % 256) as u8, 255]))
}

const DIMS: &[(u32, u32)] = &[(1, 1), (1, 7), (7, 1), (2, 3), (16, 16), (17, 9), (64, 64), (100, 1), (1, 100), (33, 257), (513, 129), (300, 200)];

#[test]
fn png_oracle_matches_image_crate_across_corpus() {
    use image::{GenericImageView, ImageEncoder};
    let mut checked = 0;
    for &(w, h) in DIMS {
        let img = fixture_image(w, h);
        let mut bytes = Vec::new();
        image::codecs::png::PngEncoder::new(&mut bytes).write_image(&img, w, h, image::ExtendedColorType::Rgba8).expect("encode png");
        let oracle = image::load_from_memory(&bytes).expect("oracle decode png");
        let ours = raster_dimensions(&bytes).expect("our png dims");
        assert_eq!(ours, oracle.dimensions(), "png {w}x{h} mismatch");
        assert_eq!(ours, (w, h));
        checked += 1;
    }
    println!("png oracle: {checked}/{} matched image crate", DIMS.len());
    assert_eq!(checked, DIMS.len());
}

#[test]
fn jpeg_oracle_matches_image_crate_across_corpus() {
    use image::GenericImageView;
    let mut checked = 0;
    for &(w, h) in DIMS {
        let img = fixture_image(w, h);
        let mut bytes = Vec::new();
        image::codecs::jpeg::JpegEncoder::new(&mut bytes).encode_image(&img).expect("encode jpeg");
        let oracle = image::load_from_memory(&bytes).expect("oracle decode jpeg");
        let ours = raster_dimensions(&bytes).expect("our jpeg dims");
        assert_eq!(ours, oracle.dimensions(), "jpeg {w}x{h} mismatch");
        assert_eq!(ours, (w, h));
        checked += 1;
    }
    println!("jpeg oracle: {checked}/{} matched image crate", DIMS.len());
    assert_eq!(checked, DIMS.len());
}

#[test]
fn gif_oracle_matches_image_crate_across_corpus() {
    use image::GenericImageView;
    let mut checked = 0;
    for &(w, h) in DIMS {
        let img = fixture_image(w, h);
        let mut bytes = Vec::new();
        {
            let mut enc = image::codecs::gif::GifEncoder::new(&mut bytes);
            enc.encode(&img, w, h, image::ExtendedColorType::Rgba8).expect("encode gif");
        }
        let oracle = image::load_from_memory(&bytes).expect("oracle decode gif");
        let ours = raster_dimensions(&bytes).expect("our gif dims");
        assert_eq!(ours, oracle.dimensions(), "gif {w}x{h} mismatch");
        assert_eq!(ours, (w, h));
        checked += 1;
    }
    println!("gif oracle: {checked}/{} matched image crate", DIMS.len());
    assert_eq!(checked, DIMS.len());
}

#[test]
fn webp_lossless_oracle_matches_image_crate_across_corpus() {
    use image::GenericImageView;
    let mut checked = 0;
    for &(w, h) in DIMS {
        let img = fixture_image(w, h);
        let mut bytes = Vec::new();
        image::codecs::webp::WebPEncoder::new_lossless(&mut bytes).encode(&img, w, h, image::ExtendedColorType::Rgba8).expect("encode webp");
        assert_eq!(&bytes[12..16], b"VP8L", "expected a lossless VP8L chunk from the encoder");
        let oracle = image::load_from_memory(&bytes).expect("oracle decode webp");
        let ours = raster_dimensions(&bytes).expect("our webp dims");
        assert_eq!(ours, oracle.dimensions(), "webp {w}x{h} mismatch");
        assert_eq!(ours, (w, h));
        checked += 1;
    }
    println!("webp (VP8L) oracle: {checked}/{} matched image crate", DIMS.len());
    assert_eq!(checked, DIMS.len());
}

/// 🧩 `image`'s webp encoder only emits lossless `VP8L`; no lossy/`VP8X` encoder was available
/// to generate a full decodable bitstream as a third-party oracle (a synthetic bitstream with
/// no real coefficient data fails `image`'s own decode, which performs a full pixel decode, not
/// a header read). Verified instead as hand-built, spec-conformant fixtures against the WebP
/// RIFF container spec's own documented byte layout — the same "hand-built known-good block"
/// technique already used for this ticket's DEFLATE module's stored-block (BTYPE=00) test.
#[test]
fn webp_lossy_and_extended_headers_match_spec_hand_built_fixtures() {
    let mut vp8 = Vec::new();
    vp8.extend_from_slice(b"RIFF");
    vp8.extend_from_slice(&0u32.to_le_bytes());
    vp8.extend_from_slice(b"WEBP");
    vp8.extend_from_slice(b"VP8 ");
    vp8.extend_from_slice(&0u32.to_le_bytes());
    vp8.extend_from_slice(&[0x30, 0x01, 0x00]);
    vp8.extend_from_slice(&[0x9D, 0x01, 0x2A]);
    let w: u16 = 200;
    let h: u16 = 100;
    vp8.extend_from_slice(&(w & 0x3FFF).to_le_bytes());
    vp8.extend_from_slice(&(h & 0x3FFF).to_le_bytes());
    assert_eq!(raster_dimensions(&vp8), Ok((200, 100)));

    let mut vp8l = Vec::new();
    vp8l.extend_from_slice(b"RIFF");
    vp8l.extend_from_slice(&0u32.to_le_bytes());
    vp8l.extend_from_slice(b"WEBP");
    vp8l.extend_from_slice(b"VP8L");
    vp8l.extend_from_slice(&0u32.to_le_bytes());
    let width_m1: u32 = 319;
    let height_m1: u32 = 149;
    let mut packed: u32 = width_m1 & 0x3FFF;
    packed |= (height_m1 & 0x3FFF) << 14;
    vp8l.push(0x2F);
    vp8l.extend_from_slice(&packed.to_le_bytes());
    assert_eq!(raster_dimensions(&vp8l), Ok((320, 150)));

    let mut vp8x = Vec::new();
    vp8x.extend_from_slice(b"RIFF");
    vp8x.extend_from_slice(&0u32.to_le_bytes());
    vp8x.extend_from_slice(b"WEBP");
    vp8x.extend_from_slice(b"VP8X");
    vp8x.extend_from_slice(&10u32.to_le_bytes());
    vp8x.push(0x00);
    vp8x.extend_from_slice(&[0, 0, 0]);
    let cw_m1: u32 = 639;
    let ch_m1: u32 = 479;
    vp8x.push((cw_m1 & 0xFF) as u8);
    vp8x.push(((cw_m1 >> 8) & 0xFF) as u8);
    vp8x.push(((cw_m1 >> 16) & 0xFF) as u8);
    vp8x.push((ch_m1 & 0xFF) as u8);
    vp8x.push(((ch_m1 >> 8) & 0xFF) as u8);
    vp8x.push(((ch_m1 >> 16) & 0xFF) as u8);
    assert_eq!(raster_dimensions(&vp8x), Ok((640, 480)));

    println!("webp VP8/VP8X hand-built spec fixtures: 3/3 matched documented byte layout");
}

#[test]
fn raster_dimensions_rejects_malformed_and_truncated_input() {
    assert!(raster_dimensions(&[]).is_err());
    assert!(raster_dimensions(&[0x00, 0x01, 0x02]).is_err());
    assert!(raster_dimensions(&PNG_SIGNATURE).is_err());
    assert!(raster_dimensions(b"GIF89a").is_err());
    assert!(raster_dimensions(&[0xFF, 0xD8]).is_err());
}

fn usvg_oracle_size(svg: &str) -> Option<(f64, f64)> {
    let opt = usvg::Options::default();
    let tree = usvg::Tree::from_str(svg, &opt).ok()?;
    let s = tree.size();
    Some((f64::from(s.width()), f64::from(s.height())))
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.01
}

#[test]
fn svg_oracle_matches_usvg_live_across_fixture_corpus() {
    let c = corpus();
    let mut checked = 0;
    let mut hard_fail_agrees = 0;
    for case in &c.svg_cases {
        let oracle = usvg_oracle_size(&case.svg);
        let ours = svg_intrinsic_size(&case.svg);
        match (oracle, ours) {
            (Some((ow, oh)), Ok((w, h))) => {
                assert!(close(ow, w) && close(oh, h), "{}: oracle=({ow},{oh}) ours=({w},{h})", case.name);
                checked += 1;
            }
            (None, Err(_)) => hard_fail_agrees += 1,
            other => panic!("{}: disagreement with usvg oracle: {other:?}", case.name),
        }
    }
    println!("svg live usvg oracle: {checked} numeric matches + {hard_fail_agrees} agreed hard failures / {} cases", c.svg_cases.len());
    assert_eq!(checked + hard_fail_agrees, c.svg_cases.len());
}
