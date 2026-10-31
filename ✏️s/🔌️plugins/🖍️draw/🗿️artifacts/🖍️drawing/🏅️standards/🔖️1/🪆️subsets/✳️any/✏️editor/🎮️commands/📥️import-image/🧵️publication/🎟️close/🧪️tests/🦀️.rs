use super::*;
use semio_framework_value::retirement::{RetirementCursor,RetirementStep};
#[test]
fn publication_hash_fixed_header_clear_has_no_payload_copy(){
 let expected:serde_json::Value=serde_json::from_str(include_str!("../../📐️.json")).unwrap();
 let mut cursor=PublicationHashRetirement{value:Some(PublicationHash(semio_framework_hash::Sha256::new()))};
 let denied=cursor.close_step(RetainedCloneGrant::default());assert!(cursor.value.is_some());assert!(matches!(&denied,RetirementStep::BudgetExhausted));assert_eq!(cursor.next_work_byte_demand().unwrap(),0);
 let clear=cursor.close_step(RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()});assert!(cursor.terminal_is_empty());
 let cached=cursor.close_step(RetainedCloneGrant::default());
 let actual=serde_json::json!({"role":"publication","turns":[{"case":"denied","work":denied.work(),"items":0,"copyBytes":0},{"case":"clear","work":clear.work(),"items":1,"copyBytes":0},{"case":"cached","work":cached.work(),"items":0,"copyBytes":0}]});assert_eq!(actual,expected);assert!(matches!(clear,RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:0,..},1)));
}
