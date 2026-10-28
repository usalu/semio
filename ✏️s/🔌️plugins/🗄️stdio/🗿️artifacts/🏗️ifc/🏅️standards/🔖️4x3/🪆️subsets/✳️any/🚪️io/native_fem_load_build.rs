//! 🏋️ Nodal cases append owned analytical rows through their original graph receiving helpers.
use super::*;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::{FemLoad, FemLoadCase};
#[derive(Default, RetireOwned)]
struct Loads {
    cases: Vec<IfcValue>,
    actions: Option<Vec<IfcValue>>,
}
pub(super) fn extent(source: &Fem3dSnapshot, owner: &mut Owner<'_, '_>) -> Result<usize, ValueError> {
    let mut count = 0usize;
    for case in &source.load_cases {
        work(owner, size_of::<FemLoadCase>())?;
        count = count.checked_add(2).ok_or_else(|| invalid("analytical load case extent overflow"))?;
        for load in &case.loads {
            work(owner, size_of::<FemLoad>())?;
            match load {
                FemLoad::Nodal { value, .. } if value.is_finite() => {}
                FemLoad::Nodal { .. } => return Err(invalid("nodal load requires a finite binary64 value")),
                _ => return Err(unsupported("native nodal export requires concentrated global point loads")),
            }
            count = count.checked_add(3).ok_or_else(|| invalid("analytical load extent overflow"))?;
        }
    }
    Ok(count)
}
pub(super) fn receive(source: &Fem3dSnapshot, identity: &FemIdentityIndex, nodes: &[(u64, u64)], graph: &mut IfcSnapshot, owner: &mut Owner<'_, '_>) -> Result<Vec<IfcValue>, ValueError> {
    owner.receive_nested::<Loads, Vec<IfcValue>>(|slot, original| {
        work(original, size_of::<Loads>())?;
        *slot = Some(Loads::default());
        let frame = slot.as_mut().unwrap();
        frame.cases = vector(source.load_cases.len(), original)?;
        for case in &source.load_cases {
            work(original, size_of::<FemLoadCase>())?;
            work(original, 8 * size_of::<V<'_>>())?;
            const GRAVITY: [V<'static>; 3] = [V::R(0.), V::R(0.), V::R(-1.)];
            let weight = if case.self_weight { V::A(&GRAVITY) } else { V::U };
            let group = root(graph, "IFCSTRUCTURALLOADCASE", &case.name, &[V::U, V::E("LOAD_CASE"), V::E("NOTDEFINED"), V::E("NOTDEFINED"), V::R(1.), V::U, weight], original)?;
            work(original, size_of::<IfcValue>())?;
            frame.cases.push(IfcValue::Reference(group));
            if case.loads.is_empty() {
                continue;
            }
            frame.actions = Some(vector(case.loads.len(), original)?);
            for load in &case.loads {
                work(original, size_of::<FemLoad>())?;
                let FemLoad::Nodal { id, node_id, dof, value } = load else {
                    return Err(unsupported("native nodal export requires concentrated global point loads"));
                };
                let target = nodes.get(node(source, identity, node_id, original)?).ok_or_else(|| invalid("nodal load target row is absent"))?.0;
                let ordinal = match dof {
                    FemDof::Tx => 0,
                    FemDof::Ty => 1,
                    FemDof::Tz => 2,
                    FemDof::Rx => 3,
                    FemDof::Ry => 4,
                    FemDof::Rz => 5,
                };
                let mut components = [V::U, V::U, V::U, V::U, V::U, V::U];
                components[ordinal] = V::R(*value);
                let force = add(graph, "IFCSTRUCTURALLOADSINGLEFORCE", &[V::S(id), components[0], components[1], components[2], components[3], components[4], components[5]], original)?;
                let action = root(graph, "IFCSTRUCTURALPOINTACTION", id, &[V::U, V::U, V::U, V::Ref(force), V::E("GLOBAL_COORDS"), V::U], original)?;
                root(graph, "IFCRELCONNECTSSTRUCTURALACTIVITY", "", &[V::Ref(target), V::Ref(action)], original)?;
                work(original, size_of::<IfcValue>())?;
                frame.actions.as_mut().unwrap().push(IfcValue::Reference(action));
            }
            root_with_objects(graph, group, &mut frame.actions, original)?;
        }
        work(original, size_of::<Vec<IfcValue>>())?;
        Ok(std::mem::take(&mut frame.cases))
    })
}
