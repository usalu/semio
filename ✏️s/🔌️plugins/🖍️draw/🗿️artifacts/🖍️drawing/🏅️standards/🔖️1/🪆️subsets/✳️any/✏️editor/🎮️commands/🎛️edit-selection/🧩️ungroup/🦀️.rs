//! 🧩️ Atomic promotion of selected groups' children into their parent stack.
use crate::{DrawingLayerNode,DrawingSnapshot};
use crate::schema::{layer_base,find_drawing_layer,find_drawing_layer_location,selected_drawing_layers,drawing_layer_is_locked};
use crate::mutations::DrawingMutation;
use semio_framework_plugin::Fault;

pub fn plan(document:&DrawingSnapshot,ids:&[String])->Result<(Vec<DrawingMutation>,Vec<String>),Fault> {
    let selected=selected_drawing_layers(document,ids);
    if selected.is_empty() || selected.len()!=ids.iter().collect::<std::collections::BTreeSet<_>>().len() {return Err(Fault::from("A selected group no longer exists"));}
    for layer in &selected {
        let DrawingLayerNode::Group(group)=layer else {return Err(Fault::from("Select groups to ungroup"));};
        if drawing_layer_is_locked(document,&group.base.id) {return Err(Fault::from("Unlock selected groups before ungrouping"));}
        if group.isolation || group.base.opacity!=1.0 || group.base.blend_mode!="normal" {return Err(Fault::from("Group compositing must be resolved before ungrouping"));}
    }
    let wanted=selected.iter().map(|layer|&layer_base(layer).id).collect::<std::collections::BTreeSet<_>>();
    fn select(layer:&DrawingLayerNode,wanted:&std::collections::BTreeSet<&semio_framework_value::paged::PagedUtf8<{usize::MAX}>>,seen:&mut std::collections::BTreeSet<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>,selection:&mut Vec<String>) {
        if wanted.contains(&layer_base(layer).id) {
            if let DrawingLayerNode::Group(group)=layer {for child in &group.children {select(child,wanted,seen,selection);}}
        }else if seen.insert(layer_base(layer).id.clone()) {selection.push(layer_base(layer).id.to_string_owner());}
    }
    let mut selection=Vec::new();
    let mut seen=std::collections::BTreeSet::new();
    for layer in &selected {select(layer,&wanted,&mut seen,&mut selection);}
    let mut working=document.clone();
    let mut mutations=Vec::new();
    let append=|working:&mut DrawingSnapshot,mutations:&mut Vec<DrawingMutation>,mutation:DrawingMutation|->Result<(),Fault> {
        crate::standards::v1::subsets::any::io::text::mutations::apply_drawing_mutation(working,&mutation).map_err(|error|Fault::from(error.to_string()))?;
        mutations.push(mutation);Ok(())
    };
    for layer in selected.into_iter().rev() {
        let id=&layer_base(layer).id;
        let location=find_drawing_layer_location(&working,id).ok_or_else(||Fault::from("A selected group no longer exists"))?;
        let Some(DrawingLayerNode::Group(group))=find_drawing_layer(&working,id).cloned() else {return Err(Fault::from("A selected group no longer exists"));};
        let matrix=crate::schema::drawing_transform_to_matrix(&group.base.transform);
        for (index,child) in group.children.iter().enumerate() {
            let base=layer_base(child);
            let combined=crate::schema::geometry::multiply(matrix,crate::schema::drawing_transform_to_matrix(&base.transform));
            if !combined.iter().all(|v|v.is_finite()) {return Err(Fault::from("Ungrouping produced a nonfinite transform"));}
            let transform=crate::schema::geometry::affine::drawing_matrix_to_transform(combined);
            if ![transform.x,transform.y,transform.scale_x,transform.scale_y,transform.rotation,transform.shear].iter().all(|v|v.is_finite()) {return Err(Fault::from("Ungrouping produced a nonfinite transform"));}
            if transform!=base.transform {append(&mut working,&mut mutations,crate::mutations::update_layer_transform(base.id.clone(),transform))?;}
            if !group.base.visible && base.visible {append(&mut working,&mut mutations,crate::mutations::set_layer_visible(base.id.clone(),false))?;}
            append(&mut working,&mut mutations,crate::mutations::reorder_layer(base.id.clone(),location.parent_id.clone(),location.index+index))?;
        }
        append(&mut working,&mut mutations,crate::mutations::delete_layer(id.clone()))?;
    }
    Ok((mutations,selection))
}
