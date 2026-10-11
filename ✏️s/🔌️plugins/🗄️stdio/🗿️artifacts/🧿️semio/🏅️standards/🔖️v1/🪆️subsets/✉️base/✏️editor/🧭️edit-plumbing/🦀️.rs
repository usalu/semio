//! 🧭️ Shared plumbing of the semio editors' edit rules: the const builders of the stdio rule tables, the reshape step for kinds whose payload is not a plain field of the edit, and the pointer arithmetic of the editors that compute a gesture's kind.

use protocol::Mutation;
use semio_framework_plugin::{Fault, FaultCode, FaultOrigin};
use semio_framework_value::{DslValue, ToValue};
use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule, InsertRule, RemoveRule, RowKey, Selector, SnapshotEditError, SnapshotEditEvent};

pub(crate) type Entries = Vec<(String, DslValue)>;
pub(crate) type Reshape = fn(&DslValue, &mut Entries) -> Result<(), String>;

pub(crate) const INDEX: Selector = Selector::Index("index");
pub(crate) const ITEM: &str = "$item";

/// ✏️ An entity rule: the value at `path` becomes `value` of `kind`, addressed through `selectors`.
pub(crate) const fn ent(path: &'static str, kind: &'static str, selectors: &'static [Selector], value: &'static str) -> EntityRule {
    EntityRule::new(path, kind, value).selecting(selectors)
}

/// ➕ An insert rule: an insert into the list at `path` raises `kind` carrying the position as `index` (none appends) and the row as `item`.
pub(crate) const fn ins(path: &'static str, kind: &'static str, selectors: &'static [Selector], index: Option<&'static str>, item: &'static str) -> InsertRule {
    InsertRule { path, kind, selectors, index, item, item_fields: &[], with: &[] }
}

/// ➖ A remove rule: a remove of a row of the list at `path` raises `kind` naming the row as `row` says.
pub(crate) const fn rem(path: &'static str, kind: &'static str, selectors: &'static [Selector], row: RowKey) -> RemoveRule {
    RemoveRule { path, kind, selectors, row, with: &[] }
}

pub(crate) const fn named(payload: &'static str, field: &'static str) -> Selector {
    Selector::Field { payload, field }
}

pub(crate) const fn keyed(payload: &'static str, field: &'static str) -> RowKey {
    RowKey::Field { payload, field }
}

/// 🚫️ A refusal with one of the module's `snapshot-edit.*` codes at the pointer `path`.
pub(crate) fn fault(code: &'static str, path: &str, message: impl Into<String>) -> Fault {
    let fault = Fault::new(FaultOrigin::App, FaultCode::new(code), message);
    if path.is_empty() {
        fault
    } else {
        fault.with_param("path", path.to_string())
    }
}

pub(crate) fn unsupported(path: &str, message: impl Into<String>) -> Fault {
    fault("snapshot-edit.unsupported-path", path, message)
}

/// 🚫️ The fault of a contract refusal.
pub(crate) fn edit_fault(error: SnapshotEditError) -> Fault {
    let mut fault = Fault::new(FaultOrigin::App, FaultCode::new(error.code), error.message);
    if !error.path.is_empty() {
        fault = fault.with_param("path", error.path);
    }
    fault.span = error.span;
    fault
}

/// 📍️ The pointer an event addresses.
pub(crate) fn event_path(event: &SnapshotEditEvent) -> &str {
    match event {
        SnapshotEditEvent::SetValue { path, .. } | SnapshotEditEvent::InsertValue { path, .. } | SnapshotEditEvent::RemoveValue { path } | SnapshotEditEvent::MoveValue { path, .. } | SnapshotEditEvent::RenameKey { path, .. } => path,
        SnapshotEditEvent::ReplaceSource { .. } => "",
    }
}

/// 🧩️ Decodes an RFC 6901 pointer into its segments.
pub(crate) fn segments(path: &str) -> Result<Vec<String>, Fault> {
    if path.is_empty() {
        return Ok(Vec::new());
    }
    let rest = path.strip_prefix('/').ok_or_else(|| fault("snapshot-edit.path-invalid", path, "a pointer starts with '/'"))?;
    Ok(rest.split('/').map(|segment| segment.replace("~1", "/").replace("~0", "~")).collect())
}

/// 🔢️ A list position spelled by one pointer segment; `-` addresses the end when `end` is the list length.
pub(crate) fn position(segment: &str, end: Option<usize>, path: &str) -> Result<usize, Fault> {
    match (segment, end) {
        ("-", Some(end)) => Ok(end),
        _ => segment.parse::<usize>().map_err(|_| fault("snapshot-edit.index-invalid", path, format!("'{segment}' is not a list position"))),
    }
}

/// 🔎️ The member `name` of a record.
pub(crate) fn field<'a>(value: &'a DslValue, name: &str) -> Option<&'a DslValue> {
    match value {
        DslValue::Object(entries) => entries.iter().find(|(key, _)| key == name).map(|(_, value)| value),
        _ => None,
    }
}

/// 🔎️ The row at `index` of a list.
pub(crate) fn item(value: &DslValue, index: usize) -> Option<&DslValue> {
    match value {
        DslValue::Array(items) => items.get(index),
        _ => None,
    }
}

/// 🔎️ The first row of the list `name` of a record whose `by` member equals `wanted`.
pub(crate) fn find<'a>(record: &'a DslValue, name: &str, key_field: &str, wanted: &DslValue) -> Option<&'a DslValue> {
    match field(record, name) {
        Some(DslValue::Array(rows)) => rows.iter().find(|row| field(row, key_field) == Some(wanted)),
        _ => None,
    }
}

pub(crate) fn uint(value: &DslValue) -> Option<usize> {
    value.as_i64().and_then(|number| usize::try_from(number).ok())
}

/// 🧲️ Removes the payload entry `name`.
pub(crate) fn take(entries: &mut Entries, name: &str) -> Result<DslValue, String> {
    let at = entries.iter().position(|(key, _)| key == name).ok_or_else(|| format!("the edit carries no '{name}'"))?;
    Ok(entries.remove(at).1)
}

fn has(entries: &Entries, name: &str) -> bool {
    entries.iter().any(|(key, _)| key == name)
}

/// 💧️ Replaces the inserted record `$item`, when the payload carries one, by its members.
pub(crate) fn spread_item(_: &DslValue, entries: &mut Entries) -> Result<(), String> {
    if !has(entries, ITEM) {
        return Ok(());
    }
    let DslValue::Object(members) = take(entries, ITEM)? else { return Err("the inserted row is not a record".into()) };
    entries.extend(members);
    Ok(())
}

/// 🐍️ [`spread_item`] for the subsets whose payload members are snake_case: the inserted row's camelCase members take their payload spelling.
pub(crate) fn spread_snake(tree: &DslValue, entries: &mut Entries) -> Result<(), String> {
    let spread = has(entries, ITEM);
    let before = entries.len();
    spread_item(tree, entries)?;
    if spread {
        for (name, _) in entries.iter_mut().skip(before - 1) {
            *name = name.chars().flat_map(|letter| if letter.is_ascii_uppercase() { vec!['_', letter.to_ascii_lowercase()] } else { vec![letter] }).collect();
        }
    }
    Ok(())
}

/// 📎️ Completes the payload with the members `names` it lacks, read from `row`.
pub(crate) fn complete(entries: &mut Entries, row: &DslValue, names: &[&str]) -> Result<(), String> {
    for name in names {
        if !has(entries, name) {
            let value = field(row, name).ok_or_else(|| format!("the row has no '{name}'"))?;
            entries.push(((*name).to_string(), value.clone()));
        }
    }
    Ok(())
}

/// 📎️ [`complete`] with the row reached through `route`: each step names a list of the current record and the payload member holding the position in it.
pub(crate) fn complete_in(tree: &DslValue, entries: &mut Entries, route: &[(&str, &str)], names: &[&str]) -> Result<(), String> {
    let mut row = tree;
    for (list, payload) in route {
        let index = entries.iter().find(|(key, _)| key == *payload).and_then(|(_, value)| uint(value)).ok_or_else(|| format!("the edit carries no position '{payload}'"))?;
        row = field(row, list).and_then(|rows| item(rows, index)).ok_or_else(|| format!("'{list}' has no row {index}"))?;
    }
    complete(entries, row, names)
}

/// 📎️ [`complete`] with the row of the list `list` of the document whose `by` member the payload already names.
pub(crate) fn complete_by(tree: &DslValue, entries: &mut Entries, list: &str, key_field: &str, names: &[&str]) -> Result<(), String> {
    let wanted = entries.iter().find(|(key, _)| key == key_field).map(|(_, value)| value.clone()).ok_or_else(|| format!("the edit carries no '{key_field}'"))?;
    let row = find(tree, list, key_field, &wanted).ok_or_else(|| format!("'{list}' has no row with that '{key_field}'"))?;
    complete(entries, row, names)
}

/// 🎯️ Builds the one kind of a payload.
pub(crate) fn build<S, M>(tree: &DslValue, reshapes: &[(&str, Reshape)], kind: &str, mut entries: Entries, path: &str) -> Result<Vec<M>, Fault>
where
    M: Mutation<S>,
{
    if let Some((_, reshape)) = reshapes.iter().find(|(name, _)| *name == kind) {
        reshape(tree, &mut entries).map_err(|message| unsupported(path, message))?;
    }
    M::from_payload_value(kind, DslValue::object(entries)).map(|mutation| vec![mutation]).map_err(|error| fault("snapshot-edit.schema-invalid", path, error.to_string()))
}

/// 🎯️ The concrete kind a pointer edit denotes through `rules`, its payload completed by `reshapes`; none when the edit changes nothing.
pub(crate) fn resolve<S, M>(rules: &EditRules, reshapes: &[(&str, Reshape)], snapshot: &S, event: &SnapshotEditEvent) -> Result<Vec<M>, Fault>
where
    S: ToValue,
    M: Mutation<S>,
{
    let tree = snapshot.to_value();
    match rules.plan(&tree, event).map_err(edit_fault)? {
        Some(plan) => build::<S, M>(&tree, reshapes, plan.kind, plan.entries, event_path(event)),
        None => Ok(Vec::new()),
    }
}

/// 🎯️ The one kind a computed gesture denotes, its payload the list positions `numbers` names.
pub(crate) fn positions<S, M>(snapshot: &S, event: &SnapshotEditEvent, kind: &str, numbers: &[(&str, usize)]) -> Result<Vec<M>, Fault>
where
    S: ToValue,
    M: Mutation<S>,
{
    let entries = numbers.iter().map(|(name, number)| ((*name).to_string(), DslValue::uint(*number as u64))).collect();
    build::<S, M>(&snapshot.to_value(), &[], kind, entries, event_path(event))
}

/// 📚️ A record member holding one optional child: the kind creating it and the kind deleting it.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Slot {
    pub field: &'static str,
    pub create: &'static str,
    pub delete: &'static str,
}

/// 🪆️ The create or delete kind of an edit of an optional child member; `None` when the edit addresses no slot.
pub(crate) fn slot_edit<S, M>(slots: &[Slot], spread: Reshape, snapshot: &S, event: &SnapshotEditEvent) -> Result<Option<Vec<M>>, Fault>
where
    S: ToValue,
    M: Mutation<S>,
{
    let path = event_path(event);
    let parts = segments(path)?;
    let [name] = parts.as_slice() else { return Ok(None) };
    let Some(slot) = slots.iter().find(|slot| slot.field == name) else { return Ok(None) };
    let tree = snapshot.to_value();
    let current = field(&tree, slot.field).filter(|value| !value.is_null());
    match (event, current) {
        (SnapshotEditEvent::RemoveValue { .. }, Some(_)) | (SnapshotEditEvent::SetValue { value: DslValue::Null, .. }, Some(_)) => build::<S, M>(&tree, &[], slot.delete, Vec::new(), path).map(Some),
        (SnapshotEditEvent::RemoveValue { .. }, None) => Err(fault("snapshot-edit.path-missing", path, format!("'{}' holds no child", slot.field))),
        (SnapshotEditEvent::SetValue { value: DslValue::Null, .. }, None) => Ok(Some(Vec::new())),
        (SnapshotEditEvent::SetValue { value, .. } | SnapshotEditEvent::InsertValue { value, .. }, None) => build::<S, M>(&tree, &[(slot.create, spread)], slot.create, vec![(ITEM.to_string(), value.clone())], path).map(Some),
        (SnapshotEditEvent::SetValue { value, .. }, Some(existing)) if value == existing => Ok(Some(Vec::new())),
        _ => Err(unsupported(path, format!("no kind replaces the child '{}': delete it, then create the other", slot.field))),
    }
}

/// 🔀️ The two positions of a move of one row of the list at `list` (a document-level pointer prefix), none when the event moves something else.
pub(crate) fn list_move(event: &SnapshotEditEvent, list: &[&str]) -> Result<Option<(usize, usize)>, Fault> {
    let SnapshotEditEvent::MoveValue { from, path } = event else { return Ok(None) };
    let (origin, target) = (segments(from)?, segments(path)?);
    let row = |parts: &[String]| -> Option<String> {
        (parts.len() == list.len() + 1 && parts.iter().zip(list).all(|(part, name)| part == name)).then(|| parts[list.len()].clone())
    };
    match (row(&origin), row(&target)) {
        (Some(from), Some(to)) => Ok(Some((position(&from, None, path)?, position(&to, None, path)?))),
        _ => Ok(None),
    }
}

/// 🧱️ A record made of `entries`.
pub(crate) fn record(entries: impl IntoIterator<Item = (&'static str, DslValue)>) -> DslValue {
    DslValue::object(entries.into_iter().map(|(name, value)| (name.to_string(), value)))
}

fn slot_of<'a>(value: &'a mut DslValue, segment: &str, path: &str) -> Result<&'a mut DslValue, Fault> {
    match value {
        DslValue::Object(entries) => entries.iter_mut().find(|(key, _)| key == segment).map(|(_, value)| value).ok_or_else(|| fault("snapshot-edit.path-missing", path, format!("no member '{segment}'"))),
        DslValue::Array(items) => {
            let index = position(segment, None, path)?;
            items.get_mut(index).ok_or_else(|| fault("snapshot-edit.index-out-of-bounds", path, format!("no row {index}")))
        }
        _ => Err(fault("snapshot-edit.not-container", path, format!("'{segment}' has a scalar parent"))),
    }
}

fn parent_of<'a>(root: &'a mut DslValue, relative: &'a [String], path: &str) -> Result<(&'a mut DslValue, &'a str), Fault> {
    let (last, parents) = relative.split_last().ok_or_else(|| unsupported(path, "the edit addresses no member"))?;
    let mut cursor = root;
    for segment in parents {
        cursor = slot_of(cursor, segment, path)?;
    }
    Ok((cursor, last.as_str()))
}

fn remove_at(root: &mut DslValue, relative: &[String], path: &str) -> Result<DslValue, Fault> {
    let (parent, last) = parent_of(root, relative, path)?;
    match parent {
        DslValue::Object(entries) => {
            let at = entries.iter().position(|(key, _)| key == last).ok_or_else(|| fault("snapshot-edit.path-missing", path, format!("no member '{last}'")))?;
            Ok(entries.remove(at).1)
        }
        DslValue::Array(items) => {
            let index = position(last, None, path)?;
            if index >= items.len() {
                return Err(fault("snapshot-edit.index-out-of-bounds", path, format!("no row {index}")));
            }
            Ok(items.remove(index))
        }
        _ => Err(fault("snapshot-edit.not-container", path, format!("'{last}' has a scalar parent"))),
    }
}

fn insert_at(root: &mut DslValue, relative: &[String], value: DslValue, path: &str) -> Result<(), Fault> {
    let (parent, last) = parent_of(root, relative, path)?;
    match parent {
        DslValue::Object(entries) => {
            if entries.iter().any(|(key, _)| key == last) {
                return Err(fault("snapshot-edit.key-exists", path, format!("member '{last}' exists")));
            }
            entries.push((last.to_string(), value));
            Ok(())
        }
        DslValue::Array(items) => {
            let index = position(last, Some(items.len()), path)?;
            if index > items.len() {
                return Err(fault("snapshot-edit.index-out-of-bounds", path, format!("no slot {index}")));
            }
            items.insert(index, value);
            Ok(())
        }
        _ => Err(fault("snapshot-edit.not-container", path, format!("'{last}' has a scalar parent"))),
    }
}

/// 📝️ A clone of `entity` with the event applied at the pointer below the first `depth` segments of the event's pointer.
pub(crate) fn edited(entity: &DslValue, event: &SnapshotEditEvent, depth: usize) -> Result<DslValue, Fault> {
    let path = event_path(event);
    let below = |pointer: &str| -> Result<Vec<String>, Fault> { Ok(segments(pointer)?.into_iter().skip(depth).collect()) };
    let mut next = entity.clone();
    match event {
        SnapshotEditEvent::SetValue { value, .. } => {
            let relative = below(path)?;
            if relative.is_empty() {
                return Ok(value.clone());
            }
            *slot_at(&mut next, &relative, path)? = value.clone();
        }
        SnapshotEditEvent::InsertValue { value, .. } => insert_at(&mut next, &below(path)?, value.clone(), path)?,
        SnapshotEditEvent::RemoveValue { .. } => {
            remove_at(&mut next, &below(path)?, path)?;
        }
        SnapshotEditEvent::MoveValue { from, .. } => {
            let moved = remove_at(&mut next, &below(from)?, path)?;
            insert_at(&mut next, &below(path)?, moved, path)?;
        }
        SnapshotEditEvent::RenameKey { key, .. } => {
            let relative = below(path)?;
            let (parent, last) = parent_of(&mut next, &relative, path)?;
            let DslValue::Object(entries) = parent else { return Err(fault("snapshot-edit.not-object", path, "only a record member has a name")) };
            let at = entries.iter().position(|(name, _)| name == last).ok_or_else(|| fault("snapshot-edit.path-missing", path, format!("no member '{last}'")))?;
            entries[at].0 = key.clone();
        }
        SnapshotEditEvent::ReplaceSource { .. } => return Err(unsupported(path, "replacing the whole source is a document load, not an edit")),
    }
    Ok(next)
}

fn slot_at<'a>(root: &'a mut DslValue, relative: &[String], path: &str) -> Result<&'a mut DslValue, Fault> {
    relative.iter().try_fold(root, |cursor, segment| slot_of(cursor, segment, path))
}
