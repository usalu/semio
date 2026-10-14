//! ↩️ `create-layer` inverse — `delete-layer` addressed by the created layer's own id (no `base`
//! lookup needed, the id is already on the payload).

use crate::delete_layer;
use semio_s_artifact_raster_raster_model::{find_layer, layer_node_id};
use semio_s_artifact_raster_raster_model::RasterLayerNode;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::delete_layer::mutation::DeleteLayer>>(payload: &crate::create_layer::mutation::CreateLayer, base: &RasterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let new_id = layer_node_id(&payload.layer);
    // 🚫️ A REFUSED create applied NOTHING — `🔺️diff` faults on a duplicate id or a non-group parent
    // and LAW 1 forces `MutationOutcome::fatal` to carry the empty diff — so its inverse must delete
    // nothing. Inverting it unconditionally deleted the very layer whose id blocked the insertion:
    // pressing undo after a refused create destroyed an existing layer (caught by
    // `every_variant_round_trips_via_inverse`, ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP).
    if find_layer(&base.layers, new_id).is_some() {
        return Vec::new();
    }
    if let Some(parent_id) = payload.parent_id.as_deref() {
        if !matches!(find_layer(&base.layers, parent_id), Some(RasterLayerNode::Group { .. })) {
            return Vec::new();
        }
    }
    vec![Op::from(delete_layer::DeleteLayer { layer_id: new_id.to_string() })]

    })())
}
//#endregion 🔖️Inverse
