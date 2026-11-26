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
 let law=fixture();let error=law["error"].as_str().unwrap().to_owned();let pointer=error.as_ptr();let mut output=None;let(mut parent,mut child)=(None,None);let mut mapped=0;
 {let reply=OriginalEffectReply::<(),String>::borrow(&mut output).unwrap();let mut outer=reply.scope_mapped(&mut parent,1u64,|result|{mapped+=1;result}).unwrap();let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{
  let reply=outer.borrow().unwrap();let mut inner=reply.scope(&mut child,1u64).unwrap();let recorded=inner.borrow().unwrap().complete(Err(error));assert!(recorded.is_recorded());drop(recorded);panic!("native child after actual reply deposit")
 }));assert!(interrupted.is_err());assert!(outer.is_recorded());assert!(outer.forwarded().is_recorded())}
 assert_eq!(output.as_ref().unwrap().as_ref().unwrap_err().as_ptr(),pointer);assert!(parent.is_none()&&child.is_none());assert_eq!(mapped,1);assert_eq!(law["nestedKeys"],2)
}

#[test]
fn exclusive_reply_scope_keeps_send_only_owned_output_before_unwind(){
 let law:serde_json::Value=serde_json::from_str(include_str!("cases.json")).unwrap();let mut output:Option<Result<Box<std::cell::Cell<u64>>,String>>=None;let value=Box::new(std::cell::Cell::new(law["sendOnly"]["value"].as_u64().unwrap()));let pointer=std::ptr::from_ref(value.as_ref());let mut key=None;let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{let mut scope=OriginalEffectReply::borrow(&mut output).unwrap().scope(&mut key,73u64).unwrap();scope.borrow().unwrap().complete(Ok(value));assert!(scope.is_recorded());panic!("send-only output deposited before unwind")}));assert!(interrupted.is_err());assert!(key.is_none());let value=output.take().unwrap().unwrap();assert_eq!(std::ptr::from_ref(value.as_ref()),pointer);assert_eq!(value.get(),law["sendOnly"]["value"].as_u64().unwrap());
}

#[test]
fn refused_producer_leaves_no_unsettled_action_key_and_a_later_close_can_begin(){
 let law=fixture();let mut output=None;let mut route=None;
 {let reply=OriginalEffectReply::<RetainedCloneStep,String>::borrow(&mut output).unwrap();let mut scope=reply.scope(&mut route,7u64).unwrap();assert!(scope.borrow().is_ok())}
 assert_eq!(route.is_none(),law["refusedStepClearsKey"].as_bool().unwrap());assert!(output.is_none());
 let mut close_action=None;
 {let reply=OriginalEffectReply::<RetainedCloneStep,String>::borrow(&mut output).unwrap();let scope=reply.progress().scope(&mut close_action,(1u64,2u8,3usize)).unwrap();drop(scope)}
 assert!(close_action.is_none());
 let reply=OriginalEffectReply::<RetainedCloneStep,String>::borrow(&mut output).unwrap();let scope=reply.progress().scope(&mut close_action,(1u64,1u8,3usize)).unwrap();assert!(!scope.recovering());assert!(scope.forwarded().is_recorded()==false);assert!(close_action.is_some())
}
#[test]
fn recovered_key_survives_a_refused_recovery_and_an_unwind(){
 let law=fixture();let mut output=None;let mut key=Some(9u64);
 {let reply=OriginalEffectReply::<(),String>::borrow(&mut output).unwrap();let scope=reply.scope(&mut key,9u64).unwrap();assert!(scope.recovering())}
 assert_eq!(key.is_some(),law["recoveredKeyRetained"].as_bool().unwrap());
 let mut fresh=None;let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{let reply=OriginalEffectReply::<(),String>::borrow(&mut output).unwrap();let _scope=reply.scope(&mut fresh,4u64).unwrap();panic!("producer unwound before any reply")}));
 assert!(interrupted.is_err());assert_eq!(fresh,Some(4))
}
