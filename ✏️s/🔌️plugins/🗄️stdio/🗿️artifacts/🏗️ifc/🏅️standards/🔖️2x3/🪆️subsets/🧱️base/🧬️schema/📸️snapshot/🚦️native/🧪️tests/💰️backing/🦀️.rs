//! 💰️ IFC2x3 inline reconstruction and actual conformance consumers request only concrete backing.
use super::*;
#[test]
fn sqlite_snapshot_ifc2x3_inline_root_and_mvd_consumers_follow_concrete_backing_roles(){
    let vector:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🪶️sqlite/💰️backing/🔣️.json")).unwrap();
    assert_eq!(vector["inlineRoot"],"noBackingRequest");assert_eq!(vector["nativeRoles"]["owned"],"maxAllocationBytes");
    let mut progress=|_|true;let mut native=NativeDecodeControl::new(0,&mut progress);
    let frame=Frame{schema:String::new(),file_description:Vec::new(),file_name:Vec::new(),file_schema:Vec::new(),instances:Vec::new(),values:Vec::new(),edm:None};
    let result=reconstruct(frame,&mut native).expect("inline empty IFC2x3 root has no heap backing");assert_eq!(native.owned_bytes(),0);
    assert!(result.schema.is_empty());assert!(result.document.instances.is_empty());assert!(result.edm_preamble.is_none());close(result);
    let value=crate::standards::v2x3::subsets::base::schema::demo_ifc2x3_snapshot();
    for subset in ["cv20","sav","cobie"]{
        let limits=store::sqlite_snapshot::SqliteDatabaseLimits{max_allocation_bytes:0,..Default::default()};
        let mut progress=|_|true;let mut control=SqliteSnapshotControl::new(&mut progress,limits);
        let refused=match subset{
            "cv20"=>crate::standards::v2x3::subsets::cv20::schema::check_cv20_conformance_controlled(&value,&mut control),
            "sav"=>crate::standards::v2x3::subsets::sav::schema::check_sav_conformance_controlled(&value,&mut control),
            _=>crate::standards::v2x3::subsets::cobie::schema::check_cobie_conformance_controlled(&value,&mut control),
        };
        assert_eq!(refused.unwrap_err().kind,ValueRefusalKind::OwnershipLimit);
    }
    close(value);
}
