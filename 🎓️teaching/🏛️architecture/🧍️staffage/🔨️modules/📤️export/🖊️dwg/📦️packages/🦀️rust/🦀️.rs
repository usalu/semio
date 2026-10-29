//! 🖊️ Staffage DWG export: a `ModelScene` (metres) becomes a genuine AC1024 drawing through the in-repo Rust writer
//! (`semio-s-artifact-stdio-dwg-write`), with insertion units set to metres and the ModelScene lineweights on the layers.
//! The browser reaches it through the raw C-ABI in [`wasm`], the Node CLI through `staffage-dwg`.
//! <https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf>

use semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::{DwgColor, DwgDrawing, DwgEntity, DwgGeometry};
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::schema::snapshot::DwgSnapshot;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableRecordBody;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
use semio_s_artifact_stdio_dwg_write::standards::v_ac1024::subsets::any::io::export::encode_dwg;
use serde::Deserialize;

#[cfg(target_arch = "wasm32")]
#[path = "🌐️wasm/🦀️.rs"]
pub mod wasm;

/// 📍️ A model point in metres.
#[derive(Deserialize, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// 〰️ An open or closed polyline.
#[derive(Deserialize)]
pub struct Polyline {
    pub points: Vec<Point>,
    pub closed: bool,
}

/// 🔤️ A single-line text.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub at: Point,
    pub height_m: f64,
    pub text: String,
}

/// 🗂️ A CAD layer: ACI colour, lineweight in 1/100 mm, polylines and texts.
#[derive(Deserialize)]
pub struct Layer {
    pub name: String,
    pub aci: u8,
    pub lineweight: u32,
    pub polylines: Vec<Polyline>,
    pub texts: Vec<Text>,
}

/// 📐️ The JSON twin of the TypeScript `ModelScene`.
#[derive(Deserialize)]
pub struct ModelScene {
    pub units: String,
    pub layers: Vec<Layer>,
}

const LINEWEIGHTS: [u32; 24] = [0, 5, 9, 13, 15, 18, 20, 25, 30, 35, 40, 50, 53, 60, 70, 80, 90, 100, 106, 120, 140, 158, 200, 211];

/// 🔢️ The DWG lineweight enumeration index of the nearest standard weight (1/100 mm).
pub fn lineweight_index(hundredths_mm: u32) -> u8 {
    LINEWEIGHTS.iter().enumerate().min_by_key(|(_, w)| w.abs_diff(hundredths_mm)).map_or(31, |(index, _)| index as u8)
}

fn drawing(scene: &ModelScene) -> DwgDrawing {
    let mut drawing = DwgDrawing::default();
    drawing.ensure_layer("0");
    for layer in &scene.layers {
        let index = drawing.ensure_layer(&layer.name);
        drawing.layers[index].color = layer.aci;
        for line in &layer.polylines {
            let vertices: Vec<[f64; 2]> = line.points.iter().map(|p| [p.x, p.y]).collect();
            let bulges = vec![0.0; vertices.len()];
            drawing.entities.push(DwgEntity { layer: index, color: DwgColor::ByLayer, geometry: DwgGeometry::LwPolyline { closed: line.closed, elevation: 0.0, vertices, bulges } });
        }
        for text in &layer.texts {
            drawing.entities.push(DwgEntity { layer: index, color: DwgColor::ByLayer, geometry: DwgGeometry::Text { at: [text.at.x, text.at.y, 0.0], height: text.height_m, rotation: 0.0, content: text.text.clone() } });
        }
    }
    drawing
}

/// 🧱️ Encodes a parsed scene as AC1024 bytes.
pub fn encode_scene(scene: &ModelScene) -> Result<Vec<u8>, String> {
    if scene.units != "m" {
        return Err(format!("ModelScene units must be \"m\", got {:?}", scene.units));
    }
    let mut snapshot = DwgSnapshot::from_drawing(&drawing(scene))?;
    snapshot.header.policy.insertion_units = 6;
    for object in &mut snapshot.drawing.objects {
        if let Some(DwgLogicalObjectBody::TableRecord(DwgTableRecordBody::Layer(record))) = object.body.as_mut() {
            if let Some(layer) = scene.layers.iter().find(|layer| layer.name == record.common.name) {
                record.lineweight = lineweight_index(layer.lineweight);
            }
        }
    }
    encode_dwg(&snapshot).map_err(|error| error.to_string())
}

/// 📥️ Encodes ModelScene JSON as AC1024 bytes.
pub fn encode_json(json: &[u8]) -> Result<Vec<u8>, String> {
    encode_scene(&serde_json::from_slice(json).map_err(|error| format!("ModelScene JSON: {error}"))?)
}
