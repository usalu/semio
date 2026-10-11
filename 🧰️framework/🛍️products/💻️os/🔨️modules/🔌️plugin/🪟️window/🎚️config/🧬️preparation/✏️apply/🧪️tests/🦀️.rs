//! 🧪️ The field-set apply edit conserves physical custody for Copy and heap-owning window states through every partial frontier, and its inverse row restores the base exactly.
use super::{WindowConfigApplyEdit,WindowConfigApplyMutation};
use crate::store;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
use semio_framework_value::{ValueError,ValueRefusalKind,retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::{RetainedClone,RetainedCloneCursor,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneSource,RetainedCloneStep}};
use serde_json::{Value,json};
use std::panic::{AssertUnwindSafe,catch_unwind};
use store::snapshot_clone_preparation::{RetainedCloneEdit,RetainedCloneEditCursor,RetainedCloneEditStep};

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
struct Camera { x: f64, y: f64, zoom: f64 }

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
struct CopyState { camera: Camera, eye: [f64; 3], selected: usize }

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
struct SetCamera { camera: Camera }
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
struct SetEye { eye: [f64; 3] }
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
struct SetSelected { selected: usize }

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[serde(tag = "kind", rename_all = "kebab-case")]
enum CopyMutation { SetCamera(SetCamera), SetEye(SetEye), SetSelected(SetSelected) }

impl WindowConfigApplyMutation<CopyState> for CopyMutation {
    fn exchange(self, post: &mut CopyState) -> Result<Self, (ValueError, Self)> {
        Ok(match self {
            Self::SetCamera(SetCamera { camera }) => Self::SetCamera(SetCamera { camera: std::mem::replace(&mut post.camera, camera) }),
            Self::SetEye(SetEye { eye }) => Self::SetEye(SetEye { eye: std::mem::replace(&mut post.eye, eye) }),
            Self::SetSelected(SetSelected { selected }) => Self::SetSelected(SetSelected { selected: std::mem::replace(&mut post.selected, selected) }),
        })
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[serde(rename_all = "camelCase")]
struct TextState { active_register: String, label: Option<String>, ids: Vec<String>, zoom: f64 }

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
struct SetActiveRegister { active_register: String }
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
struct SetLabel { label: Option<String> }
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
struct SetIds { ids: Vec<String> }
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
struct SetZoom { zoom: f64 }

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(tag = "kind", rename_all = "kebab-case")]
#[value(tag = "kind", rename_all = "kebab-case")]
enum TextMutation { SetActiveRegister(SetActiveRegister), SetLabel(SetLabel), SetIds(SetIds), SetZoom(SetZoom) }

impl WindowConfigApplyMutation<TextState> for TextMutation {
    fn exchange(self, post: &mut TextState) -> Result<Self, (ValueError, Self)> {
        Ok(match self {
            Self::SetActiveRegister(SetActiveRegister { active_register }) => Self::SetActiveRegister(SetActiveRegister { active_register: std::mem::replace(&mut post.active_register, active_register) }),
            Self::SetLabel(SetLabel { label }) => Self::SetLabel(SetLabel { label: std::mem::replace(&mut post.label, label) }),
            Self::SetIds(SetIds { ids }) => Self::SetIds(SetIds { ids: std::mem::replace(&mut post.ids, ids) }),
            Self::SetZoom(SetZoom { zoom }) if !zoom.is_finite() => return Err((ValueError::literal(ValueRefusalKind::InvalidValue, "zoom must be finite"), Self::SetZoom(SetZoom { zoom }))),
            Self::SetZoom(SetZoom { zoom }) => Self::SetZoom(SetZoom { zoom: std::mem::replace(&mut post.zoom, zoom) }),
        })
    }
    fn admissible(&self) -> bool { !matches!(self, Self::SetZoom(SetZoom { zoom }) if !zoom.is_finite()) }
    fn payload_bytes(&self) -> usize {
        match self {
            Self::SetActiveRegister(SetActiveRegister { active_register }) => active_register.len(),
            Self::SetLabel(SetLabel { label }) => label.as_ref().map_or(0, String::len),
            Self::SetIds(SetIds { ids }) => ids.iter().map(String::len).sum(),
            Self::SetZoom(_) => 0,
        }
    }
}

const SETUP: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1 << 16, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 64 };
const IDLE_TURNS: usize = 8;

struct Stall;
fn stall() -> ! { std::panic::panic_any(Stall) }
fn quiet_stalls() {
    static HOOK: std::sync::Once = std::sync::Once::new();
    HOOK.call_once(|| {
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| { if !info.payload().is::<Stall>() { previous(info) } }));
    });
}
fn limited<T>(result: Result<T, ValueError>) -> T {
    result.unwrap_or_else(|error| if matches!(error.kind, ValueRefusalKind::WorkLimit | ValueRefusalKind::OwnershipLimit | ValueRefusalKind::DepthLimit) { stall() } else { panic!("unexpected refusal {error}") })
}

#[derive(Default)]
struct Ledger { born: usize, released: usize }
impl Ledger {
    fn record(&mut self, heap: semio_framework_trace::HeapAllocationObservation) { self.born += heap.requested_bytes; self.released += heap.released_bytes; }
    fn watch<T>(&mut self, grant: RetainedCloneGrant, operation: impl FnOnce() -> (T, RetainedCloneProgress)) -> (T, RetainedCloneProgress) {
        let ((value, progress), heap) = observe(operation);
        assert!(progress.fits(grant), "receipt {progress:?} exceeds grant {grant:?}");
        assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes), "allocation accounting disagrees with receipt {progress:?}");
        self.record(heap);
        (value, progress)
    }
}

#[derive(Default)]
struct Idle(usize);
impl Idle {
    fn note(&mut self, progress: RetainedCloneProgress) {
        if progress != RetainedCloneProgress::default() { self.0 = 0; return; }
        self.0 += 1;
        if self.0 >= IDLE_TURNS { stall() }
    }
}

fn admit<T: RetireOwned + Sync>(value: T) -> (RetainedCloneSource<T, T>, RetainedCloneProgress) {
    RetainedCloneSource::admit_owned(value, (), SETUP).map(|(source, progress)| (source, progress)).unwrap_or_else(|(error, _, _)| panic!("source admission {error}"))
}

fn exact(copy: usize, capacity: usize, release: usize, depth: usize) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: depth }
}

fn drain_source<T: RetireOwned + Sync>(ledger: &mut Ledger, source: &mut RetainedCloneSource<T, T>) {
    for _ in 0..4096 {
        if source.terminal_is_empty() { return; }
        let copy = source.next_close_copy_byte_demand().unwrap();
        let grant = exact(copy, source.next_close_capacity_byte_demand(copy).unwrap(), source.next_close_release_byte_demand().unwrap(), source.next_close_depth_demand().unwrap());
        ledger.watch(grant, || { let step = source.close_step(grant).unwrap(); (step, step.progress()) });
    }
    panic!("source did not close");
}

fn drain_clone_cursor<T: RetainedClone>(ledger: &mut Ledger, cursor: &mut T::Cursor) {
    for _ in 0..4096 {
        if cursor.terminal_is_empty() { return; }
        let copy = cursor.next_close_copy_byte_demand().unwrap();
        let grant = exact(copy, cursor.next_close_capacity_byte_demand(copy).unwrap(), cursor.next_close_release_byte_demand().unwrap(), cursor.next_close_depth_demand().unwrap().max(1));
        ledger.watch(grant, || { let step = cursor.close_step(grant).unwrap(); (step, step.progress()) });
    }
    panic!("clone cursor did not close");
}

fn clone_owner<S: RetainedClone>(ledger: &mut Ledger, source: &RetainedCloneSource<S, S>, grant: RetainedCloneGrant) -> S {
    let mut cursor = S::retained_clone_cursor();
    let mut idle = Idle::default();
    let owned = loop {
        let (step, progress) = ledger.watch(grant, || { let step = limited(cursor.advance(source.borrow(), grant)); (step, step.progress()) });
        if matches!(step, RetainedCloneStep::Complete(_)) { break cursor.take().expect("clone completes with its owner"); }
        idle.note(progress);
    };
    let _ = cursor.begin_close();
    drain_clone_cursor::<S>(ledger, &mut cursor);
    assert_eq!(observe(|| drop(cursor)).1.released_bytes, 0);
    owned
}

#[derive(Debug)]
struct Run { post: Value, inverse: Value, turns: usize }

fn from_json<M: serde::de::DeserializeOwned>(json: &Value) -> impl Fn() -> M + '_ { move || serde_json::from_value(json.clone()).unwrap() }

fn drive<S, M>(base_json: &Value, build: &dyn Fn() -> M, grant: RetainedCloneGrant, cancel_at: Option<usize>) -> Result<Option<Run>, ValueError>
where S: RetainedClone + serde::Serialize + serde::de::DeserializeOwned, M: WindowConfigApplyMutation<S> + serde::Serialize {
    let mut ledger = Ledger::default();
    let ((base, mutation), heap) = observe(|| (serde_json::from_value::<S>(base_json.clone()).unwrap(), build()));
    let held = heap.requested_bytes - heap.released_bytes;
    let (mut base_source, _) = ledger.watch(SETUP, || admit(base));
    let (mut mutation_source, _) = ledger.watch(SETUP, || admit(mutation));
    let mut post = clone_owner(&mut ledger, &base_source, grant);
    let edit = WindowConfigApplyEdit::<S, M>::new();
    let (mut cursor, _) = ledger.watch(grant, || limited(edit.begin(grant)));
    let (mut turns, mut idle, mut complete, mut failure) = (0, Idle::default(), false, None);
    loop {
        if cancel_at == Some(turns) { cursor.cancel(); break; }
        let before = serde_json::to_value(&post).unwrap();
        for denied in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_depth: 1, ..grant }] {
            let (result, heap) = observe(|| cursor.advance(base_source.borrow(), &mut post, mutation_source.borrow(), denied));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(result.unwrap(), RetainedCloneEditStep::Progress(Default::default()));
            assert_eq!(serde_json::to_value(&post).unwrap(), before);
        }
        let (result, heap) = observe(|| cursor.advance(base_source.borrow(), &mut post, mutation_source.borrow(), grant));
        ledger.record(heap);
        match result {
            Ok(step) => {
                let progress = step.progress();
                assert!(progress.fits(grant), "receipt {progress:?} exceeds grant {grant:?}");
                assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes), "allocation accounting disagrees with receipt {progress:?}");
                turns += 1;
                if matches!(step, RetainedCloneEditStep::Complete(_)) { complete = true; break; }
                idle.note(progress);
            }
            Err(error) if matches!(error.kind, ValueRefusalKind::WorkLimit | ValueRefusalKind::OwnershipLimit | ValueRefusalKind::DepthLimit) => stall(),
            Err(error) => { failure = Some(error); break; }
        }
    }
    let rows = if complete { cursor.take_inverse() } else { assert!(cursor.take_inverse().is_none()); None };
    assert!(cursor.take_inverse().is_none());
    let run = complete.then(|| Run { post: serde_json::to_value(&post).unwrap(), inverse: Value::Array(rows.iter().flatten().map(|row| serde_json::to_value(row).unwrap()).collect()), turns });
    if !complete {
        let (result, heap) = observe(|| cursor.advance(base_source.borrow(), &mut post, mutation_source.borrow(), grant));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(result.unwrap(), RetainedCloneEditStep::Progress(Default::default()));
    }
    let _ = cursor.begin_close();
    for _ in 0..4096 {
        if cursor.terminal_is_empty() { break; }
        let copy = cursor.next_close_copy_byte_demand().unwrap();
        let quote = exact(copy, cursor.next_close_capacity_byte_demand(copy).unwrap(), cursor.next_close_release_byte_demand().unwrap(), cursor.next_close_depth_demand().unwrap());
        if quote.maximum_depth > 1 {
            let (result, heap) = observe(|| cursor.close_step(RetainedCloneGrant { maximum_depth: quote.maximum_depth - 1, ..quote }));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert!(result.is_err());
        }
        ledger.watch(quote, || { let step = cursor.close_step(quote).unwrap(); (step, step.progress()) });
    }
    assert!(cursor.terminal_is_empty());
    assert_eq!(observe(|| drop(cursor)).1.released_bytes, 0);
    drain_source(&mut ledger, &mut base_source);
    drain_source(&mut ledger, &mut mutation_source);
    let mut remaining = ControlledRetirement::new((post, rows)).unwrap_or_else(|(error, _)| panic!("post and inverse custody {error}"));
    for _ in 0..4096 {
        if remaining.terminal_is_empty() { break; }
        let copy = remaining.next_copy_byte_demand().unwrap();
        let quote = exact(copy, remaining.next_capacity_byte_demand(copy).unwrap(), remaining.next_release_byte_demand().unwrap(), remaining.next_depth_demand().unwrap());
        ledger.watch(quote, || { let step = remaining.step(quote).unwrap(); (step, step.progress()) });
    }
    assert!(remaining.terminal_is_empty());
    assert_eq!(ledger.released, held + ledger.born, "every byte born or held is released through admitted turns");
    assert_eq!(observe(|| drop((base_source, mutation_source, remaining))).1.released_bytes, 0);
    match failure { Some(error) => Err(error), None => Ok(run) }
}

fn floor<S, M>(base: &Value, mutation: &Value, wide: RetainedCloneGrant) -> usize
where S: RetainedClone + serde::Serialize + serde::de::DeserializeOwned, M: WindowConfigApplyMutation<S> + serde::Serialize + serde::de::DeserializeOwned {
    quiet_stalls();
    let mut copy = 1usize;
    loop {
        let grant = RetainedCloneGrant { maximum_copy_bytes: copy, ..wide };
        if matches!(catch_unwind(AssertUnwindSafe(|| drive::<S, M>(base, &from_json(mutation), grant, None))), Ok(Ok(Some(_)))) { return copy; }
        copy *= 2;
        assert!(copy <= 1 << 20, "no copy grant completes the case");
    }
}

fn fixture() -> Value { serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap() }

const WIDE: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 64 };

fn exercise<S, M>(case: &Value, policy: &Value)
where S: RetainedClone + serde::Serialize + serde::de::DeserializeOwned, M: WindowConfigApplyMutation<S> + serde::Serialize + serde::de::DeserializeOwned {
    let (base, mutation) = (&case["base"], &case["mutation"]);
    let floor = floor::<S, M>(base, mutation, WIDE);
    let mut turns = Vec::new();
    for named in policy["grants"].as_array().unwrap() {
        let grant = RetainedCloneGrant { maximum_items: named["items"].as_u64().unwrap() as usize, maximum_copy_bytes: floor + named["copyExtra"].as_u64().unwrap() as usize, ..WIDE };
        let run = drive::<S, M>(base, &from_json(mutation), grant, None).unwrap().expect("uncancelled run completes");
        assert_eq!(run.post, case["post"], "post of {} under {}", case["name"], named["name"]);
        assert_eq!(run.inverse, json!([case["inverse"]]), "inverse of {} under {}", case["name"], named["name"]);
        let restored = drive::<S, M>(&run.post, &from_json(&run.inverse[0]), grant, None).unwrap().expect("inverse run completes");
        assert_eq!(&restored.post, base, "inverse restores the base exactly for {}", case["name"]);
        assert_eq!(restored.inverse, json!([mutation]), "double inverse restores the forward row for {}", case["name"]);
        for cancel_at in policy["cancelAt"].as_array().unwrap().iter().map(|turn| turn.as_u64().unwrap() as usize) {
            let outcome = drive::<S, M>(base, &from_json(mutation), grant, Some(cancel_at)).unwrap();
            assert_eq!(outcome.is_some(), cancel_at >= run.turns, "cancel at {cancel_at} of {} turns", run.turns);
        }
        turns.push((named["name"].as_str().unwrap().to_owned(), run.turns));
    }
    eprintln!("[DEBUG] window config apply {} floor={floor} turns={turns:?}; clone, exchange, inverse and partial custody conserved", case["name"]);
}

#[test]
fn window_config_apply_conserves_custody_for_copy_state_field_sets() {
    let policy = fixture();
    for case in policy["cases"].as_array().unwrap().iter().filter(|case| case["state"] == "copy") { exercise::<CopyState, CopyMutation>(case, &policy); }
}

#[test]
fn window_config_apply_conserves_custody_for_heap_owning_state_field_sets() {
    let policy = fixture();
    for case in policy["cases"].as_array().unwrap().iter().filter(|case| case["state"] == "text" && case.get("requiresOptionNone").is_none()) { exercise::<TextState, TextMutation>(case, &policy); }
}

#[test]
fn window_config_apply_conserves_custody_for_absent_optional_fields() {
    let policy = fixture();
    for case in policy["cases"].as_array().unwrap().iter().filter(|case| case.get("requiresOptionNone").is_some()) { exercise::<TextState, TextMutation>(case, &policy); }
}

#[test]
fn window_config_apply_pages_long_text_across_small_grants() {
    let long = "é中🐚a".repeat(4000);
    let base = json!({ "activeRegister": "main", "label": "l", "ids": [], "zoom": 1.5 });
    let mutation = json!({ "kind": "set-active-register", "activeRegister": long });
    let floor = floor::<TextState, TextMutation>(&base, &mutation, WIDE);
    let tight = drive::<TextState, TextMutation>(&base, &from_json(&mutation), RetainedCloneGrant { maximum_copy_bytes: floor, ..WIDE }, None).unwrap().unwrap();
    let large = drive::<TextState, TextMutation>(&base, &from_json(&mutation), RetainedCloneGrant { maximum_copy_bytes: 1 << 20, ..WIDE }, None).unwrap().unwrap();
    assert_eq!(tight.post, json!({ "activeRegister": mutation["activeRegister"], "label": "l", "ids": [], "zoom": 1.5 }));
    assert_eq!(tight.post, large.post);
    assert_eq!(tight.inverse, large.inverse);
    assert_eq!(tight.inverse, json!([{ "kind": "set-active-register", "activeRegister": "main" }]));
    assert!(tight.turns > large.turns, "a small copy grant pages the payload over more turns ({} vs {})", tight.turns, large.turns);
    eprintln!("[DEBUG] window config apply long text floor={floor} tightTurns={} largeTurns={}", tight.turns, large.turns);
}

#[test]
fn window_config_apply_refuses_invalid_lane_payload_and_exchange() {
    let edit = WindowConfigApplyEdit::<TextState, TextMutation>::new();
    let finite = TextMutation::SetZoom(SetZoom { zoom: 2.0 });
    let infinite = TextMutation::SetZoom(SetZoom { zoom: f64::INFINITY });
    assert!(edit.preflight(&finite, store::HistoryLane::Document).unwrap().is_admissible());
    assert!(edit.preflight(&finite, store::HistoryLane::Interaction).is_err());
    assert!(edit.preflight(&infinite, store::HistoryLane::Document).is_err());
    let oversize = TextMutation::SetActiveRegister(SetActiveRegister { active_register: "a".repeat(store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES) });
    assert!(edit.preflight(&oversize, store::HistoryLane::Document).is_err());
    let base = json!({ "activeRegister": "main", "label": "keep", "ids": ["a"], "zoom": 1.5 });
    let grant = RetainedCloneGrant { maximum_copy_bytes: 1 << 16, ..WIDE };
    let refused = drive::<TextState, TextMutation>(&base, &|| infinite.clone(), grant, None).unwrap_err();
    assert_eq!(refused.kind, ValueRefusalKind::InvalidValue);
    eprintln!("[DEBUG] window config apply exchange refusal keeps the payload in custody and retires it: {}", refused.message);
}

#[test]
fn window_config_apply_factory_quotes_the_store_preparation() {
    use store::ArtifactStoreOneItemPreparationFactory;
    let factory = store::snapshot_clone_preparation::config_apply_preparation_factory::<TextState, TextMutation>();
    let mutation = TextMutation::SetZoom(SetZoom { zoom: 2.0 });
    assert!(factory.preflight(&mutation, store::HistoryLane::Document).unwrap().is_admissible());
    assert!(factory.preflight(&mutation, store::HistoryLane::Interaction).is_err());
    let demand = factory.begin_demand(&mutation, store::HistoryLane::Document).unwrap();
    assert_eq!((demand.capacity_bytes > 0, demand.depth), (true, 1));
}
