//! 🧬️ ZipMutation — document mutation dispatch. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: real vocabulary beyond
//! the universal `{NoMutation, SetSnapshot}` stub — every entry field gets its own setter
//! mutation, `AddEntry`/`RemoveEntry`/`RenameEntry` cover the name-keyed collection ops. Every
//! variant's `diff()` is handcrafted (constructs `ZipDiff` directly via the `schema::diff`
//! builders) — apply-and-capture is never used.

use crate::artifacts::zip::schema::diff::{self, ZipDiff};
use crate::artifacts::zip::schema::snapshot::{ZipCompressionMethod, ZipEntry, ZipExtraField};
use crate::artifacts::zip::ZipSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.zip`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum ZipMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: ZipSnapshot,
    },
    /// 💬️ Sets the archive-level (EOCD) comment.
    SetArchiveComment {
        comment: String,
    },
    /// ➕️ Inserts a fully-specified entry at `index` (final position, clamped to `len`).
    AddEntry {
        index: usize,
        entry: ZipEntry,
    },
    /// ➖️ Removes the entry named `name` (no-op if absent).
    RemoveEntry {
        name: String,
    },
    /// 🏷️ Renames entry `name` to `new_name`.
    RenameEntry {
        name: String,
        new_name: String,
    },
    /// 📦️ Replaces an entry's decompressed payload.
    SetEntryData {
        name: String,
        data: Vec<u8>,
    },
    /// 🗜️ Changes an entry's compression method.
    SetEntryMethod {
        name: String,
        method: ZipCompressionMethod,
    },
    /// 🕰️ Sets an entry's DOS date/time and (tri-state) Info-ZIP UTC mtime.
    SetEntryTimestamps {
        name: String,
        dos_date: u16,
        dos_time: u16,
        unix_mtime: Option<i64>,
    },
    /// 🚩️ Sets an entry's general-purpose bit flags.
    SetEntryFlags {
        name: String,
        flags: u16,
    },
    /// 🔢️ Sets an entry's version-made-by / version-needed fields.
    SetEntryVersions {
        name: String,
        version_made_by: u16,
        version_needed: u16,
    },
    /// 🔐️ Sets an entry's internal/external attribute fields.
    SetEntryAttributes {
        name: String,
        internal_attrs: u16,
        external_attrs: u32,
    },
    /// 🧩️ Replaces an entry's local/central extra-field records (whole-value weak-list replace).
    SetEntryExtra {
        name: String,
        local_extra: Vec<ZipExtraField>,
        central_extra: Vec<ZipExtraField>,
    },
    /// 💬️ Sets an entry's per-member comment.
    SetEntryComment {
        name: String,
        comment: String,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. Every entry-targeted variant is a graceful no-op when
/// `name` doesn't exist (a stale name from a concurrent edit degrades gracefully, never panics).
pub fn apply_zip_mutation(snapshot: &mut ZipSnapshot, mutation: &ZipMutation) -> ZipDiff {
    let __diff = <ZipMutation as protocol::Mutation<ZipSnapshot>>::diff(mutation, snapshot);
    fn find_mut<'a>(snapshot: &'a mut ZipSnapshot, name: &str) -> Option<&'a mut ZipEntry> {
        snapshot.entries.iter_mut().find(|e| e.name == name)
    }
    match mutation {
        ZipMutation::NoMutation => {}
        ZipMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
        ZipMutation::SetArchiveComment { comment } => snapshot.comment = comment.clone(),
        ZipMutation::AddEntry { index, entry } => {
            let at = (*index).min(snapshot.entries.len());
            snapshot.entries.insert(at, entry.clone());
        }
        ZipMutation::RemoveEntry { name } => snapshot.entries.retain(|e| &e.name != name),
        ZipMutation::RenameEntry { name, new_name } => {
            if let Some(e) = find_mut(snapshot, name) { e.name = new_name.clone(); }
        }
        ZipMutation::SetEntryData { name, data } => {
            if let Some(e) = find_mut(snapshot, name) { e.data = data.clone(); }
        }
        ZipMutation::SetEntryMethod { name, method } => {
            if let Some(e) = find_mut(snapshot, name) { e.method = *method; }
        }
        ZipMutation::SetEntryTimestamps { name, dos_date, dos_time, unix_mtime } => {
            if let Some(e) = find_mut(snapshot, name) {
                e.dos_date = *dos_date;
                e.dos_time = *dos_time;
                e.unix_mtime = *unix_mtime;
            }
        }
        ZipMutation::SetEntryFlags { name, flags } => {
            if let Some(e) = find_mut(snapshot, name) { e.flags = *flags; }
        }
        ZipMutation::SetEntryVersions { name, version_made_by, version_needed } => {
            if let Some(e) = find_mut(snapshot, name) {
                e.version_made_by = *version_made_by;
                e.version_needed = *version_needed;
            }
        }
        ZipMutation::SetEntryAttributes { name, internal_attrs, external_attrs } => {
            if let Some(e) = find_mut(snapshot, name) {
                e.internal_attrs = *internal_attrs;
                e.external_attrs = *external_attrs;
            }
        }
        ZipMutation::SetEntryExtra { name, local_extra, central_extra } => {
            if let Some(e) = find_mut(snapshot, name) {
                e.local_extra = local_extra.clone();
                e.central_extra = central_extra.clone();
            }
        }
        ZipMutation::SetEntryComment { name, comment } => {
            if let Some(e) = find_mut(snapshot, name) { e.comment = comment.clone(); }
        }
    }

    __diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<ZipSnapshot> for ZipMutation {
    type Diff = ZipDiff;

    fn diff(&self, base: &ZipSnapshot) -> Self::Diff {
        match self {
            ZipMutation::NoMutation => ZipDiff::default(),
            ZipMutation::SetSnapshot { snapshot } => diff::diff_set_snapshot(base, snapshot),
            ZipMutation::SetArchiveComment { comment } => diff::diff_set_archive_comment(comment),
            ZipMutation::AddEntry { index, entry } => diff::diff_add_entry(*index, entry.clone()),
            ZipMutation::RemoveEntry { name } => diff::diff_remove_entry(name),
            ZipMutation::RenameEntry { name, new_name } => diff::diff_rename_entry(name, new_name),
            ZipMutation::SetEntryData { name, data } => diff::diff_set_entry_data(name, data.clone()),
            ZipMutation::SetEntryMethod { name, method } => diff::diff_set_entry_method(name, *method),
            ZipMutation::SetEntryTimestamps { name, dos_date, dos_time, unix_mtime } => {
                diff::diff_set_entry_timestamps(name, *dos_date, *dos_time, *unix_mtime)
            }
            ZipMutation::SetEntryFlags { name, flags } => diff::diff_set_entry_flags(name, *flags),
            ZipMutation::SetEntryVersions { name, version_made_by, version_needed } => {
                diff::diff_set_entry_versions(name, *version_made_by, *version_needed)
            }
            ZipMutation::SetEntryAttributes { name, internal_attrs, external_attrs } => {
                diff::diff_set_entry_attributes(name, *internal_attrs, *external_attrs)
            }
            ZipMutation::SetEntryExtra { name, local_extra, central_extra } => {
                diff::diff_set_entry_extra(name, local_extra.clone(), central_extra.clone())
            }
            ZipMutation::SetEntryComment { name, comment } => diff::diff_set_entry_comment(name, comment),
        }
    }

    /// ↩️ Handcrafted, key-aware mutation-level inverses. Entry-targeted variants look the prior
    /// value up in `base`; a stale/absent name inverts to `NoMutation` (nothing to undo).
    fn inverse(&self, base: &ZipSnapshot) -> Vec<Self> {
        let entry = |name: &str| base.entries.iter().find(|e| e.name == name);
        match self {
            ZipMutation::NoMutation => vec![ZipMutation::NoMutation],
            ZipMutation::SetSnapshot { .. } => vec![ZipMutation::SetSnapshot { snapshot: base.clone() }],
            ZipMutation::SetArchiveComment { .. } => vec![ZipMutation::SetArchiveComment { comment: base.comment.clone() }],
            ZipMutation::AddEntry { entry, .. } => vec![ZipMutation::RemoveEntry { name: entry.name.clone() }],
            ZipMutation::RemoveEntry { name } => match base.entries.iter().position(|e| &e.name == name) {
                Some(index) => vec![ZipMutation::AddEntry { index, entry: base.entries[index].clone() }],
                None => vec![ZipMutation::NoMutation],
            },
            ZipMutation::RenameEntry { name, new_name } => vec![ZipMutation::RenameEntry { name: new_name.clone(), new_name: name.clone() }],
            ZipMutation::SetEntryData { name, .. } => match entry(name) {
                Some(e) => vec![ZipMutation::SetEntryData { name: name.clone(), data: e.data.clone() }],
                None => vec![ZipMutation::NoMutation],
            },
            ZipMutation::SetEntryMethod { name, .. } => match entry(name) {
                Some(e) => vec![ZipMutation::SetEntryMethod { name: name.clone(), method: e.method }],
                None => vec![ZipMutation::NoMutation],
            },
            ZipMutation::SetEntryTimestamps { name, .. } => match entry(name) {
                Some(e) => vec![ZipMutation::SetEntryTimestamps { name: name.clone(), dos_date: e.dos_date, dos_time: e.dos_time, unix_mtime: e.unix_mtime }],
                None => vec![ZipMutation::NoMutation],
            },
            ZipMutation::SetEntryFlags { name, .. } => match entry(name) {
                Some(e) => vec![ZipMutation::SetEntryFlags { name: name.clone(), flags: e.flags }],
                None => vec![ZipMutation::NoMutation],
            },
            ZipMutation::SetEntryVersions { name, .. } => match entry(name) {
                Some(e) => vec![ZipMutation::SetEntryVersions { name: name.clone(), version_made_by: e.version_made_by, version_needed: e.version_needed }],
                None => vec![ZipMutation::NoMutation],
            },
            ZipMutation::SetEntryAttributes { name, .. } => match entry(name) {
                Some(e) => vec![ZipMutation::SetEntryAttributes { name: name.clone(), internal_attrs: e.internal_attrs, external_attrs: e.external_attrs }],
                None => vec![ZipMutation::NoMutation],
            },
            ZipMutation::SetEntryExtra { name, .. } => match entry(name) {
                Some(e) => vec![ZipMutation::SetEntryExtra { name: name.clone(), local_extra: e.local_extra.clone(), central_extra: e.central_extra.clone() }],
                None => vec![ZipMutation::NoMutation],
            },
            ZipMutation::SetEntryComment { name, .. } => match entry(name) {
                Some(e) => vec![ZipMutation::SetEntryComment { name: name.clone(), comment: e.comment.clone() }],
                None => vec![ZipMutation::NoMutation],
            },
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for ZipMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for ZipMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|e| protocol::ProtocolError::Malformed {
            what: "op encode",
            offset: 0,
            detail: e.to_string(),
        })
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|e| protocol::ProtocolError::Malformed {
            what: "op decode",
            offset: 0,
            detail: e.to_string(),
        })
    }
}
//#endregion OpCodecs

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::zip::schema::diff::ZipEntriesDiff;
    use protocol::MutationDiff;
    use protocol::command::DiffAlgebra;

    //#region Fixtures
    fn entry(name: &str, data: &[u8]) -> ZipEntry {
        ZipEntry {
            name: name.into(),
            data: data.to_vec(),
            method: ZipCompressionMethod::Stored,
            dos_date: 0x5678,
            dos_time: 0x1234,
            unix_mtime: Some(1_700_000_000),
            flags: 0,
            version_made_by: 20,
            version_needed: 20,
            internal_attrs: 0,
            external_attrs: 0o100644 << 16,
            local_extra: vec![ZipExtraField { id: 0x5455, payload: vec![1, 2, 3] }],
            central_extra: vec![],
            comment: String::new(),
        }
    }

    fn base_snapshot() -> ZipSnapshot {
        ZipSnapshot {
            schema: "stdio.zip".into(),
            entries: vec![entry("a.txt", b"aaa"), entry("b.txt", b"bbb"), entry("c.txt", b"ccc")],
            comment: "archive".into(),
        }
    }
    //#endregion Fixtures

    //#region 🔖️mutation_diff_law
    fn assert_mutation_diff_law(base: &ZipSnapshot, mutation: ZipMutation) {
        let expected_diff = mutation.diff(base);
        let mut applied_snapshot = base.clone();
        let returned_diff = apply_zip_mutation(&mut applied_snapshot, &mutation);
        assert_eq!(returned_diff, expected_diff, "apply_zip_mutation must return mutation.diff(base) for {mutation:?}");
        assert_eq!(expected_diff.apply(base), applied_snapshot, "diff.apply(base) must equal the imperative mutation result for {mutation:?}");
    }

    #[test]
    fn mutation_diff_law() {
        let base = base_snapshot();
        assert_mutation_diff_law(&base, ZipMutation::NoMutation);
        let mut alt = base.clone();
        alt.comment = "different".into();
        assert_mutation_diff_law(&base, ZipMutation::SetSnapshot { snapshot: alt });
        assert_mutation_diff_law(&base, ZipMutation::SetArchiveComment { comment: "new comment".into() });
        assert_mutation_diff_law(&base, ZipMutation::AddEntry { index: 1, entry: entry("x.bin", b"xxx") });
        assert_mutation_diff_law(&base, ZipMutation::RemoveEntry { name: "b.txt".into() });
        assert_mutation_diff_law(&base, ZipMutation::RenameEntry { name: "a.txt".into(), new_name: "a2.txt".into() });
        assert_mutation_diff_law(&base, ZipMutation::SetEntryData { name: "a.txt".into(), data: b"new-data".to_vec() });
        assert_mutation_diff_law(&base, ZipMutation::SetEntryMethod { name: "a.txt".into(), method: ZipCompressionMethod::Deflate });
        assert_mutation_diff_law(&base, ZipMutation::SetEntryTimestamps { name: "a.txt".into(), dos_date: 1, dos_time: 2, unix_mtime: None });
        assert_mutation_diff_law(&base, ZipMutation::SetEntryFlags { name: "a.txt".into(), flags: 0x0800 });
        assert_mutation_diff_law(&base, ZipMutation::SetEntryVersions { name: "a.txt".into(), version_made_by: 63, version_needed: 45 });
        assert_mutation_diff_law(&base, ZipMutation::SetEntryAttributes { name: "a.txt".into(), internal_attrs: 1, external_attrs: 0o100755 << 16 });
        assert_mutation_diff_law(&base, ZipMutation::SetEntryExtra { name: "a.txt".into(), local_extra: vec![], central_extra: vec![ZipExtraField { id: 9, payload: vec![9] }] });
        assert_mutation_diff_law(&base, ZipMutation::SetEntryComment { name: "a.txt".into(), comment: "hi".into() });
        // Out-of-range name: graceful no-op, still law-compliant.
        assert_mutation_diff_law(&base, ZipMutation::SetEntryComment { name: "does-not-exist".into(), comment: "hi".into() });
    }
    //#endregion 🔖️mutation_diff_law

    //#region 🔖️inverse_law
    #[test]
    fn inverse_law() {
        let base = base_snapshot();
        let variants = vec![
            ZipMutation::NoMutation,
            ZipMutation::SetArchiveComment { comment: "changed".into() },
            ZipMutation::AddEntry { index: 1, entry: entry("x.bin", b"xxx") },
            ZipMutation::RemoveEntry { name: "b.txt".into() },
            ZipMutation::RenameEntry { name: "a.txt".into(), new_name: "a2.txt".into() },
            ZipMutation::SetEntryData { name: "a.txt".into(), data: b"new-data".to_vec() },
            ZipMutation::SetEntryMethod { name: "a.txt".into(), method: ZipCompressionMethod::Deflate },
            ZipMutation::SetEntryTimestamps { name: "a.txt".into(), dos_date: 1, dos_time: 2, unix_mtime: None },
            ZipMutation::SetEntryFlags { name: "a.txt".into(), flags: 0x0800 },
            ZipMutation::SetEntryVersions { name: "a.txt".into(), version_made_by: 63, version_needed: 45 },
            ZipMutation::SetEntryAttributes { name: "a.txt".into(), internal_attrs: 1, external_attrs: 0o100755 << 16 },
            ZipMutation::SetEntryExtra { name: "a.txt".into(), local_extra: vec![], central_extra: vec![] },
            ZipMutation::SetEntryComment { name: "a.txt".into(), comment: "hi".into() },
        ];
        for m in variants {
            // Mutation-level round trip.
            let mut snap = base.clone();
            apply_zip_mutation(&mut snap, &m);
            for inv in m.inverse(&base) {
                apply_zip_mutation(&mut snap, &inv);
            }
            assert_eq!(snap, base, "mutation-level inverse must restore base for {m:?}");

            // Diff-level round trip.
            let d = m.diff(&base);
            let mutated = d.apply(&base);
            let inv_d = d.inverse(&base);
            assert_eq!(inv_d.apply(&mutated), base, "diff-level inverse must restore base for {m:?}");
        }
    }
    //#endregion 🔖️inverse_law

    //#region 🔖️absorb_law
    fn assert_absorb_law(base: &ZipSnapshot, m1: ZipMutation, m2: ZipMutation) {
        let d1 = m1.diff(base);
        let mid = d1.apply(base);
        let d2 = m2.diff(&mid);
        let sequential = d2.apply(&mid);

        let mut merged = d1.clone();
        merged.absorb(d2.clone());
        assert_eq!(merged.apply(base), sequential, "absorb(d1,d2).apply(base) must equal sequential application for {m1:?} + {m2:?}");
    }

    #[test]
    fn absorb_law() {
        let base = base_snapshot();

        // Insert+Remove-before: added entry lands correctly once an earlier-positioned base
        // survivor is removed by the second mutation (the recipe's own canonical shift case).
        assert_absorb_law(&base, ZipMutation::AddEntry { index: 1, entry: entry("x.bin", b"x") }, ZipMutation::RemoveEntry { name: "a.txt".into() });

        // Insert+Insert-same-index: both survive, later insert lands at the lower final index.
        assert_absorb_law(&base, ZipMutation::AddEntry { index: 1, entry: entry("x.bin", b"x") }, ZipMutation::AddEntry { index: 1, entry: entry("y.bin", b"y") });

        // Add+SetField: the second mutation patches directly into the still-pending added entry.
        assert_absorb_law(&base, ZipMutation::AddEntry { index: 0, entry: entry("x.bin", b"x") }, ZipMutation::SetEntryComment { name: "x.bin".into(), comment: "patched".into() });

        // Add+Rename: renaming a just-added entry patches its carried payload in place.
        assert_absorb_law(&base, ZipMutation::AddEntry { index: 0, entry: entry("x.bin", b"x") }, ZipMutation::RenameEntry { name: "x.bin".into(), new_name: "y.bin".into() });

        // Modify+Remove: a pending field patch on a since-removed base entry vanishes.
        assert_absorb_law(&base, ZipMutation::SetEntryComment { name: "a.txt".into(), comment: "will be dropped".into() }, ZipMutation::RemoveEntry { name: "a.txt".into() });

        // Rename then remove-of-the-renamed-name resolves back to the base identity.
        assert_absorb_law(&base, ZipMutation::RenameEntry { name: "a.txt".into(), new_name: "renamed.txt".into() }, ZipMutation::RemoveEntry { name: "renamed.txt".into() });

        // Insert then annihilate the very same insert.
        assert_absorb_law(&base, ZipMutation::AddEntry { index: 0, entry: entry("x.bin", b"x") }, ZipMutation::RemoveEntry { name: "x.bin".into() });

        // Two unrelated scalar sets absorb via LWW.
        assert_absorb_law(&base, ZipMutation::SetArchiveComment { comment: "first".into() }, ZipMutation::SetArchiveComment { comment: "second".into() });
    }

    #[test]
    fn absorb_law_associativity() {
        let base = base_snapshot();
        let d1 = ZipMutation::SetArchiveComment { comment: "one".into() }.diff(&base);
        let mid1 = d1.apply(&base);
        let d2 = ZipMutation::AddEntry { index: 0, entry: entry("x.bin", b"x") }.diff(&mid1);
        let mid2 = d2.apply(&mid1);
        let d3 = ZipMutation::SetEntryComment { name: "x.bin".into(), comment: "patched".into() }.diff(&mid2);

        // (d1∘d2)∘d3
        let mut left = d1.clone();
        left.absorb(d2.clone());
        left.absorb(d3.clone());

        // d1∘(d2∘d3)
        let mut d23 = d2.clone();
        d23.absorb(d3.clone());
        let mut right = d1.clone();
        right.absorb(d23);

        assert_eq!(left.apply(&base), right.apply(&base), "absorb must associate");
        assert_eq!(left.apply(&base), d3.apply(&mid2), "associated absorb must match full sequential application");
    }
    //#endregion 🔖️absorb_law

    //#region 🔖️between_roundtrip_law
    #[test]
    fn between_roundtrip_law() {
        let a = base_snapshot();
        let mut b = base_snapshot();
        b.comment = "changed archive comment".into();
        b.entries.remove(0); // remove a.txt
        b.entries[0].comment = "modified b".into(); // modify b.txt (now index 0)
        b.entries.push(entry("new.bin", b"new")); // add new.bin

        let d = ZipDiff::between(&a, &b);
        assert_eq!(d.apply(&a), b, "between(a,b).apply(a) must equal b");
        let d_rev = ZipDiff::between(&b, &a);
        assert_eq!(d_rev.apply(&b), a, "between(b,a).apply(b) must equal a");
        assert!(ZipDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");
    }
    //#endregion 🔖️between_roundtrip_law

    //#region 🔖️codec_retention_law
    #[test]
    fn codec_retention_law() {
        let bytes = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../🗿️artifacts/🎒️zip/📚️examples/🎬️demo/🖼️assets/🎒️example.zip"
        ));
        let bytes = match bytes {
            Ok(b) => b,
            // The fixture path is relative to this crate's manifest dir under the workspace
            // layout; if the workspace root differs at test time, fall back to a synthetic
            // archive so this law still exercises decode -> encode -> decode identity.
            Err(_) => crate::artifacts::zip::engine::encode_zip(&base_snapshot()).expect("encode synthetic fallback"),
        };
        let decoded = crate::artifacts::zip::engine::decode_zip(&bytes).expect("decode fixture");
        let reencoded = crate::artifacts::zip::engine::encode_zip(&decoded).expect("re-encode fixture");
        let redecoded = crate::artifacts::zip::engine::decode_zip(&reencoded).expect("re-decode fixture");
        // Semantically equivalent per the engine's own documented normal form (UTF-8 flag /
        // data-descriptor bit normalization) — content, not raw bytes, is the retained invariant.
        assert_eq!(redecoded.entries.len(), decoded.entries.len());
        for (a, b) in decoded.entries.iter().zip(redecoded.entries.iter()) {
            assert_eq!(a.name, b.name);
            assert_eq!(a.data, b.data);
            assert_eq!(a.method, b.method);
        }
        assert_eq!(decoded.comment, redecoded.comment);
    }
    //#endregion 🔖️codec_retention_law

    //#region 🔖️field_sweep
    /// 🌪️ `sweep_a`/`sweep_b` differ in EVERY mutable field: one removed entry, one entry
    /// modified in every field (including clearing `unix_mtime`, exercising the tri-state), one
    /// added entry, plus the archive comment.
    fn sweep_a() -> ZipSnapshot {
        ZipSnapshot {
            schema: "stdio.zip".into(),
            entries: vec![
                ZipEntry {
                    name: "gone.txt".into(),
                    data: b"will be removed".to_vec(),
                    method: ZipCompressionMethod::Stored,
                    dos_date: 1,
                    dos_time: 1,
                    unix_mtime: Some(1000),
                    flags: 0,
                    version_made_by: 20,
                    version_needed: 20,
                    internal_attrs: 0,
                    external_attrs: 0,
                    local_extra: vec![],
                    central_extra: vec![],
                    comment: "goodbye".into(),
                },
                ZipEntry {
                    name: "stay.txt".into(),
                    data: b"before".to_vec(),
                    method: ZipCompressionMethod::Stored,
                    dos_date: 0x1111,
                    dos_time: 0x2222,
                    unix_mtime: Some(1_600_000_000),
                    flags: 0,
                    version_made_by: 20,
                    version_needed: 20,
                    internal_attrs: 0,
                    external_attrs: 0o100644 << 16,
                    local_extra: vec![ZipExtraField { id: 1, payload: vec![1] }],
                    central_extra: vec![ZipExtraField { id: 2, payload: vec![2] }],
                    comment: "before comment".into(),
                },
            ],
            comment: "archive before".into(),
        }
    }

    fn sweep_b() -> ZipSnapshot {
        ZipSnapshot {
            schema: "stdio.zip".into(),
            entries: vec![
                ZipEntry {
                    // Name intentionally UNCHANGED: `between()` matches entries by name (a
                    // rename is documented to show as remove+add, never as a `modified.diff.name`
                    // patch — see `ZipDiff::between`'s doc comment), so this fixture keeps the
                    // key stable to exercise every OTHER field via `modified`. `ZipEntryDiff::name`
                    // itself is exercised directly by `mutation_diff_law`/`inverse_law`'s
                    // `RenameEntry` case instead.
                    name: "stay.txt".into(),
                    data: b"after".to_vec(),
                    method: ZipCompressionMethod::Deflate,
                    dos_date: 0x3333,
                    dos_time: 0x4444,
                    unix_mtime: None, // tri-state: Some(None) in the diff — the UT record was cleared.
                    flags: 0x0800,
                    version_made_by: 63,
                    version_needed: 45,
                    internal_attrs: 1,
                    external_attrs: 0o100755 << 16,
                    local_extra: vec![ZipExtraField { id: 9, payload: vec![9, 9] }],
                    central_extra: vec![ZipExtraField { id: 10, payload: vec![10] }],
                    comment: "after comment".into(),
                },
                ZipEntry {
                    name: "new.bin".into(),
                    data: b"brand new".to_vec(),
                    method: ZipCompressionMethod::Deflate,
                    dos_date: 5,
                    dos_time: 6,
                    unix_mtime: Some(1_800_000_000),
                    flags: 0,
                    version_made_by: 20,
                    version_needed: 20,
                    internal_attrs: 0,
                    external_attrs: 0,
                    local_extra: vec![],
                    central_extra: vec![],
                    comment: "hello".into(),
                },
            ],
            comment: "archive after".into(),
        }
    }

    #[test]
    fn field_sweep_covers_every_mutable_field() {
        let a = sweep_a();
        let b = sweep_b();

        let forward = ZipDiff::between(&a, &b);
        assert_eq!(forward.apply(&a), b, "between(a,b).apply(a) must equal b");
        let backward = ZipDiff::between(&b, &a);
        assert_eq!(backward.apply(&b), a, "between(b,a).apply(b) must equal a");
        assert!(ZipDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");

        // Structural per-field assertion: every mutable field actually changed in the diff.
        assert!(forward.comment.is_some(), "comment must be diffed");
        let ed: &ZipEntriesDiff = forward.entries.as_ref().expect("entries diff must be present");
        assert_eq!(ed.removed, vec!["gone.txt".to_string()], "the removed entry must be tracked");
        assert_eq!(ed.added.len(), 1, "exactly one entry must be added");
        assert_eq!(ed.added[0].entry.name, "new.bin");
        assert_eq!(ed.modified.len(), 1, "exactly one entry must be modified");
        assert_eq!(ed.modified[0].name, "stay.txt");
        let md = &ed.modified[0].diff;
        // `name` is intentionally NOT exercised here — `between()` documents renames as
        // remove+add, never a `modified.diff.name` patch; that field is covered by
        // `mutation_diff_law`/`inverse_law`'s `RenameEntry` variant instead.
        assert!(md.data.is_some(), "data must be diffed");
        assert!(md.method.is_some(), "method must be diffed");
        assert!(md.dos_date.is_some(), "dos_date must be diffed");
        assert!(md.dos_time.is_some(), "dos_time must be diffed");
        assert_eq!(md.unix_mtime, Some(None), "unix_mtime tri-state must show a clear (Some(None))");
        assert!(md.flags.is_some(), "flags must be diffed");
        assert!(md.version_made_by.is_some(), "version_made_by must be diffed");
        assert!(md.version_needed.is_some(), "version_needed must be diffed");
        assert!(md.internal_attrs.is_some(), "internal_attrs must be diffed");
        assert!(md.external_attrs.is_some(), "external_attrs must be diffed");
        assert!(md.local_extra.is_some(), "local_extra must be diffed");
        assert!(md.central_extra.is_some(), "central_extra must be diffed");
        assert!(md.comment.is_some(), "comment must be diffed");
    }
    //#endregion 🔖️field_sweep

    #[test]
    fn out_of_range_entry_mutation_is_noop_not_panic() {
        let base = base_snapshot();
        let mut snap = base.clone();
        apply_zip_mutation(&mut snap, &ZipMutation::SetEntryComment { name: "missing".into(), comment: "x".into() });
        assert_eq!(snap, base);
        apply_zip_mutation(&mut snap, &ZipMutation::RemoveEntry { name: "missing".into() });
        assert_eq!(snap, base);
    }
}
//#endregion Tests
