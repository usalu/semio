//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered reference implementation so the subject's own mutation has an independent result to
//! be compared against instead of being checked against its own reading.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared `archive` module rather than by copying it. This subset's vocabulary additionally
//! carries the archive-level comment (`SetArchiveComment`) and a whole-snapshot replacement
//! (`SetSnapshot`), neither of which the shared module's `ArchiveSpec` projects, so this file reads
//! and writes the reference `zip` crate directly rather than routing through it.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself.

use semio_repo_test_host::Json;

//#region 🔖️Types
/// 🎒️ One archive member as read by the independent `zip` reference reader.
#[derive(Debug, Clone)]
pub struct MutationEntry {
    pub name: String,
    pub data: Vec<u8>,
}

/// 🎒️ The full archive shape this subset's vocabulary mutates: members plus the archive-level
/// (EOCD) comment.
#[derive(Debug, Clone, Default)]
pub struct MutationArchive {
    pub entries: Vec<MutationEntry>,
    pub comment: String,
}
//#endregion 🔖️Types

//#region 🔖️Live
#[cfg(feature = "oracles")]
mod live {
    use super::{MutationArchive, MutationEntry};
    use semio_repo_test_host::{digest, Json};

    //#region 🔖️Codec
    /// 🔮️ Reads every member and the archive comment with the registered `zip` reference reader.
    pub fn read_archive(input: &[u8]) -> Result<MutationArchive, String> {
        use std::io::Read;
        let mut archive = zip::ZipArchive::new(std::io::Cursor::new(input.to_vec())).map_err(|error| format!("independent reader could not parse the ZIP: {error}"))?;
        let comment = String::from_utf8_lossy(archive.comment()).into_owned();
        let mut entries = Vec::with_capacity(archive.len());
        for index in 0..archive.len() {
            let mut member = archive.by_index(index).map_err(|error| format!("independent reader could not read ZIP entry {index}: {error}"))?;
            if member.is_dir() {
                continue;
            }
            let name = member.name().to_string();
            let mut data = Vec::new();
            member.read_to_end(&mut data).map_err(|error| format!("independent reader could not decompress {name}: {error}"))?;
            entries.push(MutationEntry { name, data });
        }
        Ok(MutationArchive { entries, comment })
    }

    /// 🔮️ Writes every member and the archive comment with the registered `zip` reference writer.
    pub fn write_archive(archive: &MutationArchive) -> Result<Vec<u8>, String> {
        use std::io::Write;
        let mut writer = zip::ZipWriter::new(std::io::Cursor::new(Vec::<u8>::new()));
        let options: zip::write::FileOptions<'_, ()> = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
        for entry in &archive.entries {
            writer.start_file(entry.name.clone(), options).map_err(|error| format!("zip start_file {}: {error}", entry.name))?;
            writer.write_all(&entry.data).map_err(|error| format!("zip write {}: {error}", entry.name))?;
        }
        if !archive.comment.is_empty() {
            writer.set_comment(archive.comment.clone());
        }
        let cursor = writer.finish().map_err(|error| format!("zip finish: {error}"))?;
        Ok(cursor.into_inner())
    }

    /// 👁️ Projects an archive onto the owned `semantic-archive-mutate-v1` shape: members compared as
    /// a SET by name/size/digest, plus the comment as a normative field (this profile does not
    /// ignore it — see `../🔣️oracle.json`).
    pub fn projection(archive: &MutationArchive) -> Json {
        Json::Object(vec![
            ("format".to_string(), Json::String("zip".to_string())),
            ("entryCount".to_string(), Json::Number(archive.entries.len() as f64)),
            ("comment".to_string(), Json::String(archive.comment.clone())),
            (
                "entries".to_string(),
                Json::Array(
                    archive
                        .entries
                        .iter()
                        .map(|entry| Json::Object(vec![("name".to_string(), Json::String(entry.name.clone())), ("size".to_string(), Json::Number(entry.data.len() as f64)), ("contentDigest".to_string(), Json::String(digest(&entry.data)))]))
                        .collect(),
                ),
            ),
        ])
    }
    //#endregion 🔖️Codec

    //#region 🔖️Forward
    /// 🦠️ Applies one declared mutation kind, described by `spec` (`{"kind": ..., "params": {...}}`),
    /// to an already-decoded archive. An unrecognised kind, or a named entry that does not exist, is
    /// an error — never a silent no-op.
    pub fn apply(mut archive: MutationArchive, spec: &Json) -> Result<MutationArchive, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
        match spec.str("kind").as_str() {
            "no-mutation" => Ok(archive),
            "set-snapshot" => {
                archive.entries = params.array("entries").iter().map(|entry| MutationEntry { name: entry.str("name"), data: entry.str("content").into_bytes() }).collect();
                archive.comment = params.str("comment");
                Ok(archive)
            }
            "set-archive-comment" => {
                archive.comment = params.str("comment");
                Ok(archive)
            }
            "add-entry" => {
                let name = params.str("name");
                if archive.entries.iter().any(|entry| entry.name == name) {
                    return Err(format!("add-entry: an entry named {name:?} already exists"));
                }
                archive.entries.push(MutationEntry { name, data: params.str("content").into_bytes() });
                Ok(archive)
            }
            "remove-entry" => {
                let name = params.str("name");
                let before = archive.entries.len();
                archive.entries.retain(|entry| entry.name != name);
                if archive.entries.len() == before {
                    return Err(format!("remove-entry: no entry named {name:?}"));
                }
                Ok(archive)
            }
            "rename-entry" => {
                let name = params.str("name");
                let new_name = params.str("newName");
                match archive.entries.iter_mut().find(|entry| entry.name == name) {
                    Some(entry) => {
                        entry.name = new_name;
                        Ok(archive)
                    }
                    None => Err(format!("rename-entry: no entry named {name:?}")),
                }
            }
            "set-entry-data" => {
                let name = params.str("name");
                let content = params.str("content").into_bytes();
                match archive.entries.iter_mut().find(|entry| entry.name == name) {
                    Some(entry) => {
                        entry.data = content;
                        Ok(archive)
                    }
                    None => Err(format!("set-entry-data: no entry named {name:?}")),
                }
            }
            kind => Err(format!("mutation kind {kind:?} has no oracle implementation")),
        }
    }
    //#endregion 🔖️Forward

    //#region 🔖️Inverse
    /// ↩️ Computes and applies this mutation kind's OWN inverse against the already-mutated archive,
    /// sourcing whatever the forward mutation discarded (a removed member's bytes, an overwritten
    /// member's original bytes, the original comment, the original whole snapshot) from `original` —
    /// the archive as it stood before the forward mutation ran. Mirrors the algebra
    /// `../🧬️schema/🧬️mutations/🦀️.rs`'s `ZipMutation::inverse` defines for the subject,
    /// computed independently here so the property has two producers to disagree.
    pub fn invert(original: &MutationArchive, mutated: MutationArchive, spec: &Json) -> Result<MutationArchive, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
        match spec.str("kind").as_str() {
            "no-mutation" => Ok(mutated),
            "set-snapshot" => Ok(original.clone()),
            "set-archive-comment" => Ok(MutationArchive { comment: original.comment.clone(), ..mutated }),
            "add-entry" => {
                let name = params.str("name");
                let mut restored = mutated;
                let before = restored.entries.len();
                restored.entries.retain(|entry| entry.name != name);
                if restored.entries.len() == before {
                    return Err(format!("inverse add-entry: no entry named {name:?} to remove"));
                }
                Ok(restored)
            }
            "remove-entry" => {
                let name = params.str("name");
                let removed = original.entries.iter().find(|entry| entry.name == name).cloned().ok_or_else(|| format!("inverse remove-entry: original archive has no entry named {name:?}"))?;
                let mut restored = mutated;
                restored.entries.push(removed);
                Ok(restored)
            }
            "rename-entry" => {
                let name = params.str("name");
                let new_name = params.str("newName");
                let mut restored = mutated;
                match restored.entries.iter_mut().find(|entry| entry.name == new_name) {
                    Some(entry) => {
                        entry.name = name;
                        Ok(restored)
                    }
                    None => Err(format!("inverse rename-entry: no entry named {new_name:?}")),
                }
            }
            "set-entry-data" => {
                let name = params.str("name");
                let original_data = original.entries.iter().find(|entry| entry.name == name).map(|entry| entry.data.clone()).ok_or_else(|| format!("inverse set-entry-data: original archive has no entry named {name:?}"))?;
                let mut restored = mutated;
                match restored.entries.iter_mut().find(|entry| entry.name == name) {
                    Some(entry) => {
                        entry.data = original_data;
                        Ok(restored)
                    }
                    None => Err(format!("inverse set-entry-data: no entry named {name:?}")),
                }
            }
            kind => Err(format!("mutation kind {kind:?} has no oracle inverse implementation")),
        }
    }
    //#endregion 🔖️Inverse
}
//#endregion 🔖️Live

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    live::write_archive(&live::apply(live::read_archive(input)?, spec)?)
}

/// ↩️ Applies `spec`'s kind to `original`, then undoes it with the mutation's own inverse computed
/// against `mutated` (the result of that same forward application) — the property this subset's
/// `inverse-<kind>` scenarios check. Kept byte-exact by sourcing restored payloads straight from the
/// decoded `original`, never by round-tripping binary content through JSON.
#[cfg(feature = "oracles")]
pub fn oracle_apply_inverse(original: &[u8], mutated: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let original_archive = live::read_archive(original)?;
    let mutated_archive = live::read_archive(mutated)?;
    live::write_archive(&live::invert(&original_archive, mutated_archive, spec)?)
}

/// 🔁️ Decodes with the independent reader and re-encodes with the reference writer, no mutation
/// applied — the identity round trip this subset's `identity-round-trip` scenario checks.
#[cfg(feature = "oracles")]
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    live::write_archive(&live::read_archive(input)?)
}

/// 👁️ Projects ZIP bytes with the INDEPENDENT reader onto the `semantic-archive-mutate-v1` shape,
/// for every one of this subset's oracle outcomes.
#[cfg(feature = "oracles")]
pub fn project_zip_mutation(input: &[u8]) -> Result<Json, String> {
    Ok(live::projection(&live::read_archive(input)?))
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_inverse(_original: &[u8], _mutated: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn project_zip_mutation(_input: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch
