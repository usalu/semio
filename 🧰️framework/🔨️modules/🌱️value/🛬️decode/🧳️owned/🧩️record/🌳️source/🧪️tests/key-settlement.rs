use super::*;
use crate::record::{settlement_of,Settle};
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};

#[test]
fn settled_key_request_maps_to_its_funded_clear() {
    let stock = serde_json::json!({"items":8,"copyBytes":4096,"capacityBytes":4096,"releaseBytes":4096,"depth":4,"fuel":8,"maximumTurns":8});
    let mut bank = EffectBank::new(Wallet::from_profile(&stock));
    let mut frame = NativeSchemaKeyFrame::<dyn NativeValueSource>::default();
    assert_eq!(settlement_of(&frame), None);
    let grant = bank.loan(1, 4096);
    frame.request = Some(Request { tag: 1, body: 4096, maximum: 1, grant, demand: Default::default(), work: 1 });
    assert_eq!(settlement_of(&frame), None);
    frame.settled_request = true;
    assert_eq!(settlement_of(&frame), Some(Settle::Request));
    let demand = NativeSchemaKeyFrame::<dyn NativeValueSource>::clear_request_demand();
    let grant = bank.loan(1, demand.copy_bytes);
    let mut reply = None;
    let (poll, actual) = bank.run(grant, 1, |effect| {
        let before = effect.performed_work();
        let token = OriginalEffectReply::borrow(&mut reply).unwrap();
        drop(frame.clear_request_into(effect, token).unwrap());
        crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome { result: Ok::<_, ValueError>(()), work: effect.performed_work() - before })
    });
    let crate::RetainedWorkPoll::Complete(turn) = poll else { panic!("settlement clear remained pending") };
    assert_eq!(turn.work, 1);
    assert_eq!(actual, None);
    let progress = reply.take().unwrap().unwrap().progress();
    assert_eq!(bank.take_admitted(), Some(true));
    bank.post(progress, grant, (0, 0));
    assert!(frame.request.is_none() && !frame.settled_request);
    assert_eq!(settlement_of(&frame), None);
}
