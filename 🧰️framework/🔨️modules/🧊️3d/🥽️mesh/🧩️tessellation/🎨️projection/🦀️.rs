use super::*;
use protocol::value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep},retirement::controlled::ControlledRetirement};

#[derive(Clone,Copy,Default)]
pub(super) struct MeshAttributeReceipt(pub RetainedCloneProgress);
protocol::value::artifact_retire_leaf!(MeshAttributeReceipt);

fn refusal(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,reason)}
fn index_bytes(count:usize)->Result<usize,ValueError>{count.checked_mul(std::mem::size_of::<u32>()).ok_or_else(||refusal("original mesh attribute index extent overflow"))}

impl MeshTessellationJob{
    /// 🪹️ Original rows remain owned throughout cancellation; normal completion requires every pending row and replaced index owner to finish.
    pub(super) fn attribute_projection_complete(&self)->bool{self.cancelled||(self.attribute_slot>=self.mesh.attributes.slot_count()&&self.attribute_pending.is_none()&&self.attribute_original_indices.is_none())}
    pub(super) fn attribute_domain_ids(&self)->Result<(MeshAttributeDomain,&Vec<u32>),ValueError>{
        let attribute=&self.attribute_pending.as_ref().ok_or_else(||refusal("original mesh attribute row is absent"))?.1;let out=self.output.as_ref().ok_or_else(||refusal("original mesh attribute output is absent"))?;
        Ok(match attribute.domain{MeshAttributeDomain::Vertex=>(MeshAttributeDomain::Vertex,&out.vertex_ids),MeshAttributeDomain::Corner=>(MeshAttributeDomain::Vertex,&self.corner_ids),MeshAttributeDomain::Face=>(MeshAttributeDomain::Face,&out.face_ids),MeshAttributeDomain::Edge=>(MeshAttributeDomain::Edge,&out.edge_ids)})
    }
    pub(super) fn next_attribute_copy_byte_demand(&self)->Result<usize,ValueError>{
        if self.attribute_projection_complete(){return Ok(0);}
        if let Some(owner)=&self.attribute_original_indices{return owner.next_copy_byte_demand();}
        Ok(if self.attribute_pending.is_some()&&self.attribute_stage==2&&self.attribute_cursor<self.attribute_domain_ids()?.1.len(){std::mem::size_of::<u32>()}else{0})
    }
    pub(super) fn next_attribute_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{
        if self.attribute_projection_complete(){return Ok(0);}
        if let Some(owner)=&self.attribute_original_indices{return owner.next_capacity_byte_demand(copy);}
        let Some((key,_))=&self.attribute_pending else{return Ok(0);};
        if self.attribute_stage==1{let count=self.attribute_domain_ids()?.1.len();if self.attribute_indices.capacity()>=count{return Ok(0);}if !self.attribute_indices.is_empty()||self.attribute_indices.capacity()!=0{return Err(refusal("original mesh attribute index reservation must precede emission"));}return index_bytes(count);}
        if self.attribute_stage==6{return self.output.as_ref().unwrap().attributes.next_insert_capacity_byte_demand(key,copy);}
        Ok(0)
    }
    pub(super) fn next_attribute_release_byte_demand(&self)->Result<usize,ValueError>{if self.attribute_projection_complete(){return Ok(0);}self.attribute_original_indices.as_ref().map_or(Ok(0),ControlledRetirement::next_release_byte_demand)}
    /// 🧗️ Covers the original slot extraction, actual replaced-index child, or original output-index insertion path.
    pub(super) fn next_attribute_depth_demand(&self)->Result<usize,ValueError>{
        if self.attribute_projection_complete(){return Ok(0);}
        let depth=if let Some(owner)=&self.attribute_original_indices{owner.next_depth_demand()?}
        else if let Some((key,_))=&self.attribute_pending{if self.attribute_stage>=6{self.output.as_ref().unwrap().attributes.next_insert_depth_demand(key)?}else{2}}
        else{self.mesh.attributes.next_extract_slot_depth_demand(self.attribute_slot)?.checked_add(self.mesh.attributes.next_slot_depth_demand(self.attribute_slot)?).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original mesh attribute extraction depth overflow"))?};
        depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original mesh attribute owner depth overflow"))
    }
    /// 🎨️ Moves one exact original row, writes one quoted index, or forwards one real typed closure turn before admitting another row.
    pub(super) fn project_attribute_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.attribute_receipt.0=Default::default();
        if self.attribute_projection_complete(){return Ok(RetainedCloneStep::Complete(self.attribute_receipt.0));}
        if grant.maximum_items==0||grant.maximum_capacity_bytes<self.next_attribute_capacity_byte_demand(grant.maximum_copy_bytes)?||grant.maximum_release_bytes<self.next_attribute_release_byte_demand()?||grant.maximum_depth<self.next_attribute_depth_demand()?{return Ok(RetainedCloneStep::Progress(self.attribute_receipt.0));}
        if let Some(owner)=&mut self.attribute_original_indices{let result=owner.step(grant);self.attribute_receipt.0=owner.step_progress();let step=result?;self.attribute_receipt.0=step.progress();if owner.terminal_is_empty(){self.attribute_original_indices=None;self.attribute_stage=5;}return Ok(RetainedCloneStep::Progress(self.attribute_receipt.0));}
        if grant.maximum_copy_bytes<self.next_attribute_copy_byte_demand()?{return Ok(RetainedCloneStep::Progress(self.attribute_receipt.0));}
        if self.attribute_pending.is_none(){self.attribute_pending=self.mesh.attributes.extract_slot_if(self.attribute_slot,|_,_|false);self.attribute_slot+=1;self.attribute_stage=1;self.attribute_cursor=0;self.attribute_receipt.0.copied_items=1;}
        else{match self.attribute_stage{
            1=>{let count=self.attribute_domain_ids()?.1.len();if self.attribute_indices.capacity()<count{self.attribute_indices.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original mesh attribute index reservation failed"))?;self.attribute_receipt.0.retained_capacity_bytes=index_bytes(self.attribute_indices.capacity())?;if self.attribute_receipt.0.retained_capacity_bytes>grant.maximum_capacity_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original mesh attribute allocator exceeded admitted backing").with_retained_progress(self.attribute_receipt.0));}}self.attribute_stage=2;self.attribute_receipt.0.copied_items=1;}
            2=>{let(_,ids)=self.attribute_domain_ids()?;if self.attribute_cursor<ids.len(){let id=ids[self.attribute_cursor]as usize;let attribute=&self.attribute_pending.as_ref().unwrap().1;let sample=if let Some(indices)=&attribute.indices{*indices.get(id).ok_or_else(||refusal("original mesh attribute source index is absent"))?}else{u32::try_from(id).map_err(|_|refusal("original mesh attribute source identity exceeds index range"))?};if sample as usize>=attribute.values.len(){return Err(refusal("original mesh attribute source sample is absent"));}if self.attribute_indices.capacity()==self.attribute_indices.len(){return Err(refusal("original mesh attribute exceeded its admitted index extent"));}self.attribute_indices.push(sample);self.attribute_cursor+=1;self.attribute_receipt.0.copied_bytes=std::mem::size_of::<u32>();}else{self.attribute_stage=3;}self.attribute_receipt.0.copied_items=1;}
            3=>{let original=self.attribute_pending.as_mut().unwrap().1.indices.take();match ControlledRetirement::new(original){Ok(owner)=>self.attribute_original_indices=Some(owner),Err((error,original))=>{self.attribute_pending.as_mut().unwrap().1.indices=original;return Err(error);}}self.attribute_stage=4;self.attribute_receipt.0.copied_items=1;}
            5=>{let domain=self.attribute_domain_ids()?.0;let attribute=&mut self.attribute_pending.as_mut().unwrap().1;attribute.domain=domain;attribute.indices=Some(std::mem::take(&mut self.attribute_indices));self.attribute_stage=6;self.attribute_receipt.0.copied_items=1;}
            6=>{let(key,_)=self.attribute_pending.as_ref().unwrap();if self.output.as_ref().unwrap().attributes.contains_key(key){return Err(refusal("original mesh output already owns the incoming attribute key"));}let capacity=self.next_attribute_capacity_byte_demand(grant.maximum_copy_bytes)?;if capacity==0{self.attribute_stage=7;self.attribute_receipt.0.copied_items=1;}else{match self.output.as_mut().unwrap().attributes.reserve_insert_step(key,grant){Ok(receipt)=>self.attribute_receipt.0=receipt,Err((error,receipt))=>{self.attribute_receipt.0=receipt;return Err(error.with_retained_progress(receipt));}}}}
            7=>{let(key,attribute)=self.attribute_pending.take().unwrap();match self.output.as_mut().unwrap().attributes.insert_reserved(key,attribute,grant){Ok((None,receipt))=>self.attribute_receipt.0=receipt,Ok((Some(_),_))=>unreachable!("original attribute key uniqueness checked before admission"),Err((error,key,attribute))=>{self.attribute_pending=Some((key,attribute));return Err(error);}}self.attribute_stage=0;}
            _=>return Err(refusal("original mesh attribute projection cursor is invalid")),
        }}
        Ok(if self.attribute_projection_complete(){RetainedCloneStep::Complete(self.attribute_receipt.0)}else{RetainedCloneStep::Progress(self.attribute_receipt.0)})
    }
}
