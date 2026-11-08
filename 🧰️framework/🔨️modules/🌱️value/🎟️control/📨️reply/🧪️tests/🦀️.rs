use super::*;
fn fixture()->serde_json::Value{serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🧪️tests/cases.json")).unwrap()}
#[test]fn actual_completed_and_pending_status_share_the_original_slot(){
 let law=fixture();let mut output=None;let mut key=None;
 {let reply=OriginalEffectReply::<RetainedCloneStep,String>::borrow(&mut output).unwrap();let mut scope=reply.progress().scope(&mut key,1u64).unwrap();assert!(!scope.recovering());assert!(!scope.borrow().unwrap().pending().is_recorded());assert!(!scope.forwarded().is_recorded())}
 assert_eq!(key.is_some(),law["pendingRetainsKey"].as_bool().unwrap());assert!(output.is_none());
 {let reply=OriginalEffectReply::<RetainedCloneStep,String>::borrow(&mut output).unwrap();assert!(reply.scope(&mut key,2u64).is_err())}
 assert_eq!(key,Some(1));
 {let reply=OriginalEffectReply::borrow(&mut output).unwrap();let mut scope=reply.progress().scope(&mut key,1u64).unwrap();assert!(scope.recovering());assert!(scope.borrow().unwrap().complete(Ok(RetainedCloneStep::Complete(Default::default()))).is_recorded());assert!(scope.forwarded().is_recorded())}
 assert!(matches!(output,Some(Ok(RetainedCloneStep::Progress(_)))));assert_eq!(key.is_none(),law["completionClearsKey"].as_bool().unwrap())
}
#[test]fn nested_completed_reply_clears_every_original_key_before_producer_unwind(){
 let law=fixture();let error=law["error"].as_str().unwrap().to_owned();let pointer=error.as_ptr();let mut output=None;let(mut parent,mut child)=(None,None);
 {let reply=OriginalEffectReply::<(),String>::borrow(&mut output).unwrap();let mut outer=reply.scope(&mut parent,1u64).unwrap();let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{
  let reply=outer.borrow().unwrap();let mut inner=reply.scope(&mut child,1u64).unwrap();let recorded=inner.borrow().unwrap().complete(Err(error));assert!(recorded.is_recorded());drop(recorded);panic!("native child after actual reply deposit")
 }));assert!(interrupted.is_err());assert!(outer.is_recorded());assert!(outer.forwarded().is_recorded())}
 assert_eq!(output.as_ref().unwrap().as_ref().unwrap_err().as_ptr(),pointer);assert!(parent.is_none()&&child.is_none());assert_eq!(law["nestedKeys"],2)
}

#[test]
fn exclusive_reply_scope_keeps_send_only_owned_output_before_unwind(){
 let law:serde_json::Value=serde_json::from_str(include_str!("cases.json")).unwrap();let mut output:Option<Result<Box<std::cell::Cell<u64>>,String>>=None;let value=Box::new(std::cell::Cell::new(law["sendOnly"]["value"].as_u64().unwrap()));let pointer=std::ptr::from_ref(value.as_ref());let mut key=None;let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{let mut scope=OriginalEffectReply::borrow(&mut output).unwrap().scope(&mut key,73u64).unwrap();scope.borrow().unwrap().complete(Ok(value));assert!(scope.is_recorded());panic!("send-only output deposited before unwind")}));assert!(interrupted.is_err());assert!(key.is_none());let value=output.take().unwrap().unwrap();assert_eq!(std::ptr::from_ref(value.as_ref()),pointer);assert_eq!(value.get(),law["sendOnly"]["value"].as_u64().unwrap());
}
