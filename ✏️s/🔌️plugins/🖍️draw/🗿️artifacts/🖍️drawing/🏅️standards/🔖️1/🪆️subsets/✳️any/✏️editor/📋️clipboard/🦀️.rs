//! 📋️ Typed selection fragments reuse framework clipboard effects and semantic mutations.
use crate::{DrawingSnapshot,DrawingLayerNode,DrawingImageAsset};
use crate::schema::{layer_base,layer_base_mut,find_drawing_layer,find_drawing_layer_location};
use crate::mutations::DrawingMutation;
use semio_framework_plugin::{MediaType,MediaClass,MediaForm};
use semio_framework_plugin::kernel::{ClipboardFragment,ClipboardError,PastePlacement,PasteAnchor};
use std::collections::{BTreeMap,BTreeSet};
pub const SCHEMA:&str="drawing.clipboard.v1";
const MAX_BYTES:usize=semio_framework_plugin::kernel::CLIPBOARD_TEXT_MAX_BYTES;
const MAX_NODES:usize=1024;
const MAX_DEPTH:usize=32;
const MAX_ID_BYTES:usize=4096;
const MAX_TASKS:usize=MAX_NODES*17+3;
#[path="🛫️encode/🦀️.rs"]
mod encode;
#[path="🛬️decode/🦀️.rs"]
mod decode;
#[path="🧵️job/🦀️.rs"]
pub(crate) mod job;
#[path="🖼️assets/🦀️.rs"]
mod assets;
pub use assets::DrawingClipboardAssets;
#[derive(Clone,Debug,PartialEq,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetainedClone,semio_framework_value::RetireOwned)]
#[value(rename_all="camelCase")]
#[cfg_attr(test,derive(serde::Serialize,serde::Deserialize))]
pub struct DrawingClipboard {pub schema:String,pub roots:Vec<DrawingLayerNode>,pub selected:Vec<String>,pub assets:DrawingClipboardAssets}
pub fn media_type()->MediaType {MediaType {class:MediaClass::TwoD,form:MediaForm::Design}}
fn invalid(message:impl Into<String>)->ClipboardError {ClipboardError::ParseFailed(message.into())}
fn world(document:&DrawingSnapshot,id:&str)->Result<[f64;6],ClipboardError> {
    let mut transforms=Vec::new();let mut current=Some(id.to_owned());
    while let Some(id)=current {
        if transforms.len()>=32 {return Err(invalid("Clipboard ancestry exceeds capacity"));}
        let node=find_drawing_layer(document,id.as_str()).ok_or_else(||invalid("Selected layer no longer exists / Ausgewählte Ebene existiert nicht mehr"))?;
        transforms.push(crate::schema::drawing_transform_to_matrix(&layer_base(node).transform));
        current=find_drawing_layer_location(document,id.as_str()).and_then(|location|location.parent_id.map(|id|id.to_string_owner()));
    }
    Ok(transforms.into_iter().rev().fold([1.0,0.0,0.0,1.0,0.0,0.0],crate::schema::geometry::multiply))
}
fn visit<'a>(roots:&'a [DrawingLayerNode],output:&mut Vec<&'a DrawingLayerNode>){for node in roots {output.push(node);if let DrawingLayerNode::Group(group)=node {for child in &group.children {visit(std::slice::from_ref(child),output);}}}}
fn layer_ids(roots:&[DrawingLayerNode])->BTreeSet<String> {let mut nodes=Vec::new();visit(roots,&mut nodes);nodes.into_iter().map(|node|layer_base(node).id.to_string_owner()).collect()}
fn roots(document:&DrawingSnapshot,ids:&[String])->Result<Vec<String>,ClipboardError> {
    if ids.is_empty(){return Err(ClipboardError::EmptySelection);}
    let wanted=ids.iter().collect::<BTreeSet<_>>();
    let mut result=Vec::new();
    for id in ids {
        find_drawing_layer(document,id).ok_or_else(||invalid("Selected layer no longer exists / Ausgewählte Ebene existiert nicht mehr"))?;
        let mut parent=find_drawing_layer_location(document,id).and_then(|location|location.parent_id);
        let mut contained=false;
        while let Some(id)=parent {if wanted.contains(&id.to_string_owner()){contained=true;break;}parent=find_drawing_layer_location(document,&id).and_then(|location|location.parent_id);}
        if !contained && !result.contains(id){result.push(id.clone());}
    }
    Ok(result)
}
pub fn copy(document:&DrawingSnapshot,ids:&[String])->Result<ClipboardFragment,ClipboardError> {
    let selected=roots(document,ids)?;let mut wanted=selected.clone();let mut at=0;
    while at<wanted.len() {
        if wanted.len()>1024 {return Err(invalid("Clipboard dependency capacity exceeded"));}
        let node=find_drawing_layer(document,wanted[at].as_str()).ok_or_else(||invalid("Clipboard dependency no longer exists"))?;
        let mut nodes=Vec::new();visit(std::slice::from_ref(node),&mut nodes);
        for node in nodes {if let DrawingLayerNode::Boolean(boolean)=node {for child in &boolean.children {let id=child.to_string_owner();if !wanted.contains(&id){wanted.push(id);}}}}
        at+=1;
    }
    let wanted=roots(document,&wanted)?;
    let mut packet=DrawingClipboard {schema:SCHEMA.into(),roots:Vec::new(),selected,assets:Default::default()};
    for id in wanted {
        let mut node=find_drawing_layer(document,id.as_str()).ok_or_else(||invalid("Clipboard layer no longer exists"))?.clone();
        let mut matrix=world(document,&id)?;
        if matches!(node,DrawingLayerNode::Boolean(_)) {
            let parent=find_drawing_layer_location(document,&id).and_then(|location|location.parent_id).map(|id|world(document,&id.to_string_owner())).transpose()?.unwrap_or([1.0,0.0,0.0,1.0,0.0,0.0]);
            matrix=crate::schema::geometry::multiply(matrix,invert(parent)?);
        }
        layer_base_mut(&mut node).transform=crate::schema::geometry::affine::drawing_matrix_to_transform(matrix);
        packet.roots.push(node);
    }
    let mut nodes=Vec::new();visit(&packet.roots,&mut nodes);
    for node in nodes {
        let asset=match node {DrawingLayerNode::Image(image)=>Some(&image.image_key),DrawingLayerNode::Trace(trace)=>Some(&trace.source_key),_=>None};
        if let Some(id)=asset {packet.assets.insert(id.to_string_owner(),document.assets.get(id).ok_or_else(||invalid("Copied image asset no longer exists"))?.clone());}
    }
    let text=semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&packet));
    if text.len()>MAX_BYTES {return Err(invalid("Clipboard fragment exceeds its admitted byte capacity"));}
    Ok(ClipboardFragment {schema:SCHEMA.into(),media_type:media_type(),dsl_text:text,pack_bytes:None,source_app:super::DRAWING_PLAY_CONTROLLER_ID.into(),label:format!("{} layers / Ebenen",packet.selected.len())})
}
pub fn decode(fragment:&ClipboardFragment)->Result<DrawingClipboard,ClipboardError> {
    if fragment.media_type!=media_type(){return Err(ClipboardError::IncompatibleMediaType(fragment.media_type.clone()));}
    if fragment.schema!=SCHEMA || fragment.dsl_text.len()>MAX_BYTES {return Err(invalid("Clipboard schema or byte capacity is invalid"));}
    let packet:DrawingClipboard=semio_framework_pack_json::from_json_str(&fragment.dsl_text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|invalid(error.to_string()))?;
    validate(&packet)?;
    Ok(packet)
}
fn validate(packet:&DrawingClipboard)->Result<(),ClipboardError> {
    if packet.schema!=SCHEMA || packet.roots.is_empty() || packet.roots.len()>MAX_NODES || packet.assets.len()>MAX_NODES || packet.selected.is_empty() || packet.selected.len()>MAX_NODES{return Err(invalid("Invalid drawing clipboard packet"));}
    let mut nodes=BTreeMap::new();let mut stack=packet.roots.iter().map(|node|(node,1usize)).collect::<Vec<_>>();
    while let Some((node,depth))=stack.pop(){let base=layer_base(node);if depth>MAX_DEPTH||base.id.is_empty()||base.id.len()>MAX_ID_BYTES||nodes.len()==MAX_NODES{return Err(invalid("Invalid clipboard ancestry or identity"));}let id=base.id.to_string_owner();if nodes.insert(id,node).is_some(){return Err(invalid("Clipboard identities must be unique"));}if let DrawingLayerNode::Group(group)=node{if group.children.len()>MAX_NODES{return Err(invalid("Clipboard group exceeds capacity"));}for child in &group.children{stack.push((child,depth+1));}}}
    if packet.selected.iter().any(|id|id.is_empty()||id.len()>MAX_ID_BYTES||!nodes.contains_key(id)) || packet.selected.iter().collect::<BTreeSet<_>>().len()!=packet.selected.len(){return Err(invalid("Clipboard identities must be unique and complete"));}
    for node in nodes.values() {
        if let DrawingLayerNode::Boolean(boolean)=node {if boolean.children.len()>MAX_NODES||boolean.operation.len()>16||!matches!(boolean.operation.to_string_owner().as_str(),"union"|"difference"|"intersection"|"xor")||boolean.children.iter().any(|id|id.is_empty()||id.len()>MAX_ID_BYTES||!nodes.contains_key(&id.to_string_owner())){return Err(invalid("Clipboard Boolean references must be included"));}}
        let asset=match node {DrawingLayerNode::Image(image)=>Some(&image.image_key),DrawingLayerNode::Trace(trace)=>Some(&trace.source_key),_=>None};
        if asset.is_some_and(|id|!packet.assets.contains_key(&id.to_string_owner())){return Err(invalid("Clipboard image assets must be included"));}
    }
    let mut active=BTreeSet::new();let mut done=BTreeSet::new();
    for root in nodes.keys(){if done.contains(root){continue;}let mut stack=vec![(root.clone(),0usize)];active.insert(root.clone());
        while let Some((id,edge))=stack.last_mut(){let node=*nodes.get(id.as_str()).ok_or_else(||invalid("Clipboard reference is missing"))?;let next=match node{DrawingLayerNode::Boolean(boolean)=>boolean.children.get(*edge).map(|id|id.to_string_owner()),DrawingLayerNode::Group(group)=>group.children.get(*edge).map(|node|layer_base(node).id.to_string_owner()),_=>None};
            if let Some(next)=next{*edge+=1;if active.contains(&next){return Err(invalid("Clipboard Boolean dependencies contain a cycle / Booleanabhängigkeiten der Zwischenablage enthalten einen Zyklus"));}if done.contains(&next){continue;}active.insert(next.clone());stack.push((next,0));}
            else{let (id,_)=stack.pop().unwrap();active.remove(&id);done.insert(id);}
        }
    }
    for (id,asset) in &packet.assets {if id.is_empty() || id.len()>MAX_ID_BYTES || asset.width==0 || asset.height==0 || (asset.width as usize).checked_mul(asset.height as usize)!=Some(asset.samples.len()){return Err(invalid("Clipboard image dimensions and samples are invalid / Bildmaße und Bilddaten der Zwischenablage sind ungültig"));}}
    Ok(())
}
pub fn cut(document:&DrawingSnapshot,ids:&[String])->Result<Vec<DrawingMutation>,ClipboardError> {
    let selected=roots(document,ids)?;
    let mut removed=BTreeSet::new();
    for id in &selected {
        if crate::schema::drawing_layer_is_locked(document,id){return Err(invalid("Unlock selected layers before cutting / Ausgewählte Ebenen vor dem Ausschneiden entsperren"));}
        let node=find_drawing_layer(document,id).ok_or_else(||invalid("Selected layer no longer exists"))?;
        let mut descendants=Vec::new();visit(std::slice::from_ref(node),&mut descendants);
        if descendants.iter().any(|node|layer_base(node).locked){return Err(invalid("Unlock selected descendants before cutting / Ausgewählte Unterebenen vor dem Ausschneiden entsperren"));}
        removed.extend(layer_ids(std::slice::from_ref(node)));
    }
    for node in crate::schema::flatten_drawing_layers(&document.layers) {if !removed.contains(&layer_base(node).id.to_string_owner()) {if let DrawingLayerNode::Boolean(boolean)=node {if boolean.children.iter().any(|id|removed.contains(&id.to_string_owner())){return Err(invalid("A Boolean outside the selection still uses these layers / Ein Boolean außerhalb der Auswahl verwendet diese Ebenen"));}}}}
    Ok(selected.iter().map(|id|crate::mutations::delete_layer(id.as_str().into())).collect())
}
pub fn paste(document:&DrawingSnapshot,mut packet:DrawingClipboard,placement:&PastePlacement,parent_id:Option<&str>,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(Vec<DrawingMutation>,Vec<String>),ClipboardError> {
    plan_paste(document,&mut packet,placement,parent_id,control)
}
fn plan_paste(document:&DrawingSnapshot,packet:&mut DrawingClipboard,placement:&PastePlacement,parent_id:Option<&str>,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(Vec<DrawingMutation>,Vec<String>),ClipboardError> {
    validate(&packet)?;
    let destination=if let Some(parent)=parent_id {
        let Some(DrawingLayerNode::Group(_))=find_drawing_layer(document,parent) else {return Err(invalid("Paste destination must be a group"));};
        if crate::schema::drawing_layer_is_locked(document,parent){return Err(invalid("Unlock the destination group before pasting / Zielgruppe vor dem Einfügen entsperren"));}
        world(document,parent)?
    }else{[1.0,0.0,0.0,1.0,0.0,0.0]};
    let inverse=invert(destination)?;
    let occupied=crate::schema::flatten_drawing_layers(&document.layers).into_iter().map(|node|layer_base(node).id.to_string_owner()).collect::<BTreeSet<_>>();
    let source_ids=layer_ids(&packet.roots);
    let mut ordinal=0usize;
    let identity_map=loop{
        let mapping=source_ids.iter().map(|id|paste_identity(crate::schema::identity::DrawingIdentityKind::Layer,&document.id,id,ordinal,control).map(|target|(id.clone(),target))).collect::<Result<BTreeMap<_,_>,_>>()?;
        if mapping.values().all(|id|!occupied.contains(id))&&mapping.values().collect::<BTreeSet<_>>().len()==mapping.len(){break mapping;}
        ordinal=ordinal.checked_add(1).ok_or_else(||invalid("Unable to assign clipboard identities"))?;
    };
    let mut mutations=Vec::new();let mut asset_map=BTreeMap::new();let mut asset_ids=document.assets.iter().map(|(id,_)|id.to_string_owner()).collect::<BTreeSet<_>>();
    let mut bounds=None::<[f64;4]>;
    if !matches!(placement.anchor,PasteAnchor::Original){for node in &packet.roots {if let Some((x,y,w,h))=crate::schema::drawing_layer_world_bounds(node) {bounds=Some(match bounds {None=>[x,y,x+w,y+h],Some([left,top,right,bottom])=>[left.min(x),top.min(y),right.max(x+w),bottom.max(y+h)]});}}}
    let [left,top,right,bottom]=bounds.unwrap_or([0.0;4]);
    let anchor=match placement.anchor {PasteAnchor::Original=>[0.0,0.0],PasteAnchor::Middle|PasteAnchor::Centroid=>[(left+right)/2.0,(top+bottom)/2.0],PasteAnchor::BottomLeft=>[left,bottom],PasteAnchor::BottomRight=>[right,bottom],PasteAnchor::TopLeft=>[left,top],PasteAnchor::TopRight=>[right,top]};
    let offset=placement.position.map(|position|[position[0]-anchor[0],position[1]-anchor[1]]).unwrap_or([16.0,16.0]);
    if !offset.iter().all(|n|n.is_finite()){return Err(invalid("Paste placement must be finite"));}
    for (id,asset) in std::mem::take(&mut packet.assets) {
        let mapped={
            let mut ordinal=0usize;let mapped=loop{let target=paste_identity(crate::schema::identity::DrawingIdentityKind::ImageAsset,&document.id,&id,ordinal,control)?;if !asset_ids.contains(&target){break target;}ordinal=ordinal.checked_add(1).ok_or_else(||invalid("Unable to assign image identity"))?;};
            asset_ids.insert(mapped.clone());mutations.push(crate::mutations::import_image_asset(mapped.clone(),asset));mapped
        };asset_map.insert(id,mapped);
    }
    fn remap(node:&mut DrawingLayerNode,ids:&BTreeMap<String,String>,assets:&BTreeMap<String,String>) {
        layer_base_mut(node).id=ids[&layer_base(node).id.to_string_owner()].as_str().into();
        match node {
            DrawingLayerNode::Boolean(boolean)=>for child in &mut boolean.children {*child=ids[&child.to_string_owner()].as_str().into();},
            DrawingLayerNode::Image(image)=>image.image_key=assets[&image.image_key.to_string_owner()].as_str().into(),
            DrawingLayerNode::Trace(trace)=>trace.source_key=assets[&trace.source_key.to_string_owner()].as_str().into(),
            DrawingLayerNode::Group(group)=>for child in &mut group.children {remap(child,ids,assets);},_=>{}
        }
    }
    for mut node in packet.roots.drain(..) {
        let matrix=crate::schema::drawing_transform_to_matrix(&layer_base(&node).transform);
        let matrix=crate::schema::geometry::multiply([1.0,0.0,0.0,1.0,offset[0],offset[1]],matrix);
        let mut matrix=crate::schema::geometry::multiply(inverse,matrix);
        if matches!(node,DrawingLayerNode::Boolean(_)){matrix=crate::schema::geometry::multiply(matrix,destination);}
        layer_base_mut(&mut node).transform=crate::schema::geometry::affine::drawing_matrix_to_transform(matrix);
        remap(&mut node,&identity_map,&asset_map);mutations.push(crate::mutations::create_layer(parent_id.map(Into::into),None,node));
    }
    Ok((mutations,packet.selected.iter().map(|id|identity_map[id].clone()).collect()))
}
fn invert(matrix:[f64;6])->Result<[f64;6],ClipboardError>{
    let [a,b,c,d,x,y]=matrix;let determinant=a*d-b*c;
    if determinant==0.0 || !matrix.iter().all(|n|n.is_finite()){return Err(invalid("Clipboard transform is singular or nonfinite / Zwischenablagetransformation ist singulär oder nicht endlich"));}
    Ok([d/determinant,-b/determinant,-c/determinant,a/determinant,(c*y-d*x)/determinant,(b*x-a*y)/determinant])
}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

fn paste_identity(kind:crate::schema::identity::DrawingIdentityKind,document:&semio_framework_value::paged::PagedUtf8<{usize::MAX}>,source:&str,ordinal:usize,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<String,ClipboardError>{
 let mut material=[0u8;4096];let mut output=[0u8;76];
 let text=crate::standards::v1::subsets::any::io::text::identity::paste::prepare_paste_identity(kind,document,source,ordinal,&mut material,&mut output,control).map_err(|error|invalid(&error.to_string()))?;control.copy_text(text).map_err(|error|invalid(&error.to_string()))
}
