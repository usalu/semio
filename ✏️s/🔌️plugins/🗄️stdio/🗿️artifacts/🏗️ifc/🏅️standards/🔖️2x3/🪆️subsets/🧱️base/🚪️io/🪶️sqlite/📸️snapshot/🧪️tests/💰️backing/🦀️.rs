//! 💰️ IFC2x3 actual conformance consumers request only concrete backing.
use semio_framework_value::ValueRefusalKind;
use semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl;
#[test]
fn sqlite_snapshot_ifc2x3_mvd_consumers_follow_concrete_backing_roles(){
    let vector:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/💰️backing/🔣️.json")).unwrap();
    assert_eq!(vector["inlineRoot"],"noBackingRequest");assert_eq!(vector["nativeRoles"]["owned"],"maxAllocationBytes");
    let value=semio_s_artifact_stdio_ifc_2x3_base_model::standards::v2x3::subsets::base::schema::demo_ifc2x3_snapshot();
    for subset in ["cv20","sav","cobie"]{
        let limits=store::sqlite_snapshot::SqliteDatabaseLimits{max_allocation_bytes:0,..Default::default()};
        let mut progress=|_|true;let mut control=SqliteSnapshotControl::new(&mut progress,limits);
        let refused=match subset{
            "cv20"=>semio_s_artifact_stdio_ifc_2x3_cv20::standards::v2x3::subsets::cv20::io::check_cv20_conformance_controlled(&value,&mut control),
            "sav"=>semio_s_artifact_stdio_ifc_2x3_sav::standards::v2x3::subsets::sav::io::check_sav_conformance_controlled(&value,&mut control),
            _=>semio_s_artifact_stdio_ifc_2x3_cobie::standards::v2x3::subsets::cobie::io::check_cobie_conformance_controlled(&value,&mut control),
        };
        assert_eq!(refused.unwrap_err().kind,ValueRefusalKind::OwnershipLimit);
    }
    drop(value);
}
