//! 🧪️ Progressive guided request capture returns the exact original donor and raw typed drafts.
use super::*;
#[test]
fn guided_request_copy_preserves_original_donor_and_exact_utf8_payload_at_every_cancel_prefix(){
    use super::super::type_guidance_request::RequestCopy;
    use semio_framework_plugin::action_staging::{StagedFormRequest,StagedFormToken};
    let law:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/guided-request-copy.json")).unwrap();
    let mut successful_turns=None;
    for cancel_at in std::iter::once(None).chain((0..100000).map(Some)){
        if cancel_at.is_some_and(|prefix|successful_turns.is_some_and(|turns|prefix>=turns)){break;}
        let mut caller=OriginalWorkFixture::new();let mut copy=Some(RequestCopy::default());
        let args=super::type_input_tests::raw(&law["args"]);
        let mut donor=Some(StagedFormRequest{token:StagedFormToken{action:editing::SET_SNAPSHOT_VALUE_ACTION_ID.into(),window:"source-query".into(),generation:"1".into(),revision:"0".repeat(64)},args,locale:semio_framework_ui_locale::Locale::En,page:None});
        let original=donor.as_ref().unwrap().token.action.as_ptr();let mut done=false;
        for turn in 0..100000{
            if cancel_at==Some(turn){caller.cancel().cancel_now();}
            let demand=copy.as_ref().unwrap().demands(4096).unwrap();
            for axis in 0..5{let mut denied=grant();match axis{0=>denied.maximum_items=0,1 if demand.copy_bytes>0=>denied.maximum_copy_bytes=demand.copy_bytes-1,2 if demand.capacity_bytes>0=>denied.maximum_capacity_bytes=demand.capacity_bytes-1,3 if demand.release_bytes>0=>denied.maximum_release_bytes=demand.release_bytes-1,4=>denied.maximum_depth=0,_=>continue};
                if cancel_at.is_some_and(|prefix|turn>=prefix){break;}
                let before=donor.as_ref().map(|value|value.token.action.as_ptr());let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(denied,|cx|copy.as_mut().unwrap().advance(&mut donor,cx)));assert!(!result.unwrap());assert_eq!(before,donor.as_ref().map(|value|value.token.action.as_ptr()));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            }
            let step=caller.turn(grant(),|cx|copy.as_mut().unwrap().advance(&mut donor,cx));
            if cancel_at.is_some_and(|prefix|turn>=prefix){assert!(step.is_err());break;}
            if step.unwrap(){done=true;if cancel_at.is_none(){successful_turns=Some(turn+1);}break;}
        }
        if cancel_at.is_none(){assert!(done);assert_eq!(donor.as_ref().unwrap().token.action.as_ptr(),original);let mut args=None;assert!(caller.turn(grant(),|cx|copy.as_mut().unwrap().take_args(&mut args,cx)).unwrap());assert_eq!(args.as_ref().unwrap(),&donor.as_ref().unwrap().args);retire_owner(&mut caller,&mut args);}
        retire_owner(&mut caller,&mut copy);retire_owner(&mut caller,&mut donor);caller.finish();
    }
}
pub(super) fn retire_owner<T:semio_framework_value::retirement::RetireOwned>(caller:&mut OriginalWorkFixture,owner:&mut Option<T>){let mut close=RetainedCloneClose::default();for _ in 0..100000{if owner.is_none()&&close.is_empty(){return;}caller.turn(grant(),|cx|cx.with_original_retirement(|cx|{let demand=close.next_owner_demands::<T>(owner.is_some(),&None,4096)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|if close.is_empty(){close.begin_granted(owner,grant)?.ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"request owner close missing"))}else{close.step_granted(grant)})?;Ok(())})).unwrap();}panic!("request owner did not retire");}
