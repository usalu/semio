//! 🎒️ Archive and compression oracles: ZIP containers and raw zlib/deflate streams.
//!
//! The `semantic-archive-v1` profile compares entry names, uncompressed sizes and content digests
//! as a SET. Entry order, compression method and level, timestamps, external attributes and the
//! extra field are writer choices, not normative content.
//!
//! @see 📇️registry/🔣️component.json — the approved oracle registry these functions implement.

use semio_repo_test_host::{digest, Json};

//#region 🔖️ArchiveSpec
/// 🎒️ One named member of an archive.
#[derive(Debug, Clone)]
pub struct ArchiveEntry {
    pub name: String,
    pub bytes: Vec<u8>,
}

/// 🎒️ Owned description of an archive — the one input both producers are given.
#[derive(Debug, Clone)]
pub struct ArchiveSpec {
    pub entries: Vec<ArchiveEntry>,
}

impl ArchiveSpec {
    /// 🎒️ Reads a spec out of a scenario's owned JSON payload.
    pub fn from_json(value: &Json) -> ArchiveSpec {
        ArchiveSpec { entries: value.array("entries").iter().map(|entry| ArchiveEntry { name: entry.str("name"), bytes: entry.str("content").into_bytes() }).collect() }
    }

    /// 🔁️ The projection every archive producer is compared through.
    pub fn projection(&self) -> Json {
        Json::Object(vec![
            ("format".to_string(), Json::String("zip".to_string())),
            ("entryCount".to_string(), Json::Number(self.entries.len() as f64)),
            (
                "entries".to_string(),
                Json::Array(
                    self.entries
                        .iter()
                        .map(|entry| Json::Object(vec![("name".to_string(), Json::String(entry.name.clone())), ("size".to_string(), Json::Number(entry.bytes.len() as f64)), ("contentDigest".to_string(), Json::String(digest(&entry.bytes)))]))
                        .collect(),
                ),
            ),
        ])
    }
}
//#endregion 🔖️ArchiveSpec

//#region 🔖️Zip
/// 🔮️ Creates a ZIP with the registered `zip` reference implementation.
/// @see https://github.com/zip-rs/zip2
#[cfg(feature = "oracles")]
pub fn oracle_create_zip(spec: &ArchiveSpec) -> Result<Vec<u8>, String> {
    use std::io::Write;
    let mut cursor = std::io::Cursor::new(Vec::new());
    {
        let mut writer = zip::ZipWriter::new(&mut cursor);
        let options: zip::write::FileOptions<'_, ()> = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
        for entry in &spec.entries {
            writer.start_file(entry.name.clone(), options).map_err(|error| format!("zip start_file: {}", error))?;
            writer.write_all(&entry.bytes).map_err(|error| format!("zip write: {}", error))?;
        }
        writer.finish().map_err(|error| format!("zip finish: {}", error))?;
    }
    Ok(cursor.into_inner())
}

/// 🔮️ Removes one named member from an existing ZIP with the registered reference implementation.
#[cfg(feature = "oracles")]
pub fn oracle_remove_zip_entry(input: &[u8], name: &str) -> Result<Vec<u8>, String> {
    let mut kept = read_zip_entries(input)?;
    kept.retain(|entry| entry.name != name);
    oracle_create_zip(&ArchiveSpec { entries: kept })
}

/// 👁️ Projects ZIP bytes with the INDEPENDENT reader onto the owned `semantic-archive-v1` shape.
#[cfg(feature = "oracles")]
pub fn project_zip(input: &[u8]) -> Result<Json, String> {
    Ok(ArchiveSpec { entries: read_zip_entries(input)? }.projection())
}

#[cfg(feature = "oracles")]
fn read_zip_entries(input: &[u8]) -> Result<Vec<ArchiveEntry>, String> {
    use std::io::Read;
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(input.to_vec())).map_err(|error| format!("independent reader could not parse the ZIP: {}", error))?;
    let mut entries = Vec::new();
    for index in 0..archive.len() {
        let mut member = archive.by_index(index).map_err(|error| format!("independent reader could not read ZIP entry {}: {}", index, error))?;
        if member.is_dir() {
            continue;
        }
        let name = member.name().to_string();
        let mut bytes = Vec::new();
        member.read_to_end(&mut bytes).map_err(|error| format!("independent reader could not decompress {}: {}", name, error))?;
        entries.push(ArchiveEntry { name, bytes });
    }
    Ok(entries)
}
//#endregion 🔖️Zip

//#region 🔖️Deflate
/// 🔮️ Compresses a byte stream as zlib (RFC 1950) with the registered `flate2` reference implementation.
/// @see https://github.com/rust-lang/flate2-rs
#[cfg(feature = "oracles")]
pub fn oracle_zlib_compress(input: &[u8]) -> Result<Vec<u8>, String> {
    use std::io::Write;
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(input).map_err(|error| format!("zlib write: {}", error))?;
    encoder.finish().map_err(|error| format!("zlib finish: {}", error))
}

/// 👁️ Projects a zlib stream by DECOMPRESSING it with the independent reader. Compression level and
/// the exact encoded bytes are writer choices; only the recovered payload is normative.
#[cfg(feature = "oracles")]
pub fn project_zlib(input: &[u8]) -> Result<Json, String> {
    use std::io::Read;
    let mut decoder = flate2::read::ZlibDecoder::new(input);
    let mut out = Vec::new();
    decoder.read_to_end(&mut out).map_err(|error| format!("independent reader could not inflate the stream: {}", error))?;
    Ok(Json::Object(vec![
        ("format".to_string(), Json::String("zlib".to_string())),
        ("inflatedSize".to_string(), Json::Number(out.len() as f64)),
        ("inflatedDigest".to_string(), Json::String(digest(&out))),
        ("roundTrips".to_string(), Json::Bool(true)),
    ]))
}
//#endregion 🔖️Deflate

//#region 🔖️Unavailable
/// 🚫️ Without the `oracles` feature nothing here is linked, and every entry point fails loudly.
#[cfg(not(feature = "oracles"))]
mod unavailable {
    use super::{ArchiveSpec, Json};
    const MESSAGE: &str = "the `oracles` feature is disabled — this host was not built with the registered reference implementations";

    pub fn create_zip(_spec: &ArchiveSpec) -> Result<Vec<u8>, String> {
        Err(MESSAGE.to_string())
    }
    pub fn remove_zip_entry(_input: &[u8], _name: &str) -> Result<Vec<u8>, String> {
        Err(MESSAGE.to_string())
    }
    pub fn project_zip(_input: &[u8]) -> Result<Json, String> {
        Err(MESSAGE.to_string())
    }
    pub fn zlib_compress(_input: &[u8]) -> Result<Vec<u8>, String> {
        Err(MESSAGE.to_string())
    }
    pub fn project_zlib(_input: &[u8]) -> Result<Json, String> {
        Err(MESSAGE.to_string())
    }
}

#[cfg(not(feature = "oracles"))]
pub use unavailable::{create_zip as oracle_create_zip, project_zip, project_zlib, remove_zip_entry as oracle_remove_zip_entry, zlib_compress as oracle_zlib_compress};
//#endregion 🔖️Unavailable
