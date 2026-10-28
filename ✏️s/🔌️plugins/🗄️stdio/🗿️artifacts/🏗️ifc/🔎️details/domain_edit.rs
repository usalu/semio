//! 🛂️ Definitive declaration checks borrow captured arguments and prepared primitive owners.
use super::domain::{DomainControl, ValueChange};
use super::*;
pub(super) fn event_path(event: &editing::SnapshotEditEvent) -> Option<(&str, ValueChange)> {
    match event {
        editing::SnapshotEditEvent::SetValue { path, .. } => Some((path, ValueChange::Set)),
        editing::SnapshotEditEvent::InsertValue { path, .. } => Some((path, ValueChange::Insert)),
        editing::SnapshotEditEvent::RemoveValue { path } => Some((path, ValueChange::Remove)),
        editing::SnapshotEditEvent::MoveValue { path, .. } => Some((path, ValueChange::Move)),
        _ => None,
    }
}
pub(super) fn draft(event: &editing::SnapshotEditEvent) -> bool {
    let editing::SnapshotEditEvent::SetValue { path, value } = event else {
        return false;
    };
    let mut fields = path.split('/').skip(1);
    if fields.next()!=Some("entities")||fields.next().is_none(){return false;}
    match fields.next(){
        None=>requested_type(value).is_some(),
        Some("name")=>fields.next().is_none(),
        Some("complex")=>fields.next().is_some()&&match fields.next(){None=>requested_type(value).is_some(),Some("name")=>fields.next().is_none(),_=>false},
        _=>false,
    }
}
pub(super) fn route(before: &IfcValue, suffix: &str, indices: &mut [usize; 128]) -> Result<usize, String> {
    let mut fields = suffix.split('/').skip(1).peekable();
    let mut value = before;
    let mut length = 0;
    while let Some(field) = fields.next() {
        if field != "value" {
            return Err("ifc-domain-path".into());
        }
        if fields.peek().is_none() {
            return Ok(length);
        }
        let position = match value {
            IfcValue::Aggregate(items) => {
                let field = fields.next().unwrap();
                if field == "-" { items.len() } else { field.parse::<usize>().map_err(|_| "ifc-domain-path")? }
            }
            IfcValue::TypedValue(_) => {
                let field = fields.next().unwrap();
                if field == "name" && fields.peek().is_none() {
                    return Ok(length);
                }
                if field != "items" {
                    return Err("ifc-domain-path".into());
                }
                if fields.peek().is_none() {
                    return Ok(length);
                }
                fields.next().unwrap().parse::<usize>().map_err(|_| "ifc-domain-path")?
            }
            _ => return Err("ifc-domain-path".into()),
        };
        if length == indices.len() {
            return Err("ifc-domain-depth".into());
        }
        indices[length] = position;
        length += 1;
        let items = match value {
            IfcValue::Aggregate(items) => items.as_slice(),
            IfcValue::TypedValue(typed) => typed.items.as_slice(),
            _ => unreachable!(),
        };
        if fields.peek().is_none() {
            return Ok(length);
        }
        value = items.get(position).ok_or_else(|| "ifc-domain-path".to_owned())?;
    }
    Ok(length)
}
fn argument<C: IfcDetailsProfile>(
    snapshot: &IfcSnapshot,
    entity: &IfcEntity,
    complex: &[IfcComplexType],
    structural: bool,
    creation: bool,
    name: &str,
    before: Option<&IfcValue>,
    after: &IfcValue,
    index: usize,
    direct: bool,
    prefix: &str,
    path: &str,
    operation: ValueChange,
    tick: &mut dyn FnMut() -> Result<(), String>,
) -> Result<(), String> {
    if let Some(before) = before {
        if domain::identical(before, after, tick)? {
            return Ok(());
        }
    }
    let declaration = C::catalog().entities.get(&name.to_ascii_uppercase()).ok_or_else(|| "ifc-domain-entity".to_owned())?;
    let attributes = if direct { &declaration.attributes } else { &declaration.all_attributes };
    let attribute = attributes.get(index).ok_or_else(|| "ifc-domain-attribute".to_owned())?;
    let mut derived = attribute.derived;
    for name in std::iter::once(entity.name.as_str()).chain(complex.iter().map(|part| part.name.as_str())) {
        tick()?;
        if let Some(declaration) = C::catalog().entities.get(&name.to_ascii_uppercase()) {
            for other in &declaration.all_attributes {
                tick()?;
                derived |= other.derived && other.name == attribute.name && other.declared_by == attribute.declared_by;
            }
        }
    }
    if derived {
        return if structural && matches!(after, IfcValue::Derived) { Ok(()) } else { Err("ifc-domain-derived".into()) };
    }
    if matches!(after, IfcValue::Unset) && (creation || structural && matches!(before, Some(IfcValue::Derived))) {
        return Ok(());
    }
    let mut reference = |id: u64, expected: &str, tick: &mut dyn FnMut() -> Result<(), String>| {
        let mut count = 0;
        let mut accepted = false;
        for target in &snapshot.entities {
            tick()?;
            if target.id == id {
                count += 1;
                for name in std::iter::once(target.name.as_str()).chain(target.complex.iter().map(|part| part.name.as_str())) {
                    tick()?;
                    accepted |= C::catalog().is_subtype(name, expected);
                }
            }
        }
        if count == 0 && creation && id == entity.id {
            count = 1;
            for name in std::iter::once(entity.name.as_str()).chain(complex.iter().map(|part| part.name.as_str())) {
                tick()?;
                accepted |= C::catalog().is_subtype(name, expected);
            }
        }
        Ok(count == 1 && accepted)
    };
    let mut control = DomainControl { tick, reference: &mut reference };
    let Some(before) = before else {
        return domain::check_value(C::catalog(), &attribute.type_expr, after, attribute.optional, true, &mut control);
    };
    let suffix = path.strip_prefix(prefix).filter(|suffix| suffix.is_empty() || suffix.starts_with('/'));
    let mut indices = [0; 128];
    if let Some(suffix) = suffix {
        let length = route(before, suffix, &mut indices)?;
        domain::check_change(C::catalog(), &attribute.type_expr, before, after, &indices[..length], operation, attribute.optional, &mut control)
    } else {
        domain::check_change(C::catalog(), &attribute.type_expr, before, after, &[], ValueChange::Set, attribute.optional, &mut control)
    }
}

pub(super) fn prior_component(event: &editing::SnapshotEditEvent, path: &str, index: usize, length: usize) -> Option<usize> {
    let mut fields = path.split('/').skip(1);
    fields.next()?;
    fields.next()?;
    if fields.next()? != "complex" {
        return Some(index);
    }
    let Some(target) = fields.next() else {
        return Some(index);
    };
    if fields.next().is_some() {
        return Some(index);
    }
    let target = if target == "-" { length } else { target.parse::<usize>().ok()? };
    match event {
        editing::SnapshotEditEvent::InsertValue { .. } => {
            if index == target {
                None
            } else {
                Some(if index > target { index - 1 } else { index })
            }
        }
        editing::SnapshotEditEvent::RemoveValue { .. } => Some(if index >= target { index + 1 } else { index }),
        editing::SnapshotEditEvent::MoveValue { from, .. } => {
            let start = from.rsplit('/').next()?.parse::<usize>().ok()?;
            let end = target.min(length.saturating_sub(1));
            Some(if index == end {
                start
            } else if start < end && (start..end).contains(&index) {
                index + 1
            } else if start > end && (end + 1..=start).contains(&index) {
                index - 1
            } else {
                index
            })
        }
        _ => Some(index),
    }
}

pub(super) fn inserted_entity<'a>(event: &editing::SnapshotEditEvent, path: &str, position: usize, mutations: &'a [IfcMutation]) -> Result<Option<&'a IfcEntity>, String> {
    if !matches!(event, editing::SnapshotEditEvent::InsertValue { .. }) || path.matches('/').count() != 2 {
        return Ok(None);
    }
    let [IfcMutation::InsertEntity(value)] = mutations else {
        return Err("ifc-domain-insertion".into());
    };
    if value.index != position {
        return Err("ifc-domain-insertion".into());
    }
    Ok(Some(&value.entity))
}
/// 🛂️ Check prepared native value mutations without mutating the captured model or owning a second graph.
pub(super) fn check<C: IfcDetailsProfile>(event: &editing::SnapshotEditEvent, snapshot: &IfcSnapshot, mutations: &[IfcMutation], tick: &mut dyn FnMut() -> Result<(), String>) -> Result<(), Fault> {
    tick().map_err(Fault::from)?;
    if draft(event) {
        return Ok(());
    }
    let Some((path, operation)) = event_path(event) else {
        return Ok(());
    };
    let mut fields = path.split('/').skip(1);
    if fields.next() != Some("entities") {
        return Ok(());
    }
    let Some(position) = fields.next().and_then(|position| if position == "-" { Some(snapshot.entities.len()) } else { position.parse::<usize>().ok() }) else {
        return Ok(());
    };
    let inserted = inserted_entity(event, path, position, mutations).map_err(Fault::from)?;
    let entity = inserted.or_else(|| snapshot.entities.get(position)).ok_or_else(|| Fault::from("ifc-domain-entity"))?;
    if inserted.is_some() {
        for (part, name, values) in std::iter::once((None, entity.name.as_str(), entity.args.as_slice())).chain(entity.complex.iter().enumerate().map(|(index, part)| (Some(index), part.name.as_str(), part.args.as_slice()))) {
            tick().map_err(Fault::from)?;
            let declaration = C::catalog().entities.get(&name.to_ascii_uppercase()).ok_or_else(|| Fault::from("ifc-domain-entity"))?;
            let direct = part.is_some() || !entity.complex.is_empty();
            let attributes = if direct { &declaration.attributes } else { &declaration.all_attributes };
            if values.len() != attributes.len() {
                return Err(Fault::from("ifc-domain-argument-arity"));
            }
            for (index, value) in values.iter().enumerate() {
                tick().map_err(Fault::from)?;
                argument::<C>(snapshot, entity, &entity.complex, true, true, name, None, value, index, direct, "", path, operation, tick).map_err(Fault::from)?;
            }
        }
        return Ok(());
    }
    let mut complex = entity.complex.as_slice();
    for mutation in mutations {
        tick().map_err(Fault::from)?;
        if let IfcMutation::SetEntityComplex(next) = mutation {
            if next.id != entity.id {
                return Err(Fault::from("ifc-domain-identity"));
            }
            complex = &next.complex;
        }
    }
    let structural = path.strip_prefix(&format!("/entities/{position}/complex")).is_some_and(|suffix| suffix.is_empty() || suffix.starts_with('/') && suffix.matches('/').count() == 1);
    for mutation in mutations {
        tick().map_err(Fault::from)?;
        match mutation {
            IfcMutation::SetEntityArg(next) => {
                if next.id != entity.id {
                    return Err(Fault::from("ifc-domain-identity"));
                }
                argument::<C>(snapshot, entity, complex, structural, false, &entity.name, entity.args.get(next.index), &next.value, next.index, !complex.is_empty(), &format!("/entities/{position}/args/{}", next.index), path, operation, tick)
                    .map_err(Fault::from)?;
            }
            IfcMutation::InsertEntityArg(next) => {
                argument::<C>(snapshot, entity, complex, false, false, &entity.name, None, &next.value, next.index, !complex.is_empty(), "", path, operation, tick).map_err(Fault::from)?;
            }
            IfcMutation::RemoveEntityArg(next) => {
                let declaration = C::catalog().entities.get(&entity.name.to_ascii_uppercase()).ok_or_else(|| Fault::from("ifc-domain-entity"))?;
                let count = if entity.complex.is_empty() { declaration.all_attributes.len() } else { declaration.attributes.len() };
                if next.index < count {
                    return Err(Fault::from("ifc-domain-argument-arity"));
                }
            }
            IfcMutation::SetEntityComplex(next) => {
                for (part_index, part) in next.complex.iter().enumerate() {
                    tick().map_err(Fault::from)?;
                    let prior = prior_component(event, path, part_index, entity.complex.len()).and_then(|index| entity.complex.get(index)).filter(|prior| prior.name.eq_ignore_ascii_case(&part.name));
                    let declaration = C::catalog().entities.get(&part.name.to_ascii_uppercase()).ok_or_else(|| Fault::from("ifc-domain-entity"))?;
                    if part.args.len() != declaration.attributes.len() && prior.is_none_or(|prior| prior.args.len() != part.args.len()) {
                        return Err(Fault::from("ifc-domain-argument-arity"));
                    }
                    for (index, value) in part.args.iter().enumerate() {
                        tick().map_err(Fault::from)?;
                        argument::<C>(
                            snapshot,
                            entity,
                            complex,
                            structural,
                            structural && prior.is_none(),
                            &part.name,
                            prior.and_then(|prior| prior.args.get(index)),
                            value,
                            index,
                            true,
                            &format!("/entities/{position}/complex/{part_index}/args/{index}"),
                            path,
                            operation,
                            tick,
                        )
                        .map_err(Fault::from)?;
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}
