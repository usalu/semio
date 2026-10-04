//! 🖱️ Selection requests join the framework interaction lane after the new layer is published.
use super::{RASTER_INTERACTION_DOMAIN,RASTER_INTERACTION_GRANULARITY};
use semio_framework_plugin::{Fault,RequestId,INTERACTION_SELECT_ACTION_ID};

/// 🌳️ Gives framework selection validation the current layer membership and nesting.
pub fn layer_topology(layers:&[crate::RasterLayerNode])->semio_framework_plugin::DomainTopology {
    fn visit(layers:&[crate::RasterLayerNode],parent:Option<&str>,ordered:&mut Vec<semio_framework_plugin::TopologyNode>) {
        for layer in layers {
            let id=crate::standards::v1::subsets::any::schema::layer_node_id(layer);
            ordered.push(semio_framework_plugin::TopologyNode {id:id.into(),granularity:RASTER_INTERACTION_GRANULARITY.into(),parent:parent.map(str::to_owned)});
            if let crate::RasterLayerNode::Group {children,..}=layer {visit(children,Some(id),ordered);}
        }
    }
    let mut ordered=Vec::new();visit(layers,None,&mut ordered);
    semio_framework_plugin::DomainTopology {ordered}
}

pub fn layer_selection_args(id:&str)->Result<semio_framework_value::DslValue,Fault> {
    if id.is_empty(){return Err(Fault::from("Layer selection requires an identity"));}
    let target=semio_framework_pack_json::object([("granularity".into(),semio_framework_pack_json::Value::String(RASTER_INTERACTION_GRANULARITY.into())),("id".into(),semio_framework_pack_json::Value::String(id.into()))]);
    let targets=semio_framework_pack_json::to_string(&semio_framework_pack_json::array([target]));
    let args=semio_framework_pack_json::object([("domainId".into(),semio_framework_pack_json::Value::String(RASTER_INTERACTION_DOMAIN.into())),("targets".into(),semio_framework_pack_json::Value::String(targets)),("merge".into(),semio_framework_pack_json::Value::String("replace".into())),("method".into(),semio_framework_pack_json::Value::String("pick".into()))]);
    Ok(semio_framework_pack_json::to_dsl_value(&args))
}

pub fn select_layer_effect(id:&str)->Result<semio_framework::kernel::Effect,Fault> {
    Ok(semio_framework::kernel::Effect::DispatchAction {req:RequestId(0),action:INTERACTION_SELECT_ACTION_ID.into(),args:Some(layer_selection_args(id)?),delay_ms:0})
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
