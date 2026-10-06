//! 🎒️ Cross-language laws for the brep preview mesh wire — the `pack` record body that replaced the
//! unchunked JSON number-array string across the host↔extension boundary.
//!
//! The golden vector `🧫️fixtures/🧊️mesh/mesh-pack-body-v1.json` pins this encoder to the
//! TypeScript decoder (`decodeMeshPackBody` in `🧰️framework/🛍️products/💻️os/🟦️.ts`, exercised by
//! `🧪️tests/🧊️mesh-pack-decode`): both languages read the SAME bytes, so a drift on either side
//! fails on both.
//!
//! Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.

// #region 🔖️Imports
use semio_framework_os_flow::mesh::*;
// #endregion 🔖️Imports

// #region 🧰️Fixtures
/// 🧊️ The golden mesh both languages encode/decode — one shaded triangle plus one edge segment,
/// covering every array kind the wire carries (f32 blocks, u32 blocks, edge blocks).
fn golden_mesh() -> semio_framework::MeshData {
    semio_framework::MeshData {
        positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
        normals: vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0],
        indices: vec![0, 1, 2],
        face_ids: vec![7],
        edge_positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        edge_ids: vec![11],
        ..Default::default()
    }
}

fn fixture() -> semio_framework_pack_json::Value {
    semio_framework_pack_json::parse(include_str!("../../../../🧫️fixtures/🧊️mesh/mesh-pack-body-v1.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mesh pack fixture parses")
}
// #endregion 🧰️Fixtures

// #region 🎒️WireLaws
/// ⚖️ LAW: the `pack` mesh body round-trips exactly — every array survives bit-for-bit, so the
/// binary wire is lossless against the JSON one it replaces.
#[test]
fn mesh_pack_round_trips_every_array_exactly() {
    let mesh = golden_mesh();
    let body = encode_mesh_pack(&mesh).expect("encode");
    assert_eq!(decode_mesh_pack(&body).expect("decode"), mesh, "the pack mesh body must be lossless");
}

/// ⚖️ LAW: encoding is deterministic and byte-identical to the cross-language golden vector the
/// TypeScript decoder is pinned to.
#[test]
fn mesh_pack_matches_the_shared_cross_language_vector() {
    let fixture = fixture();
    let expected = fixture.get("packBodyBase64").and_then(semio_framework_pack_json::Value::as_str).expect("packBodyBase64");
    let body = encode_mesh_pack(&golden_mesh()).expect("encode");
    assert_eq!(encode_base64(&body), expected, "the mesh pack wire vector drifted from the shared fixture");
    assert_eq!(body.len() as u64, fixture.get("packBodyBytes").and_then(semio_framework_pack_json::Value::as_u64).expect("packBodyBytes"));
    assert_eq!(decode_mesh_pack(&decode_base64(expected).expect("base64")).expect("decode"), golden_mesh());
}

/// ⚖️ LAW: the binary body is materially smaller than the JSON number-array string it replaces —
/// the whole point of the transport change, so it is asserted rather than assumed.
#[test]
fn mesh_pack_is_smaller_than_the_json_it_replaces() {
    let mesh = golden_mesh();
    let json_bytes = semio_framework_pack_json::to_json_string(&mesh).len();
    let pack_bytes = encode_mesh_pack(&mesh).expect("encode").len();
    eprintln!("mesh wire: json={json_bytes} B, pack={pack_bytes} B, base64={} B", encode_base64(&encode_mesh_pack(&mesh).expect("encode")).len());
    assert!(pack_bytes < json_bytes, "pack body {pack_bytes} B must beat JSON {json_bytes} B");
    assert_eq!(json_bytes as u64, fixture().get("jsonBytes").and_then(semio_framework_pack_json::Value::as_u64).expect("jsonBytes"));
}

/// ⚖️ LAW: a mesh body is chunked into intake-sized pieces, and rejoining them reproduces the
/// original base64 exactly — no single continuation may exceed the chunk ceiling.
#[test]
fn chunking_a_mesh_body_is_lossless_and_bounded() {
    let base64 = encode_base64(&encode_mesh_pack(&golden_mesh()).expect("encode"));
    let chunks = chunk_mesh_base64(&base64);
    assert!(!chunks.is_empty(), "even an empty body transfers as exactly one chunk");
    for chunk in &chunks {
        assert!(chunk.len() <= MESH_PACK_CHUNK_BASE64_CHARS, "a chunk of {} exceeds the intake ceiling", chunk.len());
    }
    assert_eq!(chunks.concat(), base64, "rejoined chunks must equal the original body");
}

/// ⚖️ LAW: a body larger than one chunk really is split, and still rejoins losslessly.
#[test]
fn an_oversized_body_is_split_into_several_chunks() {
    let mesh = semio_framework::MeshData { positions: vec![0.5; 64 * 1024], indices: (0..3_u32).collect(), ..Default::default() };
    let base64 = encode_base64(&encode_mesh_pack(&mesh).expect("encode"));
    let chunks = chunk_mesh_base64(&base64);
    assert!(chunks.len() > 1, "a {} char body must span more than one chunk", base64.len());
    assert_eq!(chunks.concat(), base64);
    assert_eq!(decode_mesh_pack(&decode_base64(&chunks.concat()).expect("base64")).expect("decode"), mesh);
}
// #endregion 🎒️WireLaws

#[test]
fn retained_mesh_pack_preserves_indexed_metadata_and_cancels_bounded_encoding() {
    let fixture=semio_framework_pack_json::parse(include_str!("../../../../🧫️fixtures/🧊️mesh/mesh-pack-attributes.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mesh=semio_framework::MeshData::from(&fixture["mesh"]);
    assert_eq!(mesh.attributes["normal"].values.len(),1);
    let mut job=MeshPackEncodingJob::new(mesh.clone(),64000000).unwrap();let mut steps=0;
    let body=loop {steps+=1;if let Some(body)=job.step(1).unwrap() {break body;}assert!(steps<100000);};
    assert!(steps>4096);assert_eq!(decode_mesh_pack(&body).unwrap(),mesh);
    assert_eq!(body,encode_mesh_pack(&mesh).unwrap());
    let independent:serde_json::Value=serde_json::from_str(&fixture["mesh"].to_string()).unwrap();
    let ours=decode_mesh_pack(&body).unwrap();assert_eq!(ours.attributes["labels"].values[0].as_object().unwrap().iter().find(|(name,_)|name=="payload").unwrap().1.as_str().unwrap(),independent["attributes"]["labels"]["values"][0]["payload"].as_str().unwrap());
    let mut cancelled=MeshPackEncodingJob::new(mesh,64000000).unwrap();for _ in 0..100 {assert!(cancelled.step(1).unwrap().is_none());}cancelled.cancel();assert!(cancelled.step(1).is_err());
}

/// 🎯️ The same preview record preserves full analytic labels through bounded packing.
#[test]
fn mesh_pack_preserves_lossless_analytic_component_references() {
    use semio_framework_value::FromValue;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/🧊️3d/📐️brep/⚙️engine/🧫️fixtures/🎯️component-picking/🔣️.json")).unwrap();
    let source = serde_json::json!({"positions":fixture["transfer"]["position"],"normals":fixture["transfer"]["normal"],"indices":fixture["transfer"]["index"],"faceIds":fixture["expected"]["faceIds"],"edgePositions":fixture["transfer"]["edges"],"edgeIds":fixture["expected"]["edgeIds"],"componentReferences":fixture["expected"]["componentReferences"]});
    let source = semio_framework_pack_json::parse(&source.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mesh = semio_framework::MeshData::from_value(semio_framework_pack_json::to_dsl_value(&source)).unwrap();
    let body = encode_mesh_pack(&mesh).unwrap();
    assert_eq!(encode_base64(&body),fixture["meshPack"]["bodyBase64"].as_str().unwrap());
    assert_eq!(body.len() as u64,fixture["meshPack"]["bodyBytes"].as_u64().unwrap());
    assert_eq!(decode_mesh_pack(&body).unwrap(),mesh);
    let mut job=MeshPackEncodingJob::new(mesh.clone(),1_000_000).unwrap();
    let mut turns=0;
    let retained=loop {turns+=1;if let Some(body)=job.step(1).unwrap(){break body;}assert!(turns<1000);};
    assert_eq!(retained,body);
    let encoded=semio_framework_pack_json::Value::from(mesh);
    assert_eq!(encoded["componentReferences"].to_string(),semio_framework_pack_json::parse(&fixture["expected"]["componentReferences"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap().to_string());
    println!("[DEBUG] Analytic component references retained metadata field13 boundedTurns={turns}");
}
