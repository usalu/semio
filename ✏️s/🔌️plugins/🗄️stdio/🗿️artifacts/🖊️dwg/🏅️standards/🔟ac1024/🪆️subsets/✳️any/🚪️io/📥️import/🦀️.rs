//! 📥️ DWG byte reader of `stdio.dwg` (ac1024/✳️any): the real AC1024/R2004+ byte-level decode pipeline (ticket 26/08/10/
//! ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, 🖊️dwg D1-D2 wave): file header
//! decrypt, section/page directory walk (D1), and the bespoke R2004+ LZ77-variant section
//! decompressor (D2). Algorithm field-layout/opcode semantics cross-checked against the
//! ODA-spec-derived LibreDWG reference (GPLv3, github.com/LibreDWG/libredwg `src/decode.c` +
//! `src/r2004_file_header.spec`) and validated end to end against the real 145KB
//! `architectural.dwg` fixture via a standalone scratch crate before landing here -- clean-room
//! reimplementation, no code copied. Undecoded/unreachable content is never fabricated: every
//! function here returns a typed `Result`; native framing and page buffers remain transient and
//! only decoded logical standard concepts cross the deserialization boundary. Pure byte<->byte
//! algorithms with no `DwgSnapshot` dependency of their own — kept here per ticket 26/08/12/ENGINELESS-ARTIFACTS-
//! AND-APP-STATE-MACHINES rule 6 ("keep with the codec in 🚪️io/").
//!
//! Depends on the model only (`schema` and the binary-representation wire vocabulary); the writer lives in the
//! sibling `export` module and never reaches this one.

use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::dwg_version_sentinel;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::parse_version_header_fields;
use semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgDrawing;
use semio_s_artifact_stdio_dwg_snapshot_drawing::standards::v_ac1024::subsets::any::schema::snapshot::drawing::DwgLogicalDrawing;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::schema::snapshot::DwgSnapshot;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::schema::snapshot::DWG_PREAMBLE_LEN;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::schema::snapshot::STDIO_DWG_DOCUMENT_SCHEMA;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::container::decode_r2004_sections;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::container::decode_r2007_sections;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::container::r2004_section_data;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::document::decode_r2004_classes;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::document::decode_r2004_document_sections;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::DWG_TYPE_LAYER;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::DwgObjectLayout;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::DwgObjectRecordFrame;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::decode_entity_body;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::decode_layer_projection;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::decode_r2004_handle_map;
use semio_s_artifact_stdio_dwg_read_objects::standards::v_ac1024::subsets::any::io::import::objects::decode_r2004_object_identities;


//#region 🔖️DwgStructuralCodec
//#region DwgObjects
/// 📐️ Reads the drawing of a DWG file of any version this codec knows ([`DwgObjectLayout::of`]):
/// every LAYER record and every modelled entity, projected through the same logical drawing the
/// lossless AC1024 decoder produces (so POLYLINE_3D / POLYLINE_PFACE gather their vertex records).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dwg_from_bytes(bytes: &[u8]) -> Result<DwgDrawing, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgLayerTableRecord;
    use semio_s_artifact_stdio_dwg_snapshot_drawing::standards::v_ac1024::subsets::any::schema::snapshot::drawing::DwgLogicalDrawing;
    use semio_s_artifact_stdio_dwg_snapshot_drawing::standards::v_ac1024::subsets::any::schema::snapshot::drawing::geometry::DwgLogicalLayer;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgObjectCategory;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableRecordBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableRecordCommon;
    let version = String::from_utf8_lossy(bytes.get(0..6).ok_or("DWG file is shorter than its version sentinel")?).to_string();
    let layout = DwgObjectLayout::of(&version)?;
    let sections = if layout == DwgObjectLayout::R2007 { decode_r2007_sections(bytes)? } else { decode_r2004_sections(bytes)? };
    let handles = sections.iter().find(|section| section.name == "AcDb:Handles").ok_or("DWG Handles section missing")?;
    let objects = sections.iter().find(|section| section.name == "AcDb:AcDbObjects").ok_or("DWG AcDbObjects section missing")?;
    let handle_map = decode_r2004_handle_map(&r2004_section_data(handles)?)?;
    let object_data = r2004_section_data(objects)?;
    let mut layers = Vec::new();
    let mut logical_objects = Vec::new();
    for (handle, address) in handle_map {
        let mut frame = DwgObjectRecordFrame::open(layout, &object_data, handle, address)?;
        if frame.type_code != DWG_TYPE_LAYER && !matches!(frame.type_code, 1 | 6 | 11 | 13 | 14 | 16..=19 | 27..=29 | 35 | 36 | 77) {
            continue;
        }
        frame.skip_extended_data().map_err(|error| format!("object {handle:#x} extended data: {error}"))?;
        if frame.type_code == DWG_TYPE_LAYER {
            let layer = decode_layer_projection(&mut frame).map_err(|error| format!("LAYER {handle:#x}: {error}"))?;
            logical_objects.push(DwgLogicalObject {
                handle,
                type_code: DWG_TYPE_LAYER,
                class_name: "LAYER".into(),
                category: DwgObjectCategory::TableRecord,
                body: Some(DwgLogicalObjectBody::TableRecord(DwgTableRecordBody::Layer(DwgLayerTableRecord { common: DwgTableRecordCommon { name: layer.name.clone(), ..Default::default() }, ..Default::default() }))),
                ..Default::default()
            });
            layers.push(DwgLogicalLayer { name: layer.name, color: layer.color });
        } else if let Some((body, (owner_handle, _, _))) = decode_entity_body(&mut frame, true)? {
            logical_objects.push(DwgLogicalObject { handle, type_code: frame.type_code, category: DwgObjectCategory::Entity, owner_handle, body: Some(DwgLogicalObjectBody::Entity(body)), ..Default::default() });
        }
    }
    if layers.is_empty() {
        layers.push(DwgLogicalLayer { name: "0".into(), color: 7 });
    }
    let mut drawing = DwgLogicalDrawing { layers, objects: logical_objects, extmin: vec![0.0; 3], extmax: vec![0.0; 3] }.to_native()?;
    drawing.recompute_extents();
    Ok(drawing)
}
//#endregion DwgObjects
//#endregion 🔖️DwgStructuralCodec
//#region 📸️SnapshotDecode
/// 🗺️ Materializes section pages only while deserializing and projects their standard objects.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_drawing(bytes: &[u8]) -> Result<DwgLogicalDrawing, String> {
    DwgLogicalDrawing::from_native(&dwg_from_bytes(bytes)?)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_dwg(bytes: &[u8]) -> Result<DwgSnapshot, String> {
    let version = dwg_version_sentinel(bytes)?;
    let (maintenance_version, codepage) = parse_version_header_fields(bytes)?;
    // 🫙️ The empty document. A stream that stops at the end of the preamble carries no section map,
    // no page map and no encrypted file header, so there is nothing behind it to decode — and the
    // R2004 reader's "file too short for encrypted header" is the wrong answer to give about a
    // document this artifact itself writes and commits as an example.
    if bytes.len() == DWG_PREAMBLE_LEN {
        return Ok(DwgSnapshot { schema: STDIO_DWG_DOCUMENT_SCHEMA.into(), version, maintenance_version, codepage, ..Default::default() });
    }
    if version != "AC1024" {
        return DwgSnapshot::from_drawing(&dwg_from_bytes(bytes)?);
    }
    let mut drawing = decode_drawing(bytes)?;
    let classes = decode_r2004_classes(bytes)?;
    drawing.objects = decode_r2004_object_identities(bytes, &classes)?;
    let document = decode_r2004_document_sections(bytes)?;
    Ok(DwgSnapshot {
        schema: STDIO_DWG_DOCUMENT_SCHEMA.into(),
        version,
        maintenance_version,
        codepage,
        drawing,
        header: document.header,
        classes,
        dependencies: document.dependencies,
        summary: document.summary,
        application: document.application,
        template: document.template,
        auxiliary_header: document.auxiliary_header,
        revision_history: document.revision_history,
        preview: document.preview,
        application_history: document.application_history,
    })
}
//#endregion 📸️SnapshotDecode
