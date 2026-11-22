//! 🗜️ Bounded content check of a zip-based (OPC) file before any document is created: the central directory names `[Content_Types].xml`, its `Override` for a declared main part content type names the main part, and the root namespace of that part selects the dialect. Reads only the zip tail, the central directory and two small entries and never materializes the whole file. Rust twin of the TypeScript `ShellHelpers/🗜️package-sniff`, driven by the shared `package-routing` fixture.

use semio_framework_deflate::inflate_head;
use std::collections::HashMap;

/// 📜️ What a format claims about its package: the main part content types that make the file a valid package of this format, and the dialect each root namespace of the main part routes to.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PackageClaim {
    pub main_part_content_types: Vec<String>,
    pub dialects: Vec<PackageDialect>,
}

/// 🧭️ One dialect coordinate and the root namespace that selects it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackageDialect {
    pub artifact_ref: String,
    pub namespace: String,
}

/// 📖️ Random access to the picked bytes (`end` exclusive) and the cancellation poll of the task reading them; a read that fails is `None`.
pub trait PackageSource {
    fn size(&self) -> u64;
    fn read(&self, start: u64, end: u64) -> Option<Vec<u8>>;
    fn cancelled(&self) -> bool;
}

/// ⚖️ `Mismatch`: not a package of the claimed format. `Match`: it is, and `artifact_ref` is the dialect its root namespace selects (`None`: none of the declared namespaces, so the format owner decides).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PackageVerdict {
    Mismatch,
    Match { artifact_ref: Option<String> },
}

/// 🛑️ The check stopped because the task was cancelled; never a verdict.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PackageSniffCancelled;

/// 🔬️ Read bounds of the check.
pub const PACKAGE_TAIL_BYTES: u64 = 22 + 65535;
pub const PACKAGE_CENTRAL_DIRECTORY_BYTES: u64 = 8 * 1024 * 1024;
pub const PACKAGE_CONTENT_TYPES_BYTES: usize = 2 * 1024 * 1024;
pub const PACKAGE_ROOT_BYTES: usize = 8192;
pub const PACKAGE_COMPRESSED_BYTES: u64 = 8 * 1024 * 1024;

struct Entry {
    method: u16,
    compressed: u64,
    offset: u64,
}

fn u16_at(bytes: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([bytes[at], bytes[at + 1]])
}

fn u32_at(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
}

fn checkpoint(source: &dyn PackageSource) -> Result<(), PackageSniffCancelled> {
    if source.cancelled() { Err(PackageSniffCancelled) } else { Ok(()) }
}

fn read_central_directory(source: &dyn PackageSource) -> Result<Option<HashMap<String, Entry>>, PackageSniffCancelled> {
    let size = source.size();
    if size < 22 {
        return Ok(None);
    }
    checkpoint(source)?;
    let Some(tail) = source.read(size.saturating_sub(PACKAGE_TAIL_BYTES), size) else { return Ok(None) };
    let Some(end) = (0..=tail.len().saturating_sub(22)).rev().find(|&at| tail.len() >= 22 && u32_at(&tail, at) == 0x0605_4b50 && at + 22 + usize::from(u16_at(&tail, at + 20)) == tail.len()) else { return Ok(None) };
    let count = u16_at(&tail, end + 10);
    let (length, offset) = (u64::from(u32_at(&tail, end + 12)), u64::from(u32_at(&tail, end + 16)));
    if count == 0xffff || length == 0xffff_ffff || offset == 0xffff_ffff || length > PACKAGE_CENTRAL_DIRECTORY_BYTES || offset + length > size {
        return Ok(None);
    }
    checkpoint(source)?;
    let Some(directory) = source.read(offset, offset + length) else { return Ok(None) };
    let mut entries = HashMap::new();
    let mut at = 0usize;
    for _ in 0..count {
        if at + 46 > directory.len() || u32_at(&directory, at) != 0x0201_4b50 {
            return Ok(None);
        }
        let (name, extra, comment) = (usize::from(u16_at(&directory, at + 28)), usize::from(u16_at(&directory, at + 30)), usize::from(u16_at(&directory, at + 32)));
        if at + 46 + name > directory.len() {
            return Ok(None);
        }
        entries.insert(String::from_utf8_lossy(&directory[at + 46..at + 46 + name]).to_lowercase(), Entry { method: u16_at(&directory, at + 10), compressed: u64::from(u32_at(&directory, at + 20)), offset: u64::from(u32_at(&directory, at + 42)) });
        at += 46 + name + extra + comment;
    }
    Ok(Some(entries))
}

fn read_entry(source: &dyn PackageSource, entry: &Entry, limit: usize) -> Result<Option<(Vec<u8>, bool)>, PackageSniffCancelled> {
    if entry.compressed > PACKAGE_COMPRESSED_BYTES || entry.offset + 30 > source.size() {
        return Ok(None);
    }
    checkpoint(source)?;
    let Some(header) = source.read(entry.offset, entry.offset + 30).filter(|header| header.len() >= 30 && u32_at(header, 0) == 0x0403_4b50) else { return Ok(None) };
    let start = entry.offset + 30 + u64::from(u16_at(&header, 26)) + u64::from(u16_at(&header, 28));
    if start + entry.compressed > source.size() {
        return Ok(None);
    }
    checkpoint(source)?;
    Ok(match entry.method {
        0 => source.read(start, start + entry.compressed.min(limit as u64)).map(|head| (head, entry.compressed > limit as u64)),
        8 => source.read(start, start + entry.compressed).and_then(|compressed| inflate_head(&compressed, limit).ok()),
        _ => None,
    })
}

fn is_word(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn override_tags(text: &str) -> Vec<&str> {
    let lower = text.to_ascii_lowercase();
    let bytes = lower.as_bytes();
    let mut tags = Vec::new();
    let mut from = 0;
    while let Some(found) = lower[from..].find("<override") {
        let start = from + found;
        let after = start + "<override".len();
        from = after;
        if bytes.get(after).is_some_and(|byte| is_word(*byte)) {
            continue;
        }
        if let Some(close) = lower[after..].find('>') {
            tags.push(&text[start..after + close + 1]);
            from = after + close + 1;
        }
    }
    tags
}

fn attribute(tag: &str, name: &str) -> Option<String> {
    let lower = tag.to_ascii_lowercase();
    let bytes = tag.as_bytes();
    let mut from = 0;
    while let Some(found) = lower[from..].find(name) {
        let start = from + found;
        from = start + 1;
        if start > 0 && is_word(bytes[start - 1]) {
            continue;
        }
        let rest = tag[start + name.len()..].trim_start();
        let Some(rest) = rest.strip_prefix('=') else { continue };
        let rest = rest.trim_start();
        let Some(quote) = rest.chars().next().filter(|quote| *quote == '"' || *quote == '\'') else { continue };
        if let Some(close) = rest[1..].find(quote) {
            return Some(rest[1..1 + close].to_string());
        }
    }
    None
}

/// 🧮️ Checks one picked package against the claim of its format; unreadable, truncated, oversized or foreign content is a `Mismatch`, never a guess.
pub fn inspect_package(source: &dyn PackageSource, claim: &PackageClaim) -> Result<PackageVerdict, PackageSniffCancelled> {
    let Some(entries) = read_central_directory(source)? else { return Ok(PackageVerdict::Mismatch) };
    let Some(types_entry) = entries.get("[content_types].xml") else { return Ok(PackageVerdict::Mismatch) };
    let Some((types, false)) = read_entry(source, types_entry, PACKAGE_CONTENT_TYPES_BYTES)? else { return Ok(PackageVerdict::Mismatch) };
    let wanted: Vec<String> = claim.main_part_content_types.iter().map(|content_type| content_type.to_lowercase()).collect();
    let text = String::from_utf8_lossy(&types);
    let main_part = override_tags(&text).into_iter().find_map(|tag| {
        let content_type = attribute(tag, "contenttype")?.split(';').next().unwrap_or_default().trim().to_lowercase();
        let part_name = attribute(tag, "partname")?;
        wanted.contains(&content_type).then(|| part_name.trim_start_matches('/').to_lowercase())
    });
    let Some(main_entry) = main_part.and_then(|part| entries.get(&part)) else { return Ok(PackageVerdict::Mismatch) };
    let Some((root, _)) = read_entry(source, main_entry, PACKAGE_ROOT_BYTES)? else { return Ok(PackageVerdict::Mismatch) };
    let head = String::from_utf8_lossy(&root);
    Ok(PackageVerdict::Match { artifact_ref: claim.dialects.iter().find(|dialect| head.contains(dialect.namespace.as_str())).map(|dialect| dialect.artifact_ref.clone()) })
}
