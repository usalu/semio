use super::*;

#[test]
fn microsecond_owned_wasi_clock_is_real_nanoseconds_with_checked_unsigned_range() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🧵️job/⏱️budget/🕰️clock.json")).unwrap();
    for law in fixture["wasi"].as_array().unwrap() {
        let nanoseconds = law["nanoseconds"].as_str().unwrap().parse::<u128>().unwrap();
        let duration = std::time::Duration::new((nanoseconds / 1_000_000_000) as u64, (nanoseconds % 1_000_000_000) as u32);
        let result = owned_wasi_nanoseconds(duration);
        assert_eq!(result.is_ok(), law["accepted"].as_bool().unwrap());
        if let Ok(actual) = result {
            assert_eq!(u128::from(actual), nanoseconds);
        }
    }
    let read = || match owned_wasi_monotonic_clock().unwrap() {
        Value::I64(value) => u64::from_ne_bytes(value.to_ne_bytes()),
        _ => panic!("WASI clock primitive"),
    };
    let start = read();
    let mut last = start;
    for _ in 0..10_000 {
        let next = read();
        assert!(next >= last);
        last = next;
        if last > start {
            break;
        }
    }
    assert!(last > start, "owned WASI host must not return a frozen zero clock");
    eprintln!("[DEBUG] owned WASI real monotonic nanoseconds start={start} last={last}");
}
