use super::*;
use protocol::DiffCodec;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn tag(id: u16, kind: TiffFieldType, values: TiffValues) -> TiffTag {
    TiffTag { tag: id, kind, values }
}

/// 🧪️ `DiffCodec` round-trip laws over the hand-rolled `TiffDiff` grammar — exercises every
/// `TiffValues` variant (incl. `Rational`/`SRational` pair lists and `Ascii`/`Byte` hex),
/// both IFD-level (index-keyed) and tag-level (id-keyed) removed/modified/added, and the
/// scalar `byte_order`/`pixels` tokens.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let a = TiffSnapshot {
        schema: "stdio.tiff".into(),
        byte_order: TiffByteOrder::LittleEndian,
        ifds: vec![
            TiffIfd {
                pixels: Vec::new(),
                entries: vec![
                    tag(256, TiffFieldType::Long, TiffValues::Long(vec![4])),
                    tag(258, TiffFieldType::Short, TiffValues::Short(vec![8, 8, 8])),
                    tag(315, TiffFieldType::Ascii, TiffValues::Ascii("An Author".into())),
                    tag(282, TiffFieldType::Rational, TiffValues::Rational(vec![(72, 1)])),
                    tag(700, TiffFieldType::Undefined, TiffValues::Undefined(vec![0xde, 0xad])),
                ],
            },
            TiffIfd { pixels: Vec::new(), entries: vec![tag(1, TiffFieldType::Byte, TiffValues::Byte(vec![1, 2, 3]))] },
        ],
        pixels: vec![0u8; 16],
    };
    let mut b = a.clone();
    b.byte_order = TiffByteOrder::BigEndian;
    b.ifds[0].entries.retain(|t| t.tag != 258); // remove
    b.ifds[0].entries.iter_mut().find(|t| t.tag == 315).unwrap().values = TiffValues::Ascii("New Author".into()); // modify
    b.ifds[0].entries.push(tag(37380, TiffFieldType::SRational, TiffValues::SRational(vec![(-3, 10), (0, 1)]))); // add
    b.ifds[0].entries.push(tag(50000, TiffFieldType::SByte, TiffValues::SByte(vec![-1, -2])));
    b.ifds[0].entries.push(tag(50001, TiffFieldType::SShort, TiffValues::SShort(vec![-100])));
    b.ifds[0].entries.push(tag(50002, TiffFieldType::SLong, TiffValues::SLong(vec![-100000])));
    b.ifds[0].entries.push(tag(50003, TiffFieldType::Float, TiffValues::Float(vec![1.5, -2.25])));
    b.ifds[0].entries.push(tag(50004, TiffFieldType::Double, TiffValues::Double(vec![3.14159265358979])));
    b.ifds.push(TiffIfd { pixels: Vec::new(), entries: vec![tag(2, TiffFieldType::Long, TiffValues::Long(vec![9]))] }); // whole IFD added
    b.pixels = vec![9u8; 16];
    let c = TiffSnapshot { schema: "stdio.tiff".into(), byte_order: TiffByteOrder::LittleEndian, ifds: vec![], pixels: vec![] };

    let cases = vec![TiffDiff::default(), TiffDiff::between(&a, &b), TiffDiff::between(&b, &a), TiffDiff::between(&a, &c), TiffDiff::between(&c, &a)];
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = TiffDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = TiffDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
