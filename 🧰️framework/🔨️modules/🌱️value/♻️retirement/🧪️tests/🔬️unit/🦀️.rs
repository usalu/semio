use super::*;

#[test]
fn shared_full_grant_owns_original_aliases_weak_backing_and_each_physical_release() {
    use crate::{value::observe_retirement_allocations,retained_clone::RetainedCloneGrant};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/📏️shared-physical/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {for copy in fixture["workBytes"].as_array().unwrap() {
        let copy=copy.as_u64().unwrap()as usize;
        let (root,(born,freed))=observe_retirement_allocations(||{let mut text=String::with_capacity(row["capacity"].as_u64().unwrap()as usize);text.push_str(row["text"].as_str().unwrap());Arc::new(text)});let original=born-freed;let pointer=Arc::as_ptr(&root);let alias=Arc::clone(&root);let weak=Arc::downgrade(&root);assert_eq!(serde_json::to_value(root.as_str()).unwrap(),row["text"]);
        let mut owner=controlled::ControlledRetirement::new(root).map_err(|(error,_)|error).unwrap();let (mut births,mut released,mut copied)=(0,0,0);
        for _ in 0..4 {
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};let (step,(a,r))=observe_retirement_allocations(||owner.step(grant).unwrap());assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(a,r));births+=a;released+=r;
        }
        assert_eq!(released,0);assert_eq!(Arc::as_ptr(&alias),pointer);drop(alias);
        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};let (step,heap)=observe_retirement_allocations(||owner.step(grant).unwrap());assert_eq!(heap.1,step.progress().released_bytes);births+=heap.0;released+=heap.1;assert!(weak.upgrade().is_none());assert_eq!(weak.as_ptr(),pointer);let (_,frame)=observe_retirement_allocations(||drop(weak));released+=frame.1;
        for turn in 0..fixture["maximumTurns"].as_u64().unwrap() {
            if owner.terminal_is_empty(){break;}
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
            let (zero,heap)=observe_retirement_allocations(||owner.step(RetainedCloneGrant {maximum_items:0,..grant}).unwrap());assert_eq!(zero.progress(),Default::default());assert_eq!(heap,(0,0));
            for denied in [(grant.maximum_capacity_bytes!=0).then_some(RetainedCloneGrant {maximum_capacity_bytes:grant.maximum_capacity_bytes.saturating_sub(1),..grant}),(grant.maximum_release_bytes!=0).then_some(RetainedCloneGrant {maximum_release_bytes:grant.maximum_release_bytes.saturating_sub(1),..grant})].into_iter().flatten(){let (step,heap)=observe_retirement_allocations(||owner.step(denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));}
            let (step,(a,r))=observe_retirement_allocations(||owner.step(grant).unwrap());let progress=step.progress();assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(a,r));assert!(progress.copied_bytes<=copy);births+=a;released+=r;copied+=progress.copied_bytes;assert!(progress.copied_items!=0,"exact shared retirement stalled on turn {turn}");
        }
        assert!(owner.terminal_is_empty());assert_eq!(copied,0,"String payload retirement is native length metadata, copy0");assert_eq!(released,original+births);
        eprintln!("[DEBUG] Full shared owner copy={copy} logical={copied} original={original} births={births} physical={released}");
    }}
}

#[test]
fn erased_controlled_owner_retains_exact_independent_grants_and_returns_refused_ownership() {
    use crate::{retained_clone::{RetainedCloneGrant, RetainedCloneStep}, value::observe_retirement_allocations};
    struct Unsupported(String);
    impl RetireOwned for Unsupported { fn retirement(self) -> Box<dyn RetirementCursor> { self.0.retirement() } }
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎮️erased-controlled/🔣️.json")).unwrap();
    let text = law["text"].as_str().unwrap();
    let unsupported = Unsupported(text.into());
    let pointer = unsupported.0.as_ptr();
    let ((error, unsupported), effects) = observe_retirement_allocations(|| controlled::ControlledRetirement::new(unsupported).err().unwrap());
    assert_eq!(error.kind.as_str(), law["unsupportedKind"].as_str().unwrap());
    assert_eq!(unsupported.0.as_ptr(), pointer);
    let (_, release) = observe_retirement_allocations(|| drop(error));
    let constructor_effects = (effects.0, effects.1 + release.1);
    let ((error, unsupported), heap) = observe_retirement_allocations(|| controlled::admit_controlled_retirement(unsupported, RetainedCloneGrant::default()).err().unwrap());
    assert_eq!(error.kind.as_str(), law["unsupportedKind"].as_str().unwrap());
    assert_eq!(unsupported.0.as_ptr(), pointer);
    assert_eq!(heap, (0, 0));
    drop(unsupported);
    assert_eq!(constructor_effects, (law["unsupportedEffects"]["bornBytes"].as_u64().unwrap()as usize,law["unsupportedEffects"]["releasedBytes"].as_u64().unwrap()as usize));
    let mut source = String::with_capacity(law["reservedCapacity"].as_u64().unwrap() as usize);
    source.push_str(text);
    assert_eq!(serde_json::from_str::<String>(&serde_json::to_string(&source).unwrap()).unwrap(), source);
    let source_capacity = source.capacity();
    let pointer = source.as_ptr();
    let admission = RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: controlled::controlled_retirement_birth_bytes::<String>(), maximum_depth: law["maximumDepth"].as_u64().unwrap() as usize, ..Default::default() };
    let ((error, source), heap) = observe_retirement_allocations(|| controlled::admit_controlled_retirement(source, RetainedCloneGrant { maximum_capacity_bytes: admission.maximum_capacity_bytes - 1, ..admission }).err().unwrap());
    assert_eq!(error.kind, crate::ValueRefusalKind::OwnershipLimit);
    assert_eq!(source.as_ptr(), pointer);
    assert_eq!(heap, (0, 0));
    let ((mut owner, birth), heap) = observe_retirement_allocations(|| controlled::admit_controlled_retirement(source, admission).unwrap_or_else(|_| panic!("String exact admission")));
    assert_eq!(heap, (birth.retained_capacity_bytes, 0));
    assert!(birth.fits(admission));
    let mut born = birth.retained_capacity_bytes;
    let mut released = 0;
    let mut processed = 0;
    for _ in 0..law["maximumTurns"].as_u64().unwrap() {
        let ((copy, capacity, release, depth), heap) = observe_retirement_allocations(|| { let copy = owner.next_copy_byte_demand().unwrap(); let release = owner.next_release_byte_demand().unwrap(); (copy, owner.next_capacity_byte_demand(if copy != 0 { law["maximumCopyBytes"].as_u64().unwrap() as usize } else { release }).unwrap(), release, owner.next_depth_demand().unwrap()) });
        assert_eq!(heap, (0, 0));
        assert!(depth <= law["maximumDepth"].as_u64().unwrap() as usize);
        let grant = RetainedCloneGrant { maximum_items: law["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: law["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: depth };
        let (zero, heap) = observe_retirement_allocations(|| owner.step(RetainedCloneGrant::default()).unwrap());
        assert_eq!(zero.progress(), Default::default());
        assert_eq!(heap, (0, 0));
        if depth != 0 {
            let (error, heap) = observe_retirement_allocations(|| owner.step(RetainedCloneGrant { maximum_depth: depth - 1, ..grant }).unwrap_err());
            assert_eq!(error.kind, crate::ValueRefusalKind::DepthLimit);
            assert_eq!(heap, (0, 0));
            assert_eq!(owner.next_depth_demand().unwrap(), depth);
        }
        for denied in [(copy != 0).then_some(RetainedCloneGrant { maximum_copy_bytes: copy.saturating_sub(1), ..grant }), (capacity != 0).then_some(RetainedCloneGrant { maximum_capacity_bytes: capacity.saturating_sub(1), ..grant }), (release != 0).then_some(RetainedCloneGrant { maximum_release_bytes: release.saturating_sub(1), ..grant })].into_iter().flatten() {
            let (step, heap) = observe_retirement_allocations(|| owner.step(denied).unwrap());
            assert_eq!(step.progress(), Default::default());
            assert_eq!(heap, (0, 0));
        }
        let (step, heap) = observe_retirement_allocations(|| owner.step(grant).unwrap());
        assert!(step.progress().fits(grant));
        assert_eq!(heap, (step.progress().retained_capacity_bytes, step.progress().released_bytes));
        born += heap.0;
        released += heap.1;
        processed += step.progress().copied_bytes;
        if matches!(step, RetainedCloneStep::Complete(_)) { break; }
    }
    assert!(owner.terminal_is_empty());
    assert_eq!(processed, 0);
    let frame = owner.frame_release_bytes();
    assert_eq!(observe_retirement_allocations(|| drop(owner)).1, (0, frame));
    released += frame;
    assert_eq!(released, born + source_capacity);
    eprintln!("[DEBUG] Erased genuine ControlledRetirement preserved copy3, independent exact capacity/release, refused original pointer and observed heap conservation birth={born}/release={released}");
}

#[test]
fn native_recursive_intrinsic_retirement_admits_each_constructor_and_physical_release() {
    use crate::{retained_clone::{RetainedCloneGrant, RetainedCloneStep}, value::observe_retirement_allocations};
    fn run<T: RetireOwned>(value: T, row: &serde_json::Value, law: &serde_json::Value) {
        let (result, heap) = observe_retirement_allocations(|| controlled::ControlledRetirement::new(value));
        assert_eq!(heap, (0, 0));
        let mut owner = result.unwrap_or_else(|_| panic!("intrinsic constructor requires complete controlled authority: {}", row["kind"]));
        let mut copied = 0;
        let mut born = 0;
        let mut released = 0;
        let demands = |owner: &controlled::ControlledRetirement<T>| {
            let copy = owner.next_copy_byte_demand().unwrap();
            let release = owner.next_release_byte_demand().unwrap();
            (copy, owner.next_capacity_byte_demand(if copy == 0 { release } else { law["maximumCopyBytes"].as_u64().unwrap() as usize }).unwrap(), release, owner.next_depth_demand().unwrap())
        };
        for _ in 0..law["maximumTurns"].as_u64().unwrap() {
            let ((copy, capacity, release, depth), heap) = observe_retirement_allocations(|| demands(&owner));
            assert_eq!(heap, (0, 0));
            assert!(copy <= law["maximumCopyBytes"].as_u64().unwrap() as usize);
            assert!(depth <= law["maximumDepth"].as_u64().unwrap() as usize);
            let grant = RetainedCloneGrant { maximum_items: law["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: law["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: law["maximumDepth"].as_u64().unwrap() as usize };
            if !owner.terminal_is_empty() {
                let (refused, heap) = observe_retirement_allocations(|| owner.step(RetainedCloneGrant { maximum_depth: depth - 1, ..grant }));
                assert_eq!(refused.unwrap_err().kind, crate::ValueRefusalKind::DepthLimit);
                assert_eq!(heap, (0, 0));
                assert_eq!(observe_retirement_allocations(|| demands(&owner)), ((copy, capacity, release, depth), (0, 0)));
            }
            let (zero, heap) = observe_retirement_allocations(|| owner.step(RetainedCloneGrant::default()).unwrap());
            assert_eq!(zero.progress(), Default::default());
            assert_eq!(heap, (0, 0));
            assert_eq!(observe_retirement_allocations(|| demands(&owner)), ((copy, capacity, release, depth), (0, 0)));
            for denied in [
                (copy != 0).then_some(RetainedCloneGrant { maximum_copy_bytes: copy.saturating_sub(1), ..grant }),
                (capacity != 0).then_some(RetainedCloneGrant { maximum_capacity_bytes: capacity.saturating_sub(1), ..grant }),
                (release != 0).then_some(RetainedCloneGrant { maximum_release_bytes: release.saturating_sub(1), ..grant }),
            ].into_iter().flatten() {
                let (step, heap) = observe_retirement_allocations(|| owner.step(denied).unwrap());
                assert_eq!(step.progress(), Default::default());
                assert_eq!(heap, (0, 0));
                assert_eq!(observe_retirement_allocations(|| demands(&owner)), ((copy, capacity, release, depth), (0, 0)));
            }
            let (step, heap) = observe_retirement_allocations(|| owner.step(grant).unwrap());
            assert!(step.progress().fits(grant));
            assert_eq!(heap, (step.progress().retained_capacity_bytes, step.progress().released_bytes));
            copied += step.progress().copied_bytes;
            born += heap.0;
            released += heap.1;
            if matches!(step, RetainedCloneStep::Complete(_)) { break; }
        }
        assert!(owner.terminal_is_empty(), "intrinsic exact retirement must finish: {}", row["kind"]);
        assert_eq!(copied, row["payloadBytes"].as_u64().unwrap() as usize, "{}", row["kind"]);
        assert!(released >= born);
        assert_eq!(observe_retirement_allocations(|| drop(owner)).1, (0, 0));
        println!("[DEBUG] Intrinsic retirement kind={} copy={copied} birth={born} release={released}; exact heap receipts, zero/below nonmovement, terminal0heap", row["kind"]);
    }
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🌳️intrinsic/🔣️.json")).unwrap();
    let frontier_law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎮️frontier/🔣️.json")).unwrap();
    fn source(row: &serde_json::Value) -> crate::DslValue {
        let capacity = row["capacity"].as_u64().unwrap_or(0) as usize;
        match row["kind"].as_str().unwrap() {
            "bytes" | "reserved-bytes" => { let mut bytes = Vec::with_capacity(capacity); bytes.extend(row["value"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8)); crate::DslValue::Bytes(bytes) },
            "reserved-text" => crate::DslValue::String(String::with_capacity(capacity)),
            "reserved-array" => crate::DslValue::Array(Vec::with_capacity(capacity)),
            "reserved-object" => crate::DslValue::Object(Vec::with_capacity(capacity)),
            _ => crate::DslValue::from(&row["value"]),
        }
    }
    for row in law["cases"].as_array().unwrap() {
        let value = source(row);
        assert_eq!(serde_json::Value::from(&value), row["value"]);
        assert_eq!(serde_json::to_value(&value).unwrap(), row["value"]);
        for prefix in frontier_law["prefixTurns"].as_array().unwrap() {
            let original = source(row);
            assert_eq!(serde_json::to_value(&original).unwrap(), row["value"]);
            let mut inner = controlled::ControlledRetirement::new(original).unwrap_or_else(|_| panic!("original intrinsic frontier"));
            let mut copied = 0;
            for _ in 0..prefix.as_u64().unwrap() {
                let copy = inner.next_copy_byte_demand().unwrap();
                let release = inner.next_release_byte_demand().unwrap();
                let capacity = inner.next_capacity_byte_demand(if copy == 0 { release } else { frontier_law["maximumCopyBytes"].as_u64().unwrap() as usize }).unwrap();
                let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 3, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: 64 };
                let (step, heap) = observe_retirement_allocations(|| inner.step(grant).unwrap());
                assert_eq!(heap, (step.progress().retained_capacity_bytes, step.progress().released_bytes));
                assert!(step.progress().fits(grant));
                copied += step.progress().copied_bytes;
                if matches!(step, RetainedCloneStep::Complete(_)) { break; }
            }
            let mut remaining = row.clone();
            remaining["payloadBytes"] = (row["payloadBytes"].as_u64().unwrap() - copied as u64).into();
            run(inner, &remaining, &law);
        }
        run(Box::new(value), row, &law);
    }
}

#[test]
fn paged_native_controlled_retirement_observes_copy_demand_without_releasing_backing() {
    use crate::{retained_clone::{RetainedCloneGrant, RetainedCloneStep}, value::observe_retirement_allocations};
    fn run<T: RetireOwned>(value: T, row: &serde_json::Value) {
        let (mut owner, heap) = observe_retirement_allocations(|| controlled::ControlledRetirement::new(value).unwrap_or_else(|_| panic!("native controlled owner")));
        assert_eq!(heap, (0, 0));
        let mut copied = 0;
        let mut released = 0;
        let mut observed = false;
        for _ in 0..100000 {
            let ((copy, capacity, release), heap) = observe_retirement_allocations(|| {
                let copy = owner.next_copy_byte_demand().unwrap();
                (copy, owner.next_capacity_byte_demand(copy).unwrap(), owner.next_release_byte_demand().unwrap())
            });
            assert_eq!(heap, (0, 0));
            assert!(copy + capacity + release <= 4096);
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: 64 };
            let (zero, heap) = observe_retirement_allocations(|| owner.step(RetainedCloneGrant::default()).unwrap());
            assert_eq!(zero.progress(), Default::default());
            assert_eq!(heap, (0, 0));
            if copy > 0 {
                observed = true;
                assert_eq!(copy, row["minimumCopyBytes"].as_u64().unwrap() as usize);
                assert_eq!((capacity, release), (0, 0));
                let (denied, heap) = observe_retirement_allocations(|| owner.step(RetainedCloneGrant { maximum_copy_bytes: copy - 1, maximum_capacity_bytes: 0, ..grant }).unwrap());
                assert_eq!(denied.progress(), Default::default());
                assert_eq!(heap, (0, 0));
                assert_eq!(owner.next_copy_byte_demand().unwrap(), copy);
            }
            let (step, heap) = observe_retirement_allocations(|| owner.step(grant).unwrap());
            assert!(step.progress().fits(grant));
            assert_eq!(heap, (step.progress().retained_capacity_bytes, step.progress().released_bytes));
            if copy > 0 { assert_eq!(step.progress().released_bytes, 0); }
            copied += step.progress().copied_bytes;
            released += step.progress().released_bytes;
            if matches!(step, RetainedCloneStep::Complete(_)) { break; }
        }
        assert_eq!(observed, row["totalCopyBytes"].as_u64().unwrap() > 0);assert!(owner.terminal_is_empty());
        assert_eq!(copied, row["totalCopyBytes"].as_u64().unwrap() as usize);
        assert_eq!(owner.next_copy_byte_demand().unwrap(), 0);
        assert_eq!(observe_retirement_allocations(|| drop(owner)).1, (0, 0));
        println!("[DEBUG] Native controlled copy demand kind={} copy={copied} physical-release={released}; zero/below with independent capacity retained, exact work frees0, terminal0heap", row["kind"]);
    }
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📏️copy-demand/🔣️.json")).unwrap();
    for row in law["cases"].as_array().unwrap() {
        match row["kind"].as_str().unwrap() {
            "u32" => run(row["value"].as_u64().unwrap() as u32, row),
            "text" => { let mut text = String::with_capacity(128); text.push_str(row["value"].as_str().unwrap()); run(text, row); }
            "u32-list" => run(row["value"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u32).collect::<Vec<_>>(), row),
            _ => panic!("unknown native corpus owner"),
        }
    }
}

#[derive(crate::RetainedClone,crate::RetireOwned,serde::Serialize)]
struct CloseDemandRecord {
    name:crate::paged::PagedUtf8<{usize::MAX}>,
    bytes:crate::paged::PagedBytes<{usize::MAX}>,
    attributes:crate::paged::PagedMap<crate::paged::PagedUtf8<{usize::MAX}>,{usize::MAX}>,
}

#[derive(crate::RetainedClone,crate::RetireOwned,serde::Serialize)]
#[serde(untagged)]
enum CloseDemandEnum { Record(CloseDemandRecord), Empty }

#[test]
fn paged_native_retained_clone_close_admits_exact_leaf_and_composite_demand() {
    use crate::{retained_clone::{RetainedClone, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneSource, RetainedCloneStep}, value::observe_retirement_allocations};
    fn run<T:RetainedClone>(source:crate::retained_clone::FixtureSource<T>,label:&str) {
        let (mut cursor,allocation)=observe_retirement_allocations(T::retained_clone_cursor);assert_eq!(allocation,(0,0));
        let mut ready=false;
        for turn in 0..10000 {
            let grant=match turn%3 {0=>RetainedCloneGrant::one_capacity_turn(4096,64),1=>RetainedCloneGrant::one_payload_turn(4096,64),_=>RetainedCloneGrant::one_release_turn(4096,64)};
            let (step,allocation)=observe_retirement_allocations(||cursor.advance(source.borrow(),grant).unwrap());
            assert!(step.progress().fits(grant));assert!(allocation.0<=step.progress().retained_capacity_bytes&&allocation.1<=step.progress().released_bytes);
            if matches!(step,RetainedCloneStep::Complete(_)){ready=true;break;}
        }
        assert!(ready);cursor.begin_close();
        let mut terminal=false;let mut born=0;let mut released=0;let mut copied=0;
        for turn in 0usize..100000 {
            let ((copy,capacity,release),allocation)=observe_retirement_allocations(||{let copy=cursor.next_close_copy_byte_demand().unwrap();(copy,cursor.next_close_capacity_byte_demand(copy).unwrap(),cursor.next_close_release_byte_demand().unwrap())});
            assert_eq!(allocation,(0,0));assert!(copy+capacity+release<=4096);
            let (zero,allocation)=observe_retirement_allocations(||cursor.close_step(RetainedCloneGrant::default()).unwrap());assert_eq!(zero.progress(),RetainedCloneProgress::default());assert_eq!(allocation,(0,0));
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:64};
            if copy!=0 || capacity!=0 || release!=0 {
                let below=RetainedCloneGrant {maximum_copy_bytes:copy.saturating_sub(1),maximum_capacity_bytes:capacity.saturating_sub(1),maximum_release_bytes:release.saturating_sub(1),..grant};
                let (step,allocation)=observe_retirement_allocations(||cursor.close_step(below).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(allocation,(0,0));
            }
            let (step,allocation)=observe_retirement_allocations(||cursor.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert!(allocation.0<=step.progress().retained_capacity_bytes&&allocation.1<=step.progress().released_bytes);born+=allocation.0;released+=allocation.1;copied+=step.progress().copied_bytes;
            if copy!=0 {assert_eq!(allocation.1,0);}
            if turn>=1024&&turn.is_power_of_two(){println!("[DEBUG] Retained leaf exact close label={label} turn={turn} copy={copy} capacity={capacity} release={release} progress={:?}",step.progress());}
            if matches!(step,RetainedCloneStep::Complete(_)){terminal=true;break;}
        }
        assert!(terminal&&cursor.terminal_is_empty(),"retained leaf exact demand must reach terminal: {label}");assert_eq!(observe_retirement_allocations(||drop(cursor)).1,(0,0));
        println!("[DEBUG] Retained leaf exact close label={label} demand0heap; zero/one-below/exact separately admitted copy={copied} birth={born} release={released}, copy-turn-free0 terminal0heap");
    }
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧬️retained-clone/🧫️fixtures/📏️release-authority/🔣️.json")).unwrap();
    run(RetainedCloneSource::from_owner(fixture["leafCloseDemand"]["scalar"].as_u64().unwrap() as u8),"scalar");
    run(RetainedCloneSource::from_owner(fixture["leafCloseDemand"]["text"].as_str().unwrap().to_owned()),"UTF8");
    let value=&fixture["compositeCloseDemand"]["value"];
    let record=||CloseDemandRecord {
        name:crate::paged::PagedUtf8::try_from_str(value["name"].as_str().unwrap()).unwrap(),
        bytes:crate::paged::PagedBytes::try_from_slice(&value["bytes"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as u8).collect::<Vec<_>>()).unwrap(),
        attributes:crate::paged::PagedMap::try_from_entries(value["attributes"].as_object().unwrap().iter().map(|(key,value)|(crate::paged::PagedUtf8::try_from_str(key).unwrap(),crate::paged::PagedUtf8::try_from_str(value.as_str().unwrap()).unwrap()))).unwrap(),
    };
    assert_eq!(serde_json::to_value(record()).unwrap(),*value);
    run(RetainedCloneSource::from_owner(record()),"derived paged record");
    assert_eq!(serde_json::to_value(CloseDemandEnum::Record(record())).unwrap(),*value);
    run(RetainedCloneSource::from_owner(CloseDemandEnum::Record(record())),"derived enum");
    run(RetainedCloneSource::from_owner(CloseDemandEnum::Empty),"empty enum");
    run(RetainedCloneSource::from_owner((17u8,Some(crate::paged::PagedUtf8::<{usize::MAX}>::from("groß")),Box::new(crate::paged::PagedUtf8::<{usize::MAX}>::from("🧬")))),"tuple optional boxed native text");
}

#[derive(serde::Serialize)]
struct MacroControlledRecord { label: String, values: Vec<u32>, nested: Option<(String, Vec<u32>)> }
crate::artifact_retire_struct!(MacroControlledRecord { label, values, nested });

#[test]
fn retirement_struct_macro_defers_exact_field_birth_and_release_allocations() {
    use crate::{retained_clone::RetainedCloneGrant, value::observe_retirement_allocations};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../📦️paged/🧫️fixtures/🎮️native-owner/🔣️.json")).unwrap();
    let values = fixture["cloneClose"]["optionalValues"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u32).collect::<Vec<_>>();
    let label = fixture["controlledRetirement"]["label"].as_str().unwrap();
    let owner = MacroControlledRecord { label: label.into(), values: values.clone(), nested: Some((label.repeat(80), values.clone())) };
    assert_eq!(serde_json::to_value(&owner).unwrap(), serde_json::json!({"label": label, "values": values, "nested": [label.repeat(80), values]}));
    let (retirement, allocation) = observe_retirement_allocations(|| controlled::ControlledRetirement::new(owner));
    assert_eq!(allocation, (0, 0));
    let mut retirement = retirement.unwrap_or_else(|_| panic!("macro record must expose measured deferred constructor authority"));
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 4096, maximum_release_bytes: 4096, maximum_depth: 64 };
    let (zero, allocation) = observe_retirement_allocations(|| retirement.step(RetainedCloneGrant::default()).unwrap());
    assert_eq!(zero.progress(), Default::default());
    assert_eq!(allocation, (0, 0));
    let mut turns = 0;
    for _ in 0..10000 {
        let (step, allocation) = observe_retirement_allocations(|| retirement.step(grant).unwrap());
        let progress = step.progress();
        assert!(progress.fits(grant));
        assert!(allocation.0 <= progress.retained_capacity_bytes && allocation.1 <= progress.released_bytes, "actual field constructor/release: {allocation:?} {progress:?}");
        turns += 1;
        if matches!(step, crate::retained_clone::RetainedCloneStep::Complete(_)) { break; }
    }
    assert!(turns > 1 && retirement.terminal_is_empty());
    let (_, allocation) = observe_retirement_allocations(|| drop(retirement));
    assert_eq!(allocation, (0, 0));
    println!("[DEBUG] retirement struct macro exact nested field allocation/release observed under copy64/capacity4096/release4096, turns={turns}");
}

#[test]
fn retirement_forward_deque_keeps_original_source_and_exact_physical_backing() {
    use crate::{retained_clone::{RetainedCloneGrant, RetainedCloneStep}, value::observe_retirement_allocations};
    use std::collections::VecDeque;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📨️emission/📦️owned/🧫️fixtures/🔣️.json")).unwrap();
    for count in fixture["census"]["rowCounts"].as_array().unwrap() {
        let count = count.as_u64().unwrap() as usize;
        let expected = (0..count).map(|index| index as u32).collect::<Vec<_>>();
        let pointer = expected.as_ptr();
        let backing = expected.capacity() * std::mem::size_of::<u32>();
        let (mut source, allocations) = observe_retirement_allocations(|| VecDeque::from(expected));
        assert_eq!(allocations, (0, 0));
        assert_eq!(source.as_slices().0.as_ptr(), pointer);
        assert_eq!(source.capacity() * std::mem::size_of::<u32>(), backing);
        assert_eq!(serde_json::to_value(&source).unwrap(), serde_json::json!((0..count).collect::<Vec<_>>()));
        if count > 1 { assert_eq!(source.pop_front(), Some(0)); source.push_front(0); }
        assert!(VecDeque::<u32>::controlled_retirement_supported(), "retained forward source requires controlled child birth and physical backing authority");
        let (owner, allocations) = observe_retirement_allocations(|| controlled::ControlledRetirement::new(source));
        assert_eq!(allocations, (0, 0));
        let mut owner = owner.unwrap_or_else(|_| panic!("exact admitted forward source"));
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 4096, maximum_release_bytes: 4096, maximum_depth: 64 };
        let mut released = 0;
        for _ in 0..100000 {
            let capacity = owner.next_capacity_byte_demand(grant.maximum_release_bytes).unwrap();
            let release = owner.next_release_byte_demand().unwrap();
            if capacity == 0 && release > 0 {
                let (step, allocations) = observe_retirement_allocations(|| owner.step(RetainedCloneGrant { maximum_release_bytes: release - 1, ..grant }).unwrap());
                assert_eq!(step.progress().released_bytes, 0); assert_eq!(allocations.1, 0);
                assert_eq!(owner.next_release_byte_demand().unwrap(), release);
            }
            let (step, allocations) = observe_retirement_allocations(|| owner.step(grant).unwrap());
            assert!(step.progress().fits(grant));
            assert!(allocations.0 <= step.progress().retained_capacity_bytes && allocations.1 <= step.progress().released_bytes);
            released += allocations.1;
            if matches!(step, RetainedCloneStep::Complete(_)) { break; }
        }
        assert!(owner.terminal_is_empty() && released >= backing);
        assert_eq!(observe_retirement_allocations(|| drop(owner)).1, (0, 0));
        println!("[DEBUG] retained forward deque: rows={count} original-allocation=true backing={backing} exact-paid-release={released}");
    }
}

#[derive(crate::RetireOwned, crate::RetainedClone, serde::Serialize)]
enum ControlledTree {
    Node(crate::paged::PagedUtf8<{usize::MAX}>, crate::list::PagedList<ControlledTree, {usize::MAX}>),
    Leaf,
}

#[derive(crate::RetireOwned, crate::RetainedClone, serde::Serialize)]
struct ControlledCloneRecord {
    tree: ControlledTree,
    optional: Option<(crate::paged::PagedUtf8<{usize::MAX}>, crate::list::PagedList<u32, {usize::MAX}>)>,
    objects: crate::paged::PagedMap<crate::paged::PagedUtf8<{usize::MAX}>, {usize::MAX}>,
}

#[test]
fn paged_native_controlled_retirement_admits_actual_birth_and_release_allocations() {
    use crate::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
    use crate::value::observe_retirement_allocations;
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../📦️paged/🧫️fixtures/🎮️native-owner/🔣️.json")).unwrap();
    let law = &corpus["controlledRetirement"];
    let depth = law["depth"].as_u64().unwrap() as usize;
    let budget = law["maximumBytes"].as_u64().unwrap() as usize;
    for pause in law["pauseAt"].as_array().unwrap() {
        let mut tree = ControlledTree::Leaf;
        for _ in 0..depth { tree = ControlledTree::Node(law["label"].as_str().unwrap().into(), [tree].into_iter().collect()); }
        let (result, birth) = observe_retirement_allocations(|| controlled::ControlledRetirement::new(tree));
        assert_eq!(birth, (0, 0));
        let mut retirement = result.unwrap_or_else(|_| panic!("derived tree must expose controlled constructor authority"));
        let (zero, allocation) = observe_retirement_allocations(|| retirement.step(RetainedCloneGrant::default()).unwrap());
        assert_eq!(zero.progress(), Default::default());
        assert_eq!(allocation, (0, 0));
        let mut complete = false;
        let mut admitted = 0;
        let mut released = 0;
        for turn in 0..100000 {
            if turn == pause.as_u64().unwrap() as usize {
                let (step, allocation) = observe_retirement_allocations(|| retirement.step(RetainedCloneGrant::default()).unwrap());
                assert_eq!(step.progress(), Default::default());
                assert_eq!(allocation, (0, 0));
            }
            let grant = match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(budget, usize::MAX), 1 => RetainedCloneGrant::one_payload_turn(budget, usize::MAX), _ => RetainedCloneGrant::one_release_turn(budget, usize::MAX) };
            assert!(retirement.next_capacity_byte_demand(grant.maximum_release_bytes).unwrap() <= budget);
            assert!(retirement.next_release_byte_demand().unwrap() <= budget);
            let (result, allocation) = observe_retirement_allocations(|| retirement.step(grant));
            let step = result.unwrap();
            let progress = step.progress();
            assert!(progress.fits(grant));
            assert!(progress.copied_bytes + progress.retained_capacity_bytes + progress.released_bytes <= budget);
            assert!(allocation.0 <= progress.retained_capacity_bytes, "unadmitted birth: {allocation:?} {progress:?}");
            assert!(allocation.1 <= progress.released_bytes, "unadmitted release: {allocation:?} {progress:?}");
            admitted += allocation.0;
            released += allocation.1;
            if matches!(step, RetainedCloneStep::Complete(_)) { complete = true; break; }
        }
        assert!(complete && retirement.terminal_is_empty());
        assert!(released > admitted);
        let (_, terminal) = observe_retirement_allocations(|| drop(retirement));
        assert_eq!(terminal, (0, 0));
    }
    eprintln!("[DEBUG] controlled native retirement depth={depth} admitted every scaffold/frontier birth and actual release within4096");
}

#[test]
fn paged_native_append_close_exposes_exact_capacity_and_release_demand() {
    use crate::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
    use crate::value::observe_retirement_allocations;
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../📦️paged/🧫️fixtures/🎮️native-owner/🔣️.json")).unwrap();
    let text = corpus["prefix"].as_str().unwrap().repeat(corpus["prefixRepeat"].as_u64().unwrap() as usize);
    let budget = corpus["appendClose"]["maximumBytes"].as_u64().unwrap() as usize;
    let mut close = crate::retained_clone::RetainedCloneClose::default();
    let mut retained = Some(String::from("cold-control-refusal"));
    let pointer = retained.as_ref().unwrap().as_ptr();
    let frame = owned_retirement_birth_bytes::<String>();
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: frame - 1, maximum_release_bytes: 0, maximum_depth: 1 };
    let (denied, heap) = observe_retirement_allocations(|| close.begin_granted(&mut retained, grant).unwrap().unwrap());
    assert_eq!(denied.progress(), Default::default());assert_eq!(heap, (0,0));assert_eq!(retained.as_ref().unwrap().as_ptr(), pointer);assert!(close.is_empty());
    assert_eq!(close.next_owner_capacity_byte_demand::<String>(true,0).unwrap(),frame);
    let (admitted, heap) = observe_retirement_allocations(|| close.begin_granted(&mut retained, RetainedCloneGrant { maximum_capacity_bytes: frame, ..grant }).unwrap().unwrap());
    assert_eq!(heap,(admitted.progress().retained_capacity_bytes,0));assert!(retained.is_none());
    for _ in 0..1000 { if close.is_empty() { break; }let grant=RetainedCloneGrant { maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:close.next_capacity_byte_demand(4096).unwrap(),maximum_release_bytes:close.next_release_byte_demand().unwrap(),maximum_depth:close.next_depth_demand().unwrap() };let (step,heap)=observe_retirement_allocations(||close.step_granted(grant).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes)); }
    assert!(close.is_empty());
    for partial in corpus["appendClose"]["cancelAt"].as_array().unwrap() {
        let partial = partial.as_u64().unwrap() as usize;
        let mut destination = crate::paged::PagedUtf8::<{usize::MAX}>::try_from_str(corpus["appendClose"]["destinationPrefix"].as_str().unwrap()).unwrap();
        let (mut cursor, allocation) = observe_retirement_allocations(crate::paged::PagedUtf8AppendCursor::default);
        assert_eq!(allocation, (0, 0));
        for turn in 0..partial {
            let grant = match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(budget, usize::MAX), 1 => RetainedCloneGrant::one_payload_turn(budget, usize::MAX), _ => RetainedCloneGrant::one_release_turn(budget, usize::MAX) };
            let (step, allocation) = observe_retirement_allocations(|| cursor.advance(&text, &mut destination, grant).unwrap());
            assert!(step.progress().fits(grant));assert!(allocation.0 <= step.progress().retained_capacity_bytes && allocation.1 <= step.progress().released_bytes);
        }
        cursor.begin_close();
        let mut closed = false;
        let mut turns = 0;
        let mut released = 0;
        for _ in 0..100000 {
            let ((copy, capacity, release), allocation) = observe_retirement_allocations(|| {let copy=cursor.next_close_copy_byte_demand().unwrap();(copy,cursor.next_close_capacity_byte_demand(copy).unwrap(),cursor.next_close_release_byte_demand().unwrap())});
            assert_eq!(allocation, (0, 0));
            let (step, allocation) = observe_retirement_allocations(|| cursor.close_step(RetainedCloneGrant::default()).unwrap());
            assert_eq!(step.progress(), Default::default());assert_eq!(allocation, (0, 0));
            if copy > 0 || capacity > 0 || release > 0 {
                let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy.saturating_sub(1), maximum_capacity_bytes: capacity.saturating_sub(1), maximum_release_bytes: release.saturating_sub(1), maximum_depth: usize::MAX };
                let (step, allocation) = observe_retirement_allocations(|| cursor.close_step(grant).unwrap());
                assert_eq!(step.progress(), Default::default());assert_eq!(allocation, (0, 0));
            }
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: usize::MAX };
            assert!(grant.maximum_copy_bytes + grant.maximum_capacity_bytes + grant.maximum_release_bytes <= budget);
            let (step, allocation) = observe_retirement_allocations(|| cursor.close_step(grant).unwrap());
            assert!(step.progress().fits(grant));assert!(allocation.0 <= step.progress().retained_capacity_bytes && allocation.1 <= step.progress().released_bytes);
            if copy > 0 {assert_eq!(allocation.1,0);}
            released += allocation.1;turns += 1;
            if matches!(step, RetainedCloneStep::Complete(_)) { closed = true;break; }
        }
        assert!(closed && cursor.terminal_is_empty());
        let (_, allocation) = observe_retirement_allocations(|| drop(cursor));assert_eq!(allocation, (0, 0));
        let mut retirement = controlled::ControlledRetirement::new(destination).unwrap_or_else(|_| panic!("native destination has direct retirement"));
        for turn in 0..100000 {
            let grant = match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(budget, usize::MAX), 1 => RetainedCloneGrant::one_payload_turn(budget, usize::MAX), _ => RetainedCloneGrant::one_release_turn(budget, usize::MAX) };
            let (step, allocation) = observe_retirement_allocations(|| retirement.step(grant).unwrap());
            assert!(step.progress().fits(grant));assert!(allocation.0 <= step.progress().retained_capacity_bytes && allocation.1 <= step.progress().released_bytes);
            if matches!(step, RetainedCloneStep::Complete(_)) { break; }
        }
        assert!(retirement.terminal_is_empty());
        let (_, allocation) = observe_retirement_allocations(|| drop(retirement));assert_eq!(allocation, (0, 0));
        eprintln!("[DEBUG] Native UTF8 append close partial={partial} exactTurns={turns} actualReleased={released}; zero heap demand/pause/undergrant and exact separately admitted birth/release");
    }
}

#[test]
fn paged_native_clone_close_admits_actual_birth_release_and_zero_grants() {
    use crate::retained_clone::{RetainedClone, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneStep, RetainedCloneSource};
    use crate::value::observe_retirement_allocations;
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../📦️paged/🧫️fixtures/🎮️native-owner/🔣️.json")).unwrap();
    let law = &corpus["cloneClose"];
    let budget = law["maximumBytes"].as_u64().unwrap() as usize;
    for pause in law["pauseAt"].as_array().unwrap().iter().map(|pause| pause.as_u64().unwrap() as usize).chain(std::iter::once(usize::MAX)) {
        let mut tree = ControlledTree::Leaf;
        for _ in 0..corpus["treeDepth"].as_u64().unwrap() { tree = ControlledTree::Node(corpus["prefix"].as_str().unwrap().repeat(9).into(), [tree].into_iter().collect()); }
        let mut objects = crate::paged::PagedMap::default();
        objects.insert(corpus["prefix"].as_str().unwrap().repeat(law["textRepeat"].as_u64().unwrap() as usize), law["objectValue"].as_str().unwrap().into());
        let source = RetainedCloneSource::from_owner(ControlledCloneRecord {
            tree,
            optional: Some((corpus["prefix"].as_str().unwrap().repeat(law["textRepeat"].as_u64().unwrap() as usize).into(), law["optionalValues"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u32).collect())),
            objects,
        });
        let oracle = serde_json::to_value(source.borrow().get()).unwrap();
        let (mut cursor, birth) = observe_retirement_allocations(ControlledCloneRecord::retained_clone_cursor);
        assert_eq!(birth, (0, 0));
        let (zero, allocation) = observe_retirement_allocations(|| cursor.advance(source.borrow(), RetainedCloneGrant::default()).unwrap());
        assert_eq!(zero.progress(), Default::default());
        assert_eq!(allocation, (0, 0));
        let mut output = None;
        let mut cloned = false;
        for turn in 0..100000 {
            if turn == pause { break; }
            let grant = match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(budget, usize::MAX), 1 => RetainedCloneGrant::one_payload_turn(budget, usize::MAX), _ => RetainedCloneGrant::one_release_turn(budget, usize::MAX) };
            let (step, allocation) = observe_retirement_allocations(|| cursor.advance(source.borrow(), grant).unwrap());
            let progress = step.progress();
            assert!(progress.fits(grant));
            assert!(allocation.0 <= progress.retained_capacity_bytes && allocation.1 <= progress.released_bytes, "clone: {allocation:?} {progress:?}");
            if matches!(step, RetainedCloneStep::Complete(_)) {
                let (owner, allocation) = observe_retirement_allocations(|| cursor.take());
                assert_eq!(allocation, (0, 0));
                output = owner;
                let (second, allocation) = observe_retirement_allocations(|| cursor.take());
                assert!(second.is_none());
                assert_eq!(allocation, (0, 0));
                cloned = true;
                break;
            }
        }
        if pause == usize::MAX { assert!(cloned && output.is_some(), "complete clone must transfer its actual native owner"); assert_eq!(serde_json::to_value(output.as_ref().unwrap()).unwrap(), oracle); }
        let (_, allocation) = observe_retirement_allocations(|| cursor.begin_close());
        assert_eq!(allocation, (0, 0));
        let (zero, allocation) = observe_retirement_allocations(|| cursor.close_step(RetainedCloneGrant::default()).unwrap());
        assert_eq!(zero.progress(), Default::default());
        assert_eq!(allocation, (0, 0));
        let mut complete = false;
        for turn in 0..100000 {
            let grant = match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(budget, usize::MAX), 1 => RetainedCloneGrant::one_payload_turn(budget, usize::MAX), _ => RetainedCloneGrant::one_release_turn(budget, usize::MAX) };
            let (step, allocation) = observe_retirement_allocations(|| cursor.close_step(grant).unwrap());
            let progress = step.progress();
            assert!(progress.fits(grant));
            assert!(allocation.0 <= progress.retained_capacity_bytes, "close birth: {allocation:?} {progress:?}");
            assert!(allocation.1 <= progress.released_bytes, "close release: {allocation:?} {progress:?}");
            if matches!(step, RetainedCloneStep::Complete(_)) { complete = true; break; }
        }
        assert!(complete && cursor.terminal_is_empty());
        let (_, allocation) = observe_retirement_allocations(|| drop(cursor));
        assert_eq!(allocation, (0, 0));
        if let Some(output) = output {
            let (result, allocation) = observe_retirement_allocations(|| controlled::ControlledRetirement::new(output));
            assert_eq!(allocation, (0, 0));
            let mut retirement = result.unwrap_or_else(|_| panic!("completed clone native owner must retire directly"));
            let mut closed = false;
            for turn in 0..100000 {
                let grant = match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(budget, usize::MAX), 1 => RetainedCloneGrant::one_payload_turn(budget, usize::MAX), _ => RetainedCloneGrant::one_release_turn(budget, usize::MAX) };
                let (step, allocation) = observe_retirement_allocations(|| retirement.step(grant).unwrap());
                let progress = step.progress();
                assert!(progress.fits(grant));
                assert!(allocation.0 <= progress.retained_capacity_bytes && allocation.1 <= progress.released_bytes, "transferred owner: {allocation:?} {progress:?}");
                if matches!(step, RetainedCloneStep::Complete(_)) { closed = true; break; }
            }
            assert!(closed && retirement.terminal_is_empty());
            let (_, allocation) = observe_retirement_allocations(|| drop(retirement));
            assert_eq!(allocation, (0, 0));
        }
    }
    eprintln!("[DEBUG] controlled clone close preserves zero grants and admits every actual scaffold birth and release within4096");
}

fn admit_fixture<T:RetireOwned>(value:T)->Box<dyn ErasedSnapshotRetirement> {
    let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:owned_retirement_birth_bytes::<T>(),maximum_release_bytes:0,maximum_depth:1};
    admit_owned_retirement(value,grant).unwrap_or_else(|(error,_)|panic!("fixture original owner admission: {error}")).0
}
fn admit_shared_fixture<T:RetireOwned+Sync>(value:Arc<T>,lease:bool)->Box<dyn ErasedSnapshotRetirement> {
    let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:shared::shared_retirement_birth_bytes::<T>(),maximum_release_bytes:0,maximum_depth:1};
    shared::admit_shared_retirement(value,grant,lease).unwrap_or_else(|(error,_)|panic!("fixture original lease admission: {error}")).0
}
fn drain(retirement:Box<dyn ErasedSnapshotRetirement>,items:usize,bytes:usize)->usize {
    let mut slot=Some(retirement);let(mut born,mut freed)=(0,0);
    for _ in 0..100_000 {
        let Some(owner)=slot.as_ref()else{return freed-born;};
        let (demand,heap)=crate::value::observe_retirement_allocations(||crate::factory_ticket_demands(owner,bytes).unwrap());
        assert_eq!(heap,(0,0));
        let grant=RetainedCloneGrant {maximum_items:items,maximum_copy_bytes:bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
        let (step,heap)=crate::value::observe_retirement_allocations(||crate::close_factory_ticket(&mut slot,grant).unwrap());
        assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));
        born+=heap.0;freed+=heap.1;
    }
    panic!("bounded original fixture retirement did not finish");
}
fn physical_owned<T:RetireOwned>(value:T,items:usize,bytes:usize)->usize {
    let (retirement,heap)=crate::value::observe_retirement_allocations(||admit_fixture(value));
    drain(retirement,items,bytes)-(heap.0-heap.1)
}
fn neutral_logical_bytes(value:&serde_json::Value)->usize {
    match value {serde_json::Value::Null=>0,serde_json::Value::Bool(_)=>1,serde_json::Value::Number(_)=>8,serde_json::Value::String(value)=>value.len(),serde_json::Value::Array(values)=>values.iter().map(neutral_logical_bytes).sum(),serde_json::Value::Object(values)=>values.iter().map(|(key,value)|key.len()+neutral_logical_bytes(value)).sum()}
}
#[test]
fn owned_retirement_matches_neutral_exact_byte_grants() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();assert_eq!(fixture["cases"].as_array().unwrap().len(),11);
    for row in fixture["cases"].as_array().unwrap(){for budget in fixture["budgets"].as_array().unwrap(){
        let (retirement,heap)=crate::value::observe_retirement_allocations(||match row["kind"].as_str().unwrap(){
            "string"=>admit_fixture(row["value"].as_str().unwrap().to_owned()),
            "strings"=>admit_fixture(serde_json::from_value::<Vec<String>>(row["value"].clone()).unwrap()),
            "optionalString"=>admit_fixture(serde_json::from_value::<Option<String>>(row["value"].clone()).unwrap()),
            "pair"=>admit_fixture(serde_json::from_value::<(String,String)>(row["value"].clone()).unwrap()),
            "stringMap"=>admit_fixture(serde_json::from_value::<crate::ordered::OrderedMap<String>>(row["value"].clone()).unwrap()),
            "value"=>admit_fixture(crate::DslValue::from(&row["value"])),
            "bytes"=>admit_fixture(serde_json::from_value::<Vec<u8>>(row["value"].clone()).unwrap()),
            "words"=>admit_fixture(serde_json::from_value::<Vec<u32>>(row["value"].clone()).unwrap()),
            _=>panic!("unknown neutral case"),
        });
        let logical=match row["kind"].as_str().unwrap(){"bytes"=>row["value"].as_array().unwrap().len(),"words"=>row["value"].as_array().unwrap().len()*size_of::<u32>(),_=>neutral_logical_bytes(&row["value"])};
        assert_eq!(logical,row["bytes"].as_u64().unwrap()as usize,"{} logical oracle",row["id"]);
        assert_eq!(drain(retirement,budget["items"].as_u64().unwrap()as usize,budget["bytes"].as_u64().unwrap()as usize),heap.0-heap.1,"{} physical conservation",row["id"]);
        println!("[DEBUG] neutral full retirement id={} logical={logical} original work={} retained net={} exact heap conservation",row["id"],budget["bytes"],heap.0-heap.1);
    }}
}

/// ♻️ Preserves the original64KiB logical quantum and36 payload-turn ceiling with separately admitted scaffolds.
#[test]
fn a_byte_buffer_retires_page_by_page_and_owned_elements_one_by_one() {
    let (owner,heap)=crate::value::observe_retirement_allocations(||admit_fixture(vec![7u8;2*1024*1024]));let original=heap.0-heap.1;
    let mut slot=Some(owner);let(mut payload_turns,mut born,mut released,mut copied)=(0usize,0usize,0usize,0usize);
    for _ in 0..100_000 {
        let Some(owner)=slot.as_ref()else{break;};let demand=crate::factory_ticket_demands(owner,64*1024).unwrap();
        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:64*1024,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
        let (step,heap)=crate::value::observe_retirement_allocations(||crate::close_factory_ticket(&mut slot,grant).unwrap());let progress=step.progress();
        assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));assert!(progress.fits(grant));born+=heap.0;released+=heap.1;copied+=progress.copied_bytes;
        payload_turns+=usize::from(progress.copied_bytes!=0||progress.released_bytes==2*1024*1024);
        assert!(payload_turns<=36,"the original2MiB payload requires at most36 turns of64KiB; scaffold authority is separate");
    }
    assert!(slot.is_none());assert_eq!(copied,2*1024*1024);assert_eq!(released,original+born);
    assert_eq!(physical_owned(vec![1u32,2,3],1,4),12);assert_eq!(physical_owned(vec![1u32,2,3],1,3),12,"work narrower than one element retains each remaining byte");
    assert_eq!(physical_owned(vec!["ab".to_string(),"c".to_string()],1,1),3+2*size_of::<String>());
}

#[test]
fn owned_retirement_rejects_false_terminal_and_preserves_shared_roots() {
    let root=Arc::new("owned".to_string());let mut shared=admit_shared_fixture(Arc::clone(&root),false);
    assert_eq!(shared.close_step(RetainedCloneGrant::default()).unwrap().progress(),Default::default());
    let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:1,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};
    assert_eq!(shared.close_step(grant).unwrap().progress(),Default::default());assert_eq!(Arc::strong_count(&root),2);drop(root);
    assert_eq!(drain(shared,1,1),5+shared::shared_retirement_birth_bytes::<String>()+std::alloc::Layout::new::<[usize;2]>().extend(std::alloc::Layout::new::<String>()).unwrap().0.pad_to_align().size());
    let mut value=admit_fixture("zero".to_string());for _ in 0..4{assert_eq!(value.close_step(grant).unwrap().progress(),Default::default());}
    assert!(!value.terminal_is_empty());assert_eq!(drain(value,1,2),4+owned_retirement_birth_bytes::<String>());
    struct Hostile {mode:Option<Arc<std::sync::atomic::AtomicU8>>}
    impl RetirementCursor for Hostile {
        fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep {
            match self.mode.as_ref().unwrap().load(std::sync::atomic::Ordering::Relaxed) {1=>RetirementStep::Bytes(grant.maximum_release_bytes+1),2=>{drop(self.mode.take());RetirementStep::Advanced},_=>RetirementStep::Complete}
        }
        fn terminal_is_empty(&self)->bool {self.mode.is_none()}
        fn next_birth_bytes(&self,_:usize)->Option<usize> {Some(0)}
        fn next_close_byte_demand(&self)->Option<usize> {Some(0)}
        fn terminal_release_bytes(&self)->Option<usize> {self.terminal_is_empty().then_some(size_of::<Self>())}
    }
    impl RetireOwned for Hostile {
        fn retirement(self)->Box<dyn RetirementCursor> {Box::new(self)}
        fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<Self>())}
        fn controlled_retirement_supported()->bool {true}
    }
    for mode in [0u8,1] {
        let control=Arc::new(std::sync::atomic::AtomicU8::new(mode));let mut owner=controlled::ControlledRetirement::new(Hostile {mode:Some(control.clone())}).unwrap_or_else(|_|panic!("hostile fixture owns an admitted exact frame"));let mut refused=false;
        for _ in 0..8 {
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:1,maximum_capacity_bytes:owner.next_capacity_byte_demand(1).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
            if owner.step(grant).is_err(){refused=true;break;}
        }
        assert!(refused);assert!(!owner.terminal_is_empty());assert_eq!(Arc::strong_count(&control),2);
        control.store(2,std::sync::atomic::Ordering::Relaxed);
        for _ in 0..16 {if owner.terminal_is_empty(){break;}let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:1,maximum_capacity_bytes:owner.next_capacity_byte_demand(1).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};owner.step(grant).unwrap();}
        assert!(owner.terminal_is_empty());assert_eq!(Arc::strong_count(&control),1);
    }
}
#[test]
fn shared_source_leases_release_all_orders_with_one_bounded_owner() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let law=&fixture["leases"];let text=law["text"].as_str().unwrap();let oracle:serde_json::Value=serde_json::from_str(&serde_json::to_string(text).unwrap()).unwrap();assert_eq!(oracle.as_str().unwrap().len(),text.len());
    for canceled in law["canceled"].as_array().unwrap(){for order in law["orders"].as_array().unwrap(){
        let source=Arc::new(text.to_owned());let mut leases=vec![Some(Arc::clone(&source)),Some(Arc::clone(&source)),Some(source)];let mut released=0;
        for (position,index) in order.as_array().unwrap().iter().enumerate(){
            let alias=leases[index.as_u64().unwrap()as usize].take().unwrap();let mut retirement=admit_shared_fixture(alias,true);
            assert_eq!(retirement.close_step(RetainedCloneGrant::default()).unwrap().progress(),Default::default());assert!(!retirement.terminal_is_empty());
            assert_eq!(retirement.close_step(RetainedCloneGrant {maximum_items:0,maximum_copy_bytes:3,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1}).unwrap().progress(),Default::default());
            let bytes=drain(retirement,law["items"].as_u64().unwrap()as usize,law["bytes"].as_u64().unwrap()as usize);
            assert_eq!(bytes,shared::shared_retirement_birth_bytes::<String>()+if position==2{text.len()+std::alloc::Layout::new::<[usize;2]>().extend(std::alloc::Layout::new::<String>()).unwrap().0.pad_to_align().size()}else{0},"cancel={canceled} order={order}");released+=bytes;
        }
        assert_eq!(released,text.len()+3*shared::shared_retirement_birth_bytes::<String>()+std::alloc::Layout::new::<[usize;2]>().extend(std::alloc::Layout::new::<String>()).unwrap().0.pad_to_align().size());assert!(leases.iter().all(Option::is_none));
    }}
    eprintln!("[DEBUG] full source leases all six release orders and canceled/completed laws; one bounded original source release");
}

#[test]
fn native_controls_resume_same_cumulative_admission_and_cancel_before_more_work() {
    use crate::{NativeDecodeControl,NativeEncodeControl};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let law=&fixture["continuation"];let maximum=law["maximumBytes"].as_u64().unwrap()as usize;
    let mut accepted=|_|true;let mut encode=NativeEncodeControl::new(maximum,&mut accepted);encode.begin_stage(9).unwrap();encode.charge(5).unwrap();encode.advance(4).unwrap();let receipt=encode.pause().unwrap();
    let event=std::cell::Cell::new(None);let mut resumed=|value|{event.set(Some(value));true};let mut encode=NativeEncodeControl::resume(receipt,&mut resumed).unwrap();encode.checkpoint().unwrap();assert_eq!(event.get().unwrap().completed,4);assert_eq!(event.get().unwrap().total,9);assert_eq!(encode.owned_bytes(),5);encode.charge(8).unwrap();encode.advance(5).unwrap();assert_eq!(encode.owned_bytes(),maximum);assert_eq!(encode.charge(1).unwrap_err().kind,crate::ValueRefusalKind::OwnershipLimit);let receipt=encode.pause().unwrap();let mut canceled=|_|false;let mut encode=NativeEncodeControl::resume(receipt,&mut canceled).unwrap();assert_eq!(encode.step().unwrap_err().kind,crate::ValueRefusalKind::Canceled);assert_eq!(encode.owned_bytes(),maximum);
    let mut accepted=|_|true;let mut decode=NativeDecodeControl::new(maximum,&mut accepted);decode.begin_stage(9).unwrap();decode.charge(5).unwrap();decode.advance(4).unwrap();let receipt=decode.pause().unwrap();
    let event=std::cell::Cell::new(None);let mut resumed=|value|{event.set(Some(value));true};let mut decode=NativeDecodeControl::resume(receipt,&mut resumed).unwrap();decode.checkpoint().unwrap();assert_eq!(event.get().unwrap().completed,4);assert_eq!(event.get().unwrap().total,9);assert_eq!(decode.owned_bytes(),5);decode.charge(8).unwrap();decode.advance(5).unwrap();assert_eq!(decode.charge(1).unwrap_err().kind,crate::ValueRefusalKind::OwnershipLimit);let receipt=decode.pause().unwrap();let mut canceled=|_|false;let mut decode=NativeDecodeControl::resume(receipt,&mut canceled).unwrap();assert_eq!(decode.step().unwrap_err().kind,crate::ValueRefusalKind::Canceled);assert_eq!(decode.owned_bytes(),maximum);
    eprintln!("[DEBUG] encoding/decoding resume own receipt; cumulative13bytes; fresh callback cancellation");
}

#[test]
fn native_encoding_capacity_admission_consumes_the_same_counter_owner(){
    use crate::{NativeEncodeControl,ValueRefusalKind};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let law=&fixture["capacityAdmission"];let prior=&fixture["continuation"];
    let source=Arc::new(law["source"].as_str().unwrap().to_owned());let pointer=source.as_ptr();let reference:serde_json::Value=serde_json::from_str(&serde_json::to_string(&*source).unwrap()).unwrap();assert_eq!(reference.as_str().unwrap().len(),law["sourceBytes"].as_u64().unwrap()as usize);
    let event=std::cell::Cell::new(None);let mut accepted=|value|{assert_eq!(source.as_ptr(),pointer);event.set(Some(value));true};let mut control=NativeEncodeControl::new(prior["maximumBytes"].as_u64().unwrap()as usize,&mut accepted);control.begin_stage(prior["total"].as_u64().unwrap()as usize).unwrap();control.charge(prior["initialBytes"].as_u64().unwrap()as usize).unwrap();control.advance(prior["initialUnits"].as_u64().unwrap()as usize).unwrap();control.checkpoint().unwrap();let before=event.get().unwrap();
    let (mut control,error)=match control.admit_capacity(law["refusedBytes"].as_u64().unwrap()as usize,1,0){Err(rejection)=>rejection,Ok(_)=>panic!("existing ownership cannot fit refused capacity")};assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);assert_eq!(control.maximum_bytes(),prior["maximumBytes"].as_u64().unwrap()as usize);control.checkpoint().unwrap();assert_eq!(event.get().unwrap(),before);
    let mut control=match control.admit_capacity(law["sourceBytes"].as_u64().unwrap()as usize,law["sourceMultiples"].as_u64().unwrap()as usize,law["scaffoldBytes"].as_u64().unwrap()as usize){Ok(control)=>control,Err(_)=>panic!("source policy capacity admitted")};assert_eq!(control.maximum_bytes(),law["maximumBytes"].as_u64().unwrap()as usize);control.checkpoint().unwrap();assert_eq!(event.get().unwrap(),before);
    let maximum=control.maximum_bytes();control.charge(maximum-control.owned_bytes()).unwrap();control.advance(prior["finalUnits"].as_u64().unwrap()as usize).unwrap();assert_eq!(control.charge(law["overrunBytes"].as_u64().unwrap()as usize).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);control.checkpoint().unwrap();let complete=event.get().unwrap();assert_eq!(complete.completed,prior["total"].as_u64().unwrap()as usize);assert_eq!(complete.owned_bytes,maximum);let receipt=control.pause().unwrap();
    let canceled_event=std::cell::Cell::new(None);let mut canceled=|value|{assert_eq!(source.as_ptr(),pointer);canceled_event.set(Some(value));false};let mut control=NativeEncodeControl::resume(receipt,&mut canceled).unwrap();assert_eq!(control.step().unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!(canceled_event.get().unwrap(),complete);drop(control);assert_eq!(source.as_ptr(),pointer);assert_eq!(drain(admit_shared_fixture(source,true),1,3),law["sourceBytes"].as_u64().unwrap()as usize+shared::shared_retirement_birth_bytes::<String>()+std::alloc::Layout::new::<[usize;2]>().extend(std::alloc::Layout::new::<String>()).unwrap().0.pad_to_align().size());
    eprintln!("[DEBUG] consuming source capacity admission kept exact owned/completed/total counters and source lease identity; refusal preserved owner; one-byte overrun and resumed cancellation");
}

#[test]
fn native_capacity_closed_vectors_preserve_the_consumed_admission_owner(){
    use crate::{NativeEncodeControl,ValueRefusalKind};
    fn number(value:&serde_json::Value)->usize{match value.as_str().unwrap(){"usizeMax"=>usize::MAX,"isizeMax"=>isize::MAX as usize,decimal=>decimal.parse().unwrap()}}
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let cases=fixture["capacityAdmission"]["cases"].as_array().unwrap();assert_eq!(cases.len(),12);
    for case in cases {
        let calls=std::cell::Cell::new(0);let event=std::cell::Cell::new(None);
        let mut callback=|value|{calls.set(calls.get()+1);event.set(Some(value));true};
        let mut control=NativeEncodeControl::new(13,&mut callback);control.begin_stage(9).unwrap();control.charge(number(&case["ownedBytes"])).unwrap();control.advance(4).unwrap();control.checkpoint().unwrap();
        let before=event.get().unwrap();let before_calls=calls.get();
        let result=control.admit_capacity(number(&case["sourceBytes"]),number(&case["multiples"]),number(&case["scaffoldBytes"]));
        assert_eq!(calls.get(),before_calls,"{}",case["id"]);
        let mut control=if case["accepted"].as_bool().unwrap(){let control=match result{Ok(control)=>control,Err(_)=>panic!("capacity unexpectedly refused: {}",case["id"])};assert_eq!(control.maximum_bytes(),number(&case["maximumBytes"]));control}else{let(control,error)=match result{Err(refusal)=>refusal,Ok(_)=>panic!("capacity unexpectedly admitted: {}",case["id"])};assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);assert_eq!(control.maximum_bytes(),13);control};
        assert_eq!(control.owned_bytes(),before.owned_bytes);control.checkpoint().unwrap();assert_eq!(event.get().unwrap(),before);control.advance(5).unwrap();let complete=event.get().unwrap();assert_eq!(complete.completed,9);assert_eq!(complete.total,9);assert_eq!(complete.owned_bytes,before.owned_bytes);
        let receipt=control.pause().unwrap();let canceled_event=std::cell::Cell::new(None);let mut canceled=|value|{canceled_event.set(Some(value));false};let mut control=NativeEncodeControl::resume(receipt,&mut canceled).unwrap();assert_eq!(control.step().unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!(canceled_event.get().unwrap(),complete);assert_eq!(control.owned_bytes(),before.owned_bytes);
    }
    eprintln!("[DEBUG] native source capacity twelve closed vectors preserve owner/counters/callback; checked overflow, signed ceiling, refusal and resumed cancellation");
}

use crate::{ValueRefusalKind,retirement::allocation_return::{ParentAllocationReturn,AllocationReturnStep}};
#[test]
fn parent_return_fixture_demands_actual_full_allocation_release(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../📦️allocation-return/🧫️fixtures/📦️return.json")).unwrap();
    let maximum=fixture["maximumAllocationBytes"].as_u64().unwrap()as usize;let total=fixture["maximumTotalBytes"].as_u64().unwrap()as usize;
    let mut parent=ParentAllocationReturn::<2>::try_new(maximum,total).unwrap();let mut text=fixture["text"].as_str().unwrap().to_owned();let text_pointer=text.as_ptr();let text_capacity=text.capacity();assert_eq!(text_capacity,fixture["expectedTextBytes"].as_u64().unwrap()as usize);
    assert!(!parent.return_text(&mut text,0).unwrap());assert_eq!(text.as_ptr(),text_pointer);assert_eq!(text,fixture["text"].as_str().unwrap());assert!(parent.terminal_is_empty());
    assert!(parent.return_text(&mut text,1).unwrap());assert!(text.is_empty());assert_eq!(text.capacity(),0);assert!(!parent.terminal_is_empty());assert_eq!(parent.retained_bytes(),text_capacity);
    let mut values=Vec::<u32>::with_capacity(fixture["emptyVectorCapacity"].as_u64().unwrap()as usize);let vector_capacity=values.capacity();assert_eq!(vector_capacity*std::mem::size_of::<u32>(),fixture["expectedVectorBytes"].as_u64().unwrap()as usize);let vector_pointer=values.as_ptr().cast::<u8>();values.push(9);
    assert_eq!(parent.return_empty_vec(&mut values,1).unwrap_err().kind,ValueRefusalKind::InvariantViolated);assert_eq!(values,[9]);assert_eq!(values.as_ptr().cast::<u8>(),vector_pointer);values.clear();assert!(parent.return_empty_vec(&mut values,1).unwrap());assert_eq!(values.capacity(),0);
    let mut occupied="member".to_owned();let occupied_pointer=occupied.as_ptr();assert!(!parent.return_text(&mut occupied,1).unwrap());assert_eq!(occupied.as_ptr(),occupied_pointer);assert_eq!(occupied,"member");
    let owned=parent.retained_bytes();assert_eq!(parent.close_step(0,maximum),AllocationReturnStep::Pending{released_items:0,released_bytes:0});assert_eq!(parent.close_step(1,fixture["childBytes"].as_u64().unwrap()as usize),AllocationReturnStep::Pending{released_items:0,released_bytes:0});assert_eq!(parent.retained_bytes(),owned);assert!(!parent.terminal_is_empty());
    let mut released=0;for _ in 0..3{match parent.close_step(1,maximum){AllocationReturnStep::Complete=>break,AllocationReturnStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1&&released_bytes<=maximum);released+=released_bytes;}}}assert_eq!(released,fixture["expectedTextBytes"].as_u64().unwrap()as usize+fixture["expectedVectorBytes"].as_u64().unwrap()as usize);assert_eq!(parent.retained_bytes(),0);assert!(parent.terminal_is_empty());
    let mut oversized="x".repeat(fixture["refusedPayloadBytes"].as_u64().unwrap()as usize);let pointer=oversized.as_ptr();let capacity=oversized.capacity();assert_eq!(parent.return_text(&mut oversized,1).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(oversized.as_ptr(),pointer);assert_eq!(oversized.capacity(),capacity);assert_eq!(oversized.len(),fixture["refusedPayloadBytes"].as_u64().unwrap()as usize);assert!(parent.terminal_is_empty());
    assert_eq!(serde_json::to_string(&oversized).unwrap(),serde_json::to_string(&"x".repeat(fixture["refusedPayloadBytes"].as_u64().unwrap()as usize)).unwrap());
    eprintln!("[DEBUG] genuine parent flat-allocation tokens preserve actual source pointers/layouts, zero/full-slot/4byte refusal and terminal=false until physical full-grant deallocation; original8194 exceeds4096 authority unchanged");
}

#[test]
fn controlled_retirement_separates_logical_work_from_exact_physical_release() {
    use crate::{retained_clone::{RetainedCloneGrant,RetainedCloneStep},value::observe_retirement_allocations};
    fn run<T:RetireOwned+serde::Serialize>(value:T,row:&serde_json::Value,grant:RetainedCloneGrant) {
        assert_eq!(serde_json::to_value(&value).unwrap(),row["value"]);
        let mut owner=controlled::ControlledRetirement::new(value).unwrap_or_else(|_|panic!("controlled original owner"));
        let mut processed=0;let mut physical=0;
        for _ in 0..10000 {
            let capacity=owner.next_capacity_byte_demand(grant.maximum_copy_bytes).unwrap();
            let release=owner.next_release_byte_demand().unwrap();
            let (zero,heap)=observe_retirement_allocations(||owner.step(RetainedCloneGrant::default()).unwrap());
            assert_eq!(zero.progress(),Default::default());assert_eq!(heap,(0,0));
            if release>0 && capacity==0 {
                let (denied,heap)=observe_retirement_allocations(||owner.step(RetainedCloneGrant{maximum_release_bytes:release-1,..grant}).unwrap());
                assert_eq!(denied.progress(),Default::default());assert_eq!(heap,(0,0));
            }
            let (step,heap)=observe_retirement_allocations(||owner.step(grant).unwrap());
            assert!(step.progress().fits(grant));
            assert_eq!(heap.0,step.progress().retained_capacity_bytes);
            assert_eq!(heap.1,step.progress().released_bytes,"logical work cannot impersonate physical release: {}",row["id"]);
            processed+=step.progress().copied_bytes;physical+=step.progress().released_bytes;
            if matches!(step,RetainedCloneStep::Complete(_)){break;}
        }
        assert!(owner.terminal_is_empty());assert_eq!(processed,row["processedBytes"].as_u64().unwrap()as usize);
        assert_eq!(observe_retirement_allocations(||drop(owner)).1,(0,0));
        println!("[DEBUG] controlled retirement work/release id={} processed={processed} exact-allocator-free={physical}",row["id"]);
    }
    let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/⚖️physical-work/🔣️.json")).unwrap();
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:64};
    for row in law["cases"].as_array().unwrap(){match row["kind"].as_str().unwrap(){
        "u32"=>run(row["value"].as_u64().unwrap()as u32,row,grant),
        "text"=>run(row["value"].as_str().unwrap().to_owned(),row,grant),
        "u32-list"=>run(row["value"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u32).collect::<Vec<_>>(),row,grant),
        _=>panic!("unknown neutral owner"),
    }}
}

#[test]
fn controlled_retirement_separates_logical_work_from_exact_physical_release_cold_terminal_frame() {
    use crate::value::observe_retirement_allocations;
    let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/⚖️physical-work/🔣️.json")).unwrap();
    let text=law["cases"][1]["value"].as_str().unwrap();
    let (cursor,heap)=observe_retirement_allocations(||admit_fixture(text.to_owned()));let original=heap.0-heap.1;
    let frame=std::mem::size_of_val(cursor.as_ref());assert_eq!(frame,owned_retirement_birth_bytes::<String>());
    let mut slot=Some(cursor);let(mut born,mut physical,mut copied,mut terminal_frame)=(0,0,0,0);
    for _ in 0..10000 {
        let Some(owner)=slot.as_ref()else{break;};let terminal=owner.terminal_is_empty();
        let demand=crate::factory_ticket_demands(owner,64).unwrap();assert!(demand.capacity_bytes<=4096&&demand.release_bytes<=4096);
        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
        for denied in [demand.capacity_bytes.checked_sub(1).map(|value|RetainedCloneGrant {maximum_capacity_bytes:value,..grant}),demand.release_bytes.checked_sub(1).map(|value|RetainedCloneGrant {maximum_release_bytes:value,..grant})].into_iter().flatten(){
            let (step,heap)=observe_retirement_allocations(||crate::close_factory_ticket(&mut slot,denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));
            assert_eq!(crate::factory_ticket_demands(slot.as_ref().unwrap(),64).unwrap(),demand);
        }
        let (step,heap)=observe_retirement_allocations(||crate::close_factory_ticket(&mut slot,grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.0;physical+=heap.1;copied+=progress.copied_bytes;
        if terminal{assert_eq!(heap,(0,frame));terminal_frame+=1;}
    }
    assert!(slot.is_none());assert_eq!(terminal_frame,1);assert_eq!(copied,0);assert_eq!(physical,original+born);
    println!("[DEBUG] full typed retirement original UTF8 work physical0; all scaffolds born={born}/released={physical}; exact outer frame={frame}");
}

#[test]
fn shared_retirement_physical_demand_preserves_original_backing_and_terminal_frames() {
    use crate::value::observe_retirement_allocations;
    let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/📏️shared-physical/🔣️.json")).unwrap();
    for row in law["cases"].as_array().unwrap(){for copy in law["workBytes"].as_array().unwrap(){
        let copy=copy.as_u64().unwrap()as usize;let mut text=String::with_capacity(row["capacity"].as_u64().unwrap()as usize);text.push_str(row["text"].as_str().unwrap());assert_eq!(serde_json::to_value(&text).unwrap(),row["text"]);let backing=text.capacity();
        let root=Arc::new(text);let pointer=Arc::as_ptr(&root);let (owner,heap)=observe_retirement_allocations(||admit_shared_fixture(root,true));let mut born=heap.0;assert_eq!(heap.1,0);let mut slot=Some(owner);
        let required=std::alloc::Layout::new::<[usize;2]>().extend(std::alloc::Layout::new::<String>()).unwrap().0.pad_to_align().size();
        let (query,heap)=observe_retirement_allocations(||crate::factory_ticket_demands(slot.as_ref().unwrap(),copy).unwrap());assert_eq!(heap,(0,0));assert_eq!(query.release_bytes,required);assert_eq!(query.capacity_bytes,0);
        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:0,maximum_release_bytes:required,maximum_depth:query.depth};
        let (denied,heap)=observe_retirement_allocations(||crate::close_factory_ticket(&mut slot,RetainedCloneGrant {maximum_release_bytes:required-1,..grant}).unwrap());assert_eq!(denied.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(crate::factory_ticket_demands(slot.as_ref().unwrap(),copy).unwrap(),query);
        let (step,heap)=observe_retirement_allocations(||crate::close_factory_ticket(&mut slot,grant).unwrap());assert_eq!(heap,(0,required));assert_eq!(step.progress().released_bytes,required);assert_eq!(step.progress().retained_capacity_bytes,0);
        let(mut physical,mut copied)=(required,0);
        for _ in 0..law["maximumTurns"].as_u64().unwrap(){
            let Some(owner)=slot.as_ref()else{break;};let demand=crate::factory_ticket_demands(owner,copy).unwrap();
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
            let (step,heap)=observe_retirement_allocations(||crate::close_factory_ticket(&mut slot,grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.0;physical+=heap.1;copied+=progress.copied_bytes;
        }
        assert!(slot.is_none());assert_eq!(copied,0);assert_eq!(physical,backing+required+born);
        println!("[DEBUG] shared original backing row={} pointer={pointer:p} copy={copy} logical={copied} independent birth={born} physical={physical}",row["id"]);
    }}
}
#[test]
fn erased_snapshot_full_grant_admission_preserves_original_owner_and_all_four_currencies() {
    use crate::retained_clone::RetainedCloneProgress;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📏️shared-physical/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let text = row["text"].as_str().unwrap();
        for work in [1usize, 3, 5, 4096] {
            let ((source, pointer, original), heap) = crate::observe_retirement_allocations(|| {
                let mut source = String::with_capacity(row["capacity"].as_u64().unwrap() as usize);
                source.push_str(text);
                let pointer = source.as_ptr();
                let original = source.capacity();
                (source, pointer, original)
            });
            assert_eq!(heap, (original, 0));
            let frame = crate::owned_retirement_birth_bytes::<String>();
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: work, maximum_capacity_bytes: frame - 1, maximum_release_bytes: 0, maximum_depth: 64 };
            let (denied, heap) = crate::observe_retirement_allocations(|| crate::admit_owned_retirement(source, grant));
            let (_, source) = denied.err().expect("one-short erased frame grant must return original ownership");
            assert_eq!(source.as_ptr(), pointer);
            assert_eq!(heap, (0, 0));
            let (admitted, heap) = crate::observe_retirement_allocations(|| crate::admit_owned_retirement(source, RetainedCloneGrant { maximum_capacity_bytes: frame, ..grant }));
            let (owner, admission) = admitted.unwrap_or_else(|(error, _)| panic!("{error}"));
            assert_eq!(heap, (frame, 0));
            assert_eq!(admission.retained_capacity_bytes, frame);
            let mut slot = Some(owner);
            let (mut copied, mut born, mut released) = (0usize, frame, 0usize);
            for _ in 0..4096 {
                let Some(owner) = slot.as_ref() else { break; };
                let capacity = owner.next_capacity_byte_demand(work).unwrap();
                let release = if owner.terminal_is_empty() { std::mem::size_of_val(owner.as_ref()) } else { owner.next_release_byte_demand().unwrap() };
                let depth = if owner.terminal_is_empty() { 1 } else { owner.next_depth_demand().unwrap() };
                let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: work, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: depth };
                let (step, heap) = crate::observe_retirement_allocations(|| crate::close_factory_ticket(&mut slot, RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap());
                assert_eq!(step.progress(), RetainedCloneProgress::default());
                assert_eq!(heap, (0, 0));
                for short in [capacity.checked_sub(1).map(|value| RetainedCloneGrant { maximum_capacity_bytes: value, ..grant }), release.checked_sub(1).map(|value| RetainedCloneGrant { maximum_release_bytes: value, ..grant })].into_iter().flatten() {
                    let (step, heap) = crate::observe_retirement_allocations(|| crate::close_factory_ticket(&mut slot, short).unwrap());
                    assert_eq!(step.progress(), RetainedCloneProgress::default());
                    assert_eq!(heap, (0, 0));
                }
                let (step, heap) = crate::observe_retirement_allocations(|| crate::close_factory_ticket(&mut slot, grant).unwrap());
                let progress = step.progress();
                assert!(progress.copied_items <= 1 && progress.copied_bytes <= work);
                assert_eq!(heap, (progress.retained_capacity_bytes, progress.released_bytes));
                copied += progress.copied_bytes;
                born += progress.retained_capacity_bytes;
                released += progress.released_bytes;
            }
            assert!(slot.is_none());
            assert_eq!(copied, 0);
            assert_eq!(released, original + born);
            eprintln!("[DEBUG] erased full-grant original-pointer admitted work={work} copy={copied} births={born} physical={released}");
        }
    }
}

/// 🧺️ Original heap and Reverse use the same vector allocation through each independently granted physical turn.
#[test]
fn original_heap_and_reverse_preserve_backing_and_full_retirement_receipts(){
    use crate::{value::observe_retirement_allocations as observe,retained_clone::RetainedCloneGrant};
    use std::{collections::BinaryHeap,cmp::Reverse};
    let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧺️heap.json")).unwrap();
    for copy in law["copyGrants"].as_array().unwrap(){
        let (owner,(source_birth,source_release))=observe(||{let mut heap=BinaryHeap::new();for key in law["keys"].as_array().unwrap(){let mut value=String::with_capacity(law["capacityBytes"].as_u64().unwrap() as usize);value.push_str(key.as_str().unwrap());heap.push(Reverse(value));}heap});
        let backing=owner.as_slice().as_ptr();let pointers:Vec<_>=owner.iter().map(|key|key.0.as_ptr()).collect();let mut oracle=owner.clone();let expected:Vec<_>=std::iter::from_fn(||oracle.pop().map(|key|key.0)).collect();assert_eq!(serde_json::to_value(expected).unwrap(),law["expected"]);
        let ((mut retirement),(birth,release))=observe(||controlled::ControlledRetirement::new(owner));assert_eq!((birth,release),(law["handoffCapacityBytes"].as_u64().unwrap() as usize,law["handoffReleaseBytes"].as_u64().unwrap() as usize));let mut retirement=retirement.unwrap_or_else(|_|panic!("original heap/Reverse must declare its actual retained vector authority"));assert_eq!(retirement.original().unwrap().as_slice().as_ptr(),backing);assert_eq!(retirement.original().unwrap().iter().map(|key|key.0.as_ptr()).collect::<Vec<_>>(),pointers);
        let(mut born,mut freed,mut refused,mut turns)=(0,0,0,0);
        while !retirement.terminal_is_empty(){let copy=copy.as_u64().unwrap() as usize;let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:retirement.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:retirement.next_release_byte_demand().unwrap(),maximum_depth:retirement.next_depth_demand().unwrap()};
            if grant.maximum_release_bytes>law["maximumDeniedReleaseBytes"].as_u64().unwrap() as usize{for _ in 0..2{let (step,heap)=observe(||retirement.step(RetainedCloneGrant {maximum_release_bytes:law["maximumDeniedReleaseBytes"].as_u64().unwrap() as usize,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(retirement.next_release_byte_demand().unwrap(),grant.maximum_release_bytes);refused+=1;}}
            let (zero,heap)=observe(||retirement.step(RetainedCloneGrant {maximum_items:0,..grant}).unwrap());assert_eq!(zero.progress(),Default::default());assert_eq!(heap,(0,0));
            let (step,(birth,release))=observe(||retirement.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((birth,release),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=birth;freed+=release;turns+=1;assert!(turns<100000);
        }
        assert!(refused>0);assert_eq!(source_birth-source_release+born,freed);let (_,heap)=observe(||drop(retirement));assert_eq!(heap,(0,law["terminalDropBytes"].as_u64().unwrap() as usize));eprintln!("[DEBUG] Original heap/Reverse sameBacking=true sameKeys=true handoffBirth=0 handoffFree=0 copy={copy} source={} admitted={born} physical={freed} eightByteRefusals={refused} turns={turns} terminalDrop=0",source_birth-source_release);
    }
}

#[test]
fn original_five_field_tuple_retirement_conserves_every_native_owner_and_receipt() {
    use crate::{retained_clone::RetainedCloneGrant,value::observe_retirement_allocations as observe};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪆️five-fields/🔣️.json")).unwrap();
    type Mesh=(String,Vec<f32>,Vec<f32>,Vec<u32>,Vec<f32>);
    let (source,original_heap)=observe(||serde_json::from_value::<Mesh>(fixture["fields"].clone()).unwrap());
    assert_eq!(serde_json::to_value(&source).unwrap(),fixture["fields"]);
    let pointers=(source.0.as_ptr(),source.1.as_ptr(),source.2.as_ptr(),source.3.as_ptr(),source.4.as_ptr());
    let (result,heap)=observe(||controlled::ControlledRetirement::new(source));
    assert_eq!(heap,(0,0));
    let mut owner=result.map_err(|(error,_)|error).unwrap();
    let original=owner.original().unwrap();
    assert_eq!((original.0.as_ptr(),original.1.as_ptr(),original.2.as_ptr(),original.3.as_ptr(),original.4.as_ptr()),pointers);
    let law=&fixture["grant"];
    let grant=RetainedCloneGrant{maximum_items:law["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:law["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:law["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:law["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:law["maximumDepth"].as_u64().unwrap()as usize};
    let(mut births,mut releases)=(0,0);
    for _ in 0..fixture["maximumTurns"].as_u64().unwrap() {
        if owner.terminal_is_empty(){break;}
        let (denied,heap)=observe(||owner.step(RetainedCloneGrant::default()).unwrap());
        assert_eq!(denied.progress(),Default::default());assert_eq!(heap,(0,0));
        let (step,heap)=observe(||owner.step(grant).unwrap());let receipt=step.progress();
        assert!(receipt.fits(grant));assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));
        births+=heap.0;releases+=heap.1;
    }
    assert!(owner.terminal_is_empty());assert_eq!(observe(||drop(owner)).1,(0,0));
    assert_eq!(original_heap.0+births,original_heap.1+releases);
    eprintln!("[DEBUG] original tuple5 native ownership physicalConservation=true originalPointers=true independentSerde=true everyActualReceipt=true terminalDrop0=true");
}
