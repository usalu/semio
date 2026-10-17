//! 🖊️ The one place where the layout plugin and the Semio drawing standard meet: the codec the hub binds to the layout background drawing slot.
//!
//! Layout carries its background trace as opaque content and speaks only to the framework port (`semio_framework_io_ports`). This crate knows both owners:
//! it validates content as a `SemioDrawingSnapshot`, lowers it to and lifts it from the neutral vector scene, contributes the drawing's SQLite component and
//! reads and writes the external formats (svg, dxf, dwg, png) through the drawing subset's own import and export leaves.
//!
//! @see 🔌️ `🧰️framework/🔨️modules/🚪️io/🔌️ports/🦀️.rs` for the contract and `✏️s/🧪️tests/🏘️plugin-composition/📏️layout-drawing/🔗️binding/🟦️.ts` for its TypeScript twin.

#[cfg(test)]
extern crate semio_framework_os_kernel as store;

use semio_framework_io_ports::{EmbeddedChildCodecV1, EmbeddedChildSlotError, VectorCanvas, VectorLayer, VectorNode, VectorPoint2, VectorPoint3, VectorQuaternion, VectorRgba, VectorScene, VectorSegment, VectorStyle, VectorTransform};
use semio_framework_io_sqlite_snapshot::artifact::RowWriter;
use semio_framework_io_sqlite_snapshot::{SqliteDatabase, SqliteSnapshotControl};
use semio_framework_value::{DecodedValue, DslValue, FromValue, NativeDecodeControl, ToValue, ValueError};
use semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::{DwgDrawing, DwgGeometry};
use semio_s_artifact_stdio_dwg_read::standards::v_ac1024::subsets::any::io::import::{decode_dwg, dwg_from_bytes};
use semio_s_artifact_stdio_semio_base::schema::geometry::{SemioPoint2, SemioPoint3, SemioQuaternion, SemioRgba, SemioTransform};
use semio_s_artifact_stdio_semio_drawing::io::SemioDrawingFormat;
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio_drawing_export::encode_drawing;
use semio_s_artifact_stdio_semio_drawing_import::decode_drawing;

//#region 🔖️Binding
/// 🖊️ The Semio drawing standard's complete validate, lower, lift, SQLite component and external format facets.
pub struct SemioDrawingCodec;

/// 🔗️ The one instance the hub binds.
pub static SEMIO_DRAWING_CODEC: SemioDrawingCodec = SemioDrawingCodec;

/// 🔗️ Idempotently fills the layout background drawing slot with the Semio drawing codec.
pub fn bind_layout_background_drawing() -> Result<(), EmbeddedChildSlotError> {
    semio_s_artifact_layout_layout_model::LAYOUT_BACKGROUND_DRAWING.bind(&SEMIO_DRAWING_CODEC)
}

impl EmbeddedChildCodecV1 for SemioDrawingCodec {
    fn validate(&self, content: &DslValue, control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> {
        let decoded = DecodedValue::new(<SemioDrawingSnapshot as FromValue>::from_value_controlled(content, control)?, retire_snapshot);
        drop(decoded);
        Ok(())
    }

    fn lower(&self, content: &DslValue) -> Result<VectorScene, ValueError> {
        <SemioDrawingSnapshot as FromValue>::from_value(content.clone()).map(lower_snapshot)
    }

    fn lift(&self, scene: &VectorScene) -> Result<DslValue, ValueError> {
        Ok(lift_scene(scene).to_value())
    }

    fn sqlite_schema(&self) -> &'static str {
        <SemioDrawingSnapshot as pack::snapshot::ArtifactSqliteSnapshot>::SQLITE_SCHEMA
    }

    fn sqlite_owns_table(&self, table: &str) -> bool {
        table.starts_with("semio_drawing_")
    }

    fn sqlite_project(&self, content: &DslValue, writer: &mut RowWriter<'_, '_>) -> Result<(), ValueError> {
        <SemioDrawingSnapshot as FromValue>::from_value(content.clone())?.project_sqlite_component(writer)
    }

    fn sqlite_reconstruct(&self, database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>, declared_schema: &str) -> Result<DslValue, ValueError> {
        SemioDrawingSnapshot::reconstruct_sqlite_database(database, control, declared_schema).map(|snapshot| snapshot.to_value())
    }

    fn import_formats(&self) -> &'static [&'static str] {
        &["svg", "dxf", "dwg"]
    }

    fn export_formats(&self) -> &'static [&'static str] {
        &["svg", "dwg", "dxf", "png"]
    }

    fn import(&self, format: &str, bytes: &[u8]) -> Result<DslValue, String> {
        let snapshot = match format {
            "svg" => decode_drawing(bytes, SemioDrawingFormat::Svg)?,
            "dxf" => decode_drawing(bytes, SemioDrawingFormat::Dxf)?,
            "dwg" => import_dwg(bytes)?,
            other => return Err(format!("the Semio drawing codec cannot import {other}")),
        };
        Ok(snapshot.to_value())
    }

    fn export(&self, format: &str, content: &DslValue) -> Result<Vec<u8>, String> {
        let format = match format {
            "svg" => SemioDrawingFormat::Svg,
            "dwg" => SemioDrawingFormat::Dwg,
            "dxf" => SemioDrawingFormat::Dxf,
            "png" => SemioDrawingFormat::Png,
            other => return Err(format!("the Semio drawing codec cannot export {other}")),
        };
        let snapshot = <SemioDrawingSnapshot as FromValue>::from_value(content.clone()).map_err(|error| error.to_string())?;
        encode_drawing(&snapshot, format)
    }
}

/// ♻️ Releases a validated or replaced drawing through the controlled cold retirement, so a deep tree never unwinds on the caller's stack.
fn retire_snapshot(value: SemioDrawingSnapshot) {
    let mut owner = semio_framework_value::retirement::controlled::ControlledRetirement::new(value).unwrap_or_else(|(error, _)| panic!("Semio drawing cold retirement refused: {error}"));
    while !owner.terminal_is_empty() {
        let copy = owner.next_copy_byte_demand().expect("Semio drawing cold copy demand");
        let release = owner.next_release_byte_demand().expect("Semio drawing cold release demand");
        let capacity = owner.next_capacity_byte_demand(if copy == 0 { release } else { copy }).expect("Semio drawing cold capacity demand");
        let depth = owner.next_depth_demand().expect("Semio drawing cold depth demand");
        owner.step(semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: depth }).expect("Semio drawing cold grant");
    }
}
//#endregion 🔖️Binding

//#region 🔖️Scene
fn point2(point: SemioPoint2) -> VectorPoint2 {
    VectorPoint2 { x: point.x, y: point.y }
}

fn point3(point: SemioPoint3) -> VectorPoint3 {
    VectorPoint3 { x: point.x, y: point.y, z: point.z }
}

fn quaternion(rotation: SemioQuaternion) -> VectorQuaternion {
    VectorQuaternion { x: rotation.x, y: rotation.y, z: rotation.z, w: rotation.w }
}

fn color(rgba: SemioRgba) -> VectorRgba {
    VectorRgba { r: rgba.r, g: rgba.g, b: rgba.b, a: rgba.a }
}

fn transform(value: SemioTransform) -> VectorTransform {
    VectorTransform { translation: point3(value.translation), rotation: quaternion(value.rotation), scale: point3(value.scale) }
}

fn segment(value: PathSegment) -> VectorSegment {
    match value {
        PathSegment::MoveTo { to } => VectorSegment::MoveTo { to: point2(to) },
        PathSegment::LineTo { to } => VectorSegment::LineTo { to: point2(to) },
        PathSegment::CubicTo { c1, c2, to } => VectorSegment::CubicTo { c1: point2(c1), c2: point2(c2), to: point2(to) },
        PathSegment::QuadTo { c, to } => VectorSegment::QuadTo { c: point2(c), to: point2(to) },
        PathSegment::ArcTo { rx, ry, x_rotation, large_arc, sweep, to } => VectorSegment::ArcTo { rx, ry, x_rotation, large_arc, sweep, to: point2(to) },
        PathSegment::Close => VectorSegment::Close,
    }
}

fn node(value: DrawNode) -> VectorNode {
    match value {
        DrawNode::Path { segments, style } => VectorNode::Path { segments: segments.into_iter().map(segment).collect(), style },
        DrawNode::Text { value, at, style } => VectorNode::Text { value, at: point2(at), style },
        DrawNode::Group { transform: group, children } => VectorNode::Group { transform: transform(group), children: children.into_iter().map(node).collect() },
        DrawNode::Image { at, width, height, mime, bytes } => VectorNode::Image { at: point2(at), width, height, mime, bytes },
    }
}

fn lower_snapshot(snapshot: SemioDrawingSnapshot) -> VectorScene {
    VectorScene {
        schema: snapshot.schema,
        canvas: VectorCanvas { width: snapshot.canvas.width, height: snapshot.canvas.height, background: snapshot.canvas.background.map(color) },
        styles: snapshot.styles.into_iter().map(|style| VectorStyle { name: style.name, fill: style.fill.map(color), stroke: style.stroke.map(color), stroke_width: style.stroke_width, opacity: style.opacity }).collect(),
        layers: snapshot.layers.into_iter().map(|layer| VectorLayer { id: layer.id, name: layer.name, visible: layer.visible, root: node(layer.root) }).collect(),
    }
}

fn lifted_point2(point: VectorPoint2) -> SemioPoint2 {
    SemioPoint2 { x: point.x, y: point.y }
}

fn lifted_point3(point: VectorPoint3) -> SemioPoint3 {
    SemioPoint3 { x: point.x, y: point.y, z: point.z }
}

fn lifted_color(rgba: VectorRgba) -> SemioRgba {
    SemioRgba { r: rgba.r, g: rgba.g, b: rgba.b, a: rgba.a }
}

fn lifted_transform(value: VectorTransform) -> SemioTransform {
    SemioTransform { translation: lifted_point3(value.translation), rotation: SemioQuaternion { x: value.rotation.x, y: value.rotation.y, z: value.rotation.z, w: value.rotation.w }, scale: lifted_point3(value.scale) }
}

fn lifted_segment(value: VectorSegment) -> PathSegment {
    match value {
        VectorSegment::MoveTo { to } => PathSegment::MoveTo { to: lifted_point2(to) },
        VectorSegment::LineTo { to } => PathSegment::LineTo { to: lifted_point2(to) },
        VectorSegment::CubicTo { c1, c2, to } => PathSegment::CubicTo { c1: lifted_point2(c1), c2: lifted_point2(c2), to: lifted_point2(to) },
        VectorSegment::QuadTo { c, to } => PathSegment::QuadTo { c: lifted_point2(c), to: lifted_point2(to) },
        VectorSegment::ArcTo { rx, ry, x_rotation, large_arc, sweep, to } => PathSegment::ArcTo { rx, ry, x_rotation, large_arc, sweep, to: lifted_point2(to) },
        VectorSegment::Close => PathSegment::Close,
    }
}

fn lifted_node(value: &VectorNode) -> DrawNode {
    match value {
        VectorNode::Path { segments, style } => DrawNode::Path { segments: segments.iter().copied().map(lifted_segment).collect(), style: style.clone() },
        VectorNode::Text { value, at, style } => DrawNode::Text { value: value.clone(), at: lifted_point2(*at), style: style.clone() },
        VectorNode::Group { transform, children } => DrawNode::Group { transform: lifted_transform(*transform), children: children.iter().map(lifted_node).collect() },
        VectorNode::Image { at, width, height, mime, bytes } => DrawNode::Image { at: lifted_point2(*at), width: *width, height: *height, mime: mime.clone(), bytes: bytes.clone() },
    }
}

fn lift_scene(scene: &VectorScene) -> SemioDrawingSnapshot {
    SemioDrawingSnapshot {
        schema: if scene.schema.is_empty() { STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into() } else { scene.schema.clone() },
        canvas: DrawCanvas { width: scene.canvas.width, height: scene.canvas.height, background: scene.canvas.background.map(lifted_color) },
        styles: scene.styles.iter().map(|style| DrawStyle { name: style.name.clone(), fill: style.fill.map(lifted_color), stroke: style.stroke.map(lifted_color), stroke_width: style.stroke_width, opacity: style.opacity }).collect(),
        layers: scene.layers.iter().map(|layer| DrawLayer { id: layer.id.clone(), name: layer.name.clone(), visible: layer.visible, root: lifted_node(&layer.root) }).collect(),
    }
}
//#endregion 🔖️Scene

//#region 🔖️Dwg
/// 📐️ A closed axis-aligned rectangle as `MoveTo` + three `LineTo`s + `Close`.
fn rect_path_segments(x: f64, y: f64, width: f64, height: f64) -> Vec<PathSegment> {
    vec![
        PathSegment::MoveTo { to: SemioPoint2 { x, y } },
        PathSegment::LineTo { to: SemioPoint2 { x: x + width, y } },
        PathSegment::LineTo { to: SemioPoint2 { x: x + width, y: y + height } },
        PathSegment::LineTo { to: SemioPoint2 { x, y: y + height } },
        PathSegment::Close,
    ]
}

/// 📥️ Extracts axis-aligned rectangular boundaries from closed 4-vertex `LwPolyline`s, falling back to a single page framed to the drawing extents. Reads an already-decoded `DwgDrawing` (real geometry, not raw bytes).
fn dwg_rect_pages(drawing: &DwgDrawing) -> Vec<(f64, f64, f64, f64)> {
    let mut rects = Vec::new();
    for entity in &drawing.entities {
        let DwgGeometry::LwPolyline { closed: true, vertices, .. } = &entity.geometry else { continue };
        if vertices.len() != 4 {
            continue;
        }
        let (min_x, max_x) = vertices.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), v| (min.min(v[0]), max.max(v[0])));
        let (min_y, max_y) = vertices.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), v| (min.min(v[1]), max.max(v[1])));
        let is_axis_aligned = vertices.iter().all(|v| ((v[0] - min_x).abs() < 1e-6 || (v[0] - max_x).abs() < 1e-6) && ((v[1] - min_y).abs() < 1e-6 || (v[1] - max_y).abs() < 1e-6));
        if is_axis_aligned && max_x > min_x && max_y > min_y {
            rects.push((min_x, min_y, max_x - min_x, max_y - min_y));
        }
    }
    if rects.is_empty() {
        rects.push((drawing.extmin[0], drawing.extmin[1], (drawing.extmax[0] - drawing.extmin[0]).max(1.0), (drawing.extmax[1] - drawing.extmin[1]).max(1.0)));
    }
    rects
}

/// 🖍️ Builds a real `SemioDrawingSnapshot` from the rectangular page boundaries `dwg_rect_pages` detects — one flat layer, one unstyled rect-shaped `Path` per detected page.
/// The drawing subset registers no `drawing↔dwg` composer entry, so the already-decoded `DwgDrawing` geometry is funneled through the schema-owning `SemioDrawingSnapshot`/`DrawNode` shape instead of a bespoke tuple list.
pub fn dwg_drawing_to_semio_drawing(drawing: &DwgDrawing) -> SemioDrawingSnapshot {
    let children: Vec<DrawNode> = dwg_rect_pages(drawing).into_iter().map(|(x, y, width, height)| DrawNode::Path { segments: rect_path_segments(x, y, width, height), style: None }).collect();
    SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: DrawCanvas::default(),
        styles: Vec::new(),
        layers: vec![DrawLayer { id: "imported".into(), name: "Imported".into(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children } }],
    }
}

fn import_dwg(bytes: &[u8]) -> Result<SemioDrawingSnapshot, String> {
    decode_dwg(bytes).map_err(|error| error.to_string())?;
    let drawing = dwg_from_bytes(bytes).map_err(|error| error.to_string())?;
    Ok(dwg_drawing_to_semio_drawing(&drawing))
}
//#endregion 🔖️Dwg

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
