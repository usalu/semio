//! 🔺️ Ifc2x3Diff — real id-keyed instance diff over `Ifc2x3Snapshot.document.instances`
//! (`Part21Instance` is already keyed by a stable `u64` id, so unlike an index-keyed collection
//! this diff needs no position-transport algebra: `removed_instances`/`upserted_instances` are a
//! plain id-keyed set/map merge). Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES:
//! `4`'s `IfcDiff` is a `snapshot: Option<IfcSnapshot>` full-replace stub with no
//! `impl DiffAlgebra`; this standard's own diff is genuinely field-sparse instead.

use crate::standards::v2x3::subsets::base::schema::snapshot::{Ifc2x3EdmPreamble, Ifc2x3Snapshot};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use semio_s_artifact_stdio_contract::part21::{Part21Decimal, Part21Header, Part21Instance, Part21Value};
// 🧭️ `DiffAlgebra` isn't yet on the `protocol` facade's curated re-export list (S1 added the
// trait but the facade wasn't updated) — reached via the still-public `os_spr::command` path
// instead, same as `txt`'s own `🔺️diff/🦀️.rs`.
use framework_schema::ArtifactSchema;
use protocol::os_spr::command::DiffAlgebra;
use std::collections::{BTreeSet, HashSet};
use std::fmt::Write as _;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.ifc.2x3`. `header` is a whole-record replace (it's a 3-field header, not
/// worth a sub-algebra); `removed_instances`/`upserted_instances` are the id-keyed instance delta.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ifc.2x3.diff")]
pub struct Ifc2x3Diff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<Part21Header>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed_instances: Vec<u64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub upserted_instances: Vec<Part21Instance>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub edm_preamble: Option<Option<Ifc2x3EdmPreamble>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub instance_order: Option<Vec<u64>>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_ifc2x3_diff(diff: &Ifc2x3Diff, base: &Ifc2x3Snapshot) -> MutationApplyResult<()> {
    let mut base_ids = BTreeSet::new();
    for instance in &base.document.instances {
        if !base_ids.insert(instance.id) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-base-target", "base instance ids must be unique").at(["instances", &instance.id.to_string()]));
        }
    }
    let mut removed = BTreeSet::new();
    for &id in &diff.removed_instances {
        if !base_ids.contains(&id) || !removed.insert(id) {
            return Err(MutationApplyError::new("mutation.apply.invalid-remove-target", "instance removal target must exist exactly once").at(["instances", &id.to_string()]));
        }
    }
    let mut upserted = BTreeSet::new();
    for instance in &diff.upserted_instances {
        if removed.contains(&instance.id) || !upserted.insert(instance.id) {
            return Err(MutationApplyError::new("mutation.apply.invalid-upsert-target", "instance upsert target must be unique and not removed").at(["instances", &instance.id.to_string()]));
        }
    }
    let mut final_ids = base_ids;
    for id in removed {
        final_ids.remove(&id);
    }
    final_ids.extend(upserted);
    if let Some(order) = &diff.instance_order {
        let mut ordered = BTreeSet::new();
        for &id in order {
            if !final_ids.contains(&id) || !ordered.insert(id) {
                return Err(MutationApplyError::new("mutation.apply.invalid-instance-order", "instance order must contain each final id exactly once").at(["instanceOrder", &id.to_string()]));
            }
        }
        if ordered != final_ids {
            let missing = final_ids.difference(&ordered).next().copied().unwrap_or_default();
            return Err(MutationApplyError::new("mutation.apply.invalid-instance-order", "instance order must contain each final id exactly once").at(["instanceOrder", &missing.to_string()]));
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_ifc2x3_diff_unchecked(diff: &Ifc2x3Diff, base: &Ifc2x3Snapshot) -> Ifc2x3Snapshot {
    let mut document = base.document.clone();
    if let Some(header) = &diff.header {
        document.header = header.clone();
    }
    let removed: HashSet<u64> = diff.removed_instances.iter().copied().collect();
    document.instances.retain(|i| !removed.contains(&i.id));
    let mut positions = document.instances.iter().enumerate().map(|(position, instance)| (instance.id, position)).collect::<std::collections::HashMap<_, _>>();
    for instance in &diff.upserted_instances {
        if let Some(position) = positions.get(&instance.id).copied() {
            document.instances[position] = instance.clone();
        } else {
            positions.insert(instance.id, document.instances.len());
            document.instances.push(instance.clone());
        }
    }
    if let Some(order) = &diff.instance_order {
        let mut by_id = document.instances.drain(..).map(|instance| (instance.id, instance)).collect::<std::collections::HashMap<_, _>>();
        document.instances.reserve(by_id.len());
        for id in order {
            if let Some(instance) = by_id.remove(id) {
                document.instances.push(instance);
            }
        }
    }
    Ifc2x3Snapshot { schema: diff.schema.clone().unwrap_or_else(|| base.schema.clone()), document, edm_preamble: diff.edm_preamble.clone().unwrap_or_else(|| base.edm_preamble.clone()) }
}

impl MutationDiff<Ifc2x3Snapshot> for Ifc2x3Diff {
    fn apply(&self, base: &Ifc2x3Snapshot, _capability: protocol::ApplyCapability) -> MutationApplyResult<Ifc2x3Snapshot> {
        validate_ifc2x3_diff(self, base)?;
        Ok(apply_ifc2x3_diff_unchecked(self, base))
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
        let removed_ids = other.removed_instances.iter().copied().collect::<HashSet<_>>();
        self.upserted_instances.retain(|instance| !removed_ids.contains(&instance.id));
        let mut known_removed = self.removed_instances.iter().copied().collect::<HashSet<_>>();
        for id in other.removed_instances {
            if known_removed.insert(id) {
                self.removed_instances.push(id);
            }
        }
        let upserted_ids = other.upserted_instances.iter().map(|instance| instance.id).collect::<HashSet<_>>();
        self.removed_instances.retain(|id| !upserted_ids.contains(id));
        let mut positions = self.upserted_instances.iter().enumerate().map(|(position, instance)| (instance.id, position)).collect::<std::collections::HashMap<_, _>>();
        for inst in other.upserted_instances {
            if let Some(position) = positions.get(&inst.id).copied() {
                self.upserted_instances[position] = inst;
            } else {
                positions.insert(inst.id, self.upserted_instances.len());
                self.upserted_instances.push(inst);
            }
        }
        if other.edm_preamble.is_some() {
            self.edm_preamble = other.edm_preamble;
        }
        if other.instance_order.is_some() {
            self.instance_order = other.instance_order;
        }
    }
}

impl DiffAlgebra<Ifc2x3Snapshot> for Ifc2x3Diff {
    /// 🔁️ Diff-level undo: header, schema and preamble return to their base values, instances the diff created are removed, instances it
    /// removed or replaced return as they stood in `base`, and the base order is restored whenever the inverse alone would not land on it.
    fn inverse(&self, base: &Ifc2x3Snapshot) -> Self {
        let base_order: Vec<u64> = base.document.instances.iter().map(|instance| instance.id).collect();
        let created: Vec<u64> = self.upserted_instances.iter().map(|instance| instance.id).filter(|id| !base_order.contains(id)).collect();
        let mut upserted_instances: Vec<Part21Instance> = self.removed_instances.iter().chain(self.upserted_instances.iter().map(|instance| &instance.id).filter(|id| base_order.contains(id))).filter_map(|id| base.document.instance(*id).cloned()).collect();
        upserted_instances.sort_by_key(|instance| instance.id);
        upserted_instances.dedup_by_key(|instance| instance.id);
        let mut after_order: Vec<u64> = match &self.instance_order {
            Some(order) => order.clone(),
            None => {
                let mut order: Vec<u64> = base_order.iter().copied().filter(|id| !self.removed_instances.contains(id)).collect();
                order.extend(created.iter().copied());
                order
            }
        };
        after_order.retain(|id| !created.contains(id));
        after_order.extend(upserted_instances.iter().map(|instance| instance.id).filter(|id| !after_order.contains(id)));
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            header: self.header.as_ref().map(|_| base.document.header.clone()),
            removed_instances: created,
            upserted_instances,
            edm_preamble: self.edm_preamble.as_ref().map(|_| base.edm_preamble.clone()),
            instance_order: (after_order != base_order).then_some(base_order),
        }
    }

    fn between(base: &Ifc2x3Snapshot, other: &Ifc2x3Snapshot) -> Self {
        let schema = if base.schema != other.schema { Some(other.schema.clone()) } else { None };
        let header = if base.document.header != other.document.header { Some(other.document.header.clone()) } else { None };
        let base_order = base.document.instances.iter().map(|instance| instance.id).collect::<Vec<_>>();
        let other_order = other.document.instances.iter().map(|instance| instance.id).collect::<Vec<_>>();
        let instance_order = (base_order != other_order).then(|| other_order.clone());
        let edm_preamble = (base.edm_preamble != other.edm_preamble).then(|| other.edm_preamble.clone());
        if base.document.instances.is_empty() {
            let mut upserted_instances = other.document.instances.clone();
            upserted_instances.sort_by_key(|instance| instance.id);
            return Ifc2x3Diff { schema, header, removed_instances: Vec::new(), upserted_instances, edm_preamble, instance_order };
        }
        if other.document.instances.is_empty() {
            let mut removed_instances = base_order;
            removed_instances.sort_unstable();
            return Ifc2x3Diff { schema, header, removed_instances, upserted_instances: Vec::new(), edm_preamble, instance_order };
        }
        let base_by_id: std::collections::HashMap<u64, &Part21Instance> = base.document.instances.iter().map(|i| (i.id, i)).collect();
        let other_by_id: std::collections::HashMap<u64, &Part21Instance> = other.document.instances.iter().map(|i| (i.id, i)).collect();
        let mut removed_instances: Vec<u64> = base_by_id.keys().filter(|id| !other_by_id.contains_key(id)).copied().collect();
        removed_instances.sort_unstable();
        let mut upserted_instances: Vec<Part21Instance> = other.document.instances.iter().filter(|i| base_by_id.get(&i.id).is_none_or(|b| *b != *i)).cloned().collect();
        upserted_instances.sort_by_key(|i| i.id);
        Ifc2x3Diff { schema, header, removed_instances, upserted_instances, edm_preamble, instance_order }
    }

    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.header.is_none() && self.removed_instances.is_empty() && self.upserted_instances.is_empty() && self.edm_preamble.is_none() && self.instance_order.is_none()
    }
}

//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: real hand-rolled
/// `protocol::DiffCodec` for `Ifc2x3Diff` — this standard had NO `DiffCodec` impl at all before this
/// wave (confirmed by W0/F6's own census, the sole remaining `dsl-migration/diff-completeness`
/// breach across all 32 stdio standards). `Part21Value` is a genuine data-carrying enum (`Ref`/
/// `Str`/`Enum`/`Int`/`Real`/`List`/`Typed`, all with fields) reachable from `header`/
/// `upserted_instances` directly, so `#[derive(dsl::DslDiff)]` cannot be used here either (identical
/// `DslField`-unsatisfied root cause `4`'s own `IfcDiff`/`IfcValue` doc comment documents). Same
/// grammar style `4`'s own hand-rolled `IfcDiff`/`IfcValue` codec uses (bracket-depth-aware split,
/// hex for strings, single-uppercase-letter tag prefix for the data-carrying enum) — own local copy
/// per this dialect's per-file convention, `pub(crate)` so the mutations sibling can reuse rather
/// than duplicating a second time (same intra-artifact-reuse split `4`'s own files use).
//#region 🔖️TextPrimitives













//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives





//#endregion 🔖️BinaryPrimitives

//#region 🔖️Part21ValueCodecs







//#region 🔖️Part21ValueBinaryCodecs




//#endregion 🔖️Part21ValueBinaryCodecs
//#endregion 🔖️Part21ValueCodecs

//#region 🔖️HeaderInstanceCodecs














//#endregion 🔖️HeaderInstanceCodecs

//#region 🔖️TopLevel




//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️DemoCases
/// 🧪️ Representative `Ifc2x3Diff` cases — real `print_diff()`-conformance-law fodder
/// (`diff_grammar_conformance_law`) and `protocol_walk_law` fodder — the empty diff, a genuine
/// `between()` result exercising every top-level field (schema/header/removed/upserted, incl. a
/// COMPLEX instance and every `Part21Value` tag), and its reverse direction.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<Ifc2x3Diff> {
    let a = crate::standards::v2x3::engine::demo_ifc2x3_snapshot();
    let mut b = a.clone();
    b.schema = "stdio.ifc.2x3.v2".into();
    b.document.header.file_name = vec![Part21Value::Str("changed.ifc".into())];
    b.document.instances.retain(|i| i.id != 2);
    if let Some(first) = b.document.instances.first_mut() {
        first.entities = vec![("IFCQUANTITYAREA".into(), vec![Part21Value::Real(10.5.into()), Part21Value::Enum("EDGE".into())]), ("IFCPHYSICALSIMPLEQUANTITY".into(), vec![Part21Value::Unset])];
    }
    b.document.instances.push(Part21Instance {
        id: 300,
        entities: vec![("IFCBUILDINGSTOREY".into(), vec![Part21Value::List(vec![Part21Value::Int(1), Part21Value::Int(2)]), Part21Value::Typed { name: "IFCLENGTHMEASURE".into(), items: vec![Part21Value::Real(3000.0.into())] }])],
    });
    vec![Ifc2x3Diff::default(), Ifc2x3Diff::between(&a, &b), Ifc2x3Diff::between(&b, &a)]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
