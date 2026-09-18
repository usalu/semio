//! 📝 File writer (ISO 32000-1 §7.5): header, body, classic cross-reference table or
//! cross-reference stream with object streams (§7.5.7–7.5.8), trailer, `%%EOF`; the standard
//! security handler applied per object (§7.6). [`PdfWriter`] streams one object at a time so a
//! guest can emit a page per step without holding the document; [`serialize_document`] is the
//! whole-document convenience over it. Every byte is deterministic for the same input.

use super::encryption::{seal_standard_security, sha256, Decryptor};
use super::filters::{encode_stream, flate_encode};
use super::lexer::{object_bytes, write_dict, write_object};
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfDictEntry, PdfEncryption, PdfIndirectObject, PdfObject};
use std::collections::BTreeMap;

//#region 🔖️Options
/// 🎛️ How the file structure is laid out.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WriteOptions {
    /// 📐 Cross-reference stream + object streams (PDF 1.5+) instead of a classic table.
    pub xref_stream: bool,
    /// 🔐 Encrypt with the standard security handler.
    pub encryption: Option<PdfEncryption>,
}

/// 🧾 The trailer-level identity of a document.
#[derive(Clone, Debug, PartialEq)]
pub struct DocumentTrailer {
    pub root: ObjRef,
    pub info: Option<ObjRef>,
    pub id: Option<[Vec<u8>; 2]>,
    pub extra: Vec<PdfDictEntry>,
}
//#endregion 🔖️Options

//#region 🔖️Streaming
/// 📝 An incremental writer: `header`, any number of `object`s (in any order), then `finish`.
pub struct PdfWriter {
    offset: usize,
    offsets: BTreeMap<u32, (usize, u16)>,
    version: String,
    xref_stream: bool,
    cipher: Option<Decryptor>,
    encrypt_dict: Option<Vec<PdfDictEntry>>,
    compressible: Vec<PdfIndirectObject>,
    max_number: u32,
}

impl PdfWriter {
    /// 🏁 Starts a document; returns the header bytes.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn begin(version: &str, options: &WriteOptions, document_id: &[u8], seed: &[u8]) -> (Self, Vec<u8>) {
        let version = if version.is_empty() { "1.7" } else { version };
        let mut header = format!("%PDF-{version}\n%").into_bytes();
        header.extend_from_slice(&[0xE2, 0xE3, 0xCF, 0xD3, b'\n']);
        let (cipher, encrypt_dict) = match &options.encryption {
            Some(parameters) => {
                let (cipher, dict) = seal_standard_security(parameters, document_id, seed);
                (Some(cipher), Some(dict))
            }
            None => (None, None),
        };
        let writer = Self { offset: header.len(), offsets: BTreeMap::new(), version: version.to_string(), xref_stream: options.xref_stream, cipher, encrypt_dict, compressible: Vec::new(), max_number: 0 };
        (writer, header)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn version(&self) -> &str {
        &self.version
    }

    /// 🧱 Serializes one indirect object (filters applied, encrypted when sealed); returns the
    /// bytes to append. With object streams enabled, non-stream objects are held back and
    /// packed at `finish`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn object(&mut self, object: &PdfIndirectObject) -> Vec<u8> {
        self.max_number = self.max_number.max(object.id.num);
        let mut value = materialize_streams(&object.value);
        if let Some(cipher) = &self.cipher {
            value = cipher.encrypt_object(value, object.id.num, object.id.gen);
        }
        if self.xref_stream && !matches!(value, PdfObject::Stream { .. }) && object.id.gen == 0 {
            self.compressible.push(PdfIndirectObject { id: object.id, value });
            return Vec::new();
        }
        self.offsets.insert(object.id.num, (self.offset, object.id.gen));
        let mut out = format!("{} {} obj\n", object.id.num, object.id.gen).into_bytes();
        write_object(&mut out, &value);
        out.extend_from_slice(b"\nendobj\n");
        self.offset += out.len();
        out
    }

    /// 🏁 Writes the cross-reference section and trailer; returns the closing bytes.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn finish(mut self, trailer: &DocumentTrailer) -> Vec<u8> {
        let mut out = Vec::new();
        let mut trailer_entries: Vec<PdfDictEntry> = Vec::new();
        if let Some(dict) = self.encrypt_dict.take() {
            let number = self.max_number + 1;
            self.max_number = number;
            self.offsets.insert(number, (self.offset, 0));
            let mut bytes = format!("{number} 0 obj\n").into_bytes();
            write_object(&mut bytes, &PdfObject::Dict(dict));
            bytes.extend_from_slice(b"\nendobj\n");
            self.offset += bytes.len();
            out.extend_from_slice(&bytes);
            trailer_entries.push(PdfDictEntry::new("Encrypt", PdfObject::Ref(ObjRef { num: number, gen: 0 })));
        }
        trailer_entries.push(PdfDictEntry::new("Root", PdfObject::Ref(trailer.root)));
        if let Some(info) = trailer.info {
            trailer_entries.push(PdfDictEntry::new("Info", PdfObject::Ref(info)));
        }
        if let Some([first, second]) = &trailer.id {
            trailer_entries.push(PdfDictEntry::new("ID", PdfObject::Array(vec![PdfObject::Str(first.clone()), PdfObject::Str(second.clone())])));
        }
        trailer_entries.extend(trailer.extra.iter().filter(|entry| !matches!(entry.key.as_str(), "Root" | "Info" | "ID" | "Encrypt" | "Size" | "Prev" | "XRefStm" | "Type" | "Filter" | "DecodeParms" | "W" | "Index" | "Length")).cloned());
        if self.xref_stream {
            self.finish_xref_stream(&mut out, trailer_entries);
        } else {
            self.finish_classic(&mut out, trailer_entries);
        }
        out
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn finish_classic(self, out: &mut Vec<u8>, trailer_entries: Vec<PdfDictEntry>) {
        let size = self.max_number + 1;
        let xref_offset = self.offset;
        let free: Vec<u32> = (1..size).filter(|number| !self.offsets.contains_key(number)).collect();
        out.extend_from_slice(format!("xref\n0 {size}\n{:010} 65535 f \n", free.first().copied().unwrap_or(0)).as_bytes());
        for number in 1..size {
            match self.offsets.get(&number) {
                Some((offset, generation)) => out.extend_from_slice(format!("{offset:010} {generation:05} n \n").as_bytes()),
                None => {
                    let next = free.iter().position(|f| *f == number).and_then(|index| free.get(index + 1)).copied().unwrap_or(0);
                    out.extend_from_slice(format!("{next:010} 00000 f \n").as_bytes());
                }
            }
        }
        let mut entries = vec![PdfDictEntry::new("Size", PdfObject::Int(size as i64))];
        entries.extend(trailer_entries);
        out.extend_from_slice(b"trailer\n");
        write_dict(out, &entries);
        out.extend_from_slice(format!("\nstartxref\n{xref_offset}\n%%EOF\n").as_bytes());
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn finish_xref_stream(mut self, out: &mut Vec<u8>, trailer_entries: Vec<PdfDictEntry>) {
        let mut compressed: BTreeMap<u32, (u32, u32)> = BTreeMap::new();
        let held = std::mem::take(&mut self.compressible);
        for chunk in held.chunks(100) {
            let stream_number = self.max_number + 1;
            self.max_number = stream_number;
            let mut offsets_text = String::new();
            let mut body = Vec::new();
            for (index, object) in chunk.iter().enumerate() {
                offsets_text.push_str(&format!("{} {} ", object.id.num, body.len()));
                write_object(&mut body, &object.value);
                body.push(b'\n');
                compressed.insert(object.id.num, (stream_number, index as u32));
            }
            let mut data = offsets_text.into_bytes();
            let first = data.len();
            data.extend_from_slice(&body);
            let encoded = flate_encode(&data);
            let dict = vec![PdfDictEntry::new("Type", PdfObject::name("ObjStm")), PdfDictEntry::new("N", PdfObject::Int(chunk.len() as i64)), PdfDictEntry::new("First", PdfObject::Int(first as i64)), PdfDictEntry::new("Filter", PdfObject::name("FlateDecode")), PdfDictEntry::new("Length", PdfObject::Int(encoded.len() as i64))];
            self.offsets.insert(stream_number, (self.offset, 0));
            let mut bytes = format!("{stream_number} 0 obj\n").into_bytes();
            write_dict(&mut bytes, &dict);
            bytes.extend_from_slice(b"\nstream\n");
            bytes.extend_from_slice(&encoded);
            bytes.extend_from_slice(b"\nendstream\nendobj\n");
            self.offset += bytes.len();
            out.extend_from_slice(&bytes);
        }
        let xref_number = self.max_number + 1;
        let size = xref_number + 1;
        let xref_offset = self.offset;
        let mut rows = Vec::new();
        for number in 0..size {
            if number == xref_number {
                rows.extend_from_slice(&[1]);
                rows.extend_from_slice(&(xref_offset as u32).to_be_bytes());
                rows.extend_from_slice(&[0, 0]);
            } else if let Some((offset, generation)) = self.offsets.get(&number) {
                rows.push(1);
                rows.extend_from_slice(&(*offset as u32).to_be_bytes());
                rows.extend_from_slice(&generation.to_be_bytes());
            } else if let Some((stream_number, index)) = compressed.get(&number) {
                rows.push(2);
                rows.extend_from_slice(&stream_number.to_be_bytes());
                rows.extend_from_slice(&(*index as u16).to_be_bytes());
            } else {
                rows.push(0);
                rows.extend_from_slice(&0u32.to_be_bytes());
                rows.extend_from_slice(&0xFFFFu16.to_be_bytes());
            }
        }
        let encoded = flate_encode(&rows);
        let mut dict = vec![PdfDictEntry::new("Type", PdfObject::name("XRef")), PdfDictEntry::new("Size", PdfObject::Int(size as i64)), PdfDictEntry::new("W", PdfObject::Array(vec![PdfObject::Int(1), PdfObject::Int(4), PdfObject::Int(2)])), PdfDictEntry::new("Filter", PdfObject::name("FlateDecode")), PdfDictEntry::new("Length", PdfObject::Int(encoded.len() as i64))];
        dict.extend(trailer_entries);
        let mut bytes = format!("{xref_number} 0 obj\n").into_bytes();
        write_dict(&mut bytes, &dict);
        bytes.extend_from_slice(b"\nstream\n");
        bytes.extend_from_slice(&encoded);
        bytes.extend_from_slice(b"\nendstream\nendobj\n");
        out.extend_from_slice(&bytes);
        out.extend_from_slice(format!("startxref\n{xref_offset}\n%%EOF\n").as_bytes());
    }
}

/// 🗜️ Applies every stream's typed filter pipeline so the dictionary and bytes agree (recursing
/// into nested containers, which may hold direct stream values only in inline-image parameters).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn materialize_streams(value: &PdfObject) -> PdfObject {
    match value {
        PdfObject::Stream { dict, data, filters } => {
            let (encoded, filter_entries) = encode_stream(data, filters);
            let mut entries: Vec<PdfDictEntry> = dict.iter().filter(|entry| !matches!(entry.key.as_str(), "Filter" | "DecodeParms" | "Length")).map(|entry| PdfDictEntry { key: entry.key.clone(), value: materialize_streams(&entry.value) }).collect();
            entries.extend(filter_entries);
            PdfObject::Stream { dict: entries, data: encoded, filters: Vec::new() }
        }
        PdfObject::Array(items) => PdfObject::Array(items.iter().map(materialize_streams).collect()),
        PdfObject::Dict(entries) => PdfObject::Dict(entries.iter().map(|entry| PdfDictEntry { key: entry.key.clone(), value: materialize_streams(&entry.value) }).collect()),
        other => other.clone(),
    }
}
//#endregion 🔖️Streaming

//#region 🔖️WholeDocument
/// 🆔 A deterministic document identifier: the SHA-256 of the un-encrypted object bytes, split
/// into the two 16-byte halves the trailer `/ID` carries.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive_document_id(objects: &[PdfIndirectObject]) -> [Vec<u8>; 2] {
    let mut input = Vec::new();
    for object in objects {
        input.extend_from_slice(&object.id.num.to_be_bytes());
        input.extend_from_slice(&object_bytes(&object.value));
    }
    let digest = sha256(&input);
    [digest[..16].to_vec(), digest[16..].to_vec()]
}

/// 📤️ Serializes a whole object graph into one file.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize_document(version: &str, objects: &[PdfIndirectObject], trailer: &DocumentTrailer, options: &WriteOptions) -> Vec<u8> {
    let mut trailer = trailer.clone();
    if trailer.id.is_none() && options.encryption.is_some() {
        trailer.id = Some(derive_document_id(objects));
    }
    let seed = trailer.id.as_ref().map(|id| id[0].clone()).unwrap_or_default();
    let (mut writer, mut out) = PdfWriter::begin(version, options, trailer.id.as_ref().map(|id| id[0].as_slice()).unwrap_or(&[]), &seed);
    for object in objects {
        let bytes = writer.object(object);
        out.extend_from_slice(&bytes);
    }
    out.extend_from_slice(&writer.finish(&trailer));
    out
}
//#endregion 🔖️WholeDocument

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
