//! ↩️ Asset attachment keeps one inverse row and retains an existing member's exact image root.
use semio_s_artifact_raster_raster_model::{snapshot::RasterSnapshot,RasterImagePayload,SemioImageSnapshot};

pub fn inverse<Op:From<crate::add_layer_asset::mutation::AddLayerAsset>+From<crate::remove_layer_asset::mutation::RemoveLayerAsset>>(payload:&crate::add_layer_asset::mutation::AddLayerAsset,base:&RasterSnapshot)->Result<Vec<Op>,semio_framework_value::ValueError>{
    let Some(child)=base.assets.get(&payload.asset_id)else{return Ok(vec![Op::from(crate::remove_layer_asset::mutation::RemoveLayerAsset{asset_id:payload.asset_id.clone()})]);};
    let owner=child.local_owner::<SemioImageSnapshot>().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"raster.asset-owner-unavailable"))?;
    let asset=<RasterImagePayload as semio_framework_value::owned::NativeReceiveMap<std::sync::Arc<SemioImageSnapshot>>>::from_native(owner);
    Ok(vec![Op::from(crate::add_layer_asset::mutation::AddLayerAsset{asset_id:payload.asset_id.clone(),asset})])
}
