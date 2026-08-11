//! 🔺️ Ifc2x3Diff — real id-keyed instance diff over `Ifc2x3Snapshot.document.instances`
//! (`Part21Instance` is already keyed by a stable `u64` id, so unlike an index-keyed collection
//! this diff needs no position-transport algebra: `removed_instances`/`upserted_instances` are a
//! plain id-keyed set/map merge). Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES:
//! `4`'s `IfcDiff` is a `snapshot: Option<IfcSnapshot>` full-replace stub with no
//! `impl DiffAlgebra`; this standard's own diff is genuinely field-sparse instead.

use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
use crate::artifacts::step::engine::part21::{Part21Header, Part21Instance};
use protocol::MutationDiff;
// 🧭️ `DiffAlgebra` isn't yet on the `protocol` facade's curated re-export list (S1 added the
// trait but the facade wasn't updated) — reached via the still-public `os_spr::command` path
// instead, same as `txt`'s own `🔺️diff/🦀️component.rs`.
use protocol::os_spr::command::DiffAlgebra;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;
use std::collections::HashSet;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.ifc.2x3`. `header` is a whole-record replace (it's a 3-field header, not
/// worth a sub-algebra); `removed_instances`/`upserted_instances` are the id-keyed instance delta.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ifc.2x3.diff")]
pub struct Ifc2x3Diff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<Part21Header>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed_instances: Vec<u64>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub upserted_instances: Vec<Part21Instance>,
}

impl MutationDiff<Ifc2x3Snapshot> for Ifc2x3Diff {
    fn apply(&self, base: &Ifc2x3Snapshot) -> Ifc2x3Snapshot {
        let mut document = base.document.clone();
        if let Some(header) = &self.header {
            document.header = header.clone();
        }
        let removed: HashSet<u64> = self.removed_instances.iter().copied().collect();
        let upserted_ids: HashSet<u64> = self.upserted_instances.iter().map(|i| i.id).collect();
        document.instances.retain(|i| !removed.contains(&i.id) && !upserted_ids.contains(&i.id));
        document.instances.extend(self.upserted_instances.iter().cloned());
        Ifc2x3Snapshot { schema: self.schema.clone().unwrap_or_else(|| base.schema.clone()), document }
    }

    /// ➕️ Structural, base-free (id-keyed collections need no position transport, unlike an
    /// index-keyed one): `other`'s removal of an id cancels any pending upsert of that id in
    /// `self` (and vice versa — a later upsert of a formerly-removed id un-removes it).
    fn absorb(&mut self, other: Self) {
        if other.schema.is_some() {
            self.schema = other.schema;
        }
        if other.header.is_some() {
            self.header = other.header;
        }
        for id in other.removed_instances {
            self.upserted_instances.retain(|i| i.id != id);
            if !self.removed_instances.contains(&id) {
                self.removed_instances.push(id);
            }
        }
        for inst in other.upserted_instances {
            self.removed_instances.retain(|id| *id != inst.id);
            if let Some(slot) = self.upserted_instances.iter_mut().find(|i| i.id == inst.id) {
                *slot = inst;
            } else {
                self.upserted_instances.push(inst);
            }
        }
    }
}

impl DiffAlgebra<Ifc2x3Snapshot> for Ifc2x3Diff {
    /// 🔁️ Same `apply`+`between` composition proof `txt::TxtDiff::inverse` uses: `next =
    /// self.apply(base)`, so `between(next, base)` is by definition the diff that restores `base`.
    fn inverse(&self, base: &Ifc2x3Snapshot) -> Self {
        let next = self.apply(base);
        Self::between(&next, base)
    }

    fn between(base: &Ifc2x3Snapshot, other: &Ifc2x3Snapshot) -> Self {
        let schema = if base.schema != other.schema { Some(other.schema.clone()) } else { None };
        let header = if base.document.header != other.document.header { Some(other.document.header.clone()) } else { None };
        let base_by_id: std::collections::HashMap<u64, &Part21Instance> = base.document.instances.iter().map(|i| (i.id, i)).collect();
        let other_by_id: std::collections::HashMap<u64, &Part21Instance> = other.document.instances.iter().map(|i| (i.id, i)).collect();
        let removed_instances: Vec<u64> = base_by_id.keys().filter(|id| !other_by_id.contains_key(id)).copied().collect();
        let mut upserted_instances: Vec<Part21Instance> = other
            .document
            .instances
            .iter()
            .filter(|i| base_by_id.get(&i.id).map(|b| *b != *i).unwrap_or(true))
            .cloned()
            .collect();
        upserted_instances.sort_by_key(|i| i.id);
        Ifc2x3Diff { schema, header, removed_instances, upserted_instances }
    }

    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.header.is_none() && self.removed_instances.is_empty() && self.upserted_instances.is_empty()
    }
}

/// 🧩 Builds the sparse field-by-field diff for a `SetSnapshot` mutation.
pub fn diff_set_snapshot(base: &Ifc2x3Snapshot, snapshot: &Ifc2x3Snapshot) -> Ifc2x3Diff {
    Ifc2x3Diff::between(base, snapshot)
}
pub fn diff_upsert_instance(instance: &Part21Instance) -> Ifc2x3Diff {
    Ifc2x3Diff { upserted_instances: vec![instance.clone()], ..Default::default() }
}
pub fn diff_remove_instance(id: u64) -> Ifc2x3Diff {
    Ifc2x3Diff { removed_instances: vec![id], ..Default::default() }
}
pub fn diff_set_header(header: &Part21Header) -> Ifc2x3Diff {
    Ifc2x3Diff { header: Some(header.clone()), ..Default::default() }
}
//#endregion 🔖️Diff

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::step::engine::part21::Part21Value;

    fn inst(id: u64, name: &str) -> Part21Instance {
        Part21Instance { id, entities: vec![(name.to_string(), vec![Part21Value::Int(id as i64)])] }
    }

    fn snap(schema: &str, header: Part21Header, instances: Vec<Part21Instance>) -> Ifc2x3Snapshot {
        Ifc2x3Snapshot {
            schema: schema.into(),
            document: crate::artifacts::step::engine::part21::Part21Document { header, instances },
        }
    }

    /// 🧪️ THE acceptance criterion for "diff can change every field": schema, header, and
    /// instance add/remove/modify all round-trip through `between`+`apply`.
    #[test]
    fn field_sweep_between_covers_every_field() {
        let base = snap("stdio.ifc.2x3", Part21Header::default(), vec![inst(1, "IFCWALL"), inst(2, "IFCDOOR")]);
        let mut next_header = Part21Header::default();
        next_header.file_schema = vec![Part21Value::Str("IFC2X3".into())];
        let next = snap(
            "stdio.ifc.2x3.v2",
            next_header,
            vec![inst(1, "IFCWALLSTANDARDCASE"), inst(3, "IFCWINDOW")], // 1 modified, 2 removed, 3 added
        );
        let d = Ifc2x3Diff::between(&base, &next);
        assert!(d.schema.is_some());
        assert!(d.header.is_some());
        assert_eq!(d.removed_instances, vec![2]);
        assert_eq!(d.upserted_instances.len(), 2);
        assert_eq!(d.apply(&base), next);
    }

    #[test]
    fn absorb_upsert_then_remove_same_id_cancels_to_removed_only() {
        let mut d1 = Ifc2x3Diff { upserted_instances: vec![inst(5, "IFCSLAB")], ..Default::default() };
        let d2 = Ifc2x3Diff { removed_instances: vec![5], ..Default::default() };
        d1.absorb(d2);
        assert!(d1.upserted_instances.is_empty());
        assert_eq!(d1.removed_instances, vec![5]);
    }

    #[test]
    fn absorb_remove_then_upsert_same_id_un_removes() {
        let mut d1 = Ifc2x3Diff { removed_instances: vec![7], ..Default::default() };
        let d2 = Ifc2x3Diff { upserted_instances: vec![inst(7, "IFCBEAM")], ..Default::default() };
        d1.absorb(d2);
        assert!(d1.removed_instances.is_empty());
        assert_eq!(d1.upserted_instances, vec![inst(7, "IFCBEAM")]);
    }

    #[test]
    fn absorb_matches_sequential_apply() {
        let base = snap("stdio.ifc.2x3", Part21Header::default(), vec![inst(1, "IFCWALL")]);
        let d1 = Ifc2x3Diff { upserted_instances: vec![inst(2, "IFCDOOR")], ..Default::default() };
        let d2 = Ifc2x3Diff { removed_instances: vec![1], upserted_instances: vec![inst(3, "IFCWINDOW")], ..Default::default() };
        let mut merged = d1.clone();
        merged.absorb(d2.clone());
        let sequential = { let mid = d1.apply(&base); d2.apply(&mid) };
        assert_eq!(merged.apply(&base), sequential);
    }

    #[test]
    fn inverse_diff_level_roundtrip() {
        let base = snap("stdio.ifc.2x3", Part21Header::default(), vec![inst(1, "IFCWALL"), inst(2, "IFCDOOR")]);
        let d = Ifc2x3Diff { removed_instances: vec![2], upserted_instances: vec![inst(1, "IFCWALLSTANDARDCASE"), inst(4, "IFCCOLUMN")], ..Default::default() };
        let next = d.apply(&base);
        let inv = d.inverse(&base);
        assert_eq!(inv.apply(&next), base);
    }

    #[test]
    fn between_self_is_empty() {
        let base = snap("stdio.ifc.2x3", Part21Header::default(), vec![inst(1, "IFCWALL")]);
        assert!(Ifc2x3Diff::between(&base, &base).is_empty());
    }
}
//#endregion 🧪️Tests
