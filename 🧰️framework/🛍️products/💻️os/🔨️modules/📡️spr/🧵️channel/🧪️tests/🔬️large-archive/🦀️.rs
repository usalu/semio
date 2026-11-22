//! 📦️ Large persisted archive law: the codec accepts exactly up to its ceiling and frames bytes exactly as the language-neutral law does.

use std::{future::Future, pin::pin, task::{Context, Poll, Waker}};
use semio_os_channel::{decode_document_archive_bytes, encode_document_archive_bytes, DocumentArchivePack, DOCUMENT_ARCHIVE_MAXIMUM_BYTES};

fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let mut context = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(output) = future.as_mut().poll(&mut context) { return output; }
    }
}

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

fn fnv1a64_hex(bytes: &[u8]) -> String {
    format!("{:016x}", bytes.iter().fold(0xCBF2_9CE4_8422_2325u64, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01B3)))
}

fn write_varint(out: &mut Vec<u8>, mut value: u64) {
    while value >= 0x80 {
        out.push((value & 0x7F) as u8 | 0x80);
        value >>= 7;
    }
    out.push(value as u8);
}

fn frame_memberless_archive(pack: &[u8], spr: &[u8]) -> Vec<u8> {
    let mut bytes = vec![1];
    for field in [pack, spr] {
        write_varint(&mut bytes, field.len() as u64);
        bytes.extend_from_slice(field);
    }
    bytes.push(0);
    bytes
}

#[test]
fn large_document_archives_match_the_language_neutral_law() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📦️large-archive/🔣️.json")).unwrap();
    assert_eq!(law["ceiling"].as_u64().unwrap(), DOCUMENT_ARCHIVE_MAXIMUM_BYTES as u64);
    assert!(law["former"].as_u64().unwrap() < DOCUMENT_ARCHIVE_MAXIMUM_BYTES as u64);
    for case in law["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let seed = |key: &str| case[key].as_str().unwrap().parse::<u64>().unwrap();
        let pack = splitmix_bytes(seed("packSeed"), case["packBytes"].as_u64().unwrap() as usize);
        let spr = splitmix_bytes(seed("sprSeed"), case["sprBytes"].as_u64().unwrap() as usize);
        let framed = frame_memberless_archive(&pack, &spr);
        assert_eq!(framed.len() as u64, case["archiveBytes"].as_u64().unwrap(), "{name}");
        assert_eq!(fnv1a64_hex(&framed), case["fnv1a64"].as_str().unwrap(), "{name}");
        let archive = DocumentArchivePack { parent_pack: pack, parent_spr: spr, members: Vec::new() };
        if case["outcome"] == "accepted" {
            assert_eq!(encode_document_archive_bytes(&archive).expect(name), framed, "{name}");
            assert_eq!(block_on(decode_document_archive_bytes(&framed)).expect(name), archive, "{name}");
        } else {
            assert!(encode_document_archive_bytes(&archive).is_err(), "{name}");
            assert!(block_on(decode_document_archive_bytes(&framed)).is_err(), "{name}");
        }
    }
}
