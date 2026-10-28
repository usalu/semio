//! 🪪️ Decision hashes consume the existing borrowed canonical producers and retire their paid bytes.
use super::*;
#[derive(Default)]
pub struct DurableDecisionIdentityV1{json:DurableCanonicalOutputV1,bytes:Option<Vec<u8>>,hash:semio_framework_hash::Sha256,offset:usize,phase:u8,closing:Option<semio_framework_value::retirement::controlled::ControlledRetirement<DurablePlanningOutputV1>>}
impl DurableDecisionIdentityV1{
    pub fn initialized(&self)->bool{self.phase!=0}
    pub fn initialize(&mut self){self.phase=1;}
    pub fn ready(&self)->bool{self.phase==9&&self.bytes.is_none()&&self.closing.is_none()&&self.json.terminal_is_empty()}
    pub fn advance(&mut self,decision:&mut impl DurableDecisionSource,grant:RetainedCloneGrant,native:&mut NativeEncodeControl<'_>)->Result<RetainedCloneProgress,ValueError>{
        if self.ready(){return Ok(Default::default());}
        match self.phase{
            1|5=>{if self.phase==5&&self.json.terminal_is_empty(){return Ok(self.json.restart_empty(grant).unwrap_or_default());}let source=if self.phase==1{DurableBorrowedValue::Anchor(decision.fields().anchor)}else{decision_source(decision,true)};let receipt=self.json.advance(&source,DURABLE_OWNED_GROUP_STRUCTURAL_MAX_BYTES,grant,native)?;if self.json.ready(){self.phase+=1;}Ok(receipt)},
            2|6=>{let Some((bytes,receipt))=self.json.take(grant)else{return Ok(Default::default());};self.bytes=Some(bytes);self.phase+=1;Ok(receipt)},
            3|7=>{let demand=copy(1+size_of::<usize>()+size_of::<u8>());if !permit(demand,grant){return Ok(Default::default());}native.checkpoint()?;let bytes=self.bytes.as_ref().unwrap();if let Some(byte)=bytes.get(self.offset){self.hash.update(&[*byte]);self.offset+=1;}else{self.phase+=1;}Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})},
            4|8 if self.bytes.is_some()=>{let demand=RetirementDemand{capacity_bytes:64,..copy(size_of::<semio_framework_hash::Sha256>()+32+64+size_of::<String>()+size_of::<DurablePlanningOutputV1>()+size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<DurablePlanningOutputV1>>()+size_of::<usize>())};if !permit(demand,grant){return Ok(Default::default());}native.checkpoint()?;let mut text=[0u8;64];for(index,byte)in self.hash.clone().finalize().iter().enumerate(){const HEX:&[u8;16]=b"0123456789abcdef";text[2*index]=HEX[usize::from(byte>>4)];text[2*index+1]=HEX[usize::from(byte&15)];}native.admit_turn_capacity(grant.maximum_capacity_bytes)?;let hash=native.copy_text(std::str::from_utf8(&text).unwrap())?;if self.phase==4{decision.set_anchor_hash(hash);}else{decision.set_decision_hash(hash);}self.closing=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(DurablePlanningOutputV1::Pack(self.bytes.take().unwrap())).map_err(|(reason,_)|reason)?);self.hash=semio_framework_hash::Sha256::new();self.offset=0;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:64,..Default::default()})},
            4|8=>{let owner=self.closing.as_mut().unwrap();let receipt=owner.step(grant)?.progress();if owner.terminal_is_empty(){self.closing=None;self.phase+=1;}Ok(receipt)},
            _=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"decision identity planner lost its frontier")),
        }
    }
    pub fn close(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if !self.json.terminal_is_empty(){return Ok(self.json.close(grant));}if let Some(owner)=self.closing.as_mut(){let receipt=owner.step(grant)?.progress();if owner.terminal_is_empty(){self.closing=None;}return Ok(receipt);}let demand=copy(size_of::<DurablePlanningOutputV1>()+size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<DurablePlanningOutputV1>>()+size_of::<u8>());if !permit(demand,grant){return Ok(Default::default());}if let Some(bytes)=self.bytes.take(){self.closing=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(DurablePlanningOutputV1::Pack(bytes)).map_err(|(reason,_)|reason)?);}else{self.phase=9;}Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})}
}
