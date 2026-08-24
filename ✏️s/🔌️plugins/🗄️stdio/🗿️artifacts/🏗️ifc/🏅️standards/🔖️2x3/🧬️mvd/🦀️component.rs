//! 🏗️ IFC2X3 model-view-definition editing primitives — the ONE set of Part-21 graph edits the
//! `2x3` standard's three MVD subsets (`✳️cv20`, `✳️cobie`, `✳️sav`) share.
//!
//! A model view definition is a conformance FILTER over one schema, never a fork of it: all three
//! subsets carry the `✳️any` subset's `Ifc2x3Snapshot` verbatim and differ only in which concepts
//! they constrain. Their mutation vocabularies therefore differ in VOCABULARY, not in mechanics —
//! every one of them ultimately sets a positional argument, upserts an instance, removes an
//! instance or re-stamps the header's view definition. Those four mechanics live here rather than
//! three times over, and each subset's own `🧬️schema/🧬️mutations/🦀️component.rs` owns the MVD
//! meaning on top of them.
//!
//! Deliberately MVD-agnostic: nothing here knows what Coordination View 2.0, FM Handover or
//! Structural Analysis View require. `expect` is how a caller states the concept it is editing, so
//! a mutation that claims to edit an `IFCPROJECT` fails loudly when the id names something else
//! rather than silently editing it anyway.
//!
//! @see 🪆️subsets/✳️cv20/🧬️schema/🧬️mutations/🦀️component.rs — Coordination View 2.0's vocabulary.
//! @see 🪆️subsets/✳️cobie/🧬️schema/🧬️mutations/🦀️component.rs — Basic FM Handover's vocabulary.
//! @see 🪆️subsets/✳️sav/🧬️schema/🧬️mutations/🦀️component.rs — Structural Analysis View's vocabulary.
//! @see 🧪️oracle/🦀️component.rs — the reference Part-21 codec the same three subsets' oracles share.

use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
use crate::artifacts::step::engine::part21::{Part21Instance, Part21Value};

//#region 🔖️ViewDefinition
/// 🏷️ The view definition the document declares — `FILE_DESCRIPTION`'s first description string,
/// which is the one header field every model view definition is identified by and the field all
/// three subsets' own `check_*_conformance` functions read.
pub fn view_definition(snapshot: &Ifc2x3Snapshot) -> Option<&str> {
    snapshot.document.header.file_description.first().and_then(Part21Value::as_list).and_then(|items| items.iter().find_map(Part21Value::as_str))
}

/// 🏷️ Re-stamps `FILE_DESCRIPTION`'s first description string to `ViewDefinition [<view>]`.
pub fn set_view_definition(snapshot: &mut Ifc2x3Snapshot, view: &str) {
    let stamped = Part21Value::List(vec![Part21Value::Str(format!("ViewDefinition [{view}]"))]);
    match snapshot.document.header.file_description.first_mut() {
        Some(slot) => *slot = stamped,
        None => snapshot.document.header.file_description.push(stamped),
    }
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

/// ✏️ Replaces one positional argument of an instance's leading entity, padding with `$` when the
/// record is shorter than `index`. `expect` guards the MVD concept the caller claims to edit; an
/// empty `expect` accepts any type.
pub fn set_argument(snapshot: &mut Ifc2x3Snapshot, id: u64, expect: &[&str], index: usize, value: Part21Value) -> Result<(), String> {
    let instance = snapshot.document.instances.iter_mut().find(|candidate| candidate.id == id).ok_or_else(|| format!("no instance #{id} in the document"))?;
    let (name, args) = instance.entities.first_mut().ok_or_else(|| format!("instance #{id} carries no entity"))?;
    if !expect.is_empty() && !expect.iter().any(|expected| name.eq_ignore_ascii_case(expected)) {
        return Err(format!("instance #{id} is {name} -- expected one of {expect:?}"));
    }
    while args.len() <= index {
        args.push(Part21Value::Unset);
    }
    args[index] = value;
    Ok(())
}

/// 🧩️ One simple `#id = NAME(args...)` instance.
pub fn simple_instance(id: u64, name: &str, args: Vec<Part21Value>) -> Part21Instance {
    Part21Instance { id, entities: vec![(name.to_string(), args)] }
}

/// ➕ Inserts a brand-new instance, or replaces an existing id's whole record.
pub fn upsert_instance(snapshot: &mut Ifc2x3Snapshot, instance: Part21Instance) {
    match snapshot.document.instances.iter_mut().find(|candidate| candidate.id == instance.id) {
        Some(existing) => *existing = instance,
        None => snapshot.document.instances.push(instance),
    }
}

/// ➖ Deletes an instance. An absent id is an error, never a silent no-op, and `expect` keeps an
/// MVD concept's removal from deleting an unrelated real entity that happens to carry that id.
pub fn remove_instance(snapshot: &mut Ifc2x3Snapshot, id: u64, expect: &[&str]) -> Result<(), String> {
    let actual = instance_type(snapshot, id).ok_or_else(|| format!("no instance #{id} in the document"))?.to_string();
    if !expect.is_empty() && !expect.iter().any(|expected| actual.eq_ignore_ascii_case(expected)) {
        return Err(format!("instance #{id} is {actual} -- expected one of {expect:?}"));
    }
    snapshot.document.instances.retain(|instance| instance.id != id);
    Ok(())
}

/// 🧭️ Ids of every instance whose leading type name is one of `types`, in document order.
pub fn ids_of_types(snapshot: &Ifc2x3Snapshot, types: &[&str]) -> Vec<u64> {
    snapshot
        .document
        .instances
        .iter()
        .filter(|instance| instance.primary().map(|(name, _)| types.iter().any(|expected| name.eq_ignore_ascii_case(expected))).unwrap_or(false))
        .map(|instance| instance.id)
        .collect()
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

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::step::engine::part21::{Part21Document, Part21Header};

    fn snapshot() -> Ifc2x3Snapshot {
        let header = Part21Header {
            file_description: vec![Part21Value::List(vec![Part21Value::Str("ViewDefinition [CoordinationView_V2.0]".into())]), Part21Value::Str("2;1".into())],
            file_name: vec![],
            file_schema: vec![Part21Value::List(vec![Part21Value::Str("IFC2X3".into())])],
        };
        let project = simple_instance(1, "IFCPROJECT", vec![Part21Value::Str("guid".into()), Part21Value::Unset, Part21Value::Str("Project".into()), Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, Part21Value::Ref(2)]);
        let units = simple_instance(2, "IFCUNITASSIGNMENT", vec![]);
        Ifc2x3Snapshot { schema: "stdio.ifc.2x3".into(), document: Part21Document { header, instances: vec![project, units] }, edm_preamble: None }
    }

    #[test]
    fn view_definition_reads_and_restamps_the_header() {
        let mut snap = snapshot();
        assert_eq!(view_definition_name(&snap).as_deref(), Some("CoordinationView_V2.0"));
        set_view_definition(&mut snap, "FMHandOverView");
        assert_eq!(view_definition(&snap), Some("ViewDefinition [FMHandOverView]"));
        assert_eq!(view_definition_name(&snap).as_deref(), Some("FMHandOverView"));
    }

    #[test]
    fn set_argument_guards_the_expected_type() {
        let mut snap = snapshot();
        assert_eq!(reference_argument(&snap, 1, 8), Some(2));
        set_argument(&mut snap, 1, &["IFCPROJECT"], 8, Part21Value::Unset).expect("the project accepts the edit");
        assert_eq!(reference_argument(&snap, 1, 8), None);
        assert!(set_argument(&mut snap, 2, &["IFCPROJECT"], 8, Part21Value::Unset).is_err(), "an IFCUNITASSIGNMENT is not an IFCPROJECT");
        assert!(set_argument(&mut snap, 99, &[], 0, Part21Value::Unset).is_err(), "an absent id is an error, never a silent no-op");
    }

    #[test]
    fn set_argument_pads_a_short_record() {
        let mut snap = snapshot();
        set_argument(&mut snap, 2, &["IFCUNITASSIGNMENT"], 3, Part21Value::Str("padded".into())).expect("padding");
        assert_eq!(argument(&snap, 2, 0), Some(&Part21Value::Unset));
        assert_eq!(argument(&snap, 2, 3), Some(&Part21Value::Str("padded".into())));
    }

    #[test]
    fn upsert_replaces_and_remove_guards() {
        let mut snap = snapshot();
        upsert_instance(&mut snap, simple_instance(3, "IFCSPACE", vec![Part21Value::Str("guid".into())]));
        assert_eq!(instance_type(&snap, 3), Some("IFCSPACE"));
        upsert_instance(&mut snap, simple_instance(3, "IFCSPACE", vec![Part21Value::Str("other".into())]));
        assert_eq!(snap.document.instances.len(), 3, "upsert replaces an existing id rather than appending a duplicate");
        assert!(remove_instance(&mut snap, 3, &["IFCPROJECT"]).is_err(), "the type guard refuses an unrelated concept");
        remove_instance(&mut snap, 3, &["IFCSPACE"]).expect("removing the space");
        assert!(remove_instance(&mut snap, 3, &["IFCSPACE"]).is_err(), "removing an absent id is an error");
    }

    #[test]
    fn reference_lists_round_trip() {
        let value = reference_list(&[7, 9]);
        assert_eq!(reference_list_ids(Some(&value)), vec![7, 9]);
        assert_eq!(reference_list_ids(None), Vec::<u64>::new());
        assert_eq!(optional(None), Part21Value::Unset);
    }

    #[test]
    fn canonical_compares_documents_as_exchange_structures() {
        let start = snapshot();
        let mut reordered = start.clone();
        reordered.document.instances.reverse();
        assert_ne!(reordered, start, "Part21Document equality is line-order sensitive");
        assert_eq!(canonical(&reordered), canonical(&start), "the exchange structure is unchanged");
    }

    #[test]
    fn ids_of_types_finds_the_concept_population() {
        let snap = snapshot();
        assert_eq!(ids_of_types(&snap, &["IFCPROJECT"]), vec![1]);
        assert_eq!(ids_of_types(&snap, &["IFCSPACE"]), Vec::<u64>::new());
    }
}
//#endregion 🧪️Tests
