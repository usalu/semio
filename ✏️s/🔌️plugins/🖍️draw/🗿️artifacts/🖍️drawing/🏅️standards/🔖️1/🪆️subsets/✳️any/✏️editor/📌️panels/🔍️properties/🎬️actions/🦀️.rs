//! 🎛️ Cheap selection eligibility for existing semantic inspector commands.
use crate::{DrawingLayerNode,DrawingSnapshot};
use crate::schema::layer_base;

pub(super) struct SelectionActionEligibility {
    pub count:usize,
    pub locked:bool,
    pub same_parent:bool,
    pub convertible_shapes:bool,
    pub ungroupable_groups:bool,
    pub complete:bool,
    pub arrangement_count:usize,
    pub arrangeable:bool,
}

impl SelectionActionEligibility {
    /// 🌳️ Borrows one document traversal for parent, inherited state and effective selection boundaries.
    pub fn from_selection(document:&DrawingSnapshot,selected:&[&DrawingLayerNode],ids:&[String])->Self {
        let wanted=selected.iter().map(|layer|&layer_base(layer).id).collect::<std::collections::BTreeSet<_>>();
        let mut result=Self {
            count:selected.len(),locked:false,same_parent:true,
            convertible_shapes:selected.iter().any(|layer|matches!(layer,DrawingLayerNode::Shape(_)))&&selected.iter().all(|layer|matches!(layer,DrawingLayerNode::Shape(_)|DrawingLayerNode::Path(_))),
            ungroupable_groups:selected.iter().all(|layer|matches!(layer,DrawingLayerNode::Group(group) if !group.isolation&&group.base.opacity==1.0&&group.base.blend_mode=="normal")),
            complete:ids.iter().collect::<std::collections::BTreeSet<_>>().len()==selected.len(),arrangement_count:0,arrangeable:true,
        };
        let mut first_parent=None;
        let mut stack=vec![(&document.layers,0usize,None,false,true,false)];
        while let Some((layers,index,parent,inherited_lock,inherited_visible,ancestor_selected))=stack.last_mut() {
            let Some(layer)=layers.get(*index) else {stack.pop();continue;};
            *index+=1;
            let base=layer_base(layer);
            let parent=*parent;
            let locked=*inherited_lock||base.locked;
            let visible=*inherited_visible&&base.visible;
            let ancestor_selected=*ancestor_selected;
            let chosen=wanted.contains(&base.id);
            if chosen {
                result.locked|=locked;
                result.arrangeable&=!locked&&visible;
                if !ancestor_selected {result.arrangement_count+=1;}
                match first_parent {Some(first)=>result.same_parent&=first==parent,None=>first_parent=Some(parent)}
            }
            if let DrawingLayerNode::Group(group)=layer {stack.push((&group.children,0,Some(&base.id),locked,visible,ancestor_selected||chosen));}
        }
        result
    }

    /// 🧭️ Offers commands whose known structural requirements are satisfied.
    pub fn allows(&self,operation:&str)->bool {
        if self.count==0||self.locked||!self.complete{return false;}
        match operation {
            "toPath"=>self.convertible_shapes,
            "ungroup"=>self.ungroupable_groups,
            "alignLeft"|"alignCenter"|"alignRight"|"alignTop"|"alignMiddle"|"alignBottom"=>self.arrangeable&&self.arrangement_count>=2,
            "distributeHorizontal"|"distributeVertical"=>self.arrangeable&&self.arrangement_count>=3,
            "group"|"duplicate"|"delete"|"bringForward"|"sendBackward"|"bringToFront"|"sendToBack"=>self.same_parent,
            _=>false,
        }
    }
}