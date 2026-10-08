//! 🏗️ IFC2X3 model-view-definition editing primitives — the ONE set of Part-21 graph edits the
//! `2x3` standard's three MVD subsets (`✳️cv20`, `✳️cobie`, `✳️sav`) share.
//!
//! A model view definition is a conformance FILTER over one schema, never a fork of it: all three
//! subsets carry the `✳️base` subset's `Ifc2x3Snapshot` verbatim and differ only in which concepts
//! they constrain. Their mutation vocabularies therefore differ in VOCABULARY, not in mechanics —
//! every one of them ultimately sets a positional argument, upserts an instance, removes an
//! instance or re-stamps the header's view definition. Those four mechanics live here rather than
//! three times over, and each subset's own `🧬️schema/🧬️mutations/🦀️.rs` owns the MVD
//! meaning on top of them.
//!
//! Deliberately MVD-agnostic: nothing here knows what Coordination View 2.0, FM Handover or
//! Structural Analysis View require. `expect` is how a caller states the concept it is editing, so
//! a mutation that claims to edit an `IFCPROJECT` fails loudly when the id names something else
//! rather than silently editing it anyway.
//!
//! @see 🪆️subsets/✳️cv20/🧬️schema/🧬️mutations/🦀️.rs — Coordination View 2.0's vocabulary.
//! @see 🪆️subsets/✳️cobie/🧬️schema/🧬️mutations/🦀️.rs — Basic FM Handover's vocabulary.
//! @see 🪆️subsets/✳️sav/🧬️schema/🧬️mutations/🦀️.rs — Structural Analysis View's vocabulary.
//! @see 🦀️oracle.rs — the reference Part-21 codec the same three subsets' oracles share.

use crate::standards::v2x3::subsets::base::schema::diff::Ifc2x3Diff;
use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
use semio_s_artifact_stdio_contract::part21::{Part21Header, Part21Instance, Part21Value};

//#region 🔖️ViewDefinition
/// 🏷️ The view definition the document declares — `FILE_DESCRIPTION`'s first description string,
/// which is the one header field every model view definition is identified by and the field all
/// three subsets' own `check_*_conformance` functions read.
pub fn view_definition(snapshot: &Ifc2x3Snapshot) -> Option<&str> {
    snapshot.document.header.file_description.first().and_then(Part21Value::as_list).and_then(|items| items.iter().find_map(Part21Value::as_str))
}

/// 🏷️ The bare view name inside a `ViewDefinition [...]` stamp, for an inverse that has to restore
/// exactly what the base declared.
pub fn view_definition_name(snapshot: &Ifc2x3Snapshot) -> Option<String> {
    let stamp = view_definition(snapshot)?;
    let inner = stamp.trim().strip_prefix("ViewDefinition")?.trim();
    Some(inner.strip_prefix('[').and_then(|rest| rest.strip_suffix(']')).unwrap_or(inner).trim().to_string())
}
//#endregion 🔖️ViewDefinition

//#region 🔖️Graph
/// 🏷️ An instance's leading EXPRESS type name.
pub fn instance_type(snapshot: &Ifc2x3Snapshot, id: u64) -> Option<&str> {
    snapshot.document.instance(id).and_then(Part21Instance::primary).map(|(name, _)| name)
}

/// 🔎️ One positional argument of an instance's leading entity.
pub fn argument(snapshot: &Ifc2x3Snapshot, id: u64, index: usize) -> Option<&Part21Value> {
    snapshot.document.instance(id).and_then(Part21Instance::primary).and_then(|(_, args)| args.get(index))
}

/// 🔎️ One positional argument read as an entity reference, `None` when it is unset or not a `#id`.
pub fn reference_argument(snapshot: &Ifc2x3Snapshot, id: u64, index: usize) -> Option<u64> {
    argument(snapshot, id, index).and_then(Part21Value::as_ref_id)
}

/// 🧩️ One simple `#id = NAME(args...)` instance.
pub fn simple_instance(id: u64, name: &str, args: Vec<Part21Value>) -> Part21Instance {
    Part21Instance { id, entities: vec![(name.to_string(), args)] }
}

/// 🧭️ Ids of every instance whose leading type name is one of `types`, in document order.
pub fn ids_of_types(snapshot: &Ifc2x3Snapshot, types: &[&str]) -> Vec<u64> {
    snapshot.document.instances.iter().filter(|instance| instance.primary().is_some_and(|(name, _)| types.iter().any(|expected| name.eq_ignore_ascii_case(expected)))).map(|instance| instance.id).collect()
}

/// 🔗️ A `(#a,#b,...)` reference list argument.
pub fn reference_list(ids: &[u64]) -> Part21Value {
    Part21Value::List(ids.iter().copied().map(Part21Value::Ref).collect())
}

/// 🔗️ The entity ids inside a reference-list argument.
pub fn reference_list_ids(value: Option<&Part21Value>) -> Vec<u64> {
    value.and_then(Part21Value::as_list).map(|items| items.iter().filter_map(Part21Value::as_ref_id).collect()).unwrap_or_default()
}

/// 🕳️ An optional value: `Some` as itself, `None` as Part-21's own `$`.
pub fn optional(value: Option<Part21Value>) -> Part21Value {
    value.unwrap_or(Part21Value::Unset)
}

/// 🧭️ The id-sorted view of a document, for comparing two snapshots as EXCHANGE STRUCTURES rather
/// than as text. ISO 10303-21 defines the graph by `#id` reference, never by line order, so an
/// instance this vocabulary removes and its inverse re-appends lands in a different physical
/// position while the exchange structure is unchanged. Everything that compares two IFC2X3
/// documents in this repository does so through this same normalization — the `semantic-ifc-v1`
/// profile's own projection id-sorts for exactly this reason.
pub fn canonical(snapshot: &Ifc2x3Snapshot) -> Ifc2x3Snapshot {
    let mut canonical = snapshot.clone();
    canonical.document.instances.sort_by_key(|instance| instance.id);
    canonical
}
//#endregion 🔖️Graph

//#region 🔖️DiffBuilders
/// 🏷️ The diff that re-stamps `FILE_DESCRIPTION`'s first description string to `ViewDefinition [<view>]`; the empty diff when it already is.
pub fn view_definition_diff(base: &Ifc2x3Snapshot, view: &str) -> Ifc2x3Diff {
    let stamped = Part21Value::List(vec![Part21Value::Str(format!("ViewDefinition [{view}]"))]);
    let mut description = base.document.header.file_description.clone();
    match description.first_mut() {
        Some(slot) => *slot = stamped,
        None => description.push(stamped),
    }
    if description == base.document.header.file_description {
        return Ifc2x3Diff::default();
    }
    Ifc2x3Diff { header: Some(Part21Header { file_description: description, ..base.document.header.clone() }), ..Default::default() }
}

/// ✏️ The diff that replaces one positional argument of an instance's leading entity, padding with `$` when the record is shorter than `index`.
/// `expect` guards the MVD concept the caller claims to edit; an empty `expect` accepts any type.
pub fn argument_diff(base: &Ifc2x3Snapshot, id: u64, expect: &[&str], index: usize, value: Part21Value) -> Result<Ifc2x3Diff, String> {
    let instance = base.document.instance(id).ok_or_else(|| format!("no instance #{id} in the document"))?;
    let (name, args) = instance.entities.first().ok_or_else(|| format!("instance #{id} carries no entity"))?;
    if !expect.is_empty() && !expect.iter().any(|expected| name.eq_ignore_ascii_case(expected)) {
        return Err(format!("instance #{id} is {name} -- expected one of {expect:?}"));
    }
    if args.get(index) == Some(&value) {
        return Ok(Ifc2x3Diff::default());
    }
    let mut replacement = instance.clone();
    let args = &mut replacement.entities[0].1;
    args.resize(args.len().max(index + 1), Part21Value::Unset);
    args[index] = value;
    Ok(Ifc2x3Diff { upserted_instances: vec![replacement], ..Default::default() })
}

/// ➕ The diff that inserts a brand-new instance (at `index`, last by default) or replaces an existing id's whole record in place.
pub fn upsert_diff(base: &Ifc2x3Snapshot, instance: Part21Instance, index: Option<usize>) -> Ifc2x3Diff {
    match base.document.instance(instance.id) {
        Some(existing) if *existing == instance => Ifc2x3Diff::default(),
        Some(_) => Ifc2x3Diff { upserted_instances: vec![instance], ..Default::default() },
        None => {
            let instance_order = index.filter(|at| *at < base.document.instances.len()).map(|at| {
                let mut ids: Vec<u64> = base.document.instances.iter().map(|existing| existing.id).collect();
                ids.insert(at, instance.id);
                ids
            });
            Ifc2x3Diff { upserted_instances: vec![instance], instance_order, ..Default::default() }
        }
    }
}

/// ➖ The diff that deletes an instance. An absent id is an error, never a silent no-op, and `expect` keeps an MVD concept's removal from
/// deleting an unrelated real entity that happens to carry that id.
pub fn remove_diff(base: &Ifc2x3Snapshot, id: u64, expect: &[&str]) -> Result<Ifc2x3Diff, String> {
    let actual = instance_type(base, id).ok_or_else(|| format!("no instance #{id} in the document"))?;
    if !expect.is_empty() && !expect.iter().any(|expected| actual.eq_ignore_ascii_case(expected)) {
        return Err(format!("instance #{id} is {actual} -- expected one of {expect:?}"));
    }
    Ok(Ifc2x3Diff { removed_instances: vec![id], ..Default::default() })
}

/// 🔎️ The position of instance `id`, for a restoring mutation that has to put it back where it stood.
pub fn position(base: &Ifc2x3Snapshot, id: u64) -> Option<usize> {
    base.document.instances.iter().position(|instance| instance.id == id)
}
/// 🧭️ Where an MVD concept's instance id stands in the base: not there, there as the expected concept, or there as an unrelated entity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Standing {
    Absent,
    Present { index: usize },
    Foreign,
}

/// 🧭️ Classifies instance `id` against the entity types `expect` of one MVD concept.
pub fn standing(base: &Ifc2x3Snapshot, id: u64, expect: &[&str]) -> Standing {
    match (instance_type(base, id), position(base, id)) {
        (Some(actual), Some(index)) if expect.iter().any(|expected| actual.eq_ignore_ascii_case(expected)) => Standing::Present { index },
        (Some(_), _) => Standing::Foreign,
        _ => Standing::Absent,
    }
}

/// 🧩️ The diff that sets (`Some`, at `index` when new) or clears (`None`) one MVD concept's instance; an id held by an unrelated entity is an error.
pub fn entity_diff(base: &Ifc2x3Snapshot, id: u64, expect: &[&str], instance: Option<Part21Instance>, index: Option<usize>) -> Result<Ifc2x3Diff, String> {
    match instance {
        None => remove_diff(base, id, expect),
        Some(instance) => {
            if standing(base, id, expect) == Standing::Foreign {
                return Err(format!("instance #{id} is {} -- expected one of {expect:?}", instance_type(base, id).unwrap_or("")));
            }
            Ok(upsert_diff(base, instance, index))
        }
    }
}

//#endregion 🔖️DiffBuilders

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
