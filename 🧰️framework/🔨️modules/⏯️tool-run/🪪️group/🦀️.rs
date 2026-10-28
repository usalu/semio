//! 🧪️ Original full-range group identity keeps its caller-owned bytes.
use super::*;

#[test]
fn original_group_identity_uses_caller_owned_fixed_storage(){
    let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for row in law["cases"].as_array().unwrap(){let id=ToolRunId{app_instance_id:0,run:row["run"].as_str().unwrap().parse().unwrap()};let mut buffer=[0;32];let pointer=buffer.as_ptr();let group=id.group_id_into(&mut buffer);assert_eq!(group,row["group"].as_str().unwrap());assert_eq!(group.as_ptr(),pointer);assert_eq!(group,id.group_id());}
}
