//! 🧪️ Nested byte accounting, shared ownership, cold decoder errors, and explicit terminal guards.

use super::*;
use std::mem::size_of;

#[test]
fn recursive_dictionary_owns_typed_retirement_under_independent_grants() {
    use semio_framework_value::{retirement::{RetireOwned, controlled::ControlledRetirement}, retained_clone::{RetainedCloneGrant, RetainedCloneStep}};
    use super::super::registry::tests::observe_ownership;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎮️typed-owners/🔣️.json")).unwrap();
    assert!(Dictionary::controlled_retirement_supported());
    assert!(Value::controlled_retirement_supported());
    assert!(Atom::controlled_retirement_supported());
    for row in fixture["cases"].as_array().unwrap() {
        for copy in fixture["grants"].as_array().unwrap() {
            let ((source,pointer),original_born,original_freed)=observe_ownership(||{
                let mut text=String::with_capacity(row["reservedCapacity"].as_u64().unwrap() as usize);
                text.push_str(row["text"].as_str().unwrap());let pointer=text.as_ptr();
                (Dictionary::new().insert("nested",Value::Dictionary(Dictionary::new().insert("value",Value::Atom(Atom::String(text))))),pointer)
            });
            assert_eq!(source.get("nested").unwrap().as_dictionary().unwrap().get("value").unwrap().as_atom().unwrap().as_str().unwrap().as_ptr(),pointer);
            assert_eq!(serde_json::to_value(&source).unwrap(),serde_json::json!({"nested":{"value":row["text"]}}));
            let mut owner=ControlledRetirement::new(source).map_err(|(error,_)|error).unwrap();
            let mut copied=0;let mut released=0;let mut born=0;
            for turn in 0..100000 {
                if owner.terminal_is_empty(){break;}
                let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy.as_u64().unwrap() as usize,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy.as_u64().unwrap() as usize).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
                let (inert,a,r)=observe_ownership(||owner.step(RetainedCloneGrant {maximum_items:0,..grant}).unwrap());assert_eq!(inert.progress().copied_items,0);assert_eq!((a,r),(0,0));
                if grant.maximum_capacity_bytes!=0 {let (denied,a,r)=observe_ownership(||owner.step(RetainedCloneGrant {maximum_capacity_bytes:grant.maximum_capacity_bytes-1,..grant}).unwrap());assert_eq!(denied.progress().copied_items,0);assert_eq!((a,r),(0,0));}
                if grant.maximum_release_bytes!=0 && owner.next_copy_byte_demand()==0 {let (denied,a,r)=observe_ownership(||owner.step(RetainedCloneGrant {maximum_release_bytes:grant.maximum_release_bytes-1,..grant}).unwrap());assert_eq!(denied.progress().copied_items,0);assert_eq!((a,r),(0,0));}
                let (step,a,r)=observe_ownership(||owner.step(grant).unwrap());let progress=step.progress();assert_eq!(progress.retained_capacity_bytes,a);assert_eq!(progress.released_bytes,r);assert!(progress.copied_items<=1);assert!(progress.copied_bytes<=grant.maximum_copy_bytes);assert!(progress.retained_capacity_bytes<=grant.maximum_capacity_bytes);assert!(progress.released_bytes<=grant.maximum_release_bytes);copied+=progress.copied_bytes;released+=progress.released_bytes;born+=progress.retained_capacity_bytes;
                assert!(progress.copied_items!=0 || matches!(step,RetainedCloneStep::Complete(_)),"exact Neural owner grant blocked on turn {turn}");
            }
            assert!(owner.terminal_is_empty());assert_eq!(copied,"nestedvalue".len()+row["text"].as_str().unwrap().len());assert_eq!(released,original_born-original_freed+born);
            eprintln!("[DEBUG] Recursive Neural original-pointer typed owner copied={copied} born={born} physical={released}");
        }
    }
}

#[test]
fn finite_schema_operator_fields_preserve_typed_work_and_physical_custody() {
    use semio_framework_value::{retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::RetainedCloneGrant};
    use super::super::registry::tests::observe_ownership;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧬️finite-fields/🔣️.json")).unwrap();
    fn close<T:RetireOwned>(value:T,original:usize,copy:usize,expected:usize) {
        let mut owner=ControlledRetirement::new(value).map_err(|(error,_)|error).unwrap();let mut births=0;let mut released=0;let mut copied=0;
        for turn in 0..1000000 {
            if owner.terminal_is_empty(){break;}
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
            let (zero,a,r)=observe_ownership(||owner.step(RetainedCloneGrant {maximum_items:0,..grant}).unwrap());assert_eq!(zero.progress().copied_items,0);assert_eq!((a,r),(0,0));
            let (step,a,r)=observe_ownership(||owner.step(grant).unwrap());let progress=step.progress();assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(a,r));assert!(progress.copied_items<=1);assert!(progress.copied_bytes<=copy);assert!(a<=grant.maximum_capacity_bytes);assert!(r<=grant.maximum_release_bytes);births+=a;released+=r;copied+=progress.copied_bytes;assert!(progress.copied_items!=0,"exact finite Neural field grant blocked on turn {turn}");
        }
        assert!(owner.terminal_is_empty());assert_eq!(copied,expected);assert_eq!(released,original+births);
        eprintln!("[DEBUG] Actual Neural finite fields copy={copy} work={copied} original={original} scaffolds={births} physical={released}");
    }
    for copy in fixture["copyGrants"].as_array().unwrap() {
        let (schema,a,r)=observe_ownership(||serde_json::from_value::<Schema>(fixture["schema"].clone()).unwrap());assert_eq!(serde_json::to_value(&schema).unwrap(),fixture["schema"]);close(schema,a-r,copy.as_u64().unwrap() as usize,fixture["expected"]["schemaStringBytes"].as_u64().unwrap() as usize);
        let (operator,a,r)=observe_ownership(||serde_json::from_value::<OperatorInfo>(fixture["operator"].clone()).unwrap());assert_eq!(serde_json::to_value(&operator).unwrap(),fixture["operator"]);close(operator,a-r,copy.as_u64().unwrap() as usize,fixture["expected"]["operatorStringBytes"].as_u64().unwrap() as usize);
    }
}

#[test]
fn original_typed_owner_demands_propagate_without_false_fixed_page_progress() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📏️owned-demand/🔣️.json")).unwrap();
    let mut source = String::with_capacity(fixture["reservedCapacity"].as_u64().unwrap() as usize);
    source.push_str(fixture["text"].as_str().unwrap());
    assert_eq!(serde_json::from_str::<String>(&serde_json::to_string(&source).unwrap()).unwrap(), source);
    let mut owner = ValueRetirement::default();
    owner.push_owned(source);
    let first_demand = owner.next_close_byte_demand().unwrap();
    assert!(first_demand > fixture["logicalPage"].as_u64().unwrap() as usize);
    assert_eq!(owner.close_step(0, first_demand), ValueRetirementStep::Blocked);
    assert_eq!(owner.close_step(1, first_demand - 1), ValueRetirementStep::Blocked);
    assert_eq!(owner.next_close_byte_demand().unwrap(), first_demand);
    let mut released = 0;
    for turn in 0..fixture["maximumTurns"].as_u64().unwrap() {
        if owner.terminal_is_empty() { break; }
        let grant = owner.next_step_byte_demand(fixture["logicalPage"].as_u64().unwrap() as usize).unwrap();
        match owner.close_step(1, grant) {
            ValueRetirementStep::Pending { released_items, released_bytes } => { assert!(released_items <= 1 && released_bytes <= grant); released += released_bytes; },
            ValueRetirementStep::Complete => {},
            ValueRetirementStep::Blocked => panic!("exact typed owner grant blocked on turn {turn} with demand {grant}"),
        }
    }
    assert!(owner.terminal_is_empty());
    assert!(released >= fixture["reservedCapacity"].as_u64().unwrap() as usize);
    eprintln!("[DEBUG] Original Neural typed owner borrowed exact demands, retained undersized grant, physical release={released}");
}

//#region 🔣️FixtureLaws
/// 🎟️ An emptied element-vector backing is accounted PHYSICALLY by `allocated_bytes` — the number a
/// terminal-empty proof and a frontier reservation need — and freed in one turn under any positive
/// grant, reporting ZERO payload bytes. `capacity * size_of::<T>()` is machine-width dependent
/// (`size_of::<String>()` is 24 native and 12 on `wasm32`), so it can never be part of the released
/// payload total the language-agnostic drawdown fixture pins
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
fn exact_backing(owner: Owner, allocated: usize) {
    assert!(allocated > 4096);
    let mut retirement = ValueRetirement::default();
    retirement.owners.push_back(owner);
    assert_eq!(retirement.allocated_bytes(), allocated);
    assert_eq!(retirement.next_close_byte_demand().unwrap(), 1);
    assert_eq!(retirement.close_step(0, allocated), ValueRetirementStep::Blocked);
    assert_eq!(retirement.close_step(1, 0), ValueRetirementStep::Blocked);
    assert_eq!(retirement.allocated_bytes(), allocated);
    assert_eq!(retirement.close_step(1, 1), ValueRetirementStep::Pending { released_items: 1, released_bytes: 0 });
    assert_eq!(retirement.allocated_bytes(), 0);
    assert_eq!(retirement.close_step(0, 0), ValueRetirementStep::Complete);
    assert!(retirement.terminal_is_empty());
}

/// 🎟️ A byte buffer is charged `min(grant, left)` per turn against its LIVE payload and the whole
/// allocation is freed once the charge reaches zero — the `min(grant, left)` drawdown the
/// language-agnostic contract states, and the reason a fixed-page driver never stalls on one.
/// Reserved capacity beyond the payload is freed with the buffer and is never charged for, which is
/// what `empty_reserved_text_does_not_require_capacity_sized_credit` pins on the session frontier
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn neural_physical_retirement_charges_byte_buffers_down_and_frees_the_whole_reservation() {
    let mut values = Vec::with_capacity(8193);
    values.extend(std::iter::repeat_n(1u8, 8000));
    let capacity = values.capacity();
    let payload = values.len();
    let mut retirement = ValueRetirement::default();
    retirement.owners.push_back(byte_owner(values));
    assert_eq!(retirement.allocated_bytes(), capacity);
    assert_eq!(retirement.next_close_byte_demand().unwrap(), 1);
    assert_eq!(retirement.close_step(0, capacity), ValueRetirementStep::Blocked);
    assert_eq!(retirement.close_step(1, 0), ValueRetirementStep::Blocked);
    assert_eq!(close(retirement, 1), payload);

    let mut reserved = ValueRetirement::default();
    reserved.text(String::with_capacity(8193));
    assert_eq!(reserved.next_close_byte_demand().unwrap(), 1);
    assert_eq!(close(reserved, 1), 0);
}

#[test]
fn neural_physical_retirement_direct_vector_backings_free_whole_under_any_positive_grant() {
    let strings = Vec::<String>::with_capacity(8193usize.div_ceil(size_of::<String>()));
    let capacity = strings.capacity() * size_of::<String>();
    exact_backing(Owner::Strings(strings), capacity);
    let channels = Vec::<ChannelSpec>::with_capacity(8193usize.div_ceil(size_of::<ChannelSpec>()));
    let capacity = channels.capacity() * size_of::<ChannelSpec>();
    exact_backing(Owner::Channels(channels), capacity);
    let fields = Vec::<FieldSpec>::with_capacity(8193usize.div_ceil(size_of::<FieldSpec>()));
    let capacity = fields.capacity() * size_of::<FieldSpec>();
    exact_backing(Owner::Fields(fields), capacity);
}

/// 🎟️ A nested ordered-map key is charged down through the delegating frontier exactly like a direct
/// buffer: its reserved capacity is freed with it and never charged for, and a one-byte grant is
/// enough (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn neural_physical_retirement_charges_a_nested_ordered_key_down_under_a_one_byte_grant() {
    let mut key = String::with_capacity(8193);
    key.push_str("nested-key");
    let capacity = key.capacity();
    let payload = key.len();
    let mut retirement = ValueRetirement::from_dictionary(Dictionary::new().insert(key, Value::null()));
    assert!(retirement.allocated_bytes() < capacity);
    assert_eq!(retirement.next_close_byte_demand().unwrap(), 1);
    assert_eq!(close(retirement, 1), payload);
}

fn close(mut owner: ValueRetirement, maximum_bytes: usize) -> usize {
    let mut bytes = 0;
    assert_eq!(owner.close_step(0, maximum_bytes), ValueRetirementStep::Blocked);
    assert_eq!(owner.close_step(1, 0), ValueRetirementStep::Blocked);
    for _ in 0..1_000_000 {
        match owner.close_step(1, maximum_bytes) {
            ValueRetirementStep::Pending { released_items, released_bytes } => { assert!(released_items <= 1 && released_bytes <= maximum_bytes); bytes += released_bytes; }
            ValueRetirementStep::Complete => { assert!(owner.terminal_is_empty()); return bytes; }
            ValueRetirementStep::Blocked => panic!("positive domain grant blocked"),
        }
    }
    panic!("domain retirement did not reach terminal-empty")
}

#[test]
fn nested_fixture_retires_exact_bytes_under_every_actual_grant() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️value-retirement.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let text = row["expandedText"]["text"].as_str().unwrap().repeat(row["expandedText"]["repetitions"].as_u64().unwrap() as usize);
        let json = row["json"].as_str().unwrap().replace("$text", &text);
        for maximum_bytes in [1, 64, 4096] {
            let dictionary: Dictionary = serde_json::from_str(&json).unwrap();
            let oracle: serde_json::Value = serde_json::from_str(&json).unwrap();
            assert_eq!(serde_json::to_value(&dictionary).unwrap(), oracle);
            assert_eq!(close(dictionary.into_retirement(), maximum_bytes), row["expectedBytes"].as_u64().unwrap() as usize);
        }
    }
}

#[test]
fn immutable_dictionary_aliases_release_without_copying_or_retiring_payloads() {
    let dictionary = Dictionary::new().insert("label", Value::Atom(Atom::String("🌊".repeat(4096))));
    let alias = dictionary.clone();
    let pointer = dictionary.get("label").unwrap().as_atom().unwrap().as_str().unwrap().as_ptr();
    assert_eq!(alias.get("label").unwrap().as_atom().unwrap().as_str().unwrap().as_ptr(), pointer);
    drop(alias);
    assert_eq!(close(dictionary.into_retirement(), 1), 16389);
}

#[test]
fn cold_builder_closes_nested_replacements_and_decoder_partial_errors() {
    let mut builder = ColdDictionaryBuilder::new();
    builder.insert("value".into(), Value::Dictionary(Dictionary::new().insert("old", Value::Atom(Atom::String("discarded".into())))));
    builder.insert("value".into(), Value::Dictionary(Dictionary::new().insert("new", Value::Atom(Atom::String("kept".into())))));
    assert_eq!(serde_json::to_string(builder.dictionary()).unwrap(), r#"{"value":{"new":"kept"}}"#);
    drop(builder);
    assert!(serde_json::from_str::<Dictionary>(r#"{"ok":{"long":"value"},"bad":[]}"#).is_err());
    drop(ColdValueOwner::new(Value::Dictionary(Dictionary::new().insert("owned", Value::null()))));
}

#[test]
fn strict_dictionary_and_retirement_drop_guards_reject_live_final_owners() {
    for retirement in [false, true] {
        let guarded = std::panic::catch_unwind(|| {
            let dictionary = Dictionary::new().insert("nested", Value::Dictionary(Dictionary::new().insert("label", Value::Atom(Atom::String("guarded".into())))));
            if retirement { drop(dictionary.into_retirement()); } else { drop(dictionary); }
        });
        assert!(guarded.is_err());
    }
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn partial_nested_retirement_transfers_workers_without_byte_loss() {
    let dictionary = Dictionary::new().insert("label", Value::Atom(Atom::String("高".repeat(6000))));
    let mut owner = dictionary.into_retirement(); let mut bytes = 0;
    for _ in 0..23 { if let ValueRetirementStep::Pending { released_bytes, .. } = owner.close_step(1, 1) { bytes += released_bytes; } }
    let remainder = std::thread::spawn(move || close(owner, 64)).join().unwrap();
    assert_eq!(bytes + remainder, 18005);
}
//#endregion 🔣️FixtureLaws

//#region 🧠️CacheRetirement
#[test]
fn cache_retirement_preserves_shared_roots_and_drains_replaced_nested_values() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🗃️cache-retirement/🔣️.json")).unwrap();
    for maximum_bytes in [1, 64, 4096] {
        let cache = Arc::new(super::super::NeuralCache::new());
        for seed in fixture["operations"].as_array().unwrap().iter().filter(|value| value["op"] == "seed") {
            let text = seed["text"].as_str().unwrap().repeat(seed["repeat"].as_u64().unwrap() as usize);
            cache.seed(fixture["key"].as_u64().unwrap(), Dictionary::new().insert(seed["field"].as_str().unwrap(), Value::Atom(Atom::String(text))));
        }
        let mut shared = super::super::NeuralCacheRetirement::new(Arc::clone(&cache));
        while !matches!(shared.close_step(1, maximum_bytes), ValueRetirementStep::Complete) {}
        assert!(shared.terminal_nonopaque_is_empty()); assert_eq!(cache.len(), 1);
        let mut final_owner = super::super::NeuralCacheRetirement::new(cache); let mut bytes = 0;
        assert_eq!(final_owner.close_step(1, 0), ValueRetirementStep::Blocked);
        for _ in 0..1_000_000 {
            match final_owner.close_step(1, maximum_bytes) {
                ValueRetirementStep::Pending { released_items, released_bytes } => { assert!(released_items <= 1 && released_bytes <= maximum_bytes); bytes += released_bytes; }
                ValueRetirementStep::Complete => break,
                ValueRetirementStep::Blocked => panic!("positive cache grant blocked"),
            }
        }
        assert!(final_owner.terminal_nonopaque_is_empty()); assert_eq!(bytes, fixture["expected"]["finalReleasedBytes"].as_u64().unwrap() as usize);
    }
}

#[test]
fn cache_live_final_drop_is_guarded_instead_of_recursively_destroying_dictionaries() {
    assert!(std::panic::catch_unwind(|| {
        let cache = super::super::NeuralCache::new(); cache.seed(1, Dictionary::new().insert("label", Value::Atom(Atom::String("guarded".into())))); drop(cache);
    }).is_err());
}
//#endregion 🧠️CacheRetirement

//#region 📸️EvaluationRetirement
#[test]
fn evaluation_snapshot_and_channel_owners_retire_exact_long_key_and_payload_bytes() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧮️evaluation-owners/🔣️.json")).unwrap();
    for maximum_bytes in [1, 64, 4096] {
        let node = fixture["node"]["text"].as_str().unwrap().repeat(fixture["node"]["repeat"].as_u64().unwrap() as usize);
        let payload = fixture["payload"]["text"].as_str().unwrap().repeat(fixture["payload"]["repeat"].as_u64().unwrap() as usize);
        let snapshot = TreeSnapshot { neurons: BTreeMap::from([(node, NeuronSnapshot { key: 1, incoming: 2, dependents: vec![payload.clone()] })]), seed_keys: BTreeMap::from([("seed".into(), 3)]) };
        let channels = EvalChannels { outputs: BTreeMap::from([("node".into(), Dictionary::new().insert("label", Value::Atom(Atom::String(payload))))]), inputs: BTreeMap::new() };
        let mut retirement = ValueRetirement::default(); retirement.push_snapshot(snapshot); retirement.push_channels(channels);
        assert_eq!(close(retirement, maximum_bytes), fixture["expectedBytes"].as_u64().unwrap() as usize);
    }
}
//#endregion 📸️EvaluationRetirement
