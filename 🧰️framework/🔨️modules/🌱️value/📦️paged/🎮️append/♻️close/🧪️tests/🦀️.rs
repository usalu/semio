use super::*;
#[test]
fn append_close_preserves_real_header_and_counts_new_failure_not_cached_failure(){
 let expected:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();
 let mut owner=PagedUtf8AppendCursor{source:Some((17,3)),destination:Some((29,7)),..Default::default()};
 let grant=RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()};
 let denied=owner.close_step(Default::default());assert_eq!(owner.source,Some((17,3)));assert_eq!(owner.destination,Some((29,7)));
 let invalid=owner.close_step(grant);assert!(invalid.result.is_err());let cached_invalid=owner.close_step(grant);assert!(cached_invalid.result.is_err());
 owner.begin_close();let under_depth=owner.close_step(RetainedCloneGrant{maximum_depth:0,..grant});assert_eq!(owner.source,Some((17,3)));assert_eq!(owner.destination,Some((29,7)));assert_eq!(owner.next_close_copy_byte_demand().unwrap(),0);
 let clear=owner.close_step(grant);let copied=clear.result.as_ref().unwrap().progress().copied_bytes;assert!(owner.terminal_is_empty());assert_eq!(clear.result.as_ref().unwrap().progress().copied_items,1);
 let cached=owner.close_step(Default::default());assert!(matches!(cached.result,Ok(RetainedCloneStep::Complete(_))));
 let actual=serde_json::json!({"turns":[{"case":"denied","work":denied.work},{"case":"invalid","work":invalid.work},{"case":"cachedInvalid","work":cached_invalid.work},{"case":"underDepth","work":under_depth.work},{"case":"clear","work":clear.work},{"case":"cachedComplete","work":cached.work}],"copyBytes":copied,"headerIdentity":[[17,3],[29,7]]});assert_eq!(actual,expected);
}
