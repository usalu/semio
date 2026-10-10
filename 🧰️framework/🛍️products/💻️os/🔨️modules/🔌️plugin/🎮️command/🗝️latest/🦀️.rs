//! 🗝️ Original framed latest key keeps its borrowed source identity and exact full currency receipts.
use semio_framework_value::{RetirementDemand,ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};

pub(crate) struct ToolLatestWinsKeyCopy {
    pub(crate) key:Option<String>,instance:u32,pub(crate) part:usize,offset:usize,prefix:usize,source:[(usize,usize);4],
}
impl ToolLatestWinsKeyCopy {
    pub(crate) fn admit_original(instance:u32,parts:[&str;4],grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
        let size=parts.iter().try_fold(72usize,|size,part|part.len().checked_mul(2).and_then(|bytes|size.checked_add(bytes))).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"latest-wins original key layout overflow"))?;
        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<size{return Ok(None);}
        let mut key=String::new();key.try_reserve_exact(size).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"latest-wins original key backing refused"))?;let retained_capacity_bytes=key.capacity();
        Ok(Some((Self{key:Some(key),instance,part:0,offset:0,prefix:0,source:parts.map(|part|(part.as_ptr()as usize,part.len()))},RetainedCloneProgress{copied_items:1,retained_capacity_bytes,..Default::default()})))
    }
    pub(crate) fn next_copy_byte_demand(&self,parts:[&str;4])->usize{if self.part==0||self.part<=4&&self.prefix<16{1}else if self.part<=4&&self.offset<parts[self.part-1].len(){3}else{0}}
    pub(crate) fn advance(&mut self,parts:[&str;4],grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let idle=RetainedCloneProgress::default();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(idle));}
        if self.source!=parts.map(|part|(part.as_ptr()as usize,part.len())){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"latest-wins original borrowed key source changed"));}
        if self.part>4{return Ok(RetainedCloneStep::Complete(idle));}
        const HEX:&[u8;16]=b"0123456789abcdef";let key=self.key.as_mut().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"latest-wins original key handed off before encoding"))?;
        let copied_bytes;
        if self.part==0{let count=(8-self.prefix).min(grant.maximum_copy_bytes);if count==0{return Ok(RetainedCloneStep::Progress(idle));}for _ in 0..count{key.push(HEX[((self.instance>>(28-self.prefix*4))&15)as usize]as char);self.prefix+=1;}if self.prefix==8{self.part=1;self.prefix=0;}copied_bytes=count;}
        else{let part=parts[self.part-1].as_bytes();if self.prefix<16{let count=(16-self.prefix).min(grant.maximum_copy_bytes);if count==0{return Ok(RetainedCloneStep::Progress(idle));}for _ in 0..count{key.push(HEX[((part.len()as u64>>(60-self.prefix*4))&15)as usize]as char);self.prefix+=1;}copied_bytes=count;}
        else if self.offset<part.len(){let count=(part.len()-self.offset).min(grant.maximum_copy_bytes/3);if count==0{return Ok(RetainedCloneStep::Progress(idle));}for byte in &part[self.offset..self.offset+count]{key.push(HEX[(byte>>4)as usize]as char);key.push(HEX[(byte&15)as usize]as char);}self.offset+=count;copied_bytes=count*3;}
        else{self.part+=1;self.offset=0;self.prefix=0;copied_bytes=0;}}
        let progress=RetainedCloneProgress{copied_items:1,copied_bytes,..idle};Ok(if self.part>4{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }
    pub(crate) fn take_key(&mut self,grant:RetainedCloneGrant)->Result<Option<(semio_framework_value::ordered::SharedOwner<String>,RetainedCloneProgress)>,ValueError>{
        use semio_framework_value::ordered::SharedOwner;
        if self.part<=4||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<SharedOwner::<String>::allocation_bytes(){return Ok(None);}
        let Some(key)=self.key.take()else{return Ok(None)};
        match SharedOwner::admit(key,grant){Ok(value)=>Ok(Some(value)),Err((error,key))=>{self.key=Some(key);Err(error)}}
    }
    pub(crate) fn retirement_demands(&self)->RetirementDemand{RetirementDemand{release_bytes:self.key.as_ref().map_or(0,String::capacity),depth:usize::from(self.key.is_some()),..Default::default()}}
    pub(crate) fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedCloneStep{
        let idle=RetainedCloneProgress::default();if self.key.is_none(){return RetainedCloneStep::Complete(idle);}let demand=self.retirement_demands();if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_release_bytes<demand.release_bytes{return RetainedCloneStep::Progress(idle);}drop(self.key.take());RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..idle})
    }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
