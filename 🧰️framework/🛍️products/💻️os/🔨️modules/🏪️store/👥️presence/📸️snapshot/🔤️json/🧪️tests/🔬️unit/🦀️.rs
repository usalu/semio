//! 🧪️ Presence JSON native codec round-trips language-agnostic fixtures and agrees with an independent Serde oracle.
use super::*;
use semio_framework_io_control::{NativeSnapshotCause, NativeSnapshotDecodeOwner, NativeSnapshotEncodeOwner, NativeSnapshotFailure};
use semio_framework_value::{AuthoringWallet, OriginalRetirementPoll, RetainedAllocationEffect, RetainedWorkOutcome, RetainedWorkPoll};
use crate::ArtifactPresenceSnapshot;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
use semio_framework_value::{native_encoding::NativeEncodeRetirementRecipient, NativeEncodeControl};
use semio_framework_value_native_owned::{native_decoding::NativeDecodeRetirementRecipient, native_decoding::NativeDecodeControl};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ToValue, FromValue, semio_framework_value_derive::NativeReceive, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct Camera { eye_height: f64, zoom: f64, target: Option<String> }

use semio_framework_value_native_owned::native_decoding::{OriginalConstructionAccept, OriginalConstructionAcceptComplete, OriginalConstructionAuthority, OriginalConstructionGrant, OriginalConstructionGrantComplete, OriginalConstructionRecipient, OriginalSourceTurnIdentity, OriginalWorkAuthority, OriginalWorkPartialCheck, OriginalWorkPartialCheckComplete, OriginalWorkQuote, OriginalWorkQuoteComplete, OriginalWorkRecipient};

/// 🏗️ Test construction recipient: issues the stock grant for every quote and accumulates the submitted receipts.
struct ConstructionLoan { current: semio_framework_value::RetainedCloneGrant, submitted: semio_framework_value::RetainedCloneProgress }
impl OriginalConstructionRecipient for ConstructionLoan {
    type Context<'turn, 'source> = () where 'source: 'turn;
    fn validate_context<'turn, 'source>(&self, _: &()) -> Result<(), ValueError> where 'source: 'turn { Ok(()) }
    fn source_turn<'turn, 'source>(&self, _: &()) -> Option<OriginalSourceTurnIdentity> where 'source: 'turn { None }
    fn grant<'a, 'turn, 'source>(&mut self, _: &mut (), quote: OriginalConstructionGrant<'a>) -> OriginalConstructionGrantComplete<'a> where 'source: 'turn { quote.complete(Ok(self.current)) }
    fn accept<'a, 'turn, 'source>(&mut self, _: &mut (), receipt: OriginalConstructionAccept<'a>) -> OriginalConstructionAcceptComplete<'a> where 'source: 'turn {
        let (granted, progress) = (receipt.grant(), receipt.progress());
        assert!(progress.fits(granted) && progress.fits(self.current));
        let posted = self.submitted.checked_add(progress).map(|total| self.submitted = total);
        receipt.complete(posted)
    }
    fn submitted<'turn, 'source>(&self, _: &()) -> semio_framework_value::RetainedCloneProgress where 'source: 'turn { self.submitted }
}

/// ⛽️ Test work recipient: admits any quote that fits the finite original stock.
struct WorkLoan<'a>(&'a u128);
impl OriginalWorkRecipient for WorkLoan<'_> {
    type Context<'turn, 'source> = () where 'source: 'turn;
    fn validate_context<'turn, 'source>(&self, _: &()) -> Result<(), ValueError> where 'source: 'turn { Ok(()) }
    fn source_turn<'turn, 'source>(&self, _: &()) -> Option<OriginalSourceTurnIdentity> where 'source: 'turn { None }
    fn admits<'reply, 'turn, 'source>(&mut self, _: &mut (), quote: OriginalWorkQuote<'reply>, observed: u64, additional: u64) -> OriginalWorkQuoteComplete<'reply> where 'source: 'turn { quote.complete(Ok(observed.checked_add(additional).is_some_and(|total| u128::from(total) <= *self.0))) }
    fn settle_partial<'reply, 'turn, 'source>(&mut self, _: &mut (), check: OriginalWorkPartialCheck<'reply>) -> OriginalWorkPartialCheckComplete<'reply> where 'source: 'turn { check.complete(Ok(())) }
    fn recover_partial<'reply, 'turn, 'source>(&mut self, _: &mut (), check: OriginalWorkPartialCheck<'reply>) -> OriginalWorkPartialCheckComplete<'reply> where 'source: 'turn { check.complete(Ok(())) }
}

fn receive<T, E: From<ValueError>>(control: &mut NativeDecodeControl<'_>, grant: semio_framework_value::RetainedCloneGrant, work_loan: WorkLoan<'_>, apply: impl FnOnce(&mut NativeDecodeControl<'_>) -> RetainedWorkOutcome<T, E>) -> RetainedWorkOutcome<T, E> {
    let mut construction = OriginalConstructionAuthority::new(ConstructionLoan { current: grant, submitted: Default::default() });
    let mut authority = OriginalWorkAuthority::new(work_loan);
    let mut work = 0;
    let result = authority.with_loan(&mut (), |work_loan| construction.with_loan(&mut (), |construction_loan| control.with_work_recipient(work_loan, |control| control.with_construction_recipient(construction_loan, |control| { let turn = apply(control); work = turn.work; turn.result }))));
    let result = match result { Ok(Ok(result)) => result, Ok(Err(error)) | Err(error) => Err(E::from(error)) };
    RetainedWorkOutcome { result, work }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ToValue, FromValue, semio_framework_value_derive::NativeReceive, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
enum ShowMode { #[default] Shaded, Wire }

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ToValue, FromValue, semio_framework_value_derive::NativeReceive, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct FixturePresence { camera: Camera, show_mode: ShowMode, selected_generation_id: Option<String>, label: String, revision: u64 }

impl ArtifactPresenceSnapshot for FixturePresence { fn decode_presence_native(bytes:&[u8],owner:&mut NativeSnapshotDecodeOwner<'_ ,'_>)->RetainedWorkOutcome<Self,NativeSnapshotFailure>{decode_presence_json(bytes,owner)} }

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap() }
fn grant(fixture: &serde_json::Value) -> semio_framework_value::RetainedCloneGrant {
    let axes = &fixture["grant"];
    semio_framework_value::RetainedCloneGrant { maximum_items: axes["items"].as_u64().unwrap() as usize, maximum_copy_bytes: axes["copyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: axes["capacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: axes["releaseBytes"].as_u64().unwrap() as usize, maximum_depth: axes["depth"].as_u64().unwrap() as usize }
}

const CLOSE_GRANT: semio_framework_value::RetainedCloneGrant = semio_framework_value::RetainedCloneGrant { maximum_items: 4096, maximum_copy_bytes: 65536, maximum_capacity_bytes: 16_777_216, maximum_release_bytes: 16_777_216, maximum_depth: 256 };

struct Observed<T> { result: Result<T, NativeSnapshotFailure>, progress: semio_framework_value::RetainedCloneProgress, heap: semio_framework_trace::HeapAllocationObservation, closure: semio_framework_trace::HeapAllocationObservation }

fn decode_with(bytes: &[u8], fixture: &serde_json::Value, grant: semio_framework_value::RetainedCloneGrant, alive: &std::cell::Cell<bool>) -> Observed<FixturePresence> {
    let mut callback = |_| alive.get();
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let mut native = NativeDecodeControl::new_retained(&mut callback);
    native.install_retirement_recipient(&mut recipient).unwrap();
    native.admit_turn_capacity(grant.maximum_capacity_bytes.min(fixture["nativeMaximumBytes"].as_u64().unwrap() as usize)).unwrap();
    let stock = 1u128 << 40;
    let mut progress = semio_framework_value::RetainedCloneProgress::default();
    let (outcome, heap) = observe(|| receive(&mut native, grant, WorkLoan(&stock), |control| {
        let mut owner = NativeSnapshotDecodeOwner::new(control, grant);
        let outcome = FixturePresence::decode_presence_native(bytes, &mut owner);
        progress = owner.progress();
        drop(owner);
        outcome
    }));
    let result = outcome.result;
    alive.set(true);
    let ((), closure) = observe(|| { for _ in 0..100_000 { if !native.has_retirement_owner() { return } native.admit_turn_capacity(CLOSE_GRANT.maximum_capacity_bytes).unwrap(); receive(&mut native, CLOSE_GRANT, WorkLoan(&stock), |control| { if control.with_retirement::<_, ValueError>(CLOSE_GRANT, |original| Ok(original.requires_original_birth())).unwrap() { settle_close(control.with_retirement::<_, ValueError>(CLOSE_GRANT, |original| Ok(original.close_original_birth_step(CLOSE_GRANT))).unwrap()); } else { let quote = control.with_retirement::<_, ValueError>(CLOSE_GRANT, |original| original.retirement_work_demand(CLOSE_GRANT.maximum_copy_bytes)).unwrap(); settle_close(control.with_effect(CLOSE_GRANT, quote, |control, effect| control.close_retirement_recipient(effect)).unwrap().expect("close work quote admitted")); } RetainedWorkOutcome { result: Ok::<(), ValueError>(()), work: 0 } }).result.unwrap(); } panic!("retirement did not terminate") });
    Observed { result, progress, heap, closure }
}

fn encode_with(value: &FixturePresence, fixture: &serde_json::Value, grant: semio_framework_value::RetainedCloneGrant, alive: &std::cell::Cell<bool>) -> Observed<Vec<u8>> {
    let mut callback = |_| alive.get();
    let mut recipient = NativeEncodeRetirementRecipient::new();
    let mut native = NativeEncodeControl::new_retained(&mut callback);
    native.install_retirement_recipient(&mut recipient).unwrap();
    native.admit_turn_capacity(grant.maximum_capacity_bytes.min(fixture["nativeMaximumBytes"].as_u64().unwrap() as usize)).unwrap();
    let mut owner = NativeSnapshotEncodeOwner::new(&mut native, grant);
    let (outcome, heap) = observe(|| value.encode_presence_native(&mut owner));
    let result = outcome.result;
    let progress = owner.progress();
    drop(owner);
    alive.set(true);
    let ((), closure) = observe(|| { for _ in 0..100_000 { if !native.has_retirement_owner() { return } native.admit_turn_capacity(CLOSE_GRANT.maximum_capacity_bytes).unwrap(); close_paid(|effect| native.close_retirement_recipient(effect)); } panic!("retirement did not terminate") });
    Observed { result, progress, heap, closure }
}

fn close_paid(close: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll) {
    let mut wallet = AuthoringWallet::new(CLOSE_GRANT, 1 << 20);
    let paid = wallet.run(CLOSE_GRANT, 1 << 10, close).expect("authoring close effect").expect("close work quote admitted");
    settle_close(paid.output);
}

fn settle_close(poll: OriginalRetirementPoll) {
    match poll {
        RetainedWorkPoll::Complete(turn) => { turn.result.unwrap_or_else(|failure| panic!("retirement close refused: {failure:?}")); }
        RetainedWorkPoll::Pending { .. } => panic!("retirement close stayed pending on a synchronous recipient"),
    }
}

fn primary(failure: &NativeSnapshotFailure) -> &ValueError {
    match failure.original_fields().cause { Some(NativeSnapshotCause::Value(error)) => error, _ => panic!("refusal carries a value cause") }
}

fn alive() -> std::cell::Cell<bool> { std::cell::Cell::new(true) }

fn refused_net<T>(observed: &Observed<T>) -> usize {
    let backing = match &primary(observed.result.as_ref().err().expect("refusal")).message { std::borrow::Cow::Owned(text) => text.capacity(), _ => 0 };
    (observed.heap.requested_bytes + observed.closure.requested_bytes) - (observed.heap.released_bytes + observed.closure.released_bytes) - backing
}

#[test]
fn accepted_rows_decode_like_serde_and_round_trip_exactly() {
    let fixture = fixture();
    for row in fixture["accepted"].as_array().unwrap() {
        let name = &row["name"];
        let bytes = row["input"].as_str().unwrap().as_bytes();
        let oracle: FixturePresence = serde_json::from_slice(bytes).unwrap();
        let decoded = decode_with(bytes, &fixture, grant(&fixture), &alive());
        let value = decoded.result.unwrap_or_else(|error| panic!("{name}: {error:?}"));
        assert_eq!(value, oracle, "{name}");
        assert_eq!(decoded.heap.requested_bytes, decoded.progress.retained_capacity_bytes, "{name}: decode requests are exactly the accepted capacity receipts");
        assert!(decoded.heap.released_bytes >= decoded.progress.released_bytes, "{name}: decode releases are never over-reported");
        assert!(decoded.progress.fits(grant(&fixture)), "{name}");
        assert_eq!((decoded.closure.requested_bytes, decoded.closure.released_bytes), (0, 0), "{name}: completed receiving leaves no pending retirement");
        let encoded = encode_with(&value, &fixture, grant(&fixture), &alive());
        let bytes_out = encoded.result.unwrap_or_else(|error| panic!("{name}: {error:?}"));
        assert_eq!((encoded.heap.requested_bytes, encoded.heap.released_bytes), (encoded.progress.retained_capacity_bytes, encoded.progress.released_bytes), "{name}: encode heap equals accepted receipts on both axes");
        assert!(encoded.progress.fits(grant(&fixture)), "{name}");
        let expected = serde_json::to_vec(&oracle).unwrap();
        assert_eq!(bytes_out, expected, "{name}: bytes equal the independent Serde serialization");
        let again = decode_with(&bytes_out, &fixture, grant(&fixture), &alive()).result.unwrap();
        assert_eq!(again, value, "{name}: decode(encode(x)) == x");
        let reencoded = encode_with(&again, &fixture, grant(&fixture), &alive()).result.unwrap();
        assert_eq!(reencoded, bytes_out, "{name}: canonical bytes are a fixed point");
    }
}

#[test]
fn refused_rows_agree_with_serde_and_leave_no_heap() {
    let fixture = fixture();
    for row in fixture["refused"].as_array().unwrap() {
        let name = &row["name"];
        let bytes = row["input"].as_str().unwrap().as_bytes();
        assert!(serde_json::from_slice::<FixturePresence>(bytes).is_err(), "{name}: oracle refuses");
        let decoded = decode_with(bytes, &fixture, grant(&fixture), &alive());
        assert!(decoded.result.is_err(), "{name}: ours refuses");
        assert_eq!(refused_net(&decoded), 0, "{name}: every refused partial owner is funded back to zero heap");
    }
}

#[test]
fn undersized_grants_and_cancellation_refuse_typed_without_leaking() {
    let fixture = fixture();
    let bytes = fixture["accepted"][1]["input"].as_str().unwrap().as_bytes();
    let value: FixturePresence = serde_json::from_slice(bytes).unwrap();
    for axis in ["items", "copyBytes", "capacityBytes", "depth"] {
        let mut denied = grant(&fixture);
        match axis { "items" => denied.maximum_items = 1, "copyBytes" => denied.maximum_copy_bytes = 0, "capacityBytes" => denied.maximum_capacity_bytes = 1024, _ => denied.maximum_depth = 1 }
        let decoded = decode_with(bytes, &fixture, denied, &alive());
        assert!(decoded.result.is_err(), "decode denied on {axis}");
        assert_eq!(refused_net(&decoded), 0, "decode {axis}: no leak");
        let encoded = encode_with(&value, &fixture, denied, &alive());
        assert!(encoded.result.is_err(), "encode denied on {axis}");
        assert_eq!(refused_net(&encoded), 0, "encode {axis}: no leak");
    }
    let dead = std::cell::Cell::new(false);
    let decoded = decode_with(bytes, &fixture, grant(&fixture), &dead);
    assert_eq!(decoded.result.as_ref().unwrap_err().kind(), ValueRefusalKind::Canceled);
    assert_eq!(refused_net(&decoded), 0, "canceled decode: no leak");
    dead.set(false);
    let encoded = encode_with(&value, &fixture, grant(&fixture), &dead);
    assert_eq!(encoded.result.as_ref().unwrap_err().kind(), ValueRefusalKind::Canceled);
    assert_eq!(refused_net(&encoded), 0, "canceled encode: no leak");
}
