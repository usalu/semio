//! ✍️ Exercises the private byte-buffer primitive without claiming a funded queue or receiver.

use super::input_writer::{InputByteBuffer, InputWriteGrant, InputWriteKind};
use super::input_root_tests::{allocations_end, allocations_start};

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("🔣️.json")).unwrap() }
fn grant(work_bytes: usize, physical_bytes: usize) -> InputWriteGrant { InputWriteGrant { items: 1, work_bytes, physical_bytes } }
fn full() -> InputWriteGrant { grant(32_768, 32_768) }
fn hex(value: &str) -> Vec<u8> { value.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect() }
fn close(buffer: &mut InputByteBuffer) {
    for _ in 0..8_200 {
        if buffer.terminal_is_empty() { break; }
        let step = buffer.close_step(full());
        assert!(step.work_bytes <= full().work_bytes);
        assert!(step.released_bytes <= full().physical_bytes);
    }
    assert!(buffer.terminal_is_empty());
}

//#region 🧪️ByteWriter
#[test]
fn input_writer_native_one_byte_validation_copy_and_seal_are_separate() {
    let fixture = fixture();
    let source = fixture["source"]["text"].as_str().unwrap().to_owned();
    let original = (source.as_ptr(), source.capacity());
    let mut buffer = InputByteBuffer::new();
    let reserved = buffer.reserve(source.len(), fixture["source"]["minimumCapacity"].as_u64().unwrap() as usize, full());
    assert_eq!(reserved.kind, InputWriteKind::Allocated);
    let capacity = buffer.capacity();
    assert_eq!(reserved.allocated_bytes, capacity);
    let pointer = buffer.bytes().as_ptr();
    let mut turns = 0usize;
    allocations_start();
    for byte in source.bytes() {
        let before = buffer.bytes().len();
        let validation = buffer.validate_byte(byte, grant(1, 0));
        assert_eq!(validation.kind, InputWriteKind::Validated);
        assert_eq!(validation.work_bytes, 1);
        assert_eq!(buffer.bytes().len(), before);
        assert_eq!(buffer.copy_validated(grant(0, 0)).kind, InputWriteKind::Blocked);
        assert_eq!(buffer.bytes().len(), before);
        let copied = buffer.copy_validated(grant(1, 0));
        assert_eq!(copied.kind, InputWriteKind::Copied);
        assert_eq!(copied.work_bytes, 1);
        assert_eq!(buffer.bytes().len(), before + 1);
        turns += 2;
    }
    let seal_bytes = InputByteBuffer::seal_work_bytes();
    assert_eq!(buffer.seal(grant(seal_bytes - 1, 0)).kind, InputWriteKind::Blocked);
    assert!(buffer.text().is_none());
    let sealed = buffer.seal(grant(seal_bytes, 0));
    let allocations = allocations_end();
    assert_eq!(sealed.kind, InputWriteKind::Sealed);
    assert_eq!(sealed.work_bytes, seal_bytes);
    assert_eq!(seal_bytes, fixture["work"]["descriptorTransfers"].as_u64().unwrap() as usize * std::mem::size_of::<Vec<u8>>());
    assert_eq!(turns as u64, fixture["work"]["oneByteValidationAndCopyTurns"].as_u64().unwrap());
    assert_eq!(allocations, 0);
    assert_eq!(buffer.text().unwrap(), source);
    assert_eq!(buffer.text().unwrap().as_ptr(), pointer);
    assert_eq!(buffer.capacity(), capacity);
    assert_eq!((source.as_ptr(), source.capacity()), original);
    close(&mut buffer);
}

#[test]
fn input_writer_native_utf8_matches_std_without_a_final_scan() {
    let fixture = fixture();
    for vector in fixture["utf8"].as_array().unwrap() {
        let source = hex(vector["hex"].as_str().unwrap());
        let valid = std::str::from_utf8(&source).is_ok();
        assert_eq!(valid, vector["valid"].as_bool().unwrap());
        let mut buffer = InputByteBuffer::new();
        assert_eq!(buffer.reserve(source.len(), 64, full()).kind, InputWriteKind::Allocated);
        let mut fault = false;
        for byte in &source {
            let step = buffer.validate_byte(*byte, grant(1, 0));
            if step.kind == InputWriteKind::InvalidUtf8 { fault = true; break; }
            assert_eq!(step.kind, InputWriteKind::Validated);
            assert_eq!(buffer.copy_validated(grant(1, 0)).kind, InputWriteKind::Copied);
        }
        let sealed = buffer.seal(full());
        assert_eq!(!fault && sealed.kind == InputWriteKind::Sealed, valid, "{}", vector["name"]);
        if valid { assert_eq!(buffer.text().unwrap().as_bytes(), source); }
        else { assert_eq!(buffer.validate_byte(0, full()).kind, InputWriteKind::InvalidUtf8); }
        close(&mut buffer);
    }
}

#[test]
fn input_writer_native_refusal_and_bytewise_close_retain_physical_backing() {
    let fixture = fixture();
    let source = fixture["source"]["text"].as_str().unwrap().as_bytes();
    let minimum = fixture["source"]["minimumCapacity"].as_u64().unwrap() as usize;
    allocations_start();
    let mut buffer = InputByteBuffer::new();
    for refused in [InputWriteGrant { items: 0, work_bytes: 32_768, physical_bytes: 32_768 }, grant(0, minimum), grant(32_768, minimum - 1)] {
        assert_eq!(buffer.reserve(source.len(), minimum, refused).kind, InputWriteKind::Blocked);
        assert_eq!(buffer.capacity(), 0);
    }
    let refused_allocations = allocations_end();
    assert_eq!(refused_allocations, 0);
    let reserved = buffer.reserve(source.len(), minimum, full());
    assert_eq!(reserved.kind, InputWriteKind::Allocated);
    let pointer = buffer.bytes().as_ptr();
    let capacity = buffer.capacity();
    for byte in source {
        buffer.validate_byte(*byte, grant(1, 0));
        buffer.copy_validated(grant(1, 0));
    }
    allocations_start();
    for (index, bytes) in fixture["close"]["byteGrants"].as_array().unwrap().iter().enumerate() {
        let step = buffer.close_step(grant(bytes.as_u64().unwrap() as usize, 0));
        assert_eq!(step.released_bytes, 0);
        assert_eq!(buffer.inspected_bytes() as u64, fixture["close"]["inspected"][index].as_u64().unwrap());
        assert_eq!(buffer.bytes().as_ptr(), pointer);
        assert_eq!(buffer.capacity(), capacity);
    }
    let inspected_allocations = allocations_end();
    assert_eq!(inspected_allocations, 0);
    let short = buffer.close_step(grant(32_768, capacity - 1));
    assert_eq!(short.kind, InputWriteKind::Blocked);
    assert_eq!(buffer.capacity(), capacity);
    assert!(!buffer.terminal_is_empty());
    let released = buffer.close_step(full());
    assert_eq!(released.kind, InputWriteKind::Released);
    assert_eq!(released.released_bytes, capacity);
    assert_eq!(buffer.capacity(), 0);
    assert!(buffer.terminal_is_empty());
}

#[test]
fn input_writer_native_every_actual_partial_copy_unwind_keeps_original_and_backing() {
    let fixture = fixture();
    for frontier in fixture["unwind"]["frontiers"].as_array().unwrap() {
        let source = fixture["source"]["text"].as_str().unwrap().to_owned();
        let original = (source.as_ptr(), source.capacity());
        let mut buffer = InputByteBuffer::new();
        buffer.reserve(source.len(), 64, full());
        let pointer = buffer.bytes().as_ptr();
        let capacity = buffer.capacity();
        let count = frontier.as_u64().unwrap() as usize;
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            for byte in source.bytes().take(count) {
                buffer.validate_byte(byte, grant(1, 0));
                buffer.copy_validated(grant(1, 0));
            }
            panic!("[DEBUG] after actual partial input byte copy");
        }));
        assert!(outcome.is_err());
        drop(outcome);
        assert_eq!((source.as_ptr(), source.capacity()), original);
        assert_eq!(buffer.bytes(), &source.as_bytes()[..count]);
        assert_eq!(buffer.bytes().as_ptr(), pointer);
        assert_eq!(buffer.capacity(), capacity);
        close(&mut buffer);
    }
}
//#endregion 🧪️ByteWriter

//#region 🧪️ActualByteRetirement
#[test]
fn input_writer_native_close_scrubs_actual_initialized_bytes_before_backing_release() {
    let fixture = fixture();
    let source = fixture["source"]["text"].as_str().unwrap().as_bytes();
    let mut buffer = InputByteBuffer::new();
    buffer.reserve(source.len(), 64, full());
    for byte in source { buffer.validate_byte(*byte, grant(1, 0)); buffer.copy_validated(grant(1, 0)); }
    let pointer = buffer.bytes().as_ptr();
    let capacity = buffer.capacity();
    let mut observed = Vec::new();
    for bytes in fixture["close"]["byteGrants"].as_array().unwrap() {
        let step = buffer.close_step(grant(bytes.as_u64().unwrap() as usize, 0));
        observed.push((buffer.bytes().to_vec(), step.work_bytes, step.released_bytes));
        assert_eq!(buffer.bytes().as_ptr(), pointer);
        assert_eq!(buffer.capacity(), capacity);
    }
    close(&mut buffer);
    for (index, (bytes, work, released)) in observed.iter().enumerate() {
        assert_eq!(*bytes, hex(fixture["close"]["scrubHexAfterGrant"][index].as_str().unwrap()), "actual bytes at close frontier {index}");
        assert_eq!(*work as u64, fixture["close"]["byteGrants"][index].as_u64().unwrap());
        assert_eq!(*released, 0);
    }
}

#[test]
fn input_writer_native_close_scrubs_sealed_text_only_after_granted_descriptor_transfer() {
    let fixture = fixture();
    let source = fixture["source"]["text"].as_str().unwrap().as_bytes();
    let mut buffer = InputByteBuffer::new();
    buffer.reserve(source.len(), 64, full());
    for byte in source { buffer.validate_byte(*byte, grant(1, 0)); buffer.copy_validated(grant(1, 0)); }
    buffer.seal(full());
    let pointer = buffer.bytes().as_ptr();
    let capacity = buffer.capacity();
    let short = buffer.close_step(grant(InputByteBuffer::seal_work_bytes() - 1, 0));
    let short_kept_text = buffer.text().is_some_and(|text| text.as_bytes() == source);
    let short_kept_progress = buffer.inspected_bytes() == 0;
    let converted = buffer.close_step(grant(InputByteBuffer::seal_work_bytes(), 0));
    let converted_to_bytes = buffer.text().is_none();
    let converted_kept_bytes = buffer.bytes() == source;
    let converted_kept_progress = buffer.inspected_bytes() == 0;
    assert_eq!(buffer.bytes().as_ptr(), pointer);
    assert_eq!(buffer.capacity(), capacity);
    let scrub = buffer.close_step(grant(1, 0));
    let first_scrub = buffer.bytes().to_vec();
    close(&mut buffer);
    assert_eq!(short.kind, InputWriteKind::Blocked);
    assert!(short_kept_text && short_kept_progress);
    assert!(converted_to_bytes && converted_kept_bytes && converted_kept_progress);
    assert_eq!(converted.work_bytes, InputByteBuffer::seal_work_bytes());
    assert_eq!(converted.released_bytes, 0);
    assert_eq!(scrub.work_bytes, 1);
    assert_eq!(first_scrub, hex(fixture["close"]["scrubHexAfterGrant"][1].as_str().unwrap()));
}
//#endregion 🧪️ActualByteRetirement

