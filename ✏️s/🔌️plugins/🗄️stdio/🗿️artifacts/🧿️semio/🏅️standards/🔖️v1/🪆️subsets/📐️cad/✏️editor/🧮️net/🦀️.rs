//! 🧮️ Net of one snapshot edit as cad domain leaves: the delta between the edited snapshot and the base, expressed as the
//! concrete `cad` mutations that carry the base to it. A change the vocabulary cannot express yields leaves whose fold differs from
//! the edit, which the editor's publication check refuses.

use crate::standards::v1::subsets::base::schema::triples::net_keyed;
use crate::standards::v1::subsets::cad::schema::mutations::{add_block, add_block_entity, add_entity, add_layer, remove_block, remove_block_entity, remove_entity, remove_layer, set_block_base_point, set_block_entity_geometry, set_block_entity_layer, set_entity_geometry, set_entity_layer, set_layer, SemioCadMutation};
use crate::standards::v1::subsets::cad::schema::snapshot::SemioCadSnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn net(base: &SemioCadSnapshot, next: &SemioCadSnapshot) -> Vec<SemioCadMutation> {
    let layers = net_keyed(&base.layers, &next.layers, |layer| layer.name.clone());
    let blocks = net_keyed(&base.blocks, &next.blocks, |block| block.name.clone());
    let entities = net_keyed(&base.entities, &next.entities, |record| record.handle.clone());
    let mut out = Vec::new();
    out.extend(entities.removed.iter().map(|record| SemioCadMutation::RemoveEntity(remove_entity::RemoveEntity { handle: record.handle.clone() })));
    out.extend(blocks.removed.iter().map(|block| SemioCadMutation::RemoveBlock(remove_block::RemoveBlock { name: block.name.clone() })));
    out.extend(layers.added.iter().map(|layer| SemioCadMutation::AddLayer(add_layer::AddLayer { layer: (*layer).clone(), at: None })));
    for (before, after) in &layers.modified {
        out.push(SemioCadMutation::SetLayer(set_layer::SetLayer {
            name: after.name.clone(),
            color_index: (before.color_index != after.color_index).then_some(after.color_index),
            line_type: (before.line_type != after.line_type).then(|| after.line_type.clone()),
            visible: (before.visible != after.visible).then_some(after.visible),
        }));
    }
    out.extend(blocks.added.iter().map(|block| SemioCadMutation::AddBlock(add_block::AddBlock { block: (*block).clone(), at: None })));
    for (before, after) in &blocks.modified {
        if before.base_point != after.base_point {
            out.push(SemioCadMutation::SetBlockBasePoint(set_block_base_point::SetBlockBasePoint { name: after.name.clone(), base_point: after.base_point }));
        }
        let inner = net_keyed(&before.entities, &after.entities, |record| record.handle.clone());
        out.extend(inner.removed.iter().map(|record| SemioCadMutation::RemoveBlockEntity(remove_block_entity::RemoveBlockEntity { block_name: after.name.clone(), handle: record.handle.clone() })));
        for (old, new) in &inner.modified {
            if old.layer != new.layer {
                out.push(SemioCadMutation::SetBlockEntityLayer(set_block_entity_layer::SetBlockEntityLayer { block_name: after.name.clone(), handle: new.handle.clone(), layer: new.layer.clone() }));
            }
            if old.entity != new.entity {
                out.push(SemioCadMutation::SetBlockEntityGeometry(set_block_entity_geometry::SetBlockEntityGeometry { block_name: after.name.clone(), handle: new.handle.clone(), entity: new.entity.clone() }));
            }
        }
        out.extend(inner.added.iter().map(|record| SemioCadMutation::AddBlockEntity(add_block_entity::AddBlockEntity { block_name: after.name.clone(), entity: (*record).clone(), at: None })));
    }
    out.extend(entities.added.iter().map(|record| SemioCadMutation::AddEntity(add_entity::AddEntity { entity: (*record).clone(), at: None })));
    for (before, after) in &entities.modified {
        if before.layer != after.layer {
            out.push(SemioCadMutation::SetEntityLayer(set_entity_layer::SetEntityLayer { handle: after.handle.clone(), layer: after.layer.clone() }));
        }
        if before.entity != after.entity {
            out.push(SemioCadMutation::SetEntityGeometry(set_entity_geometry::SetEntityGeometry { handle: after.handle.clone(), entity: after.entity.clone() }));
        }
    }
    out.extend(layers.removed.iter().map(|layer| SemioCadMutation::RemoveLayer(remove_layer::RemoveLayer { name: layer.name.clone() })));
    out
}
