//! 🧪️ Every registered hop, exercised end to end. The four `🧫️fixtures/` files are authored by
//! `🐍️w6-io-fixtures.py` (Python stdlib `struct`/`zlib` and hand-written text only, never by this
//! repo's own encoders), so reading one back is a genuine cross-implementation check rather than a
//! self-round-trip. The two formats without such a fixture (`las`, `gltf`) are covered by an
//! encode→decode pass through stdio's real codecs — see `📓️w6-io.md` for the third-party oracle
//! rows (`las` 0.11, `ply-rs`, `tobj`, `stl_io`) that belong in this subset's `🔮️oracles/🔣️.json`.

use super::*;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
use semio_framework::io::io_mechanism::{Deserializer, Serializer};
use semio_framework::io_schema::IoPayload;
use semio_framework_plugin::mesh_from_kind;

const PLY_FIXTURE: &[u8] = include_bytes!("../../🧫️fixtures/🧱️four-points.ply");
const OBJ_FIXTURE: &str = include_str!("../../🧫️fixtures/🗿️unit-cube.obj");
const STL_FIXTURE: &str = include_str!("../../🧫️fixtures/🔺️unit-tetra.stl");
const PNG_FIXTURE: &[u8] = include_bytes!("../../🧫️fixtures/📷️two-by-two.png");

/// 🔺️ Triangle count under EITHER `SemioPrimitive` convention: stdio's obj/stl/gltf import leaves
/// flatten face corners into a non-indexed soup ("empty `indices` means sequential"), while its ply
/// leaf keeps the source's shared index list. Both are legal inputs here.
fn triangle_count(mesh: &MeshData) -> usize {
    if mesh.indices.is_empty() {
        mesh.positions.len() / 9
    } else {
        mesh.indices.len() / 3
    }
}

/// ☁️ A cloud-only scene: `results.mesh` is reset to the EMPTY handle so `scene_mesh_or_cloud_semio`
/// falls through to the cloud (`default_remodeling_scene` ships a placeholder box that would
/// otherwise win the mesh-first precedence).
fn scene_with_a_sparse_cloud() -> RemodelingSnapshot {
    let mut scene = default_remodeling_scene();
    scene.results.mesh = RemodelingMesh::default();
    scene.results.sparse = Some(SparseCloud { points: PackedF32::from_f32_slice(&[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]), colors: Some(PackedU8::from_u8_slice(&[255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 255])) });
    scene
}

#[semio_framework_async_macros::async_test]
async fn ply_fixture_seeds_four_sparse_points_with_colors() {
    let outcome = import::ply::v1_0::any::PlyIntoRemodeling::deserialize(&IoPayload::Binary(PLY_FIXTURE.to_vec())).await.expect("ply fixture imports");
    let sparse = outcome.value.results.sparse.expect("the fixture's points land in results.sparse");
    assert_eq!(sparse.points.to_f32_vec(), vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
    assert_eq!(sparse.colors.expect("the fixture carries red/green/blue").to_u8_vec(), vec![255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 255]);
    assert_eq!(outcome.value.results.mesh.source, MeshSource::Placeholder, "a face-less ply must not fabricate a mesh");
}

#[semio_framework_async_macros::async_test]
async fn ply_export_then_import_recovers_the_sparse_cloud() {
    let scene = scene_with_a_sparse_cloud();
    let exported = export::ply::v1_0::any::RemodelingIntoPly::serialize(&scene).await.expect("ply export");
    let IoPayload::Binary(bytes) = exported.value else { panic!("ply is a binary payload") };
    assert!(bytes.starts_with(b"ply"), "the export must be a real ply file");
    let back = import::ply::v1_0::any::PlyIntoRemodeling::deserialize(&IoPayload::Binary(bytes)).await.expect("ply re-import");
    assert_eq!(back.value.results.sparse.expect("sparse survives").points.to_f32_vec(), scene.results.sparse.expect("sparse").points.to_f32_vec());
}

#[semio_framework_async_macros::async_test]
async fn las_export_then_import_recovers_the_sparse_positions() {
    let scene = scene_with_a_sparse_cloud();
    let exported = export::las::v1_0::any::RemodelingIntoLas::serialize(&scene).await.expect("las export");
    let IoPayload::Binary(bytes) = exported.value else { panic!("las is a binary payload") };
    assert_eq!(&bytes[0..4], b"LASF");
    let back = import::las::v1_0::any::LasIntoRemodeling::deserialize(&IoPayload::Binary(bytes)).await.expect("las re-import");
    let recovered = back.value.results.sparse.expect("sparse survives").points.to_f32_vec();
    assert_eq!(recovered.len(), 12);
    for (left, right) in recovered.iter().zip(scene.results.sparse.expect("sparse").points.to_f32_vec()) {
        assert!((left - right).abs() < 1e-3, "{left} vs {right} within LAS scale quantization");
    }
}

#[semio_framework_async_macros::async_test]
async fn las_export_refuses_a_scene_with_no_cloud() {
    let error = export::las::v1_0::any::RemodelingIntoLas::serialize(&default_remodeling_scene()).await.expect_err("a cloud-less scene has nothing to write as las");
    assert!(error.message.contains("results.sparse and results.dense are both absent or empty"), "{}", error.message);
}

#[semio_framework_async_macros::async_test]
async fn obj_fixture_seeds_a_durable_mesh_that_replays() {
    let outcome = import::obj::v3_0::any::ObjIntoRemodeling::deserialize(&IoPayload::Text(OBJ_FIXTURE.to_string())).await.expect("obj fixture imports");
    let scene = outcome.value;
    assert_eq!(scene.results.mesh.source, MeshSource::Imported);
    let mesh = scene_mesh_data(&scene).expect("the seeded durable mesh replays");
    assert_eq!(triangle_count(&mesh), 12, "the cube's 12 triangles survive");
}

#[semio_framework_async_macros::async_test]
async fn obj_export_writes_the_scene_mesh_as_real_obj_text() {
    let scene = default_remodeling_scene();
    let exported = export::obj::v3_0::any::RemodelingIntoObj::serialize(&scene).await.expect("obj export of the placeholder box");
    let IoPayload::Text(text) = exported.value else { panic!("obj is a text payload") };
    assert_eq!(text.lines().filter(|line| line.starts_with("v ")).count(), mesh_from_kind("box").vertex_count());
    assert!(text.lines().any(|line| line.starts_with("f ")), "{text}");
}

#[semio_framework_async_macros::async_test]
async fn stl_fixture_seeds_a_four_facet_triangle_soup() {
    let outcome = import::stl::v_ascii::any::StlIntoRemodeling::deserialize(&IoPayload::Text(STL_FIXTURE.to_string())).await.expect("stl fixture imports");
    let mesh = scene_mesh_data(&outcome.value).expect("the seeded durable mesh replays");
    assert_eq!(triangle_count(&mesh), 4, "the fixture's 4 facets survive");
}

#[semio_framework_async_macros::async_test]
async fn stl_export_writes_an_ascii_solid() {
    let exported = export::stl::v_ascii::any::RemodelingIntoStl::serialize(&default_remodeling_scene()).await.expect("stl export");
    let IoPayload::Text(text) = exported.value else { panic!("ascii stl is a text payload") };
    assert!(text.trim_start().starts_with("solid"), "{text}");
    assert_eq!(text.matches("facet normal").count(), mesh_from_kind("box").triangle_count());
}

#[semio_framework_async_macros::async_test]
async fn gltf_export_then_import_recovers_the_mesh() {
    let exported = export::gltf::v2_0::any::RemodelingIntoGltf::serialize(&default_remodeling_scene()).await.expect("gltf export");
    let IoPayload::Binary(bytes) = exported.value else { panic!("glb is a binary payload") };
    assert!(bytes.starts_with(b"glTF"), "the export must be a real glb container");
    let back = import::gltf::v2_0::any::GltfIntoRemodeling::deserialize(&IoPayload::Binary(bytes)).await.expect("glb re-import");
    let mesh = scene_mesh_data(&back.value).expect("the seeded durable mesh replays");
    assert_eq!(triangle_count(&mesh), mesh_from_kind("box").triangle_count());
}

#[semio_framework_async_macros::async_test]
async fn png_fixture_becomes_a_single_frame_image_sequence() {
    let outcome = import::png::v1_2::any::PngIntoRemodeling::deserialize(&IoPayload::Binary(PNG_FIXTURE.to_vec())).await.expect("png fixture imports");
    let scene = outcome.value;
    assert_eq!(scene.streams.len(), 1);
    assert_eq!(scene.streams[0].kind, MediaKind::ImageSequence);
    assert_eq!(scene.streams[0].frames.len(), 1);
    let asset_id = scene.streams[0].frames[0].asset_id.clone();
    assert!(scene.assets.contains_key(&asset_id), "the frame points at a real registered asset handle");
    let decoded = semio_image_from_png_bytes(PNG_FIXTURE).expect("the python-authored fixture is a real png");
    assert_eq!((decoded.width, decoded.height), (2, 2));
}

#[semio_framework_async_macros::async_test]
async fn png_export_refuses_a_scene_with_no_raster_or_texture() {
    let error = export::png::v1_2::any::RemodelingIntoPng::serialize(&default_remodeling_scene()).await.expect_err("a raster-less scene has nothing to write as png");
    assert!(error.message.contains("no raster or texture asset is available"), "{}", error.message);
}

#[semio_framework_async_macros::async_test]
async fn json_round_trips_the_scene_exactly() {
    let scene = scene_with_a_sparse_cloud();
    let exported = export::json::v_rfc8259::any::RemodelingIntoJson::serialize(&scene).await.expect("json export");
    let IoPayload::Text(text) = exported.value else { panic!("json is a text payload") };
    let back = import::json::v_rfc8259::any::JsonIntoRemodeling::deserialize(&IoPayload::Text(text)).await.expect("json import");
    assert_eq!(back.value, scene);
}

#[semio_framework_async_macros::async_test]
async fn txt_round_trips_the_scene_exactly_through_this_subsets_own_dsl() {
    let scene = scene_with_a_sparse_cloud();
    let exported = export::txt::v_utf_8::any::RemodelingIntoTxt::serialize(&scene).await.expect("dsl export");
    let IoPayload::Text(text) = exported.value else { panic!("txt is a text payload") };
    let back = import::txt::v_utf_8::any::TxtIntoRemodeling::deserialize(&IoPayload::Text(text)).await.expect("dsl import");
    assert_eq!(back.value, scene);
}

#[semio_framework_async_macros::async_test]
async fn every_declared_hop_is_a_distinct_directed_pair() {
    let entries = io().entries;
    assert_eq!(entries.len(), 16, "8 formats x 2 directions");
    let coordinate = |dialect: Dialect| format!("{}@{}/{}", dialect.artifact_kind, dialect.standard.0, dialect.subset.0);
    let mut pairs: Vec<(String, String)> = entries.iter().map(|entry| (coordinate(entry.from), coordinate(entry.into))).collect();
    pairs.sort();
    let before = pairs.len();
    pairs.dedup();
    assert_eq!(pairs.len(), before, "preflight_io_entries rejects a duplicate (from, into) at a different fidelity");
}

#[semio_framework_async_macros::async_test]
async fn mesh_to_ply_bytes_writes_a_well_formed_ascii_file_via_stdio() {
    let mesh = mesh_from_kind("box");
    let bytes = mesh_to_ply_bytes(&mesh).expect("ply export");
    let text = String::from_utf8(bytes).expect("ply is ascii");
    assert!(text.starts_with("ply\nformat ascii 1.0\n"));
    assert!(text.contains(&format!("element vertex {}\n", mesh.vertex_count())));
    assert!(text.contains(&format!("element face {}\n", mesh.triangle_count())));
    assert!(text.contains("end_header\n"));
}

#[semio_framework_async_macros::async_test]
async fn mesh_to_las_bytes_writes_a_227_byte_header_plus_20_bytes_per_point_via_stdio() {
    let mesh = mesh_from_kind("box");
    let bytes = mesh_to_las_bytes(&mesh).expect("las export");
    assert_eq!(&bytes[0..4], b"LASF");
    assert_eq!(bytes.len(), 227 + mesh.vertex_count() * 20);
    assert_eq!(u16::from_le_bytes([bytes[94], bytes[95]]), 227);
    assert_eq!(u32::from_le_bytes([bytes[107], bytes[108], bytes[109], bytes[110]]) as usize, mesh.vertex_count());
}

#[semio_framework_async_macros::async_test]
async fn semio_mesh_to_mesh_data_recovers_the_representable_buffers() {
    let mesh = mesh_from_kind("box");
    let semio = mesh_data_to_semio_mesh(&mesh);
    let recovered = semio_mesh_to_mesh_data(&semio);
    assert_eq!(recovered.positions, mesh.positions);
    assert_eq!(recovered.normals, mesh.normals);
    assert_eq!(recovered.uvs, mesh.uvs);
    assert_eq!(recovered.indices, mesh.indices);
    assert!(recovered.face_ids.is_empty(), "face_ids has no SemioMeshSnapshot slot, honestly absent");
    assert!(semio_mesh_to_mesh_data(&SemioMeshSnapshot::default()).positions.is_empty(), "no primitive at all yields a default MeshData, not a panic");
}

#[semio_framework_async_macros::async_test]
async fn png_export_round_trips_a_stored_texture_asset() {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame};
    let mut scene = default_remodeling_scene();
    let pixels: Vec<u8> = (0..4 * 4 * 4).map(|i| (i % 256) as u8).collect();
    let image = SemioImageSnapshot { width: 4, height: 4, colorspace: SemioColorspace::Rgba, bit_depth: 8, frames: vec![SemioImageFrame { delay_ms: 0, rgba8: pixels.clone() }], ..SemioImageSnapshot::default() };
    let asset = image_asset_from_semio_image_snapshot(&image).expect("real png bridge encode");
    scene.assets.insert("tex-1".into(), crate::store_remodeling_asset("tex-1", &asset));
    scene.results.mesh.texture_asset_id = Some("tex-1".into());
    let result = remodeling_png_export(&scene).expect("png export");
    assert_eq!(result.mime_type, "image/png");
    assert_eq!(result.encoding.as_deref(), Some("base64"));
    let redecoded = semio_image_from_png_bytes(&base64_codec::base64_standard_decode(result.data.as_bytes()).expect("valid base64")).expect("exported bytes are real PNG");
    assert_eq!((redecoded.width, redecoded.height), (4, 4));
    assert_eq!(redecoded.frames.first().map(|frame| frame.rgba8.clone()), Some(pixels));
}
