//! ⚙️ ZipEngine — ZIP store+deflate, CRC32 hand-rolled; uses deflate artifact.

use crate::artifacts::zip::{ZipArtifact, ZipDiff, ZipMutation, ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};
use crate::artifacts::zip::schema::snapshot::ZipEntry;

//#region CRC32
fn crc32_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    for i in 0..256u32 {
        let mut c = i;
        for _ in 0..8 {
            if c & 1 != 0 {
                c = 0xEDB88320 ^ (c >> 1);
            } else {
                c >>= 1;
            }
        }
        table[i as usize] = c;
    }
    table
}

/// 🧮 CRC-32 (ISO-HDLC / ZIP).
pub fn crc32(data: &[u8]) -> u32 {
    let table = crc32_table();
    let mut c = 0xFFFFFFFFu32;
    for &b in data {
        c = table[((c ^ b as u32) & 0xFF) as usize] ^ (c >> 8);
    }
    c ^ 0xFFFFFFFF
}
//#endregion CRC32

//#region ZipCodec
fn u16_le(n: u16) -> [u8; 2] { n.to_le_bytes() }
fn u32_le(n: u32) -> [u8; 4] { n.to_le_bytes() }

fn read_u16(data: &[u8], off: usize) -> Result<u16, String> {
    if off + 2 > data.len() { return Err("truncated u16".into()); }
    Ok(u16::from_le_bytes([data[off], data[off + 1]]))
}
fn read_u32(data: &[u8], off: usize) -> Result<u32, String> {
    if off + 4 > data.len() { return Err("truncated u32".into()); }
    Ok(u32::from_le_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]]))
}

/// 🎒️ Encode ZipSnapshot entries as a ZIP container.
/// When `use_deflate` is true, entry payloads use method 8 via deflate_raw.
pub fn encode_zip(snapshot: &ZipSnapshot, use_deflate: bool) -> Result<Vec<u8>, String> {
    let mut locals = Vec::new();
    let mut central = Vec::new();
    let mut offset = 0u32;

    for entry in &snapshot.entries {
        let name = entry.name.as_bytes();
        if name.len() > u16::MAX as usize {
            return Err("entry name too long".into());
        }
        let crc = crc32(&entry.data);
        let (method, payload): (u16, Vec<u8>) = if use_deflate {
            (8, crate::artifacts::deflate::engine::deflate_raw(&entry.data))
        } else {
            (0, entry.data.clone())
        };
        let comp_size = payload.len() as u32;
        let uncomp_size = entry.data.len() as u32;
        let name_len = name.len() as u16;

        let mut local = Vec::new();
        local.extend_from_slice(&u32_le(0x04034b50));
        local.extend_from_slice(&u16_le(20)); // version needed
        local.extend_from_slice(&u16_le(0)); // flags
        local.extend_from_slice(&u16_le(method));
        local.extend_from_slice(&u16_le(0)); // time
        local.extend_from_slice(&u16_le(0)); // date
        local.extend_from_slice(&u32_le(crc));
        local.extend_from_slice(&u32_le(comp_size));
        local.extend_from_slice(&u32_le(uncomp_size));
        local.extend_from_slice(&u16_le(name_len));
        local.extend_from_slice(&u16_le(0)); // extra
        local.extend_from_slice(name);
        local.extend_from_slice(&payload);

        let mut cen = Vec::new();
        cen.extend_from_slice(&u32_le(0x02014b50));
        cen.extend_from_slice(&u16_le(20)); // version made
        cen.extend_from_slice(&u16_le(20)); // version needed
        cen.extend_from_slice(&u16_le(0));
        cen.extend_from_slice(&u16_le(method));
        cen.extend_from_slice(&u16_le(0));
        cen.extend_from_slice(&u16_le(0));
        cen.extend_from_slice(&u32_le(crc));
        cen.extend_from_slice(&u32_le(comp_size));
        cen.extend_from_slice(&u32_le(uncomp_size));
        cen.extend_from_slice(&u16_le(name_len));
        cen.extend_from_slice(&u16_le(0)); // extra
        cen.extend_from_slice(&u16_le(0)); // comment
        cen.extend_from_slice(&u16_le(0)); // disk
        cen.extend_from_slice(&u16_le(0)); // int attrs
        cen.extend_from_slice(&u32_le(0)); // ext attrs
        cen.extend_from_slice(&u32_le(offset));
        cen.extend_from_slice(name);

        offset += local.len() as u32;
        locals.extend_from_slice(&local);
        central.extend_from_slice(&cen);
    }

    let cd_offset = locals.len() as u32;
    let cd_size = central.len() as u32;
    let count = snapshot.entries.len() as u16;
    let mut eocd = Vec::new();
    eocd.extend_from_slice(&u32_le(0x06054b50));
    eocd.extend_from_slice(&u16_le(0));
    eocd.extend_from_slice(&u16_le(0));
    eocd.extend_from_slice(&u16_le(count));
    eocd.extend_from_slice(&u16_le(count));
    eocd.extend_from_slice(&u32_le(cd_size));
    eocd.extend_from_slice(&u32_le(cd_offset));
    eocd.extend_from_slice(&u16_le(0));

    let mut out = locals;
    out.extend_from_slice(&central);
    out.extend_from_slice(&eocd);
    Ok(out)
}

/// 🎒️ Decode ZIP container bytes into a ZipSnapshot (uncompressed entry payloads).
pub fn decode_zip(data: &[u8]) -> Result<ZipSnapshot, String> {
    if data.len() < 22 {
        return Err("zip too short".into());
    }
    // find EOCD
    let mut eocd = None;
    let start = data.len().saturating_sub(22 + 65535);
    for i in (start..=data.len().saturating_sub(22)).rev() {
        if &data[i..i + 4] == [0x50, 0x4b, 0x05, 0x06] {
            eocd = Some(i);
            break;
        }
    }
    let eocd = eocd.ok_or("EOCD not found")?;
    let count = read_u16(data, eocd + 10)? as usize;
    let cd_size = read_u32(data, eocd + 12)? as usize;
    let cd_offset = read_u32(data, eocd + 16)? as usize;
    if cd_offset + cd_size > data.len() {
        return Err("central directory out of range".into());
    }

    let mut entries = Vec::with_capacity(count);
    let mut pos = cd_offset;
    for _ in 0..count {
        if read_u32(data, pos)? != 0x02014b50 {
            return Err("bad central header sig".into());
        }
        let method = read_u16(data, pos + 10)?;
        let crc = read_u32(data, pos + 16)?;
        let comp_size = read_u32(data, pos + 20)? as usize;
        let uncomp_size = read_u32(data, pos + 24)? as usize;
        let name_len = read_u16(data, pos + 28)? as usize;
        let extra_len = read_u16(data, pos + 30)? as usize;
        let comment_len = read_u16(data, pos + 32)? as usize;
        let local_off = read_u32(data, pos + 42)? as usize;
        let name_start = pos + 46;
        let name_end = name_start + name_len;
        if name_end > data.len() {
            return Err("truncated name".into());
        }
        let name = String::from_utf8(data[name_start..name_end].to_vec())
            .map_err(|e| e.to_string())?;
        pos = name_end + extra_len + comment_len;

        if read_u32(data, local_off)? != 0x04034b50 {
            return Err("bad local header sig".into());
        }
        let l_name_len = read_u16(data, local_off + 26)? as usize;
        let l_extra_len = read_u16(data, local_off + 28)? as usize;
        let data_off = local_off + 30 + l_name_len + l_extra_len;
        let data_end = data_off + comp_size;
        if data_end > data.len() {
            return Err("truncated file data".into());
        }
        let payload = &data[data_off..data_end];
        let raw = match method {
            0 => payload.to_vec(),
            8 => crate::artifacts::deflate::engine::inflate_raw(payload)?,
            other => return Err(format!("unsupported zip method {other}")),
        };
        if raw.len() != uncomp_size && uncomp_size != 0 {
            // allow zero uncomp in some writers; still verify crc
        }
        let got = crc32(&raw);
        if got != crc {
            return Err(format!("crc32 mismatch for {name}: expected {crc:#010x}, got {got:#010x}"));
        }
        entries.push(ZipEntry { name, data: raw });
    }

    Ok(ZipSnapshot {
        schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(),
        entries,
    })
}
//#endregion ZipCodec

//#region DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_zip_snapshot() -> ZipSnapshot {
    ZipSnapshot::default()
}
//#endregion DocumentHelpers

//#region Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::zip::io::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::DocumentCodec::of::<ZipSnapshot, ZipMutation>(
        STDIO_ZIP_DOCUMENT_SCHEMA,
    ));
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.zip",
        extension: Some("zip"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::zip::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::zip::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::zip::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::zip::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.zip"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.zip`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(
        crate::artifacts::zip::schema::zip_artifact_schema_descriptor(),
    );
}
//#endregion Register

//#region ArtifactEngine
/// ⚙️ `stdio.zip` artifact engine.
pub struct ZipEngine {
    artifact_state: ZipArtifact,
    snapshot_state: ZipSnapshot,
}

impl ZipEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: ZipSnapshot) -> Self {
        let artifact_state = ZipArtifact::from_snapshot(snapshot.clone());
        Self { artifact_state, snapshot_state: snapshot }
    }
}

impl protocol::ArtifactEngine for ZipEngine {
    type Artifact = ZipArtifact;
    type Snapshot = ZipSnapshot;
    type Mutation = ZipMutation;
    type Diff = ZipDiff;

    fn artifact(&self) -> &Self::Artifact { &self.artifact_state }
    fn snapshot(&self) -> &Self::Snapshot { &self.snapshot_state }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(mutation, &self.snapshot_state);
        self.snapshot_state = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(&diff, &self.snapshot_state);
        self.artifact_state.set_snapshot(self.snapshot_state.clone());
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Snapshot>>::inverse(mutation, &self.snapshot_state)
    }
}
//#endregion ArtifactEngine

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc32_known_vector() {
        // CRC of "123456789" is 0xCBF43926
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
    }

    #[test]
    fn zip_store_round_trip() {
        let snap = ZipSnapshot {
            schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(),
            entries: vec![
                ZipEntry { name: "a.txt".into(), data: b"hello".to_vec() },
                ZipEntry { name: "b/bin.dat".into(), data: vec![0, 1, 2, 3, 255] },
            ],
        };
        let bytes = encode_zip(&snap, false).expect("encode store");
        let decoded = decode_zip(&bytes).expect("decode store");
        assert_eq!(decoded.entries.len(), 2);
        assert_eq!(decoded.entries[0].name, "a.txt");
        assert_eq!(decoded.entries[0].data, b"hello");
        assert_eq!(decoded.entries[1].data, vec![0, 1, 2, 3, 255]);
    }

    #[test]
    fn zip_deflate_round_trip() {
        let snap = ZipSnapshot {
            schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(),
            entries: vec![ZipEntry {
                name: "poem.txt".into(),
                data: b"deflate inside zip via stdio.deflate raw".to_vec(),
            }],
        };
        let bytes = encode_zip(&snap, true).expect("encode deflate");
        let decoded = decode_zip(&bytes).expect("decode deflate");
        assert_eq!(decoded.entries[0].data, snap.entries[0].data);
    }

    #[test]
    fn codec_round_trip() {
        let snap = ZipSnapshot {
            schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(),
            entries: vec![ZipEntry { name: "x".into(), data: b"y".to_vec() }],
        };
        let pack = store::DocumentPack::encode_pack(&snap);
        let decoded = <ZipSnapshot as store::DocumentPack>::decode_pack(&pack).expect("decode");
        assert_eq!(decoded.entries, snap.entries);
    }
}
//#endregion Tests
