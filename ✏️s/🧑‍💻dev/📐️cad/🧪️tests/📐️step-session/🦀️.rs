//! 📐️ Portable STEP session authority laws with independent JSON and Parry volume oracles.
use semio_s_spatial_kernel_semio_session::Session;
use parry3d::shape::Shape;
use semio_s_artifact_stdio_step::geometry::session::geometry_session;
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../../🧫️fixtures/📐️step-session/🔣️.json")).unwrap() }
fn invoke(session:&Session,method:&str,args:serde_json::Value) -> serde_json::Value { serde_json::from_str(&session.brep_invoke_json(method,&args.to_string())).unwrap() }
fn shape(session:&Session) -> String { invoke(session,"box",fixture()["box"].clone())["handle"].as_str().unwrap().into() }
#[test]
fn neutral_session_does_not_publish_step_operations() {
    let session=Session::new();
    for method in ["exportStep","importStep"] { assert!(invoke(&session,method,serde_json::json!({}))["error"].as_str().unwrap().contains("unknown")); }
    session.close();
}
#[test]
fn step_round_trip_preserves_prior_handles_and_import_claims() {
    let session=geometry_session();
    let prior=shape(&session);
    let exported=invoke(&session,"exportStep",serde_json::json!({"shapes":[prior]}));
    assert!(exported["value"].as_str().unwrap().contains("MANIFOLD_SOLID_BREP"));
    let imported=invoke(&session,"importStep",serde_json::json!({"data":exported["value"]}));
    let handle=imported["handles"][0].as_str().unwrap();
    for handle in [prior.as_str(),handle] {
        let value=invoke(&session,"volume",serde_json::json!({"shape":handle}))["value"].as_f64().unwrap();
        assert!((value-fixture()["expectedVolume"].as_f64().unwrap()).abs()<1e-6);
    }
    let other=session.port();
    assert!(other.dispose(handle).unwrap_err().contains("retained-by-other-authority"));
    drop_port(other);
    session.close();
}
fn drop_port(mut port:Box<dyn semio_framework_os_flow::geometry::GeometryPort>) {
    port.begin_close();
    loop { if matches!(port.close_step(1,1_048_576).unwrap(),neural_engine::ValueRetirementStep::Complete) { break; } }
}
#[test]
fn step_operations_obey_retirement_and_closed_authority() {
    let session=geometry_session();
    let handle=shape(&session);
    session.begin_close();
    assert!(invoke(&session,"exportStep",serde_json::json!({"shapes":[handle]}))["error"].as_str().unwrap().contains("session-closed"));
    assert!(matches!(session.close_step(0,1_048_576).unwrap(),neural_engine::ValueRetirementStep::Blocked));
    session.close();
    assert!(session.terminal_is_empty());
}
#[test]
fn step_mesh_volume_agrees_with_independent_parry_oracle() {
    let session=geometry_session();
    let handle=shape(&session);
    let mesh=session.tessellate_geometry(&handle,0.01).unwrap();
    let points=mesh.positions.chunks_exact(3).map(|p|parry3d::math::Point::new(p[0],p[1],p[2])).collect();
    let indices=mesh.indices.chunks_exact(3).map(|i|[i[0],i[1],i[2]]).collect();
    let oracle=parry3d::shape::TriMesh::new(points,indices).mass_properties(1.0).mass();
    assert!((oracle as f64-fixture()["expectedVolume"].as_f64().unwrap()).abs()<1e-5);
    session.close();
}
