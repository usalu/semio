extern crate p9_a6_codec_abi_feature;

#[cfg(not(any(hostile_inject_raw_bytes, hostile_inject_whole_slice)))]
const SOURCE: &[u8] = include_bytes!("../🦀️.rs");
#[cfg(hostile_inject_raw_bytes)]
const SOURCE: &[u8] = b"enum Input { Bytes(Vec<u8>) }";
#[cfg(hostile_inject_whole_slice)]
const SOURCE: &[u8] = b"let input = input.bytes();";

const fn contains(source: &[u8], needle: &[u8]) -> bool {
    let mut index = 0;
    while index + needle.len() <= source.len() {
        let mut offset = 0;
        while offset < needle.len() && source[index + offset] == needle[offset] {
            offset += 1;
        }
        if offset == needle.len() {
            return true;
        }
        index += 1;
    }
    false
}

fn main() {
    assert!(!contains(SOURCE, b"Bytes(Vec<u8>)"));
    assert!(!contains(SOURCE, b"Self::Bytes("));
    assert!(!contains(SOURCE, b"input.bytes()"));
    assert!(!contains(SOURCE, b"execute_filter("));
    #[cfg(hostile_inject_batch_edge)]
    let _ = p9_a6_codec_abi_feature::codec_abi::UiForbiddenOsHostWorkflowBatchBackend;
}
