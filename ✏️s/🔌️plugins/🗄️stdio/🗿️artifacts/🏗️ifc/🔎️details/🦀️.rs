//! 🧭️ Catalog-profile declaration-aware controls over the shared lossless IFC graph.

use super::{IfcAttributeDeclaration, IfcCatalog, IfcTypeExpression};
use semio_framework_plugin::Fault;
use semio_framework_ui_locale::Locale;
use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_s_artifact_stdio_contract::editing;
use semio_s_artifact_stdio_contract::editing::{DslSnapshotDetailsProvider, SnapshotDetailItemCapabilities, SnapshotDetailPathSegment as P, SnapshotDetailPresentation, SnapshotDetailValue, SnapshotDetailVariant, SnapshotDetailsProvider};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{ifc_arguments_identical, ifc_complex_identical, IfcComplexType, IfcEntity, IfcSnapshot, IfcTypedValue, IfcValue};
use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::{edit_rules, IfcMutation};

/// 🪪️ Borrowed declaration roles distinguish derived slots from directly owned identity slots.
#[derive(Clone,Copy)]
pub struct IfcNormalizationAttribute{pub derived:bool,pub global_id:bool,pub optional:bool,pub name:&'static str,pub type_expr:&'static dyn IfcDetailsTypeMetadata}
#[path="type_metadata.rs"]mod type_metadata;
#[path="type_remap.rs"]mod type_remap;
#[path="type_input.rs"]mod type_input;
#[path="type_input_kind.rs"]mod type_input_kind;
#[path="type_input_select.rs"]mod type_input_select;
#[path="type-guidance.rs"]mod type_guidance;
pub use type_guidance::{IfcTypeInputSourceWork,IfcPreparedTypeInputSource};
pub use type_metadata::{IfcDetailsTypeMetadata,IfcTypeProjection};
#[path="candidate_rules.rs"]mod candidate_rules;
pub use candidate_rules::{IfcCandidateRules,IfcCandidateRuleStep,IfcTypeCandidate};
pub trait IfcDetailsProfile {
    type CandidateRules: IfcCandidateRules;
    const MAXIMUM_ATTRIBUTES: usize;
    fn catalog() -> &'static IfcCatalog;
    /// 🧬️ Borrow one defined alias expression from the same profile metadata authority.
    fn defined_type_expression(name:&str)->Option<&'static dyn IfcDetailsTypeMetadata>{Some(Self::catalog().types.get(name)?.type_expr.as_ref()? as &dyn IfcDetailsTypeMetadata)}
    /// 🧭️ Borrow one direct or inherited attribute's normalization roles without copying its declaration.
    fn normalization_attribute(name:&str,index:usize,direct:bool)->Result<Option<IfcNormalizationAttribute>,()>{let declaration=Self::catalog().entities.get(name).ok_or(())?;let attributes=if direct{&declaration.attributes}else{&declaration.all_attributes};Ok(attributes.get(index).map(|attribute|IfcNormalizationAttribute{derived:attribute.derived,global_id:attribute.name=="GlobalId",optional:attribute.optional,name:&attribute.name,type_expr:&attribute.type_expr}))}
    /// 📏️ Borrow the direct attribute count for a canonical released entity name.
    fn direct_attribute_count(name:&str)->Option<usize>{Some(Self::catalog().entities.get(name)?.attributes.len())}
    /// 🌱️ A rootless declaration proves canonical full and direct tuple positions coincide.
    fn rootless_attribute_count(name:&str)->Option<usize>{let declaration=Self::catalog().entities.get(name)?;declaration.supertype.is_none().then_some(declaration.all_attributes.len())}
    /// 🔤️ Borrow each released enumeration member through the same immutable profile authority.
    fn enumeration_count(name:&str)->Option<usize>{let declaration=Self::catalog().types.get(name)?;(declaration.kind=="enum").then_some(declaration.members.len())}
    fn enumeration_member(name:&str,index:usize)->Option<&'static str>{let declaration=Self::catalog().types.get(name)?;if declaration.kind!="enum"{return None;}Some(declaration.members.get(index)?.as_str())}
    /// 🔀️ Borrow the complete SELECT roster from the same immutable declaration authority.
    fn select_count(name:&str)->Option<usize>{let declaration=Self::catalog().types.get(name)?;(declaration.kind=="select").then_some(declaration.members.len())}
    fn select_member(name:&str,index:usize)->Option<&'static str>{let declaration=Self::catalog().types.get(name)?;if declaration.kind!="select"{return None;}Some(declaration.members.get(index)?.as_str())}
    fn detail_variants(_snapshot: &IfcSnapshot, _path: &[P], _locale: Locale) -> Option<Vec<SnapshotDetailVariant>> {
        None
    }
    fn entity_presentation(_entity: &IfcEntity, _locale: Locale) -> Option<SnapshotDetailPresentation> {
        None
    }
}

#[path = "receive.rs"]
mod receive;
#[path = "value_edit.rs"]
mod value_edit;
#[path = "../📚️catalog/⚖️domain/🦀️.rs"]
mod domain;
#[path="../📚️catalog/⚖️domain/cursor.rs"]
mod domain_cursor;
#[path="domain_work.rs"]
mod domain_work;
#[path = "domain_edit.rs"]
mod domain_edit;
#[path = "wire.rs"]
mod wire;
#[path="source_loan.rs"]mod source_loan;
#[path="type_guidance_request.rs"]mod type_guidance_request;
#[path="type_guidance_candidate.rs"]mod type_guidance_candidate;
#[path = "work.rs"]
mod work;
#[path="admission.rs"]
mod admission;
#[path="command_wire.rs"]
mod command_wire;
#[path="pointer.rs"]
mod pointer;
#[path="reference.rs"]mod reference;
#[path="contextual.rs"]mod contextual;
pub use contextual::{IfcEnumerationAddress,IfcEnumerationDiagnostic,IfcEnumerationQuery,IfcEnumerationView};
#[path="contextual_render.rs"]mod contextual_render;
pub use contextual_render::IfcDetailsRenderWork;
#[path="contextual_edit.rs"]mod contextual_edit;
#[path="../📥️source/🗂️index/🦀️.rs"]mod source_index;
#[path="entity_remove.rs"]mod entity_remove;
#[path="constituent_roster.rs"]mod constituent_roster;
pub use receive::{IfcValueReceiveMode, IfcValueReceiver};
pub use wire::{IfcPublication, SnapshotSource};
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

const REFERENCE_WINDOW: usize = 256;
const DEPTH_LIMIT: usize = 64;

pub struct IfcDetailsProvider<'a, C: IfcDetailsProfile> {
    snapshot: &'a IfcSnapshot,
    profile: std::marker::PhantomData<fn() -> C>,
    raw: DslSnapshotDetailsProvider<'a, IfcSnapshot>,
    derived: std::cell::RefCell<std::collections::BTreeMap<usize, std::collections::BTreeSet<(&'static str, &'static str)>>>,
    contextual_enumerations:Option<(&'a IfcDetailsRenderWork,u64)>,
}

fn derived_keys<C: IfcDetailsProfile>(name: &str, complex: &[IfcComplexType]) -> std::collections::BTreeSet<(&'static str, &'static str)> {
    std::iter::once(name)
        .chain(complex.iter().map(|part| part.name.as_str()))
        .filter_map(|name| C::catalog().entities.get(&name.to_ascii_uppercase()))
        .flat_map(|declaration| declaration.all_attributes.iter())
        .filter(|attribute| attribute.derived)
        .map(|attribute| (attribute.declared_by.as_str(), attribute.name.as_str()))
        .collect()
}

fn key(segment: &P, expected: &str) -> bool {
    matches!(segment, P::Key(value) if value == expected)
}

fn named(name: &str) -> IfcTypeExpression {
    IfcTypeExpression { kind: "named".into(), name: Some(name.into()), ..Default::default() }
}

fn type_request(name: &str) -> DslValue {
    DslValue::Object(vec![("$ifcType".into(), DslValue::String(name.into()))])
}

fn requested_type(value: &DslValue) -> Option<&str> {
    type_input::request(value).map(|(name,_)|name)
}

fn resolved<C: IfcDetailsProfile>(expression: &IfcTypeExpression, depth: usize) -> IfcTypeExpression {
    if depth >= DEPTH_LIMIT {
        return expression.clone();
    }
    let Some(name) = expression.name.as_ref() else { return expression.clone() };
    let Some(declaration) = C::catalog().types.get(&name.to_ascii_uppercase()) else { return expression.clone() };
    match declaration.type_expr.as_ref() {
        Some(next) if declaration.kind == "defined" => resolved::<C>(next, depth + 1),
        _ => expression.clone(),
    }
}

impl<'a, C: IfcDetailsProfile> IfcDetailsProvider<'a, C> {
    fn attributes(name: &str) -> Option<&'static [IfcAttributeDeclaration]> {
        C::catalog().attribute_declarations(name)
    }
    pub fn new(snapshot: &'a IfcSnapshot) -> Self {
        Self { snapshot, profile: std::marker::PhantomData, raw: DslSnapshotDetailsProvider::new(snapshot), derived: Default::default(),contextual_enumerations:None }
    }
    pub fn with_contextual_enumerations(mut self,work:&'a IfcDetailsRenderWork,revision:u64)->Result<Self,semio_framework_value::ValueError>{work.validate(self.snapshot,revision)?;self.contextual_enumerations=Some((work,revision));Ok(self)}
    fn enumeration_choices(&self,path:&[P])->Option<(IfcEnumerationView<'a>,bool)>{let(work,revision)=self.contextual_enumerations?;let(entry,prefix)=work.at(self.snapshot,revision,path)?;let list=if path.len()==prefix{true}else if path.len()==prefix+2&&key(&path[prefix],"value")&&matches!(path[prefix+1],P::Index(_)){false}else{return None;};let view=entry.view(self.snapshot,revision).ok()?;view.allowed?;Some((view,list))}
    fn enumeration_variant(&self,path:&[P],locale:Locale,index:usize)->Option<SnapshotDetailVariant>{let(view,list)=self.enumeration_choices(path)?;if list&&index==0{return Some(SnapshotDetailVariant{label:match locale{Locale::En=>"Unset ($)",Locale::De=>"Nicht gesetzt ($)"}.into(),value:IfcValue::Unset.to_value()});}let index=index.checked_sub(usize::from(list))?;let preview=contextual_render::choice_label(view.allowed?.get(index)?);Some(SnapshotDetailVariant{label:match(locale,list){(Locale::En,false)=>format!("Choice {}: {preview}",index+1),(Locale::De,false)=>format!("Wert {}: {preview}",index+1),(Locale::En,true)=>format!("Use only {}: {preview}",index+1),(Locale::De,true)=>format!("Nur {} verwenden: {preview}",index+1)},value:contextual_render::choice_request(view.enumeration?,index,list)})}

    fn entity_at(&self, path: &[P]) -> Option<&'a IfcEntity> {
        let [P::Key(root), P::Index(index)] = path else { return None };
        (root == "entities").then(|| self.snapshot.entities.get(*index)).flatten()
    }

    fn complex_at(&self, path: &[P]) -> Option<&'a [semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType]> {
        let (P::Key(field), parent) = path.split_last()? else { return None };
        (field == "complex").then(|| self.entity_at(parent).map(|entity| entity.complex.as_slice())).flatten()
    }

    fn constituent_at(&self, path: &[P]) -> Option<&'a IfcComplexType> {
        let [P::Key(root), P::Index(entity), P::Key(field), P::Index(index)] = path else { return None };
        (root == "entities" && field == "complex").then(|| self.snapshot.entities.get(*entity)?.complex.get(*index)).flatten()
    }

    pub fn is_reference_payload(&self, path: &[P]) -> bool {
        if let Some((P::Key(field), parent)) = path.split_last() {
            if field == "appendReference" {
                return self.reference_collection(parent).is_some();
            }
            if field == "reference" {
                return self.reference_slot(parent);
            }
        }
        matches!(self.context(path), Some((attribute, _, IfcValue::Reference(_), true)) if !attribute.derived)
    }

    pub fn accepts_reference(&self, path: &[P], id: Option<u64>) -> bool {
        self.accepts_reference_controlled(path, id, &mut || Ok(())).unwrap_or(false)
    }
    pub fn accepts_reference_controlled(&self, path: &[P], id: Option<u64>, tick: &mut dyn FnMut() -> Result<(), String>) -> Result<bool, String> {
        let context_path = if matches!(path.last(),Some(P::Key(field))if field=="reference") { &path[..path.len() - 1] } else { path };
        let Some((attribute, expression, current, _)) = self.context(context_path) else { return Ok(false) };
        let Some(id) = id else { return Ok(expression.optional || (attribute.optional && (context_path.len() == 4 || (context_path.len() == 6 && key(&context_path[2], "complex"))))) };
        if attribute.derived {
            return Ok(false);
        }
        let slot = if matches!(context_path.last(), Some(P::Key(field)) if field == "value") { &context_path[..context_path.len() - 1] } else { context_path };
        if let Some((P::Index(selected), parent)) = slot.split_last() {
            if let Some((P::Key(field), collection)) = parent.split_last() {
                if field == "value" {
                    if let Some((_, expression, IfcValue::Aggregate(items), false)) = self.context(collection) {
                        let domain = resolved::<C>(&expression, 0);
                        if domain.unique || domain.aggregate.as_deref() == Some("SET") {
                            for (index, value) in items.iter().enumerate() {
                                tick()?;
                                if index != *selected && value == &IfcValue::Reference(id) && current != &IfcValue::Reference(id) {
                                    return Ok(false);
                                }
                            }
                        }
                    }
                }
            }
        }
        for entity in &self.snapshot.entities {
            tick()?;
            if entity.id == id {
                for name in std::iter::once(entity.name.as_str()).chain(entity.complex.iter().map(|part| part.name.as_str())) {
                    tick()?;
                    if Self::accepts_entity(&expression, name, 0) {
                        return Ok(true);
                    }
                }
                return Ok(false);
            }
        }
        Ok(false)
    }

    fn reference_collection(&self, path: &[P]) -> Option<(IfcTypeExpression, &'a [IfcValue])> {
        let (attribute, expression, IfcValue::Aggregate(items), false) = self.context(path)? else { return None };
        let expression = resolved::<C>(&expression, 0);
        if attribute.derived || expression.aggregate.as_deref() == Some("ARRAY") || expression.upper.is_some_and(|upper| items.len() >= upper.max(0) as usize) || !Self::has_entity_type(expression.item.as_deref()?, 0) {
            return None;
        }
        Some((expression, items))
    }

    pub fn accepts_reference_append(&self, path: &[P], id: u64) -> bool {
        self.accepts_reference_append_controlled(path, id, &mut || Ok(())).unwrap_or(false)
    }
    pub fn accepts_reference_append_controlled(&self, path: &[P], id: u64, tick: &mut dyn FnMut() -> Result<(), String>) -> Result<bool, String> {
        let Some((P::Key(field), parent)) = path.split_last() else { return Ok(false) };
        if field != "appendReference" {
            return Ok(false);
        }
        let Some((expression, items)) = self.reference_collection(parent) else { return Ok(false) };
        if expression.unique || expression.aggregate.as_deref() == Some("SET") {
            for item in items {
                tick()?;
                if item == &IfcValue::Reference(id) {
                    return Ok(false);
                }
            }
        }
        for entity in &self.snapshot.entities {
            tick()?;
            if entity.id == id {
                for name in std::iter::once(entity.name.as_str()).chain(entity.complex.iter().map(|part| part.name.as_str())) {
                    tick()?;
                    if Self::accepts_entity(expression.item.as_deref().expect("reference collection item"), name, 0) {
                        return Ok(true);
                    }
                }
                return Ok(false);
            }
        }
        Ok(false)
    }

    fn reference_slot(&self, path: &[P]) -> bool {
        self.context(path).is_some_and(|(attribute, expression, _, payload)| !attribute.derived && !payload && Self::has_entity_type(&expression, 0))
    }

    fn has_entity_type(expression: &IfcTypeExpression, depth: usize) -> bool {
        if depth >= DEPTH_LIMIT {
            return false;
        }
        let expression = resolved::<C>(expression, depth);
        let Some(name) = expression.name.as_ref() else { return false };
        if C::catalog().entities.contains_key(&name.to_ascii_uppercase()) {
            return true;
        }
        C::catalog().types.get(&name.to_ascii_uppercase()).is_some_and(|declaration| declaration.kind == "select" && declaration.members.iter().any(|member| Self::has_entity_type(&named(member), depth + 1)))
    }

    fn argument_templates(&self, attributes: &[IfcAttributeDeclaration], previous: Option<&[IfcAttributeDeclaration]>, values: &[IfcValue]) -> Vec<IfcValue> {
        attributes
            .iter()
            .map(|attribute| {
                if attribute.derived {
                    return IfcValue::Derived;
                }
                if let Some(value) = previous
                    .and_then(|previous| previous.iter().position(|old| old.name == attribute.name && old.type_expr == attribute.type_expr))
                    .and_then(|index| values.get(index))
                    .filter(|value| !matches!(value, IfcValue::Derived) && (!matches!(value, IfcValue::Unset) || attribute.optional))
                {
                    return value.clone();
                }
                if attribute.optional {
                    return IfcValue::Unset;
                }
                if attribute.name == "GlobalId" {
                    return IfcValue::String(String::new());
                }
                self.template(&attribute.type_expr, 0).unwrap_or(IfcValue::Unset)
            })
            .collect()
    }

    fn entity_template(&self, name: &str, current: Option<&IfcEntity>) -> Option<IfcEntity> {
        let partial = current.is_some_and(|entity| !entity.complex.is_empty());
        let attributes = if partial { C::catalog().entities.get(&name.to_ascii_uppercase())?.attributes.as_slice() } else { Self::attributes(name)? };
        let previous = current.and_then(|entity| if partial { C::catalog().entities.get(&entity.name.to_ascii_uppercase()).map(|declaration| declaration.attributes.as_slice()) } else { Self::attributes(&entity.name) });
        let args = self.argument_templates(&attributes, previous.as_deref(), current.map_or(&[], |entity| entity.args.as_slice()));
        let id = current.map_or(0, |entity| entity.id);
        let mut entity = IfcEntity { id, name: name.to_ascii_uppercase(), args, complex: current.map_or_else(Vec::new, |entity| entity.complex.clone()) };
        self.normalize_entity(&mut entity, partial);
        Some(entity)
    }

    fn normalized_argument(&self, attribute: &IfcAttributeDeclaration, value: &IfcValue, derived: &std::collections::BTreeSet<(&str, &str)>) -> Option<IfcValue> {
        let effective = attribute.derived || derived.contains(&(attribute.declared_by.as_str(), attribute.name.as_str()));
        if effective {
            return (!matches!(value, IfcValue::Derived)).then_some(IfcValue::Derived);
        }
        matches!(value, IfcValue::Derived).then(|| if attribute.optional { IfcValue::Unset } else { self.template(&attribute.type_expr, 0).unwrap_or(IfcValue::Unset) })
    }

    fn normalize_entity(&self, entity: &mut IfcEntity, partial: bool) {
        let derived = derived_keys::<C>(&entity.name, &entity.complex);
        let attributes = if partial || !entity.complex.is_empty() { C::catalog().entities.get(&entity.name.to_ascii_uppercase()).map(|declaration| declaration.attributes.as_slice()) } else { Self::attributes(&entity.name) };
        if let Some(attributes) = attributes {
            for (attribute, value) in attributes.iter().zip(&mut entity.args) {
                if let Some(next) = self.normalized_argument(attribute, value, &derived) {
                    *value = next;
                }
            }
        }
        for part in &mut entity.complex {
            if let Some(declaration) = C::catalog().entities.get(&part.name.to_ascii_uppercase()) {
                for (attribute, value) in declaration.attributes.iter().zip(&mut part.args) {
                    if let Some(next) = self.normalized_argument(attribute, value, &derived) {
                        *value = next;
                    }
                }
            }
        }
    }

    pub fn constituent_template(&self, name: &str, current: Option<&IfcComplexType>) -> Option<IfcComplexType> {
        let declaration = C::catalog().entities.get(&name.to_ascii_uppercase())?;
        let previous = current.and_then(|part| C::catalog().entities.get(&part.name.to_ascii_uppercase())).map(|declaration| declaration.attributes.as_slice());
        Some(IfcComplexType { name: declaration.name.clone(), args: self.argument_templates(&declaration.attributes, previous, current.map_or(&[], |part| part.args.as_slice())) })
    }

    fn context(&self, path: &[P]) -> Option<(IfcAttributeDeclaration, IfcTypeExpression, &'a IfcValue, bool)> {
        if path.len() < 4 || path.len() > DEPTH_LIMIT || !key(&path[0], "entities") {
            return None;
        }
        let P::Index(entity_index) = path[1] else { return None };
        let entity = self.snapshot.entities.get(entity_index)?;
        let (name, args, start) = if key(&path[2], "args") {
            (&entity.name, &entity.args, 3)
        } else if path.len() >= 6 && key(&path[2], "complex") && key(&path[4], "args") {
            let P::Index(index) = path[3] else { return None };
            let constituent = entity.complex.get(index)?;
            (&constituent.name, &constituent.args, 5)
        } else {
            return None;
        };
        let P::Index(index) = path[start] else { return None };
        let attributes = if start == 5 || !entity.complex.is_empty() { C::catalog().entities.get(&name.to_ascii_uppercase())?.attributes.as_slice() } else { Self::attributes(name)? };
        let mut attribute = attributes.get(index)?.clone();
        let mut cache = self.derived.borrow_mut();
        let derived = cache.entry(entity_index).or_insert_with(|| derived_keys::<C>(&entity.name, &entity.complex));
        if derived.contains(&(attribute.declared_by.as_str(), attribute.name.as_str())) {
            attribute.derived = true;
        }
        let mut expression = attribute.type_expr.clone();
        let mut value = args.get(index)?;
        let mut cursor = start + 1;
        let mut payload = false;
        while cursor < path.len() {
            if !key(&path[cursor], "value") {
                return None;
            }
            cursor += 1;
            payload = true;
            if cursor == path.len() {
                break;
            }
            match value {
                IfcValue::Aggregate(items) => {
                    let P::Index(index) = path[cursor] else { return None };
                    value = items.get(index)?;
                    let aggregate = resolved::<C>(&expression, 0);
                    expression = *aggregate.item?;
                    expression.optional |= aggregate.optional;
                    cursor += 1;
                    payload = false;
                }
                IfcValue::TypedValue(typed) if key(&path[cursor], "items") => {
                    cursor += 1;
                    if cursor == path.len() {
                        return Some((attribute, named(&typed.name), value, true));
                    }
                    let P::Index(index) = path[cursor] else { return None };
                    value = typed.items.get(index)?;
                    expression = named(&typed.name);
                    cursor += 1;
                    payload = false;
                }
                _ => return None,
            }
        }
        Some((attribute, expression, value, payload))
    }

    fn accepts_entity(expression: &IfcTypeExpression, name: &str, depth: usize) -> bool {
        if depth >= DEPTH_LIMIT {
            return false;
        }
        let Some(expected) = expression.name.as_ref() else { return false };
        if C::catalog().is_subtype(name, expected) {
            return true;
        }
        let Some(declaration) = C::catalog().types.get(&expected.to_ascii_uppercase()) else { return false };
        declaration.kind == "select" && declaration.members.iter().any(|member| Self::accepts_entity(&named(member), name, depth + 1))
    }

    fn template(&self, expression: &IfcTypeExpression, depth: usize) -> Option<IfcValue> {
        if depth >= DEPTH_LIMIT {
            return None;
        }
        let expression = resolved::<C>(expression, depth);
        if expression.kind == "aggregate" {
            let count = if expression.aggregate.as_deref() == Some("ARRAY") { expression.upper?.checked_sub(expression.lower?)?.checked_add(1)?.max(0) as usize } else { expression.lower.unwrap_or(0).max(0) as usize };
            if count > REFERENCE_WINDOW {
                return None;
            }
            if expression.unique && count > 1 {
                return None;
            }
            let value = self.template(expression.item.as_deref()?, depth + 1)?;
            return Some(IfcValue::Aggregate(vec![value; count]));
        }
        let name = expression.name.as_ref()?.to_ascii_uppercase();
        if C::catalog().entities.contains_key(&name) {
            return self.snapshot.entities.iter().take(REFERENCE_WINDOW).find(|entity| C::catalog().is_subtype(&entity.name, &name)).map(|entity| IfcValue::Reference(entity.id));
        }
        if let Some(declaration) = C::catalog().types.get(&name) {
            return match declaration.kind.as_str() {
                "enum" => declaration.members.first().map(|member| IfcValue::Enum(member.to_ascii_uppercase())),
                "select" => declaration.members.iter().find_map(|member| self.select_template(member, depth + 1)),
                _ => None,
            };
        }
        match name.as_str() {
            "STRING" => Some(IfcValue::String(String::new())),
            "BINARY" => Some(IfcValue::Binary("0".into())),
            "REAL" | "NUMBER" => Some(IfcValue::Real(0.0)),
            "INTEGER" => Some(IfcValue::Integer(0)),
            "BOOLEAN" | "LOGICAL" => Some(IfcValue::Enum("F".into())),
            _ => None,
        }
    }

    fn select_template(&self, name: &str, depth: usize) -> Option<IfcValue> {
        let value = self.template(&named(name), depth)?;
        if matches!(value, IfcValue::Reference(_) | IfcValue::TypedValue(_)) {
            return Some(value);
        }
        Some(IfcValue::TypedValue(IfcTypedValue { name: name.to_ascii_uppercase(), items: vec![value] }))
    }

    fn select_members(name: &str, depth: usize, members: &mut std::collections::BTreeSet<String>) {
        if depth >= DEPTH_LIMIT {
            return;
        }
        if let Some(declaration) = C::catalog().types.get(&name.to_ascii_uppercase()).filter(|declaration| declaration.kind == "select") {
            for member in &declaration.members {
                Self::select_members(member, depth + 1, members);
            }
        } else {
            members.insert(name.to_ascii_uppercase());
        }
    }

    fn unused_template(&self, expression: &IfcTypeExpression, existing: &[IfcValue]) -> Option<IfcValue> {
        let candidate = self.template(expression, 0)?;
        if !existing.contains(&candidate) {
            return Some(candidate);
        }
        if let Some(entity) = self.snapshot.entities.iter().take(REFERENCE_WINDOW).find(|entity| Self::accepts_entity(expression, &entity.name, 0) && !existing.contains(&IfcValue::Reference(entity.id))) {
            return Some(IfcValue::Reference(entity.id));
        }
        let domain = resolved::<C>(expression, 0);
        if let Some(declaration) = domain.name.as_ref().and_then(|name| C::catalog().types.get(&name.to_ascii_uppercase())) {
            if declaration.kind == "enum" {
                return declaration.members.iter().map(|member| IfcValue::Enum(member.to_ascii_uppercase())).find(|value| !existing.contains(value));
            }
            if declaration.kind == "select" {
                let mut members = std::collections::BTreeSet::new();
                Self::select_members(&declaration.name, 0, &mut members);
                return members.iter().filter_map(|member| self.select_template(member, 0)).find(|value| !existing.contains(value));
            }
        }
        (0..REFERENCE_WINDOW)
            .filter_map(|index| {
                let value = match &candidate {
                    IfcValue::String(_) => {
                        let text = if domain.fixed { format!("{index:0width$}", width = domain.width.unwrap_or(0).clamp(0, REFERENCE_WINDOW as i64) as usize) } else { index.to_string() };
                        if domain.width.is_some_and(|width| text.chars().count() > width.max(0) as usize) {
                            return None;
                        }
                        IfcValue::String(text)
                    }
                    IfcValue::Integer(_) => IfcValue::Integer(index as i64),
                    IfcValue::Real(_) => IfcValue::Real(index as f64),
                    IfcValue::Binary(_) => IfcValue::Binary(format!("0{index:02X}")),
                    _ => return None,
                };
                (!existing.contains(&value)).then_some(value)
            })
            .next()
    }

    fn alternatives(&self, path: &[P], locale: Locale) -> Vec<SnapshotDetailVariant> {
        let Some((attribute, expression, value, payload)) = self.context(path) else { return Vec::new() };
        if payload || attribute.derived {
            return Vec::new();
        }
        let mut variants = Vec::new();
        if expression.optional || (attribute.optional && (path.len() == 4 || (path.len() == 6 && key(&path[2], "complex")))) {
            variants.push(SnapshotDetailVariant {
                label: match locale {
                    Locale::En => "Unset ($)",
                    Locale::De => "Nicht gesetzt ($)",
                }
                .into(),
                value: IfcValue::Unset.to_value(),
            });
        }
        if let Some(template) = self.template(&expression, 0) {
            if matches!(value, IfcValue::Unset) {
                variants.push(SnapshotDetailVariant {
                    label: match locale {
                        Locale::En => "Create value",
                        Locale::De => "Wert erstellen",
                    }
                    .into(),
                    value: template.to_value(),
                });
            }
        }
        let expression = resolved::<C>(&expression, 0);
        if let Some(declaration) = expression.name.as_ref().and_then(|name| C::catalog().types.get(&name.to_ascii_uppercase())).filter(|declaration| declaration.kind == "select") {
            let mut members = std::collections::BTreeSet::new();
            Self::select_members(&declaration.name, 0, &mut members);
            for member in members {
                if let Some(value) = self.select_template(&member, 0) {
                    variants.push(SnapshotDetailVariant { label: member, value: value.to_value() });
                }
            }
        }
        for entity in self.snapshot.entities.iter().take(REFERENCE_WINDOW).filter(|entity| Self::accepts_entity(&expression, &entity.name, 0)) {
            let title = entity.args.get(2).and_then(IfcValue::as_str).unwrap_or("");
            variants.push(SnapshotDetailVariant { label: format!("#{} {} {title}", entity.id, entity.name), value: IfcValue::Reference(entity.id).to_value() });
        }
        variants
    }
}

impl<C: IfcDetailsProfile> SnapshotDetailsProvider for IfcDetailsProvider<'_, C> {
    fn value(&self, path: &[P]) -> Option<SnapshotDetailValue> {
        if self.complex_at(path).is_some() {
            return Some(SnapshotDetailValue::Array);
        }
        if let Some((P::Key(field), parent)) = path.split_last() {
            if field == "appendReference" && self.reference_collection(parent).is_some() {
                return Some(SnapshotDetailValue::String(String::new()));
            }
            if field == "reference" && self.reference_slot(parent) {
                let (_, _, value, _) = self.context(parent)?;
                return Some(SnapshotDetailValue::String(value.as_reference().map_or_else(String::new, |id| id.to_string())));
            }
        }
        if let Some((_, _, IfcValue::Reference(id), true)) = self.context(path) {
            return Some(SnapshotDetailValue::String(id.to_string()));
        }
        self.raw.value(path)
    }
    fn child_count(&self, path: &[P]) -> usize {
        if self.entity_at(path).is_some() {
            return 4;
        }
        if let Some(complex) = self.complex_at(path) {
            return complex.len();
        }
        if let Some((_, _, value, false)) = self.context(path) {
            return usize::from(!matches!(value, IfcValue::Unset | IfcValue::Derived | IfcValue::Reference(_))) + usize::from(self.reference_slot(path)) + usize::from(self.reference_collection(path).is_some());
        }
        self.raw.child_count(path)
    }
    fn object_key(&self, path: &[P], index: usize) -> Option<String> {
        if self.entity_at(path).is_some() {
            return ["id", "name", "args", "complex"].get(index).map(|field| (*field).into());
        }
        if let Some((_, _, value, false)) = self.context(path) {
            let payload = !matches!(value, IfcValue::Unset | IfcValue::Derived | IfcValue::Reference(_));
            if index == 0 && payload {
                return Some("value".into());
            }
            if index == usize::from(payload) {
                if self.reference_slot(path) {
                    return Some("reference".into());
                }
                if self.reference_collection(path).is_some() {
                    return Some("appendReference".into());
                }
            }
            return None;
        }
        self.raw.object_key(path, index)
    }
    fn has_source(&self) -> bool {
        true
    }
    fn source(&self) -> Option<String> {
        self.raw.source()
    }
    fn allows_untyped_creation(&self, _path: &[P]) -> bool {
        false
    }
    fn enum_values(&self, path: &[P]) -> Vec<DslValue> {
        let Some((_, expression, value, true)) = self.context(path) else { return Vec::new() };
        if !matches!(value, IfcValue::Enum(_)) {
            return Vec::new();
        }
        let expression = resolved::<C>(&expression, 0);
        let Some(name) = expression.name else { return Vec::new() };
        if matches!(name.as_str(), "BOOLEAN" | "LOGICAL") {
            return if name == "BOOLEAN" { vec!["T", "F"] } else { vec!["T", "F", "U"] }.into_iter().map(|value| DslValue::String(value.into())).collect();
        }
        C::catalog().types.get(&name.to_ascii_uppercase()).filter(|declaration| declaration.kind == "enum").map(|declaration| declaration.members.iter().map(|member| DslValue::String(member.to_ascii_uppercase())).collect()).unwrap_or_default()
    }
    fn presentation(&self, path: &[P], locale: Locale) -> Option<SnapshotDetailPresentation> {
        if let Some((work,revision))=self.contextual_enumerations{if let Some((entry,prefix))=work.at(self.snapshot,revision,path){if path.len()==prefix{if let Some(diagnostic)=entry.view(self.snapshot,revision).ok()?.diagnostic{return Some(SnapshotDetailPresentation{label:"EnumerationValues".into(),description:Some(match(locale,diagnostic){(Locale::En,IfcEnumerationDiagnostic::Unset)=>"Enumeration reference is unset. Set its exact entity ID to obtain source choices.",(Locale::De,IfcEnumerationDiagnostic::Unset)=>"Aufzählungsreferenz ist nicht gesetzt. Exakte Element-ID für Quellwerte festlegen.",(Locale::En,IfcEnumerationDiagnostic::Missing)=>"Enumeration reference target is missing. Repair the reference to obtain source choices.",(Locale::De,IfcEnumerationDiagnostic::Missing)=>"Ziel der Aufzählungsreferenz fehlt. Referenz für Quellwerte korrigieren.",(Locale::En,IfcEnumerationDiagnostic::WrongKind)=>"Referenced source is not a complete enumeration. Repair its type or values.",(Locale::De,IfcEnumerationDiagnostic::WrongKind)=>"Referenzierte Quelle ist keine vollständige Aufzählung. Typ oder Werte korrigieren.",(Locale::En,IfcEnumerationDiagnostic::Ambiguous)=>"Enumeration source identities or constituents are ambiguous. Repair the source.",(Locale::De,IfcEnumerationDiagnostic::Ambiguous)=>"Quellidentitäten oder Bestandteile sind mehrdeutig. Quelle korrigieren.",(Locale::En,IfcEnumerationDiagnostic::Empty)=>"Referenced enumeration has no allowed values. Repair its values.",(Locale::De,IfcEnumerationDiagnostic::Empty)=>"Referenzierte Aufzählung hat keine zulässigen Werte. Werte korrigieren."}.into())});}}}}
        if let Some((attribute, expression, value, payload)) = self.context(path) {
            if !payload {
                let state = match (locale, value) {
                    (Locale::En, IfcValue::Unset) => " — unset",
                    (Locale::De, IfcValue::Unset) => " — nicht gesetzt",
                    (Locale::En, IfcValue::Derived) => " — derived",
                    (Locale::De, IfcValue::Derived) => " — abgeleitet",
                    _ => "",
                };
                let type_name = expression.name.as_deref().or(expression.aggregate.as_deref()).unwrap_or("");
                return Some(SnapshotDetailPresentation {
                    label: format!("{}{state}", attribute.name),
                    description: Some(match locale {
                        Locale::En => format!("{} · {type_name}. Suggestions inspect the first {REFERENCE_WINDOW} entities. The reference field accepts every exact entity ID.", attribute.declared_by),
                        Locale::De => format!("{} · {type_name}. Vorschläge prüfen die ersten {REFERENCE_WINDOW} Entitäten. Das Referenzfeld nimmt jede exakte Entitäts-ID an.", attribute.declared_by),
                    }),
                });
            }
        }
        if path.len() == 2 && key(&path[0], "entities") {
            let P::Index(index) = path[1] else { return None };
            let entity = self.snapshot.entities.get(index)?;
            return C::entity_presentation(entity, locale).or_else(|| Some(SnapshotDetailPresentation { label: format!("#{} {}", entity.id, entity.name), description: None }));
        }
        let P::Key(last) = path.last()? else { return None };
        let label = match (locale, last.as_str()) {
            (Locale::En, "appendReference") => "Add reference by entity ID",
            (Locale::De, "appendReference") => "Referenz per Entitäts-ID hinzufügen",
            (Locale::En, "reference") => "Reference entity ID",
            (Locale::De, "reference") => "Referenzierte Entitäts-ID",
            (Locale::En, "entities") => "Entities",
            (Locale::De, "entities") => "Entitäten",
            (Locale::En, "header") => "File header",
            (Locale::De, "header") => "Dateikopf",
            (Locale::En, "args") => "Attributes",
            (Locale::De, "args") => "Attribute",
            (Locale::En, "complex") => "Complex constituents",
            (Locale::De, "complex") => "Komplexe Bestandteile",
            (Locale::En, "name") => "IFC type",
            (Locale::De, "name") => "IFC-Typ",
            (Locale::En, "value") => "Value",
            (Locale::De, "value") => "Wert",
            _ => return self.raw.presentation(path, locale),
        };
        Some(SnapshotDetailPresentation { label: label.into(), description: None })
    }
    fn item_capabilities(&self, path: &[P]) -> SnapshotDetailItemCapabilities {
        let mut capabilities = SnapshotDetailItemCapabilities { edit: false, rename: false, remove: false, reorder: false };
        if self.is_reference_payload(path) {
            capabilities.edit = true;
            return capabilities;
        }
        if let Some((attribute, _, value, payload)) = self.context(path) {
            capabilities.edit = payload && !attribute.derived && !matches!(value, IfcValue::Aggregate(_) | IfcValue::TypedValue(_));
            if matches!(path.last(), Some(P::Index(_))) && payload == false && path.len() > 4 {
                let parent = &path[..path.len() - 1];
                if let Some((_, parent_type, IfcValue::Aggregate(items), true)) = self.context(parent) {
                    let aggregate = resolved::<C>(&parent_type, 0);
                    capabilities.remove = aggregate.aggregate.as_deref() != Some("ARRAY") && items.len() > aggregate.lower.unwrap_or(0).max(0) as usize;
                    capabilities.reorder = matches!(aggregate.aggregate.as_deref(), Some("LIST") | Some("ARRAY"));
                }
            }
            return capabilities;
        }
        if path.len() >= 2 && key(&path[0], "header") {
            capabilities.edit = !matches!(path.last(), Some(P::Key(value)) if value == "kind");
        }
        if path.len() > 3 && key(&path[0], "header") && matches!(path.last(), Some(P::Index(_))) && path.iter().any(|segment| key(segment, "value")) {
            capabilities.remove = self.raw.child_count(&path[..path.len() - 1]) > 1;
            capabilities.reorder = true;
        }
        if path.len() == 2 && key(&path[0], "entities") {
            capabilities.remove = true;
        }
        if path.len() >= 4 && key(&path[0], "entities") && key(&path[2], "complex") {
            if path.len() == 4 {
                capabilities.remove = true;
                capabilities.reorder = true;
            }
            if path.len() == 5 && key(&path[4], "name") {
                capabilities.edit = true;
            }
        }
        capabilities
    }
    fn allows_collection_insert(&self, path: &[P]) -> bool {
        if path.len() == 1 && key(&path[0], "entities") {
            return self.creation_template(path, true).is_some();
        }
        if path.len() == 3 && key(&path[0], "entities") && key(&path[2], "complex") {
            return true;
        }
        if path.len() > 2 && key(&path[0], "header") && path.last().is_some_and(|segment| key(segment, "value")) && matches!(self.raw.value(path), Some(SnapshotDetailValue::Array)) {
            return true;
        }
        let Some((attribute, expression, IfcValue::Aggregate(items), true)) = self.context(path) else { return false };
        let expression = resolved::<C>(&expression, 0);
        !attribute.derived && expression.aggregate.as_deref() != Some("ARRAY") && expression.upper.is_none_or(|upper| items.len() < upper.max(0) as usize) && self.creation_template(path, true).is_some()
    }
    fn creation_template(&self, path: &[P], collection_item: bool) -> Option<DslValue> {
        if let Some((work,revision))=self.contextual_enumerations{if let Some((entry,prefix))=work.at(self.snapshot,revision,path){if collection_item&&path.len()==prefix+1&&key(&path[prefix],"value"){let view=entry.view(self.snapshot,revision).ok()?;view.allowed?.first()?;return Some(contextual_render::choice_request(view.enumeration?,0,false));}if !collection_item&&path.len()==prefix{let view=entry.view(self.snapshot,revision).ok()?;view.allowed?.first()?;return Some(contextual_render::choice_request(view.enumeration?,0,true));}}}
        if collection_item && path.len() == 1 && key(&path[0], "entities") {
            return self.entity_template("IFCCARTESIANPOINT", None).map(|entity| entity.to_value());
        }
        if collection_item && path.len() == 3 && key(&path[0], "entities") && key(&path[2], "complex") {
            return Some(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType { name: "IFCREPRESENTATIONITEM".into(), args: Vec::new() }.to_value());
        }
        if collection_item && path.len() > 2 && key(&path[0], "header") && path.last().is_some_and(|segment| key(segment, "value")) {
            return Some(IfcValue::String(String::new()).to_value());
        }
        let (attribute, expression, value, payload) = self.context(path)?;
        if attribute.derived {
            return None;
        }
        if collection_item {
            let expression = resolved::<C>(&expression, 0);
            if expression.unique || expression.aggregate.as_deref() == Some("SET") {
                return self.unused_template(expression.item.as_deref()?, value.as_aggregate()?).map(|value| value.to_value());
            }
            self.template(expression.item.as_deref()?, 0).map(|candidate| candidate.to_value())
        } else if !payload {
            self.template(&expression, 0).map(|value| value.to_value())
        } else {
            None
        }
    }
    fn variants(&self, path: &[P], locale: Locale) -> Vec<SnapshotDetailVariant> {
        if self.enumeration_choices(path).is_some(){return(0..self.variant_count(path,locale)).filter_map(|index|self.enumeration_variant(path,locale,index)).collect();}
        C::detail_variants(self.snapshot, path, locale).unwrap_or_else(|| self.alternatives(path, locale))
    }
    fn variant_count(&self, path: &[P], locale: Locale) -> usize {
        if let Some((view,list))=self.enumeration_choices(path){return view.allowed.map_or(0,|values|values.len())+usize::from(list);}
        if let Some(variants) = C::detail_variants(self.snapshot, path, locale) {
            return variants.len();
        }
        if let Some(current) = self.entity_at(path) {
            return C::catalog().entities.values().filter(|entity| !current.complex.is_empty() || !entity.abstract_entity).count();
        }
        if self.constituent_at(path).is_some() {
            return C::catalog().entities.len();
        }
        self.alternatives(path, locale).len()
    }
    fn variant(&self, path: &[P], locale: Locale, index: usize) -> Option<SnapshotDetailVariant> {
        if self.enumeration_choices(path).is_some(){return self.enumeration_variant(path,locale,index);}
        if let Some(variants) = C::detail_variants(self.snapshot, path, locale) {
            return variants.into_iter().nth(index);
        }
        if let Some(current) = self.entity_at(path) {
            let declaration = C::catalog().entities.values().filter(|entity| !current.complex.is_empty() || !entity.abstract_entity).nth(index)?;
            return Some(SnapshotDetailVariant { label: declaration.name.clone(), value: type_request(&declaration.name) });
        }
        if let Some(current) = self.constituent_at(path) {
            let declaration = C::catalog().entities.values().nth(index)?;
            let _ = current;
            return Some(SnapshotDetailVariant { label: declaration.name.clone(), value: type_request(&declaration.name) });
        }
        self.alternatives(path, locale).into_iter().nth(index)
    }
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
                                                                                                        
