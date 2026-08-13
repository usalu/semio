//! 🚪️ IO s.remodel (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from `declaration()` at the artifact root), not per-leaf register().
//!
//! 🧭️ `🔖️Exporters` and `🚪️DerivedIoRegistry` relocated from `⚙️engine/🦀️component.rs`
//! (26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES, #2553): serializer dispatch and the
//! composer entry table are IO, not engine behaviour.

use crate::artifacts::remodel::{ImageAsset, RemodelSnapshot};
use base64::Engine as _;
use semio_framework::{io_dispatch, Dialect, ErasedComposeSource, IoDirection, IoKey, IoPayload, StandardId, SubsetId};
use semio_framework_plugin::{ArtifactSerializer, MeshData};
use semio_s_plugin_stdio::artifacts::{
    las::standards::v1_0::engine as las_engine,
    png::PngSnapshot,
    ply::standards::v1_0::engine as ply_engine,
    semio::standards::v1::{
        subsets::any::schema::geometry::{SemioPoint3, SemioRgba, SemioUv},
        subsets::image::schema::snapshot::SemioImageSnapshot,
        subsets::mesh::{
            io::export::serializers::artifacts::{las::v1_0::any::SemioMeshToLas, ply::v1_0::any::SemioMeshToPly},
            schema::snapshot::{SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTopology},
        },
    },
};
use serde_json::Value;

pub fn import_stdio_kinds() -> &'static [&'static str] { &["stdio.dwg", "stdio.gltf", "stdio.json", "stdio.las", "stdio.obj", "stdio.ply", "stdio.png", "stdio.stl", "stdio.txt"] }
pub fn export_stdio_kinds() -> &'static [&'static str] { &["stdio.dwg", "stdio.gltf", "stdio.json", "stdio.las", "stdio.obj", "stdio.ply", "stdio.png", "stdio.stl", "stdio.txt"] }

//#region 🔖️Exporters
/// 🧬️ Builds a real `semio/mesh` snapshot (one mesh, one primitive) from this engine's flat
/// `MeshData` buffer — the hand-off point onto stdio's real `SemioMeshToPly`/`SemioMeshToLas`
/// serializers (`26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT`, W5a),
/// replacing the two hand-rolled `MeshExporter` codecs this file used to carry. Topology is
/// `Triangles` whenever an explicit triangle index list is present (or the flat position count is
/// itself a multiple of 3, mirroring this engine's own pre-extraction always-triangulate
/// assumption), else `Points` — the only two topologies `SemioMeshToPly` accepts.
pub(crate) fn mesh_data_to_semio_mesh(mesh: &MeshData) -> SemioMeshSnapshot {
    let positions: Vec<SemioPoint3> = mesh.positions.chunks(3).map(|p| SemioPoint3 { x: f64::from(p[0]), y: f64::from(p[1]), z: f64::from(p[2]) }).collect();
    let normals: Vec<SemioPoint3> = mesh.normals.chunks(3).map(|n| SemioPoint3 { x: f64::from(n[0]), y: f64::from(n[1]), z: f64::from(n[2]) }).collect();
    let colors: Vec<SemioRgba> = if mesh.colors.len() == mesh.positions.len() {
        mesh.colors.chunks(3).map(|c| SemioRgba { r: c[0], g: c[1], b: c[2], a: 1.0 }).collect()
    } else {
        Vec::new()
    };
    let uvs: Vec<SemioUv> = mesh.uvs.chunks(2).map(|uv| SemioUv { u: f64::from(uv[0]), v: f64::from(uv[1]) }).collect();
    let topology = if !mesh.indices.is_empty() || positions.len().is_multiple_of(3) { SemioTopology::Triangles } else { SemioTopology::Points };
    let primitive = SemioPrimitive { id: "remodel-mesh-0".into(), topology, positions, normals, uvs, colors, indices: mesh.indices.clone(), material_id: None };
    SemioMeshSnapshot { schema: "stdio.semio.mesh".into(), meshes: vec![SemioMesh { id: "remodel-mesh".into(), primitives: vec![primitive] }], materials: Vec::new(), textures: Vec::new() }
}

/// 🧬️ Inverse of `mesh_data_to_semio_mesh` — reconstructs a flat `MeshData` buffer from the FIRST
/// primitive of the FIRST mesh (the only shape `mesh_data_to_semio_mesh` ever produces). Real,
/// bidirectional, ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`🧩️Composition` region,
/// artifact root): backs `RemodelMesh.mesh: store::ArtifactChild<SemioMeshSnapshot>`'s working-scene
/// cache accessor. `face_ids`/`vertex_ids`/`edge_*`/`paint_texture_base64` are NOT representable in
/// `SemioMeshSnapshot`'s gltf-shaped primitive (positions/normals/uvs/colors/indices only) — honestly
/// absent here (empty/`None`), never fabricated. The working-scene cache stores the REAL, full-fidelity
/// `MeshData` directly (never round-tripped through this conversion) precisely so those buffers are
/// never lost for the live document; this inverse exists for the case the cache has gone cold (see
/// `remodel_mesh_workspace`'s doc comment) and only the canonical-but-partial `SemioMeshSnapshot`
/// content is available.
pub(crate) fn semio_mesh_to_mesh_data(semio: &SemioMeshSnapshot) -> MeshData {
    let Some(primitive) = semio.meshes.first().and_then(|mesh| mesh.primitives.first()) else {
        return MeshData::default();
    };
    let positions: Vec<f32> = primitive.positions.iter().flat_map(|p| [p.x as f32, p.y as f32, p.z as f32]).collect();
    let normals: Vec<f32> = primitive.normals.iter().flat_map(|n| [n.x as f32, n.y as f32, n.z as f32]).collect();
    let colors: Vec<f32> = primitive.colors.iter().flat_map(|c| [c.r, c.g, c.b]).collect();
    let uvs: Vec<f32> = primitive.uvs.iter().flat_map(|uv| [uv.u as f32, uv.v as f32]).collect();
    MeshData { positions, normals, colors, indices: primitive.indices.clone(), uvs, ..MeshData::default() }
}

/// 🌐️ Encodes a mesh as an ASCII Stanford PLY file via stdio's real `SemioMeshToPly` serializer +
/// `ply::engine::encode_ply` — real codec reuse, not a re-implementation (replaces this file's
/// former hand-rolled `PlyExporter`/`mesh_to_ply`).
pub fn mesh_to_ply_bytes(mesh: &MeshData) -> Result<Vec<u8>, String> {
    let semio = mesh_data_to_semio_mesh(mesh);
    let ply = SemioMeshToPly::serialize(&semio).map_err(|error| error.to_string())?;
    ply_engine::encode_ply(&ply)
}

/// 🛰️ Encodes a mesh as a binary LAS point cloud (point data format 0-3, chosen automatically from
/// whether any point carries RGB) via stdio's real `SemioMeshToLas` serializer + `las::engine::encode_las`
/// — real codec reuse, not a re-implementation (replaces this file's former hand-rolled
/// `LasExporter`/`mesh_to_las`; a mesh's face/index connectivity is honestly dropped, matching LAS's
/// own "point cloud, no topology" semantics — see `SemioMeshToLas`'s own doc comment).
pub fn mesh_to_las_bytes(mesh: &MeshData) -> Result<Vec<u8>, String> {
    let semio = mesh_data_to_semio_mesh(mesh);
    let las = SemioMeshToLas::serialize(&semio).map_err(|error| error.to_string())?;
    las_engine::encode_las(&las)
}

/// 🧩️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `results.mesh.mesh` is now a composed
/// `store::ArtifactChild<SemioMeshSnapshot>` handle, not embedded `MeshData` — real content only
/// lives in the working-scene cache (`crate::artifacts::remodel::remodel_mesh_workspace`, populated at
/// mutation-diff-build/fixture-construction time). A `doc: &Value` parsed fresh from JSON (this
/// function's only call shape) has no session context of its own, so this reads through the SAME
/// process-wide `thread_local!` cache every other real call site funnels through — honestly `Err` on a
/// cold cache (documented staleness gap, matches `raster`/`lowpoly`'s precedent), never a fabricated
/// empty mesh.
pub fn remodel_mesh_from_document(doc: &Value) -> Result<MeshData, String> {
    let scene: RemodelSnapshot = serde_json::from_value(doc.clone()).map_err(|error| error.to_string())?;
    crate::artifacts::remodel::remodel_mesh_workspace(&scene.results.mesh.mesh)
        .ok_or_else(|| "remodel_mesh_from_document: composed mesh content not resolvable (cold working-scene cache)".to_string())
}

/// 🖼️ Exports whichever raster/texture asset is available (DSM, else ortho, else the mesh's baked
/// texture) — `scene.assets` now holds composed `s.stdio.semio.image` child handles, so this reads
/// the real bytes back through `crate::artifacts::remodel::remodel_asset` (working-scene cache), then
/// re-encodes through the real png bridge below (never a raw pass-through of possibly-stale bytes).
pub fn remodel_png_export(doc: &Value) -> Result<semio_framework_os::OsMediaExportResult, String> {
    let scene: RemodelSnapshot = serde_json::from_value(doc.clone()).map_err(|error| error.to_string())?;
    let asset_id = scene
        .results
        .geo
        .as_ref()
        .and_then(|geo| geo.dsm_asset_id.clone().or_else(|| geo.ortho_asset_id.clone()).or_else(|| geo.dtm_asset_id.clone()))
        .or_else(|| scene.results.mesh.texture_asset_id.clone())
        .ok_or_else(|| "no raster or texture asset is available to export as PNG".to_string())?;
    let asset = crate::artifacts::remodel::remodel_asset(&scene.assets, &asset_id).ok_or_else(|| "the referenced raster/texture asset is missing (or its working-scene cache is cold)".to_string())?;
    Ok(semio_framework_os::OsMediaExportResult { data: asset.data, mime_type: "image/png".into(), file_name: "remodel-export.png".into(), encoding: Some("base64".into()) })
}
//#endregion 🔖️Exporters

//#region 🔖️SemioBridge
/// 🌉️ Real `s.stdio.semio/v1/image` ↔ `s.stdio.png` bridge, reused verbatim from `🖨️raster`'s own
/// established pattern (`🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`)
/// — dispatches through stdio's real registered PNG codec (`io_dispatch`), never a hand-rolled PNG
/// reader/writer. `remodel`'s own `ImageAsset.data` is already base64 TEXT (unlike raster's raw
/// `Vec<u8>`), so the base64 (de)coding step happens in `semio_image_snapshot_from_image_asset`/
/// `image_asset_from_semio_image_snapshot` below, one layer up from this raw-bytes bridge.
const SEMIO_IMAGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };
const PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

fn semio_io_key(owner: &Dialect, direction: IoDirection, counterpart: &Dialect) -> IoKey {
    IoKey {
        artifact_kind: owner.artifact_kind.into(),
        standard: owner.standard.0.into(),
        subset: owner.subset.0.into(),
        direction,
        format_kind: counterpart.artifact_kind.into(),
        format_standard: counterpart.standard.0.into(),
        format_subset: counterpart.subset.0.into(),
    }
}

/// 📌️ Registers stdio's `semio` v1 (image subset composer) and `png` engine into the process-global
/// `io` registry exactly once, so `io_dispatch` below resolves regardless of host-boot ordering — a
/// bare `cargo test` process never runs the plugin-host boot path that would normally call this
/// (matches raster's `ensure_stdio_semio_and_png_registered`).
fn ensure_stdio_semio_and_png_registered() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        semio_s_plugin_stdio::artifacts::semio::register();
        semio_s_plugin_stdio::artifacts::png::register();
    });
}

pub(crate) fn semio_image_from_png_bytes(raw_png_bytes: &[u8]) -> Result<SemioImageSnapshot, String> {
    ensure_stdio_semio_and_png_registered();
    let png_snapshot = semio_s_plugin_stdio::artifacts::png::io::decode_png(raw_png_bytes)?;
    let payload = IoPayload::Binary(<PngSnapshot as store::ArtifactPack>::encode_pack(&png_snapshot));
    let key = semio_io_key(&SEMIO_IMAGE_DIALECT, IoDirection::Import, &PNG_DIALECT);
    let composed = io_dispatch(&key, &[ErasedComposeSource { dialect: PNG_DIALECT, payload }]).map_err(|error| error.message)?;
    let IoPayload::Binary(bytes) = composed.payload else { return Err("s.stdio.semio image composer returned a non-binary payload".into()) };
    <SemioImageSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| format!("{error:?}"))
}

pub(crate) fn png_bytes_from_semio_image(image: &SemioImageSnapshot) -> Result<Vec<u8>, String> {
    ensure_stdio_semio_and_png_registered();
    let payload = IoPayload::Binary(<SemioImageSnapshot as store::ArtifactPack>::encode_pack(image));
    let key = semio_io_key(&SEMIO_IMAGE_DIALECT, IoDirection::Export, &PNG_DIALECT);
    let composed = io_dispatch(&key, &[ErasedComposeSource { dialect: SEMIO_IMAGE_DIALECT, payload }]).map_err(|error| error.message)?;
    let IoPayload::Binary(bytes) = composed.payload else { return Err("s.stdio.png composer returned a non-binary payload".into()) };
    let png_snapshot = <PngSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| format!("{error:?}"))?;
    semio_s_plugin_stdio::artifacts::png::io::encode_png(&png_snapshot)
}

/// 🧩️ Real bidirectional CHILD-CONTENT converters between `ImageAsset` (mime + base64 text, the
/// mutation-payload/working-scene shape) and the composed `s.stdio.semio/v1/image` child's real
/// content. Only `image/png` round-trips losslessly today (textures/DSM/DTM/ortho exports are always
/// PNG per this file's own `raster_to_png_asset`/mesh-texture doc comments) — `image/jpeg` (video
/// frames, `MediaStream.frames`) is honestly reported as unsupported rather than silently coerced or
/// dropped; wiring stdio's real `jpg` codec through this same bridge is a scoped, concrete follow-up
/// (see this ticket's `remodel-report.md`), not attempted here.
pub fn semio_image_snapshot_from_image_asset(asset: &ImageAsset) -> Result<SemioImageSnapshot, String> {
    if asset.mime != "image/png" {
        return Err(format!("semio_image_snapshot_from_image_asset: unsupported mime {:?} (only image/png round-trips today)", asset.mime));
    }
    let bytes = base64::engine::general_purpose::STANDARD.decode(asset.data.as_bytes()).map_err(|error| error.to_string())?;
    semio_image_from_png_bytes(&bytes)
}

pub fn image_asset_from_semio_image_snapshot(image: &SemioImageSnapshot) -> Result<ImageAsset, String> {
    let (width, height) = (image.width, image.height);
    let bytes = png_bytes_from_semio_image(image)?;
    Ok(ImageAsset { mime: "image/png".into(), data: base64::engine::general_purpose::STANDARD.encode(bytes), width, height })
}
//#endregion 🔖️SemioBridge

//#region 🧪️ExportersTests
#[cfg(test)]
mod exporters_tests {
    use super::*;
    use crate::artifacts::remodel::default_remodel_scene;
    use semio_framework_plugin::mesh_from_kind;

    #[test]
    fn mesh_to_ply_bytes_writes_a_well_formed_ascii_file_via_stdio() {
        let mesh = mesh_from_kind("box");
        let bytes = mesh_to_ply_bytes(&mesh).expect("ply export");
        let text = String::from_utf8(bytes).expect("ply is ascii");
        assert!(text.starts_with("ply\nformat ascii 1.0\n"));
        assert!(text.contains(&format!("element vertex {}\n", mesh.vertex_count())));
        assert!(text.contains(&format!("element face {}\n", mesh.triangle_count())));
        assert!(text.contains("end_header\n"));
    }

    #[test]
    fn mesh_to_las_bytes_writes_a_227_byte_header_plus_20_bytes_per_point_via_stdio() {
        // 🧪️ `mesh_from_kind("box")` carries no vertex colors, so `SemioMeshToLas` + stdio's
        // `choose_point_format` land on point-data format 0 (no RGB/GPS) — 20 bytes/point, same
        // shape this file's former hand-rolled LAS 1.2 writer always produced.
        let mesh = mesh_from_kind("box");
        let bytes = mesh_to_las_bytes(&mesh).expect("las export");
        assert_eq!(&bytes[0..4], b"LASF");
        assert_eq!(bytes.len(), 227 + mesh.vertex_count() * 20);
        let header_size = u16::from_le_bytes([bytes[94], bytes[95]]);
        assert_eq!(header_size, 227);
        let point_count = u32::from_le_bytes([bytes[107], bytes[108], bytes[109], bytes[110]]);
        assert_eq!(point_count as usize, mesh.vertex_count());
    }

    /// 🧩️ `semio_mesh_to_mesh_data` is the real inverse of `mesh_data_to_semio_mesh`, backing
    /// `crate::artifacts::remodel::remodel_mesh_workspace`'s documented cold-cache fallback path
    /// (ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`). Round-trips positions/normals/uvs/
    /// colors/indices exactly (the shape `SemioMeshSnapshot`'s one gltf-style primitive can represent);
    /// `face_ids`/`vertex_ids`/`edge_*`/`paint_texture_base64` are honestly NOT recovered (empty/`None`
    /// — documented on the function itself) since that shape has no slot for them.
    #[test]
    fn semio_mesh_to_mesh_data_recovers_the_representable_buffers() {
        let mesh = mesh_from_kind("box");
        let semio = mesh_data_to_semio_mesh(&mesh);
        let recovered = semio_mesh_to_mesh_data(&semio);
        assert_eq!(recovered.positions, mesh.positions);
        assert_eq!(recovered.normals, mesh.normals);
        assert_eq!(recovered.uvs, mesh.uvs);
        assert_eq!(recovered.indices, mesh.indices);
        assert_eq!(recovered.colors, mesh.colors, "mesh_from_kind(\"box\") carries no vertex colors, so both sides are empty");
        assert!(recovered.face_ids.is_empty(), "face_ids has no SemioMeshSnapshot slot, honestly absent");
        assert!(recovered.paint_texture_base64.is_none(), "paint_texture_base64 has no SemioMeshSnapshot slot, honestly absent");

        let empty = semio_mesh_to_mesh_data(&SemioMeshSnapshot::default());
        assert!(empty.positions.is_empty(), "no primitive at all yields a default MeshData, not a panic");
    }

    #[test]
    fn png_export_round_trips_a_stored_texture_asset() {
        // 🧪️ `remodel_png_export` now reads real composed `s.stdio.semio/v1/image` content back
        // through `remodel_asset`'s working-scene cache (ticket
        // `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`), so this test seeds a REAL 4x4 RGBA8
        // `SemioImageSnapshot`, round-trips it through the real png bridge to build the `ImageAsset`
        // (exactly what a real texture-export call site does), and verifies the export decodes back
        // to the SAME dimensions/pixels — never a stand-in non-PNG payload.
        use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame};
        let mut scene = default_remodel_scene();
        let pixels: Vec<u8> = (0..4 * 4 * 4).map(|i| (i % 256) as u8).collect();
        let image = SemioImageSnapshot { width: 4, height: 4, colorspace: SemioColorspace::Rgba, bit_depth: 8, frames: vec![SemioImageFrame { delay_ms: 0, rgba8: pixels.clone() }], ..SemioImageSnapshot::default() };
        let asset = image_asset_from_semio_image_snapshot(&image).expect("real png bridge encode");
        let handle = crate::artifacts::remodel::mint_and_stash_asset("tex-1", &asset);
        scene.assets.insert("tex-1".into(), handle);
        scene.results.mesh.texture_asset_id = Some("tex-1".into());
        let doc = serde_json::to_value(&scene).expect("serialize scene");
        let result = remodel_png_export(&doc).expect("png export");
        assert_eq!(result.mime_type, "image/png");
        assert_eq!(result.encoding.as_deref(), Some("base64".into()));
        let exported_bytes = base64::engine::general_purpose::STANDARD.decode(result.data.as_bytes()).expect("valid base64");
        let redecoded = semio_image_from_png_bytes(&exported_bytes).expect("exported bytes are real PNG");
        assert_eq!(redecoded.width, 4);
        assert_eq!(redecoded.height, 4);
        assert_eq!(redecoded.frames.first().map(|frame| frame.rgba8.clone()), Some(pixels));
    }
}
//#endregion 🧪️ExportersTests
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use semio_framework_plugin::{ArtifactComposition, ArtifactBuilder, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource};
    use crate::artifacts::remodel::RemodelSnapshot;
    use crate::artifacts::remodel::standards::v1::subsets::any::schema::RemodelAnalyzer;
    use semio_framework_plugin::ArtifactAnalyzer as _;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.remodel", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_DWG: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    const DEP_GLTF: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_LAS: Dialect = Dialect { artifact_kind: "s.stdio.las", standard: StandardId("1.0"), subset: SubsetId("*") };
    const DEP_OBJ: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
    const DEP_PLY: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId("*") };
    const DEP_PNG: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    const DEP_STL: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };


    pub struct RemodelComposerComposition;

    impl ArtifactComposition for RemodelComposerComposition {
        type Snapshot = RemodelSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_DWG, DEP_GLTF, DEP_JSON, DEP_LAS, DEP_OBJ, DEP_PLY, DEP_PNG, DEP_STL, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
                    };
                    let analysis = RemodelAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
                if source.dialect == DEP_DWG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::remodel::io::import::deserializers::artifacts::dwg::v_ac1018::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_GLTF {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::remodel::io::import::deserializers::artifacts::gltf::v2_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_JSON {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::remodel::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_LAS {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::remodel::io::import::deserializers::artifacts::las::v1_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_OBJ {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::remodel::io::import::deserializers::artifacts::obj::v3_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_PLY {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::remodel::io::import::deserializers::artifacts::ply::v1_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_PNG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::remodel::io::import::deserializers::artifacts::png::v1_2::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_STL {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::remodel::io::import::deserializers::artifacts::stl::v_ascii::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_TXT {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::remodel::io::import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }

            }
            Err(ComposeError { message: "RemodelComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ArtifactBuilder, ComposerEntry, ComposedArtifact, ComposeError, Dialect, StandardId, SubsetId, ErasedComposeSource, IoPayload, IoConfidence, composer_entry_of};
    use crate::artifacts::remodel::standards::v1::subsets::any::schema::RemodelComposer as RemodelAnyComposer;
    use crate::artifacts::remodel::standards::v1::subsets::any::schema::RemodelBuilder as RemodelAnyBuilder;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    //#region 🔖️ExportEntries
    /// 🗄️ Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO W15: the typed registry (W11-W14) only ever grew
    /// IMPORT-direction entries (each composer's own `reads()`) -- nothing registers the REVERSE
    /// ("this domain artifact can be exported AS format Y"), because `ArtifactComposer` only models
    /// "produce my own snapshot." These entries wrap the artifact's EXISTING `🚪️io/📤️export/🧵️serializers`
    /// leaves (which already convert this artifact's snapshot straight to target-format bytes/text) as
    /// their own `ComposerEntry` rows: `writes` = the target format's dialect, `reads` = just this
    /// artifact's own dialect. `register_composer_entries` already inserts BOTH an Import key (target
    /// reads from us) and an Export key (we export to target) per entry, so no framework change was
    /// needed, only populating the missing direction. Generated by generators/w15_add_export_entries.py
    /// -- hand-validated pattern on note/json first (see that file's own tests), pilot kept as reference.
    const REMODEL_DIALECT: Dialect = Dialect { artifact_kind: "s.remodel", standard: StandardId("1"), subset: SubsetId("*") };
    const REMODEL_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::remodel::RemodelSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == REMODEL_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => RemodelAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => RemodelAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "RemodelComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == REMODEL_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::artifacts::remodel::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "RemodelComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_LAS_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.las", standard: StandardId("1.0"), subset: SubsetId("*") };
    fn compose_export_las(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::remodel::io::export::serializers::artifacts::las::v1_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_LAS_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_PLY_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId("*") };
    fn compose_export_ply(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::remodel::io::export::serializers::artifacts::ply::v1_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PLY_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    fn compose_export_png(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::remodel::io::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PNG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::remodel::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    fn compose_export_dwg(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::remodel::io::export::serializers::artifacts::dwg::v_ac1018::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_DWG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_STL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };
    fn compose_export_stl(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::remodel::io::export::serializers::artifacts::stl::v_ascii::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_STL_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_GLTF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
    fn compose_export_gltf(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::remodel::io::export::serializers::artifacts::gltf::v2_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_GLTF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_OBJ_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
    fn compose_export_obj(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::remodel::io::export::serializers::artifacts::obj::v3_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_OBJ_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    //#endregion 🔖️ExportEntries


    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<RemodelAnyComposer>(),
            ComposerEntry { writes: EXPORT_LAS_DIALECT, reads: &[REMODEL_DIALECT], compose: compose_export_las },
            ComposerEntry { writes: EXPORT_PLY_DIALECT, reads: &[REMODEL_DIALECT], compose: compose_export_ply },
            ComposerEntry { writes: EXPORT_PNG_DIALECT, reads: &[REMODEL_DIALECT], compose: compose_export_png },
            ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[REMODEL_DIALECT], compose: compose_export_json },
            ComposerEntry { writes: EXPORT_DWG_DIALECT, reads: &[REMODEL_DIALECT], compose: compose_export_dwg },
            ComposerEntry { writes: EXPORT_STL_DIALECT, reads: &[REMODEL_DIALECT], compose: compose_export_stl },
            ComposerEntry { writes: EXPORT_GLTF_DIALECT, reads: &[REMODEL_DIALECT], compose: compose_export_gltf },
            ComposerEntry { writes: EXPORT_OBJ_DIALECT, reads: &[REMODEL_DIALECT], compose: compose_export_obj },
        ]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
