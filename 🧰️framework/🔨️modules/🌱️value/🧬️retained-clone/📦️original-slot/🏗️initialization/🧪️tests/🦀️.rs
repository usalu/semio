use super::*;

fn exercise<const N: usize>(row: &serde_json::Value) {
    let mut output = MaybeUninit::<Option<[u8; N]>>::uninit();
    let extent = absent_field_bytes::<[u8; N]>();
    assert_eq!(extent, row["extentBytes"].as_u64().unwrap() as usize);
    assert!(!unsafe { initialize_absent_field_at(output.as_mut_ptr(), 0, 0) }.unwrap());
    let mut offset = 0;
    for count in row["chunks"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
        assert!(count <= row["pageBytes"].as_u64().unwrap() as usize);
        assert_eq!(absent_chunk_bytes(extent, offset, row["pageBytes"].as_u64().unwrap() as usize).unwrap(), count);
        let complete = unsafe { initialize_absent_field_at(output.as_mut_ptr(), offset, count) }.unwrap();
        offset += count;
        assert_eq!(complete, offset == extent);
    }
    assert_eq!(offset, extent);
    assert!(unsafe { output.assume_init_ref() }.is_none());
    unsafe { output.assume_init_drop(); }
}

#[test]
fn bounded_absent_fields_match_neutral_pages_and_reject_out_of_range() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    exercise::<56000>(&fixture["cases"][0]);
    exercise::<98304>(&fixture["cases"][1]);
    exercise::<15>(&fixture["cases"][2]);
    let mut output = MaybeUninit::<Option<[u8; 56000]>>::uninit();
    for row in fixture["invalidRanges"].as_array().unwrap() {
        assert!(unsafe { initialize_absent_field_at(output.as_mut_ptr(), row["offset"].as_u64().unwrap() as usize, row["count"].as_u64().unwrap() as usize) }.is_err());
    }
}

#[test]
fn absent_arrays_of_drop_owners_require_no_owner_construction() {
    type Slots = [Option<Box<[u8; 512]>>; 64];
    let mut output = MaybeUninit::<Slots>::uninit();
    let extent = absent_array_bytes::<Box<[u8; 512]>, 64>();
    let mut offset = 0;
    while offset < extent {
        let count = (extent - offset).min(7);
        let complete = unsafe { initialize_absent_array_at(output.as_mut_ptr(), offset, count) }.unwrap();
        offset += count;
        assert_eq!(complete, offset == extent);
    }
    assert!(unsafe { output.assume_init_ref() }.iter().all(Option::is_none));
    unsafe { output.assume_init_drop(); }
}

#[test]
fn zero_sized_empty_and_non_sync_payloads_preserve_absence() {
    fn initialize<T: 'static>() {
        let mut output = MaybeUninit::<Option<T>>::uninit();
        let extent = absent_field_bytes::<T>();
        assert_eq!(absent_chunk_bytes(extent, 0, 0).unwrap(), 0);
        assert!(unsafe { initialize_absent_field_at(output.as_mut_ptr(), 0, extent) }.unwrap());
        assert!(unsafe { output.assume_init_ref() }.is_none());
        unsafe { output.assume_init_drop(); }
    }
    initialize::<()>();
    initialize::<std::cell::UnsafeCell<u8>>();
    initialize::<&'static u8>();
    let mut empty = MaybeUninit::<[Option<Box<u8>>; 0]>::uninit();
    assert_eq!(absent_array_bytes::<Box<u8>, 0>(), 0);
    assert!(unsafe { initialize_absent_array_at(empty.as_mut_ptr(), 0, 0) }.unwrap());
    unsafe { empty.assume_init_drop(); }
}
