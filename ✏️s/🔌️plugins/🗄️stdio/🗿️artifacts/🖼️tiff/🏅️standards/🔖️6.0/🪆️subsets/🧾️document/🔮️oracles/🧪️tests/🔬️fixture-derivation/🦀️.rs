
use super::*;

fn ifd0_from_image_encoder(rgb: image::RgbImage) -> OracleIfd {
    let mut cursor = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgb8(rgb).write_to(&mut cursor, image::ImageFormat::Tiff).expect("image crate: encode reference TIFF");
    let doc = read_tiff(&cursor.into_inner()).expect("re-parse the reference encoder's own bytes");
    doc.ifds.into_iter().next().expect("reference encoder wrote at least one IFD")
}

/// 🧭️ Walks up from `start` looking for the repo root's own `CLAUDE.md` — robust regardless of
/// how deep the compiling crate's manifest happens to sit (this file is also `#[path]`-included
/// from a throwaway type-checking crate outside the real oracle crate's tree during review).
fn find_repo_root(start: &std::path::Path) -> std::path::PathBuf {
    let mut dir = start.to_path_buf();
    for _ in 0..32 {
        let mut candidate = dir.clone();
        candidate.push("CLAUDE.md");
        if candidate.is_file() {
            return dir;
        }
        if !dir.pop() {
            break;
        }
    }
    panic!("could not find repo root (CLAUDE.md) above {}", start.display());
}

#[test]
#[ignore]
fn derive_real_world_fixture() {
    let repo_root = find_repo_root(std::path::Path::new(env!("CARGO_MANIFEST_DIR")));
    let mut jpeg_path = repo_root.clone();
    jpeg_path.push("🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️assets/🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg");
    let mut png_path = repo_root.clone();
    png_path.push("🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️assets/🏛️rathaus-ahlen-grundriss/🖼️.png");
    let mut out_path = repo_root.clone();
    out_path.push("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🧫️fixtures/🖼️abbau-aufbau-masterarbeit-grundriss.tiff");

    // IFD 0: the real 500 DPI scan, RGB8, via the registered `image` reference encoder.
    let photo = image::open(&jpeg_path).expect("open real JPEG scan").to_rgb8();
    assert_eq!((photo.width(), photo.height()), (2275, 2560), "source scan dimensions moved — re-check the ticket's own numbers");
    let ifd0 = ifd0_from_image_encoder(photo.clone());

    // IFD 1: the real second page — genuine decoded+downsampled pixels of the rathaus PNG,
    // decoded with the registered `png` reference decoder (independent of `image`, which has
    // no PNG feature linked in this crate) then downsampled with `image`'s own generic resize.
    let png_bytes = std::fs::read(&png_path).expect("read real PNG floor plan");
    let mut png_reader = png::Decoder::new(std::io::Cursor::new(&png_bytes)).read_info().expect("png: read_info");
    let mut buf = vec![0u8; png_reader.output_buffer_size().unwrap_or(0)];
    let frame = png_reader.next_frame(&mut buf).expect("png: next_frame");
    let info = png_reader.info();
    let palette = info.palette.clone();
    let trns = info.trns.clone();
    let rgba: Vec<u8> = match frame.color_type {
        png::ColorType::Rgba => buf[..frame.buffer_size()].to_vec(),
        png::ColorType::Rgb => buf[..frame.buffer_size()].chunks_exact(3).flat_map(|p| [p[0], p[1], p[2], 255]).collect(),
        png::ColorType::Grayscale => buf[..frame.buffer_size()].iter().flat_map(|&g| [g, g, g, 255]).collect(),
        png::ColorType::GrayscaleAlpha => buf[..frame.buffer_size()].chunks_exact(2).flat_map(|p| [p[0], p[0], p[0], p[1]]).collect(),
        png::ColorType::Indexed => {
            let table = palette.as_deref().expect("indexed PNG without a palette");
            buf[..frame.buffer_size()]
                .iter()
                .flat_map(|&index| {
                    let base = index as usize * 3;
                    let alpha = trns.as_deref().and_then(|t| t.get(index as usize).copied()).unwrap_or(255);
                    [table[base], table[base + 1], table[base + 2], alpha]
                })
                .collect()
        }
    };
    let full = image::RgbaImage::from_raw(frame.width, frame.height, rgba).expect("rathaus PNG raw buffer matches its own dimensions");
    let small = image::imageops::thumbnail(&full, 16, 16);
    let ifd1 = ifd0_from_image_encoder(image::DynamicImage::ImageRgba8(small).to_rgb8());

    let doc = OracleDoc { little_endian: true, ifds: vec![ifd0, ifd1] };
    let bytes = write_tiff(&doc);

    // Prove the spliced file is genuinely readable back — both by this module's own
    // independent reader AND by the registered `image` decoder (IFD 0 only, its own scope).
    let reparsed = read_tiff(&bytes).expect("re-parse the derived multi-IFD fixture");
    assert_eq!(reparsed.ifds.len(), 2, "derived fixture must carry exactly two real IFDs");
    let (w, h, _) = decode_raster(&reparsed.ifds[0]).expect("decode IFD 0 raster");
    assert_eq!((w, h), (2275, 2560));
    let via_image = image::codecs::tiff::TiffDecoder::new(std::io::Cursor::new(&bytes)).expect("image crate: independently parse derived fixture");
    assert_eq!(image::ImageDecoder::dimensions(&via_image), (2275, 2560));

    std::fs::write(&out_path, &bytes).expect("write committed shared:// fixture");
    eprintln!("wrote {} ({} bytes)", out_path.display(), bytes.len());

    // A SMALL real thumbnail (8x8, genuine decoded+downsampled rathaus pixels — not
    // synthetic) reused inline as `insert-ifd`'s real embedded-IFD content in
    // the feature file's Examples table: printed as hex here rather than committed, since it's
    // small enough to live directly in the feature text (192 bytes = 384 hex chars).
    let tiny = image::imageops::thumbnail(&full, 8, 8);
    let tiny_rgb = image::DynamicImage::ImageRgba8(tiny).to_rgb8();
    let tiny_hex: String = tiny_rgb.as_raw().iter().map(|b| format!("{b:02x}")).collect();
    eprintln!("inline 8x8 real thumbnail hex (insert-ifd pixels): {tiny_hex}");

    // A committed `shared://` binary fixture for `replace-pixels`: the SAME real photo's own
    // pixels, horizontally flipped (still 100% real content, but a genuinely different,
    // provable raster) — full IFD 0 resolution, so it can only reasonably live as a binary
    // fixture, not inline JSON hex.
    let (w, h) = (photo.width(), photo.height());
    let mut flipped_rgba = Vec::with_capacity(w as usize * h as usize * 4);
    for y in 0..h {
        for x in 0..w {
            let px = photo.get_pixel(w - 1 - x, y);
            flipped_rgba.extend_from_slice(&[px[0], px[1], px[2], 255]);
        }
    }
    let mut case_fixture_dir = repo_root.clone();
    case_fixture_dir.push("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🧪️tests/mutate-tiff-6-0/🧫️fixtures");
    std::fs::create_dir_all(&case_fixture_dir).expect("create case fixtures dir");
    let mut flipped_path = case_fixture_dir.clone();
    flipped_path.push("🖼️.rgba");
    std::fs::write(&flipped_path, &flipped_rgba).expect("write shared:// replace-pixels fixture");
    eprintln!("wrote {} ({} bytes, {w}x{h} RGBA8)", flipped_path.display(), flipped_rgba.len());
}
