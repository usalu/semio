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

use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::{dwg_version_sentinel, DwgBitReader};
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::parse_version_header_fields;
use semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::{DwgDrawing, DwgDrawingExtents};
use semio_s_artifact_stdio_dwg_snapshot_drawing::standards::v_ac1024::subsets::any::schema::snapshot::drawing::{DwgEntityProjection, DwgEntityRead, DwgLogicalDrawing};
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::schema::snapshot::DwgSnapshot;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::schema::snapshot::DWG_PREAMBLE_LEN;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::schema::snapshot::STDIO_DWG_DOCUMENT_SCHEMA;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::container::decode_r2004_sections;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::container::DwgDrawingSectionStep;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::container::DwgDrawingSections;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::container::r2004_section_data;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::document::decode_r2004_classes;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::document::decode_r2004_document_sections;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::DWG_TYPE_LAYER;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::{begin_entity_body, DwgEntityRecordRead, DwgObjectLayout};
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::{DwgObjectFrames, DwgObjectRecordFrame};
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::decode_layer_projection;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::HandleMapRows;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgLayerTableRecord;
use semio_s_artifact_stdio_dwg_snapshot_drawing::standards::v_ac1024::subsets::any::schema::snapshot::drawing::geometry::DwgLogicalLayer;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgObjectCategory;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableRecordBody;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableRecordCommon;
use semio_s_artifact_stdio_dwg_read_objects::standards::v_ac1024::subsets::any::io::import::objects::decode_r2004_object_identities;


//#region 🔖️DwgStructuralCodec
//#region DwgObjects
/// 📐️ Reads the drawing of a DWG file of any version this codec supports.
pub fn dwg_from_bytes(bytes: &[u8]) -> Result<DwgDrawing, String> {
    let mut read = DwgDrawingRead::open(bytes)?;
    loop {
        if let DwgDrawingReadStep::Complete(drawing) = read.step()? {
            return Ok(drawing);
        }
    }
}

/// 🔖️ Progress while projecting drawing pages and handle records into native geometry.
pub enum DwgDrawingReadStep {
    Directory,
    PageData,
    Page,
    Padding,
    RecordData,
    Record,
    Geometry,
    Complete(DwgDrawing),
}

enum DrawingReadPhase<Bytes> {
    Sections(DwgDrawingSections<Bytes>),
    Records { handles: HandleMapRows<Vec<u8>>, frames: DwgObjectFrames, record: DrawingRecordProjection },
    Projection(DrawingProjection),
    Complete,
    Failed(String),
}

#[derive(Default)]
struct DrawingRecordProjection {
    extended_bit: Option<usize>,
    entity: Option<DwgEntityRecordRead>,
}

#[derive(Default)]
struct DrawingProjection {
    layer: usize,
    object: usize,
    entity: Option<DwgEntityRead>,
    drawing: DwgDrawing,
    extents: DwgDrawingExtents,
}

/// 🏗️ Keeps one DWG byte source and projects pages, records and native geometry incrementally.
pub struct DwgDrawingRead<Bytes> {
    layout: DwgObjectLayout,
    phase: DrawingReadPhase<Bytes>,
    logical: DwgLogicalDrawing,
    entity_index: DwgEntityProjection,
}

impl<Bytes: AsRef<[u8]>> DwgDrawingRead<Bytes> {
    pub fn open(bytes: Bytes) -> Result<Self, String> {
        let version = String::from_utf8_lossy(bytes.as_ref().get(0..6).ok_or("DWG file is shorter than its version sentinel")?);
        let layout = DwgObjectLayout::of(&version)?;
        let sections = DwgDrawingSections::open(bytes, layout == DwgObjectLayout::R2007)?;
        let logical = DwgLogicalDrawing { layers: Vec::new(), objects: Vec::new(), extmin: vec![0.0; 3], extmax: vec![0.0; 3] };
        Ok(Self { layout, phase: DrawingReadPhase::Sections(sections), logical, entity_index: Default::default() })
    }

    /// 📈 Reports progress through physical pages, object records and native geometry.
    pub fn progress(&self) -> f32 {
        let fraction = |(done, total): (usize, usize)| done as f32 / total.max(1) as f32;
        match &self.phase {
            DrawingReadPhase::Sections(sections) => sections.fraction() / 3.0,
            DrawingReadPhase::Records { handles, .. } => (1.0 + fraction(handles.progress())) / 3.0,
            DrawingReadPhase::Projection(projection) => {
                let partial = projection.entity.as_ref().map_or(0.0, |entity| fraction(entity.progress()));
                let done = (projection.layer + projection.object) as f32 + partial;
                let total = (self.logical.layers.len() + self.logical.objects.len()).max(1) as f32;
                (2.0 + done / total) / 3.0
            }
            DrawingReadPhase::Complete => 1.0,
            DrawingReadPhase::Failed(_) => 0.0,
        }
    }

    /// ⏭️ Advances one physical page, padding chunk, drawing record, or native geometry row.
    pub fn step(&mut self) -> Result<DwgDrawingReadStep, String> {
        match &self.phase {
            DrawingReadPhase::Complete => return Err("DWG drawing was already consumed".into()),
            DrawingReadPhase::Failed(error) => return Err(error.clone()),
            _ => {}
        }
        let result = self.advance();
        if let Err(error) = &result {
            self.phase = DrawingReadPhase::Failed(error.clone());
        }
        result
    }

    fn advance(&mut self) -> Result<DwgDrawingReadStep, String> {
        if let DrawingReadPhase::Sections(sections) = &mut self.phase {
            self.phase = match sections.step()? {
                DwgDrawingSectionStep::Directory => return Ok(DwgDrawingReadStep::Directory),
                DwgDrawingSectionStep::PageData => return Ok(DwgDrawingReadStep::PageData),
                DwgDrawingSectionStep::Page => return Ok(DwgDrawingReadStep::Page),
                DwgDrawingSectionStep::Padding => return Ok(DwgDrawingReadStep::Padding),
                DwgDrawingSectionStep::Complete { handles, objects } => DrawingReadPhase::Records { handles: HandleMapRows::new(handles), frames: DwgObjectFrames::new(objects, self.layout), record: Default::default() },
            };
        }
        if let DrawingReadPhase::Records { handles, frames, record } = &mut self.phase {
            if !frames.is_pending() {
                if let Some(location) = handles.next() {
                    let (handle, address) = location?;
                    frames.start(handle, address)?;
                }
            }
            if frames.is_pending() {
                let index = self.logical.objects.len();
                let logical = &mut self.logical;
                let pending = frames.step(|handle, mut frame| project_drawing_record(logical, record, &mut frame, handle))?;
                if pending.is_pending() { return Ok(DwgDrawingReadStep::RecordData); }
                *record = DrawingRecordProjection::default();
                if let Some(object) = self.logical.objects.get(index) {
                    self.entity_index.index(index, object);
                }
                return Ok(DwgDrawingReadStep::Record);
            }
            if self.logical.layers.is_empty() {
                self.logical.layers.push(DwgLogicalLayer { name: "0".into(), color: 7 });
            }
            self.phase = DrawingReadPhase::Projection(DrawingProjection::default());
        }
        let DrawingReadPhase::Projection(projection) = &mut self.phase else { return Err("DWG drawing was already consumed".into()) };
        if let Some(layer) = self.logical.layers.get(projection.layer) {
            projection.drawing.layers.push(layer.to_native());
            projection.layer += 1;
            return Ok(DwgDrawingReadStep::Geometry);
        }
        if projection.object < self.logical.objects.len() {
            if projection.entity.is_none() {
                projection.entity = self.entity_index.begin(&self.logical.objects, projection.object);
            }
            if let Some(cursor) = &mut projection.entity {
                let std::task::Poll::Ready(entity) = cursor.step(&self.entity_index, &self.logical.objects, &mut projection.extents)? else {
                    return Ok(DwgDrawingReadStep::Geometry);
                };
                projection.drawing.entities.push(entity);
                projection.entity = None;
            }
            projection.object += 1;
            return Ok(DwgDrawingReadStep::Geometry);
        }
        projection.extents.apply(&mut projection.drawing);
        let drawing = std::mem::take(&mut projection.drawing);
        self.phase = DrawingReadPhase::Complete;
        Ok(DwgDrawingReadStep::Complete(drawing))
    }
}

fn project_drawing_record(logical: &mut DwgLogicalDrawing, record: &mut DrawingRecordProjection, frame: &mut DwgObjectRecordFrame<'_>, handle: u64) -> Result<std::task::Poll<()>, String> {
    use std::task::Poll;
    if frame.type_code != DWG_TYPE_LAYER && !matches!(frame.type_code, 1 | 6 | 11 | 13 | 14 | 16..=19 | 27..=29 | 35 | 36 | 77) {
        return Ok(Poll::Ready(()));
    }
    if record.entity.is_none() {
        if let Some(bit) = record.extended_bit {
            frame.data = DwgBitReader::at_bit(frame.data.bytes, bit)?;
        }
        if !frame.skip_extended_block().map_err(|error| format!("object {handle:#x} extended data: {error}"))? {
            record.extended_bit = Some(frame.data.bit_position());
            return Ok(Poll::Pending);
        }
        if frame.type_code != DWG_TYPE_LAYER {
            record.entity = begin_entity_body(frame, true)?;
        }
    }
    if frame.type_code == DWG_TYPE_LAYER {
        let layer = decode_layer_projection(frame).map_err(|error| format!("LAYER {handle:#x}: {error}"))?;
        logical.objects.push(DwgLogicalObject {
            handle,
            type_code: DWG_TYPE_LAYER,
            class_name: "LAYER".into(),
            category: DwgObjectCategory::TableRecord,
            body: Some(DwgLogicalObjectBody::TableRecord(DwgTableRecordBody::Layer(DwgLayerTableRecord { common: DwgTableRecordCommon { name: layer.name.clone(), ..Default::default() }, ..Default::default() }))),
            ..Default::default()
        });
        logical.layers.push(DwgLogicalLayer { name: layer.name, color: layer.color });
    } else if let Some(entity) = &mut record.entity {
        if !entity.step(frame)? { return Ok(Poll::Pending); }
        let (body, (owner_handle, _, _)) = record.entity.take().and_then(DwgEntityRecordRead::into_body).ok_or("DWG record links are incomplete")?;
        logical.objects.push(DwgLogicalObject { handle, type_code: frame.type_code, category: DwgObjectCategory::Entity, owner_handle, body: Some(DwgLogicalObjectBody::Entity(body)), ..Default::default() });
    }
    Ok(Poll::Ready(()))
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
