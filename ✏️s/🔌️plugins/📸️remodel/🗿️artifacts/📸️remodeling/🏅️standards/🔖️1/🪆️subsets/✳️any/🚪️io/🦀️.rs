//! 🚪️ IO `s.remodel.remodeling@1/*` — the subset's complete io surface.
//!
//! 🧭️ Ticket `26/09/06/REMODEL-PLUGIN-END-TO-END` (W6): registration moved off the OLD
//! `ArtifactComposition`/`ComposerEntry`/`io_registry` channel onto the typed
//! `Serializer`/`Deserializer` → [`IoDeclaration`] → `SubsetDeclaration.io` channel every healthy
//! plugin already runs (`🗒️note`, `🧱️block`). The eight `compose_export_*` rows the old
//! `io_registry` published were cross-type `ArtifactPack` casts — `RemodelingSnapshot::encode_pack`
//! followed by `PlySnapshot::decode_pack` and friends — which `store::ArtifactPack`'s own
//! same-type round-trip law makes a deterministic `PackError` on every invocation; they are deleted,
//! not shimmed. Every foreign hop below now runs through a REAL codec: `🧿️semio`'s `🔺️mesh` subset
//! owns bidirectional `SemioMeshSnapshot ↔ {ply,las,obj,stl,gltf}` bridges and `🗄️stdio` owns the
//! byte-level `encode_*`/`decode_*` engines, so this subset only ever maps its own scene onto
//! `SemioMeshSnapshot` and back.
//!
//! 🧭️ `🔖️Exporters` and `🚪️DerivedIoRegistry` were relocated here from `⚙️engine/🦀️.rs`
//! (26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES, #2553): serializer dispatch is IO, not
//! engine behaviour.

use crate::{
    default_remodeling_scene, image_asset_child_handle, remodeling_asset, replayable_remodeling_mesh_handle, resolve_bounded_remodeling_mesh, FrameRef, ImageAsset, MediaKind, MediaStream, MeshSource, PackedF32, PackedU8, RemodelingDurableArtifact,
    RemodelingMesh, RemodelingSnapshot, SparseCloud,
};
use semio_framework::{io_dispatch, resolve_ready, Dialect, ErasedComposeSource, IoDirection, IoKey, IoPayload, StandardId, SubsetId};
use semio_framework_plugin::{ArtifactSerializer, MeshData};
use semio_s_artifact_stdio_las::standards::v1_0::engine as las_engine;
use semio_s_artifact_stdio_ply::standards::v1_0::engine as ply_engine;
use semio_s_artifact_stdio_png::PngSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::{
        subsets::base::schema::geometry::{SemioPoint3, SemioRgba, SemioUv},
        subsets::image::schema::snapshot::SemioImageSnapshot,
        subsets::mesh::{
            io::export::serializers::artifacts::{las::v1_0::any::SemioMeshToLas, ply::v1_0::any::SemioMeshToPly},
            schema::snapshot::{SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTopology},
        },
    };

pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.gltf", "stdio.json", "stdio.las", "stdio.obj", "stdio.ply", "stdio.png", "stdio.stl", "stdio.txt"]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.gltf", "stdio.json", "stdio.las", "stdio.obj", "stdio.ply", "stdio.png", "stdio.stl", "stdio.txt"]
}

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
    let colors: Vec<SemioRgba> = if mesh.colors.len() == mesh.positions.len() { mesh.colors.chunks(3).map(|c| SemioRgba { r: c[0], g: c[1], b: c[2], a: 1.0 }).collect() } else { Vec::new() };
    let uvs: Vec<SemioUv> = mesh.uvs.chunks(2).map(|uv| SemioUv { u: f64::from(uv[0]), v: f64::from(uv[1]) }).collect();
    let topology = if !mesh.indices.is_empty() || positions.len().is_multiple_of(3) { SemioTopology::Triangles } else { SemioTopology::Points };
    let primitive = SemioPrimitive { id: "remodeling-mesh-0".into(), topology, positions, normals, uvs, colors, indices: mesh.indices.clone(), material_id: None };
    SemioMeshSnapshot { schema: "stdio.semio.mesh".into(), meshes: vec![SemioMesh { id: "remodeling-mesh".into(), primitives: vec![primitive] }], materials: Vec::new(), textures: Vec::new() }
}

/// 🧬️ Inverse of `mesh_data_to_semio_mesh` — reconstructs a flat `MeshData` buffer from the FIRST
/// primitive of the FIRST mesh (the only shape `mesh_data_to_semio_mesh` ever produces). Real,
/// bidirectional, ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`🧩️Composition` region,
/// artifact root): backs `RemodelingMesh.mesh: store::ArtifactChild<SemioMeshSnapshot>`'s working-scene
/// cache accessor, and (since W6) every `{ply,las,obj,stl,gltf}` IMPORT hop below.
/// `face_ids`/`vertex_ids`/`edge_*`/`paint_texture_base64` are NOT representable in
/// `SemioMeshSnapshot`'s gltf-shaped primitive (positions/normals/uvs/colors/indices only) — honestly
/// absent here (empty/`None`), never fabricated.
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

/// 🌐️ Encodes a mesh as a Stanford PLY file via stdio's real `SemioMeshToPly` serializer +
/// `ply::engine::encode_ply` — real codec reuse, not a re-implementation (replaces this file's
/// former hand-rolled `PlyExporter`/`mesh_to_ply`).
pub fn mesh_to_ply_bytes(mesh: &MeshData) -> Result<Vec<u8>, String> {
    let semio = mesh_data_to_semio_mesh(mesh);
    semio_mesh_to_ply_bytes(&semio)
}

/// 🛰️ Encodes a mesh as a binary LAS point cloud (point data format 0-3, chosen automatically from
/// whether any point carries RGB) via stdio's real `SemioMeshToLas` serializer + `las::engine::encode_las`
/// — real codec reuse, not a re-implementation (a mesh's face/index connectivity is honestly dropped,
/// matching LAS's own "point cloud, no topology" semantics — see `SemioMeshToLas`'s own doc comment).
pub fn mesh_to_las_bytes(mesh: &MeshData) -> Result<Vec<u8>, String> {
    let semio = mesh_data_to_semio_mesh(mesh);
    semio_mesh_to_las_bytes(&semio)
}

/// 🌐️ `SemioMeshSnapshot` → PLY bytes, the shared tail of both `mesh_to_ply_bytes` and the `🧱️ply`
/// export leaf (which feeds a point-cloud snapshot rather than a mesh one).
pub fn semio_mesh_to_ply_bytes(semio: &SemioMeshSnapshot) -> Result<Vec<u8>, String> {
    let ply = resolve_ready(SemioMeshToPly::serialize(semio)).map_err(|error| error.to_string())?;
    ply_engine::encode_ply(&ply)
}

/// 🛰️ `SemioMeshSnapshot` → LAS bytes, the shared tail of `mesh_to_las_bytes` and the `☁️las` leaf.
pub fn semio_mesh_to_las_bytes(semio: &SemioMeshSnapshot) -> Result<Vec<u8>, String> {
    let las = resolve_ready(SemioMeshToLas::serialize(semio)).map_err(|error| error.to_string())?;
    las_engine::encode_las(&las)
}

/// 🖼️ Exports whichever raster/texture asset is available (DSM, else ortho, else DTM, else the mesh's
/// baked texture) — `scene.assets` holds composed `s.stdio.semio.image` child handles, so this reads
/// the real bytes back through `remodeling_asset` (working-scene cache).
pub fn remodeling_png_asset(scene: &RemodelingSnapshot) -> Result<ImageAsset, String> {
    let asset_id = scene
        .results
        .geo
        .as_ref()
        .and_then(|geo| geo.dsm_asset_id.clone().or_else(|| geo.ortho_asset_id.clone()).or_else(|| geo.dtm_asset_id.clone()))
        .or_else(|| scene.results.mesh.texture_asset_id.clone())
        .ok_or_else(|| "no raster or texture asset is available to export as PNG".to_string())?;
    remodeling_asset(scene, &asset_id).ok_or_else(|| "the referenced raster/texture asset is missing".to_string())
}

/// 🖼️ `remodeling_png_asset` in the OS media-export envelope the editor's export command speaks.
/// Takes the typed scene: `RemodelingSnapshot` left the serde type graph with `store::ArtifactChild`
/// (W9), so a `serde_json::Value` document is no longer a carrier this crate can decode.
pub fn remodeling_png_export(scene: &RemodelingSnapshot) -> Result<semio_framework_os::OsMediaExportResult, String> {
    let asset = remodeling_png_asset(scene)?;
    Ok(semio_framework_os::OsMediaExportResult { data: asset.data, mime_type: "image/png".into(), file_name: "remodeling-export.png".into(), encoding: Some("base64".into()) })
}
//#endregion 🔖️Exporters

//#region 🔖️SceneGeometry
/// 🧱️ Vertex/triangle ceiling `resolve_bounded_remodeling_mesh` enforces on replay — an imported
/// mesh larger than this would be admitted into `durable_artifacts` and then never resolve again, so
/// every import hop rejects it up front with a reason instead.
const REMODELING_IMPORT_MESH_VERTICES: usize = 512;
const REMODELING_IMPORT_MESH_TRIANGLES: usize = 512;
/// 🧱️ One durable chunk is at most 4096 raw bytes: a one-byte field tag plus 4092 payload bytes
/// (the largest 4-byte-aligned remainder), matching `apply_mesh_chunk`'s framing exactly.
const REMODELING_MESH_CHUNK_VALUE_BYTES: usize = 4_092;
/// 🧱️ Staging identity every io-seeded durable mesh handle carries, so a replayed import is
/// distinguishable from a reconstruction commit in `durable_artifacts`.
pub const REMODELING_IO_MESH_STAGING_ID: &str = "io-import";

/// 🧊️ The scene's own reconstructed/imported mesh as flat buffers, or a reason it is unavailable.
pub fn scene_mesh_data(scene: &RemodelingSnapshot) -> Result<MeshData, String> {
    resolve_bounded_remodeling_mesh(&scene.durable_artifacts, &scene.results.mesh.mesh).ok_or_else(|| "results.mesh carries no durable content admitted by the bounded 512/512 envelope".to_string())
}

/// 🧊️ The scene's mesh as a `semio/mesh` snapshot — the input every mesh-shaped export hop needs.
pub fn scene_mesh_semio(scene: &RemodelingSnapshot) -> Result<SemioMeshSnapshot, String> {
    let mesh = scene_mesh_data(scene)?;
    if mesh.positions.is_empty() {
        return Err("results.mesh resolves to an empty mesh (no vertex positions)".to_string());
    }
    Ok(mesh_data_to_semio_mesh(&mesh))
}

fn packed_colors_to_unit(colors: &PackedU8) -> Vec<f32> {
    colors.to_u8_vec().iter().map(|component| f32::from(*component) / 255.0).collect()
}

fn point_cloud_semio(id: &str, positions: &[f32], colors: &[f32]) -> SemioMeshSnapshot {
    let points: Vec<SemioPoint3> = positions.as_chunks::<3>().0.iter().map(|p| SemioPoint3 { x: f64::from(p[0]), y: f64::from(p[1]), z: f64::from(p[2]) }).collect();
    let rgba: Vec<SemioRgba> = if colors.len() == points.len() * 3 { colors.as_chunks::<3>().0.iter().map(|c| SemioRgba { r: c[0], g: c[1], b: c[2], a: 1.0 }).collect() } else { Vec::new() };
    let primitive = SemioPrimitive { id: format!("remodeling-{id}-0"), topology: SemioTopology::Points, positions: points, normals: Vec::new(), uvs: Vec::new(), colors: rgba, indices: Vec::new(), material_id: None };
    SemioMeshSnapshot { schema: "stdio.semio.mesh".into(), meshes: vec![SemioMesh { id: format!("remodeling-{id}"), primitives: vec![primitive] }], materials: Vec::new(), textures: Vec::new() }
}

/// ☁️ The scene's point cloud as a `Points`-topology `semio/mesh` snapshot — `results.dense` when a
/// dense run has produced one (strictly more points), else `results.sparse`.
pub fn scene_cloud_semio(scene: &RemodelingSnapshot) -> Result<SemioMeshSnapshot, String> {
    if let Some(dense) = scene.results.dense.as_ref() {
        let positions = dense.positions.to_f32_vec_from(&scene.durable_artifacts);
        if !positions.is_empty() {
            let colors = dense.colors.as_ref().map(packed_colors_to_unit).unwrap_or_default();
            return Ok(point_cloud_semio("dense", &positions, &colors));
        }
    }
    if let Some(sparse) = scene.results.sparse.as_ref() {
        let positions = sparse.points.to_f32_vec_from(&scene.durable_artifacts);
        if !positions.is_empty() {
            let colors = sparse.colors.as_ref().map(packed_colors_to_unit).unwrap_or_default();
            return Ok(point_cloud_semio("sparse", &positions, &colors));
        }
    }
    Err("results.sparse and results.dense are both absent or empty".to_string())
}

/// 🧊️☁️ Mesh first, point cloud second — the precedence PLY (the one format that carries both a
/// surface and a bare point set) uses. Both reasons are reported when neither exists.
pub fn scene_mesh_or_cloud_semio(scene: &RemodelingSnapshot) -> Result<SemioMeshSnapshot, String> {
    match scene_mesh_semio(scene) {
        Ok(semio) => Ok(semio),
        Err(mesh_reason) => scene_cloud_semio(scene).map_err(|cloud_reason| format!("{mesh_reason}; {cloud_reason}")),
    }
}

fn push_f32_chunks(chunks: &mut Vec<String>, field: u8, values: &[f32]) {
    for window in values.chunks(REMODELING_MESH_CHUNK_VALUE_BYTES / 4) {
        let mut framed = Vec::with_capacity(1 + window.len() * 4);
        framed.push(field);
        framed.extend(window.iter().flat_map(|value| value.to_le_bytes()));
        chunks.push(base64_codec::base64_standard_encode(framed));
    }
}

fn push_u32_chunks(chunks: &mut Vec<String>, field: u8, values: &[u32]) {
    for window in values.chunks(REMODELING_MESH_CHUNK_VALUE_BYTES / 4) {
        let mut framed = Vec::with_capacity(1 + window.len() * 4);
        framed.push(field);
        framed.extend(window.iter().flat_map(|value| value.to_le_bytes()));
        chunks.push(base64_codec::base64_standard_encode(framed));
    }
}

/// 🧱️ Frames a `MeshData` into the exact durable-chunk shape `apply_mesh_chunk` replays: one leading
/// field tag per chunk (0 positions, 1 normals, 2 colors, 3 indices, 4 uvs — emitted in strictly
/// non-decreasing tag order, which that replayer requires) and at most 4092 payload bytes each.
fn mesh_durable_chunks(mesh: &MeshData) -> Vec<String> {
    let mut chunks = Vec::new();
    push_f32_chunks(&mut chunks, 0, &mesh.positions);
    push_f32_chunks(&mut chunks, 1, &mesh.normals);
    push_f32_chunks(&mut chunks, 2, &mesh.colors);
    push_u32_chunks(&mut chunks, 3, &mesh.indices);
    push_f32_chunks(&mut chunks, 4, &mesh.uvs);
    chunks
}

fn mesh_content_id(chunks: &[String]) -> String {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for chunk in chunks {
        chunk.hash(&mut hasher);
    }
    format!("remodeling-mesh-io-{:016x}-{:016x}", hasher.finish(), chunks.len())
}

/// 🧱️ Admits an imported mesh into the scene as REAL durable content: content-addressed chunks in
/// `durable_artifacts` plus a replayable `results.mesh.mesh` handle, exactly the shape a committed
/// reconstruction produces (`🏗️run-reconstruction`'s `TerminalPhase::Mesh`). A mesh outside the
/// bounded envelope is rejected with its measured size rather than stored unreadable.
pub fn seed_remodeling_mesh(scene: &mut RemodelingSnapshot, mesh: &MeshData) -> Result<(), String> {
    let vertices = mesh.positions.len() / 3;
    let triangles = mesh.indices.len() / 3;
    if vertices > REMODELING_IMPORT_MESH_VERTICES || triangles > REMODELING_IMPORT_MESH_TRIANGLES {
        return Err(format!("imported mesh is {vertices} vertices / {triangles} triangles, beyond the bounded {REMODELING_IMPORT_MESH_VERTICES}/{REMODELING_IMPORT_MESH_TRIANGLES} envelope this document can replay"));
    }
    let chunks = mesh_durable_chunks(mesh);
    let chunk_count = u64::try_from(chunks.len()).map_err(|error| error.to_string())?;
    let content_id = mesh_content_id(&chunks);
    scene.durable_artifacts.insert(content_id.clone(), RemodelingDurableArtifact { kind: "mesh".into(), mime: None, width: 0, height: 0, chunks });
    scene.results.mesh = RemodelingMesh { mesh: replayable_remodeling_mesh_handle(&content_id, REMODELING_IO_MESH_STAGING_ID, chunk_count), source: MeshSource::Imported, texture_asset_id: None, watertight: None };
    Ok(())
}

/// 🧊️ A decoded foreign mesh as a whole scene: a fresh document whose `results.mesh` is the imported
/// surface. Triangle-less input is rejected here — `scene_from_semio_cloud` is the point-set door.
pub fn scene_from_semio_mesh(semio: &SemioMeshSnapshot) -> Result<RemodelingSnapshot, String> {
    let mesh = semio_mesh_to_mesh_data(semio);
    if mesh.positions.is_empty() {
        return Err("the decoded geometry carries no vertex positions".to_string());
    }
    let mut scene = default_remodeling_scene();
    seed_remodeling_mesh(&mut scene, &mesh)?;
    Ok(scene)
}

/// ☁️ A decoded foreign point set as a whole scene: `results.sparse` carries the points and colors,
/// and `results.mesh` is additionally seeded when the source also carried triangles (a PLY may) —
/// dropping real faces would be silent loss. `results.dense` stays `None`: neither `SemioMeshSnapshot`
/// nor any of the five foreign mesh dialects has a per-point confidence channel, which is the field
/// that distinguishes a dense cloud from a sparse one in this schema.
pub fn scene_from_semio_cloud(semio: &SemioMeshSnapshot) -> Result<RemodelingSnapshot, String> {
    let mesh = semio_mesh_to_mesh_data(semio);
    if mesh.positions.is_empty() {
        return Err("the decoded point set carries no positions".to_string());
    }
    let mut scene = default_remodeling_scene();
    let colors = (!mesh.colors.is_empty()).then(|| PackedU8::from_u8_slice(&mesh.colors.iter().map(|component| (component.clamp(0.0, 1.0) * 255.0).round() as u8).collect::<Vec<u8>>()));
    scene.results.sparse = Some(SparseCloud { points: PackedF32::from_f32_slice(&mesh.positions), colors });
    if !mesh.indices.is_empty() {
        seed_remodeling_mesh(&mut scene, &mesh)?;
    }
    Ok(scene)
}

/// 🖼️ One PNG file as a whole scene: a single-frame `MediaKind::ImageSequence` stream whose frame
/// points at a real durable image asset, the same shape `📥️import-frames` builds for a photo set.
pub fn scene_from_png_bytes(bytes: &[u8]) -> Result<RemodelingSnapshot, String> {
    let png = semio_s_artifact_stdio_png::io::decode_png(bytes)?;
    let asset = ImageAsset { mime: "image/png".into(), data: base64_codec::base64_standard_encode(bytes), width: png.width, height: png.height };
    let asset_id = "png-import-0".to_string();
    let mut scene = default_remodeling_scene();
    scene.assets.insert(asset_id.clone(), image_asset_child_handle(&asset_id, &asset));
    scene.streams.push(MediaStream {
        id: "png-import".into(),
        name: "Imported PNG".into(),
        kind: MediaKind::ImageSequence,
        camera_id: None,
        sync_offset_ms: 0.0,
        fps_hint: 0.0,
        frames: vec![FrameRef { index: 0, timestamp_ms: 0.0, asset_id }],
        source: None,
    });
    Ok(scene)
}
//#endregion 🔖️SceneGeometry

//#region 🔖️SemioBridge
/// 🌉️ Real `s.stdio.semio/v1/image` ↔ `s.stdio.png` bridge, reused verbatim from `🖨️raster`'s own
/// established pattern (`🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs`)
/// — dispatches through stdio's real registered PNG codec (`io_dispatch`), never a hand-rolled PNG
/// reader/writer. `remodeling`'s own `ImageAsset.data` is already base64 TEXT (unlike raster's raw
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
        semio_s_artifact_stdio_semio::register();
        semio_s_artifact_stdio_png::register();
    });
}

pub(crate) fn semio_image_from_png_bytes(raw_png_bytes: &[u8]) -> Result<SemioImageSnapshot, String> {
    ensure_stdio_semio_and_png_registered();
    let png_snapshot = semio_s_artifact_stdio_png::io::decode_png(raw_png_bytes)?;
    let payload = IoPayload::Binary(<PngSnapshot as store::ArtifactPack>::encode_pack(&png_snapshot));
    let key = semio_io_key(&SEMIO_IMAGE_DIALECT, IoDirection::Import, &PNG_DIALECT);
    let composed = resolve_ready(io_dispatch(&key, &[ErasedComposeSource { dialect: PNG_DIALECT, payload }])).map_err(|error| error.message)?;
    let IoPayload::Binary(bytes) = composed.payload else { return Err("s.stdio.semio image composer returned a non-binary payload".into()) };
    <SemioImageSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| format!("{error:?}"))
}

pub(crate) fn png_bytes_from_semio_image(image: &SemioImageSnapshot) -> Result<Vec<u8>, String> {
    ensure_stdio_semio_and_png_registered();
    let payload = IoPayload::Binary(<SemioImageSnapshot as store::ArtifactPack>::encode_pack(image));
    let key = semio_io_key(&SEMIO_IMAGE_DIALECT, IoDirection::Export, &PNG_DIALECT);
    let composed = resolve_ready(io_dispatch(&key, &[ErasedComposeSource { dialect: SEMIO_IMAGE_DIALECT, payload }])).map_err(|error| error.message)?;
    let IoPayload::Binary(bytes) = composed.payload else { return Err("s.stdio.png composer returned a non-binary payload".into()) };
    let png_snapshot = <PngSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| format!("{error:?}"))?;
    semio_s_artifact_stdio_png::io::encode_png(&png_snapshot)
}

/// 🧩️ Real bidirectional CHILD-CONTENT converters between `ImageAsset` (mime + base64 text, the
/// mutation-payload/working-scene shape) and the composed `s.stdio.semio/v1/image` child's real
/// content. Only `image/png` round-trips losslessly today (textures/DSM/DTM/ortho exports are always
/// PNG per this file's own `raster_to_png_asset`/mesh-texture doc comments) — `image/jpeg` (video
/// frames, `MediaStream.frames`) is honestly reported as unsupported rather than silently coerced or
/// dropped.
pub fn semio_image_snapshot_from_image_asset(asset: &ImageAsset) -> Result<SemioImageSnapshot, String> {
    if asset.mime != "image/png" {
        return Err(format!("semio_image_snapshot_from_image_asset: unsupported mime {:?} (only image/png round-trips today)", asset.mime));
    }
    let bytes = base64_codec::base64_standard_decode(asset.data.as_bytes()).map_err(|error| error.to_string())?;
    semio_image_from_png_bytes(&bytes)
}

pub fn image_asset_from_semio_image_snapshot(image: &SemioImageSnapshot) -> Result<ImageAsset, String> {
    let (width, height) = (image.width, image.height);
    let bytes = png_bytes_from_semio_image(image)?;
    Ok(ImageAsset { mime: "image/png".into(), data: base64_codec::base64_standard_encode(bytes), width, height })
}
//#endregion 🔖️SemioBridge

//#region 🎹️DerivedComposition
/// 🎹️ Native-dialect-only composition facet, bound by `derive_artifact_facets!` in this subset's
/// `🧬️schema/🦀️.rs`. Its nine foreign-format branches are GONE (W6): each called a
/// `📥️import` leaf's `deserialize_bytes`, all of which were cross-type `ArtifactPack` casts. Foreign
/// dialects now reach this subset only through the typed `IoEntry` rows `io()` publishes.
pub mod derived_composition {
    use crate::standards::v1::subsets::any::schema::RemodelingAnalyzer;
    use crate::RemodelingSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.remodel.remodeling", standard: StandardId("1"), subset: SubsetId("*") };

    pub struct RemodelingComposerComposition;

    impl ArtifactComposition for RemodelingComposerComposition {
        type Snapshot = RemodelingSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                    };
                    let analysis = RemodelingAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
            }
            Err(ComposeError { message: "RemodelingComposerComposition: no source in this subset's own native dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️NativeComposer
/// 🎹️ The ONE surviving row of the old `ComposerEntry` channel: this subset's own derive-generated
/// native composer, which the OS document-open path still resolves by `IoKey`. The eight
/// `compose_export_*` rows that sat beside it are deleted (see this file's module doc); every foreign
/// hop now lives on the typed `io()` channel below.
pub fn native_composer_entries() -> &'static [semio_framework_plugin::ComposerEntry] {
    use crate::standards::v1::subsets::any::schema::RemodelingComposer;
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::ComposerEntry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| vec![semio_framework_plugin::composer_entry_of::<RemodelingComposer>()]).as_slice()
}
//#endregion 🚪️NativeComposer

//#region 🔖️IoDeclaration
/// 🚪️ This subset's complete io surface on the framework's `io_mechanism` channel — the five native
/// `dsl::LanguageSpec`s plus one `IoEntry` per registered foreign hop, preflighted and registered by
/// `commit_artifact_declarations` (one `io_register` per subset).
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use crate::{RemodelingMutation, RemodelingSnapshot, REMODELING_DIALECT, REMODELING_DOCUMENT_SCHEMA};
    use semio_framework::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
    use std::sync::OnceLock;

    /// 🗣️ The five hand-authored `dsl::LanguageSpec`s this subset carries — `OnceLock` because
    /// `dsl::passthrough_hooks` is not `const fn`. Indices: 0=document 1=op 2=diff 3=pack 4=spr.
    fn languages() -> &'static [dsl::LanguageSpec; 5] {
        static LANGUAGES: OnceLock<[dsl::LanguageSpec; 5]> = OnceLock::new();
        LANGUAGES.get_or_init(|| {
            [
                dsl::LanguageSpec {
                    id: "remodeling.document",
                    extension: Some("remodeling"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("remodeling.document"),
                },
                dsl::LanguageSpec {
                    id: "remodeling.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("remodeling.op"),
                },
                dsl::LanguageSpec {
                    id: "remodeling.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("remodeling.diff"),
                },
                dsl::LanguageSpec {
                    id: "remodeling.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("remodeling.pack"),
                },
                dsl::LanguageSpec {
                    id: "remodeling.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("remodeling.spr"),
                },
            ]
        })
    }

    fn entries() -> &'static [IoEntry] {
        static ENTRIES: OnceLock<Vec<IoEntry>> = OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    serializer_entry::<RemodelingSnapshot, export::json::v_rfc8259::any::RemodelingIntoJson>(REMODELING_DIALECT),
                    deserializer_entry::<RemodelingSnapshot, import::json::v_rfc8259::any::JsonIntoRemodeling>(REMODELING_DIALECT),
                    serializer_entry::<RemodelingSnapshot, export::txt::v_utf_8::any::RemodelingIntoTxt>(REMODELING_DIALECT),
                    deserializer_entry::<RemodelingSnapshot, import::txt::v_utf_8::any::TxtIntoRemodeling>(REMODELING_DIALECT),
                    serializer_entry::<RemodelingSnapshot, export::ply::v1_0::any::RemodelingIntoPly>(REMODELING_DIALECT),
                    deserializer_entry::<RemodelingSnapshot, import::ply::v1_0::any::PlyIntoRemodeling>(REMODELING_DIALECT),
                    serializer_entry::<RemodelingSnapshot, export::las::v1_0::any::RemodelingIntoLas>(REMODELING_DIALECT),
                    deserializer_entry::<RemodelingSnapshot, import::las::v1_0::any::LasIntoRemodeling>(REMODELING_DIALECT),
                    serializer_entry::<RemodelingSnapshot, export::obj::v3_0::any::RemodelingIntoObj>(REMODELING_DIALECT),
                    deserializer_entry::<RemodelingSnapshot, import::obj::v3_0::any::ObjIntoRemodeling>(REMODELING_DIALECT),
                    serializer_entry::<RemodelingSnapshot, export::stl::v_ascii::any::RemodelingIntoStl>(REMODELING_DIALECT),
                    deserializer_entry::<RemodelingSnapshot, import::stl::v_ascii::any::StlIntoRemodeling>(REMODELING_DIALECT),
                    serializer_entry::<RemodelingSnapshot, export::gltf::v2_0::any::RemodelingIntoGltf>(REMODELING_DIALECT),
                    deserializer_entry::<RemodelingSnapshot, import::gltf::v2_0::any::GltfIntoRemodeling>(REMODELING_DIALECT),
                    serializer_entry::<RemodelingSnapshot, export::png::v1_2::any::RemodelingIntoPng>(REMODELING_DIALECT),
                    deserializer_entry::<RemodelingSnapshot, import::png::v1_2::any::PngIntoRemodeling>(REMODELING_DIALECT),
                ]
            })
            .as_slice()
    }

    let langs = languages();
    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: Some(&langs[0]), binary: Some(&langs[3]) },
            diff: LanguagePair { text: Some(&langs[2]), binary: None },
            mutations: LanguagePair { text: Some(&langs[1]), binary: Some(&langs[4]) },
            inferences: None,
            codec: store::ArtifactCodec::of::<RemodelingSnapshot, RemodelingMutation>(REMODELING_DOCUMENT_SCHEMA.to_string()),
        },
        entries: entries(),
    }
}
//#endregion 🔖️IoDeclaration

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️io/🦀️.rs"]
mod io_tests;
//#endregion 🧪️Tests
