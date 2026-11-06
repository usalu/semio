//! 🧪️ The source scope retains the original caller and decoder while work is pending.
use super::*;
use semio_framework_job::{Generation, OperationId, OriginalJobDecodeOwner};
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
use semio_framework_value::RetainedCloneGrant;

fn clock() -> Option<u64> { Some(1) }
fn grant() -> RetainedCloneGrant { RetainedCloneGrant { maximum_items: 128, maximum_copy_bytes: 32768, maximum_capacity_bytes: 32768, maximum_release_bytes: 32768, maximum_depth: 128 } }

#[test]
fn scope_fences_the_original_decoder_until_pending_work_closes() {
    let expected: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(381), Generation(7), grant(), clock, u64::MAX, 128);
    let mut original = OriginalJobDecodeOwner::empty();
    let mut foreign = OriginalJobDecodeOwner::empty();
    let mut scope = None;
    assert!(!caller.turn_with_decoder(grant(), &mut original, |cx| claim_scope(&mut scope, false, cx)).unwrap());
    assert_eq!(caller.turn_with_decoder(grant(), &mut original, |cx| claim_scope(&mut scope, true, cx)).unwrap(), expected["same"].as_bool().unwrap());
    assert_eq!(caller.turn_with_decoder(grant(), &mut foreign, |cx| claim_scope(&mut scope, true, cx)).unwrap(), expected["foreignPending"].as_bool().unwrap());
    assert_eq!(caller.turn_with_decoder(grant(), &mut foreign, |cx| claim_scope(&mut scope, false, cx)).unwrap(), expected["foreignIdle"].as_bool().unwrap());
    assert_eq!(caller.turn_with_decoder(grant(), &mut foreign, |cx| claim_scope(&mut scope, true, cx)).unwrap(), expected["rebound"].as_bool().unwrap());
    for decoder in [&mut original, &mut foreign] {
        while !decoder.terminal_is_empty() {
            caller.turn(grant(), |cx| cx.with_original_retirement(|cx| {
                let demand = decoder.retirement_demands(32768)?;
                decoder.close_on_source(cx)?;
                Ok(())
            })).unwrap();
        }
    }
    caller.retire(grant());
}
