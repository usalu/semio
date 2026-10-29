//! 🎫️ Source-window capture advances one immutable roster, slot or string-byte action per caller turn.
use super::*;
use semio_framework_value::{ValueError,ValueRefusalKind};
#[derive(Default,semio_framework_value::RetireOwned)]
pub struct SurfaceWindowLeaseCursor {
    source:usize,version:u64,window:usize,length:usize,locale:Option<semio_framework::Locale>,
    phase:u8,row:usize,byte:usize,actual:Option<usize>,slot:Option<usize>,mounted:Option<usize>,failed:bool,
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"staging.context-unavailable")}
fn compare(left:&str,right:&str,byte:&mut usize)->Option<bool>{
    if left.len()!=right.len(){*byte=0;return Some(false);}
    if *byte<left.len(){let equal=left.as_bytes()[*byte]==right.as_bytes()[*byte];*byte+=1;if !equal{*byte=0;return Some(false);}return None;}
    *byte=0;Some(true)
}
impl SurfaceWindowLeaseCursor {
    fn advance(&mut self,contexts:&SurfaceContexts,window:&str,locale:semio_framework::Locale)->Result<Option<SurfaceWindowLease>,ValueError>{
        if contexts.leases_exhausted{return Err(invalid());}
        let host=contexts.view_state.as_ref().ok_or_else(invalid)?;
        if host.locale!=locale{return Err(invalid());}
        if self.phase==0{
            self.source=contexts as*const _ as usize;self.version=contexts.version;self.window=window.as_ptr()as usize;self.length=window.len();self.locale=Some(locale);self.phase=1;return Ok(None);
        }
        if self.source!=contexts as*const _ as usize||self.version!=contexts.version||self.window!=window.as_ptr()as usize||self.length!=window.len()||self.locale!=Some(locale){return Err(invalid());}
        match self.phase{
            1=>{
                let Some(row)=host.window_instances.get(self.row)else{if self.actual.is_none(){return Err(invalid());}self.phase=2;self.row=0;return Ok(None);};
                if let Some(equal)=compare(&row.id,window,&mut self.byte){if equal&&self.actual.replace(self.row).is_some(){return Err(invalid());}self.row+=1;}Ok(None)
            },
            2=>{
                let Some(row)=contexts.slots.get(self.row)else{if self.slot.is_none(){return Err(invalid());}self.phase=3;self.row=0;return Ok(None);};
                let Some(id)=row.as_ref().and_then(|binding|binding.role.window_id())else{self.row+=1;return Ok(None);};
                if let Some(equal)=compare(id,window,&mut self.byte){if equal&&self.slot.replace(self.row).is_some(){return Err(invalid());}self.row+=1;}Ok(None)
            },
            3=>{
                let binding=contexts.slots[self.slot.unwrap()].as_ref().ok_or_else(invalid)?;
                if binding.view_state.as_ref().ok_or_else(invalid)?.locale!=locale{return Err(invalid());}
                if let Some(equal)=compare(binding.view_state.as_ref().ok_or_else(invalid)?.window_id.as_deref().ok_or_else(invalid)?,window,&mut self.byte){if !equal{return Err(invalid());}self.phase=4;}Ok(None)
            },
            4=>{
                let binding=contexts.slots[self.slot.unwrap()].as_ref().ok_or_else(invalid)?;
                let Some(row)=binding.view_state.as_ref().ok_or_else(invalid)?.window_instances.get(self.row)else{if self.mounted.is_none(){return Err(invalid());}self.phase=5;return Ok(None);};
                if let Some(equal)=compare(&row.id,window,&mut self.byte){if equal&&self.mounted.replace(self.row).is_some(){return Err(invalid());}self.row+=1;}Ok(None)
            },
            5=>{
                let slot=self.slot.unwrap();let binding=contexts.slots[slot].as_ref().ok_or_else(invalid)?;
                let actual=&host.window_instances[self.actual.unwrap()].window_kind_id;let mounted=&binding.view_state.as_ref().ok_or_else(invalid)?.window_instances[self.mounted.unwrap()].window_kind_id;
                if let Some(equal)=compare(actual,mounted,&mut self.byte){if !equal{return Err(invalid());}self.phase=6;return Ok(Some(SurfaceWindowLease{source:self.source,context_version:self.version,slot,version:binding.version}));}Ok(None)
            },
            _=>Err(invalid()),
        }
    }
    /// 🧷️ The caller funds one step and retains its same source and target owner until completion or retirement.
    pub fn step(&mut self,contexts:&SurfaceContexts,window:&str,locale:semio_framework::Locale)->Result<Option<SurfaceWindowLease>,ValueError>{
        if self.failed{return Err(invalid());}let result=self.advance(contexts,window,locale);if result.is_err(){self.failed=true;}result
    }
}
