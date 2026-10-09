use super::*;
use protocol::value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};

#[derive(Clone,Copy,Default)]
pub(super) struct MeshMetadataReceipt(pub RetainedCloneProgress);
protocol::value::artifact_retire_leaf!(MeshMetadataReceipt);

fn semantic_index(semantic:MeshAttributeSemantic)->Option<usize>{match semantic{MeshAttributeSemantic::Normal=>Some(0),MeshAttributeSemantic::Uv=>Some(1),MeshAttributeSemantic::Color=>Some(2),_=>None}}

impl MeshTessellationJob{
    /// 🎨️ Declares completion of the original borrowed semantic-slot capture frontier.
    pub(super) fn metadata_capture_complete(&self)->bool{self.cancelled||self.metadata_cursor>=self.mesh.attributes.slot_count()}
    /// 🧭️ Borrows the retained original authored sample owner without copying its key or values.
    pub(super) fn semantic_attribute(&self,semantic:MeshAttributeSemantic)->Option<&MeshAttribute>{self.mesh.attributes.slot_entry(self.semantic_slots[semantic_index(semantic)?]?).map(|(_,attribute)|attribute)}
    pub(super) fn next_metadata_copy_byte_demand(&self)->Result<usize,ValueError>{self.next_metadata_depth_demand().map(|_|0)}
    pub(super) fn next_metadata_capacity_byte_demand(&self,_maximum_copy_bytes:usize)->Result<usize,ValueError>{self.next_metadata_depth_demand().map(|_|0)}
    pub(super) fn next_metadata_release_byte_demand(&self)->Result<usize,ValueError>{self.next_metadata_depth_demand().map(|_|0)}
    /// 🧗️ Quotes the same original paged slot path before the producer borrows that row.
    pub(super) fn next_metadata_depth_demand(&self)->Result<usize,ValueError>{
        if self.metadata_capture_complete(){return Ok(0);}
        self.mesh.attributes.next_slot_depth_demand(self.metadata_cursor)?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original mesh metadata slot depth overflow"))
    }
    /// 🪪️ Captures at most one original slot identity, retaining lexicographic source precedence and every original allocation.
    pub(super) fn capture_metadata_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.metadata_receipt.0=Default::default();
        if self.metadata_capture_complete(){return Ok(RetainedCloneStep::Complete(self.metadata_receipt.0));}
        if grant.maximum_items==0||grant.maximum_depth<self.next_metadata_depth_demand()?{return Ok(RetainedCloneStep::Progress(self.metadata_receipt.0));}
        if let Some((key,attribute))=self.mesh.attributes.slot_entry(self.metadata_cursor){
            if let Some(semantic)=semantic_index(attribute.semantic){
                let earlier=self.semantic_slots[semantic].and_then(|slot|self.mesh.attributes.slot_entry(slot)).is_some_and(|(original,_)|original<=key);
                if !earlier{self.semantic_slots[semantic]=Some(self.metadata_cursor);}
            }
        }
        self.metadata_cursor+=1;self.metadata_receipt.0.copied_items=1;
        Ok(if self.metadata_capture_complete(){RetainedCloneStep::Complete(self.metadata_receipt.0)}else{RetainedCloneStep::Progress(self.metadata_receipt.0)})
    }
}
