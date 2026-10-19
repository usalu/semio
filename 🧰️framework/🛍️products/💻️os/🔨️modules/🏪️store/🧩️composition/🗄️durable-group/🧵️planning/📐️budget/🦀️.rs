//! 📐️ Cold verification and retained production share the exact declared JSON resource scanner.
use super::*;
#[derive(Default)]
pub struct DurableJsonBudgetV1{depth:usize,items:usize,in_string:bool,escaped:bool}
impl DurableJsonBudgetV1{
    pub fn advance_byte(&mut self,byte:u8)->Result<(),DurableOwnedGroupDecisionError>{
        if self.items==0{self.items=1;}
        if self.in_string{if self.escaped{self.escaped=false;}else if byte==b'\\'{self.escaped=true;}else if byte==b'"'{self.in_string=false;}else if byte<0x20{return Err(DurableOwnedGroupDecisionError::NonCanonical);}return Ok(());}
        match byte{b'"'=>self.in_string=true,b'{'|b'['=>{self.depth=self.depth.checked_add(1).ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;if self.depth>DURABLE_OWNED_GROUP_JSON_MAX_DEPTH{return Err(DurableOwnedGroupDecisionError::InvalidOutcome);}},b'}'|b']'=>self.depth=self.depth.checked_sub(1).ok_or(DurableOwnedGroupDecisionError::NonCanonical)?,b','=>{self.items=self.items.checked_add(1).ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;if self.items>DURABLE_OWNED_GROUP_JSON_MAX_ITEMS{return Err(DurableOwnedGroupDecisionError::InvalidOutcome);}},_=>{}}Ok(())
    }
    pub fn finish(&self)->Result<(),DurableOwnedGroupDecisionError>{if self.depth!=0||self.in_string||self.escaped{Err(DurableOwnedGroupDecisionError::NonCanonical)}else{Ok(())}}
}
