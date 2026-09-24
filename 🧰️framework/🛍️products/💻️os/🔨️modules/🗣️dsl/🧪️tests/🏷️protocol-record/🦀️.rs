//! 🏷️ The record-tag scanner agrees with the protocol dialect parser, and tagged ops carry exactly the declared tag.
use super::*;

const PROTOCOL: &str = "dialect protocol\nprotocol test.mutations\nversion 1\nschema test.op\nstart record\nframing record\n\n# a comment naming record ghost tag=9 is not a record\nheader fixed 2\nfield format u8\nfield tag u8\nrecord set-category tag=7\nfield payload bytes\n  record set-airtightness tag=300\nfield payload bytes\nrecord reset tag=0\nfield payload bytes\n";

#[derive(Clone, Debug, PartialEq, DslOps)]
enum TaggedMutation {
    SetCategory { category: String },
    SetAirtightness { n50: f64 },
    Reset,
}

const SET_CATEGORY: u8 = protocol_record::tag_u8(PROTOCOL, "set-category");

#[semio_framework_async_macros::async_test]
async fn scanner_agrees_with_the_dialect_parser() {
    let parsed = parse_protocol(PROTOCOL).expect("protocol parses");
    let declared: Vec<(String, u64)> = parsed.blocks.iter().filter_map(|block| match block { Block::Record { name, tag: Some(tag), .. } => Some((name.clone(), *tag)), _ => None }).collect();
    let scanned: Vec<(String, u64)> = protocol_record::records(PROTOCOL).map(|(kind, tag)| (kind.to_string(), tag)).collect();
    assert_eq!(scanned, declared);
    assert_eq!(SET_CATEGORY, 7);
    assert_eq!(protocol_record::tag(PROTOCOL, "set-airtightness"), 300);
    assert_eq!(protocol_record::kind(PROTOCOL, 0), Some("reset"));
    assert_eq!(protocol_record::kind(PROTOCOL, 9), None);
}

#[semio_framework_async_macros::async_test]
async fn tagged_ops_carry_the_declared_tag_and_round_trip() {
    for (op, tag) in [(TaggedMutation::SetCategory { category: "roof".into() }, vec![7u8]), (TaggedMutation::SetAirtightness { n50: 0.9 }, vec![0xac, 0x02]), (TaggedMutation::Reset, vec![0])] {
        let bytes = variants_binary::encode_tagged_op(PROTOCOL, &op).expect("encodes");
        assert_eq!(bytes[0], variants_binary::OP_BINARY_FORMAT);
        assert_eq!(&bytes[1..1 + tag.len()], tag.as_slice());
        assert_eq!(variants_binary::decode_tagged_op::<TaggedMutation>(PROTOCOL, &bytes).expect("decodes"), op);
    }
    assert!(variants_binary::decode_tagged_op::<TaggedMutation>(PROTOCOL, &[variants_binary::OP_BINARY_FORMAT, 9]).is_err());
}

#[semio_framework_async_macros::async_test]
async fn tagged_text_lines_carry_the_record_tag_instead_of_the_keyword() {
    let bytes = tagged_text_binary::encode_line(PROTOCOL, "set-category name=\"roof\"").expect("encodes");
    assert_eq!(&bytes[..2], &[tagged_text_binary::OP_BINARY_FORMAT, 7]);
    assert_eq!(&bytes[2..], b"name=\"roof\"");
    assert_eq!(tagged_text_binary::decode_line(PROTOCOL, &bytes).expect("decodes"), "set-category name=\"roof\"");
    assert_eq!(tagged_text_binary::decode_line(PROTOCOL, &tagged_text_binary::encode_line(PROTOCOL, "reset").unwrap()).unwrap(), "reset");
    assert!(tagged_text_binary::encode_line(PROTOCOL, "ghost").is_err());
}
