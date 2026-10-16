//! 🔢️ The draft editor input rows of the history panel: RFC 6901 pointer reads and edits of the draft value, variant switching, snaps, options, coercion, selection values and entity names.

use semio_framework::{ActionArgDef, ActionArgOption, ArgPresentation, ArgSchema, FaultFrom, OptionSource, SnapSource, input_label_glossary, reference_id_text, registered_input_schema_document};
use semio_framework_plugin_decl_app::ArtifactApp;
#[cfg(test)]
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
#[cfg(test)]
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use pack::snapshot::ArtifactPack;
use std::collections::{BTreeMap, BTreeSet};
use semio_framework_value::DslValue;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_ui_locale::LocalizedLabel;
use store::{Mutation, MutationDiff};
use protocol::{OpBinary, OpText};
use crate::panel_sections::{TIME_TRAVEL_ENTITY_NAME_VISITS, TIME_TRAVEL_SHORT_ID_CHARS};
use semio_framework_plugin_command::lifted_command::{TimeTravelInputRow, TimeTravelList, TimeTravelListItem};
#[cfg(test)]
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_ui_contract::*;

//#region 🔖️Pointer
/// 🔢️ The array index an RFC 6901 reference token names: `0` or a digit run without a leading zero (no sign, no padding),
/// so two pointers never address one item (audit W2A-14).
fn time_travel_pointer_index(segment: &str) -> Option<usize> {
    segment.parse::<usize>().ok().filter(|index| index.to_string() == segment)
}

/// 🧭️ The value at RFC 6901 `pointer` inside `value`.
#[doc(hidden)]
pub fn time_travel_pointer_get<'a>(value: &'a DslValue, pointer: &str) -> Option<&'a DslValue> {
    let Some(rest) = pointer.strip_prefix('/') else { return (pointer.is_empty()).then_some(value) };
    let mut cursor = value;
    for segment in rest.split('/').map(|segment| segment.replace("~1", "/").replace("~0", "~")) {
        cursor = match cursor {
            DslValue::Object(entries) => &entries.iter().find(|(key, _)| *key == segment)?.1,
            DslValue::Array(items) => items.get(time_travel_pointer_index(&segment)?)?,
            _ => return None,
        };
    }
    Some(cursor)
}

/// 🧭️ Replaces the value at RFC 6901 `pointer` inside `value` (an absent last object member or the next array slot is
/// created); `false` when the pointer does not address a slot.
#[doc(hidden)]
pub fn time_travel_pointer_set(value: &mut DslValue, pointer: &str, replacement: DslValue) -> bool {
    let Some(rest) = pointer.strip_prefix('/') else { return false };
    let segments: Vec<String> = rest.split('/').map(|segment| segment.replace("~1", "/").replace("~0", "~")).collect();
    let mut cursor = value;
    for (index, segment) in segments.iter().enumerate() {
        let last = index + 1 == segments.len();
        cursor = match cursor {
            DslValue::Object(entries) => match entries.iter().position(|(key, _)| key == segment) {
                Some(position) => &mut entries[position].1,
                None if last => {
                    entries.push((segment.clone(), DslValue::Null));
                    &mut entries.last_mut().expect("just pushed").1
                }
                None => return false,
            },
            DslValue::Array(items) => {
                let Some(position) = time_travel_pointer_index(segment) else { return false };
                if position == items.len() && last {
                    items.push(DslValue::Null);
                }
                let Some(item) = items.get_mut(position) else { return false };
                item
            }
            _ => return false,
        };
    }
    *cursor = replacement;
    true
}

/// 🪝️ The value at RFC 6901 `pointer` inside `value`, mutably.
fn time_travel_pointer_get_mut<'a>(value: &'a mut DslValue, pointer: &str) -> Option<&'a mut DslValue> {
    let Some(rest) = pointer.strip_prefix('/') else { return pointer.is_empty().then_some(value) };
    let mut cursor = value;
    for segment in rest.split('/').map(|segment| segment.replace("~1", "/").replace("~0", "~")) {
        cursor = match cursor {
            DslValue::Object(entries) => &mut entries.iter_mut().find(|(key, _)| *key == segment)?.1,
            DslValue::Array(items) => items.get_mut(time_travel_pointer_index(&segment)?)?,
            _ => return None,
        };
    }
    Some(cursor)
}

/// ➕️ Inserts `item` into the array RFC 6901 `pointer` ends in: before the item its last segment indexes, or after the last
/// one for `-` (or the array's length); `false` when the pointer names no array slot.
#[doc(hidden)]
pub fn time_travel_pointer_insert(value: &mut DslValue, pointer: &str, item: DslValue) -> bool {
    let Some((array, slot)) = pointer.rsplit_once('/') else { return false };
    let Some(DslValue::Array(items)) = time_travel_pointer_get_mut(value, array) else { return false };
    let Some(position) = (if slot == "-" { Some(items.len()) } else { time_travel_pointer_index(slot).filter(|position| *position <= items.len()) }) else { return false };
    items.insert(position, item);
    true
}

/// ➖️ Removes the array item RFC 6901 `pointer` addresses; `false` when it names no item.
#[doc(hidden)]
pub fn time_travel_pointer_remove(value: &mut DslValue, pointer: &str) -> bool {
    let Some((array, slot)) = pointer.rsplit_once('/') else { return false };
    let Some(DslValue::Array(items)) = time_travel_pointer_get_mut(value, array) else { return false };
    let Some(position) = time_travel_pointer_index(slot).filter(|position| *position < items.len()) else { return false };
    items.remove(position);
    true
}

/// 🌱️ The value a new item of `schema` starts as: a number's lowest admitted value (a soft or hard minimum, an excluded one
/// stepped over, else 0), a choice's first option, an empty text, `false`, a vector of such numbers, an empty reference
/// list (an empty id for one reference), the fewest items a list admits, and an object of its required fields (their
/// declared defaults first); the payload schema then judges it like any drafted value.
#[doc(hidden)]
pub fn time_travel_default_value(schema: &ArgSchema) -> DslValue {
    let number = |min: Option<f64>, max: Option<f64>, exclusive: bool, step: Option<f64>, integer: bool| {
        let low = min.map_or_else(|| max.map_or(0.0, |max| max.min(0.0)), |min| if exclusive { min + step.filter(|step| *step > 0.0).unwrap_or(1.0) } else { min });
        DslValue::json_number(if integer { low.ceil() } else { low })
    };
    match schema {
        ArgSchema::Number { min, min_exclusive, max, step, integer, soft_min, .. } => number(soft_min.or(*min), *max, *min_exclusive && soft_min.is_none(), *step, *integer),
        ArgSchema::String { options, .. } => DslValue::String(options.first().map(|option| option.value.clone()).unwrap_or_default()),
        ArgSchema::Boolean => DslValue::Bool(false),
        ArgSchema::Vector { dims, min, max, .. } => DslValue::Array((0..*dims).map(|_| number(*min, *max, false, None, false)).collect()),
        ArgSchema::Reference { many: true, .. } => DslValue::Array(Vec::new()),
        ArgSchema::Reference { .. } => DslValue::String(String::new()),
        ArgSchema::Array { items, min_items, .. } => DslValue::Array((0..min_items.unwrap_or(0)).map(|_| time_travel_default_value(items)).collect()),
        ArgSchema::Object { fields } => DslValue::Object(fields.iter().filter(|field| field.required).map(|field| (field.key(), field.default.clone().unwrap_or_else(|| time_travel_default_value(&field.schema)))).collect()),
        ArgSchema::Any => DslValue::Null,
    }
}

/// 🔀️ The variant selector of a union payload's inputs: the ungrouped string input whose options name the group of
/// every grouped input (the reader groups each variant's fields by the variant's value); `None` for a payload that is
/// no union, whose `group`s only lay its inputs out.
#[doc(hidden)]
pub fn time_travel_variant_selector(inputs: &[ActionArgDef]) -> Option<&ActionArgDef> {
    let groups: BTreeSet<&str> = inputs.iter().filter_map(|input| input.group.as_deref()).collect();
    if groups.is_empty() {
        return None;
    }
    inputs.iter().filter(|input| input.group.is_none()).find(|input| matches!(&input.schema, ArgSchema::String { options, .. } if groups.iter().all(|group| options.iter().any(|option| option.value == *group))))
}

/// 🔀️ The inputs of the variant `value` is in: the selector, the ungrouped inputs and the active variant's fields; every
/// input of a payload that is no union.
fn time_travel_active_inputs<'a>(inputs: &'a [ActionArgDef], value: &DslValue) -> Vec<&'a ActionArgDef> {
    let Some(selector) = time_travel_variant_selector(inputs) else { return inputs.iter().collect() };
    let active = time_travel_pointer_get(value, &selector.id).and_then(DslValue::as_str);
    inputs.iter().filter(|input| input.group.is_none() || input.group.as_deref() == active).collect()
}

/// 🎛️ The input descriptor RFC 6901 `pointer` addresses in `value`, resolved against the variant `value` is in: object
/// members and array items recurse to any depth (an array item is described by the array's item schema), and a vector
/// or reference list component is one element of its input (`true`).
#[doc(hidden)]
pub fn time_travel_input_at(inputs: &[ActionArgDef], value: &DslValue, pointer: &str) -> Option<(ActionArgDef, bool)> {
    let rest = pointer.strip_prefix('/')?;
    let mut segments = rest.split('/').map(|segment| segment.replace("~1", "/").replace("~0", "~"));
    let first = segments.next()?;
    let mut input = time_travel_active_inputs(inputs, value).into_iter().find(|input| input.key() == first)?.clone();
    while let Some(segment) = segments.next() {
        input = match &input.schema {
            ArgSchema::Object { fields } => fields.iter().find(|field| field.key() == segment)?.clone(),
            ArgSchema::Vector { .. } | ArgSchema::Reference { many: true, .. } => {
                time_travel_pointer_index(&segment)?;
                return segments.next().is_none().then_some((input, true));
            }
            ArgSchema::Array { items, .. } => {
                time_travel_pointer_index(&segment)?;
                ActionArgDef { id: format!("/{segment}"), schema: items.as_ref().clone(), required: true, nullable: false, default: None, ..input.clone() }
            }
            _ => return None,
        };
    }
    Some((input, false))
}

/// 🎛️ The draft editor's rows over `value`: the variant selector and every input of the variant `value` is in, objects
/// flattened into their fields, and a list input (an array of any item kind) as its own row ([`TimeTravelList`], which
/// adds an item) followed by every item `value` holds — an object item as one item row then its fields, any other item as
/// its own row ([`TimeTravelListItem`], which removes the item); a hidden input is never a row.
#[doc(hidden)]
pub fn time_travel_input_rows(inputs: &[ActionArgDef], value: &DslValue) -> Vec<TimeTravelInputRow> {
    fn nested(parent: &LocalizedLabel, child: &LocalizedLabel) -> LocalizedLabel {
        LocalizedLabel::from_fn(|terminology, locale| format!("{} \u{b7} {}", parent.resolve(terminology, locale), child.resolve(terminology, locale)))
    }
    fn numbered(parent: &LocalizedLabel, index: usize) -> LocalizedLabel {
        LocalizedLabel::from_fn(|terminology, locale| format!("{} {}", parent.resolve(terminology, locale), index + 1))
    }
    fn push(input: &ActionArgDef, pointer: String, label: LocalizedLabel, value: &DslValue, item: Option<TimeTravelListItem>, rows: &mut Vec<TimeTravelInputRow>) {
        if input.presentation == Some(ArgPresentation::Hidden) {
            return;
        }
        let current = time_travel_pointer_get(value, &pointer);
        let row = |list: Option<TimeTravelList>| TimeTravelInputRow { pointer: pointer.clone(), label: label.clone(), input: input.clone(), value: current.cloned().unwrap_or(DslValue::Null), list, item };
        match &input.schema {
            ArgSchema::Object { fields } if !fields.is_empty() => {
                if item.is_some() {
                    rows.push(row(None));
                }
                for field in fields {
                    push(field, format!("{pointer}{}", field.id), nested(&label, &field.label), value, None, rows);
                }
            }
            ArgSchema::Array { items, min_items, max_items } => {
                let len = current.and_then(DslValue::as_array).map_or(0, <[DslValue]>::len);
                let count = u32::try_from(len).unwrap_or(u32::MAX);
                rows.push(row(Some(TimeTravelList { len, addable: max_items.is_none_or(|max| count < max), max: *max_items })));
                for index in 0..len {
                    let element = ActionArgDef { id: format!("/{index}"), schema: items.as_ref().clone(), required: true, nullable: false, default: None, ..input.clone() };
                    push(&element, format!("{pointer}/{index}"), numbered(&label, index), value, Some(TimeTravelListItem { index, removable: min_items.is_none_or(|min| count > min), min: *min_items }), rows);
                }
            }
            _ => rows.push(row(None)),
        }
    }
    let mut rows = Vec::new();
    for input in time_travel_active_inputs(inputs, value) {
        push(input, input.id.clone(), input.label.clone(), value, None, &mut rows);
    }
    rows
}

/// 🔀️ `value` switched to union variant `variant` (its selector `selector`): the selector set, every member only other
/// variants declare removed, and every absent field of `variant` filled from its declared default.
#[doc(hidden)]
pub fn time_travel_switch_variant(inputs: &[ActionArgDef], selector: &ActionArgDef, value: &DslValue, variant: &str) -> DslValue {
    let DslValue::Object(entries) = value else { return value.clone() };
    let declared = |key: &str| inputs.iter().any(|input| input.key() == key);
    let kept = |key: &str| inputs.iter().any(|input| input.key() == key && (input.group.is_none() || input.group.as_deref() == Some(variant)));
    let mut entries: Vec<(String, DslValue)> = entries.iter().filter(|(key, _)| kept(key) || !declared(key)).cloned().collect();
    for input in inputs.iter().filter(|input| input.group.as_deref() == Some(variant)) {
        if let Some(default) = input.default.as_ref().filter(|_| !entries.iter().any(|(key, _)| *key == input.key())) {
            entries.push((input.key(), default.clone()));
        }
    }
    let mut switched = DslValue::Object(entries);
    time_travel_pointer_set(&mut switched, &selector.id, DslValue::String(variant.to_string()));
    switched
}

/// 🧲️ Makes every `snapSource` of `rows` concrete: `spacing` answers the grid a `Config` or `Snapshot` source names
/// ([`time_travel_apply_snap_spacing`]).
#[doc(hidden)]
pub fn time_travel_resolve_snaps(rows: &mut [TimeTravelInputRow], mut spacing: impl FnMut(&SnapSource) -> Option<f64>) {
    for row in rows {
        let source = match &row.input.schema {
            ArgSchema::Number { snap_source: Some(source), .. } | ArgSchema::Vector { snap_source: Some(source), .. } => source.clone(),
            _ => continue,
        };
        let resolved = match &source {
            SnapSource::Step => None,
            other => spacing(other),
        };
        time_travel_apply_snap_spacing(&mut row.input.schema, resolved);
    }
}

/// 🗝️ The object of `document` an option source names for `payload`: its template walked segment by segment (RFC 6901
/// escapes), each `{field}` segment taking the payload's top-level member `field` (text, or an integer in decimal) — the key
/// of an object, or, where the walk stands on an array, the record whose own `field` member equals it
/// (`/hostSnapshot/widgets/{id}/params`: the `params` of the widget whose `id` is the payload's `id`). `None` while a member
/// is missing or nothing matches.
#[doc(hidden)]
pub fn time_travel_option_target<'a>(template: &str, payload: &DslValue, document: &'a DslValue) -> Option<&'a DslValue> {
    let text = |value: &DslValue| match value {
        DslValue::String(text) => Some(text.clone()),
        other => other.as_i64().map(|number| number.to_string()).or_else(|| other.as_u64().map(|number| number.to_string())),
    };
    let mut cursor = document;
    for segment in template.strip_prefix('/')?.split('/') {
        let field = segment.strip_prefix('{').and_then(|rest| rest.strip_suffix('}'));
        let key = match field {
            Some(field) => text(payload.get(field)?)?,
            None => segment.replace("~1", "/").replace("~0", "~"),
        };
        cursor = match (cursor, field) {
            (DslValue::Array(records), Some(field)) => records.iter().find(|record| record.get(field).and_then(text).as_deref() == Some(key.as_str()))?,
            (DslValue::Array(items), None) => items.get(time_travel_pointer_index(&key)?)?,
            (DslValue::Object(entries), _) => &entries.iter().find(|(name, _)| *name == key)?.1,
            _ => return None,
        };
    }
    Some(cursor)
}

/// 🏷️ The options an object of the previewed document offers: one per key, in order, labelled by the record's own `label`
/// or `name` (`{en, de}` or text), else the framework input-label glossary, else the key itself.
fn time_travel_document_options(document: &DslValue) -> Vec<ActionArgOption> {
    let DslValue::Object(entries) = document else { return Vec::new() };
    let named = |record: &DslValue| {
        ["label", "name"].iter().filter_map(|field| record.get(field)).find_map(|label| match label {
            DslValue::String(text) if !text.trim().is_empty() => Some(LocalizedLabel::data(text.clone())),
            DslValue::Object(_) => label.get("en").and_then(DslValue::as_str).zip(label.get("de").and_then(DslValue::as_str)).map(|(en, de)| LocalizedLabel::native(en, de)),
            _ => None,
        })
    };
    entries.iter().map(|(key, record)| ActionArgOption { value: key.clone(), label: named(record).or_else(|| input_label_glossary().get(key).cloned()).unwrap_or_else(|| LocalizedLabel::data(key.clone())) }).collect()
}

/// 🗝️ Makes every `optionSource` of `rows` concrete (design §20.11): the options are the keys of the object `document` — the
/// document the editor previews — holds where the source's template leads for the edited `payload`
/// ([`time_travel_option_target`], [`time_travel_document_options`]). A row's own value stays an option when the document lacks it, and the
/// row keeps its `optionSource`, so its JSON schema lists no enum: the fold, not the editor, refuses an unknown key.
#[doc(hidden)]
pub fn time_travel_resolve_options(rows: &mut [TimeTravelInputRow], payload: &DslValue, document: &DslValue) {
    for row in rows {
        let ArgSchema::String { options, option_source: Some(OptionSource::Snapshot { pointer }), .. } = &mut row.input.schema else { continue };
        let pointer = pointer.clone();
        let mut resolved = time_travel_option_target(&pointer, payload, document).map(time_travel_document_options).unwrap_or_default();
        if let Some(current) = row.value.as_str().filter(|current| !current.is_empty() && !resolved.iter().any(|option| option.value == *current)) {
            resolved.push(ActionArgOption { value: current.to_string(), label: LocalizedLabel::data(current) });
        }
        *options = resolved;
    }
}

/// 🔢️ A host value read as a number: numbers as they are, numeric text parsed; an integer input rounds.
fn time_travel_number(value: &DslValue, integer: bool) -> Option<DslValue> {
    let number = value.as_f64().or_else(|| value.as_str().and_then(|text| text.trim().parse::<f64>().ok())).filter(|number| number.is_finite())?;
    Some(DslValue::json_number(if integer { number.round() } else { number }))
}

/// 🎛️ Coerces one host-dispatched control value into the shape `input` declares (an element of it when `element`); `null`
/// clears a nullable input.
#[doc(hidden)]
pub fn time_travel_coerce(input: &ActionArgDef, element: bool, value: &DslValue) -> Option<DslValue> {
    let text = |value: &DslValue| value.as_str().map(|text| DslValue::String(text.to_string()));
    if !element && input.nullable && value.is_null() {
        return Some(DslValue::Null);
    }
    match (&input.schema, element) {
        (ArgSchema::Number { integer, .. }, false) => time_travel_number(value, *integer),
        (ArgSchema::Vector { .. }, true) => time_travel_number(value, false),
        (ArgSchema::Vector { dims, .. }, false) if input.presentation == Some(ArgPresentation::Color) && value.as_str().is_some() => {
            let components = time_travel_color_components(value.as_str()?)?;
            (components.len() == 3 || components.len() == *dims as usize).then(|| DslValue::Array(components.into_iter().map(DslValue::float).collect()))
        }
        (ArgSchema::Vector { dims, .. }, false) => {
            let items = value.as_array()?;
            (items.len() == *dims as usize).then(|| items.iter().map(|item| time_travel_number(item, false)).collect::<Option<Vec<_>>>().map(DslValue::Array)).flatten()
        }
        (ArgSchema::Boolean, false) => value.as_bool().or_else(|| value.as_str().and_then(|text| text.parse().ok())).map(DslValue::Bool),
        (ArgSchema::String { .. }, false) => text(value),
        (ArgSchema::Reference { many: false, id_type, .. }, false) | (ArgSchema::Reference { many: true, id_type, .. }, true) => reference_id_text(value).and_then(|id| id_type.id_value(&id)),
        (ArgSchema::Reference { many: true, id_type, .. }, false) => match value {
            DslValue::Array(items) => items.iter().map(|item| reference_id_text(item).and_then(|id| id_type.id_value(&id))).collect::<Option<Vec<_>>>().map(DslValue::Array),
            other => reference_id_text(other).and_then(|id| id_type.id_value(&id)).map(|id| DslValue::Array(vec![id])),
        },
        _ => Some(value.clone()),
    }
}

/// 🎨️ The sRGB components (`0..=1`) of a colour text by the UI contract's colour law ([`parse_ui_color_hex`]): three
/// for a text without alpha (the draft keeps the stored alpha), four for one with it.
#[doc(hidden)]
pub fn time_travel_color_components(text: &str) -> Option<Vec<f64>> {
    let rgba = parse_ui_color_hex(text)?;
    let digits = text.trim().trim_start_matches('#').len();
    Some(if digits == 4 || digits == 8 { rgba.to_vec() } else { rgba[..3].to_vec() })
}

/// 🎨️ The components of a colour value, as many as it holds.
pub(crate) fn time_travel_color_rgba(value: &DslValue) -> Vec<f64> {
    value.as_array().map(|items| items.iter().map(|item| item.as_f64().unwrap_or(0.0)).collect()).unwrap_or_default()
}

/// 🧲️ Makes a resolved snap `spacing` concrete on one number or vector schema: the multiples of it inside the travel
/// range become detents when at most [`UI_FIXED_LIST_ITEMS`] of them fit, else the spacing becomes the step. A `Step`
/// source is left to the control; a resolved or unresolvable `Config`/`Snapshot` source is cleared, so a host only ever
/// sees numbers.
#[doc(hidden)]
pub fn time_travel_apply_snap_spacing(schema: &mut ArgSchema, spacing: Option<f64>) {
    let (snaps, snap_source, step, low, high) = match schema {
        ArgSchema::Number { snaps, snap_source, step, min, max, soft_min, soft_max, .. } => (snaps, snap_source, step, soft_min.or(*min), soft_max.or(*max)),
        ArgSchema::Vector { snaps, snap_source, step, min, max, .. } => (snaps, snap_source, step, *min, *max),
        _ => return,
    };
    if matches!(snap_source, None | Some(SnapSource::Step)) {
        return;
    }
    *snap_source = None;
    let Some(spacing) = spacing.filter(|spacing| spacing.is_finite() && *spacing > 0.0) else { return };
    let detents = low
        .zip(high)
        .filter(|(low, high)| low <= high && ((high - low) / spacing).floor() < UI_FIXED_LIST_ITEMS as f64)
        .map(|(low, high)| ((low / spacing).ceil() as i64..=(high / spacing).floor() as i64).map(|index| index as f64 * spacing).collect::<Vec<_>>());
    match detents {
        Some(detents) => {
            snaps.extend(detents);
            snaps.sort_by(f64::total_cmp);
            snaps.dedup();
            snaps.truncate(UI_FIXED_LIST_ITEMS);
        }
        None => *step = Some(spacing),
    }
}

/// 🔗️ The JSON texts of every document `schema_json` references by `$id` (transitively), for the payload validator.
#[doc(hidden)]
pub fn time_travel_schema_documents(schema_json: &str) -> Vec<String> {
    fn references(value: &DslValue, into: &mut Vec<String>) {
        match value {
            DslValue::Object(entries) => {
                for (key, value) in entries {
                    match (key.as_str(), value) {
                        ("$ref", DslValue::String(reference)) if !reference.starts_with('#') => into.push(reference.split('#').next().unwrap_or_default().to_string()),
                        _ => references(value, into),
                    }
                }
            }
            DslValue::Array(items) => items.iter().for_each(|item| references(item, into)),
            _ => {}
        }
    }
    let Ok(root) = semio_framework_pack_json::parse(schema_json, semio_framework_pack_json::JsonMemberPolicy::Reject) else { return Vec::new() };
    let mut pending = Vec::new();
    references(&semio_framework_pack_json::to_dsl_value(&root), &mut pending);
    let mut seen = BTreeSet::new();
    let mut documents = Vec::new();
    while let Some(id) = pending.pop() {
        if id.is_empty() || !seen.insert(id.clone()) {
            continue;
        }
        let Some(document) = registered_input_schema_document(&id) else { continue };
        references(&document, &mut pending);
        documents.push(semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&document)));
    }
    documents
}

/// 🧾️ The JSON text of `value`.
#[doc(hidden)]
pub fn time_travel_json(value: &DslValue) -> String {
    semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(value))
}

/// 🔎️ The names the document value gives the entities `ids` — the generic default behind reference chips (gap N3): the
/// first object at any depth whose `id` is one of `ids` is named by its first `label`, `name`, `title` or `text` that is a
/// non-empty string (data in every locale) or an `{en, de}` pair. The walk visits at most
/// [`TIME_TRAVEL_ENTITY_NAME_VISITS`] values and stops once every id is named.
pub fn time_travel_entity_names(document: &DslValue, ids: &BTreeSet<&str>) -> BTreeMap<String, LocalizedLabel> {
    let text = |value: &DslValue| value.as_str().map(str::trim).filter(|text| !text.is_empty()).map(str::to_string);
    let name = |value: &DslValue| match value {
        DslValue::Object(fields) => {
            let locale = |key: &str| fields.iter().find(|(field, _)| field == key).and_then(|(_, value)| text(value));
            Some(LocalizedLabel::native(&locale("en")?, &locale("de")?))
        }
        other => text(other).map(LocalizedLabel::data),
    };
    let mut names = BTreeMap::new();
    let mut stack = vec![document];
    let mut visits = 0;
    while let Some(value) = stack.pop() {
        visits += 1;
        if visits > TIME_TRAVEL_ENTITY_NAME_VISITS || names.len() == ids.len() {
            break;
        }
        match value {
            DslValue::Object(fields) => {
                let id = fields.iter().find(|(key, _)| key == "id").and_then(|(_, id)| id.as_str()).filter(|id| ids.contains(id) && !names.contains_key(*id));
                if let Some((id, label)) = id.and_then(|id| Some((id, ["label", "name", "title", "text"].iter().find_map(|key| fields.iter().find(|(field, _)| field == key).and_then(|(_, value)| name(value)))?))) {
                    names.insert(id.to_string(), label);
                }
                stack.extend(fields.iter().rev().map(|(_, value)| value));
            }
            DslValue::Array(items) => stack.extend(items.iter().rev()),
            _ => {}
        }
    }
    names
}

/// 🏷️ The chip of an entity `id` that neither the app nor the document names (gap N3): the reference's first declared kind
/// as a word — its last segment through the framework input-label glossary in every locale, else that segment spelled out —
/// then a short id (the id before any `!` qualifier, cut to [`TIME_TRAVEL_SHORT_ID_CHARS`] characters). Without a declared
/// kind the short id alone.
pub fn time_travel_reference_fallback_label(kinds: &[String], id: &str) -> LocalizedLabel {
    let bare = id.split('!').next().unwrap_or(id);
    let short = if bare.chars().count() > TIME_TRAVEL_SHORT_ID_CHARS { format!("{}\u{2026}", bare.chars().take(TIME_TRAVEL_SHORT_ID_CHARS).collect::<String>()) } else { bare.to_string() };
    let Some(segment) = kinds.first().and_then(|kind| kind.split('@').next()).and_then(|kind| kind.rsplit(['.', ':', '/', '#']).next()).filter(|segment| !segment.is_empty()) else {
        return LocalizedLabel::data(short);
    };
    let word = input_label_glossary().get(segment).cloned().unwrap_or_else(|| LocalizedLabel::data(time_travel_spelled_out(segment)));
    LocalizedLabel::from_fn(move |terminology, locale| format!("{} {short}", word.resolve(terminology, locale)))
}

/// 🔤️ An identifier segment as words with a leading capital: `targetRegion`, `target-region` and `target_region` read
/// "Target region".
fn time_travel_spelled_out(segment: &str) -> String {
    let mut words = String::new();
    for (index, character) in segment.chars().enumerate() {
        match character {
            '-' | '_' => words.push(' '),
            upper if upper.is_uppercase() && index > 0 => words.extend([' '].into_iter().chain(upper.to_lowercase())),
            first if index == 0 => words.extend(first.to_uppercase()),
            other => words.push(other),
        }
    }
    words
}
//#endregion 🔖️Pointer
