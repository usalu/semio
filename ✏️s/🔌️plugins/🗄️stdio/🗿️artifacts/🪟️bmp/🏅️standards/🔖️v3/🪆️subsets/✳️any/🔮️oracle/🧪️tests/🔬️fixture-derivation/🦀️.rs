
/// 🧭️ Walks up from `start` looking for the repo root's own `CLAUDE.md`.
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
    let mut png_path = repo_root.clone();
    png_path.push("🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️assets/🏛️rathaus-ahlen-grundriss/🖼️.png");
    let mut out_path = repo_root.clone();
    out_path.push("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🏛️rathaus-ahlen-grundriss/🖼️.bmp");

    let png_bytes = std::fs::read(&png_path).expect("read the real PNG floor plan");
    let mut reader = png::Decoder::new(std::io::Cursor::new(&png_bytes)).read_info().expect("png: read_info");
    let mut buffer = vec![0u8; reader.output_buffer_size().unwrap_or(0)];
    let frame = reader.next_frame(&mut buffer).expect("png: next_frame");
    assert_eq!(frame.color_type, png::ColorType::Indexed, "the source is an indexed PNG; a resolved one would lose the palette this fixture exists for");
    let table = reader.info().palette.clone().expect("indexed PNG carries a PLTE");
    let mut palette: Vec<[u8; 3]> = table.chunks_exact(3).map(|entry| [entry[0], entry[1], entry[2]]).collect();
    let indices = buffer[..frame.buffer_size()].to_vec();
    assert_eq!(palette.len(), 233, "the real PLTE moved — re-check the padding arithmetic below");

    let mut referenced = [false; 256];
    for index in &indices {
        referenced[*index as usize] = true;
    }
    assert!(referenced[..palette.len()].iter().all(|used| *used), "every real palette entry is referenced; the padding below is what creates the slack");

    let spare: Vec<[u8; 3]> = (0u16..=255).map(|value| [value as u8, 0, (255 - value) as u8]).filter(|candidate| !palette.contains(candidate)).take(7).collect();
    assert_eq!(spare.len(), 7, "the deterministic ramp must yield seven colours the real table does not already hold");
    palette.extend(spare);
    assert_eq!(palette.len(), 240);
    assert!(indices.iter().all(|index| (*index as usize) < 233), "no pixel may reference a spare entry");

    let mut bytes = Vec::new();
    image::codecs::bmp::BmpEncoder::new(&mut bytes).encode_with_palette(&indices, frame.width, frame.height, image::ExtendedColorType::L8, Some(&palette)).expect("image crate: write the indexed BMP");

    let reparsed = super::oracles::decode(&bytes).expect("re-parse the derived fixture with the independent reader");
    assert_eq!((reparsed.width, reparsed.height), (frame.width, frame.height));
    match &reparsed.content {
        super::oracles::Content::Indexed { indices: back, palette: table } => {
            assert_eq!(table.len(), 240, "the derived fixture must declare its full 240-entry table");
            assert_eq!(back, &indices, "the index buffer must survive the round trip through the reference encoder");
        }
        super::oracles::Content::Direct { .. } => panic!("the derived fixture decoded as direct-colour; the palette was lost"),
    }

    std::fs::write(&out_path, &bytes).expect("write the committed shared:// fixture");
    eprintln!("wrote {} ({} bytes, {}x{}, 240-entry table)", out_path.display(), bytes.len(), frame.width, frame.height);
}
