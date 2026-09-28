use super::*;
use crate::standards::v1::subsets::any::schema::{create_pixel_layer,layer_node_id,snapshot::retire_raster_snapshot};
#[semio_framework_async_macros::async_test]
async fn completed_selection_checks_revision_extent_and_visibility_without_requiring_an_unlocked_layer(){
    let mut doc=crate::RasterSnapshot::default();doc.layers.push(create_pixel_layer("Paint",3,2));
    let selection=RasterPixelSelection{layer_id:layer_node_id(&doc.layers[0]).into(),target:"pixels".into(),width:3,height:2,spans:"[]".into()};
    assert!(selection.validate_current(&doc,None).is_ok());
    assert!(selection.validate_current(&doc,Some("stale")).is_err());
    if let crate::RasterLayerNode::Pixel{locked,..}=&mut doc.layers[0]{*locked=true;}
    assert!(selection.validate_current(&doc,None).is_ok());
    if let crate::RasterLayerNode::Pixel{visible,..}=&mut doc.layers[0]{*visible=false;}
    assert!(selection.validate_current(&doc,None).is_err());
    if let crate::RasterLayerNode::Pixel{visible,width,..}=&mut doc.layers[0]{*visible=true;*width=Some(4);}
    assert!(selection.validate_current(&doc,None).is_err());
    retire_raster_snapshot(doc);
}
