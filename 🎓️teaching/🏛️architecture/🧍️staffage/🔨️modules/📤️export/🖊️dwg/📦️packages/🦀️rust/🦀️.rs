//! 🖊️ Staffage DWG export: a `ModelScene` (metres) becomes a genuine AC1024 (AutoCAD 2010) drawing written by this crate alone, with insertion units set to metres, one layer per scene layer with its ACI colour and lineweight, LWPOLYLINE and TEXT entities in model space, and the standard tables, blocks, layouts and dictionaries readers expect.
//! The crate has no runtime dependencies: the bit codes, checksums, compression, JSON reader and container are its own; `acadrust` and `ezdwg` are independent readers used only as test oracles.
//! The browser reaches it through the raw C-ABI in [`wasm`], the Node CLI through `staffage-dwg`.
//! <https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf>

#[path = "🧮️bits/🦀️.rs"]
pub mod bits;
#[path = "🔐️checksum/🦀️.rs"]
pub mod checksum;
#[path = "📦️container/🦀️.rs"]
pub mod container;
#[path = "🔣️json/🦀️.rs"]
pub mod json;
#[path = "🗜️lz77/🦀️.rs"]
pub mod lz77;
#[path = "🧱️objects/🦀️.rs"]
pub mod objects;
#[path = "🗂️scene/🦀️.rs"]
pub mod scene;
#[path = "📚️sections/🦀️.rs"]
pub mod sections;
#[cfg(target_arch = "wasm32")]
#[path = "🌐️wasm/🦀️.rs"]
pub mod wasm;

pub use objects::lineweight_index;
pub use scene::{Layer, ModelScene, Point, Polyline, Text};

use container::Section;

/// 🧱️ Encodes a validated scene as AC1024 bytes; the same scene always yields the same bytes.
pub fn encode_scene(scene: &ModelScene, identity: &[u8]) -> Result<Vec<u8>, String> {
    let database = objects::build(scene);
    let mut object_section = 0x0DCAu32.to_le_bytes().to_vec();
    let mut offsets = Vec::with_capacity(database.records.len());
    for (handle, bytes) in &database.records {
        offsets.push((*handle, object_section.len() as u32));
        object_section.extend_from_slice(bytes);
    }
    let header = sections::header(&database, &sections::guid(identity, 1), &sections::guid(identity, 2));
    let classes = sections::classes(database.polylines);
    let handles = sections::handles(&offsets);
    let (summary, app_info, dependencies, revisions) = (sections::summary(), sections::app_info(), sections::file_dependencies(), sections::revision_history());
    let (aux, free, template) = (sections::aux_header(database.next_handle), sections::free_space(offsets.len()), sections::template());
    let page = 0x7400;
    let sections = [
        Section { name: "AcDb:SummaryInfo", data: &summary, compressed: false, page: 0x100 },
        Section { name: "AcDb:AppInfo", data: &app_info, compressed: false, page: 0x80 },
        Section { name: "AcDb:FileDepList", data: &dependencies, compressed: false, page: 0x80 },
        Section { name: "AcDb:RevHistory", data: &revisions, compressed: true, page },
        Section { name: "AcDb:AcDbObjects", data: &object_section, compressed: true, page },
        Section { name: "AcDb:ObjFreeSpace", data: &free, compressed: true, page },
        Section { name: "AcDb:Template", data: &template, compressed: true, page },
        Section { name: "AcDb:Handles", data: &handles, compressed: true, page },
        Section { name: "AcDb:Classes", data: &classes, compressed: true, page },
        Section { name: "AcDb:AuxHeader", data: &aux, compressed: true, page },
        Section { name: "AcDb:Header", data: &header, compressed: true, page },
    ];
    container::write(&sections, "AcDb:SummaryInfo")
}

/// 📥️ Encodes ModelScene JSON as AC1024 bytes.
pub fn encode_json(json: &[u8]) -> Result<Vec<u8>, String> {
    encode_scene(&ModelScene::from_json(json)?, json)
}
