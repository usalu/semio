//! ↩️ Inverse for `DragNodes`.

use crate::schema::mutations::move_node::MoveNode;
use crate::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
/// ↩️ The exact undo: one absolute `move-node` per moved node carrying its BASE position — never a negated offset.
pub fn inverse<Op>(payload: &super::DragNodes, base: &SemioGraphSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::move_node::MoveNode>,
{
    Ok((|| {
    if payload.targets.is_empty() || !(payload.dx.is_finite() && payload.dy.is_finite()) || (payload.dx, payload.dy) == (0.0, 0.0) {
        return Vec::new();
    }
    let mut seen = std::collections::HashSet::new();
    payload
        .targets
        .iter()
        .filter(|id| seen.insert(id.value.as_str()))
        .filter_map(|id| base.nodes.iter().find(|node| node.id == *id))
        .map(|node| Op::from(MoveNode { id: node.id.clone(), new_position: node.position }))
        .collect()

    })())
}
//#endregion 🔖️Inverse
