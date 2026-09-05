//! 🔮️ Mutation oracle for `stdio.zip` 2.0/🌐️iso21320 — every mutation kind THIS subset declares,
//! performed by the registered `zip` reference implementation so the subject's own mutation has an
//! independent result to be compared against instead of being checked against its own reading.
//!
//! The vocabulary is per SUBSET. ISO/IEC 21320-1:2015 (Document Container File — Part 1: Core) is a
//! RESTRICTION of the ZIP 2.0 container: §4.4 admits exactly two compression methods, Stored (0) and
//! Deflate (8), and §4.1 forbids encryption outright. `🧱️base`'s `add-entry` declares no method at
//! all — the wire method is an accident of the canonical serializer — so this subset splits it into
//! `add-stored-entry` and `add-deflated-entry`, which is the profile's defining constraint made
//! representable instead of implicit. The subset's own production builder already declares that
//! distinction as `with_stored_entry`/`with_deflate_entry`; see this module's routing note and the
//! case's feature description for what is and is not honoured on the subject side today.
//!
//! This module reads and writes the reference `zip` crate directly rather than routing through the
//! shared `🎒️archive` family module, for the same reason `🧱️base`'s own oracle does: that module's
//! `ArchiveSpec` projects neither the archive-level comment nor per-entry ISO conformance, both of
//! which this subset's vocabulary and comparison profile depend on.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the vocabulary itself (`ZipIso21320Mutation::KINDS`).

use semio_repo_test_host::Json;

//#region 🔖️Types
/// 🎒️ The two compression methods ISO/IEC 21320-1 §4.4 admits. Every other ZIP method is
/// unrepresentable here by construction, which is the profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsoMethod {
    Stored,
    Deflate,
}

/// 🎒️ One archive member as read by the independent `zip` reference reader, carrying the wire
/// compression method the profile constrains.
#[derive(Debug, Clone)]
pub struct IsoEntry {
    pub name: String,
    pub data: Vec<u8>,
    pub method: IsoMethod,
    pub encrypted: bool,
}

/// 🎒️ The full archive shape this subset's vocabulary mutates: members plus the archive-level
/// (EOCD) comment.
#[derive(Debug, Clone, Default)]
pub struct IsoArchive {
    pub entries: Vec<IsoEntry>,
    pub comment: String,
}
//#endregion 🔖️Types

//#region 🔖️Live
#[cfg(feature = "oracles")]
mod live {
    use super::{IsoArchive, IsoEntry, IsoMethod};
    use semio_repo_test_host::{digest, Json};

    //#region 🔖️Codec
    /// 🔮️ Reads every member, its wire compression method, its encryption bit and the archive
    /// comment with the registered `zip` reference reader. A member compressed with anything other
    /// than Stored or Deflate is an ISO/IEC 21320-1 violation in the INPUT, reported as an error
    /// rather than silently normalized — the profile's whole point.
    pub fn read_archive(input: &[u8]) -> Result<IsoArchive, String> {
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
            let encrypted = member.encrypted();
            let method = match member.compression() {
                zip::CompressionMethod::Stored => IsoMethod::Stored,
                zip::CompressionMethod::Deflated => IsoMethod::Deflate,
                other => return Err(format!("ISO/IEC 21320-1 §4.4 admits only Stored and Deflate; entry {name:?} is compressed with {other:?}")),
            };
            let mut data = Vec::new();
            member.read_to_end(&mut data).map_err(|error| format!("independent reader could not decompress {name}: {error}"))?;
            entries.push(IsoEntry { name, data, method, encrypted });
        }
        Ok(IsoArchive { entries, comment })
    }

    /// 🔮️ Writes every member with the method the profile declared for it, and the archive comment,
    /// through the registered `zip` reference writer.
    pub fn write_archive(archive: &IsoArchive) -> Result<Vec<u8>, String> {
        use std::io::Write;
        let mut writer = zip::ZipWriter::new(std::io::Cursor::new(Vec::<u8>::new()));
        for entry in &archive.entries {
            let method = match entry.method {
                IsoMethod::Stored => zip::CompressionMethod::Stored,
                IsoMethod::Deflate => zip::CompressionMethod::Deflated,
            };
            let options: zip::write::FileOptions<'_, ()> = zip::write::FileOptions::default().compression_method(method);
            writer.start_file(entry.name.clone(), options).map_err(|error| format!("zip start_file {}: {error}", entry.name))?;
            writer.write_all(&entry.data).map_err(|error| format!("zip write {}: {error}", entry.name))?;
        }
        if !archive.comment.is_empty() {
            writer.set_comment(archive.comment.clone());
        }
        let cursor = writer.finish().map_err(|error| format!("zip finish: {error}"))?;
        Ok(cursor.into_inner())
    }

    /// 👁️ Projects an archive onto the owned `semantic-zip-iso21320-v1` shape: members as a SET by
    /// name/size/digest, the comment as a normative field, and — this profile's own addition — the
    /// ISO/IEC 21320-1 predicates per member.
    ///
    /// The predicate, not the method. Both Stored and Deflate are legal under §4.4, so WHICH of the
    /// two a writer picks for a given member is writer freedom; comparing the method itself would
    /// compare this repository's canonical serialization policy against a copy of that same policy
    /// planted in the oracle. What ISO actually fixes — that the method is one of exactly those two,
    /// and that no member is encrypted — is what is compared.
    pub fn projection(archive: &IsoArchive) -> Json {
        Json::Object(vec![
            ("format".to_string(), Json::String("zip-iso21320".to_string())),
            ("entryCount".to_string(), Json::Number(archive.entries.len() as f64)),
            ("comment".to_string(), Json::String(archive.comment.clone())),
            ("encryptedEntryCount".to_string(), Json::Number(archive.entries.iter().filter(|entry| entry.encrypted).count() as f64)),
            (
                "entries".to_string(),
                Json::Array(
                    archive
                        .entries
                        .iter()
                        .map(|entry| {
                            Json::Object(vec![
                                ("name".to_string(), Json::String(entry.name.clone())),
                                ("size".to_string(), Json::Number(entry.data.len() as f64)),
                                ("contentDigest".to_string(), Json::String(digest(&entry.data))),
                                ("isoCompressionAllowed".to_string(), Json::Bool(matches!(entry.method, IsoMethod::Stored | IsoMethod::Deflate))),
                                ("encrypted".to_string(), Json::Bool(entry.encrypted)),
                            ])
                        })
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
    pub fn apply(mut archive: IsoArchive, spec: &Json) -> Result<IsoArchive, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
        let add = |archive: &mut IsoArchive, method: IsoMethod| -> Result<(), String> {
            let name = params.str("name");
            if archive.entries.iter().any(|entry| entry.name == name) {
                return Err(format!("add entry: a member named {name:?} already exists"));
            }
            archive.entries.push(IsoEntry { name, data: params.str("content").into_bytes(), method, encrypted: false });
            Ok(())
        };
        match spec.str("kind").as_str() {
            "no-mutation" => Ok(archive),
            "set-snapshot" => {
                archive.entries = params
                    .array("entries")
                    .iter()
                    .map(|entry| IsoEntry {
                        name: entry.str("name"),
                        data: entry.str("content").into_bytes(),
                        method: if entry.str("method") == "stored" { IsoMethod::Stored } else { IsoMethod::Deflate },
                        encrypted: false,
                    })
                    .collect();
                archive.comment = params.str("comment");
                Ok(archive)
            }
            "set-archive-comment" => {
                archive.comment = params.str("comment");
                Ok(archive)
            }
            "add-stored-entry" => {
                add(&mut archive, IsoMethod::Stored)?;
                Ok(archive)
            }
            "add-deflated-entry" => {
                add(&mut archive, IsoMethod::Deflate)?;
                Ok(archive)
            }
            "remove-entry" => {
                let name = params.str("name");
                let before = archive.entries.len();
                archive.entries.retain(|entry| entry.name != name);
                if archive.entries.len() == before {
                    return Err(format!("remove-entry: no member named {name:?}"));
                }
                Ok(archive)
            }
            "rename-entry" => {
                let name = params.str("name");
                let new_name = params.str("newName");
                if archive.entries.iter().any(|entry| entry.name == new_name) {
                    return Err(format!("rename-entry: a member named {new_name:?} already exists"));
                }
                match archive.entries.iter_mut().find(|entry| entry.name == name) {
                    Some(entry) => {
                        entry.name = new_name;
                        Ok(archive)
                    }
                    None => Err(format!("rename-entry: no member named {name:?}")),
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
                    None => Err(format!("set-entry-data: no member named {name:?}")),
                }
            }
            kind => Err(format!("mutation kind {kind:?} has no oracle implementation")),
        }
    }
    //#endregion 🔖️Forward

    //#region 🔖️Inverse
    /// ↩️ Computes and applies this kind's OWN inverse against the already-mutated archive, sourcing
    /// whatever the forward mutation discarded from `original` — the archive as it stood before the
    /// forward mutation ran.
    pub fn invert(original: &IsoArchive, mutated: IsoArchive, spec: &Json) -> Result<IsoArchive, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
        match spec.str("kind").as_str() {
            "no-mutation" => Ok(mutated),
            "set-snapshot" => Ok(original.clone()),
            "set-archive-comment" => Ok(IsoArchive { comment: original.comment.clone(), ..mutated }),
            "add-stored-entry" | "add-deflated-entry" => {
                let name = params.str("name");
                let mut restored = mutated;
                let before = restored.entries.len();
                restored.entries.retain(|entry| entry.name != name);
                if restored.entries.len() == before {
                    return Err(format!("inverse add entry: no member named {name:?} to remove"));
                }
                Ok(restored)
            }
            "remove-entry" => {
                let name = params.str("name");
                let removed = original.entries.iter().find(|entry| entry.name == name).cloned().ok_or_else(|| format!("inverse remove-entry: the original archive has no member named {name:?}"))?;
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
                    None => Err(format!("inverse rename-entry: no member named {new_name:?}")),
                }
            }
            "set-entry-data" => {
                let name = params.str("name");
                let original_data = original.entries.iter().find(|entry| entry.name == name).map(|entry| entry.data.clone()).ok_or_else(|| format!("inverse set-entry-data: the original archive has no member named {name:?}"))?;
                let mut restored = mutated;
                match restored.entries.iter_mut().find(|entry| entry.name == name) {
                    Some(entry) => {
                        entry.data = original_data;
                        Ok(restored)
                    }
                    None => Err(format!("inverse set-entry-data: no member named {name:?}")),
                }
            }
            kind => Err(format!("mutation kind {kind:?} has no oracle inverse implementation")),
        }
    }
    //#endregion 🔖️Inverse

    #[cfg(test)]
    mod tests {
        use super::*;

        fn archive() -> IsoArchive {
            IsoArchive {
                entries: vec![IsoEntry { name: "bild.jpg".into(), data: b"jpegbytes".to_vec(), method: IsoMethod::Stored, encrypted: false }, IsoEntry { name: "notiz.txt".into(), data: b"text".to_vec(), method: IsoMethod::Deflate, encrypted: false }],
                comment: "Bestand".into(),
            }
        }

        fn spec(kind: &str, params: Vec<(&str, Json)>) -> Json {
            Json::Object(vec![("kind".into(), Json::String(kind.into())), ("params".into(), Json::Object(params.into_iter().map(|(k, v)| (k.to_string(), v)).collect()))])
        }

        #[test]
        fn both_declared_methods_survive_a_reference_write_and_read() {
            let bytes = write_archive(&archive()).expect("writes");
            let read = read_archive(&bytes).expect("reads");
            assert_eq!(read.entries.iter().find(|entry| entry.name == "bild.jpg").expect("member").method, IsoMethod::Stored);
            assert_eq!(read.entries.iter().find(|entry| entry.name == "notiz.txt").expect("member").method, IsoMethod::Deflate);
        }

        #[test]
        fn add_stored_entry_and_add_deflated_entry_are_different_operations() {
            let stored = apply(archive(), &spec("add-stored-entry", vec![("name", Json::String("neu.bin".into())), ("content", Json::String("payload".into()))])).expect("applies");
            let deflated = apply(archive(), &spec("add-deflated-entry", vec![("name", Json::String("neu.bin".into())), ("content", Json::String("payload".into()))])).expect("applies");
            assert_eq!(stored.entries.last().expect("member").method, IsoMethod::Stored);
            assert_eq!(deflated.entries.last().expect("member").method, IsoMethod::Deflate);
        }

        #[test]
        fn unknown_kind_is_an_error_not_a_no_op() {
            assert!(apply(archive(), &spec("add-entry", vec![])).is_err(), "the parent subset's ungated add-entry is not part of this vocabulary");
        }

        #[test]
        fn every_declared_kind_round_trips_through_its_own_inverse() {
            let specs = vec![
                spec("no-mutation", vec![]),
                spec("set-snapshot", vec![("entries", Json::Array(vec![Json::Object(vec![("name".into(), Json::String("x".into())), ("content".into(), Json::String("y".into()))])])), ("comment", Json::String("neu".into()))]),
                spec("set-archive-comment", vec![("comment", Json::String("geaendert".into()))]),
                spec("add-stored-entry", vec![("name", Json::String("a.png".into())), ("content", Json::String("p".into()))]),
                spec("add-deflated-entry", vec![("name", Json::String("a.txt".into())), ("content", Json::String("p".into()))]),
                spec("remove-entry", vec![("name", Json::String("notiz.txt".into()))]),
                spec("rename-entry", vec![("name", Json::String("notiz.txt".into())), ("newName", Json::String("notiz2.txt".into()))]),
                spec("set-entry-data", vec![("name", Json::String("notiz.txt".into())), ("content", Json::String("anders".into()))]),
            ];
            for one in specs {
                let base = archive();
                let mutated = apply(base.clone(), &one).expect("forward applies");
                let restored = invert(&base, mutated, &one).expect("inverse applies");
                assert_eq!(projection(&restored).to_string(), projection(&base).to_string(), "kind {} is not invertible", one.str("kind"));
            }
        }
    }
}
//#endregion 🔖️Live

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    live::write_archive(&live::apply(live::read_archive(input)?, spec)?)
}

/// ↩️ Applies `spec`'s kind to `original`, then undoes it with the mutation's own inverse computed
/// against `mutated`. Restored payloads are sourced straight from the decoded `original`, never
/// round-tripped through JSON.
#[cfg(feature = "oracles")]
pub fn oracle_apply_inverse(original: &[u8], mutated: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let original_archive = live::read_archive(original)?;
    let mutated_archive = live::read_archive(mutated)?;
    live::write_archive(&live::invert(&original_archive, mutated_archive, spec)?)
}

/// 🔁️ Decodes with the independent reader and re-encodes with the reference writer, no mutation
/// applied — the identity round trip.
#[cfg(feature = "oracles")]
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    live::write_archive(&live::read_archive(input)?)
}

/// 👁️ Projects ZIP bytes with the INDEPENDENT reader onto the `semantic-zip-iso21320-v1` shape.
#[cfg(feature = "oracles")]
pub fn project_zip_iso21320(input: &[u8]) -> Result<Json, String> {
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
pub fn project_zip_iso21320(_input: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch
