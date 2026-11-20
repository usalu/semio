use semio_framework_value_testing_allocator as _;
use super::*;
/// 🏭️ The actual Drawing catalog declares every constructor currency and retains one shared semantic source.
#[test]
fn original_drawing_catalog_prices_real_wire_and_shared_semantic_sources(){
 let d=DrawingMemberStoreProducer::source_demands().unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};for axis in[0,1,2,4]{let mut denied=g;match axis{0=>denied.maximum_items=0,1=>denied.maximum_copy_bytes=d.copy_bytes-1,2=>denied.maximum_capacity_bytes=d.capacity_bytes-1,_=>denied.maximum_depth=d.depth-1}let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||DrawingMemberStoreProducer::admit_sources(denied));let rejected=match result{Err(rejected)=>rejected,Ok(_)=>panic!("denied original catalog admitted")};assert!(rejected.owners.is_none());assert_eq!(rejected.progress,Default::default());assert_eq!((born,released),(0,0));}
 let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||DrawingMemberStoreProducer::admit_sources(g));let(mut owners,p)=match result{Ok(v)=>v,Err(_)=>panic!("full original catalog refused")};assert!(p.fits(g));assert_eq!((born,released),(p.retained_capacity_bytes,p.released_bytes));assert!(owners.original_semantic_issuer_is_shared());assert!(owners.one_item_wire_preparation.is_some());
 in_decoder(|control| close_catalog(&mut owners,control));
}

use semio_framework_value::{
    RetainedWorkPoll,
    factory::owned::FactoryOwnedRetirement,
    retained_clone::{OriginalEffectReply, RetainedCloneSource, RetainedCloneStep, RetainedOwnedProjectionAdmission},
};
use semio_framework_value_native_owned::{
    OriginalSourceTurnIdentity, OriginalSourceTurnIssuer, native_decoding::*,
};

const BODY: usize = 1 << 16;
const WORK: u64 = 1 << 12;
const GRANT: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 1, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20,
    maximum_release_bytes: 1 << 20, maximum_depth: 128,
};

struct DrawingWireFixture(OriginalSourceTurnIdentity, RetainedCloneProgress);

impl OriginalConstructionRecipient for DrawingWireFixture {
    type Context<'turn, 'source> = () where 'source: 'turn;

    fn validate_context<'turn, 'source>(&self, _: &()) -> Result<(), ValueError> where 'source: 'turn { Ok(()) }
    fn source_turn<'turn, 'source>(&self, _: &()) -> Option<OriginalSourceTurnIdentity> where 'source: 'turn { Some(self.0) }
    fn grant<'a, 'turn, 'source>(&mut self, _: &mut (), quote: OriginalConstructionGrant<'a>) -> OriginalConstructionGrantComplete<'a> where 'source: 'turn {
        assert!(semio_framework_value::retained_clone::retained_clone_grant_admits(GRANT, quote.demand()));
        quote.complete(Ok(GRANT))
    }
    fn accept<'a, 'turn, 'source>(&mut self, _: &mut (), receipt: OriginalConstructionAccept<'a>) -> OriginalConstructionAcceptComplete<'a> where 'source: 'turn {
        let result = self.1.checked_add(receipt.progress()).map(|total| self.1 = total);
        receipt.complete(result)
    }
    fn submitted<'turn, 'source>(&self, _: &()) -> RetainedCloneProgress where 'source: 'turn { self.1 }
}

impl OriginalWorkRecipient for DrawingWireFixture {
    type Context<'turn, 'source> = () where 'source: 'turn;

    fn validate_context<'turn, 'source>(&self, _: &()) -> Result<(), ValueError> where 'source: 'turn { Ok(()) }
    fn source_turn<'turn, 'source>(&self, _: &()) -> Option<OriginalSourceTurnIdentity> where 'source: 'turn { Some(self.0) }
    fn settle_partial<'a, 'turn, 'source>(&mut self, _: &mut (), check: OriginalWorkPartialCheck<'a>) -> OriginalWorkPartialCheckComplete<'a> where 'source: 'turn { check.complete(Ok(())) }
    fn recover_partial<'a, 'turn, 'source>(&mut self, _: &mut (), check: OriginalWorkPartialCheck<'a>) -> OriginalWorkPartialCheckComplete<'a> where 'source: 'turn { check.complete(Ok(())) }
    fn admits<'a, 'turn, 'source>(&mut self, _: &mut (), quote: OriginalWorkQuote<'a>, _: u64, _: u64) -> OriginalWorkQuoteComplete<'a> where 'source: 'turn { quote.complete(Ok(true)) }
}

fn in_decoder<R>(run: impl FnOnce(&mut NativeDecodeControl<'_>) -> R) -> R {
    let source = OriginalSourceTurnIssuer::new().issue().unwrap();
    let mut work = OriginalWorkAuthority::new(DrawingWireFixture(source.identity(), Default::default()));
    let mut construction = OriginalConstructionAuthority::new(DrawingWireFixture(source.identity(), Default::default()));
    let mut accept = |_| true;
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let mut control = NativeDecodeControl::new_retained(&mut accept);
    control.install_retirement_recipient(&mut recipient).unwrap();
    let value = control.with_source_witness(Some(source.witness()), |control| {
        work.with_loan(&mut (), |work| control.with_work_recipient(work, |control| {
            construction.with_loan(&mut (), |construction| control.with_construction_recipient(construction, |control| {
                Ok::<_, ValueError>(run(control))
            }))?
        })).unwrap().unwrap()
    });
    drop(control);
    assert!(recipient.terminal_is_empty());
    value
}

fn close_catalog(
    owners: &mut DocumentStoreOwners<DrawingSnapshot, DrawingMutation>,
    control: &mut NativeDecodeControl<'_>,
) {
    for _ in 0..10000 {
        let (step, born, released) = semio_framework_trace_memory_testing::observe_backing(|| {
            control.with_effect(GRANT, WORK, |_, effect| owners.close_uninstalled_owners_step(effect)).unwrap().unwrap().unwrap()
        });
        assert!(step.progress().fits(GRANT));
        assert_eq!((born, released), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
        if matches!(step, RetainedCloneStep::Complete(_)) { return }
    }
    panic!("actual Drawing catalog failed to close")
}

fn receive(
    control: &mut NativeDecodeControl<'_>,
    octets: Vec<u8>,
    issuer: &Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<DrawingMutation>>,
    expected: &serde_json::Value,
) {
    let pointer = octets.as_ptr();
    let mut admission = RetainedCloneSource::admission_owned(octets, ());
    let mut source = None;
    for _ in 0..10000 {
        if admission.terminal_is_empty() { break }
        let (demand, work) = match admission.phase() {
            "ready" => (admission.take_output_demand(), admission.take_output_work_demand()),
            "projectionStart" => (admission.owned_projection_demand().unwrap(), 1),
            _ => (admission.next_demand().unwrap(), admission.advance_work_demand().unwrap()),
        };
        control.with_allocated_effect_reported(demand, work, |_, effect| {
            match admission.phase() {
                "ready" => admission.take_output(&mut source, effect),
                "projectionStart" => admission.project_owned(effect),
                _ => admission.advance(effect),
            }.result.unwrap()
        }).unwrap().unwrap();
    }
    assert!(admission.terminal_is_empty());
    let mut source = source.unwrap();
    let mut admission = RetainedOwnedProjectionAdmission::default();
    let original = source.try_borrow().unwrap();
    control.with_allocated_effect_reported(admission.demand(original).unwrap(), admission.work_demand(original).unwrap(), |_, effect| {
        let mut reply = None;
        drop(admission.admit(original, effect, OriginalEffectReply::borrow(&mut reply).unwrap()).unwrap());
        assert!(matches!(reply.unwrap().unwrap(), RetainedCloneStep::Complete(_)));
    }).unwrap().unwrap();
    let mut projection = None;
    control.with_effect(GRANT, WORK, |_, effect| {
        let mut reply = None;
        drop(admission.take(&mut projection, effect, OriginalEffectReply::borrow(&mut reply).unwrap()).unwrap());
        assert!(matches!(reply.unwrap().unwrap(), RetainedCloneStep::Complete(_)));
    }).unwrap().unwrap();
    let projection = projection.unwrap();
    assert_eq!(projection.borrow().unwrap().get().as_ptr(), pointer);
    let ledger = control.ledger_identity();
    let identity = control.retirement_recipient_identity();
    let (mut receiver, _) = semio_framework_pack_json::OriginalJsonReceiving::<DrawingMutation>::admit(
        projection, issuer, b"op", |m| m, control, GRANT,
    ).map_err(|(error, _)| error).unwrap();

    for _ in 0..100000 {
        if receiver.ready() { break }
        let witness = control.source_witness().unwrap();
        if let RetainedWorkPoll::Complete(outcome) = receiver.advance(control, witness, BODY, WORK, GRANT) {
            outcome.result.unwrap();
        }
    }
    assert!(receiver.ready());
    let quote = receiver.take_original_demand(BODY).unwrap();
    let denied = control.with_effect(GRANT, 0, |control, effect| receiver.take_original(control, BODY, effect)).unwrap();
    if let Some(denied) = denied {
        assert_eq!(denied.work, 0);
        assert!(denied.result.is_err());
    }
    assert!(receiver.ready());
    assert_eq!(receiver.take_original_demand(BODY).unwrap(), quote);

    let mut received = None;
    for _ in 0..100000 {
        let (_, value) = control.with_effect(GRANT, WORK, |control, effect| {
            receiver.take_original(control, BODY, effect)
        }).unwrap().unwrap().result.unwrap();
        if let Some((value, exact_issuer)) = value {
            assert!(Arc::ptr_eq(&exact_issuer, issuer));
            assert_eq!(&serde_json::to_value(&value).unwrap(), expected);
            received = Some((value, exact_issuer));
            break
        }
    }
    let (value, exact_issuer) = received.unwrap();

    for _ in 0..100000 {
        if receiver.terminal_is_empty() { break }
        control.with_effect(GRANT, WORK, |control, effect| {
            control.with_retirement(GRANT, |original| {
                assert_eq!(original.ledger_identity(), ledger);
                assert_eq!(original.retirement_recipient_identity(), identity);
                assert_close(receiver.close_step(original, effect));
                Ok::<_, ValueError>(())
            }).unwrap();
        }).unwrap().unwrap();
    }
    assert!(receiver.terminal_is_empty());
    for _ in 0..100000 {
        if source.terminal_is_empty() { break }
        let demand = source.next_close_demand(BODY).unwrap();
        let work = source.next_close_work_demand(BODY).unwrap();
        control.with_allocated_effect_reported(demand, work, |_, effect| assert_close(source.close_step(BODY, effect))).unwrap().unwrap();
    }
    assert!(source.terminal_is_empty());
    let (mut value, mut issuer, mut retirement) = (Some(value), Some(exact_issuer), None);
    let demand = FactoryOwnedRetirement::original_admission_demands(&value);
    control.with_allocated_effect_reported(demand, 1, |_, effect| {
        if let RetainedWorkPoll::Complete(turn) = FactoryOwnedRetirement::admit_original(&mut value, &mut issuer, &mut retirement, effect) {
            turn.result.unwrap();
        }
    }).unwrap().unwrap();
    let mut retirement = retirement.unwrap();
    for _ in 0..100000 {
        if retirement.terminal_is_empty() { break }
        let demand = retirement.next_demand(BODY).unwrap();
        let work = retirement.next_computational_work(BODY).unwrap();
        control.with_allocated_effect_reported(demand, work, |control, effect| {
            if retirement.requires_original_control() {
                control.with_retirement(GRANT, |original| original.with_original_close(|loan| {
                    assert_close(retirement.close_original_step(loan, BODY, effect));
                    Ok::<_, ValueError>(())
                })).unwrap();
            } else {
                assert_close(retirement.close_step(BODY, effect));
            }
        }).unwrap().unwrap();
    }
    assert!(retirement.terminal_is_empty());
}

fn assert_close(poll: semio_framework_value::OriginalRetirementPoll) {
    if let RetainedWorkPoll::Complete(turn) = poll { turn.result.unwrap(); }
}

/// 📨️ Both paths publish native Drawing mutations through the original wire decoder and return their exact issuer.
#[test]
fn original_drawing_wire_receives_actual_mutations_without_a_cold_decoder() {
    for fixture in [
        include_str!("../../../../🏷️metadata/🧪️tests/🧫️fixtures/🧬️mutations/🖼️set-artboard/↔️resize/🦠️mutation/🔣️.json"),
        include_str!("../../../../🏷️metadata/🧪️tests/🧫️fixtures/🧬️mutations/👁️set-layer-visible/🙈️hides/🦠️mutation/🔣️.json"),
    ] {
        let typed: DrawingMutation = serde_json::from_str(fixture).unwrap();
        let expected = serde_json::to_value(&typed).unwrap();
        let demand = DrawingMemberStoreProducer::source_demands().unwrap();
        let grant = RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_depth: demand.depth, ..GRANT };
        let (mut catalog, _) = DrawingMemberStoreProducer::admit_sources(grant).unwrap_or_else(|_| panic!("actual Drawing catalog refused"));
        let mut octets = b"op".to_vec();
        octets.extend(serde_json::to_vec(&typed).unwrap());
        in_decoder(|control| {
            receive(control, octets, catalog.mutation_retirement.as_ref().unwrap(), &expected);
            close_catalog(&mut catalog, control);
        });
    }
}
