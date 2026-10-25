//! 📦️ Binary primitives of the DXF diff codec: the length-prefixed bytes and strings, the optional presence byte, the floats and points and the DXF value pair. See [`super`].

use super::*;

/// 🧪️ P2-FG1: real LEB128-varint-framed binary primitives (length-prefixed bytes/utf8, tri-state
/// `Option<T>` presence byte) backing the upgraded `OpBinary`/`DiffCodec` frames below — reuses
/// `store::write_varint_u64`/`store::write_varint_i64`/`store::ByteReader` (crate-root re-exports
/// of `os_pack`'s own varint/reader primitives — `store`/`dsl`/`protocol` are all `extern crate
/// self as …` aliases for the SAME kernel crate, see `🦀️.rs`) rather than reinventing varint
/// encode/decode. Same shape `stdio.json`'s own upgrade introduced
/// (`🔣️json/…/🔺️diff/🦀️.rs`'s `#region 🔖️BinaryPrimitives`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_option_bin<T>(out: &mut Vec<u8>, opt: &Option<T>, enc: impl Fn(&T, &mut Vec<u8>)) {
    match opt {
        None => out.push(0),
        Some(v) => {
            out.push(1);
            enc(v, out);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_option_bin<T>(reader: &mut store::ByteReader<'_>, dec: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<Option<T>, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(None),
        1 => Ok(Some(dec(reader)?)),
        other => Err(format!("option binary: bad tag {other}")),
    }
}

/// 🧪️ P2-FG1: real recursive binary twins of every `enc_*`/`dec_*` item-literal codec above
/// (`#region 🔖️GeometryCodecs`/`#region 🔖️ValueCodecs`/`#region 🔖️EntityValueCodecs`/
/// `#region 🔖️ItemCodecs`) — backs the upgraded `OpBinary` frame
/// (`../🧬️mutations/🦀️.rs`) and every collection's `added`-entry / `entity-diff`'s
/// `Replace`-arm payload below. Tag numbering mirrors each text codec's own letter-tag order
/// (`0`=first letter, `1`=second, …), independent per type (not reused across unrelated shapes).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_f64_bin(out: &mut Vec<u8>, v: f64) {
    out.extend_from_slice(&v.to_le_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_f64_bin(reader: &mut store::ByteReader<'_>) -> Result<f64, String> {
    reader.read_f64_le().map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_point3_bin(out: &mut Vec<u8>, p: &[f64; 3]) {
    write_f64_bin(out, p[0]);
    write_f64_bin(out, p[1]);
    write_f64_bin(out, p[2]);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_point3_bin(reader: &mut store::ByteReader<'_>) -> Result<[f64; 3], String> {
    Ok([read_f64_bin(reader)?, read_f64_bin(reader)?, read_f64_bin(reader)?])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_points4_bin(out: &mut Vec<u8>, p: &[[f64; 3]; 4]) {
    for pt in p {
        write_point3_bin(out, pt);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_points4_bin(reader: &mut store::ByteReader<'_>) -> Result<[[f64; 3]; 4], String> {
    Ok([read_point3_bin(reader)?, read_point3_bin(reader)?, read_point3_bin(reader)?, read_point3_bin(reader)?])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_dxf_value_bin(v: &DxfValue, out: &mut Vec<u8>) {
    match v {
        DxfValue::Str { value } => {
            out.push(0);
            write_str_lp(out, value);
        }
        DxfValue::Int { value } => {
            out.push(1);
            store::write_varint_i64(out, *value);
        }
        DxfValue::Double { value } => {
            out.push(2);
            write_f64_bin(out, *value);
        }
        DxfValue::Point { value } => {
            out.push(3);
            write_point3_bin(out, value);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_dxf_value_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfValue, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(DxfValue::Str { value: read_str_lp(reader)? }),
        1 => Ok(DxfValue::Int { value: reader.read_varint_i64().map_err(|e| e.to_string())? }),
        2 => Ok(DxfValue::Double { value: read_f64_bin(reader)? }),
        3 => Ok(DxfValue::Point { value: read_point3_bin(reader)? }),
        other => Err(format!("dxf value binary: unknown tag {other}")),
    }
}
