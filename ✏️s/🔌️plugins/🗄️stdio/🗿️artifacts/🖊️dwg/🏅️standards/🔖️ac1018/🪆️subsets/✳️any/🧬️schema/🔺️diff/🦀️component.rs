//! 🔺️ DwgDiff — handcrafted sparse diff for `ac1018`. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION F5: replaces the old
//! `DwgDiff{snapshot: Option<DwgSnapshot>}` full-replace template with a real per-field patch —
//! `version`/`maintenance_version`/`codepage` header scalars, raw `bytes`, and `section_names`.
//!
//! `section_names` is a WHOLE-VALUE weak-entity replace (`Option<Vec<String>>`), not a keyed
//! collection triple. Deliberate, not a shortcut: ac1018 is a deliberately frozen legacy shim
//! (Decision #5, see `DwgArtifact::to_snapshot`) whose `section_names` carries no per-section
//! byte payload or identity beyond the bare label string — there is no per-name content to
//! modify, no rename concept, and (unlike ac1024's `sections`) no meaningful position semantics
//! a user-facing mutation needs to preserve across composition. An earlier revision modeled this
//! as an add/remove name-multiset (mirroring zip's name-keyed triple) and hit a real, reproducible
//! `between_roundtrip_law` failure: reconstructing "survivors in their prior relative order, new
//! names appended at the end" does not, in general, reproduce an arbitrary target order (e.g.
//! `between(b,a)` where `a`'s first element became `b`'s last survivor) — position information a
//! multiset structurally cannot carry. Per the recipe's own weak-entity rule ("value structs …
//! whole-value replaced in diffs, never sub-diffed"), `Vec<String>` with no sub-structure is
//! exactly a weak entity; whole-value replace makes `between` exact by construction (it just
//! compares `Vec<String>` equality) and makes `absorb` trivially correct via LWW (composing two
//! "assign final value" ops sequentially IS taking the second one's value, for any op shape).

// ⚠️ NOT `crate::artifacts::dwg::DwgSnapshot` — that top-level re-export is aliased to the
// CANONICAL richer standard (ac1024, per S-6), same shim pattern as gif 89a/87a. ac1018's own
// `DwgSnapshot` (this standard's real, distinct, less-rich type) must be reached through its own
// fully-qualified standard path, mirroring gif 87a's precedent (`v87a::subsets::any::schema::...`).
use crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::snapshot::DwgSnapshot;
use protocol::MutationDiff;
use protocol::command::DiffAlgebra;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.dwg` (ac1018). `schema` is an identity field and never appears here.
///
/// 🧪️ F6: `dsl::DslDiff` derive added — emits `protocol::DiffCodec` (`print_diff`/`parse_diff`/
/// `encode_diff`/`decode_diff`) directly from this struct's `RecordSpec`, zero hand-written
/// grammar needed. Verified DERIVE-eligible per `f6-recon-report.md` §3's decision rule: every
/// field here is a single-level `Option<T>` (never tri-state `Option<Option<T>>`), and `T` is
/// always a plain scalar/`Vec` of scalars — no data-carrying enum anywhere in this struct or in
/// `DwgSnapshot` (the type `SetSnapshot`'s payload embeds). NOTE: `#[dsl(base64)]` on `bytes`
/// would be a documented no-op here — `classify_field` peels the outer `Option` unconditionally
/// before ever checking `attrs.base64`, so an `Option<Vec<u8>>` field always falls back to
/// `Shape::List(UInt)` (verbose decimal-byte-list grammar) regardless of the attribute (see the
/// recon report's "Known derive quirk" note) — omitted rather than left on as a misleading no-op.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema, dsl::DslDiff)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dwg.diff")]
pub struct DwgDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintenance_version: Option<u8>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub codepage: Option<u16>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
    /// 🗂️ Whole-value weak-entity replace — see module doc for why this is not a keyed triple.
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section_names: Option<Vec<String>>,
}

impl MutationDiff<DwgSnapshot> for DwgDiff {
    fn apply(&self, base: &DwgSnapshot) -> DwgSnapshot {
        DwgSnapshot {
            schema: base.schema.clone(),
            version: self.version.clone().unwrap_or_else(|| base.version.clone()),
            maintenance_version: self.maintenance_version.unwrap_or(base.maintenance_version),
            codepage: self.codepage.unwrap_or(base.codepage),
            bytes: self.bytes.clone().unwrap_or_else(|| base.bytes.clone()),
            section_names: self.section_names.clone().unwrap_or_else(|| base.section_names.clone()),
        }
    }

    /// ➕️ Structural, total, base-free sequential-coalesce (`## Absorb` contract). Every field
    /// here is LWW — correct not just for scalars but for `section_names` too, since a
    /// whole-value-replace diff already encodes "the complete state after this op"; taking the
    /// later diff's value IS the sequential composition, for any two ops of this shape.
    fn absorb(&mut self, other: Self) {
        if other.version.is_some() { self.version = other.version; }
        if other.maintenance_version.is_some() { self.maintenance_version = other.maintenance_version; }
        if other.codepage.is_some() { self.codepage = other.codepage; }
        if other.bytes.is_some() { self.bytes = other.bytes; }
        if other.section_names.is_some() { self.section_names = other.section_names; }
    }
}

impl DiffAlgebra<DwgSnapshot> for DwgDiff {
    /// 🔁️ Diff-level undo, derived generically (correct by construction): the state delta from
    /// `self.apply(base)` back to `base`.
    fn inverse(&self, base: &DwgSnapshot) -> Self {
        let mutated = self.apply(base);
        Self::between(&mutated, base)
    }

    /// 🧭️ State delta (compose `GetXDiff`): every field compares directly by value.
    fn between(base: &DwgSnapshot, other: &DwgSnapshot) -> Self {
        DwgDiff {
            version: (base.version != other.version).then(|| other.version.clone()),
            maintenance_version: (base.maintenance_version != other.maintenance_version).then_some(other.maintenance_version),
            codepage: (base.codepage != other.codepage).then_some(other.codepage),
            bytes: (base.bytes != other.bytes).then(|| other.bytes.clone()),
            section_names: (base.section_names != other.section_names).then(|| other.section_names.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.version.is_none()
            && self.maintenance_version.is_none()
            && self.codepage.is_none()
            && self.bytes.is_none()
            && self.section_names.is_none()
    }
}
//#endregion 🔖️Diff

//#region 🔖️MutationDiffBuilders
/// 🧩 `SetSnapshot`'s diff is the sparse field-by-field `between(base, next)` — no full-replace
/// slot exists on `DwgDiff` to short-circuit into.
pub fn diff_set_snapshot(base: &DwgSnapshot, next: &DwgSnapshot) -> DwgDiff {
    DwgDiff::between(base, next)
}

/// 🧩 Patches `bytes` at the plain-preamble offsets (`0..6` for `version` when it is exactly 6
/// bytes, `0x12` for `maintenance_version`, `0x13..0x15` LE for `codepage`), growing `bytes` with
/// zero padding first if it's too short to reach `0x15`. Keeps the scalar mirror fields and the
/// byte-level ground truth in sync, the same invariant `encode_dwg` already enforces for
/// `version`.
pub fn patch_version_info_bytes(bytes: &[u8], version: &str, maintenance_version: u8, codepage: u16) -> Vec<u8> {
    let mut out = bytes.to_vec();
    if out.len() < 0x15 {
        out.resize(0x15, 0);
    }
    if version.as_bytes().len() == 6 {
        out[0..6].copy_from_slice(version.as_bytes());
    }
    out[0x12] = maintenance_version;
    out[0x13..0x15].copy_from_slice(&codepage.to_le_bytes());
    out
}

pub fn diff_set_version_info(base: &DwgSnapshot, version: &str, maintenance_version: u8, codepage: u16) -> DwgDiff {
    let new_bytes = patch_version_info_bytes(&base.bytes, version, maintenance_version, codepage);
    DwgDiff {
        version: (base.version != version).then(|| version.to_string()),
        maintenance_version: (base.maintenance_version != maintenance_version).then_some(maintenance_version),
        codepage: (base.codepage != codepage).then_some(codepage),
        bytes: (base.bytes != new_bytes).then_some(new_bytes),
        section_names: None,
    }
}

/// ➕️ Inserts `name` at `index` (final position, clamped to `len` — matches ac1024's own
/// `InsertSection{index,section}` convention). Positional (not append-only) so
/// `RemoveSectionName`'s mutation-level inverse can restore the exact original position.
pub fn diff_insert_section_name(base: &DwgSnapshot, index: usize, name: &str) -> DwgDiff {
    let mut names = base.section_names.clone();
    let at = index.min(names.len());
    names.insert(at, name.to_string());
    DwgDiff { section_names: Some(names), ..Default::default() }
}

pub fn diff_remove_section_name(base: &DwgSnapshot, name: &str) -> DwgDiff {
    let mut names = base.section_names.clone();
    let changed = if let Some(pos) = names.iter().position(|n| n == name) {
        names.remove(pos);
        true
    } else {
        false
    };
    DwgDiff { section_names: changed.then_some(names), ..Default::default() }
}
//#endregion 🔖️MutationDiffBuilders

//#region 🔖️DemoCases
/// 🎬️ Representative `DwgDiff` cases, one per field-transition shape — reused by
/// `diff_codec_text_binary_roundtrip_law` below AND by `⚙️engine`'s
/// `conformance_laws::diff_grammar_conformance_law`/`protocol_walk_law` (mirrors
/// `BinaryDiff::demo_diff_cases`, `💾️binary/…/🔺️diff/🦀️component.rs`).
#[cfg(test)]
pub(crate) fn demo_diff_cases() -> Vec<DwgDiff> {
    vec![
        DwgDiff::default(),
        DwgDiff { version: Some("AC1018".into()), ..Default::default() },
        DwgDiff { maintenance_version: Some(9), codepage: Some(65001), ..Default::default() },
        DwgDiff { bytes: Some(vec![0xAA, 0xBB, 0x00, 0xFF]), ..Default::default() },
        DwgDiff { section_names: Some(vec!["AcDb:Header".into(), "AcDb:Classes".into()]), ..Default::default() },
        DwgDiff { section_names: Some(vec![]), ..Default::default() },
        DwgDiff {
            version: Some("AC1032".into()),
            maintenance_version: Some(2),
            codepage: Some(30),
            bytes: Some(vec![1, 2, 3, 4, 5]),
            section_names: Some(vec!["AcDb:Handles".into()]),
        },
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🧪️ F6: `DiffCodec` round-trip laws (derived via `dsl::DslDiff`) — exercises every field's
    /// `None`/`Some` transition at once, plus the default (fully-empty) diff.
    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        use protocol::DiffCodec;
        for d in demo_diff_cases() {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = DwgDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch for {d:?} (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff({d:?}) failed: {e}"));
            let decoded = DwgDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch for {d:?}");
        }
    }
}
//#endregion 🧪️Tests
