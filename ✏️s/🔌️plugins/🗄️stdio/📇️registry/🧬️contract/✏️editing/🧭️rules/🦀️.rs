//! 🧭️ Per-artifact edit rules: which JSON-pointer snapshot edit raises which ONE concrete kind.
//!
//! A details-pane edit addresses a field of the document by RFC 6901 pointer. An artifact declares, once, a table of
//! pointer templates (`*` stands for one array index) and the kind each one raises; [`EditRules::resolve`] reads the pointer
//! against the current document, takes the selectors the pointer names (list positions or row keys) and the new value of
//! the addressed entity, and builds that kind's payload. The edited document is never rebuilt or compared: an address no rule
//! names is refused with a `snapshot-edit.unsupported-path` fault naming it, and the kind's own diff guards the value.
//!
//! - [`EntityRule`] — the entity at `path` is replaced by a kind carrying its whole new value; any `set`, `insert`, `remove`,
//!   `move` or `rename` strictly inside it (or a `set` of the entity itself) raises it.
//! - [`InsertRule`] — an insert into the list at `path` raises a kind carrying the position (when `index` names it) and the item.
//! - [`RemoveRule`] — a remove of a row of the list at `path` raises a kind addressing the row by position or by a key field.
//!
//! Modelled on norm's `NormEditRules`; the table is plain `const` data so every language implementation can read it.

use super::{array_index, decode_pointer, insert_value, move_value, object_index, remove_value, rename_key, set_value, values_equivalent, SnapshotEditError, SnapshotEditEvent};
use crate::kernel::Mutation;
use semio_framework_value::{DslValue, ToValue};

/// 🔑 Where one `*` of a rule template lands in the kind payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selector {
    /// 🔢️ The list position the wildcard matched, spelled `payload`.
    Index(&'static str),
    /// 🏷️ The `field` of the row the wildcard matched (its name, id, …), spelled `payload`.
    Field { payload: &'static str, field: &'static str },
    /// 🫥️ The kind implies the row; nothing lands in the payload.
    Implied,
}

/// 📎 A payload field the kind needs that the edit does not change: `payload` takes the current value of the document at `pointer`
/// (each `*` of the pointer takes the position the rule's own wildcards matched, in order).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Carried {
    pub payload: &'static str,
    pub pointer: &'static str,
}

/// ✏️ One replaceable entity: the `path` template and the kind that carries its whole new value as `value`.
#[derive(Clone, Copy, Debug)]
pub struct EntityRule {
    pub path: &'static str,
    pub kind: &'static str,
    pub selectors: &'static [Selector],
    pub value: &'static str,
    pub with: &'static [Carried],
}

impl EntityRule {
    /// 🧱️ A rule with no selectors and nothing carried.
    pub const fn new(path: &'static str, kind: &'static str, value: &'static str) -> Self {
        Self { path, kind, selectors: &[], value, with: &[] }
    }

    /// 🔑 The same rule addressing its wildcards through `selectors`.
    pub const fn selecting(self, selectors: &'static [Selector]) -> Self {
        Self { path: self.path, kind: self.kind, selectors, value: self.value, with: self.with }
    }

    /// 📎 The same rule also carrying the listed current document fields.
    pub const fn carrying(self, with: &'static [Carried]) -> Self {
        Self { path: self.path, kind: self.kind, selectors: self.selectors, value: self.value, with }
    }
}

/// ➕ One insertable list: the `path` template of the list, the kind, the payload name of the position (`None` appends) and of the item
/// (an empty `item` leaves the whole item out, for kinds that take its fields through [`ItemField`]).
#[derive(Clone, Copy, Debug)]
pub struct InsertRule {
    pub path: &'static str,
    pub kind: &'static str,
    pub selectors: &'static [Selector],
    pub index: Option<&'static str>,
    pub item: &'static str,
    pub item_fields: &'static [ItemField],
    pub with: &'static [Carried],
}

impl InsertRule {
    /// 🧱️ A rule that appends the item, carrying it as `item`.
    pub const fn new(path: &'static str, kind: &'static str, item: &'static str) -> Self {
        Self { path, kind, selectors: &[], index: None, item, item_fields: &[], with: &[] }
    }

    /// 🔢️ The same rule carrying the insert position as `index`.
    pub const fn at(self, index: &'static str) -> Self {
        Self { path: self.path, kind: self.kind, selectors: self.selectors, index: Some(index), item: self.item, item_fields: self.item_fields, with: self.with }
    }

    /// 🔑 The same rule addressing its wildcards through `selectors`.
    pub const fn selecting(self, selectors: &'static [Selector]) -> Self {
        Self { path: self.path, kind: self.kind, selectors, index: self.index, item: self.item, item_fields: self.item_fields, with: self.with }
    }

    /// 🏷️ The same rule also lifting key fields of the inserted item into the payload.
    pub const fn keyed(self, item_fields: &'static [ItemField]) -> Self {
        Self { path: self.path, kind: self.kind, selectors: self.selectors, index: self.index, item: self.item, item_fields, with: self.with }
    }

    /// 📎 The same rule also carrying the listed current document fields.
    pub const fn carrying(self, with: &'static [Carried]) -> Self {
        Self { path: self.path, kind: self.kind, selectors: self.selectors, index: self.index, item: self.item, item_fields: self.item_fields, with }
    }
}

/// 🏷️ A payload field lifted from the inserted item: `payload` takes the item's `field`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ItemField {
    pub payload: &'static str,
    pub field: &'static str,
}

/// 🎯 How a remove kind addresses the row it deletes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RowKey {
    /// 🔢️ By list position, spelled `payload`.
    Index(&'static str),
    /// 🏷️ By a key `field` of the row, spelled `payload`.
    Field { payload: &'static str, field: &'static str },
}

/// ➖ One removable list: the `path` template of the list, the kind, and how the kind names the row.
#[derive(Clone, Copy, Debug)]
pub struct RemoveRule {
    pub path: &'static str,
    pub kind: &'static str,
    pub selectors: &'static [Selector],
    pub row: RowKey,
    pub with: &'static [Carried],
}

impl RemoveRule {
    /// 🔢️ A rule addressing the removed row by its list position, spelled `payload`.
    pub const fn by_index(path: &'static str, kind: &'static str, payload: &'static str) -> Self {
        Self { path, kind, selectors: &[], row: RowKey::Index(payload), with: &[] }
    }

    /// 🏷️ A rule addressing the removed row by its key `field`, spelled `payload`.
    pub const fn by_key(path: &'static str, kind: &'static str, payload: &'static str, field: &'static str) -> Self {
        Self { path, kind, selectors: &[], row: RowKey::Field { payload, field }, with: &[] }
    }

    /// 🔑 The same rule addressing its wildcards through `selectors`.
    pub const fn selecting(self, selectors: &'static [Selector]) -> Self {
        Self { path: self.path, kind: self.kind, selectors, row: self.row, with: self.with }
    }

    /// 📎 The same rule also carrying the listed current document fields.
    pub const fn carrying(self, with: &'static [Carried]) -> Self {
        Self { path: self.path, kind: self.kind, selectors: self.selectors, row: self.row, with }
    }
}

/// 📚 An artifact's declarative edit vocabulary.
#[derive(Clone, Copy, Debug)]
pub struct EditRules {
    pub entities: &'static [EntityRule],
    pub inserts: &'static [InsertRule],
    pub removes: &'static [RemoveRule],
}

struct Capture<'a> {
    index: usize,
    row: &'a DslValue,
}

fn template_segments(template: &str) -> Vec<&str> {
    if template.is_empty() {
        Vec::new()
    } else {
        template[1..].split('/').collect()
    }
}

fn encode_segment(segment: &str) -> String {
    segment.replace('~', "~0").replace('/', "~1")
}

fn relative_pointer(segments: &[String]) -> String {
    segments.iter().map(|segment| format!("/{}", encode_segment(segment))).collect()
}

fn child<'a>(cursor: &'a DslValue, segment: &str, path: &str) -> Result<&'a DslValue, SnapshotEditError> {
    match cursor {
        DslValue::Object(entries) => Ok(&entries[object_index(entries, segment, path)?].1),
        DslValue::Array(items) => Ok(&items[array_index(segment, items.len(), path, false)?]),
        _ => Err(SnapshotEditError::new("snapshot-edit.not-container", path, format!("path segment '{segment}' has a scalar parent"))),
    }
}

fn value_at<'a>(root: &'a DslValue, segments: &[String], path: &str) -> Result<&'a DslValue, SnapshotEditError> {
    segments.iter().try_fold(root, |cursor, segment| child(cursor, segment, path))
}

/// 🧭️ Matches `template` against the first segments of `segments`, walking `tree`; `Ok(None)` when a literal segment differs.
fn match_prefix<'a>(template: &[&str], segments: &[String], tree: &'a DslValue, path: &str) -> Result<Option<Vec<Capture<'a>>>, SnapshotEditError> {
    if template.len() > segments.len() {
        return Ok(None);
    }
    let mut cursor = tree;
    let mut captures = Vec::new();
    for (expected, segment) in template.iter().zip(segments) {
        if *expected != "*" && expected != segment {
            return Ok(None);
        }
        if *expected == "*" {
            let DslValue::Array(items) = cursor else { return Ok(None) };
            let index = array_index(segment, items.len(), path, false)?;
            captures.push(Capture { index, row: &items[index] });
        }
        cursor = child(cursor, segment, path)?;
    }
    Ok(Some(captures))
}

fn selector_entries(selectors: &[Selector], captures: &[Capture<'_>], path: &str) -> Result<Vec<(String, DslValue)>, SnapshotEditError> {
    if selectors.len() != captures.len() {
        return Err(SnapshotEditError::new("snapshot-edit.rule-arity", path, format!("the pointer selects {} list rows, its rule addresses {}", captures.len(), selectors.len())));
    }
    let mut entries = Vec::new();
    for (selector, capture) in selectors.iter().zip(captures) {
        match selector {
            Selector::Index(payload) => entries.push(((*payload).to_string(), DslValue::uint(capture.index as u64))),
            Selector::Field { payload, field } => entries.push(((*payload).to_string(), row_field(capture.row, field, path)?)),
            Selector::Implied => {}
        }
    }
    Ok(entries)
}

/// 📎 The carried fields the document currently holds; a field it omits (a defaulted one) is left out of the payload, which defaults it.
fn carried_entries(with: &[Carried], captures: &[Capture<'_>], tree: &DslValue, path: &str) -> Result<Vec<(String, DslValue)>, SnapshotEditError> {
    let mut entries = Vec::new();
    for carried in with {
        let mut positions = captures.iter().map(|capture| capture.index.to_string());
        let pointer: String = carried.pointer.split('/').map(|segment| if segment == "*" { positions.next().unwrap_or_else(|| "*".to_string()) } else { segment.to_string() }).collect::<Vec<_>>().join("/");
        if let Ok(value) = value_at(tree, &decode_pointer(&pointer)?, path) {
            entries.push((carried.payload.to_string(), value.clone()));
        }
    }
    Ok(entries)
}

fn row_field(row: &DslValue, field: &str, path: &str) -> Result<DslValue, SnapshotEditError> {
    let DslValue::Object(entries) = row else { return Err(SnapshotEditError::new("snapshot-edit.not-object", path, "the addressed row is not a record")) };
    Ok(entries[object_index(entries, field, path)?].1.clone())
}

/// 📦️ The kind an edit resolved to and the payload entries it carries.
#[derive(Clone, Debug, PartialEq)]
pub struct EditPlan {
    pub kind: &'static str,
    pub entries: Vec<(String, DslValue)>,
}

fn unsupported(path: &str) -> SnapshotEditError {
    SnapshotEditError::new("snapshot-edit.unsupported-path", path, format!("no kind edits '{path}'"))
}

impl EditRules {
    /// 🎯 The concrete kind an edit denotes — at most one, none when the edit changes nothing.
    pub fn resolve<S, M>(&self, snapshot: &S, event: &SnapshotEditEvent) -> Result<Vec<M>, SnapshotEditError>
    where
        S: ToValue,
        M: Mutation<S>,
    {
        let path = event_path(event);
        self.plan(&snapshot.to_value(), event)?
            .map(|plan| <M as Mutation<S>>::from_payload_value(plan.kind, DslValue::object(plan.entries)).map_err(|error| SnapshotEditError::new("snapshot-edit.schema-invalid", path, error.to_string())))
            .transpose()
            .map(|mutation| mutation.into_iter().collect())
    }

    /// 🗺️ The kind and payload an edit resolves to over the document's value tree, `None` when the edit changes nothing.
    pub fn plan(&self, tree: &DslValue, event: &SnapshotEditEvent) -> Result<Option<EditPlan>, SnapshotEditError> {
        match event {
            SnapshotEditEvent::SetValue { path, value } => self.entity(tree, path, true, |entity, relative| set_value(entity, relative, value.clone())),
            SnapshotEditEvent::InsertValue { path, value } => match self.insert(tree, path, value)? {
                Some(plan) => Ok(Some(plan)),
                None => match self.entity_key(tree, path, Some(value))? {
                    Some(plan) => Ok(Some(plan)),
                    None => self.entity(tree, path, false, |entity, relative| insert_value(entity, relative, value.clone())),
                },
            },
            SnapshotEditEvent::RemoveValue { path } => match self.remove(tree, path)? {
                Some(plan) => Ok(Some(plan)),
                None => match self.entity_key(tree, path, None)? {
                    Some(plan) => Ok(Some(plan)),
                    None => self.entity(tree, path, false, |entity, relative| remove_value(entity, relative).map(|_| ())),
                },
            },
            SnapshotEditEvent::MoveValue { from, path } => self.entity_move(tree, from, path),
            SnapshotEditEvent::RenameKey { path, key } => self.entity(tree, path, false, |entity, relative| rename_key(entity, relative, key)),
            SnapshotEditEvent::ReplaceSource { .. } => Err(SnapshotEditError::new("snapshot-edit.unsupported-path", "", "replacing the whole source is a document load, not an edit")),
        }
    }

    fn insert(&self, tree: &DslValue, path: &str, item: &DslValue) -> Result<Option<EditPlan>, SnapshotEditError> {
        let segments = decode_pointer(path)?;
        let Some((last, parent)) = segments.split_last() else { return Ok(None) };
        for rule in self.inserts {
            let template = template_segments(rule.path);
            if template.len() != parent.len() {
                continue;
            }
            let Some(captures) = match_prefix(&template, parent, tree, path)? else { continue };
            let DslValue::Array(items) = value_at(tree, parent, path)? else { continue };
            let position = array_index(last, items.len(), path, true)?;
            let mut entries = selector_entries(rule.selectors, &captures, path)?;
            if let Some(index) = rule.index {
                entries.push((index.to_string(), DslValue::uint(position as u64)));
            }
            if !rule.item.is_empty() {
                entries.push((rule.item.to_string(), item.clone()));
            }
            for lifted in rule.item_fields {
                entries.push((lifted.payload.to_string(), row_field(item, lifted.field, path)?));
            }
            entries.extend(carried_entries(rule.with, &captures, tree, path)?);
            return Ok(Some(EditPlan { kind: rule.kind, entries }));
        }
        Ok(None)
    }

    fn remove(&self, tree: &DslValue, path: &str) -> Result<Option<EditPlan>, SnapshotEditError> {
        let segments = decode_pointer(path)?;
        let Some((last, parent)) = segments.split_last() else { return Ok(None) };
        for rule in self.removes {
            let template = template_segments(rule.path);
            if template.len() != parent.len() {
                continue;
            }
            let Some(captures) = match_prefix(&template, parent, tree, path)? else { continue };
            let DslValue::Array(items) = value_at(tree, parent, path)? else { continue };
            let position = array_index(last, items.len(), path, false)?;
            let mut entries = selector_entries(rule.selectors, &captures, path)?;
            match rule.row {
                RowKey::Index(payload) => entries.push((payload.to_string(), DslValue::uint(position as u64))),
                RowKey::Field { payload, field } => entries.push((payload.to_string(), row_field(&items[position], field, path)?)),
            }
            entries.extend(carried_entries(rule.with, &captures, tree, path)?);
            return Ok(Some(EditPlan { kind: rule.kind, entries }));
        }
        Ok(None)
    }

    /// 🔎 The entity rule governing `segments`: the one with the longest template that is a prefix (`inclusive` admits the entity itself).
    fn governing<'a>(&self, tree: &'a DslValue, segments: &[String], inclusive: bool, path: &str) -> Result<Option<(&EntityRule, usize, Vec<Capture<'a>>)>, SnapshotEditError> {
        let mut rules: Vec<&EntityRule> = self.entities.iter().collect();
        rules.sort_by_key(|rule| std::cmp::Reverse(template_segments(rule.path).len()));
        for rule in rules {
            let template = template_segments(rule.path);
            if template.len() > segments.len() || (!inclusive && template.len() == segments.len()) {
                continue;
            }
            if let Some(captures) = match_prefix(&template, segments, tree, path)? {
                return Ok(Some((rule, template.len(), captures)));
            }
        }
        Ok(None)
    }

    /// 🗝️ An optional member that is itself an entity: inserting its key sets the entity to the inserted value, removing it sets the entity to null
    /// (an absent optional decodes as `None`). `None` when the path is no entity key of the document.
    fn entity_key(&self, tree: &DslValue, path: &str, inserted: Option<&DslValue>) -> Result<Option<EditPlan>, SnapshotEditError> {
        let segments = decode_pointer(path)?;
        let Some((last, parent)) = segments.split_last() else { return Ok(None) };
        let Some((rule, depth, captures)) = self.governing(tree, &segments, true, path)? else { return Ok(None) };
        if depth != segments.len() {
            return Ok(None);
        }
        let DslValue::Object(members) = value_at(tree, parent, path)? else { return Ok(None) };
        let present = members.iter().any(|(key, _)| key == last);
        if inserted.is_some() == present {
            return Ok(None);
        }
        let mut entries = selector_entries(rule.selectors, &captures, path)?;
        entries.push((rule.value.to_string(), inserted.cloned().unwrap_or(DslValue::Null)));
        entries.extend(carried_entries(rule.with, &captures, tree, path)?);
        Ok(Some(EditPlan { kind: rule.kind, entries }))
    }

    fn entity(&self, tree: &DslValue, path: &str, inclusive: bool, edit: impl FnOnce(&mut DslValue, &str) -> Result<(), SnapshotEditError>) -> Result<Option<EditPlan>, SnapshotEditError> {
        let segments = decode_pointer(path)?;
        let Some((rule, depth, captures)) = self.governing(tree, &segments, inclusive, path)? else { return Err(unsupported(path)) };
        let before = value_at(tree, &segments[..depth], path)?;
        let mut entity = before.clone();
        expand_along(&mut entity, &segments[depth..]);
        edit(&mut entity, &relative_pointer(&segments[depth..]))?;
        let entity = restore_bytes(before, entity);
        if values_equivalent(before, &entity) {
            return Ok(None);
        }
        let mut entries = selector_entries(rule.selectors, &captures, path)?;
        entries.push((rule.value.to_string(), entity));
        entries.extend(carried_entries(rule.with, &captures, tree, path)?);
        Ok(Some(EditPlan { kind: rule.kind, entries }))
    }

    fn entity_move(&self, tree: &DslValue, from: &str, path: &str) -> Result<Option<EditPlan>, SnapshotEditError> {
        let origin = decode_pointer(from)?;
        let destination = decode_pointer(path)?;
        let Some((rule, depth, captures)) = self.governing(tree, &destination, false, path)? else { return Err(unsupported(path)) };
        if origin.len() <= depth || origin[..depth] != destination[..depth] {
            return Err(SnapshotEditError::new("snapshot-edit.unsupported-path", path, format!("no kind moves '{from}' to '{path}': the move leaves its entity")));
        }
        let before = value_at(tree, &destination[..depth], path)?;
        let mut entity = before.clone();
        expand_along(&mut entity, &origin[depth..]);
        expand_along(&mut entity, &destination[depth..]);
        move_value(&mut entity, &relative_pointer(&origin[depth..]), &relative_pointer(&destination[depth..]))?;
        let entity = restore_bytes(before, entity);
        if values_equivalent(before, &entity) {
            return Ok(None);
        }
        let mut entries = selector_entries(rule.selectors, &captures, path)?;
        entries.push((rule.value.to_string(), entity));
        entries.extend(carried_entries(rule.with, &captures, tree, path)?);
        Ok(Some(EditPlan { kind: rule.kind, entries }))
    }
}

/// 🧩️ The new value of the subtree held at `prefix` after `event` edits somewhere inside it: only that subtree is cloned and edited, so a
/// computed gesture can build its kind's payload from a field edit without rebuilding the document. The event must address the subtree
/// itself or something below it.
pub fn edited_subtree(subtree: &DslValue, prefix: &str, event: &SnapshotEditEvent) -> Result<DslValue, SnapshotEditError> {
    let origin = decode_pointer(prefix)?;
    let inside = |path: &str| -> Result<String, SnapshotEditError> {
        let segments = decode_pointer(path)?;
        if segments.len() < origin.len() || segments[..origin.len()] != origin[..] {
            return Err(SnapshotEditError::new("snapshot-edit.unsupported-path", path, format!("the edit leaves '{prefix}'")));
        }
        Ok(relative_pointer(&segments[origin.len()..]))
    };
    let mut value = subtree.clone();
    let below = |path: &str| -> Result<Vec<String>, SnapshotEditError> { Ok(decode_pointer(path)?.into_iter().skip(origin.len()).collect()) };
    match event {
        SnapshotEditEvent::SetValue { path, .. } | SnapshotEditEvent::InsertValue { path, .. } | SnapshotEditEvent::RemoveValue { path } | SnapshotEditEvent::RenameKey { path, .. } => expand_along(&mut value, &below(path)?),
        SnapshotEditEvent::MoveValue { from, path } => {
            expand_along(&mut value, &below(from)?);
            expand_along(&mut value, &below(path)?);
        }
        SnapshotEditEvent::ReplaceSource { .. } => {}
    }
    match event {
        SnapshotEditEvent::SetValue { path, value: next } => set_value(&mut value, &inside(path)?, next.clone())?,
        SnapshotEditEvent::InsertValue { path, value: next } => insert_value(&mut value, &inside(path)?, next.clone())?,
        SnapshotEditEvent::RemoveValue { path } => {
            remove_value(&mut value, &inside(path)?)?;
        }
        SnapshotEditEvent::MoveValue { from, path } => move_value(&mut value, &inside(from)?, &inside(path)?)?,
        SnapshotEditEvent::RenameKey { path, key } => rename_key(&mut value, &inside(path)?, key)?,
        SnapshotEditEvent::ReplaceSource { .. } => return Err(SnapshotEditError::new("snapshot-edit.unsupported-path", prefix, "replacing the whole source is a document load, not an edit")),
    }
    Ok(restore_bytes(subtree, value))
}

/// 🔢️ Opens the octet strings the edit descends into as lists of numbers (an octet string is addressable by position); only the nodes on `segments` are opened.
fn expand_along(node: &mut DslValue, segments: &[String]) {
    let Some((head, rest)) = segments.split_first() else { return };
    if let DslValue::Bytes(bytes) = node {
        *node = DslValue::Array(bytes.iter().map(|byte| DslValue::uint(u64::from(*byte))).collect());
    }
    match node {
        DslValue::Object(entries) => {
            if let Some((_, child)) = entries.iter_mut().find(|(key, _)| key == head) {
                expand_along(child, rest);
            }
        }
        DslValue::Array(items) => {
            if let Some(child) = head.parse::<usize>().ok().and_then(|index| items.get_mut(index)) {
                expand_along(child, rest);
            }
        }
        _ => {}
    }
}

/// 🔢️ Closes the lists of octets back into octet strings wherever `before` held one, so the payload carries the field's own representation.
fn restore_bytes(before: &DslValue, after: DslValue) -> DslValue {
    match (before, after) {
        (DslValue::Bytes(_), DslValue::Array(items)) => {
            let octets: Option<Vec<u8>> = items.iter().map(|item| if let DslValue::Number(number) = item { number.as_u64().and_then(|value| u8::try_from(value).ok()) } else { None }).collect();
            octets.map_or(DslValue::Array(items), DslValue::Bytes)
        }
        (DslValue::Object(previous), DslValue::Object(entries)) => DslValue::Object(entries.into_iter().map(|(key, value)| match previous.iter().find(|(other, _)| *other == key) { Some((_, old)) => (key, restore_bytes(old, value)), None => (key, value) }).collect()),
        (DslValue::Array(previous), DslValue::Array(items)) => DslValue::Array(items.into_iter().enumerate().map(|(index, value)| match previous.get(index) { Some(old) => restore_bytes(old, value), None => value }).collect()),
        (_, after) => after,
    }
}

fn event_path(event: &SnapshotEditEvent) -> &str {
    match event {
        SnapshotEditEvent::SetValue { path, .. } | SnapshotEditEvent::InsertValue { path, .. } | SnapshotEditEvent::RemoveValue { path } | SnapshotEditEvent::MoveValue { path, .. } | SnapshotEditEvent::RenameKey { path, .. } => path,
        SnapshotEditEvent::ReplaceSource { .. } => "",
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
