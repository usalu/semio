//! 🖼️ SVG rendering and planar geometry extraction for artifact I/O.
// #region media_export_raster

//#region 🔖️MediaRegistry
// 🧬️ The workflow registry owns `register_*` and `export_os_app_instance_media_kind`, so both share one OnceLock. Keyed on
// string format kind ids (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6).
pub use crate::workflow::{export_os_app_instance_media_kind, import_os_app_instance_media_kind, register_os_media_export_handler_kind, register_os_media_import_handler_kind, OsMediaExportResult};

/// 🗂️ Build a file-picker `accept` filter from stdio format kind ids (`dwg` / `stdio.dwg`).
pub fn media_accept_filter_kinds(format_artifact_kinds: &[&str]) -> Result<String, semio_framework_os_kernel::io::FormatRegistryError> {
    semio_framework_os_kernel::io::format_accept_filter(format_artifact_kinds)
}
//#endregion 🔖️MediaRegistry
use serde_json::Value;

/// 🖼️ Rasterizes SVG markup to a base64-encoded PNG payload.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn rasterize_svg_to_png_base64(svg: &str, width: u32, height: u32) -> Result<String, String> {
    let tree = usvg::Tree::from_str(svg, &usvg::Options::default()).map_err(|error| error.to_string())?;
    let size = tree.size();
    let render_w = if width > 0 { width } else { size.width().ceil().max(1.0) as u32 };
    let render_h = if height > 0 { height } else { size.height().ceil().max(1.0) as u32 };
    let mut pixmap = tiny_skia::Pixmap::new(render_w, render_h).ok_or_else(|| "invalid raster dimensions".to_string())?;
    let scale_x = render_w as f32 / size.width().max(1.0);
    let scale_y = render_h as f32 / size.height().max(1.0);
    resvg::render(&tree, tiny_skia::Transform::from_scale(scale_x, scale_y), &mut pixmap.as_mut());
    let png_bytes = encode_rgba_png(pixmap.data(), pixmap.width(), pixmap.height())?;
    Ok(base64_codec::base64_standard_encode(png_bytes))
}

/// 🖼️ Preserves the raster-export API where the shipped guest has no native renderer tier.
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub fn rasterize_svg_to_png_base64(_: &str, _: u32, _: u32) -> Result<String, String> {
    Err("SVG rasterization requires the native semio-framework-os host".into())
}

/// 📤️ Encodes the renderer's RGBA8 pixmap with the framework's own PNG codec.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn encode_rgba_png(pixels: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
    semio_framework_pixels::encode_png(&semio_framework_pixels::RasterImage { width, height, pixels: pixels.to_vec() }).map_err(|error| error.to_string())
}

/// 📏️ One transformed SVG polyline in its source layer.
#[derive(Debug, PartialEq)]
pub struct SvgPolyline {
    pub layer: String,
    pub vertices: Vec<[f64; 2]>,
    pub closed: bool,
}

/// 📐️ Extracts transformed SVG paths as layered planar polylines.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn svg_to_polylines(svg: &str) -> Result<Vec<SvgPolyline>, String> {
    let tree = usvg::Tree::from_str(svg, &usvg::Options::default()).map_err(|error| error.to_string())?;
    let mut polylines = Vec::new();
    let height = tree.size().height() as f64;
    collect_svg_children(tree.root().children(), &mut polylines, "0", height);
    Ok(polylines)
}

/// 🪟️ Reports an unavailable native SVG parser tier.
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub fn svg_to_polylines(_: &str) -> Result<Vec<SvgPolyline>, String> {
    Err("SVG path extraction requires the native semio-framework-os host".into())
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn collect_svg_children(nodes: &[usvg::Node], polylines: &mut Vec<SvgPolyline>, layer: &str, height: f64) {
    for node in nodes {
        match node {
            usvg::Node::Group(group) => {
                let id = node.id();
                let group_layer = if id.is_empty() { layer } else { id };
                collect_svg_children(group.children(), polylines, group_layer, height);
            }
            usvg::Node::Path(path) => collect_svg_path(path, polylines, layer, height),
            _ => {}
        }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn collect_svg_path(path: &usvg::Path, polylines: &mut Vec<SvgPolyline>, layer: &str, height: f64) {
    let transform = path.abs_transform();
    let mut vertices: Vec<[f64; 2]> = Vec::new();
    let mut closed = false;
    for segment in path.data().segments() {
        match segment {
            usvg::tiny_skia_path::PathSegment::MoveTo(p) => {
                flush_svg_polyline(polylines, layer, &mut vertices, &mut closed);
                vertices.push(transformed_svg_point(transform, p, height));
            }
            usvg::tiny_skia_path::PathSegment::LineTo(p) => {
                vertices.push(transformed_svg_point(transform, p, height));
            }
            usvg::tiny_skia_path::PathSegment::QuadTo(c, p) => {
                flatten_quad_into(&mut vertices, transform, c, p, height);
            }
            usvg::tiny_skia_path::PathSegment::CubicTo(c1, c2, p) => {
                flatten_cubic_into(&mut vertices, transform, c1, c2, p, height);
            }
            usvg::tiny_skia_path::PathSegment::Close => {
                closed = true;
            }
        }
    }
    flush_svg_polyline(polylines, layer, &mut vertices, &mut closed);
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn transformed_svg_point(transform: usvg::Transform, point: usvg::tiny_skia_path::Point, height: f64) -> [f64; 2] {
    let mut p = point;
    transform.map_point(&mut p);
    [p.x as f64, height - p.y as f64]
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn flatten_quad_into(vertices: &mut Vec<[f64; 2]>, transform: usvg::Transform, ctrl: usvg::tiny_skia_path::Point, to: usvg::tiny_skia_path::Point, height: f64) {
    let from = vertices.last().copied().unwrap_or([0.0, 0.0]);
    let ctrl_p = transformed_svg_point(transform, ctrl, height);
    let to_p = transformed_svg_point(transform, to, height);
    const STEPS: usize = 12;
    for step in 1..=STEPS {
        let t = step as f64 / STEPS as f64;
        let mt = 1.0 - t;
        vertices.push([mt * mt * from[0] + 2.0 * mt * t * ctrl_p[0] + t * t * to_p[0], mt * mt * from[1] + 2.0 * mt * t * ctrl_p[1] + t * t * to_p[1]]);
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn flatten_cubic_into(vertices: &mut Vec<[f64; 2]>, transform: usvg::Transform, c1: usvg::tiny_skia_path::Point, c2: usvg::tiny_skia_path::Point, to: usvg::tiny_skia_path::Point, height: f64) {
    let from = vertices.last().copied().unwrap_or([0.0, 0.0]);
    let c1p = transformed_svg_point(transform, c1, height);
    let c2p = transformed_svg_point(transform, c2, height);
    let to_p = transformed_svg_point(transform, to, height);
    const STEPS: usize = 16;
    for step in 1..=STEPS {
        let t = step as f64 / STEPS as f64;
        let mt = 1.0 - t;
        vertices.push([mt * mt * mt * from[0] + 3.0 * mt * mt * t * c1p[0] + 3.0 * mt * t * t * c2p[0] + t * t * t * to_p[0], mt * mt * mt * from[1] + 3.0 * mt * mt * t * c1p[1] + 3.0 * mt * t * t * c2p[1] + t * t * t * to_p[1]]);
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn flush_svg_polyline(polylines: &mut Vec<SvgPolyline>, layer: &str, vertices: &mut Vec<[f64; 2]>, closed: &mut bool) {
    if vertices.len() > 1 {
        polylines.push(SvgPolyline { layer: layer.into(), vertices: std::mem::take(vertices), closed: *closed });
    } else {
        vertices.clear();
    }
    *closed = false;
}

/// 🧷️ Signature every 2D-resource-kind SVG document renderer must match to register via {@link register_2d_export_handlers}.
pub type Svg2dDocumentRenderer = fn(&Value) -> Result<(String, u32, u32), String>;

/// 💾️ Registers SVG and PNG export handlers for one 2D resource kind.
pub fn register_2d_export_handlers(artifact_kind: &'static str, file_stem: &'static str, document_to_svg: Svg2dDocumentRenderer) {
    register_os_media_export_handler_kind(artifact_kind, "svg", move |doc| {
        let (svg, _width, _height) = document_to_svg(doc)?;
        Ok(OsMediaExportResult { data: svg, mime_type: "image/svg+xml".into(), file_name: format!("{file_stem}.svg"), encoding: None })
    });
    register_os_media_export_handler_kind(artifact_kind, "png", move |doc| {
        let (svg, width, height) = document_to_svg(doc)?;
        let data = rasterize_svg_to_png_base64(&svg, width, height)?;
        Ok(OsMediaExportResult { data, mime_type: "image/png".into(), file_name: format!("{file_stem}.png"), encoding: Some("base64".into()) })
    });
}

/// 🧵️ Registers a mesh exporter supplied by an artifact.
pub fn register_mesh_exporter(artifact_kind: &'static str, file_stem: &'static str, mesh_from_document: fn(&Value) -> Result<semio_framework::MeshData, String>, exporter: Box<dyn semio_framework::mesh_io::MeshExporter>) {
    let format_kind = exporter.format_kind();
    register_os_media_export_handler_kind(artifact_kind, format_kind, move |doc| {
        let descriptor = semio_framework_os_kernel::io::format_descriptor(format_kind).map_err(|error| error.to_string())?.ok_or_else(|| format!("unknown mesh export format kind `{format_kind}`"))?;
        let extension = descriptor.extensions.first().ok_or_else(|| format!("mesh export format kind `{format_kind}` has no extension claim"))?;
        let mime_type = descriptor.mimes.first().cloned().ok_or_else(|| format!("mesh export format kind `{format_kind}` has no MIME claim"))?;
        let mesh = mesh_from_document(doc)?;
        let bytes = exporter.export(&mesh)?;
        let data = if descriptor.is_binary { base64_codec::base64_standard_encode(&bytes) } else { String::from_utf8(bytes).map_err(|error| error.to_string())? };
        Ok(OsMediaExportResult { data, mime_type, file_name: format!("{file_stem}{extension}"), encoding: descriptor.is_binary.then(|| "base64".into()) })
    });
}

/// 🧵️ Registers one `MeshImporter` format (Obj/Glb/Stl/…) for a mesh resource kind; `document_from_mesh` bridges the decoded `MeshData` back into the app's own document shape.
pub fn register_mesh_importer(artifact_kind: &'static str, document_from_mesh: fn(&semio_framework::MeshData) -> Result<Value, String>, importer: Box<dyn semio_framework::mesh_io::MeshImporter>) {
    let format_kind = importer.format_kind();
    register_os_media_import_handler_kind(artifact_kind, format_kind, move |bytes| {
        let mesh = importer.import(bytes)?;
        document_from_mesh(&mesh)
    });
}

#[cfg(all(test, target_arch = "wasm32", target_env = "p2"))]
include!("../🧪️tests/🔬️media-export-raster-wasip2/🦀️.rs");

#[cfg(all(test, not(target_arch = "wasm32")))]
include!("../🧪️tests/🔬️media-export-raster-unit/🦀️.rs");

// 🚪️ `//#region SolidMediaExport` DELETED WHOLESALE (ticket
// 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave IO1): this used to
// hold `SolidExporterRegistry`/`SolidImporterRegistry`/`solid_exporters`/`solid_importers`/
// `solid_registry_key`/`register_solid_exporter`/`register_solid_importer`/`solid_exporter_for`/
// `export_registered_solid`/`import_registered_solid`, all keyed on
// `semio_framework_3d::brep::kernel::{SolidExporter, SolidImporter, Brep, GeometryHandle}`.
// Census (repo-wide, `grep -rn "register_solid_exporter(\|register_solid_importer("`, excluding
// `🎯️target` and this ticket's own scratch files) found exactly one registrant (cad's
// `register_host_io`, three formats each) and ZERO production callers of the read side --
// `export_registered_solid`/`import_registered_solid` were called from nowhere but this file's
// own self-test below (now also deleted) and cad's now-deleted `solid_exporter_for` assertions.
// cad's REAL per-solid export/import path (`export_solids_as`/`import_step_object`/
// `import_obj_object`/`import_stl_object`, `🗿️artifacts/📐️cad/…/🚪️io/🦀️.rs`) already
// called the genuine stdio `ArtifactSerializer`/`ArtifactDeserializer` leaves directly
// (`SemioMeshToObj`, `SemioMeshToStl`, `SemioBrepToStep`, `SemioBrepFromStep`) -- this whole
// region was dead weight shadowing that real path, not a gap needing a new artifact-io leaf.
// Deleting it also removes the host's last three references to
// `semio_framework_3d::brep::kernel::{SolidExporter, SolidImporter, Brep, GeometryHandle}` --
// see the wave's report for the repo-wide census of who ELSE still depends on that module
// (several framework/plugin crates do; wave IO1 does not touch it further).
// #endregion media_export_raster
