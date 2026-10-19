//! 🧪️ Document: 2 tests covering value round trip matches serde shape; triangle document boundary matches shared vectors.
use super::*;

/// 🧬️ Additive `#[derive(ToValue, FromValue)]` round-trip (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
/// 26/09/01): `FromValue(ToValue(x)) == x` for the transparent newtypes, the `default`-field
/// structs, and the `u32_hashset_bridge`-covered `HalfedgeMesh` (a real, non-empty mesh, so
/// `vertices`/`halfedges`/`faces`/`uv_seams` are all actually exercised).
#[test]
fn value_round_trip_matches_serde_shape() {
    fn check<T: semio_framework_value::ToValue + semio_framework_value::FromValue + std::fmt::Debug + PartialEq>(value: T) {
        let round_tripped = <T as semio_framework_value::FromValue>::from_value(semio_framework_value::ToValue::to_value(&value)).expect("round-trip decode");
        assert_eq!(round_tripped, value);
    }

    check(Vec3::new(1.0, 2.0, 3.0));
    check(VertexId(7));
    check(WeldMode::ByDistance);
    let corpus:serde_json::Value=crate::fixture_json("../../🌱️value/⚠️refusal/🔁️codec/🧪️tests/🧫️fixtures/🔣️.json");
    let wire=&corpus["valid"][0]["wire"];
    let error=MeshKernelError::Retained(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::from_wire(wire["kind"].as_str().unwrap()).unwrap(),wire["message"].as_str().unwrap()).with_retained_progress(serde_json::from_value(wire["retainedProgress"].clone()).unwrap()));
    let MeshKernelError::Retained(cause)=error else{unreachable!()};let mut permit=|_|true;
    let encoded=cause.to_value_controlled(&mut semio_framework_value::NativeEncodeControl::new(1_000_000,&mut permit)).unwrap();
    assert_eq!(serde_json::Value::from(encoded.clone()),*wire);
    let mut decode_permit=|_|true;
    assert_eq!(semio_framework_value::ValueError::from_value_controlled(&encoded,&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(1_000_000,&mut decode_permit)).unwrap(),cause);

    let mesh = HalfedgeMesh::from_faces(&[[-0.5,-0.5,-0.5],[0.5,-0.5,-0.5],[0.5,0.5,-0.5],[-0.5,0.5,-0.5],[-0.5,-0.5,0.5],[0.5,-0.5,0.5],[0.5,0.5,0.5],[-0.5,0.5,0.5]],&[vec![0,3,2,1],vec![4,5,6,7],vec![0,1,5,4],vec![3,7,6,2],vec![0,4,7,3],vec![1,2,6,5]]).expect("cube document");
    let encoded = semio_framework_value::ToValue::to_value(&mesh);
    let decoded = <HalfedgeMesh as semio_framework_value::FromValue>::from_value(encoded.clone()).expect("round-trip decode");
    assert_eq!(semio_framework_value::ToValue::to_value(&decoded), encoded);

    let via_serde: serde_json::Value = serde_json::to_value(&mesh).expect("serde encode");
    assert_eq!(serde_json::Value::from(encoded), via_serde);
}

#[test]
fn triangle_document_boundary_matches_shared_vectors() {
    let cases:serde_json::Value=crate::fixture_json("📄️document/🧪️tests/🔣️.json");
    for row in cases.as_array().unwrap() {
        let positions:Vec<Vec<f32>>=serde_json::from_value(row["positions"].clone()).unwrap();
        let indices:Vec<u32>=serde_json::from_value(row["indices"].clone()).unwrap();
        let mesh=HalfedgeMesh::from_indexed_triangles(&positions.into_iter().flatten().collect::<Vec<_>>(),&indices).unwrap();
        assert_eq!(mesh.vertex_count(),row["expected"]["vertices"].as_u64().unwrap()as usize);
        assert_eq!(mesh.face_count(),row["expected"]["faces"].as_u64().unwrap()as usize);
        assert_eq!(mesh.halfedge_count(),row["expected"]["halfedges"].as_u64().unwrap()as usize);
        assert_eq!(mesh.edge_count(),row["expected"]["edges"].as_u64().unwrap()as usize);
        assert_eq!(mesh.face_normal(FaceId(0)).unwrap().0,serde_json::from_value::<[f32;3]>(row["expected"]["normal"].clone()).unwrap());
    }
}
