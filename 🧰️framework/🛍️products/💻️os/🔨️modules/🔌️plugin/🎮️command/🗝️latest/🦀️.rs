//! 🗝️ Original framed latest key keeps its borrowed source identity and exact full currency receipts.
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value_ordered::ordered::SharedOwner;
use semio_plugin_job::original_steps::{acted,forgotten,forward,produced,settle};
use std::mem::size_of;
const CHUNK:usize=64;
const HEX:&[u8;16]=b"0123456789abcdef";
fn invalid(detail:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,detail)}

#[doc(hidden)]
pub struct ToolLatestWinsKeyCopy {
    pub key:Option<String>,pub instance:u32,pub part:usize,pub offset:usize,pub prefix:usize,pub source:[(usize,usize);4],
}
impl ToolLatestWinsKeyCopy {
    /// 📏️ Quotes the exact framed extent and the header move of its byte cursor.
    pub fn birth_demands(parts:[&str;4])->Result<RetirementDemand,ValueError>{
        let size=parts.iter().try_fold(72usize,|size,part|part.len().checked_mul(2).and_then(|bytes|size.checked_add(bytes))).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"latest-wins original key layout overflow"))?;
        Ok(RetirementDemand{copy_bytes:size_of::<Self>(),capacity_bytes:size,depth:1,..Default::default()})
    }
    /// 🌱️ The framed key backing is born once, paid with its exact capacity, before any byte is encoded.
    pub fn admit(output:&mut Option<Self>,instance:u32,parts:[&str;4],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        let d=Self::birth_demands(parts)?;
        produced(cx,d,output,||{let mut key=String::new();key.try_reserve_exact(d.capacity_bytes).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"latest-wins original key backing refused"))?;Ok(Self{key:Some(key),instance,part:0,offset:0,prefix:0,source:parts.map(|part|(part.as_ptr()as usize,part.len()))})})
    }
    pub fn ready(&self)->bool{self.part>4}
    /// 📏️ Quotes the exact next encoding turn: at most sixty-four source bytes, three copied bytes each.
    pub fn demands(&self,parts:[&str;4])->RetirementDemand{
        let copy_bytes=match self.part{
            0=>(8-self.prefix).min(CHUNK),
            1..=4=>{let length=parts[self.part-1].len();if self.prefix<16{(16-self.prefix).min(CHUNK)}else if self.offset<length{3*(length-self.offset).min(CHUNK)}else{0}},
            _=>0,
        };
        RetirementDemand{copy_bytes,depth:1,..Default::default()}
    }
    fn encode(&mut self,parts:[&str;4])->Result<(),ValueError>{
        let key=self.key.as_mut().ok_or_else(||invalid("latest-wins original key handed off before encoding"))?;
        if self.part==0{let count=(8-self.prefix).min(CHUNK);for _ in 0..count{key.push(HEX[((self.instance>>(28-self.prefix*4))&15)as usize]as char);self.prefix+=1;}if self.prefix==8{self.part=1;self.prefix=0;}return Ok(())}
        let part=parts[self.part-1].as_bytes();
        if self.prefix<16{let count=(16-self.prefix).min(CHUNK);for _ in 0..count{key.push(HEX[((part.len()as u64>>(60-self.prefix*4))&15)as usize]as char);self.prefix+=1;}}
        else if self.offset<part.len(){let count=(part.len()-self.offset).min(CHUNK);for byte in &part[self.offset..self.offset+count]{key.push(HEX[(byte>>4)as usize]as char);key.push(HEX[(byte&15)as usize]as char);}self.offset+=count;}
        else{self.part+=1;self.offset=0;self.prefix=0;}
        Ok(())
    }
    /// ✍️ One funded turn encodes the next bounded fragment; `true` once the whole frame is encoded.
    pub fn advance(&mut self,parts:[&str;4],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        if self.source!=parts.map(|part|(part.as_ptr()as usize,part.len())){return Err(invalid("latest-wins original borrowed key source changed").into());}
        if self.ready(){return Ok(true);}
        let d=self.demands(parts);
        acted(cx,d,||self.encode(parts))?;
        Ok(self.ready())
    }
    /// 📏️ Quotes the shared header admission of the finished key.
    pub fn take_demands()->Result<RetirementDemand,ValueError>{SharedOwner::<String>::source_admission_demand()}
    /// 🤝️ The finished key bytes move into one issued shared owner without being copied again.
    pub fn take_key(&mut self,output:&mut Option<SharedOwner<String>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        if !self.ready(){return Err(invalid("latest-wins original key is taken before it is encoded").into());}
        if output.is_some(){return Ok(true);}
        let d=Self::take_demands()?;
        forward(cx,d,|effect|{let turn=SharedOwner::admit_from(&mut self.key,output,effect);settle(effect,turn.result)})?;
        Ok(output.is_some())
    }
    pub fn close_demands(&self)->RetirementDemand{RetirementDemand{release_bytes:self.key.as_ref().map_or(0,String::capacity),depth:usize::from(self.key.is_some()),..Default::default()}}
    pub fn terminal_is_empty(&self)->bool{self.key.is_none()}
    /// ♻️ Releases the unfinished key backing in one paid retirement turn.
    pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        if self.key.is_none(){return Ok(true);}
        let d=self.close_demands();
        forgotten(cx,d,||drop(self.key.take()))?;
        Ok(self.key.is_none())
    }
}

#[doc(hidden)]
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
pub mod tests;
