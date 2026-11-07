//! 🎟️ The single admitted-input slot of the actor world: admission order, charge and consumption, free of the WIT bindings so native laws pin it against the neutral actor-input fixtures.
use semio_plugin_abi::actor_input_wire::{ActorInputWire, TurnShape};
use semio_framework_value::{RetainedCloneProgress, ValueError, ValueRefusalKind};
use std::cell::{Cell, RefCell};

/// 🧭️ What happened to the wire the host offered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdmissionDisposition {
    Borrowed,
    Retained,
    Returned,
}

/// 📏️ The retained policy extents an admission is checked against.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmissionPolicy {
    pub maximum_items: u64,
    pub maximum_copy_bytes: u64,
    pub maximum_capacity_bytes: u64,
}

/// 🧾️ The complete answer of one admission: a refused wire always travels back to the host.
#[derive(Debug, PartialEq, Eq)]
pub struct InputAdmission {
    pub ready: bool,
    pub spent: RetainedCloneProgress,
    pub disposition: AdmissionDisposition,
    pub returned_wire: Option<Vec<u8>>,
    pub refusal: Option<ValueRefusalKind>,
}

fn returned(bytes: Vec<u8>, refusal: ValueRefusalKind) -> InputAdmission {
    InputAdmission { ready: false, spent: Default::default(), disposition: AdmissionDisposition::Returned, returned_wire: Some(bytes), refusal: Some(refusal) }
}

/// 🔒️ Another export still holds the slot or its outputs.
pub fn busy() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvariantViolated, "original actor export remains checked out")
}

fn wrong_operation() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "admitted actor input belongs to another operation")
}

fn outside_schema() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "actor input wire is outside its schema")
}

/// 🎟️ Retains one wire in the slot, charging exactly its bytes; refusals are ordered cancelled, policy, occupied-or-held. No wire admits nothing and charges nothing.
pub fn admit(slot: &RefCell<Option<Vec<u8>>>, wire: Option<Vec<u8>>, policy: AdmissionPolicy, cancelled: bool, outputs_held: bool) -> InputAdmission {
    let Some(bytes) = wire else {
        return InputAdmission { ready: true, spent: Default::default(), disposition: AdmissionDisposition::Borrowed, returned_wire: None, refusal: None };
    };
    let length = bytes.len() as u64;
    if cancelled {
        return returned(bytes, ValueRefusalKind::Canceled);
    }
    if policy.maximum_items < 1 || length > policy.maximum_copy_bytes || length > policy.maximum_capacity_bytes {
        return returned(bytes, ValueRefusalKind::OwnershipLimit);
    }
    if slot.try_borrow().map_or(true, |held| held.is_some()) || outputs_held {
        return returned(bytes, ValueRefusalKind::InvariantViolated);
    }
    *slot.borrow_mut() = Some(bytes);
    InputAdmission {
        ready: true,
        spent: RetainedCloneProgress { copied_items: 1, copied_bytes: length as usize, retained_capacity_bytes: length as usize, ..Default::default() },
        disposition: AdmissionDisposition::Retained,
        returned_wire: None,
        refusal: None,
    }
}

/// 📥️ Takes the slot if a wire was admitted; an empty slot is a wire-free turn, a wire outside its schema is refused.
pub fn take_optional(slot: &RefCell<Option<Vec<u8>>>) -> Result<Option<ActorInputWire>, ValueError> {
    let bytes = slot.try_borrow_mut().map(|mut held| held.take()).map_err(|_| busy())?;
    bytes.map(|bytes| ActorInputWire::decode(&bytes).map_err(|_| outside_schema())).transpose()
}

/// 📥️ Takes the single admitted wire; an empty slot or a wire outside its schema is refused.
pub fn take(slot: &RefCell<Option<Vec<u8>>>) -> Result<ActorInputWire, ValueError> {
    take_optional(slot)?.ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "no actor input is admitted"))
}

/// 🌀️ A poll consumes its own wire and remembers the shape; with an empty slot it reuses the remembered shape, and only the first poll must admit one.
pub fn take_poll(slot: &RefCell<Option<Vec<u8>>>, shape: &Cell<Option<TurnShape>>) -> Result<(Vec<semio_framework_value::DslValue>, TurnShape), ValueError> {
    match take_optional(slot)? {
        Some(ActorInputWire::Poll { events, shape: admitted }) => {
            shape.set(Some(admitted));
            Ok((events, admitted))
        }
        Some(_) => Err(wrong_operation()),
        None => shape.get().map(|held| (Vec::new(), held)).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "the first actor poll must admit its turn shape")),
    }
}

#[cfg(test)]
mod laws {
    use super::*;

    fn corpus() -> serde_json::Value {
        serde_json::from_str(include_str!("../../../🧬️schema/🎟️actor-input/🧫️fixtures/🔣️.json")).unwrap()
    }

    fn pinned(case: &serde_json::Value) -> Vec<u8> {
        let hex = case["packHex"].as_str().unwrap();
        (0..hex.len()).step_by(2).map(|at| u8::from_str_radix(&hex[at..at + 2], 16).unwrap()).collect()
    }

    fn roomy() -> AdmissionPolicy {
        AdmissionPolicy { maximum_items: 4, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20 }
    }

    fn case_of<'a>(corpus: &'a serde_json::Value, operation: &str) -> &'a serde_json::Value {
        corpus["valid"].as_array().unwrap().iter().find(|case| case["operation"] == operation).unwrap()
    }

    #[test]
    fn every_fixture_wire_is_retained_charged_by_its_bytes_and_consumed_once() {
        for case in corpus()["valid"].as_array().unwrap() {
            let bytes = pinned(case);
            let slot = RefCell::new(None);
            let admission = admit(&slot, Some(bytes.clone()), roomy(), false, false);
            assert_eq!((admission.ready, admission.disposition, admission.refusal), (true, AdmissionDisposition::Retained, None));
            assert_eq!(admission.spent, RetainedCloneProgress { copied_items: 1, copied_bytes: bytes.len(), retained_capacity_bytes: bytes.len(), ..Default::default() });
            let wire = take(&slot).unwrap();
            assert_eq!(wire.operation(), case["operation"].as_str().unwrap());
            assert!(take(&slot).is_err(), "the slot is consumed exactly once");
        }
    }

    #[test]
    fn refusals_follow_cancelled_then_policy_then_occupied_and_return_the_wire() {
        let bytes = pinned(case_of(&corpus(), "poll"));
        let slot = RefCell::new(None);
        let cancelled = admit(&slot, Some(bytes.clone()), AdmissionPolicy { maximum_items: 0, ..roomy() }, true, true);
        assert_eq!((cancelled.refusal, cancelled.disposition, cancelled.returned_wire.as_deref()), (Some(ValueRefusalKind::Canceled), AdmissionDisposition::Returned, Some(bytes.as_slice())));
        for policy in [AdmissionPolicy { maximum_items: 0, ..roomy() }, AdmissionPolicy { maximum_copy_bytes: bytes.len() as u64 - 1, ..roomy() }, AdmissionPolicy { maximum_capacity_bytes: bytes.len() as u64 - 1, ..roomy() }] {
            let refused = admit(&slot, Some(bytes.clone()), policy, false, true);
            assert_eq!((refused.refusal, refused.returned_wire.as_deref()), (Some(ValueRefusalKind::OwnershipLimit), Some(bytes.as_slice())));
            assert!(slot.borrow().is_none(), "a refusal never occupies the slot");
        }
        let held = admit(&slot, Some(bytes.clone()), roomy(), false, true);
        assert_eq!((held.refusal, held.returned_wire.as_deref()), (Some(ValueRefusalKind::InvariantViolated), Some(bytes.as_slice())));
        assert_eq!(admit(&slot, Some(bytes.clone()), roomy(), false, false).disposition, AdmissionDisposition::Retained);
        let occupied = admit(&slot, Some(bytes.clone()), roomy(), false, false);
        assert_eq!((occupied.refusal, occupied.returned_wire.as_deref()), (Some(ValueRefusalKind::InvariantViolated), Some(bytes.as_slice())));
    }

    #[test]
    fn no_wire_is_borrowed_for_free_even_when_cancelled() {
        let slot = RefCell::new(None);
        let admission = admit(&slot, None, AdmissionPolicy { maximum_items: 0, maximum_copy_bytes: 0, maximum_capacity_bytes: 0 }, true, true);
        assert_eq!(admission, InputAdmission { ready: true, spent: Default::default(), disposition: AdmissionDisposition::Borrowed, returned_wire: None, refusal: None });
    }

    #[test]
    fn a_poll_consumes_its_slot_and_an_empty_poll_is_wire_free_after_the_first_shape() {
        let corpus = corpus();
        let poll = case_of(&corpus, "poll");
        let slot = RefCell::new(None);
        let shape = Cell::new(None);
        assert!(take_poll(&slot, &shape).is_err(), "the first poll must admit its turn shape");
        admit(&slot, Some(pinned(poll)), roomy(), false, false);
        let (events, admitted) = take_poll(&slot, &shape).unwrap();
        assert!(events.is_empty());
        assert_eq!((admitted.max_effects, admitted.max_patch_bytes, admitted.max_frames), (8, 4096, 2));
        let (events, repeated) = take_poll(&slot, &shape).unwrap();
        assert!(events.is_empty() && repeated == admitted, "an empty slot reuses the remembered shape without a wire");
    }

    #[test]
    fn only_a_poll_may_consume_a_poll_slot_and_only_its_own_operation_consumes_the_rest() {
        let corpus = corpus();
        for case in corpus["valid"].as_array().unwrap().iter().filter(|case| case["operation"] != "poll") {
            let slot = RefCell::new(None);
            let shape = Cell::new(None);
            admit(&slot, Some(pinned(case)), roomy(), false, false);
            assert!(take_poll(&slot, &shape).is_err(), "{} must not be consumed by a poll", case["operation"]);
            assert!(slot.borrow().is_none(), "a refused consumer still empties the slot");
        }
    }

    #[test]
    fn checkpoint_restore_wire_round_trips_through_the_slot_byte_for_byte() {
        let corpus = corpus();
        let restore = case_of(&corpus, "restore");
        let slot = RefCell::new(None);
        admit(&slot, Some(pinned(restore)), roomy(), false, false);
        let wire = take(&slot).unwrap();
        assert_eq!(wire.operation(), "restore");
        assert!(wire.to_value() == semio_framework_value::DslValue::from(&restore["wire"]));
        let again = RefCell::new(None);
        admit(&again, Some(wire.encode()), roomy(), false, false);
        assert_eq!(take(&again).unwrap().encode(), pinned(restore));
    }

    #[test]
    fn an_invalid_fixture_wire_is_refused_at_consumption() {
        for case in corpus()["invalid"].as_array().unwrap().iter().filter(|case| case.get("stage").is_none()) {
            let value = semio_framework_value::DslValue::from(&case["wire"]);
            let slot = RefCell::new(None);
            admit(&slot, Some(semio_os_kernel::pack_rt::encode_wire_value(&value)), roomy(), false, false);
            assert!(take(&slot).is_err(), "{}", case["reason"]);
        }
    }
}
