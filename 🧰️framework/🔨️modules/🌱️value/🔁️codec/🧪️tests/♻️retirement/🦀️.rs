//! ♻️ Exact canonical intrinsic cleanup request and retained-capacity proof.
use semio_framework_value::{DslValue,FromValue,NativeEncodeControl,ValueRefusalKind};
#[test]
fn value_intrinsic_retirement_full_system_requests_and_release() {
    fn literal(value: &serde_json::Value) -> DslValue {
        match value["kind"].as_str().unwrap() {
            "null" => DslValue::Null,
            "bool" => DslValue::Bool(value["value"].as_bool().unwrap()),
            "uint" => DslValue::uint(value["value"].as_str().unwrap().parse().unwrap()),
            "int" => DslValue::int(value["value"].as_str().unwrap().parse().unwrap()),
            "float" => DslValue::float(f64::from_bits(u64::from_str_radix(value["value"].as_str().unwrap(), 16).unwrap())),
            "string" => DslValue::String(value["value"].as_str().unwrap().into()),
            "bytes" => { let text = value["value"].as_str().unwrap(); DslValue::Bytes((0..text.len()).step_by(2).map(|index| u8::from_str_radix(&text[index..index + 2], 16).unwrap()).collect()) },
            "array" => DslValue::Array(value["value"].as_array().unwrap().iter().map(literal).collect()),
            "object" => DslValue::Object(value["value"].as_array().unwrap().iter().map(|member| (member["key"].as_str().unwrap().into(), literal(&member["value"]))).collect()),
            _ => panic!("closed intrinsic fixture kind"),
        }
    }
    let contract: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🛬️controlled/🔗️borrowed-keys.json")).unwrap();
    let contract = &contract["canonicalRetirement"];
    assert_eq!(contract["releasedBytes"], "completeBorrowedCapacityCensus");
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../🎒️pack/🌱️value/🧫️fixtures/🎞️intrinsic-media/🔣️.json")).unwrap();
    assert_eq!(corpus["contract"], contract["sourceCorpus"]);
    let long = contract["longBranch"]["unit"].as_str().unwrap().repeat(usize::try_from(contract["longBranch"]["repeat"].as_u64().unwrap()).unwrap());
    assert_eq!(long.len(), usize::try_from(contract["longBranch"]["utf8Bytes"].as_u64().unwrap()).unwrap());
    let mut cases = vec![
        ("null", DslValue::Null), ("bool", DslValue::Bool(true)), ("uint", DslValue::uint(u64::MAX)),
        ("int", DslValue::int(i64::MIN)), ("float", DslValue::float(f64::from_bits(0xfff800000000002a))),
        ("string", DslValue::String("\0literal 語".into())), ("bytes", DslValue::Bytes(vec![0, 255, 128, 127])),
        ("empty array", DslValue::Array(Vec::new())), ("empty object", DslValue::Object(Vec::new())),
        ("complete ordered intrinsic owner", literal(&corpus["value"])),
    ];
    let depth = usize::try_from(contract["depth"].as_u64().unwrap()).unwrap();
    let mut array = DslValue::String(long.clone());
    let mut object = DslValue::String(long.clone());
    for _ in 0..depth { array = DslValue::Array(vec![array]); object = DslValue::Object(vec![("duplicate\0語".into(), object)]); }
    cases.push(("deep array", array)); cases.push(("deep object", object));
    cases.push(("long ordered duplicate branches", DslValue::Object(vec![("same".into(), DslValue::String(long)), ("same".into(), literal(&corpus["value"]))])));
    for (label, owner) in cases {
        let mut reject = |_| false;
        let mut canceled = NativeEncodeControl::new(0, &mut reject);
        assert_eq!(canceled.begin_stage(1).unwrap_err().kind, ValueRefusalKind::Canceled);
        let mut source = vec![&owner];
        let mut expected_release = 0usize;
        while let Some(value) = source.pop() {
            match value {
                DslValue::String(value) => expected_release += value.capacity(),
                DslValue::Bytes(value) => expected_release += value.capacity(),
                DslValue::Array(values) => { expected_release += values.capacity() * std::mem::size_of::<DslValue>(); source.extend(values.iter()); },
                DslValue::Object(values) => { expected_release += values.capacity() * std::mem::size_of::<(String, DslValue)>(); for (key, value) in values { expected_release += key.capacity(); source.push(value); } },
                DslValue::Null | DslValue::Bool(_) | DslValue::Number(_) => {},
            }
        }
        drop(source);
        let ((), requests, released) = crate::test_allocation::observe_backing(|| <DslValue as FromValue>::retire_decoded(owner));
        assert_eq!(requests, usize::try_from(contract["requestBytes"].as_u64().unwrap()).unwrap(), "{label} mandatory retirement requested unadmitted backing after cancellation");
        assert_eq!(released, expected_release, "{label} complete source-owned backing must be deallocated");
    }
}


#[path="🔗️keys/🦀️.rs"]
mod borrowed_key_index;
