use super::*;

fn short(tag: u16, values: Vec<u16>) -> TiffTag { TiffTag { tag, values: TiffValues::Short(values) } }
fn long(tag: u16, value: u32) -> TiffTag { TiffTag { tag, values: TiffValues::Long(vec![value]) } }

fn rgb_snapshot(order: TiffByteOrder, chunks: Vec<Vec<u8>>) -> TiffSnapshot {
    TiffSnapshot {
        schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(),
        byte_order: order,
        ifds: vec![TiffIfd {
            entries: vec![
                long(TAG_IMAGE_WIDTH, 2), long(TAG_IMAGE_LENGTH, 1),
                short(TAG_BITS_PER_SAMPLE, vec![8, 8, 8]), short(TAG_COMPRESSION, vec![1]),
                short(TAG_PHOTOMETRIC, vec![2]), short(TAG_SAMPLES_PER_PIXEL, vec![3]),
                long(TAG_ROWS_PER_STRIP, 1),
            ],
            storage: TiffStorage { kind: TiffStorageKind::Strips, offsets_kind: TiffFieldType::Long, byte_counts_kind: TiffFieldType::Long, chunks },
        }],
    }
}

#[test]
fn canonical_strip_storage_round_trips_in_both_byte_orders() {
    for order in [TiffByteOrder::LittleEndian, TiffByteOrder::BigEndian] {
        let snapshot = rgb_snapshot(order, vec![vec![255, 0, 0, 0, 255, 0]]);
        let decoded = decode_tiff(&encode_tiff(&snapshot).expect("encode")).expect("decode");
        assert_eq!(decoded.byte_order, order);
        assert_eq!(decoded.ifds[0].storage, snapshot.ifds[0].storage);
        assert_eq!(decoded.ifds[0].entries, snapshot.ifds[0].entries);
    }
}

#[test]
fn png_projection_is_ephemeral_and_pixel_exact() {
    let snapshot = rgb_snapshot(TiffByteOrder::LittleEndian, vec![vec![255, 0, 0, 0, 255, 0]]);
    let page = decode_tiff_page_rgba(&snapshot, 0).expect("RGBA projection");
    assert_eq!((page.width, page.height), (2, 1));
    assert_eq!(page.pixels, vec![255, 0, 0, 255, 0, 255, 0, 255]);
    let png = encode_tiff_page_png(&snapshot, 0).expect("PNG projection");
    assert_eq!((png.width, png.height), (2, 1));
    assert_eq!(&png.bytes[..8], b"\x89PNG\r\n\x1a\n");
    assert_eq!(snapshot.ifds[0].storage.chunks, vec![vec![255, 0, 0, 0, 255, 0]]);
}

#[test]
fn split_strips_are_joined_in_authored_order() {
    let mut snapshot = rgb_snapshot(TiffByteOrder::LittleEndian, vec![vec![255, 0, 0], vec![0, 255, 0]]);
    snapshot.ifds[0].entries.iter_mut().find(|entry| entry.tag == TAG_IMAGE_WIDTH).expect("width").values = TiffValues::Long(vec![1]);
    snapshot.ifds[0].entries.iter_mut().find(|entry| entry.tag == TAG_IMAGE_LENGTH).expect("height").values = TiffValues::Long(vec![2]);
    assert_eq!(decode_tiff_page_rgba(&snapshot, 0).expect("RGBA").pixels, vec![255, 0, 0, 255, 0, 255, 0, 255]);
}

#[test]
fn unsupported_storage_is_refused_without_rewriting_source() {
    let mut snapshot = rgb_snapshot(TiffByteOrder::LittleEndian, vec![vec![1, 2, 3, 4, 5, 6]]);
    snapshot.ifds[0].storage.kind = TiffStorageKind::Tiles;
    let before = snapshot.clone();
    assert!(decode_tiff_page_rgba(&snapshot, 0).unwrap_err().contains("TileWidth"));
    assert_eq!(snapshot, before);
}

#[test]
fn malformed_strip_length_is_refused() {
    assert!(decode_tiff_page_rgba(&rgb_snapshot(TiffByteOrder::LittleEndian, vec![vec![1, 2, 3]]), 0).is_err());
}

const TILED_FIXTURE: &str = include_str!("../../../🧫️fixtures/🧬️tiled-8bit/🔣️.json");

fn fixture_u32(value: &serde_json::Value, key: &str) -> u32 {
    u32::try_from(value[key].as_u64().expect("fixture integer")).expect("fixture u32")
}

fn fixture_bytes(value: &serde_json::Value, key: &str) -> Vec<u8> {
    value[key].as_array().expect("fixture byte array").iter().map(|value| u8::try_from(value.as_u64().expect("fixture byte")).expect("fixture u8")).collect()
}

fn tiled_snapshot() -> TiffSnapshot {
    let fixture: serde_json::Value = serde_json::from_str(TILED_FIXTURE).expect("neutral tiled fixture");
    let width = fixture_u32(&fixture, "width");
    let height = fixture_u32(&fixture, "height");
    let tile_width = fixture_u32(&fixture, "tileWidth");
    let tile_length = fixture_u32(&fixture, "tileLength");
    let samples = fixture_u32(&fixture, "samplesPerPixel");
    let mut chunks = Vec::new();
    for tile in fixture["tiles"].as_array().expect("fixture tiles") {
        let color = fixture_bytes(tile, "rgb");
        let marker = u8::try_from(tile["paddingMarker"].as_u64().expect("padding marker")).expect("padding u8");
        let tile_row = fixture_u32(tile, "row");
        let tile_column = fixture_u32(tile, "column");
        let mut chunk = vec![marker; usize::try_from(tile_width * tile_length * samples).expect("tile bytes")];
        for local_y in 0..tile_length {
            for local_x in 0..tile_width {
                let x = tile_column * tile_width + local_x;
                let y = tile_row * tile_length + local_y;
                if x < width && y < height {
                    let offset = usize::try_from((local_y * tile_width + local_x) * samples).expect("sample offset");
                    chunk[offset..offset + 3].copy_from_slice(&color);
                }
            }
        }
        chunks.push(chunk);
    }
    let first = TiffIfd {
        entries: vec![
            long(TAG_IMAGE_WIDTH, width), long(TAG_IMAGE_LENGTH, height), short(TAG_BITS_PER_SAMPLE, vec![8, 8, 8]),
            short(TAG_COMPRESSION, vec![1]), short(TAG_PHOTOMETRIC, vec![2]), short(TAG_SAMPLES_PER_PIXEL, vec![3]),
            long(TAG_TILE_WIDTH, tile_width), long(TAG_TILE_LENGTH, tile_length), TiffTag { tag: 65000, values: TiffValues::Undefined(vec![4, 2, 4, 2]) },
        ],
        storage: TiffStorage { kind: TiffStorageKind::Tiles, offsets_kind: TiffFieldType::Long, byte_counts_kind: TiffFieldType::Long, chunks },
    };
    let second = rgb_snapshot(TiffByteOrder::LittleEndian, vec![vec![1, 2, 3, 4, 5, 6]]).ifds.remove(0);
    TiffSnapshot { schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order: TiffByteOrder::LittleEndian, ifds: vec![first, second] }
}

fn single_tile_snapshot(samples: u16, photometric: u16, extra_samples: Option<Vec<u16>>, pixel: &[u8]) -> TiffSnapshot {
    let mut entries = vec![
        long(TAG_IMAGE_WIDTH, 16),
        long(TAG_IMAGE_LENGTH, 16),
        short(TAG_BITS_PER_SAMPLE, vec![8; usize::from(samples)]),
        short(TAG_COMPRESSION, vec![1]),
        short(TAG_PHOTOMETRIC, vec![photometric]),
        short(TAG_SAMPLES_PER_PIXEL, vec![samples]),
        long(TAG_TILE_WIDTH, 16),
        long(TAG_TILE_LENGTH, 16),
    ];
    if let Some(extra_samples) = extra_samples { entries.push(short(338, extra_samples)); }
    let mut chunk = Vec::with_capacity(16 * 16 * pixel.len());
    for _ in 0..16 * 16 { chunk.extend_from_slice(pixel); }
    TiffSnapshot {
        schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(),
        byte_order: TiffByteOrder::LittleEndian,
        ifds: vec![TiffIfd { entries, storage: TiffStorage { kind: TiffStorageKind::Tiles, offsets_kind: TiffFieldType::Long, byte_counts_kind: TiffFieldType::Long, chunks: vec![chunk] } }],
    }
}

#[test]
fn neutral_tiled_8bit_projection_is_pixel_exact_and_preserves_canonical_source() {
    let fixture: serde_json::Value = serde_json::from_str(TILED_FIXTURE).expect("neutral tiled fixture");
    let snapshot = tiled_snapshot();
    let before = snapshot.clone();
    let page = decode_tiff_page_rgba(&snapshot, 0).expect("tiled RGBA projection");
    assert_eq!((page.width, page.height), (fixture_u32(&fixture, "width"), fixture_u32(&fixture, "height")));
    for sample in fixture["expectedSamples"].as_array().expect("expected samples") {
        let x = fixture_u32(sample, "x");
        let y = fixture_u32(sample, "y");
        let offset = usize::try_from((y * page.width + x) * 4).expect("RGBA offset");
        assert_eq!(&page.pixels[offset..offset + 4], fixture_bytes(sample, "rgba"));
    }
    assert_eq!(snapshot, before, "ephemeral projection changed canonical storage");
    assert_eq!(decode_tiff(&encode_tiff(&snapshot).expect("encode tiled")).expect("decode tiled"), snapshot);
}

#[test]
fn tiled_region_paint_is_revision_guarded_cancellable_and_preserves_other_authority() {
    let fixture: serde_json::Value = serde_json::from_str(TILED_FIXTURE).expect("neutral tiled fixture");
    let before = tiled_snapshot();
    let edit = &fixture["edit"];
    let region = TiffRegion { x: fixture_u32(edit, "x"), y: fixture_u32(edit, "y"), width: fixture_u32(edit, "width"), height: fixture_u32(edit, "height") };
    let color: [u8; 4] = fixture_bytes(edit, "rgba").try_into().expect("RGBA fixture");
    let revision = tiff_revision(&before);
    let mut calls = Vec::new();
    let after = paint_tiff_region_controlled(&before, &revision, 0, region, color, &mut |done, total| { calls.push((done, total)); true }).expect("tile paint");
    assert_eq!(after.ifds[0].entries, before.ifds[0].entries, "paint changed authored tags");
    assert_eq!(after.ifds[1], before.ifds[1], "paint changed another page");
    assert_eq!(after.ifds[0].storage.chunks.len(), before.ifds[0].storage.chunks.len(), "paint changed tile partition");
    assert_eq!(calls.last(), Some(&(usize::try_from(region.height).unwrap(), usize::try_from(region.height).unwrap())));
    let page = decode_tiff_page_rgba(&after, 0).expect("edited projection");
    for y in region.y..region.y + region.height {
        for x in region.x..region.x + region.width {
            let offset = usize::try_from((y * page.width + x) * 4).expect("RGBA offset");
            assert_eq!(&page.pixels[offset..offset + 4], color);
        }
    }
    assert!(paint_tiff_region_controlled(&before, "stale", 0, region, color, &mut |_, _| true).unwrap_err().contains("revision"));
    assert!(paint_tiff_region_controlled(&before, &revision, 0, region, color, &mut |_, _| false).unwrap_err().contains("cancelled"));
    assert_eq!(before, tiled_snapshot(), "failed paint changed its input");
}

#[test]
fn image_rs_independently_decodes_tiled_save_and_edited_samples() {
    let before = tiled_snapshot();
    let revision = tiff_revision(&before);
    let edited = paint_tiff_region_controlled(&before, &revision, 0, TiffRegion { x: 16, y: 16, width: 1, height: 1 }, [9, 8, 7, 255], &mut |_, _| true).expect("tile paint");
    let encoded = encode_tiff(&edited).expect("encode tiled edit");
    let image = image::load_from_memory_with_format(&encoded, image::ImageFormat::Tiff).expect("image-rs tiled decode").to_rgba8();
    assert_eq!(image.dimensions(), (17, 17));
    assert_eq!(image.get_pixel(0, 0).0, [255, 0, 0, 255]);
    assert_eq!(image.get_pixel(16, 16).0, [9, 8, 7, 255]);
}

#[test]
fn packbits_tiled_projection_and_image_rs_oracle_match_uncompressed_pixels() {
    let plain = tiled_snapshot();
    let expected = decode_tiff_page_rgba(&plain, 0).expect("plain tiled projection");
    let mut packed = plain.clone();
    packed.ifds[0].entries.iter_mut().find(|entry| entry.tag == TAG_COMPRESSION).expect("compression tag").values = TiffValues::Short(vec![32773]);
    packed.ifds[0].storage.chunks = plain.ifds[0].storage.chunks.iter().map(|chunk| packbits_encode(chunk)).collect();
    assert_eq!(decode_tiff_page_rgba(&packed, 0).expect("PackBits tiled projection"), expected);
    let encoded = encode_tiff(&packed).expect("encode PackBits tiled TIFF");
    let image = image::load_from_memory_with_format(&encoded, image::ImageFormat::Tiff).expect("image-rs PackBits tiled decode").to_rgba8();
    assert_eq!(image.dimensions(), (17, 17));
    assert_eq!(image.get_pixel(16, 0).0, [0, 255, 0, 255]);
    assert!(validate_tiff_region_paint(&packed, 0, TiffRegion { x: 0, y: 0, width: 1, height: 1 }, [1, 2, 3, 255]).unwrap_err().contains("uncompressed"));
}

#[test]
fn tiled_white_is_zero_and_unassociated_alpha_profiles_project_and_paint_exact_samples() {
    let white_is_zero = single_tile_snapshot(1, 0, None, &[0]);
    assert_eq!(&decode_tiff_page_rgba(&white_is_zero, 0).expect("white-is-zero projection").pixels[..4], &[255, 255, 255, 255]);
    let painted_gray = paint_tiff_region_controlled(&white_is_zero, &tiff_revision(&white_is_zero), 0, TiffRegion { x: 0, y: 0, width: 1, height: 1 }, [10, 10, 10, 255], &mut |_, _| true).expect("white-is-zero paint");
    assert_eq!(painted_gray.ifds[0].storage.chunks[0][0], 245);
    assert_eq!(&decode_tiff_page_rgba(&painted_gray, 0).expect("painted grayscale projection").pixels[..4], &[10, 10, 10, 255]);

    let rgba = single_tile_snapshot(4, 2, Some(vec![2]), &[1, 2, 3, 4]);
    assert_eq!(&decode_tiff_page_rgba(&rgba, 0).expect("unassociated alpha projection").pixels[..4], &[1, 2, 3, 4]);
    let painted_rgba = paint_tiff_region_controlled(&rgba, &tiff_revision(&rgba), 0, TiffRegion { x: 0, y: 0, width: 1, height: 1 }, [5, 6, 7, 8], &mut |_, _| true).expect("unassociated alpha paint");
    assert_eq!(&painted_rgba.ifds[0].storage.chunks[0][..4], &[5, 6, 7, 8]);
    assert_eq!(&decode_tiff_page_rgba(&painted_rgba, 0).expect("painted alpha projection").pixels[..4], &[5, 6, 7, 8]);
}
