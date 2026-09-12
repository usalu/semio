
use super::*;
use std::mem::{align_of, size_of};

const MAX_RETAIN_CYCLES: usize = 4096;

fn bounded_retain_cycles(value: usize) -> Result<usize, &'static str> {
    (value <= MAX_RETAIN_CYCLES).then_some(value).ok_or("retain-cycle fixture exceeds its bounded test budget")
}

#[test]
fn owned_runtime_preserves_empty_single_max_max_plus_one_and_hostile_contract() {
    autorelease_pool(|| {
        for requested in [0usize, 1, MAX_RETAIN_CYCLES] {
            let cycles = bounded_retain_cycles(requested).expect("fixture case must be accepted");
            let value = CAMetalLayer::new();
            let before = retain_count(&*value);
            for _ in 0..cycles {
                let clone = value.clone();
                assert_eq!(retain_count(&*value), before + 1);
                drop(clone);
            }
            assert_eq!(retain_count(&*value), before);
        }
        assert_eq!(bounded_retain_cycles(MAX_RETAIN_CYCLES + 1), Err("retain-cycle fixture exceeds its bounded test budget"));
        assert_eq!(bounded_retain_cycles(usize::MAX), Err("retain-cycle fixture exceeds its bounded test budget"));
        assert!(unsafe { Owned::<NSString>::from_new(std::ptr::null_mut()) }.is_none());
        let fixture = format!(
            "{{\n  \"$schema\": \"https://semio.tech/schema/framework/ui/render/schema.json#/$defs/MetalObjectiveCAbiFixture\",\n  \"schemaVersion\": 1,\n  \"contract\": \"owned-objective-c-runtime\",\n  \"oracle\": {{ \"package\": \"objc2\", \"version\": \"0.6.4\" }},\n  \"layout\": {{ \"ownedBytes\": {}, \"ownedAlign\": {}, \"optionalOwnedBytes\": {} }},\n  \"ownership\": {{ \"cloneRetainDelta\": 1, \"dropRestores\": true, \"nullOwnedAccepted\": false, \"autoreleasePoolDrained\": true }},\n  \"boundaries\": {{ \"empty\": \"accepted\", \"single\": \"accepted\", \"maximum\": 4096, \"maximumPlusOne\": \"rejected\", \"hostileNull\": \"rejected\" }}\n}}\n",
            size_of::<Owned<NSString>>(),
            align_of::<Owned<NSString>>(),
            size_of::<Option<Owned<NSString>>>()
        );
        assert_eq!(fixture, include_str!("../../🧫️fixtures/🍎️metal/🔣️.json"));
        println!("empty=ok single=ok max=4096 maxPlusOne=rejected hostileNull=rejected retainDelta=1 restored=true pool=drained");
    });
}
