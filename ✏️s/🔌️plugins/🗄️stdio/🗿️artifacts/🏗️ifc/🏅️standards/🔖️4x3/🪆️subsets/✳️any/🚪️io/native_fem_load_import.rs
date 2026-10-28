//! 📥️ Source-ordered nodal cases receive owned canonical rows through original declared query loans.
use super::*;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::{FemLoad, FemLoadCase};
#[derive(Default, RetireOwned)]
pub(super) struct State {
    cases: Vec<usize>,
    usage: Vec<u8>,
    seen: Vec<usize>,
    case: Option<FemLoadCase>,
    load: Option<FemLoad>,
    force_checked: bool,
    torque_checked: bool,
}
fn visit<O: RetireOwned + 'static>(
    graph: &Graph<'_>,
    id: u64,
    name: &'static str,
    declared: &'static str,
    owner: &mut Owner<'_, '_>,
    receive: impl for<'v> FnOnce(Option<&'v IfcValue>, &mut Owner<'_, '_>) -> Result<O, ValueError>,
) -> Result<O, ValueError> {
    let row = graph.ordinal(id, owner)?;
    load_query::with_attribute(graph.source, row, name, declared, 0, owner, receive)
}
pub(super) fn reference_at(source: &IfcSnapshot, row: usize, name: &'static str, declared: &'static str, owner: &mut Owner<'_, '_>) -> Result<u64, ValueError> {
    load_query::with_attribute(source, row, name, declared, 0, owner, |value, _| reference(value))
}
fn reference_of(graph: &Graph<'_>, id: u64, name: &'static str, declared: &'static str, owner: &mut Owner<'_, '_>) -> Result<u64, ValueError> {
    visit(graph, id, name, declared, owner, |value, _| reference(value))
}
pub(super) fn prepare(graph: &Graph<'_>, analysis: u64, state: &mut State, owner: &mut Owner<'_, '_>) -> Result<(), ValueError> {
    state.usage = vector(graph.source.entities.len(), owner)?;
    state.seen = vector(graph.source.entities.len(), owner)?;
    for _ in &graph.source.entities {
        work(owner, size_of::<u8>() + size_of::<usize>())?;
        state.usage.push(0);
        state.seen.push(0);
    }
    visit(graph, analysis, "LoadedBy", "IFCSTRUCTURALANALYSISMODEL", owner, |value, original| {
        let values = match value {
            Some(IfcValue::Unset) => return Ok(()),
            Some(IfcValue::Aggregate(values)) if !values.is_empty() => values,
            _ => return Err(invalid("analysis load cases require an explicit optional set")),
        };
        if values.len() > graph.source.entities.len() {
            return Err(invalid("analysis load case cardinality exceeds source population"));
        }
        state.cases = vector(values.len(), original)?;
        for value in values {
            work(original, size_of::<usize>())?;
            let row = graph.ordinal(reference(Some(value))?, original)?;
            if graph.source.entities[row].name != "IFCSTRUCTURALLOADCASE" || state.usage[row] != 0 {
                return Err(unsupported("analysis requires distinct direct structural load cases"));
            }
            state.usage[row] = 1;
            state.cases.push(row);
        }
        Ok(())
    })
}
fn unset(graph: &Graph<'_>, id: u64, name: &'static str, declared: &'static str, owner: &mut Owner<'_, '_>) -> Result<(), ValueError> {
    visit(graph, id, name, declared, owner, |value, _| if value == Some(&IfcValue::Unset) { Ok(()) } else { Err(unsupported("analytical action placement representation or destabilizing semantics require additional mapping")) })
}
fn validate_case(graph: &Graph<'_>, id: u64, owner: &mut Owner<'_, '_>) -> Result<bool, ValueError> {
    visit(graph, id, "PredefinedType", "IFCSTRUCTURALLOADGROUP", owner, |value, _| if value.and_then(IfcValue::as_enum) == Some("LOAD_CASE") { Ok(()) } else { Err(unsupported("direct LOAD_CASE group semantics required")) })?;
    visit(
        graph,
        id,
        "Coefficient",
        "IFCSTRUCTURALLOADGROUP",
        owner,
        |value, _| {
            if value == Some(&IfcValue::Unset) || number(value).is_ok_and(|v| v == 1.) {
                Ok(())
            } else {
                Err(unsupported("nonunit load case coefficient requires explicit scaling"))
            }
        },
    )?;
    visit(graph, id, "SelfWeightCoefficients", "IFCSTRUCTURALLOADCASE", owner, |value, original| {
        if value == Some(&IfcValue::Unset) {
            return Ok(false);
        }
        let values = list(value)?;
        if values.len() != 3 {
            return Err(invalid("self-weight coefficients require three declared components"));
        }
        work(original, 3 * size_of::<bool>() + size_of::<usize>())?;
        let (mut zero, mut downward) = (true, true);
        for (index, value) in values.iter().enumerate() {
            work(original, size_of::<f64>() + 2 * size_of::<bool>() + size_of::<usize>())?;
            let IfcValue::Real(value) = value else {
                return Err(invalid("self-weight coefficients require direct finite REAL ratios"));
            };
            if !value.is_finite() {
                return Err(invalid("self-weight coefficient must be finite"));
            }
            zero &= *value == 0.;
            downward &= *value == if index == 2 { -1. } else { 0. };
        }
        if !zero && !downward {
            return Err(unsupported("self-weight requires zero or exact global upright [0,0,-1] ratios"));
        }
        Ok(downward)
    })
}
fn case_actions(
    graph: &Graph<'_>,
    case: u64,
    stamp: usize,
    first: bool,
    seen: &mut [usize],
    usage: &mut [u8],
    owner: &mut Owner<'_, '_>,
    mut receive: impl FnMut(u64, u64, load_query::Component, f64, &mut Owner<'_, '_>) -> Result<(), ValueError>,
) -> Result<(), ValueError> {
    for relation in graph.rows(5, case) {
        work(owner, size_of::<Relation>())?;
        let assignment = &graph.source.entities[relation.index];
        if assignment.name != "IFCRELASSIGNSTOGROUP" {
            return Err(unsupported("factored load assignment requires explicit scaling"));
        }
        visit(graph, assignment.id, "RelatedObjects", "IFCRELASSIGNS", owner, |value, original| {
            let values = list(value)?;
            if values.is_empty() {
                return Err(invalid("load assignment requires an explicit nonempty object set"));
            }
            for value in values {
                work(original, 2 * size_of::<usize>())?;
                let row = graph.ordinal(reference(Some(value))?, original)?;
                let action = &graph.source.entities[row];
                if action.name != "IFCSTRUCTURALPOINTACTION" {
                    return Err(unsupported("native nodal projection requires direct structural point actions"));
                }
                if first {
                    if seen[row] == stamp {
                        return Err(invalid("load action repeats inside its case"));
                    }
                    seen[row] = stamp;
                    usage[row] |= 2;
                } else if seen[row] != stamp {
                    return Err(invalid("load action preparation no longer matches its original case"));
                }
                unset(graph, action.id, "ObjectPlacement", "IFCPRODUCT", original)?;
                unset(graph, action.id, "Representation", "IFCPRODUCT", original)?;
                unset(graph, action.id, "DestabilizingLoad", "IFCSTRUCTURALACTION", original)?;
                visit(
                    graph,
                    action.id,
                    "GlobalOrLocal",
                    "IFCSTRUCTURALACTIVITY",
                    original,
                    |value, _| {
                        if value.and_then(IfcValue::as_enum) == Some("GLOBAL_COORDS") {
                            Ok(())
                        } else {
                            Err(unsupported("native nodal action requires global coordinates"))
                        }
                    },
                )?;
                let links = graph.rows(6, action.id);
                if links.len() != 1 {
                    return Err(invalid("point action requires exactly one structural activity target"));
                }
                let link = &graph.source.entities[links[0].index];
                let target = reference_of(graph, link.id, "RelatingElement", "IFCRELCONNECTSSTRUCTURALACTIVITY", original)?;
                if graph.entity(target, original)?.name != "IFCSTRUCTURALPOINTCONNECTION" {
                    return Err(unsupported("point action target must be a mapped structural node"));
                }
                let load = reference_of(graph, action.id, "AppliedLoad", "IFCSTRUCTURALACTIVITY", original)?;
                let load_row = graph.ordinal(load, original)?;
                if graph.source.entities[load_row].name != "IFCSTRUCTURALLOADSINGLEFORCE" {
                    return Err(unsupported("point action requires a direct single-force load"));
                }
                if first {
                    work(original, size_of::<u8>())?;
                    usage[load_row] |= 4;
                }
                let mut present = false;
                for component in load_query::Component::ALL {
                    if let Some(value) = load_query::receive(graph.source, load_row, component, 0, original)? {
                        present = true;
                        receive(action.id, target, component, value, original)?;
                    }
                }
                if !present {
                    return Err(unsupported("point action requires at least one explicit force or moment component"));
                }
            }
            Ok(())
        })?;
    }
    Ok(())
}
const COMPONENT_KEY_STACK_BYTES: usize = 23 + 25 + 3 + 20 + 20 + 23 + 2;
fn component_id(id: u64, component: load_query::Component, owner: &mut Owner<'_, '_>) -> Result<String, ValueError> {
    work(owner, COMPONENT_KEY_STACK_BYTES)?;
    let mut buffer = [0; 25];
    let mut word = [0; 23];
    let name = key(id, &mut word);
    buffer[..name.len()].copy_from_slice(name.as_bytes());
    buffer[name.len()] = b'_';
    let rank = match component {
        load_query::Component::ForceX => 0,
        load_query::Component::ForceY => 1,
        load_query::Component::ForceZ => 2,
        load_query::Component::MomentX => 3,
        load_query::Component::MomentY => 4,
        load_query::Component::MomentZ => 5,
    };
    buffer[name.len() + 1] = b'0' + rank;
    copy(std::str::from_utf8(&buffer[..name.len() + 2]).expect("ASCII source component identity"), owner)
}
pub(super) fn receive(graph: &Graph<'_>, units: u64, state: &mut State, output: &mut Fem3dSnapshotValue, base: usize, owner: &mut Owner<'_, '_>) -> Result<(), ValueError> {
    output.load_cases = vector(state.cases.len(), owner)?;
    let mut rows = base.checked_add(state.cases.len()).ok_or_else(|| invalid("analytical output row extent overflow"))?;
    for rank in 0..state.cases.len() {
        let id = graph.source.entities[state.cases[rank]].id;
        let self_weight = validate_case(graph, id, owner)?;
        let mut count = 0usize;
        let State { seen, usage, force_checked, torque_checked, .. } = state;
        case_actions(graph, id, rank + 1, true, seen, usage, owner, |_, _, component, _, original| {
            let checked = match component.unit() {
                LoadUnit::Force => &mut *force_checked,
                LoadUnit::Torque => &mut *torque_checked,
            };
            if !*checked {
                graph.require_load_unit(graph.entity(units, original)?, component.unit(), original)?;
                work(original, size_of::<bool>())?;
                *checked = true;
            }
            work(original, size_of::<usize>())?;
            count = count.checked_add(1).ok_or_else(|| invalid("analytical load row extent overflow"))?;
            Ok(())
        })?;
        rows = rows.checked_add(count).ok_or_else(|| invalid("analytical load row extent overflow"))?;
        if rows > 64000 {
            return Err(unsupported("analytical destination exceeds retained row admission"));
        }
        work(owner, size_of::<FemLoadCase>())?;
        state.case = Some(FemLoadCase { id: String::new(), name: String::new(), loads: Vec::new(), self_weight });
        state.case.as_mut().unwrap().id = copy(key(id, &mut [0; 23]), owner)?;
        state.case.as_mut().unwrap().name = visit(graph, id, "Name", "IFCROOT", owner, |value, original| match value {
            Some(IfcValue::Unset) => copy("", original),
            Some(IfcValue::String(value)) => copy(value, original),
            _ => Err(invalid("load case name requires its declared optional text")),
        })?;
        state.case.as_mut().unwrap().loads = vector(count, owner)?;
        let State { seen, usage, case, load, .. } = state;
        case_actions(graph, id, rank + 1, false, seen, usage, owner, |action, target, component, value, original| {
            work(original, size_of::<FemLoad>())?;
            *load = Some(FemLoad::Nodal { id: String::new(), node_id: String::new(), dof: component.dof(), value });
            let Some(FemLoad::Nodal { id, node_id, .. }) = load.as_mut() else { unreachable!() };
            *id = component_id(action, component, original)?;
            *node_id = copy(key(target, &mut [0; 23]), original)?;
            work(original, size_of::<FemLoad>())?;
            case.as_mut().unwrap().loads.push(load.take().unwrap());
            Ok(())
        })?;
        work(owner, size_of::<FemLoadCase>())?;
        output.load_cases.push(state.case.take().unwrap());
    }
    for (row, entity) in graph.source.entities.iter().enumerate() {
        work(owner, size_of::<u8>())?;
        if matches!(entity.name.as_str(), "IFCSTRUCTURALLOADCASE" | "IFCSTRUCTURALPOINTACTION" | "IFCSTRUCTURALLOADSINGLEFORCE") && state.usage[row] == 0 {
            return Err(unsupported("unassigned structural load semantics require explicit selection"));
        }
    }
    Ok(())
}
#[cfg(test)]
#[path = "fem/nodal/native-import-tests.rs"]
mod tests;
