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
const R2004_PAGE_SIZE: usize = 0x7400;

struct R2004Section {
    id: u32,
    name: &'static str,
    content: Vec<u8>,
    stored: Option<(u32, u32)>,
}

impl R2004Section {
    fn compressed(id: u32, name: &'static str, content: Vec<u8>) -> Self {
        Self { id, name, content, stored: None }
    }

    fn stored(id: u32, name: &'static str, content: Vec<u8>, capacity: u32, encryption: u32) -> Self {
        Self { id, name, content, stored: Some((capacity, encryption)) }
    }

    fn descriptor(&self) -> R2004SectionDescriptor {
        let (max_decompressed_size, encryption) = self.stored.unwrap_or((R2004_PAGE_SIZE as u32, 0));
        R2004SectionDescriptor { name: self.name, section_id: self.id, semantic_size: self.content.len() as u64,
            max_decompressed_size, compression: if self.stored.is_some() { 1 } else { 2 }, encryption }
    }

    fn encode_page(&self, start_offset: usize, address: u64, page_number: i32) -> Result<EncodedR2004Page, String> {
        let size = self.stored.map_or(R2004_PAGE_SIZE, |(capacity, _)| capacity as usize);
        let source = &self.content[start_offset..];
        if self.stored.is_some() && source.len() > size {
            return Err(format!("stored section {} exceeds capacity {size}", self.id));
        }
        let mut buffer = vec![0; size];
        let copied = source.len().min(size);
        buffer[..copied].copy_from_slice(&source[..copied]);
        let (payload, allocation_size) = if self.stored.is_some() {
            (buffer, 32 + size)
        } else {
            let payload = compress_r2004_section(&buffer)?;
            let allocation_size = align_r2004(32 + payload.len());
            (payload, allocation_size)
        };
        Ok(EncodedR2004Page { section_id: self.id, page_number, start_offset: start_offset as u64,
            payload, address, allocation_size: allocation_size as u32 })
    }

    fn append_pages(&self, pages: &mut Vec<EncodedR2004Page>) -> Result<(), String> {
        let span = if self.stored.is_some() { 1 } else { self.content.len() };
        for offset in (0..span).step_by(R2004_PAGE_SIZE) {
            let address = pages.last().map_or(0x100, |page| page.address + u64::from(page.allocation_size));
            let number = i32::try_from(pages.len() + 1).map_err(|_| "R2004 page count exceeds i32")?;
            pages.push(self.encode_page(offset, address, number)?);
        }
        Ok(())
    }
}

const SECTION_SLOTS: usize = 13;
const OBJECTS_SLOT: usize = 6;
const HANDLES_SLOT: usize = 9;

fn encode_slot(snapshot: &DwgSnapshot, slot: usize) -> Result<R2004Section, String> {
    Ok(match slot {
        0 => R2004Section::stored(9, "AcDb:SummaryInfo", encode_summary_info(&snapshot.summary)?, 128, 0),
        1 => R2004Section::stored(10, "AcDb:Preview", encode_indexed_preview(&snapshot.preview, 0x1c0)?, 87_040, 0),
        2 => R2004Section::stored(11, "AcDb:AppInfo", encode_application_info(&snapshot.application)?, 768, 0),
        3 => R2004Section::stored(12, "AcDb:AppInfoHistory", encode_application_history(&snapshot.application_history)?, 1_408, 0),
        4 => R2004Section::stored(13, "AcDb:FileDepList", encode_dependencies(&snapshot.dependencies)?, 768, 2),
        5 => R2004Section::compressed(8, "AcDb:RevHistory", encode_revision_history(&snapshot.revision_history)?),
        7 => R2004Section::compressed(6, "AcDb:ObjFreeSpace", encode_object_free_space(&snapshot.auxiliary_header.updated_at)),
        8 => R2004Section::compressed(5, "AcDb:Template", encode_template(&snapshot.template)?),
        10 => R2004Section::compressed(3, "AcDb:Classes", encode_r2010_classes_section(&snapshot.classes)?),
        11 => R2004Section::compressed(2, "AcDb:AuxHeader", encode_auxiliary_header(&snapshot.auxiliary_header)?),
        12 => R2004Section::compressed(1, "AcDb:Header", encode_r2010_header_section(&snapshot.header)?),
        _ => return Err(format!("section slot {slot} carries streamed content")),
    })
}

fn streamed_slot(slot: usize, content: Vec<u8>) -> R2004Section {
    if slot == OBJECTS_SLOT { R2004Section::compressed(7, "AcDb:AcDbObjects", content) } else { R2004Section::compressed(4, "AcDb:Handles", content) }
}

fn materialize_r2004_sections(snapshot: &DwgSnapshot) -> Result<Vec<R2004Section>, String> {
    let (objects, pairs) = materialize_r2010_objects(&snapshot.drawing.objects)?;
    let handles = materialize_r2004_handles(&pairs)?;
    let (mut objects, mut handles) = (Some(objects), Some(handles));
    (0..SECTION_SLOTS).map(|slot| match slot {
        OBJECTS_SLOT => Ok(streamed_slot(slot, objects.take().unwrap_or_default())),
        HANDLES_SLOT => Ok(streamed_slot(slot, handles.take().unwrap_or_default())),
        other => encode_slot(snapshot, other),
    }).collect()
}

/// 🗺️ The container bytes that depend only on the finished pages: the preamble, both system pages and the file header inputs.
struct R2004Layout {
    prefix: Vec<u8>,
    section_info: Vec<u8>,
    page_map: Vec<u8>,
    section_info_allocation: usize,
    section_map_allocation: usize,
    section_map_page_number: u32,
    last_page_number: u32,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn layout_r2004(snapshot: &DwgSnapshot, mut descriptors: Vec<R2004SectionDescriptor>, pages: &[EncodedR2004Page], application_history: &[u8]) -> Result<R2004Layout, String> {
    if snapshot.version.len() != 6 {
        return Err("version sentinel must contain six bytes".into());
    }
    let last_page_number = u32::try_from(pages.len()).map_err(|_| "R2004 page count exceeds u32")?;
    let section_info_page_number = last_page_number + 3;
    let section_map_page_number = last_page_number + 4;
    descriptors.sort_unstable_by_key(|descriptor| std::cmp::Reverse(descriptor.section_id));
    descriptors.insert(0, R2004Section::compressed(0, "", Vec::new()).descriptor());
    let page_map_with = |section_info_allocation: u32, section_map_allocation: u32| {
        let mut page_map = Vec::new();
        for page in pages {
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
    let section_info = encode_r2004_section_info(&descriptors, pages, &reserved_name, application_history)?;
    let section_info_allocation = align_r2004(40 + compress_r2004_section(&section_info)?.len()).max(1_024);
    let page_map = page_map_with(section_info_allocation as u32, 1_600);
    let section_map_allocation = align_r2004(40 + compress_r2004_section(&page_map)?.len()).max(1_600);
    let page_map = page_map_with(section_info_allocation as u32, section_map_allocation as u32);

    let mut prefix = vec![0u8; 0x100];
    prefix[0..6].copy_from_slice(snapshot.version.as_bytes());
    prefix[0x0b] = 2;
    prefix[0x0c] = 3;
    let preview_address = pages.iter().find(|page| page.section_id == 10).ok_or("Preview page missing")?.address as u32 + 32;
    prefix[0x0d..0x11].copy_from_slice(&preview_address.to_le_bytes());
    prefix[0x11] = 0x1d;
    prefix[0x12] = snapshot.maintenance_version;
    prefix[0x13..0x15].copy_from_slice(&snapshot.codepage.to_le_bytes());
    prefix[0x16] = 0x1d;
    prefix[0x17] = snapshot.maintenance_version;
    let summary_address = pages.iter().find(|page| page.section_id == 9).ok_or("SummaryInfo page missing")?.address as u32 + 32;
    prefix[0x20..0x24].copy_from_slice(&summary_address.to_le_bytes());
    prefix[0x28..0x2c].copy_from_slice(&0x80u32.to_le_bytes());
    let application_address = pages.iter().find(|page| page.section_id == 11).ok_or("AppInfo page missing")?.address as u32 + 32;
    prefix[0x2c..0x30].copy_from_slice(&application_address.to_le_bytes());
    let application_history_address = pages.iter().find(|page| page.section_id == 12).ok_or("AppInfoHistory page missing")?.address as u32 + 32;
    prefix[0x30..0x34].copy_from_slice(&application_history_address.to_le_bytes());
    Ok(R2004Layout { prefix, section_info, page_map, section_info_allocation, section_map_allocation, section_map_page_number, last_page_number })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn seal_r2004(layout: &R2004Layout, mut output: Vec<u8>) -> Result<Vec<u8>, String> {
    let R2004Layout { section_info, page_map, section_info_allocation, section_map_allocation, section_map_page_number, last_page_number, .. } = layout;
    let (section_info_allocation, section_map_allocation, section_map_page_number, last_page_number) = (*section_info_allocation, *section_map_allocation, *section_map_page_number, *last_page_number);
    let section_info_page_number = last_page_number + 3;
    write_r2004_system_page(&mut output, 0x4163003b, section_info, Some(section_info_allocation), 1)?;
    let page_map_address = output.len();
    write_r2004_system_page(&mut output, 0x41630e3b, page_map, None, 0)?;
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
    file_header[0x40..0x44].copy_from_slice(&(last_page_number + 2).to_le_bytes());
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

/// 🏗️ Materializes a canonical R2004-family directory from logical AC1024 section descriptors.
/// Section payloads are serialization products and are never retained in the artifact schema.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2004_canonical(snapshot: &DwgSnapshot) -> Result<Vec<u8>, String> {
    if snapshot.version.len() != 6 {
        return Err("version sentinel must contain six bytes".into());
    }
    let mut pages = Vec::new();
    let mut descriptors = Vec::with_capacity(14);
    let mut application_history = Vec::new();
    for section in materialize_r2004_sections(snapshot)? {
        descriptors.push(section.descriptor());
        section.append_pages(&mut pages)?;
        if section.id == 12 {
            application_history = section.content;
        }
    }
    let layout = layout_r2004(snapshot, descriptors, &pages, &application_history)?;
    let mut output = layout.prefix.clone();
    for page in &pages {
        write_data_page(&mut output, page)?;
    }
    seal_r2004(&layout, output)
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
#[path = "🧵️write-cursor/🦀️.rs"]
mod write_cursor;
pub use write_cursor::{DwgWriteCursor, DwgWriteStep};
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
    let applied = snapshot.drawing.layers_applied(snapshot.header.relations.handle_seed.max(snapshot.auxiliary_header.handle_seed)).map_err(DwgExportError::InvalidLogical)?;
    let mut reconciled;
    let source = match applied {
        Some(drawing) => {
            let seed = drawing.handle_seed();
            reconciled = DwgSnapshot { drawing, ..snapshot.clone() };
            reconciled.header.relations.handle_seed = reconciled.header.relations.handle_seed.max(seed);
            reconciled.auxiliary_header.handle_seed = reconciled.auxiliary_header.handle_seed.max(seed);
            &reconciled
        }
        None => snapshot,
    };
    let bytes = if is_preamble_only_document(source) { encode_preamble_only_document(source) } else { encode_r2004_snapshot(source).map_err(DwgExportError::Writer)? };
    validate_export_header(&bytes, source)?;
    Ok(bytes)
}
//#endregion 📸️SnapshotEncode
