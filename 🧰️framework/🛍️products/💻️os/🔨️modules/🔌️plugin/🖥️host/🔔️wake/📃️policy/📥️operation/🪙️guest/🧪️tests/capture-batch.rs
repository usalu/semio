//! 🧾️ Actual capture progress and clock boundaries against the neutral receipt corpus.

use super::*;
use semio_framework_job::{ClockStride, Generation, OperationId, OriginalJobFixtureCaller, OriginalNativeLease};
use semio_framework_value::RetainedCloneGrant;
use std::cell::Cell;

const DEADLINE: u64 = 100_000;
const TURN_DEADLINE: u64 = 500;
const STOCK: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 1 << 20,
    maximum_copy_bytes: 1 << 26,
    maximum_capacity_bytes: 1 << 26,
    maximum_release_bytes: 1 << 26,
    maximum_depth: 64,
};
thread_local! { static NOW: Cell<u64> = const { Cell::new(100) }; }
fn clock() -> Option<u64> { Some(NOW.get()) }
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("🧫️fixtures/capture-batch.json")).unwrap() }

struct Producer;
impl OriginalNativeRetirement for Producer {
    fn close_on_source(&mut self, _: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> { Ok(true) }
    fn terminal_is_empty(&self) -> bool { true }
}

fn caller() -> OriginalJobFixtureCaller {
    NOW.set(100);
    OriginalJobFixtureCaller::admit(OperationId(71), Generation(3), STOCK, clock, DEADLINE, 100_000)
}

fn resident(law: &serde_json::Value) -> ([u8; 152], Vec<u8>, Vec<[u64; 23]>) {
    let rows: Vec<[u64; 23]> = serde_json::from_value(law["rows"].clone()).unwrap();
    let mut escrow = b"SRE1".to_vec();
    escrow.extend((rows.len() as u32).to_le_bytes());
    for word in law["source"].as_array().unwrap().iter().chain([&law["revision"]]) {
        escrow.extend(word.as_u64().unwrap().to_le_bytes());
    }
    escrow.extend(0_u64.to_le_bytes());
    for row in &rows { for word in row { escrow.extend(word.to_le_bytes()); } }
    let mut header = [0; 152];
    header[..8].copy_from_slice(b"SAC3\x05\x03\0\0");
    for (bytes, word) in header[8..136].chunks_exact_mut(8).zip(law["headerWords"].as_array().unwrap()) {
        bytes.copy_from_slice(&word.as_u64().unwrap().to_le_bytes());
    }
    for (bytes, word) in header[136..].chunks_exact_mut(4).zip([80, escrow.len() as u32, 200, 3]) {
        bytes.copy_from_slice(&word.to_le_bytes());
    }
    (header, escrow, rows)
}

fn word(bytes: &[u8], offset: usize) -> Option<&[u8]> {
    (offset < bytes.len()).then(|| &bytes[offset..(offset + 8).min(bytes.len())])
}

fn drive(caller: &mut OriginalJobFixtureCaller, window: RetainedCloneGrant, mut step: impl FnMut(&mut StepContext<'_>) -> Result<bool, OriginalTurnFailure>) -> (usize, [u128; 3]) {
    let mut total = [0; 3];
    for turns in 1..=256 {
        let done = caller.turn(STOCK, |cx| cx.with_retained_loan(window, |cx| {
            let before = (cx.retained_progress(), cx.original_work_units_spent());
            let done = step(cx)?;
            let after = (cx.retained_progress(), cx.original_work_units_spent());
            let items = after.0.copied_items - before.0.copied_items;
            let copied = after.0.copied_bytes - before.0.copied_bytes;
            assert!(items <= window.maximum_items && copied <= window.maximum_copy_bytes);
            assert_eq!(before.0.retained_capacity_bytes, after.0.retained_capacity_bytes);
            assert_eq!(before.0.released_bytes, after.0.released_bytes);
            for (sum, delta) in total.iter_mut().zip([items as u128, copied as u128, after.1 - before.1]) { *sum += delta; }
            Ok(done)
        })).unwrap();
        if done { return (turns, total); }
    }
    panic!("batched capture stalled under an authorized window");
}

#[test]
fn batched_frozen_receipts_preserve_paid_totals_and_claim_words() {
    let law = fixture();
    let (header, escrow, rows) = resident(&law);
    for schedule in law["schedules"].as_array().unwrap() {
        let mut caller = caller();
        caller.audit_heap();
        caller.admit_return_table();
        let (mut input, mut lease) = (Some(Producer), None);
        for _ in 0..128 {
            caller.turn(STOCK, |cx| cx.admit_original_native(&mut input, &mut lease)).unwrap();
            if lease.is_some() { break; }
        }
        let producer = lease.as_ref().unwrap().recipient_identity();
        let book = GuestEscrowBook::default();
        let mut words = EscrowWords::empty();
        words.header_length = header.len();
        words.length = escrow.len();
        let window = RetainedCloneGrant { maximum_items: schedule["items"].as_u64().unwrap() as usize, ..STOCK };
        let expect_phase = |name: &str, idx: usize, (turns, total): (usize, [u128; 3])| {
            let [items, fixed, pointers, work]: [u64; 4] = serde_json::from_value(law["phases"][name].clone()).unwrap();
            assert_eq!(total, [items as u128, fixed as u128 + (size_of::<usize>() as u128) * pointers as u128, work as u128], "{} {name}", schedule["name"]);
            assert_eq!(turns as u64, schedule["turns"][idx].as_u64().unwrap(), "{} {name}", schedule["name"]);
        };
        expect_phase("copy", 0, drive(&mut caller, window, |cx| Ok(copy_frozen_words_step(&mut words, cx, |fixed, offset| word(if fixed { &header } else { &escrow }, offset))? == FrozenCaptureProgress::WordsCopied)));
        assert_eq!(&words.header[..header.len()], &header);
        assert_eq!(&words.bytes[..escrow.len()], &escrow);
        expect_phase("header", 1, drive(&mut caller, window, |cx| Ok(validate_fixed_words_step(&mut words, cx)? == HeaderValidationProgress::HeaderValidated)));
        expect_phase("sre", 2, drive(&mut caller, window, |cx| Ok(validate_sre_words_step(&mut words, &book, None, producer, cx)? == SreValidationProgress::SreValidated)));
        for (row, original) in rows.iter().enumerate() {
            let mut draft = ClaimDraft::empty();
            expect_phase("claim", 3, drive(&mut caller, window, |cx| copy_claim_fields_step(&words, &mut draft, row, cx)));
            assert_eq!(&draft.values, original);
        }
        for _ in 0..128 {
            if caller.retirement_turn(STOCK, |cx| OriginalNativeLease::return_slot_step(&mut lease, cx)).unwrap() { break; }
        }
        assert!(lease.is_none());
        let retired = (0..128).any(|_| caller.retirement_turn(STOCK, |cx| cx.close_original_native_step()).unwrap());
        assert!(retired);
        caller.retire(STOCK);
    }
}

#[test]
fn frozen_batch_stops_at_partial_funding_cancellation_and_real_deadline() {
    let law = fixture();
    let (header, _, _) = resident(&law);
    for case in law["stops"].as_array().unwrap() {
        let mut caller = caller();
        let mut words = EscrowWords::empty();
        words.header_length = header.len();
        let limit = case["limit"].as_u64().unwrap();
        let mode = case["mode"].as_str().unwrap();
        let cancel = matches!(mode, "cancel" | "retire").then(|| caller.admit_alias(STOCK));
        let mut stopped_turn = |cx: &mut StepContext<'_>| {
            let before = (cx.retained_progress(), cx.original_work_units_spent());
            cx.restore_clock_stride(ClockStride::new());
            let mut read = 0;
            let mut copy = |cx: &mut StepContext<'_>| copy_frozen_words_step(&mut words, cx, |_, offset| {
                if read == limit {
                    if mode == "cancel" { cancel.as_ref().unwrap().cancel_now(); }
                    if mode == "deadline" { NOW.set(TURN_DEADLINE); }
                }
                read += 1;
                word(&header, offset)
            });
            let progress = match mode {
                "copy" => cx.with_retained_loan(RetainedCloneGrant { maximum_copy_bytes: size_of::<usize>() + limit as usize, ..STOCK }, copy)?,
                "fuel" => cx.with_original_work_loan(limit, DEADLINE, copy)?,
                "items" => cx.with_retained_loan(RetainedCloneGrant { maximum_items: limit as usize, ..STOCK }, copy)?,
                "retire" => {
                    cancel.as_ref().unwrap().cancel_now();
                    cx.with_retained_loan(RetainedCloneGrant { maximum_items: limit as usize, ..STOCK }, |cx| cx.with_original_retirement(copy))?
                }
                _ => copy(cx)?,
            };
            assert!(progress == FrozenCaptureProgress::Pending, "{}", case["name"]);
            let bytes = case["bytes"].as_u64().unwrap() as usize;
            let items = case["items"].as_u64().unwrap() as usize;
            assert_eq!(words.header_cursor, bytes, "{}", case["name"]);
            assert_eq!(&words.header[..bytes], &header[..bytes]);
            assert_eq!(cx.retained_progress().copied_bytes - before.0.copied_bytes, bytes + items * size_of::<usize>());
            assert_eq!(cx.retained_progress().copied_items - before.0.copied_items, items);
            assert_eq!(cx.original_work_units_spent() - before.1, case["work"].as_u64().unwrap() as u128);
            if matches!(mode, "cancel" | "deadline") {
                let before_validation = (cx.retained_progress(), cx.original_work_units_spent());
                assert!(validate_fixed_words_step(&mut words, cx)? == HeaderValidationProgress::Pending);
                let mut draft = ClaimDraft::empty();
                assert!(!copy_claim_fields_step(&words, &mut draft, 0, cx)?);
                assert_eq!(words.header_validation.phase, 0);
                assert!(!draft.active);
                assert_eq!((cx.retained_progress(), cx.original_work_units_spent()), before_validation);
            }
            Ok(())
        };
        caller.turn(STOCK, |cx| {
            if mode == "deadline" {
                cx.with_original_work_loan(cx.fuel_remaining(), TURN_DEADLINE, stopped_turn)
            } else {
                stopped_turn(cx)
            }
        }).unwrap();
        if let Some(cancel) = cancel { caller.return_alias(cancel, STOCK); }
        if matches!(mode, "copy" | "fuel" | "items") {
            let remaining = header.len() - words.header_cursor;
            let (_, receipt) = drive(&mut caller, STOCK, |cx| {
                Ok(copy_frozen_words_step(&mut words, cx, |_, offset| word(&header, offset))? == FrozenCaptureProgress::WordsCopied)
            });
            let items = remaining.div_ceil(8);
            assert_eq!(receipt, [items as u128, (remaining + items * size_of::<usize>()) as u128, (remaining + items) as u128]);
            assert_eq!(&words.header[..header.len()], &header);
        }
        caller.retire(STOCK);
    }
}
