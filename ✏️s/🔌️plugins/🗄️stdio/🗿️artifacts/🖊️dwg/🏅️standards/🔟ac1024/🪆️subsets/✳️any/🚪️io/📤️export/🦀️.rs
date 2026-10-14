//! 📤️ DWG byte writer of `stdio.dwg` (ac1024/✳️any): the R2004 container, LZ77-variant section compression,
//! section/page maps and object bitcode encoded from the typed snapshot and the structural drawing model.
//! Depends on the model only; its round-trip laws read back through the sibling `import` module.

use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::dwg_version_sentinel;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::parse_version_header_fields;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::R2004_HEADER_LEN;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::decrypt_r2004_header;
use semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgColor;
use semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgDrawing;
use semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgEntity;
use semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgGeometry;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::schema::snapshot::is_preamble_only_document;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::schema::snapshot::unwritable_version;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::schema::snapshot::DwgExportError;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::schema::snapshot::DwgSnapshot;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::schema::snapshot::DWG_PREAMBLE_LEN;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::schema::snapshot::STDIO_DWG_DOCUMENT_SCHEMA;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::dwg_drawing_to_mesh;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::dwg_drawing_to_paths;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::dwg_drawing_to_svg;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::dwg_geometry_to_path_segments;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::mesh_to_dwg_drawing;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::paths_to_dwg_drawing;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgLayer;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgPathSegment;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::document::decode_r2004_document_sections;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::decode_r2004_handle_map;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::container::decode_r2004_sections;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::document::decode_r2010_classes_section;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::container::decompress_r2004_section;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_read::standards::v_ac1024::subsets::any::io::import::dwg_from_bytes;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::container::locate_r2004_sections;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::container::parse_page_directory;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::container::parse_r2004_file_header;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::container::r2004_section_data;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::container::DwgRawSection;
#[cfg(test)]
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::DWG_TYPE_LAYER;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::bits::push_u32;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::container::EncodedR2004Page;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::container::R2004SectionDescriptor;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::container::align_r2004;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::container::compress_r2004_section;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::container::encode_r2004_section_info;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::container::r2004_crc32;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::container::write_data_page;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::container::write_r2004_system_page;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::document::encode_application_history;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::document::encode_application_info;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::document::encode_auxiliary_header;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::document::encode_dependencies;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::document::encode_indexed_preview;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::document::encode_object_free_space;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::document::encode_r2010_classes_section;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::document::encode_r2010_header_section;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::document::encode_revision_history;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::document::encode_summary_info;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::document::encode_template;
use semio_s_artifact_stdio_dwg_write_objects::standards::v_ac1024::subsets::any::io::export::objects::materialize_r2004_handles;
use semio_s_artifact_stdio_dwg_write_objects::standards::v_ac1024::subsets::any::io::export::objects::materialize_r2010_objects;


//#region 🔖️SectionMapAndInfo
//#region 🔖️R2004Writer
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn materialize_r2004_ordinary_pages_without_header(snapshot: &DwgSnapshot) -> Result<Vec<EncodedR2004Page>, String> {
    let (objects, pairs) = materialize_r2010_objects(&snapshot.drawing.objects)?;
    let handles = materialize_r2004_handles(&pairs)?;
    let sections = [
        (9u32, encode_summary_info(&snapshot.summary)?, Some(128usize)),
        (10, encode_indexed_preview(&snapshot.preview, 0x1c0)?, Some(87_040)),
        (11, encode_application_info(&snapshot.application)?, Some(768)),
        (12, encode_application_history(&snapshot.application_history)?, Some(1_408)),
        (13, encode_dependencies(&snapshot.dependencies)?, Some(768)),
        (8, encode_revision_history(&snapshot.revision_history)?, None),
        (7, objects, None),
        (6, encode_object_free_space(&snapshot.auxiliary_header.updated_at), None),
        (5, encode_template(&snapshot.template)?, None),
        (4, handles, None),
        (3, encode_r2010_classes_section(&snapshot.classes)?, None),
        (2, encode_auxiliary_header(&snapshot.auxiliary_header)?, None),
    ];
    let mut pages = Vec::new();
    let mut address = 0x100usize;
    let mut page_number = 1i32;
    for (section_id, content, stored_capacity) in sections {
        if let Some(capacity) = stored_capacity {
            if content.len() > capacity {
                return Err(format!("stored section {section_id} exceeds capacity {capacity}"));
            }
            let mut payload = content;
            payload.resize(capacity, 0);
            let allocation_size = (32 + capacity) as u32;
            pages.push(EncodedR2004Page { section_id, page_number, start_offset: 0, payload, address: address as u64, allocation_size });
            address += allocation_size as usize;
            page_number += 1;
            continue;
        }
        for (page_index, chunk) in content.chunks(0x7400).enumerate() {
            let mut decoded = vec![0; 0x7400];
            decoded[..chunk.len()].copy_from_slice(chunk);
            let payload = compress_r2004_section(&decoded)?;
            let allocation_size = align_r2004(32 + payload.len()) as u32;
            pages.push(EncodedR2004Page { section_id, page_number, start_offset: (page_index * 0x7400) as u64, payload, address: address as u64, allocation_size });
            address += allocation_size as usize;
            page_number += 1;
        }
    }
    Ok(pages)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn r2004_section_descriptors(snapshot: &DwgSnapshot, header_size: usize) -> Result<Vec<R2004SectionDescriptor>, String> {
    let (objects, pairs) = materialize_r2010_objects(&snapshot.drawing.objects)?;
    let handles = materialize_r2004_handles(&pairs)?;
    let sizes = [
        (13, "AcDb:FileDepList", encode_dependencies(&snapshot.dependencies)?.len(), 768, 1, 2),
        (12, "AcDb:AppInfoHistory", encode_application_history(&snapshot.application_history)?.len(), 1_408, 1, 0),
        (11, "AcDb:AppInfo", encode_application_info(&snapshot.application)?.len(), 768, 1, 0),
        (10, "AcDb:Preview", encode_indexed_preview(&snapshot.preview, 0x1c0)?.len(), 87_040, 1, 0),
        (9, "AcDb:SummaryInfo", encode_summary_info(&snapshot.summary)?.len(), 128, 1, 0),
        (8, "AcDb:RevHistory", encode_revision_history(&snapshot.revision_history)?.len(), 0x7400, 2, 0),
        (7, "AcDb:AcDbObjects", objects.len(), 0x7400, 2, 0),
        (6, "AcDb:ObjFreeSpace", encode_object_free_space(&snapshot.auxiliary_header.updated_at).len(), 0x7400, 2, 0),
        (5, "AcDb:Template", encode_template(&snapshot.template)?.len(), 0x7400, 2, 0),
        (4, "AcDb:Handles", handles.len(), 0x7400, 2, 0),
        (3, "AcDb:Classes", encode_r2010_classes_section(&snapshot.classes)?.len(), 0x7400, 2, 0),
        (2, "AcDb:AuxHeader", encode_auxiliary_header(&snapshot.auxiliary_header)?.len(), 0x7400, 2, 0),
        (1, "AcDb:Header", header_size, 0x7400, 2, 0),
    ];
    let mut descriptors = vec![R2004SectionDescriptor { name: "", section_id: 0, semantic_size: 0, max_decompressed_size: 0x7400, compression: 2, encryption: 0 }];
    descriptors.extend(sizes.into_iter().map(|(section_id, name, semantic_size, max_decompressed_size, compression, encryption)| R2004SectionDescriptor {
        name,
        section_id,
        semantic_size: semantic_size as u64,
        max_decompressed_size,
        compression,
        encryption,
    }));
    Ok(descriptors)
}
/// 🏗️ Materializes a canonical R2004-family directory from logical AC1024 section descriptors.
/// Section payloads are serialization products and are never retained in the artifact schema.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2004_canonical(snapshot: &DwgSnapshot) -> Result<Vec<u8>, String> {
    if snapshot.version.len() != 6 {
        return Err("version sentinel must contain six bytes".into());
    }
    let header = encode_r2010_header_section(&snapshot.header)?;
    let mut pages = materialize_r2004_ordinary_pages_without_header(snapshot)?;
    let mut decoded_header = vec![0; 0x7400];
    decoded_header[..header.len()].copy_from_slice(&header);
    let header_payload = compress_r2004_section(&decoded_header)?;
    let header_address = pages.last().map_or(0x100, |page| page.address + u64::from(page.allocation_size));
    let header_page_number = i32::try_from(pages.len() + 1).map_err(|_| "R2004 page count exceeds i32")?;
    pages.push(EncodedR2004Page { section_id: 1, page_number: header_page_number, start_offset: 0, allocation_size: align_r2004(32 + header_payload.len()) as u32, address: header_address, payload: header_payload });
    let section_info_page_number = header_page_number as u32 + 3;
    let section_map_page_number = header_page_number as u32 + 4;
    let descriptors = r2004_section_descriptors(snapshot, header.len())?;
    let application_history_scratch = encode_application_history(&snapshot.application_history)?;
    let page_map_with = |section_info_allocation: u32, section_map_allocation: u32| {
        let mut page_map = Vec::new();
        for page in &pages {
            push_u32(&mut page_map, page.page_number as u32);
            push_u32(&mut page_map, page.allocation_size);
        }
        push_u32(&mut page_map, section_info_page_number);
        push_u32(&mut page_map, section_info_allocation);
        push_u32(&mut page_map, section_map_page_number);
        push_u32(&mut page_map, section_map_allocation);
        page_map
    };
    let compressed_map = compress_r2004_section(&page_map_with(1_024, 1_600))?;
    let mut reserved_name = [0u8; 64];
    let reserved_source = compressed_map.get(52..).unwrap_or_default();
    let reserved_len = reserved_source.len().min(64);
    reserved_name[..reserved_len].copy_from_slice(&reserved_source[..reserved_len]);
    reserved_name[0] = 0;
    let section_info = encode_r2004_section_info(&descriptors, &pages, &reserved_name, &application_history_scratch)?;
    let section_info_allocation = align_r2004(40 + compress_r2004_section(&section_info)?.len()).max(1_024);
    let page_map = page_map_with(section_info_allocation as u32, 1_600);
    let section_map_allocation = align_r2004(40 + compress_r2004_section(&page_map)?.len()).max(1_600);
    let page_map = page_map_with(section_info_allocation as u32, section_map_allocation as u32);

    let mut output = vec![0u8; 0x100];
    output[0..6].copy_from_slice(snapshot.version.as_bytes());
    output[0x0b] = 2;
    output[0x0c] = 3;
    let preview_address = pages.iter().find(|page| page.section_id == 10).ok_or("Preview page missing")?.address as u32 + 32;
    output[0x0d..0x11].copy_from_slice(&preview_address.to_le_bytes());
    output[0x11] = 0x1d;
    output[0x12] = snapshot.maintenance_version;
    output[0x13..0x15].copy_from_slice(&snapshot.codepage.to_le_bytes());
    output[0x16] = 0x1d;
    output[0x17] = snapshot.maintenance_version;
    let summary_address = pages.iter().find(|page| page.section_id == 9).ok_or("SummaryInfo page missing")?.address as u32 + 32;
    output[0x20..0x24].copy_from_slice(&summary_address.to_le_bytes());
    output[0x28..0x2c].copy_from_slice(&0x80u32.to_le_bytes());
    let application_address = pages.iter().find(|page| page.section_id == 11).ok_or("AppInfo page missing")?.address as u32 + 32;
    output[0x2c..0x30].copy_from_slice(&application_address.to_le_bytes());
    let application_history_address = pages.iter().find(|page| page.section_id == 12).ok_or("AppInfoHistory page missing")?.address as u32 + 32;
    output[0x30..0x34].copy_from_slice(&application_history_address.to_le_bytes());
    for page in &pages {
        write_data_page(&mut output, page)?;
    }
    write_r2004_system_page(&mut output, 0x4163003b, &section_info, Some(section_info_allocation), 1)?;
    let page_map_address = output.len();
    write_r2004_system_page(&mut output, 0x41630e3b, &page_map, None, 0)?;
    let second_header_address = output.len() as u64;

    let mut file_header = vec![0u8; R2004_HEADER_LEN];
    file_header[0..12].copy_from_slice(b"AcFssFcAJMB\0");
    file_header[0x10..0x14].copy_from_slice(&0x6cu32.to_le_bytes());
    file_header[0x14..0x18].copy_from_slice(&4u32.to_le_bytes());
    file_header[0x24..0x28].copy_from_slice(&1u32.to_le_bytes());
    file_header[0x28..0x2c].copy_from_slice(&section_map_page_number.to_le_bytes());
    let last_section_address = page_map_address + section_map_allocation - 0x100;
    file_header[0x2c..0x34].copy_from_slice(&(last_section_address as u64).to_le_bytes());
    file_header[0x34..0x3c].copy_from_slice(&second_header_address.to_le_bytes());
    file_header[0x40..0x44].copy_from_slice(&(header_page_number as u32 + 2).to_le_bytes());
    file_header[0x44..0x48].copy_from_slice(&0x20u32.to_le_bytes());
    file_header[0x48..0x4c].copy_from_slice(&0x80u32.to_le_bytes());
    file_header[0x4c..0x50].copy_from_slice(&0x40u32.to_le_bytes());
    file_header[0x50..0x54].copy_from_slice(&section_map_page_number.to_le_bytes());
    file_header[0x54..0x5c].copy_from_slice(&((page_map_address - 0x100) as u64).to_le_bytes());
    file_header[0x5c..0x60].copy_from_slice(&section_info_page_number.to_le_bytes());
    file_header[0x60..0x64].copy_from_slice(&section_map_page_number.to_le_bytes());
    let crc = r2004_crc32(&file_header);
    file_header[0x68..0x6c].copy_from_slice(&crc.to_le_bytes());
    let encrypted_header = decrypt_r2004_header(&file_header);
    output[0x80..0xec].copy_from_slice(&encrypted_header);
    let magic = decrypt_r2004_header(&vec![0; 0x100]);
    output[0xec..0x100].copy_from_slice(&magic[0xec..0x100]);
    output.extend_from_slice(&encrypted_header);
    Ok(output)
}
/// 🧱️ Deterministically materializes AC1024 from logical drawing and section state.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2004_snapshot(snapshot: &DwgSnapshot) -> Result<Vec<u8>, String> {
    encode_r2004_canonical(snapshot)
}
//#endregion 🔖️R2004Writer
//#endregion 🔖️SectionMapAndInfo
//#region 🔖️DwgStructuralCodec
//#region DwgWrite
/// 📐️ Serializes a drawing as a new AutoCAD 2010 (AC1024) document: the drawing becomes the model
/// space of [`DwgSnapshot::from_drawing`], written by the same R2004-family container and R2010
/// object writer as every other AC1024 snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dwg_to_bytes(drawing: &DwgDrawing) -> Result<Vec<u8>, String> {
    encode_r2004_snapshot(&DwgSnapshot::from_drawing(drawing)?)
}
//#endregion DwgWrite
//#endregion 🔖️DwgStructuralCodec
//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
include!("🧪️tests/🔬️standalone/🦀️.rs");
//#endregion 🧪️Tests
/// 📏️ Encodes layered planar polylines supplied by an artifact I/O conversion.
pub fn polylines_to_dwg_bytes<'a>(polylines: impl IntoIterator<Item = (&'a str, &'a [[f64; 2]], bool)>) -> Result<Vec<u8>, String> {
    let mut drawing = DwgDrawing::default();
    drawing.ensure_layer("0");
    for (name, vertices, closed) in polylines {
        let layer = drawing.ensure_layer(name);
        drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::LwPolyline { closed, elevation: 0.0, vertices: vertices.to_vec(), bulges: vec![0.0; vertices.len()] } });
    }
    dwg_to_bytes(&drawing)
}
#[cfg(test)]
#[path = "🧪️tests/🔬️polyline-io/🦀️.rs"]
mod polyline_io_tests;
#[cfg(test)]
#[path = "🧪️tests/🔮️acadrust-oracle/🦀️.rs"]
mod acadrust_oracle_tests;
//#region 📸️SnapshotEncode
/// 🖨️ The empty document's bytes: the preamble region, version stamp and preamble triple written in,
/// every other byte zero — byte for byte what the committed demo example already is.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_preamble_only_document(snapshot: &DwgSnapshot) -> Vec<u8> {
    let mut bytes = vec![0u8; DWG_PREAMBLE_LEN];
    bytes[0..6].copy_from_slice(snapshot.version.as_bytes());
    bytes[0x12] = snapshot.maintenance_version;
    bytes[0x13..0x15].copy_from_slice(&snapshot.codepage.to_le_bytes());
    bytes
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate_export_header(bytes: &[u8], snapshot: &DwgSnapshot) -> Result<(), DwgExportError> {
    let version = dwg_version_sentinel(bytes).map_err(DwgExportError::Writer)?;
    if snapshot.version.len() != 6 {
        return Err(DwgExportError::InvalidVersion("AC10xx sentinel must contain six ASCII bytes".into()));
    }
    if version != snapshot.version {
        return Err(DwgExportError::HeaderMismatch(format!("version {} != {}", version, snapshot.version)));
    }
    let (maintenance_version, codepage) = parse_version_header_fields(bytes).map_err(DwgExportError::Writer)?;
    if maintenance_version != snapshot.maintenance_version {
        return Err(DwgExportError::HeaderMismatch(format!("maintenance version {maintenance_version} != {}", snapshot.maintenance_version)));
    }
    if codepage != snapshot.codepage {
        return Err(DwgExportError::HeaderMismatch(format!("codepage {codepage} != {}", snapshot.codepage)));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_dwg(snapshot: &DwgSnapshot) -> Result<Vec<u8>, DwgExportError> {
    if snapshot.schema != STDIO_DWG_DOCUMENT_SCHEMA {
        return Err(DwgExportError::InvalidLogical("schema identity changed".into()));
    }
    if let Some((_, message)) = unwritable_version(snapshot) {
        return Err(DwgExportError::InvalidVersion(message));
    }
    // 🫙️ The empty document is written as the preamble and nothing else — the inverse of the read
    // above, and the only representation an R2004 container HAS for a snapshot that carries no
    // Header, no Classes and no object sections to build one from.
    let bytes = if is_preamble_only_document(snapshot) { encode_preamble_only_document(snapshot) } else { encode_r2004_snapshot(snapshot).map_err(DwgExportError::Writer)? };
    validate_export_header(&bytes, snapshot)?;
    Ok(bytes)
}
//#endregion 📸️SnapshotEncode
