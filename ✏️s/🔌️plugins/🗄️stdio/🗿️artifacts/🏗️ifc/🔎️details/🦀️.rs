//! 🧭️ Catalog-profile declaration-aware controls over the shared lossless IFC graph.

use semio_s_artifact_stdio_ifc_declarations::{IfcAttributeDeclaration, IfcCatalog, IfcTypeExpression};
use semio_framework_plugin::Fault;
use semio_framework_ui_locale::Locale;
use semio_framework_value::{DslValue, FromValue, RetainedWorkOutcome, ToValue};
use semio_s_artifact_stdio_contract::editing;
use semio_s_artifact_stdio_contract::editing::{DslSnapshotDetailsProvider, SnapshotDetailItemCapabilities, SnapshotDetailPathSegment as P, SnapshotDetailPresentation, SnapshotDetailValue, SnapshotDetailVariant, SnapshotDetailsProvider};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{ifc_arguments_identical, ifc_complex_identical, IfcComplexType, IfcEntity, IfcSnapshot, IfcTypedValue, IfcValue};
use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::{edit_rules, IfcMutation};
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;

pub use semio_s_artifact_stdio_ifc_details_provider::*;
pub use semio_s_artifact_stdio_ifc_details_guidance::*;
#[path="admission.rs"]pub mod admission;
#[path="command_wire.rs"]pub mod command_wire;
#[path="entity_remove.rs"]pub mod entity_remove;
#[path="work.rs"]pub mod work;
pub use work::IfcDetailsWork;

pub fn details_work_capacity<C: IfcDetailsProfile>() -> semio_framework_plugin::retained_command::ArtifactRetainedWorkCapacity {
    let attributes = C::MAXIMUM_ATTRIBUTES;
    semio_framework_plugin::retained_command::ArtifactRetainedWorkCapacity::for_invertible_items(attributes.saturating_mul(2).saturating_add(3))
}

fn value_error(message: &str) -> Fault {
    Fault::from(message.to_owned())
}

fn native_value_mut<'a>(value: &'a mut IfcValue, path: &[String]) -> Result<&'a mut IfcValue, Fault> {
    if path.is_empty() {
        return Ok(value);
    }
    if path[0] != "value" {
        return Err(value_error("IFC edit does not address a native value"));
    }
    match value {
        IfcValue::Aggregate(items) if path.len() >= 2 => {
            let index = path[1].parse::<usize>().map_err(|_| value_error("Invalid IFC aggregate index"))?;
            native_value_mut(items.get_mut(index).ok_or_else(|| value_error("IFC aggregate index is out of bounds"))?, &path[2..])
        }
        IfcValue::TypedValue(typed) if path.len() >= 3 && path[1] == "items" => {
            let index = path[2].parse::<usize>().map_err(|_| value_error("Invalid IFC typed item index"))?;
            native_value_mut(typed.items.get_mut(index).ok_or_else(|| value_error("IFC typed item index is out of bounds"))?, &path[3..])
        }
        _ => Err(value_error("IFC nested edit requires an aggregate or typed item")),
    }
}

fn native_collection_mut<'a>(value: &'a mut IfcValue, path: &[String]) -> Result<&'a mut Vec<IfcValue>, Fault> {
    if let Some((last, parent)) = path.split_last() {
        if last == "value" {
            if let IfcValue::Aggregate(items) = native_value_mut(value, parent)? {
                return Ok(items);
            }
        } else if last == "items" && parent.last().is_some_and(|field| field == "value") {
            if let IfcValue::TypedValue(typed) = native_value_mut(value, &parent[..parent.len() - 1])? {
                return Ok(&mut typed.items);
            }
        }
    }
    Err(value_error("IFC collection edit requires an aggregate or typed items"))
}

fn native_argument_edit(value: &mut IfcValue, event: &editing::SnapshotEditEvent, prefix: &str, garbage: &mut Vec<IfcValue>, tick: &mut dyn FnMut() -> Result<(), String>) -> Result<(), Fault> {
    tick().map_err(Fault::from)?;
    let relative = |pointer: &str| -> Result<Vec<String>, Fault> {
        let suffix = pointer.strip_prefix(prefix).filter(|suffix| suffix.is_empty() || suffix.starts_with('/')).ok_or_else(|| value_error("IFC move leaves its owning argument"))?;
        editing::decode_pointer(suffix).map_err(|error| Fault::from(error.to_string()))
    };
    match event {
        editing::SnapshotEditEvent::SetValue { path, value: next } => {
            let path = relative(path)?;
            if path.last().is_some_and(|field| field == "value") {
                let target = native_value_mut(value, &path[..path.len() - 1])?;
                let replacement = match target {
                    IfcValue::String(_) | IfcValue::Enum(_) | IfcValue::Binary(_) => {
                        let DslValue::String(text) = next else { return Err(value_error("IFC text payload requires text")) };
                        match target {
                            IfcValue::String(_) => IfcValue::String(text.clone()),
                            IfcValue::Enum(_) => IfcValue::Enum(text.clone()),
                            _ => IfcValue::Binary(text.clone()),
                        }
                    }
                    IfcValue::Integer(_) => {
                        let DslValue::Number(number) = next else { return Err(value_error("IFC INTEGER requires a number")) };
                        IfcValue::Integer(number.as_i64().ok_or_else(|| value_error("IFC INTEGER is outside i64"))?)
                    }
                    IfcValue::Real(_) => {
                        let DslValue::Number(number) = next else { return Err(value_error("IFC REAL requires a number")) };
                        IfcValue::Real(number.as_f64())
                    }
                    IfcValue::Reference(_) => {
                        let DslValue::Number(number) = next else { return Err(value_error("IFC reference requires an exact integer")) };
                        IfcValue::Reference(number.as_u64().ok_or_else(|| value_error("IFC reference is outside u64"))?)
                    }
                    IfcValue::Aggregate(_) => IfcValue::Aggregate(Vec::<IfcValue>::from_value(next.clone()).map_err(|error| Fault::from(error.to_string()))?),
                    IfcValue::TypedValue(_) => IfcValue::TypedValue(IfcTypedValue::from_value(next.clone()).map_err(|error| Fault::from(error.to_string()))?),
                    _ => return Err(value_error("IFC marker has no payload")),
                };
                garbage.push(std::mem::replace(target, replacement));
            } else if path.last().is_some_and(|field| field == "name") && path.len() >= 2 && path[path.len() - 2] == "value" {
                let IfcValue::TypedValue(typed) = native_value_mut(value, &path[..path.len() - 2])? else { return Err(value_error("IFC type name requires a typed value")) };
                let DslValue::String(name) = next else { return Err(value_error("IFC type name requires text")) };
                garbage.push(IfcValue::String(std::mem::replace(&mut typed.name, name.clone())));
            } else {
                let target = native_value_mut(value, &path)?;
                garbage.push(std::mem::replace(target, IfcValue::from_value(next.clone()).map_err(|error| Fault::from(error.to_string()))?));
            }
        }
        editing::SnapshotEditEvent::InsertValue { path, value: next } => {
            let path = relative(path)?;
            let (last, parent) = path.split_last().ok_or_else(|| value_error("IFC insert requires an item path"))?;
            let items = native_collection_mut(value, parent)?;
            let index = if last == "-" { items.len() } else { last.parse::<usize>().map_err(|_| value_error("Invalid IFC insertion index"))? };
            if index > items.len() {
                return Err(value_error("IFC insertion index is out of bounds"));
            }
            items.insert(index, IfcValue::from_value(next.clone()).map_err(|error| Fault::from(error.to_string()))?);
        }
        editing::SnapshotEditEvent::RemoveValue { path } => {
            let path = relative(path)?;
            let (last, parent) = path.split_last().ok_or_else(|| value_error("IFC removal requires an item path"))?;
            let items = native_collection_mut(value, parent)?;
            let index = last.parse::<usize>().map_err(|_| value_error("Invalid IFC removal index"))?;
            if index >= items.len() {
                return Err(value_error("IFC removal index is out of bounds"));
            }
            garbage.push(items.remove(index));
        }
        editing::SnapshotEditEvent::MoveValue { from, path } => {
            let from = relative(from)?;
            let to = relative(path)?;
            let (start, parent) = from.split_last().ok_or_else(|| value_error("IFC move requires an item path"))?;
            let (end, destination) = to.split_last().ok_or_else(|| value_error("IFC move requires an item path"))?;
            if parent != destination {
                return Err(value_error("IFC move must remain in its owning collection"));
            }
            let items = native_collection_mut(value, parent)?;
            let start = start.parse::<usize>().map_err(|_| value_error("Invalid IFC move origin"))?;
            let end = if end == "-" { items.len().saturating_sub(1) } else { end.parse::<usize>().map_err(|_| value_error("Invalid IFC move destination"))? };
            if start >= items.len() || end >= items.len() {
                return Err(value_error("IFC move index is out of bounds"));
            }
            if start < end {
                items[start..=end].rotate_left(1);
            } else {
                items[end..=start].rotate_right(1);
            }
        }
        _ => return Err(value_error("IFC native value fields cannot be renamed")),
    }
    Ok(())
}

fn retype_arguments<C: IfcDetailsProfile>(
    provider: &IfcDetailsProvider<'_, C>,
    attributes: &[IfcAttributeDeclaration],
    previous: &[IfcAttributeDeclaration],
    args: &mut Vec<IfcValue>,
    garbage: &mut Vec<IfcValue>,
    derived: &std::collections::BTreeSet<(&str, &str)>,
    tick: &mut dyn FnMut() -> Result<(), String>,
) -> Result<Vec<bool>, Fault> {
    let displaced = garbage.len();
    garbage.push(IfcValue::Aggregate(std::mem::take(args)));
    let IfcValue::Aggregate(old) = &mut garbage[displaced] else { unreachable!() };
    let mut changed = Vec::with_capacity(attributes.len());
    for (index, attribute) in attributes.iter().enumerate() {
        tick().map_err(Fault::from)?;
        let previous_index = previous.iter().position(|candidate| candidate.name == attribute.name && candidate.type_expr == attribute.type_expr);
        let effective_derived = attribute.derived || derived.contains(&(attribute.declared_by.as_str(), attribute.name.as_str()));
        if effective_derived {
            changed.push(previous_index != Some(index) || !matches!(old.get(index), Some(IfcValue::Derived)));
            args.push(IfcValue::Derived);
        } else if let Some(previous) = previous_index.filter(|position| old.get(*position).is_some_and(|value| !matches!(value, IfcValue::Derived) && (!matches!(value, IfcValue::Unset) || attribute.optional))) {
            changed.push(previous != index);
            args.push(std::mem::replace(&mut old[previous], IfcValue::Unset));
        } else {
            changed.push(true);
            args.push(if attribute.optional {
                IfcValue::Unset
            } else if attribute.name == "GlobalId" {
                IfcValue::String(String::new())
            } else {
                provider.template(&attribute.type_expr, 0).unwrap_or(IfcValue::Unset)
            });
        }
    }
    Ok(changed)
}

fn owned_entity_edit<C: IfcDetailsProfile>(
    event: &editing::SnapshotEditEvent,
    snapshot: &IfcSnapshot,
    position: usize,
    entity: &mut IfcEntity,
    garbage: &mut Vec<IfcValue>,
    mutations: &mut Vec<IfcMutation>,
    tick: &mut dyn FnMut() -> Result<(), String>,
) -> Result<(), Fault> {
    tick().map_err(Fault::from)?;
    use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::{
        insert_entity_arg::InsertEntityArg, remove_entity_arg::RemoveEntityArg, set_entity_arg::SetEntityArg, set_entity_complex::SetEntityComplex, set_entity_name::SetEntityName,
    };
    let path = match event {
        editing::SnapshotEditEvent::SetValue { path, .. }
        | editing::SnapshotEditEvent::InsertValue { path, .. }
        | editing::SnapshotEditEvent::RemoveValue { path }
        | editing::SnapshotEditEvent::MoveValue { path, .. }
        | editing::SnapshotEditEvent::RenameKey { path, .. } => path,
        _ => return Err(value_error("IFC entity edit requires a field path")),
    };
    let segments = editing::decode_pointer(path).map_err(|error| Fault::from(error.to_string()))?;
    let provider = IfcDetailsProvider::<C>::new(snapshot);
    let original = &snapshot.entities[position];
    let mut complex_changed = false;
    if let editing::SnapshotEditEvent::SetValue { value, .. } = event {
        let root_type = (segments.len() == 2).then(|| requested_type(value)).flatten().or_else(|| {
            if segments.len() == 3 && segments[2] == "name" {
                if let DslValue::String(name) = value {
                    Some(name.as_str())
                } else {
                    None
                }
            } else {
                None
            }
        });
        if let Some(name) = root_type {
            let name = name.to_ascii_uppercase();
            let declaration = C::catalog().entities.get(&name).ok_or_else(|| value_error("Unknown IFC entity type"))?;
            let partial = !entity.complex.is_empty();
            if declaration.abstract_entity && !partial {
                return Err(value_error("Abstract IFC entity requires a complex instance"));
            }
            let previous = C::catalog().entities.get(&entity.name.to_ascii_uppercase()).ok_or_else(|| value_error("Unknown previous IFC entity type"))?;
            let attributes = if partial { declaration.attributes.as_slice() } else { declaration.all_attributes.as_slice() };
            let previous = if partial { previous.attributes.as_slice() } else { previous.all_attributes.as_slice() };
            let derived = derived_keys::<C>(&name, &entity.complex);
            let mut changed = retype_arguments(&provider, attributes, previous, &mut entity.args, garbage, &derived, tick)?;
            for (index, attribute) in attributes.iter().enumerate() {
                if attribute.name == "GlobalId" && !matches!(entity.args.get(index), Some(IfcValue::String(value)) if !value.is_empty()) {
                    changed[index] = true;
                }
            }
            populate_global_ids(snapshot, entity.id, attributes, &mut entity.args, tick)?;
            garbage.push(IfcValue::String(std::mem::replace(&mut entity.name, name)));
            if entity.name != original.name {
                mutations.push(IfcMutation::SetEntityName(SetEntityName { id: entity.id, name: entity.name.clone() }));
            }
            for index in (entity.args.len()..original.args.len()).rev() {
                tick().map_err(Fault::from)?;
                mutations.push(IfcMutation::RemoveEntityArg(RemoveEntityArg { id: entity.id, index }));
            }
            for (index, changed) in changed.into_iter().enumerate() {
                tick().map_err(Fault::from)?;
                if !changed {
                    continue;
                }
                let value = std::mem::replace(&mut entity.args[index], IfcValue::Unset);
                mutations.push(if index < original.args.len() { IfcMutation::SetEntityArg(SetEntityArg { id: entity.id, index, value }) } else { IfcMutation::InsertEntityArg(InsertEntityArg { id: entity.id, index, value }) });
            }
            for part in &mut entity.complex {
                tick().map_err(Fault::from)?;
                if let Some(declaration) = C::catalog().entities.get(&part.name.to_ascii_uppercase()) {
                    for (attribute, value) in declaration.attributes.iter().zip(&mut part.args) {
                        if let Some(next) = provider.normalized_argument(attribute, value, &derived) {
                            garbage.push(std::mem::replace(value, next));
                            complex_changed = true;
                        }
                    }
                }
            }
            if complex_changed {
                mutations.push(IfcMutation::SetEntityComplex(SetEntityComplex { id: entity.id, complex: std::mem::take(&mut entity.complex) }));
            }
            return Ok(());
        }
        if segments.len() == 2 {
            let next = IfcEntity::from_value(value.clone()).map_err(|error| Fault::from(error.to_string()))?;
            let old = std::mem::replace(entity, next);
            garbage.push(IfcValue::String(old.name));
            garbage.push(IfcValue::Aggregate(old.args));
            for part in old.complex {
                garbage.push(IfcValue::String(part.name));
                garbage.push(IfcValue::Aggregate(part.args));
            }
            entity.id = original.id;
            entity.name.make_ascii_uppercase();
            let declaration = C::catalog().entities.get(&entity.name).ok_or_else(|| value_error("Unknown IFC entity type"))?;
            if declaration.abstract_entity && entity.complex.is_empty() {
                return Err(value_error("Abstract IFC entity requires a complex instance"));
            }
            let attributes = if entity.complex.is_empty() { &declaration.all_attributes } else { &declaration.attributes };
            populate_global_ids(snapshot, entity.id, attributes, &mut entity.args, tick)?;
            let derived = derived_keys::<C>(&entity.name, &entity.complex);
            for (attribute, value) in attributes.iter().zip(&mut entity.args) {
                tick().map_err(Fault::from)?;
                if let Some(next) = provider.normalized_argument(attribute, value, &derived) {
                    garbage.push(std::mem::replace(value, next));
                }
            }
            for part in &mut entity.complex {
                tick().map_err(Fault::from)?;
                part.name.make_ascii_uppercase();
                let declaration = C::catalog().entities.get(&part.name).ok_or_else(|| value_error("Unknown IFC constituent type"))?;
                populate_global_ids(snapshot, entity.id, &declaration.attributes, &mut part.args, tick)?;
                for (attribute, value) in declaration.attributes.iter().zip(&mut part.args) {
                    tick().map_err(Fault::from)?;
                    if let Some(next) = provider.normalized_argument(attribute, value, &derived) {
                        garbage.push(std::mem::replace(value, next));
                    }
                }
            }
            if entity.name != original.name {
                mutations.push(IfcMutation::SetEntityName(SetEntityName { id: entity.id, name: entity.name.clone() }));
            }
            for index in (entity.args.len()..original.args.len()).rev() {
                tick().map_err(Fault::from)?;
                mutations.push(IfcMutation::RemoveEntityArg(RemoveEntityArg { id: entity.id, index }));
            }
            for index in 0..entity.args.len() {
                tick().map_err(Fault::from)?;
                if index >= original.args.len() || !ifc_arguments_identical(&original.args[index..index + 1], &entity.args[index..index + 1]) {
                    let value = std::mem::replace(&mut entity.args[index], IfcValue::Unset);
                    mutations.push(if index < original.args.len() { IfcMutation::SetEntityArg(SetEntityArg { id: entity.id, index, value }) } else { IfcMutation::InsertEntityArg(InsertEntityArg { id: entity.id, index, value }) });
                }
            }
            if !ifc_complex_identical(&original.complex, &entity.complex) {
                mutations.push(IfcMutation::SetEntityComplex(SetEntityComplex { id: entity.id, complex: std::mem::take(&mut entity.complex) }));
            }
            return Ok(());
        }
    }
    if segments.len() >= 4 && segments[2] == "args" {
        let index = if segments[3] == "-" { entity.args.len() } else { segments[3].parse::<usize>().map_err(|_| value_error("Invalid IFC argument index"))? };
        if segments.len() == 4 {
            match event {
                editing::SnapshotEditEvent::InsertValue { value, .. } => {
                    mutations.push(IfcMutation::InsertEntityArg(InsertEntityArg { id: entity.id, index, value: IfcValue::from_value(value.clone()).map_err(|error| Fault::from(error.to_string()))? }));
                    return Ok(());
                }
                editing::SnapshotEditEvent::RemoveValue { .. } => {
                    mutations.push(IfcMutation::RemoveEntityArg(RemoveEntityArg { id: entity.id, index }));
                    return Ok(());
                }
                _ => {}
            }
        }
        let target = entity.args.get_mut(index).ok_or_else(|| value_error("IFC argument does not exist"))?;
        native_argument_edit(target, event, &format!("/entities/{position}/args/{index}"), garbage, tick)?;
        mutations.push(IfcMutation::SetEntityArg(SetEntityArg { id: entity.id, index, value: std::mem::replace(target, IfcValue::Unset) }));
        return Ok(());
    }
    if segments.len() >= 3 && segments[2] == "complex" {
        let keep_part = |part: IfcComplexType, garbage: &mut Vec<IfcValue>| {
            garbage.push(IfcValue::String(part.name));
            garbage.extend(part.args);
        };
        if segments.len() == 3 {
            let replacement = match event {
                editing::SnapshotEditEvent::SetValue { value, .. } | editing::SnapshotEditEvent::InsertValue { value, .. } => Vec::<IfcComplexType>::from_value(value.clone()).map_err(|error| Fault::from(error.to_string()))?,
                editing::SnapshotEditEvent::RemoveValue { .. } => Vec::new(),
                _ => return Err(value_error("IFC complex field requires replacement")),
            };
            for part in std::mem::replace(&mut entity.complex, replacement) {
                keep_part(part, garbage);
            }
        } else {
            let index = if segments[3] == "-" { entity.complex.len() } else { segments[3].parse::<usize>().map_err(|_| value_error("Invalid IFC constituent index"))? };
            if segments.len() == 4 {
                match event {
                    editing::SnapshotEditEvent::InsertValue { value, .. } => {
                        if index > entity.complex.len() {
                            return Err(value_error("IFC constituent insertion is out of bounds"));
                        }
                        entity.complex.insert(index, IfcComplexType::from_value(value.clone()).map_err(|error| Fault::from(error.to_string()))?);
                    }
                    editing::SnapshotEditEvent::RemoveValue { .. } => {
                        if index >= entity.complex.len() {
                            return Err(value_error("IFC constituent removal is out of bounds"));
                        }
                        keep_part(entity.complex.remove(index), garbage);
                    }
                    editing::SnapshotEditEvent::MoveValue { from, .. } => {
                        let origin = editing::decode_pointer(from).map_err(|error| Fault::from(error.to_string()))?;
                        if origin.len() != 4 || origin[..3] != segments[..3] {
                            return Err(value_error("IFC constituent move leaves its owner"));
                        }
                        let start = origin[3].parse::<usize>().map_err(|_| value_error("Invalid IFC constituent origin"))?;
                        let end = index.min(entity.complex.len().saturating_sub(1));
                        if start >= entity.complex.len() || end >= entity.complex.len() {
                            return Err(value_error("IFC constituent move is out of bounds"));
                        }
                        if start < end {
                            entity.complex[start..=end].rotate_left(1);
                        } else {
                            entity.complex[end..=start].rotate_right(1);
                        }
                    }
                    editing::SnapshotEditEvent::SetValue { value, .. } if requested_type(value).is_none() => {
                        let next = IfcComplexType::from_value(value.clone()).map_err(|error| Fault::from(error.to_string()))?;
                        let part = entity.complex.get_mut(index).ok_or_else(|| value_error("IFC constituent does not exist"))?;
                        keep_part(std::mem::replace(part, next), garbage);
                    }
                    _ => {}
                }
            }
            if let editing::SnapshotEditEvent::SetValue { value, .. } = event {
                let type_name = requested_type(value).or_else(|| {
                    if segments.len() == 5 && segments[4] == "name" {
                        if let DslValue::String(name) = value {
                            Some(name.as_str())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                });
                if let Some(name) = type_name {
                    let part = entity.complex.get(index).ok_or_else(|| value_error("IFC constituent does not exist"))?;
                    let previous = C::catalog().entities.get(&part.name.to_ascii_uppercase()).ok_or_else(|| value_error("Unknown previous IFC constituent type"))?;
                    let name = name.to_ascii_uppercase();
                    let declaration = C::catalog().entities.get(&name).ok_or_else(|| value_error("Unknown IFC constituent type"))?;
                    garbage.push(IfcValue::String(std::mem::replace(&mut entity.complex[index].name, name)));
                    let derived = derived_keys::<C>(&entity.name, &entity.complex);
                    retype_arguments(&provider, &declaration.attributes, &previous.attributes, &mut entity.complex[index].args, garbage, &derived, tick)?;
                    populate_global_ids(snapshot, entity.id, &declaration.attributes, &mut entity.complex[index].args, tick)?;
                }
            }
            if segments.len() >= 6 && segments[4] == "args" {
                let part = entity.complex.get_mut(index).ok_or_else(|| value_error("IFC constituent does not exist"))?;
                let argument = if segments[5] == "-" { part.args.len() } else { segments[5].parse::<usize>().map_err(|_| value_error("Invalid IFC partial argument index"))? };
                if segments.len() == 6 && matches!(event, editing::SnapshotEditEvent::InsertValue { .. } | editing::SnapshotEditEvent::RemoveValue { .. }) {
                    match event {
                        editing::SnapshotEditEvent::InsertValue { value, .. } if argument <= part.args.len() => part.args.insert(argument, IfcValue::from_value(value.clone()).map_err(|error| Fault::from(error.to_string()))?),
                        editing::SnapshotEditEvent::RemoveValue { .. } if argument < part.args.len() => garbage.push(part.args.remove(argument)),
                        _ => return Err(value_error("IFC partial argument index is out of bounds")),
                    }
                } else {
                    native_argument_edit(part.args.get_mut(argument).ok_or_else(|| value_error("IFC partial argument does not exist"))?, event, &format!("/entities/{position}/complex/{index}/args/{argument}"), garbage, tick)?;
                }
            }
        }
        let derived = derived_keys::<C>(&entity.name, &entity.complex);
        if let Some(declaration) = C::catalog().entities.get(&entity.name.to_ascii_uppercase()) {
            for (index, (attribute, value)) in declaration.attributes.iter().zip(&original.args).enumerate() {
                if let Some(value) = provider.normalized_argument(attribute, value, &derived) {
                    mutations.push(IfcMutation::SetEntityArg(SetEntityArg { id: entity.id, index, value }));
                }
            }
        }
        for part in &mut entity.complex {
            tick().map_err(Fault::from)?;
            part.name.make_ascii_uppercase();
            if let Some(declaration) = C::catalog().entities.get(&part.name.to_ascii_uppercase()) {
                populate_global_ids(snapshot, entity.id, &declaration.attributes, &mut part.args, tick)?;
                for (attribute, value) in declaration.attributes.iter().zip(&mut part.args) {
                    if let Some(next) = provider.normalized_argument(attribute, value, &derived) {
                        garbage.push(std::mem::replace(value, next));
                    }
                }
            }
        }
        mutations.push(IfcMutation::SetEntityComplex(SetEntityComplex { id: entity.id, complex: std::mem::take(&mut entity.complex) }));
        return Ok(());
    }
    Err(value_error("IFC retained edit requires a native entity, argument or constituent path"))
}


fn reference_id(decimal: &str) -> Result<u64, Fault> {
    if decimal.is_empty() || decimal.len() > 20 || !decimal.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(Fault::from("IFC reference requires an exact positive decimal entity ID"));
    }
    decimal.parse::<u64>().ok().filter(|id| *id > 0).ok_or_else(|| Fault::from("IFC reference is outside the positive u64 domain"))
}

pub fn normalized_reference_edit_controlled<C: IfcDetailsProfile>(event: &editing::SnapshotEditEvent, snapshot: &IfcSnapshot, check_target: bool, tick: &mut dyn FnMut() -> Result<(), String>) -> Result<Option<editing::SnapshotEditEvent>, Fault> {
    tick().map_err(Fault::from)?;
    let editing::SnapshotEditEvent::SetValue { path, value } = event else { return Ok(None) };
    if path.len() > 8192 {
        return Err(Fault::from("IFC edit path exceeds its bounded native pointer extent"));
    }
    let segments = editing::decode_pointer(path).map_err(|error| Fault::from(error.to_string()))?;
    if segments.len() > 128 {
        return Err(Fault::from("IFC edit path exceeds its bounded depth"));
    }
    let detail_path = segments.iter().map(|segment| segment.parse::<usize>().map_or_else(|_| editing::SnapshotDetailPathSegment::Key(segment.clone()), editing::SnapshotDetailPathSegment::Index)).collect::<Vec<_>>();
    let provider = IfcDetailsProvider::<C>::new(snapshot);
    if matches!(detail_path.last(), Some(P::Index(_))) && provider.reference_slot(&detail_path) {
        let DslValue::Object(fields) = value else { return Err(Fault::from("IFC reference slot requires a closed reference or optional unset value")) };
        if fields.len() > 2 { return Err(Fault::from("IFC reference slot has unknown or repeated fields")); }
        let decimal = match value.get("kind").and_then(DslValue::as_str) {
            Some("reference") if fields.len() == 2 => value.get("value").and_then(DslValue::as_u64).ok_or_else(|| Fault::from("IFC reference slot requires an exact positive integer"))?.to_string(),
            Some("unset") if fields.len() == 1 => String::new(),
            _ => return Err(Fault::from("IFC reference slot requires a reference or an optional unset value")),
        };
        let normalized = editing::SnapshotEditEvent::SetValue { path: format!("{path}/reference"), value: DslValue::String(decimal) };
        return normalized_reference_edit_controlled::<C>(&normalized, snapshot, check_target, tick);
    }
    if !provider.is_reference_payload(&detail_path) {
        return Ok(None);
    }
    let virtual_slot = segments.last().is_some_and(|segment| matches!(segment.as_str(), "reference" | "appendReference"));
    let DslValue::String(decimal) = value else {
        return Err(Fault::from("IFC reference input requires an exact decimal entity ID"));
    };
    if segments.last().is_some_and(|segment| segment == "appendReference") {
        let id = reference_id(decimal)?;
        if check_target && !provider.accepts_reference_append_controlled(&detail_path, id, tick).map_err(Fault::from)? {
            return Err(Fault::from("IFC reference insertion requires an existing admitted entity, available collection capacity and a unique target for SET or UNIQUE values"));
        }
        return Ok(Some(editing::SnapshotEditEvent::InsertValue { path: format!("{}/value/-", path.rsplit_once('/').map_or("", |(parent, _)| parent)), value: IfcValue::Reference(id).to_value() }));
    }
    let reference = if virtual_slot && decimal.is_empty() { IfcValue::Unset } else { IfcValue::Reference(reference_id(decimal)?) };
    if check_target && !provider.accepts_reference_controlled(&detail_path, reference.as_reference(), tick).map_err(Fault::from)? {
        return Err(Fault::from("IFC reference requires an existing entity of an admitted EXPRESS type; only optional references can be unset"));
    }
    let normalized = if virtual_slot {
        editing::SnapshotEditEvent::SetValue { path: path.rsplit_once('/').map_or("", |(parent, _)| parent).into(), value: reference.to_value() }
    } else {
        let IfcValue::Reference(id) = reference else { return Err(Fault::from("IFC reference cannot be empty")) };
        editing::SnapshotEditEvent::SetValue { path: path.clone(), value: DslValue::uint(id) }
    };
    Ok(Some(normalized))
}

fn next_entity_id_controlled(snapshot: &IfcSnapshot, tick: &mut dyn FnMut() -> Result<(), String>) -> Result<u64, Fault> {
    let mut occupied = std::collections::BTreeSet::new();
    for entity in &snapshot.entities {
        tick().map_err(Fault::from)?;
        occupied.insert(entity.id);
    }
    if let Some(next) = occupied.last().copied().unwrap_or(0).checked_add(1) {
        return Ok(next.max(1));
    }
    let mut candidate = 1;
    for id in occupied {
        tick().map_err(Fault::from)?;
        if id > candidate {
            return Ok(candidate);
        }
        if id == candidate {
            candidate = candidate.checked_add(1).ok_or_else(|| Fault::from("IFC entity identity space is exhausted"))?;
        }
    }
    Ok(candidate)
}
pub(crate) fn next_entity_id(snapshot: &IfcSnapshot) -> Result<u64, Fault> {
    next_entity_id_controlled(snapshot, &mut || Ok(()))
}

#[path = "../🪪️identity/🦀️.rs"]
mod identity;
pub(crate) fn encode_global_id(value: u128) -> String {
    identity::encode_global_id(value)
}

fn fresh_global_id_controlled(snapshot: &IfcSnapshot, id: u64, tick: &mut dyn FnMut() -> Result<(), String>) -> Result<String, Fault> {
    let mut occupied = std::collections::BTreeSet::new();
    for entity in &snapshot.entities {
        tick().map_err(Fault::from)?;
        if let Some(value) = entity.args.first().and_then(IfcValue::as_str) {
            occupied.insert(value);
        }
        for part in &entity.complex {
            tick().map_err(Fault::from)?;
            if let Some(value) = part.args.first().and_then(IfcValue::as_str) {
                occupied.insert(value);
            }
        }
    }
    loop {
        tick().map_err(Fault::from)?;
        let guid = identity::random_global_id(id);
        if !occupied.contains(guid.as_str()) {
            return Ok(guid);
        }
    }
}

pub(crate) fn fresh_global_id(snapshot: &IfcSnapshot, id: u64) -> String {
    fresh_global_id_controlled(snapshot, id, &mut || Ok(())).expect("unlimited identity generation")
}

fn populate_global_ids(snapshot: &IfcSnapshot, id: u64, attributes: &[IfcAttributeDeclaration], args: &mut [IfcValue], tick: &mut dyn FnMut() -> Result<(), String>) -> Result<(), Fault> {
    for (index, attribute) in attributes.iter().enumerate() {
        tick().map_err(Fault::from)?;
        if attribute.name == "GlobalId" && matches!(args.get(index), Some(IfcValue::String(value)) if value.is_empty()) {
            args[index] = IfcValue::String(fresh_global_id_controlled(snapshot, id, tick)?);
        }
    }
    Ok(())
}

fn prepare_constituent<C: IfcDetailsProfile>(
    snapshot: &IfcSnapshot,
    id: u64,
    value: &DslValue,
    tick: &mut dyn FnMut() -> Result<(), String>,
) -> Result<semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType, Fault> {
    let mut part = semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType::from_value(value.clone()).map_err(|error| Fault::from(error.to_string()))?;
    part.name = part.name.to_ascii_uppercase();
    let declaration = C::catalog().entities.get(&part.name).ok_or_else(|| Fault::from("Unknown IFC constituent type"))?;
    populate_global_ids(snapshot, id, &declaration.attributes, &mut part.args, tick)?;
    Ok(part)
}

fn prepare_entity<C: IfcDetailsProfile>(
    snapshot: &IfcSnapshot,
    value: &DslValue,
    allocate: bool,
    tick: &mut dyn FnMut() -> Result<(), String>,
) -> Result<semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcEntity, Fault> {
    let mut entity = semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcEntity::from_value(value.clone()).map_err(|error| Fault::from(error.to_string()))?;
    entity.name = entity.name.to_ascii_uppercase();
    let declaration = C::catalog().entities.get(&entity.name).ok_or_else(|| value_error("Unknown IFC entity type"))?;
    if declaration.abstract_entity && entity.complex.is_empty() {
        return Err(value_error("Abstract IFC entity requires a complex instance"));
    }
    if allocate && entity.id == 0 {
        entity.id = next_entity_id_controlled(snapshot, tick)?;
    }
    let attributes = if entity.complex.is_empty() { IfcDetailsProvider::<C>::attributes(&entity.name) } else { C::catalog().entities.get(&entity.name.to_ascii_uppercase()).map(|declaration| declaration.attributes.as_slice()) };
    if let Some(attributes) = attributes {
        populate_global_ids(snapshot, entity.id, &attributes, &mut entity.args, tick)?;
    }
    for part in &mut entity.complex {
        if let Some(declaration) = C::catalog().entities.get(&part.name.to_ascii_uppercase()) {
            populate_global_ids(snapshot, entity.id, &declaration.attributes, &mut part.args, tick)?;
        }
    }
    IfcDetailsProvider::<C>::new(snapshot).normalize_entity(&mut entity, false);
    Ok(entity)
}

fn tick_value(value: &IfcValue, tick: &mut dyn FnMut() -> Result<(), String>, depth: usize) -> Result<(), Fault> {
    tick().map_err(Fault::from)?;
    if depth > 128 {
        return Err(Fault::from("IFC edit value exceeds its bounded depth"));
    }
    match value {
        IfcValue::Aggregate(items) => {
            for item in items {
                tick_value(item, tick, depth + 1)?;
            }
        }
        IfcValue::TypedValue(value) => {
            for item in &value.items {
                tick_value(item, tick, depth + 1)?;
            }
        }
        _ => {}
    }
    Ok(())
}
fn tick_entity(entity: &IfcEntity, tick: &mut dyn FnMut() -> Result<(), String>) -> Result<(), Fault> {
    tick().map_err(Fault::from)?;
    for value in &entity.args {
        tick_value(value, tick, 0)?;
    }
    for part in &entity.complex {
        tick().map_err(Fault::from)?;
        for value in &part.args {
            tick_value(value, tick, 0)?;
        }
    }
    Ok(())
}

fn normalize_complex_changes<C: IfcDetailsProfile>(snapshot: &IfcSnapshot, mut mutations: Vec<IfcMutation>, tick: &mut dyn FnMut() -> Result<(), String>) -> Result<Vec<IfcMutation>, Fault> {
    use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::set_entity_arg::SetEntityArg;
    let provider = IfcDetailsProvider::<C>::new(snapshot);
    let mut primary = Vec::new();
    for mutation in &mut mutations {
        tick().map_err(Fault::from)?;
        let IfcMutation::SetEntityComplex(change) = mutation else { continue };
        let mut current = None;
        for entity in &snapshot.entities {
            tick().map_err(Fault::from)?;
            if entity.id == change.id {
                current = Some(entity);
                break;
            }
        }
        let Some(current) = current else { continue };
        tick_entity(current, tick)?;
        let derived = derived_keys::<C>(&current.name, &change.complex);
        if let Some(declaration) = C::catalog().entities.get(&current.name.to_ascii_uppercase()) {
            for (index, (attribute, value)) in declaration.attributes.iter().zip(&current.args).enumerate() {
                if let Some(value) = provider.normalized_argument(attribute, value, &derived) {
                    primary.push(IfcMutation::SetEntityArg(SetEntityArg { id: current.id, index, value }));
                }
            }
        }
        for part in &mut change.complex {
            if let Some(declaration) = C::catalog().entities.get(&part.name.to_ascii_uppercase()) {
                for (attribute, value) in declaration.attributes.iter().zip(&mut part.args) {
                    if let Some(next) = provider.normalized_argument(attribute, value, &derived) {
                        *value = next;
                    }
                }
            }
        }
    }
    primary.extend(mutations);
    Ok(primary)
}

fn entity_changes(current: &IfcEntity, next: IfcEntity, tick: &mut dyn FnMut() -> Result<(), String>) -> Result<Vec<IfcMutation>, Fault> {
    tick_entity(current, tick)?;
    tick_entity(&next, tick)?;
    use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::{
        insert_entity_arg::InsertEntityArg, remove_entity_arg::RemoveEntityArg, set_entity_arg::SetEntityArg, set_entity_complex::SetEntityComplex, set_entity_name::SetEntityName,
    };
    let id = current.id;
    let mut mutations = Vec::new();
    if next.name != current.name {
        mutations.push(IfcMutation::SetEntityName(SetEntityName { id, name: next.name }));
    }
    for index in (next.args.len()..current.args.len()).rev() {
        tick().map_err(Fault::from)?;
        mutations.push(IfcMutation::RemoveEntityArg(RemoveEntityArg { id, index }));
    }
    for (index, value) in next.args.into_iter().enumerate() {
        tick().map_err(Fault::from)?;
        if index >= current.args.len() {
            mutations.push(IfcMutation::InsertEntityArg(InsertEntityArg { id, index, value }));
        } else if !ifc_arguments_identical(&current.args[index..index + 1], std::slice::from_ref(&value)) {
            mutations.push(IfcMutation::SetEntityArg(SetEntityArg { id, index, value }));
        }
    }
    if !ifc_complex_identical(&next.complex, &current.complex) {
        mutations.push(IfcMutation::SetEntityComplex(SetEntityComplex { id, complex: next.complex }));
    }
    Ok(mutations)
}

pub fn details_edit_is_admitted<C: IfcDetailsProfile>(event: &editing::SnapshotEditEvent, snapshot: &IfcSnapshot) -> bool {
    if protocol::OpBinary::encode_op(event).map_or(true, |bytes| bytes.len() > editing::SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES) {
        return false;
    }
    match normalized_reference_edit::<C>(event, snapshot, false) {
        Ok(Some(normalized)) => return editing::snapshot_edit_value_is_admitted(&normalized, snapshot),
        Err(_) => return false,
        Ok(None) => {}
    }
    if editing::snapshot_edit_value_is_admitted(event, snapshot) {
        return true;
    }
    let editing::SnapshotEditEvent::InsertValue { path, value } = event else { return false };
    let Ok(segments) = editing::decode_pointer(path) else { return false };
    let [root, position, field, index] = segments.as_slice() else { return false };
    let Ok(position) = position.parse::<usize>() else { return false };
    if root != "entities" || field != "complex" || !matches!(index.as_str(), "0" | "-") || !snapshot.entities.get(position).is_some_and(|entity| entity.complex.is_empty()) {
        return false;
    }
    editing::snapshot_edit_value_is_admitted(&editing::SnapshotEditEvent::InsertValue { path: format!("/entities/{position}/complex"), value: DslValue::Array(vec![value.clone()]) }, snapshot)
}

pub fn details_edit_retained_path_is_admitted<C:IfcDetailsProfile>(event:&editing::SnapshotEditEvent,snapshot:&IfcSnapshot)->bool {
    match normalized_reference_edit::<C>(event,snapshot,false) {
        Ok(Some(normalized))=>return editing::snapshot_edit_path_is_admitted(&normalized,snapshot),Err(_)=>return false,Ok(None)=>{}
    }
    if editing::snapshot_edit_path_is_admitted(event,snapshot){return true;}
    let editing::SnapshotEditEvent::InsertValue {path,..}=event else{return false;};
    let Ok(segments)=editing::decode_pointer(path)else{return false;};
    let [root,position,field,index]=segments.as_slice()else{return false;};
    root=="entities"&&field=="complex"&&matches!(index.as_str(),"0"|"-")&&position.parse::<usize>().ok().and_then(|position|snapshot.entities.get(position)).is_some_and(|entity|entity.complex.is_empty())
}

fn plan_details_edit_controlled<C: IfcDetailsProfile>(event: &editing::SnapshotEditEvent, snapshot: &IfcSnapshot, tick: &mut dyn FnMut() -> Result<(), String>) -> Result<Option<Vec<IfcMutation>>, Fault> {
    tick().map_err(Fault::from)?;
    let resolve = |event: &editing::SnapshotEditEvent, tick: &mut dyn FnMut() -> Result<(), String>| -> Result<Vec<IfcMutation>, Fault> {
        tick().map_err(Fault::from)?;
        if let Some(mutations) = edit_rules::complex_edit(event, snapshot).map_err(|error| Fault::from(error.to_string()))? {
            return normalize_complex_changes::<C>(snapshot, mutations, tick);
        }
        edit_rules::EDIT_RULES.resolve::<IfcSnapshot, IfcMutation>(snapshot, event).map_err(|error| Fault::from(error.to_string()))
    };
    if let Some(normalized) = normalized_reference_edit_controlled::<C>(event, snapshot, true, tick)? {
        return resolve(&normalized, tick).map(Some);
    }
    let mutations = match event {
        editing::SnapshotEditEvent::SetValue { path, value } => {
            let segments = editing::decode_pointer(path).map_err(|error| Fault::from(error.to_string()))?;
            if segments.len() >= 4 && segments[0] == "entities" && segments[2] == "complex" && (segments.len() == 4 || (segments.len() == 5 && segments[4] == "name")) {
                let entity = snapshot.entities.get(segments[1].parse::<usize>().map_err(|_| Fault::from("Invalid IFC entity position"))?).ok_or_else(|| Fault::from("IFC entity does not exist"))?;
                let index = segments[3].parse::<usize>().map_err(|_| Fault::from("Invalid IFC constituent position"))?;
                let current = entity.complex.get(index).ok_or_else(|| Fault::from("IFC constituent does not exist"))?;
                tick_entity(entity, tick)?;
                let part = if segments.len() == 5 || requested_type(value).is_some() {
                    let name = if segments.len() == 5 {
                        let DslValue::String(name) = value else { return Err(Fault::from("IFC constituent type requires a name")) };
                        name.as_str()
                    } else {
                        requested_type(value).expect("type request checked")
                    };
                    IfcDetailsProvider::<C>::new(snapshot).constituent_template(name, Some(current)).ok_or_else(|| Fault::from("Unknown IFC constituent type"))?.to_value()
                } else {
                    value.clone()
                };
                let part = prepare_constituent::<C>(snapshot, entity.id, &part, tick)?;
                let target = if segments.len() == 5 { path.rsplit_once('/').map_or("", |(parent, _)| parent) } else { path.as_str() };
                resolve(&editing::SnapshotEditEvent::SetValue { path: target.into(), value: part.to_value() }, tick)?
            } else if (segments.len() == 2 || (segments.len() == 3 && segments[2] == "name")) && segments[0] == "entities" {
                let index = segments[1].parse::<usize>().map_err(|_| Fault::from("invalid IFC entity position"))?;
                let current = snapshot.entities.get(index).ok_or_else(|| Fault::from("IFC entity position does not exist"))?;
                tick_entity(current, tick)?;
                let template = if segments.len() == 3 || requested_type(value).is_some() {
                    let name = if segments.len() == 3 {
                        let DslValue::String(name) = value else { return Err(Fault::from("IFC entity type requires a name")) };
                        name.as_str()
                    } else {
                        requested_type(value).expect("type request checked")
                    };
                    Some(IfcDetailsProvider::<C>::new(snapshot).entity_template(name, Some(current)).ok_or_else(|| Fault::from("Unknown IFC entity type"))?.to_value())
                } else {
                    None
                };
                let entity = prepare_entity::<C>(snapshot, template.as_ref().unwrap_or(value), false, tick)?;
                if entity.id != current.id {
                    return Err(Fault::from("IFC type switching preserves entity identity"));
                }
                entity_changes(current, entity, tick)?
            } else {
                resolve(event, tick)?
            }
        }
        editing::SnapshotEditEvent::InsertValue { path, value } if path.starts_with("/entities/") && path.matches('/').count() == 2 => {
            let entity = prepare_entity::<C>(snapshot, value, true, tick)?;
            let normalized = editing::SnapshotEditEvent::InsertValue { path: path.clone(), value: entity.to_value() };
            resolve(&normalized, tick)?
        }
        editing::SnapshotEditEvent::InsertValue { path, value } if path.starts_with("/entities/") && path.matches('/').count() == 4 => {
            let segments = editing::decode_pointer(path).map_err(|error| Fault::from(error.to_string()))?;
            if segments[2] != "complex" {
                return resolve(event, tick).map(Some);
            }
            let entity = snapshot.entities.get(segments[1].parse::<usize>().map_err(|_| Fault::from("Invalid IFC entity position"))?).ok_or_else(|| Fault::from("IFC entity does not exist"))?;
            let part = prepare_constituent::<C>(snapshot, entity.id, value, tick)?;
            resolve(&editing::SnapshotEditEvent::InsertValue { path: path.clone(), value: part.to_value() }, tick)?
        }
        _ => resolve(event, tick)?,
    };
    Ok(Some(mutations))
}

pub fn details_edit_special_controlled<C: IfcDetailsProfile>(event:&editing::SnapshotEditEvent,snapshot:&IfcSnapshot,tick:&mut dyn FnMut()->Result<(),String>)->Result<Option<Vec<IfcMutation>>,Fault>{
    let mutations=plan_details_edit_controlled::<C>(event,snapshot,tick)?;
    if let Some(mutations)=mutations.as_deref(){let normalized=normalized_reference_edit_controlled::<C>(event,snapshot,true,tick)?;domain_edit::check::<C>(normalized.as_ref().unwrap_or(event),snapshot,mutations,tick)?;}
    Ok(mutations)
}

pub fn normalized_reference_edit<C: IfcDetailsProfile>(event: &editing::SnapshotEditEvent, snapshot: &IfcSnapshot, check_target: bool) -> Result<Option<editing::SnapshotEditEvent>, Fault> {
    normalized_reference_edit_controlled::<C>(event, snapshot, check_target, &mut || Ok(()))
}
pub fn details_edit_special<C: IfcDetailsProfile>(event: &editing::SnapshotEditEvent, snapshot: &IfcSnapshot) -> Result<Option<Vec<IfcMutation>>, Fault> {
    details_edit_special_controlled::<C>(event, snapshot, &mut || Ok(()))
}

#[cfg(test)]
mod profile_tests {
    use super::*;
    struct HeaderProfile;
    impl IfcDetailsProfile for HeaderProfile {
        type CandidateRules = ();
        const MAXIMUM_ATTRIBUTES: usize = 21;
        fn catalog() -> &'static IfcCatalog {
            panic!("overridden header variants must not query entity declarations")
        }
        fn detail_variants(_: &IfcSnapshot, _: &[P], locale: Locale) -> Option<Vec<SnapshotDetailVariant>> {
            let label = match locale {
                Locale::En => "Declared view",
                Locale::De => "Deklarierte Ansicht",
            };
            Some(vec![SnapshotDetailVariant { label: label.into(), value: DslValue::String("ViewDefinition [ReferenceView]".into()) }])
        }
    }
    #[test]
    fn retained_capacity_never_materializes_the_entity_catalog() {
        let _ = details_work_capacity::<HeaderProfile>();
    }

    #[test]
    fn profile_variants_share_collection_count_and_index_contract() {
        let snapshot = IfcSnapshot::default();
        let provider = IfcDetailsProvider::<HeaderProfile>::new(&snapshot);
        for locale in [Locale::En, Locale::De] {
            let variants = provider.variants(&[], locale);
            assert_eq!(variants.len(), provider.variant_count(&[], locale));
            let indexed = provider.variant(&[], locale, 0).unwrap();
            assert_eq!(indexed.label, variants[0].label);
            assert_eq!(indexed.value, variants[0].value);
            assert!(provider.variant(&[], locale, 1).is_none());
        }
    }
}

#[cfg(test)]
#[path="domain_tests.rs"]
mod domain_tests;
                                                                                                        

