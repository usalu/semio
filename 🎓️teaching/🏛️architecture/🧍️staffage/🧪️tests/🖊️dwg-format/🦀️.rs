//! 🧪️ The DWG writer owns its checksums, compression, bit codes, JSON reader and container; every one of them is compared with an independent implementation: acadrust (checksums, AC18 decompressor, bit reader), serde_json (JSON), and ezdwg-core (container, header variables, object index).
use acadrust::io::dwg::checksum as theirs;
use acadrust::io::dwg::crc;
use acadrust::io::dwg::decompressor_ac18::decompress_ac18;
use acadrust::io::dwg::dwg_stream_readers::bit_reader::DwgBitReader;
use acadrust::io::dwg::DwgVersion;
use acadrust::types::DxfVersion;
use staffage_dwg::bits::{aci, Bits, Ref};
use staffage_dwg::checksum::{adler, crc16, crc32, magic, mask_page_header, CRC16_SEED};
use staffage_dwg::json::{parse, Json};
use staffage_dwg::lz77::compress;

const FIXTURE: &str = include_str!("../../🧫️fixtures/📐️model-scene/🔣️.json");
const SHEET: &str = include_str!("../../🧫️fixtures/📑️sheet-model-scene/🔣️.json");

struct Random(u64);

impl Random {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 11
    }

    fn bytes(&mut self, count: usize) -> Vec<u8> {
        (0..count).map(|_| (self.next() >> 3) as u8).collect()
    }
}

#[test]
fn checksums_masks_and_the_magic_sequence_equal_acadrust() {
    let mut random = Random(7);
    for length in [0, 1, 2, 31, 32, 255, 5552, 5553, 70_000] {
        let data = random.bytes(length);
        assert_eq!(crc16(CRC16_SEED, &data), crc::crc16(0xC0C1, &data), "crc16 of {length} bytes");
        assert_eq!(crc16(0x1234, &data), crc::crc16(0x1234, &data), "crc16 with a seed");
        assert_eq!(crc32(&data), crc::crc32(&data), "crc32 of {length} bytes");
        assert_eq!(adler(0, &data), theirs::adler32_checksum(0, &data), "adler of {length} bytes");
        assert_eq!(adler(0xABCD_1234, &data), theirs::adler32_checksum(0xABCD_1234, &data), "adler with a seed");
    }
    assert_eq!(magic(), theirs::magic_sequence());
    let header = random.bytes(32);
    let (mut ours, mut other) = (header.clone(), header);
    mask_page_header(&mut ours, 0x1A40);
    theirs::apply_mask(&mut other, 0x1A40);
    assert_eq!(ours, other);
}

fn roundtrip(data: &[u8]) -> usize {
    let packed = compress(data).expect("compressible");
    assert_eq!(decompress_ac18(&packed, data.len()), data, "{} bytes", data.len());
    packed.len()
}

#[test]
fn compressed_pages_decompress_to_their_input() {
    let mut random = Random(11);
    assert!(roundtrip(&vec![0; 0x7400]) < 200);
    assert!(roundtrip(&vec![0xAB; 0x7400]) < 200);
    roundtrip(&[1, 2, 3, 4]);
    roundtrip(&[5; 4]);
    roundtrip(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9]);
    roundtrip(&random.bytes(100_000));
    let mut mixed = Vec::new();
    for round in 0..200usize {
        mixed.extend(random.bytes(round % 23));
        mixed.extend(std::iter::repeat_n((round % 7) as u8, round % 90));
    }
    roundtrip(&mixed);
    for length in [4usize, 5, 17, 18, 19, 20, 270, 271, 272, 0x22, 0x23, 0x21, 600] {
        roundtrip(&random.bytes(length));
        roundtrip(&[vec![9, 8, 7, 6], vec![0; length]].concat());
        roundtrip(&[vec![0; length], vec![1, 2, 3, 4, 5]].concat());
        roundtrip(&[vec![3, 3, 3, 3, 3, 3], random.bytes(length), vec![0; length]].concat());
    }
    assert!(compress(&[1, 2, 3]).is_err());
}

enum Value {
    Bit(bool),
    Pair(u8),
    Short(i16),
    Long(i32),
    Double(f64),
    Default(f64, f64),
    Byte(u8),
    RawShort(u16),
    RawLong(u32),
    RawDouble(f64),
    Handle(u64),
    Color(u32),
    Type(u16),
}

#[test]
fn bit_codes_are_read_back_by_an_independent_decoder() {
    let mut random = Random(3);
    let doubles = [0.0, 1.0, -1.0, 0.1, 412_148_564_080.0, 1e20, -1e-9, std::f64::consts::PI, 2.5, 1.0000000001];
    let mut values = Vec::new();
    for round in 0..4000u64 {
        let r = random.next();
        values.push(match round % 13 {
            0 => Value::Bit(r & 1 == 1),
            1 => Value::Pair((r % 4) as u8),
            2 => Value::Short([0, 1, 255, 256, 257, -1, 3020, i16::MAX, i16::MIN][(r % 9) as usize]),
            3 => Value::Long([0, 1, 255, 256, -1, 70_000, i32::MAX, i32::MIN, 24][(r % 9) as usize]),
            4 => Value::Double(doubles[(r % 10) as usize]),
            5 => {
                let base = doubles[(r % 10) as usize];
                let shifted = match r % 4 {
                    0 => base,
                    1 => f64::from_bits(base.to_bits() ^ (r & 0xFFFF_FFFF)),
                    2 => f64::from_bits(base.to_bits() ^ (r & 0xFFFF_FFFF_FFFF)),
                    _ => doubles[((r >> 7) % 10) as usize],
                };
                Value::Default(shifted, base)
            }
            6 => Value::Byte((r >> 5) as u8),
            7 => Value::RawShort((r >> 4) as u16),
            8 => Value::RawLong((r >> 2) as u32),
            9 => Value::RawDouble(f64::from_bits(r << 11)),
            10 => Value::Handle([0, 1, 0x1F, 0xFF, 0x100, 0x12345, 0xFFFF_FFFF, 0x1_0000_0000, u64::MAX >> 1][(r % 9) as usize]),
            11 => Value::Color([aci(7), aci(255), 0xC000_0000, 0xC100_0000, 0xC2FF_8040][(r % 5) as usize]),
            _ => Value::Type([0, 1, 77, 255, 0x1F0, 503, 0x2EF, 0x2F0, 0x1F2][(r % 9) as usize]),
        });
    }
    let mut bits = Bits::new();
    for value in &values {
        match value {
            Value::Bit(v) => bits.bit(*v),
            Value::Pair(v) => bits.bits(u64::from(*v), 2),
            Value::Short(v) => bits.bs(*v),
            Value::Long(v) => bits.bl(*v),
            Value::Double(v) => bits.bd(*v),
            Value::Default(v, d) => bits.dd(*v, *d),
            Value::Byte(v) => bits.rc(*v),
            Value::RawShort(v) => bits.rs(*v),
            Value::RawLong(v) => bits.rl(*v),
            Value::RawDouble(v) => bits.rd(*v),
            Value::Handle(v) => bits.handle(Ref::HardPointer, *v),
            Value::Color(v) => bits.cmc(*v),
            Value::Type(v) => bits.object_type(*v),
        }
    }
    let mut reader = DwgBitReader::new(bits.into_bytes(), DwgVersion::AC24, DxfVersion::AC1024);
    for (index, value) in values.iter().enumerate() {
        match value {
            Value::Bit(v) => assert_eq!(reader.read_bit(), *v, "#{index}"),
            Value::Pair(v) => assert_eq!(reader.read_2bits(), *v, "#{index}"),
            Value::Short(v) => assert_eq!(reader.read_bit_short(), *v, "#{index}"),
            Value::Long(v) => assert_eq!(reader.read_bit_long(), *v, "#{index}"),
            Value::Double(v) => assert_eq!(reader.read_bit_double().to_bits(), v.to_bits(), "#{index}"),
            Value::Default(v, d) => assert_eq!(reader.read_bit_double_with_default(*d).to_bits(), v.to_bits(), "#{index}: {v} against {d}"),
            Value::Byte(v) => assert_eq!(reader.read_byte(), *v, "#{index}"),
            Value::RawShort(v) => assert_eq!(reader.read_raw_ushort(), *v, "#{index}"),
            Value::RawLong(v) => assert_eq!(reader.read_raw_ulong(), u64::from(*v), "#{index}"),
            Value::RawDouble(v) => assert_eq!(reader.read_raw_double().to_bits(), v.to_bits(), "#{index}"),
            Value::Handle(v) => assert_eq!(reader.read_handle(), *v, "#{index}"),
            Value::Color(v) => {
                let color = reader.read_cm_color();
                assert_eq!(format!("{color}"), format!("{}", expected_color(*v)), "#{index}");
            }
            Value::Type(v) => assert_eq!(reader.read_object_type(), *v as i16, "#{index}"),
        }
    }
}

fn expected_color(packed: u32) -> acadrust::types::Color {
    use acadrust::types::Color;
    match packed >> 24 {
        0xC0 => Color::ByLayer,
        0xC1 => Color::ByBlock,
        0xC2 => Color::Rgb { r: (packed >> 16) as u8, g: (packed >> 8) as u8, b: packed as u8 },
        _ => Color::Index(packed as u8),
    }
}

fn same(ours: &Json, theirs: &serde_json::Value) -> bool {
    match (ours, theirs) {
        (Json::Null, serde_json::Value::Null) => true,
        (Json::Bool(a), serde_json::Value::Bool(b)) => a == b,
        (Json::Number(a), serde_json::Value::Number(b)) => b.as_f64() == Some(*a),
        (Json::String(a), serde_json::Value::String(b)) => a == b,
        (Json::Array(a), serde_json::Value::Array(b)) => a.len() == b.len() && a.iter().zip(b).all(|(x, y)| same(x, y)),
        (Json::Object(a), serde_json::Value::Object(b)) => a.len() == b.len() && a.iter().all(|(key, value)| b.get(key).is_some_and(|other| same(value, other))),
        _ => false,
    }
}

#[test]
fn the_json_reader_equals_serde_json() {
    let accepted = [
        "null", "true", " false ", "0", "-0.5", "1e3", "1E-3", "123456789.125", "\"\"", "\"a\\\"b\\\\c\\/d\\b\\f\\n\\r\\t\"", "\"\\u00e9\\u20ac\"", "\"\\ud83d\\ude00\"", "\"héllo · m²\"",
        "[]", "[1,2,[3,[4]]]", "{}", "{\"a\":{\"b\":[true,null]},\"c\":\"d\"}", "{\"x\" : 1 , \"y\" : -2.5e+2}", FIXTURE, SHEET,
    ];
    for text in accepted {
        let ours = parse(text.as_bytes()).unwrap_or_else(|error| panic!("{text}: {error}"));
        let theirs: serde_json::Value = serde_json::from_str(text).unwrap();
        assert!(same(&ours, &theirs), "{text}");
    }
    let rejected = ["", "{", "[1,]", "{\"a\":}", "{'a':1}", "01x", "\"\\x\"", "\"\\ud800\"", "\"\\ud800\\u0041\"", "\"a\nb\"", "tru", "[1 2]", "{\"a\" 1}", "1 2", "--1", "NaN", "1e999", "\"\\u12\"", "[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[1]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]"];
    for text in rejected {
        let theirs = serde_json::from_str::<serde_json::Value>(text);
        assert!(parse(text.as_bytes()).is_err(), "{text:?} must be rejected");
        if text != "1e999" && !text.starts_with("[[[[") {
            assert!(theirs.is_err(), "{text:?} is rejected by serde_json too");
        }
    }
    assert!(parse(&[b'"', 0xFF, b'"']).is_err());
}

fn probe(bytes: &[u8], objects: usize) {
    use ezdwg_core::core::config::ParseConfig;
    use ezdwg_core::dwg::decoder::Decoder;
    let decoder = Decoder::new(bytes, ParseConfig::default()).expect("ezdwg-core opens the container");
    assert_eq!(decoder.version().as_str(), "AC1024");
    decoder.ensure_supported().expect("a supported version");
    let index = decoder.build_object_index().expect("the object map");
    assert_eq!(index.len(), objects, "one object-map entry per written record");
    for object in &index.objects {
        let record = decoder.parse_object_record(object.offset).expect("a record with a valid size and CRC");
        assert!(record.size > 0);
    }
    let header = decoder.header_variables().expect("the header variables");
    assert_eq!(header.insunits, Some(6));
    assert_eq!(header.lunits, Some(2));
    assert_eq!(header.luprec, Some(4));
    assert_eq!(header.textsize, Some(2.5));
    assert_eq!(header.ltscale, Some(1.0));
    let classes = decoder.dynamic_type_map().expect("the class map");
    assert_eq!(classes.get(&503).map(String::as_str), Some("LWPOLYLINE"));
    assert_eq!(classes.get(&500).map(String::as_str), Some("ACDBDICTIONARYWDFLT"));
}

fn polyline_count(json: &str) -> usize {
    let value: serde_json::Value = serde_json::from_str(json).unwrap();
    value["layers"].as_array().unwrap().iter().map(|layer| layer["polylines"].as_array().unwrap().len() + layer["texts"].as_array().unwrap().len()).sum()
}

#[test]
fn the_container_is_valid_for_a_second_independent_reader() {
    const STANDARD_OBJECTS: usize = 36;
    for json in [FIXTURE, SHEET] {
        let bytes = staffage_dwg::encode_json(json.as_bytes()).expect("encodes");
        let layers = serde_json::from_str::<serde_json::Value>(json).unwrap()["layers"].as_array().unwrap().len();
        probe(&bytes, STANDARD_OBJECTS + layers + polyline_count(json));
    }
    let big = serde_json::json!({"units": "m", "layers": [{"name": "BIG", "aci": 3, "lineweight": 25, "texts": [], "polylines": (0..4000).map(|i| serde_json::json!({"closed": i % 2 == 0, "points": (0..30).map(|k| serde_json::json!({"x": f64::from(i) * 0.37 + f64::from(k) * 0.011, "y": f64::from(k * k) * 0.0007 - f64::from(i) * 0.002})).collect::<Vec<_>>()})).collect::<Vec<_>>()}]}).to_string();
    let bytes = staffage_dwg::encode_json(big.as_bytes()).expect("encodes a large scene");
    assert!(bytes.len() > 0x7400 * 3, "the object section spans several pages");
    probe(&bytes, STANDARD_OBJECTS + 1 + 4000);
}
