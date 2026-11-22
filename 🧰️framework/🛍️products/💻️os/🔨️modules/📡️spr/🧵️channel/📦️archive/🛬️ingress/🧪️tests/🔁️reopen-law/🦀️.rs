//! 🔁️ Reopens archives above the former contiguous ceiling through paid fixed-page receiving, with the retained pages and the largest heap request checked.
use semio_framework_value::{AuthoringWallet, ByteReadSource, RetainedCloneGrant};
use semio_framework_value_testing_allocator::{LARGEST_ALLOCATION, OBSERVE_ALLOCATIONS};
use semio_os_channel_archive_ingress::*;
use std::sync::atomic::Ordering;

const GRANT: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 4096, maximum_copy_bytes: 1 << 18, maximum_capacity_bytes: 4096,
    maximum_release_bytes: 8192, maximum_depth: 32,
};
const STOCK: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 1 << 26, maximum_copy_bytes: 1 << 31, maximum_capacity_bytes: 1 << 27,
    maximum_release_bytes: 1 << 27, maximum_depth: 32,
};
const WORK: u64 = 1 << 16;

fn splitmix_bytes(seed: u64, length: usize) -> Vec<u8> {
    let mut state = seed;
    let mut out = Vec::with_capacity(length + 8);
    while out.len() < length {
        state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = (state ^ (state >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        out.extend_from_slice(&(z ^ (z >> 31)).to_le_bytes());
    }
    out.truncate(length);
    out
}

fn varint(out: &mut Vec<u8>, mut value: usize) {
    while value >= 0x80 {
        out.push((value & 0x7F) as u8 | 0x80);
        value >>= 7;
    }
    out.push(value as u8);
}

fn frame(pack: &[u8], spr: &[u8]) -> Vec<u8> {
    let mut bytes = vec![1];
    for field in [pack, spr] {
        varint(&mut bytes, field.len());
        bytes.extend_from_slice(field);
    }
    bytes.push(0);
    bytes
}

fn hex(text: &str) -> [u8; 32] {
    let mut out = [0; 32];
    for (at, byte) in out.iter_mut().enumerate() { *byte = u8::from_str_radix(&text[2 * at..2 * at + 2], 16).unwrap(); }
    out
}

fn same(source: &dyn ByteReadSource, expected: &[u8]) -> bool {
    let mut scratch = [0; DOCUMENT_ARCHIVE_INGRESS_PAGE_BYTES];
    let mut offset = 0;
    while offset < expected.len() {
        let count = source.read_bytes(offset, &mut scratch);
        if count == 0 || scratch[..count] != expected[offset..offset + count] { return false; }
        offset += count;
    }
    source.byte_len() == expected.len()
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome { Ready, Refused, Cancelled }

fn reopen(archive: &[u8], digest: [u8; 32], limits: DocumentArchiveIngressLimits, cancel_after: Option<usize>, expected: Option<(&[u8], &[u8])>) -> Outcome {
    let mut wallet = AuthoringWallet::new(STOCK, 1 << 29);
    let mut cursor = PagedDocumentArchiveIngress::new(DocumentArchiveIngressSeal { bytes: archive.len(), sha256: digest }, limits).unwrap();
    let mut refused = false;
    LARGEST_ALLOCATION.store(0, Ordering::Relaxed);
    OBSERVE_ALLOCATIONS.store(true, Ordering::Relaxed);
    while !cursor.ready() {
        let offset = cursor.accepted();
        let cancelling = cancel_after.is_some_and(|after| offset > after);
        if cancelling { cursor.cancel(); }
        let window = &archive[offset.min(archive.len())..(offset + DOCUMENT_ARCHIVE_INGRESS_PAGE_BYTES).min(archive.len())];
        let turn = wallet.run(GRANT, WORK, |effect| if window.is_empty() { cursor.finish(effect) } else { cursor.step(offset, window, effect) }).unwrap().unwrap().output;
        match turn.result {
            Ok(receipt) => { assert!(receipt.fits(GRANT)); wallet.post(receipt).unwrap(); }
            Err(error) => { wallet.post(error.retained_progress()).unwrap(); refused = true; break; }
        }
        if cancelling { break; }
    }
    let ready = cursor.ready();
    let pages = if ready {
        let (parent, receipt) = wallet.run(GRANT, WORK, |effect| cursor.take_verified(effect)).unwrap().unwrap().output.unwrap().unwrap();
        wallet.post(receipt).unwrap();
        assert_eq!(parent.seal().sha256, digest);
        parent.into_pages()
    } else {
        let (partial, receipt) = wallet.run(GRANT, WORK, |effect| cursor.take_unverified(effect)).unwrap().unwrap().output.unwrap().expect(&format!("unverified pages unavailable: refused={refused} accepted={} extent={} cancel_after={cancel_after:?}", cursor.accepted(), archive.len()));
        wallet.post(receipt).unwrap();
        partial
    };
    if let (true, Some((pack, spr))) = (ready, expected) {
        assert!(same(&pages.pack, pack), "retained Pack differs from its source field");
        assert!(same(&pages.spr, spr), "retained SPR differs from its source field");
    }
    OBSERVE_ALLOCATIONS.store(false, Ordering::Relaxed);
    let heap = LARGEST_ALLOCATION.load(Ordering::Relaxed);
    assert!(heap <= DOCUMENT_ARCHIVE_INGRESS_PAGE_BYTES, "largest allocation {heap}");
    let mut controlled = semio_framework_value::retirement::controlled::ControlledRetirement::retain(pages);
    for _ in 0..200_000 {
        if controlled.terminal_is_empty() { break; }
        let turn = wallet.run_settled(GRANT, WORK, |effect| controlled.step(GRANT.maximum_copy_bytes, effect)).unwrap().unwrap();
        wallet.post(turn.0.result.unwrap().progress()).unwrap();
    }
    assert!(controlled.terminal_is_empty(), "paid retirement did not settle");
    if ready { Outcome::Ready } else if cancel_after.is_some() { Outcome::Cancelled } else if refused { Outcome::Refused } else { panic!("input ended without a verdict") }
}

#[test]
fn reopen_above_the_former_ceiling_matches_the_language_neutral_law() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/📦️large-archive/🔣️.json")).unwrap();
    let case = &law["cases"][0];
    let seed = |key: &str| case[key].as_str().unwrap().parse::<u64>().unwrap();
    let pack = splitmix_bytes(seed("packSeed"), case["packBytes"].as_u64().unwrap() as usize);
    let spr = splitmix_bytes(seed("sprSeed"), case["sprBytes"].as_u64().unwrap() as usize);
    let archive = frame(&pack, &spr);
    assert!(archive.len() as u64 > law["former"].as_u64().unwrap());
    assert_eq!(archive.len() as u64, case["archiveBytes"].as_u64().unwrap());
    let digest = hex(case["sha256"].as_str().unwrap());
    assert_eq!(semio_framework_hash::Sha256::digest(&archive), digest, "the archive matches the independent SHA-256");
    let limits = DocumentArchiveIngressLimits { archive_bytes: law["ceiling"].as_u64().unwrap() as usize, pack_bytes: pack.len(), spr_bytes: 65536 };
    assert_eq!(reopen(&archive, digest, limits, None, Some((&pack, &spr))), Outcome::Ready);
    let mut wrong = digest;
    wrong[0] ^= 1;
    assert_eq!(reopen(&archive, wrong, limits, None, None), Outcome::Refused);
    assert_eq!(reopen(&archive, digest, limits, Some(8192), None), Outcome::Cancelled);
    let tight = DocumentArchiveIngressLimits { pack_bytes: pack.len() - 1, ..limits };
    assert_eq!(reopen(&archive, digest, tight, None, None), Outcome::Refused);
}
