
use super::*;
use crate::DwgSnapshot;
use crate::STDIO_DWG_DOCUMENT_SCHEMA;

#[semio_framework_async_macros::async_test]
async fn empty_snapshot_matches_schema() {
    let snapshot = crate::standards::v_ac1024::engine::empty_dwg_snapshot();
    assert_eq!(snapshot.schema, STDIO_DWG_DOCUMENT_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn codec_round_trip() {
    let bytes = dwg_to_bytes(&DwgDrawing::default()).expect("encode empty drawing");
    let snap = crate::schema::snapshot::decode_dwg(&bytes).expect("decode structural drawing");
    let text = store::ArtifactDsl::print_dsl(&snap);
    let parsed = <DwgSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(parsed.version, "AC1015");
    let bytes = store::ArtifactPack::encode_pack(&snap);
    let decoded = <DwgSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, snap);
}

//#region 🔖️RelocatedDwgCodecUnit
/// 🧪️ Relocated verbatim from `🧰️framework/🔨️modules/🔺️mesh/🦀️.rs`'s own
/// `#[cfg(test)] mod tests` (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-
/// ARTIFACTS G2) — the 9 tests that actually exercised the DWG codec now living in
/// `DwgStructuralCodec` above (the file's other 20 tests exercised `semio_framework_mesh_engine`
/// itself, orphaned in that file since its own mesh content dissolved into that crate; those
/// moved to `semio-framework-mesh-engine`'s own package glue, not here).
#[semio_framework_async_macros::async_test]
async fn dwg_bit_primitives_round_trip_at_unaligned_offsets() {
    let mut writer = DwgBitWriter::new();
    writer.write_bit(true);
    writer.write_bit(false);
    writer.write_bit(true);
    writer.write_bs(0);
    writer.write_bs(256);
    writer.write_bs(42);
    writer.write_bs(12345);
    writer.write_bl(0);
    writer.write_bl(200);
    writer.write_bl(70000);
    writer.write_bd(0.0);
    writer.write_bd(1.0);
    writer.write_bd(3.14159);
    writer.write_ms(70000);
    writer.write_handle(5, 0x1234);
    writer.write_t("héllo");
    writer.pad_to_byte();

    let mut reader = DwgBitReader::new(&writer.bytes);
    assert!(reader.read_bit().unwrap());
    assert!(!reader.read_bit().unwrap());
    assert!(reader.read_bit().unwrap());
    assert_eq!(reader.read_bs().unwrap(), 0);
    assert_eq!(reader.read_bs().unwrap(), 256);
    assert_eq!(reader.read_bs().unwrap(), 42);
    assert_eq!(reader.read_bs().unwrap(), 12345);
    assert_eq!(reader.read_bl().unwrap(), 0);
    assert_eq!(reader.read_bl().unwrap(), 200);
    assert_eq!(reader.read_bl().unwrap(), 70000);
    assert_eq!(reader.read_bd().unwrap(), 0.0);
    assert_eq!(reader.read_bd().unwrap(), 1.0);
    assert_eq!(reader.read_bd().unwrap(), 3.14159);
    assert_eq!(reader.read_ms().unwrap(), 70000);
    assert_eq!(reader.read_handle().unwrap(), (5, 0x1234));
    assert_eq!(reader.read_t().unwrap(), "héllo");
}

#[semio_framework_async_macros::async_test]
async fn dwg_crc16_matches_seed_on_empty_input() {
    assert_eq!(dwg_crc16(0xC0C1, &[]), 0xC0C1);
    assert_ne!(dwg_crc16(0xC0C1, &[1, 2, 3]), 0xC0C1);
}

#[semio_framework_async_macros::async_test]
async fn dwg_writer_produces_a_structurally_valid_container() {
    let bytes = dwg_to_bytes(&DwgDrawing::default()).expect("encode empty drawing");
    assert_eq!(&bytes[0..6], b"AC1015");
    let section_count = u32::from_le_bytes(bytes[6..10].try_into().unwrap());
    assert_eq!(section_count, 3);
    assert_eq!(&bytes[DWG_FILE_HEADER_LEN - 16..DWG_FILE_HEADER_LEN], &DWG_SENTINEL_FILE_HEADER_END);
}

#[semio_framework_async_macros::async_test]
async fn dwg_full_entity_set_round_trips() {
    let mut drawing = DwgDrawing::default();
    let layer_a = drawing.ensure_layer("outline");
    let layer_b = drawing.ensure_layer("solids");
    drawing.entities.push(DwgEntity { layer: layer_a, color: DwgColor::Index(3), geometry: DwgGeometry::Line { start: [0.0, 0.0, 0.0], end: [10.0, 5.0, 0.0] } });
    drawing.entities.push(DwgEntity { layer: layer_a, color: DwgColor::ByLayer, geometry: DwgGeometry::Point { at: [1.0, 2.0, 3.0] } });
    drawing.entities.push(DwgEntity { layer: layer_a, color: DwgColor::ByBlock, geometry: DwgGeometry::Circle { center: [0.0, 0.0, 0.0], radius: 5.0, normal: [0.0, 0.0, 1.0] } });
    drawing.entities.push(DwgEntity { layer: layer_a, color: DwgColor::Index(1), geometry: DwgGeometry::Arc { center: [0.0, 0.0, 0.0], radius: 3.0, start_angle: 0.0, end_angle: 1.57, normal: [0.0, 0.0, 1.0] } });
    drawing.entities.push(DwgEntity { layer: layer_a, color: DwgColor::Index(2), geometry: DwgGeometry::Ellipse { center: [1.0, 1.0, 0.0], major_axis: [4.0, 0.0, 0.0], ratio: 0.5, start_param: 0.0, end_param: 6.28, normal: [0.0, 0.0, 1.0] } });
    drawing.entities.push(DwgEntity { layer: layer_a, color: DwgColor::ByLayer, geometry: DwgGeometry::LwPolyline { closed: true, elevation: 0.0, vertices: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0]], bulges: vec![0.0, 0.5, 0.0] } });
    drawing.entities.push(DwgEntity {
        layer: layer_a,
        color: DwgColor::ByLayer,
        geometry: DwgGeometry::Spline { degree: 3, control_points: vec![[0.0, 0.0, 0.0], [1.0, 2.0, 0.0], [3.0, 2.0, 0.0], [4.0, 0.0, 0.0]], knots: vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], weights: vec![1.0; 4] },
    });
    drawing.entities.push(DwgEntity { layer: layer_a, color: DwgColor::ByLayer, geometry: DwgGeometry::Text { at: [0.0, 0.0, 0.0], height: 2.5, rotation: 0.0, content: "semio".to_string() } });
    drawing.entities.push(DwgEntity { layer: layer_b, color: DwgColor::ByLayer, geometry: DwgGeometry::Face3d { corners: [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]] } });
    drawing.entities.push(DwgEntity { layer: layer_b, color: DwgColor::ByLayer, geometry: DwgGeometry::Polyline3d { closed: false, vertices: vec![[0.0, 0.0, 0.0], [0.0, 0.0, 5.0], [1.0, 0.0, 5.0]] } });
    drawing.entities.push(DwgEntity { layer: layer_b, color: DwgColor::ByLayer, geometry: DwgGeometry::PolyfaceMesh { vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]], faces: vec![[1, 2, 3, 4]] } });

    let bytes = dwg_to_bytes(&drawing).expect("encode");
    let decoded = dwg_from_bytes(&bytes).expect("decode");

    assert_eq!(decoded.entities.len(), drawing.entities.len());
    assert_eq!(decoded.layers.len(), drawing.layers.len());
    for (original, round_tripped) in drawing.entities.iter().zip(decoded.entities.iter()) {
        assert_eq!(original.geometry, round_tripped.geometry);
        assert_eq!(original.color, round_tripped.color);
        assert_eq!(drawing.layers[original.layer].name, decoded.layers[round_tripped.layer].name);
    }
}

#[semio_framework_async_macros::async_test]
async fn dwg_mesh_bridge_round_trips_triangle_count_and_positions() {
    let mesh = semio_framework_mesh_engine::mesh_box(2.0, 2.0, 2.0);
    let drawing = mesh_to_dwg_drawing(&mesh);
    let bytes = dwg_to_bytes(&drawing).expect("encode");
    let decoded_drawing = dwg_from_bytes(&bytes).expect("decode");
    let decoded_mesh = dwg_drawing_to_mesh(&decoded_drawing);
    assert_eq!(decoded_mesh.triangle_count(), mesh.triangle_count());
    assert_eq!(decoded_mesh.vertex_count(), mesh.vertex_count());
}

#[semio_framework_async_macros::async_test]
async fn dwg_path_bridge_round_trips_cubic_control_points_exactly() {
    let paths = vec![vec![DwgPathSegment::Move { to: [0.0, 0.0] }, DwgPathSegment::Line { to: [5.0, 0.0] }, DwgPathSegment::Cubic { ctrl1: [6.0, 1.0], ctrl2: [7.0, 3.0], to: [5.0, 4.0] }, DwgPathSegment::Close]];
    let drawing = paths_to_dwg_drawing(&paths);
    let bytes = dwg_to_bytes(&drawing).expect("encode");
    let decoded = dwg_from_bytes(&bytes).expect("decode");
    let round_tripped_paths = dwg_drawing_to_paths(&decoded);

    let cubic_found = round_tripped_paths.iter().flatten().any(|segment| {
        matches!(segment, DwgPathSegment::Cubic { ctrl1, ctrl2, to }
                if (ctrl1[0] - 6.0).abs() < 1e-9 && (ctrl2[1] - 3.0).abs() < 1e-9 && (to[1] - 4.0).abs() < 1e-9)
    });
    assert!(cubic_found, "expected the exact cubic control points to survive the dwg round trip");

    let line_found = round_tripped_paths.iter().flatten().any(|segment| matches!(segment, DwgPathSegment::Line { to } if (to[0] - 5.0).abs() < 1e-9));
    assert!(line_found, "expected the polyline segment to survive the dwg round trip");
}

#[semio_framework_async_macros::async_test]
async fn dwg_rejects_unsupported_version() {
    let mut bytes = dwg_to_bytes(&DwgDrawing::default()).expect("encode");
    bytes[0..6].copy_from_slice(b"AC1018");
    let err = dwg_from_bytes(&bytes).expect_err("should reject non-R2000 version");
    assert!(err.contains("AC1018"));
}

#[semio_framework_async_macros::async_test]
async fn dwg_reader_skips_unknown_object_types_without_failing() {
    let mut drawing = DwgDrawing::default();
    let layer = drawing.ensure_layer("0");
    drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::Point { at: [1.0, 1.0, 1.0] } });
    let mut bytes = dwg_to_bytes(&drawing).expect("encode");

    let mut bogus_body = DwgBitWriter::new();
    bogus_body.write_rc(0xFF);
    let mut bogus_handles = DwgBitWriter::new();
    let bogus_offset = bytes.len();
    dwg_write_object(&mut bytes, 900, 0x9999, &mut bogus_body, &mut bogus_handles);

    let map_locator_pos = 10 + 2 * 9;
    let map_offset = u32::from_le_bytes(bytes[map_locator_pos + 1..map_locator_pos + 5].try_into().unwrap());
    let map_size = u32::from_le_bytes(bytes[map_locator_pos + 5..map_locator_pos + 9].try_into().unwrap());
    let mut new_entry = Vec::new();
    new_entry.extend_from_slice(&0x9999u64.to_le_bytes());
    new_entry.extend_from_slice(&((bogus_offset + 16) as u64).to_le_bytes());
    let insert_at = map_offset as usize + 4;
    for (i, b) in new_entry.iter().enumerate() {
        bytes.insert(insert_at + i, *b);
    }
    let new_count = u32::from_le_bytes(bytes[map_offset as usize..map_offset as usize + 4].try_into().unwrap()) + 1;
    bytes[map_offset as usize..map_offset as usize + 4].copy_from_slice(&new_count.to_le_bytes());
    let new_size = map_size + new_entry.len() as u32;
    bytes[map_locator_pos + 5..map_locator_pos + 9].copy_from_slice(&new_size.to_le_bytes());

    let decoded = dwg_from_bytes(&bytes).expect("reader should tolerate the unknown object type");
    assert_eq!(decoded.entities.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn dwg_ensure_layer_reuses_existing_index_and_appends_new_ones() {
    let mut drawing = DwgDrawing::default();
    let outline = drawing.ensure_layer("outline");
    let outline_again = drawing.ensure_layer("outline");
    let solids = drawing.ensure_layer("solids");
    assert_eq!(outline, outline_again);
    assert_ne!(outline, solids);
    assert_eq!(drawing.layers.len(), 2);
}
//#endregion 🔖️RelocatedDwgCodecUnit

//#region 🔖️Lz77VariantUnit
#[semio_framework_async_macros::async_test]
async fn lcg_decrypt_is_its_own_inverse() {
    let plain: Vec<u8> = (0..R2004_HEADER_LEN as u8).collect();
    let enc = decrypt_r2004_header(&plain);
    let dec = decrypt_r2004_header(&enc);
    assert_eq!(dec, plain);
}

#[semio_framework_async_macros::async_test]
async fn lz_round_trip_literal_only_stream() {
    // opcode low-nibble 3 -> literal run length 3+3=6, followed by 6 literal bytes, then the
    // 0x11 terminator (read as the "next opcode" by `copy_bytes`'s trailing byte read).
    let mut comp = vec![0x03u8];
    comp.extend_from_slice(b"abcdef");
    comp.push(0x11);
    let out = decompress_r2004_section(&comp, 64).expect("decompress");
    assert_eq!(&out, b"abcdef");
}

#[semio_framework_async_macros::async_test]
async fn lz_writer_roundtrips_every_literal_length_boundary() {
    for length in [4usize, 18, 19, 20, 272, 273, 274, 527, 528, 4096] {
        let input: Vec<u8> = (0..length).map(|index| index as u8).collect();
        let encoded = compress_r2004_section(&input).expect("compress");
        let decoded = decompress_r2004_section(&encoded, input.len()).expect("decompress");
        assert_eq!(decoded, input, "literal length {length}");
    }
}

#[semio_framework_async_macros::async_test]
async fn page_checksum_supports_seeded_stages() {
    assert_eq!(r2004_page_checksum(0, b""), 0);
    let header = r2004_page_checksum(0, b"header");
    assert_eq!(r2004_page_checksum(header, b"payload"), 0x250a0553);
}

#[semio_framework_async_macros::async_test]
async fn lz_rejects_out_of_bounds_backref() {
    // opcode 0x40 (short-match branch, comp_bytes=(0x40>>4)-1=3) with a huge encoded offset
    // and nothing decoded yet -- must error, never panic or fabricate bytes.
    let comp = vec![0x40u8, 0xFFu8];
    let err = decompress_r2004_section(&comp, 16);
    assert!(err.is_err(), "backref past the start of output must be a typed error");
}
//#endregion 🔖️Lz77VariantUnit

//#region 🔖️RealFixture
const ARCHITECTURAL_FIXTURE: &[u8] = include_bytes!("../../../../../../../../../../../../temp/architectural_example.dwg");

/// 🧪️ D1: file header decrypts cleanly and every section+page is located by name, on the
/// real ~145KB AC1024 fixture -- the actual regression test for "sentinel + passthrough"
/// (the pre-ticket behavior, which never found a single real section on this file).
#[semio_framework_async_macros::async_test]
async fn real_fixture_d1_locates_every_named_section() {
    let sections = locate_r2004_sections(ARCHITECTURAL_FIXTURE).expect("D1 section location");
    let expected_names =
        ["AcDb:Header", "AcDb:AuxHeader", "AcDb:Classes", "AcDb:Handles", "AcDb:Template", "AcDb:ObjFreeSpace", "AcDb:AcDbObjects", "AcDb:RevHistory", "AcDb:SummaryInfo", "AcDb:Preview", "AcDb:AppInfo", "AcDb:AppInfoHistory", "AcDb:FileDepList"];
    for name in expected_names {
        assert!(sections.iter().any(|s| s.name == name), "missing real section {name}");
    }
    // Every located page must carry a real, in-bounds file address and nonzero decoded
    // size -- proof this is genuine location, not a stub returning empty placeholders.
    for section in &sections {
        assert!(!section.pages.is_empty(), "section {} has no pages", section.name);
        for page in &section.pages {
            assert!(page.file_address > 0, "section {} page {} has null address", section.name, page.page_number);
            assert!(!page.decoded.is_empty(), "section {} page {} decoded to zero bytes", section.name, page.page_number);
        }
    }
}

/// 🧪️ D2: every located section's page content actually decompresses (or, for stored
/// sections, copies) into nonzero real bytes -- the genuine "not just located but decoded"
/// bar. `AcDb:Header`/`AcDb:Classes`/`AcDb:Handles` are asserted individually since they're
/// the sections D4/D5 (stretch) would need to interpret further.
#[semio_framework_async_macros::async_test]
async fn real_fixture_d2_decompresses_every_section() {
    let sections = decode_r2004_sections(ARCHITECTURAL_FIXTURE).expect("D2 section decode");
    let mut any_errors = Vec::new();
    for section in &sections {
        for page in &section.pages {
            if let Some(err) = &page.error {
                any_errors.push(format!("{}[{}]: {err}", section.name, page.page_number));
            } else {
                assert!(!page.decoded.is_empty(), "section {} page {} decoded to zero bytes", section.name, page.page_number);
            }
        }
    }
    assert!(any_errors.is_empty(), "D2 page decode errors on real fixture: {any_errors:?}");

    for must_have in ["AcDb:Header", "AcDb:Classes", "AcDb:Handles"] {
        let s = sections.iter().find(|s| s.name == must_have).unwrap_or_else(|| panic!("{must_have} missing"));
        let total: usize = s.pages.iter().map(|p| p.decoded.len()).sum();
        assert!(total > 0, "{must_have} decoded to zero total bytes");
    }
}

#[semio_framework_async_macros::async_test]
async fn real_fixture_r2010_object_frames_are_logically_identified() {
    let sections = decode_r2004_sections(ARCHITECTURAL_FIXTURE).expect("D2 section decode");
    let inventory = r2010_object_inventory(&sections).expect("R2010 object framing");
    let mut counts = std::collections::BTreeMap::new();
    for (_, object_type) in &inventory {
        *counts.entry(*object_type).or_insert(0usize) += 1;
    }
    assert!(inventory.len() > 100, "real fixture must expose its standard object frames");
    assert!(counts.contains_key(&DWG_TYPE_LAYER), "real fixture must contain layer records");
}

#[semio_framework_async_macros::async_test]
async fn real_fixture_classes_roundtrip_as_logical_records() {
    let sections = decode_r2004_sections(ARCHITECTURAL_FIXTURE).expect("D2 section decode");
    let section = sections.iter().find(|section| section.name == "AcDb:Classes").expect("classes section");
    let classes = decode_r2010_classes_section(&r2004_section_data(section).expect("class data")).expect("typed classes");
    assert!(!classes.is_empty(), "real fixture must contain dynamic class records");
    let encoded = encode_r2010_classes_section(&classes).expect("canonical classes");
    let reconstructed = decode_r2010_classes_section(&encoded).expect("canonical class decode");
    assert_eq!(reconstructed, classes);
}

#[semio_framework_async_macros::async_test]
async fn real_fixture_named_sections_roundtrip_as_logical_records() {
    let sections = decode_r2004_sections(ARCHITECTURAL_FIXTURE).expect("D2 section decode");
    let document = decode_r2004_document_sections(ARCHITECTURAL_FIXTURE).expect("typed document sections");
    let original = |name: &str| r2004_section_data(sections.iter().find(|section| section.name == name).unwrap_or_else(|| panic!("{name} section"))).expect("section data");
    assert_eq!(encode_summary_info(&document.summary).expect("summary encode"), original("AcDb:SummaryInfo"));
    assert_eq!(encode_application_info(&document.application).expect("application encode"), original("AcDb:AppInfo"));
    assert_eq!(encode_dependencies(&document.dependencies).expect("dependencies encode"), original("AcDb:FileDepList"));
    assert_eq!(encode_template(&document.template).expect("template encode"), original("AcDb:Template"));
}

/// 🧪️ D1 cross-validation: the page directory's total cumulative size must independently
/// match the file header's own `last_section_address` field (decrypted from a completely
/// different byte range) -- if the LZ decompressor or page-directory parser silently
/// produced wrong-but-plausible-looking output, this arithmetic identity would not hold.
#[semio_framework_async_macros::async_test]
async fn real_fixture_page_directory_matches_header_cross_check() {
    let enc = &ARCHITECTURAL_FIXTURE[0x80..0x80 + R2004_HEADER_LEN];
    let hdr = parse_r2004_file_header(&decrypt_r2004_header(enc)).expect("header decrypt");
    let map_hdr_addr = (hdr.section_map_address + 0x100) as usize;
    let map_decomp_size = u32::from_le_bytes(ARCHITECTURAL_FIXTURE[map_hdr_addr + 4..map_hdr_addr + 8].try_into().unwrap()) as usize;
    let map_comp_size = u32::from_le_bytes(ARCHITECTURAL_FIXTURE[map_hdr_addr + 8..map_hdr_addr + 12].try_into().unwrap()) as usize;
    let map_data_start = map_hdr_addr + 0x14;
    let map_dec = decompress_r2004_section(&ARCHITECTURAL_FIXTURE[map_data_start..map_data_start + map_comp_size], map_decomp_size).unwrap();
    let page_dir = parse_page_directory(&map_dec, hdr.section_array_size);
    assert_eq!(page_dir.len() as u32, hdr.numgaps + hdr.numsections);

    // Independent re-derivation straight from the decompressed bytes (not reusing
    // `parse_page_directory`'s own running-address bookkeeping) as the actual cross-check.
    let mut pos = 0usize;
    let mut total: u64 = 0x100;
    while pos + 8 <= map_dec.len() {
        let number = i32::from_le_bytes(map_dec[pos..pos + 4].try_into().unwrap());
        let size = u32::from_le_bytes(map_dec[pos + 4..pos + 8].try_into().unwrap());
        pos += 8;
        if number <= hdr.section_array_size as i32 {
            total += size as u64;
        }
        if number < 0 && pos + 16 <= map_dec.len() {
            pos += 16;
        }
    }
    assert_eq!(total, hdr.last_section_address + 0x100, "page directory total size must match independent header field");
}

/// 🔁 Exact imported bytes survive every persisted snapshot/diff/mutation/raw-I/O route.
#[semio_framework_async_macros::async_test]
async fn well_known_fixture_lossless_system_roundtrip() {
    use crate::schema::diff::DwgDiff;
    use crate::schema::mutations::{DwgMutation, apply_dwg_mutation, set_snapshot, set_version_info};
    use crate::schema::snapshot::encode_dwg;
    use protocol::command::DiffAlgebra;
    use protocol::{DiffCodec, Mutation, MutationDiff, OpBinary, OpText};
    use semio_s_artifact_stdio_binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};

    assert_eq!(ARCHITECTURAL_FIXTURE.len(), 148_638);
    assert_eq!(&ARCHITECTURAL_FIXTURE[..6], b"AC1024");
    let snapshot = crate::schema::snapshot::decode_dwg(ARCHITECTURAL_FIXTURE).expect("import fixture");
    assert_eq!(encode_dwg(&snapshot).expect("direct export"), ARCHITECTURAL_FIXTURE);

    let raw = BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes: ARCHITECTURAL_FIXTURE.to_vec() };
    let raw_snapshot = crate::standards::v_ac1024::subsets::any::io::import::deserializers::artifacts::binary::v_raw::any::deserialize(&raw).expect("raw deserialize");
    let raw_export = crate::standards::v_ac1024::subsets::any::io::export::serializers::artifacts::binary::v_raw::any::serialize(&raw_snapshot).expect("raw serialize");
    assert_eq!(raw_export.bytes, ARCHITECTURAL_FIXTURE);

    let dsl = store::ArtifactDsl::print_dsl(&snapshot);
    let fixture_hex: String = ARCHITECTURAL_FIXTURE.iter().map(|byte| format!("{byte:02x}")).collect();
    assert!(!dsl.contains("physical"));
    assert!(!dsl.contains(&fixture_hex), "DSL must serialize typed snapshot state, not a native DWG hex replay");
    let dsl_snapshot = <DwgSnapshot as store::ArtifactDsl>::parse_dsl(&dsl).expect("DSL parse");
    assert_eq!(encode_dwg(&dsl_snapshot).expect("DSL export"), ARCHITECTURAL_FIXTURE);

    let pack = store::ArtifactPack::encode_pack(&snapshot);
    assert!(!pack.windows(ARCHITECTURAL_FIXTURE.len()).any(|window| window == ARCHITECTURAL_FIXTURE), "pack must serialize typed snapshot state, not embed the native DWG document",);
    let pack_snapshot = <DwgSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("pack decode");
    assert_eq!(encode_dwg(&pack_snapshot).expect("pack export"), ARCHITECTURAL_FIXTURE);

    let self_diff = DwgDiff::between(&snapshot, &snapshot);
    assert!(self_diff.is_empty());
    assert_eq!(encode_dwg(&self_diff.apply(&snapshot).expect("self-diff must apply")).expect("self-diff export"), ARCHITECTURAL_FIXTURE);

    let mut no_op_snapshot = snapshot.clone();
    let no_op_diff = apply_dwg_mutation(&mut no_op_snapshot, &DwgMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: Box::new(snapshot.clone()) }));
    assert!(no_op_diff.diff().is_empty());
    assert_eq!(encode_dwg(&no_op_snapshot).expect("no-op export"), ARCHITECTURAL_FIXTURE);

    let set_snapshot = DwgMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: Box::new(snapshot.clone()) });
    let set_text = set_snapshot.print_op();
    let set_from_text = DwgMutation::parse_op(&set_text).expect("set-snapshot text decode");
    let set_binary = set_snapshot.encode_op().expect("set-snapshot binary encode");
    let set_from_binary = DwgMutation::decode_op(&set_binary).expect("set-snapshot binary decode");
    assert_eq!(set_from_text, set_snapshot);
    assert_eq!(set_from_binary, set_snapshot);
    let mut applied_set = DwgSnapshot::default();
    apply_dwg_mutation(&mut applied_set, &set_from_binary);
    assert_eq!(encode_dwg(&applied_set).expect("set-snapshot export"), ARCHITECTURAL_FIXTURE);

    let persisted_diff = DwgDiff::between(&DwgSnapshot::default(), &snapshot);
    let diff_text = persisted_diff.print_diff();
    assert_eq!(DwgDiff::parse_diff(&diff_text).expect("diff text decode"), persisted_diff);
    let diff_binary = persisted_diff.encode_diff().expect("diff binary encode");
    let decoded_diff = DwgDiff::decode_diff(&diff_binary).expect("diff binary decode");
    let from_persisted_diff = decoded_diff.apply(&DwgSnapshot::default()).expect("persisted diff must apply");
    assert_eq!(encode_dwg(&from_persisted_diff).expect("diff export"), ARCHITECTURAL_FIXTURE);

    let mut absorbed = persisted_diff.clone();
    absorbed.absorb(DwgDiff::between(&snapshot, &snapshot));
    assert_eq!(encode_dwg(&absorbed.apply(&DwgSnapshot::default()).expect("absorbed diff must apply")).expect("absorbed export"), ARCHITECTURAL_FIXTURE,);

    let header_mutation = DwgMutation::SetVersionInfo(set_version_info::SetVersionInfo { version: "AC1024".into(), maintenance_version: snapshot.maintenance_version.wrapping_add(1), codepage: 1252 });
    let mut header_snapshot = snapshot.clone();
    apply_dwg_mutation(&mut header_snapshot, &header_mutation);
    let header_bytes = encode_dwg(&header_snapshot).expect("supported header export");
    assert_ne!(header_bytes, ARCHITECTURAL_FIXTURE);
    assert_eq!(header_bytes[0x12], header_snapshot.maintenance_version);
    assert_eq!(u16::from_le_bytes([header_bytes[0x13], header_bytes[0x14]]), 1252);
    for inverse in header_mutation.inverse(&snapshot) {
        apply_dwg_mutation(&mut header_snapshot, &inverse);
    }
    assert_eq!(encode_dwg(&header_snapshot).expect("inverse export"), ARCHITECTURAL_FIXTURE);
    assert_eq!(header_snapshot, snapshot);
}
//#endregion 🔖️RealFixture

//#region 🔖️ConformanceLaws
/// 🧪️ 🎫️26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION FG2: per-standard
/// conformance laws for ac1024's real facets — grammar/protocol parseability, `Recognizer`
/// against real fixtures AND real `print_op`/`print_diff` output, `walk_protocol` against real
/// `encode_pack`/`encode_op`/`encode_diff` bytes, and the fixture-honesty round-trip. Dissolved
/// out of `⚙️engine`'s own test region (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES)
/// — mirrors `stdio.binary`/`stdio.txt`'s own `conformance_laws` module shape exactly.
mod conformance_laws {
    use super::*;
    use crate::schema::{diff, mutations, snapshot};
    use crate::standards::v_ac1024::subsets::any::schema::inferences;
    use protocol::{DiffCodec, OpBinary, OpText};

    /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
    /// parse under the real dialect.
    #[semio_framework_async_macros::async_test]
    async fn committed_facet_files_parse() {
        for (label, text) in [
            ("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO),
            ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO),
            ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO),
            ("inference grammar", inferences::text::COMPONENT_GRAMMAR_SEMIO),
        ] {
            let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
            assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
        }
        for (label, text) in [
            ("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
            ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO),
            ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO),
            ("inference protocol", inferences::binary::COMPONENT_PROTOCOL_SEMIO),
        ] {
            dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn schema_facets_contain_no_container_shadow_state() {
        let descriptor = crate::schema::dwg_artifact_schema_descriptor();
        let inference_descriptor = inferences::dwg_artifact_inference_descriptor();
        let leaves = [&descriptor.artifact, &descriptor.snapshot, &descriptor.diff, &descriptor.mutations, &inference_descriptor.inference];
        let forbidden = [
            "DwgSection",
            "sectionNames",
            "section_names",
            "decodeStatus",
            "decode_status",
            "insertSection",
            "removeSection",
            "setSectionData",
            "pageNumber",
            "page_number",
            "startOffset",
            "start_offset",
            "declaredSize",
            "declared_size",
            "decompressedSize",
            "decompressed_size",
            "compressed:",
            "encrypted:",
            "decoded:",
            "bytes_wire",
        ];
        for (facet_index, leaf) in leaves.iter().enumerate() {
            for (language, source) in [("rust", leaf.rust), ("typescript", leaf.typescript), ("graphql", leaf.graphql), ("json", leaf.json_schema), ("proto", leaf.proto)] {
                for term in forbidden {
                    assert!(!source.contains(term), "facet {facet_index} {language} retains forbidden DWG shadow term {term}");
                }
            }
        }
        for (language, source) in [
            ("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO),
            ("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
            ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO),
            ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO),
            ("mutation grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO),
            ("mutation protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO),
            ("inference grammar", inferences::text::COMPONENT_GRAMMAR_SEMIO),
            ("inference protocol", inferences::binary::COMPONENT_PROTOCOL_SEMIO),
        ] {
            for term in forbidden {
                assert!(!source.contains(term), "{language} retains forbidden DWG shadow term {term}");
            }
            for term in ["payload = *OCTET", "size-eos", "bytes &eod", "chain body bytes"] {
                assert!(!source.contains(term), "{language} retains opaque terminal schema term {term}");
            }
        }
        for term in ["hex-body", "chain remainder", "field magic fixed", "BINARY-NATIVE"] {
            assert!(!snapshot::text::COMPONENT_GRAMMAR_SEMIO.contains(term), "snapshot grammar retains native-document persistence term {term}");
            assert!(!snapshot::binary::COMPONENT_PROTOCOL_SEMIO.contains(term), "snapshot protocol retains native-document persistence term {term}");
        }
    }

    /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output
    /// for the ac1024 demo snapshot AND the real, ~145KB `architectural.dwg` fixture (a
    /// second, genuinely non-trivial real-fixture recognition, beyond the minimal demo stub).
    #[semio_framework_async_macros::async_test]
    async fn grammar_conformance_law() {
        let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);
        let text = store::ArtifactDsl::print_dsl(&crate::standards::v_ac1024::engine::demo_dwg_snapshot());
        let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
        let reconstructed = format!("{}\n{body}", envelope.envelope_id());
        assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");

        let real_snap = snapshot::decode_dwg(crate::examples::architectural::FIXTURE_BYTES).expect("decode real fixture");
        let real_text = store::ArtifactDsl::print_dsl(&real_snap);
        let (real_envelope, real_body) = store::semio_format::split_text_preamble(&real_text).expect("split preamble");
        let real_reconstructed = format!("{}\n{real_body}", real_envelope.envelope_id());
        assert!(recognizer.recognize(&real_reconstructed).expect("recognize"), "grammar did not recognize the real architectural.dwg fixture's dsl body");
    }

    /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
    /// output for every `mutations::demo_mutation_cases()` variant.
    #[semio_framework_async_macros::async_test]
    async fn ops_grammar_conformance_law() {
        let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);
        for mutation in mutations::demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
        }
    }

    /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
    /// for every `diff::demo_diff_cases()`, incl. the empty (all-`None`) diff and a rich
    /// `sections` triple case.
    #[semio_framework_async_macros::async_test]
    async fn diff_grammar_conformance_law() {
        let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);
        for d in diff::demo_diff_cases() {
            let printed = d.print_diff();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
        }
    }

    /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets —
    /// snapshot pack (`encode_pack`, envelope-unwrapped first, both the demo AND the real
    /// architectural.dwg fixture), every demo mutation's `encode_op`, and every demo diff's
    /// `encode_diff` — asserting `consumed == bytes.len()`.
    #[semio_framework_async_macros::async_test]
    async fn protocol_walk_law() {
        let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
        let packed = store::ArtifactPack::encode_pack(&crate::standards::v_ac1024::engine::demo_dwg_snapshot());
        let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
        let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
        assert_eq!(trace.consumed, inner.len(), "pack walk did not consume every byte");

        let real_snap = snapshot::decode_dwg(crate::examples::architectural::FIXTURE_BYTES).expect("decode real fixture");
        let real_packed = store::ArtifactPack::encode_pack(&real_snap);
        let (_, real_inner) = store::semio_format::unwrap_binary(&real_packed).expect("unwrap semio envelope (real fixture)");
        let real_trace = dsl::walk_protocol(&pack_spec, &real_inner).unwrap_or_else(|e| panic!("walk_protocol(pack, real fixture) failed @{}: {}", e.offset, e.message));
        assert_eq!(real_trace.consumed, real_inner.len(), "real-fixture pack walk did not consume every byte");

        let op_spec = dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
        for mutation in mutations::demo_mutation_cases() {
            let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
            let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
        }

        let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
        for d in diff::demo_diff_cases() {
            let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
            let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
        }
    }

    /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are GENUINE
    /// `print_dsl`/`encode_pack` output of `demo_dwg_snapshot()`.
    #[semio_framework_async_macros::async_test]
    async fn fixture_honesty_law() {
        const FIXTURE_DSL: &str = include_str!("../../../../../../4️⃣ac1018/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");
        const FIXTURE_PACK: &[u8] = include_bytes!("../../../../../../4️⃣ac1018/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio");

        let demo = crate::standards::v_ac1024::engine::demo_dwg_snapshot();

        let parsed = <DwgSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
        assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_dwg_snapshot()");
        assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_dwg_snapshot()) drifted from the shipped .dsl.semio fixture");

        let decoded = <DwgSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
        assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_dwg_snapshot()");
        assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_dwg_snapshot()) drifted from the shipped .pack.semio fixture");
    }
}
//#endregion 🔖️ConformanceLaws
