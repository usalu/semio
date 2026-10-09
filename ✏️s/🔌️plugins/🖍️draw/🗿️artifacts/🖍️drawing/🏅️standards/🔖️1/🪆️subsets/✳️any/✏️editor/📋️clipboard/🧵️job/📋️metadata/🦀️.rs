//! 📋️ Clipboard metadata claims actual paged backing and bounded keys before constructing candidates.
use super::*;
use semio_framework_value::list::PagedList;
#[derive(RetireOwned)]
pub(super) struct Table<T:RetireAuthority>{entries:PagedList<(String,T,bool),MAX_NODES>,live:usize,admitted:usize,pending_key:Option<String>,pending_value:Option<T>}
impl<T:RetireAuthority> Table<T>{
    pub(super) fn new()->Self{Self{entries:PagedList::new(),live:0,admitted:0,pending_key:None,pending_value:None}}
    pub(super) fn len(&self)->usize{self.live}
    pub(super) fn get(&self,key:&str)->Option<&T>{self.entries.iter().find(|(candidate,_,live)|*live&&candidate==key).map(|(_,value,_)|value)}
    pub(super) fn contains_key(&self,key:&str)->bool{self.get(key).is_some()}
    pub(super) fn contains(&self,key:&str)->bool{self.contains_key(key)}
    pub(super) fn values(&self)->impl Iterator<Item=&T>{self.entries.iter().filter(|(_,_,live)|*live).map(|(_,value,_)|value)}
    pub(super) fn remove(&mut self,key:&str){if let Some((_,_,live))=self.entries.iter_mut().find(|(candidate,_,live)|*live&&candidate==key){*live=false;self.live-=1;}}
    fn insert_with(&mut self,key:&str,extra:usize,make:impl FnOnce()->Result<T,semio_framework::Fault>)->Result<bool,semio_framework::Fault>{
        if let Some((_,_,live))=self.entries.iter_mut().find(|(candidate,_,_)|candidate==key){if *live{return Ok(false);}*live=true;self.live+=1;return Ok(true);}
        if key.is_empty()||key.len()>MAX_ID_BYTES||self.entries.len()==MAX_NODES||self.pending_key.is_some()||self.pending_value.is_some(){return Err(fault(refused()));}
        let maximum=MAX_NODES.checked_mul(MAX_ID_BYTES+4096+std::mem::size_of::<(String,T,bool)>()*2).ok_or_else(||fault(refused()))?;
        let mut accepted=|_|true;let mut control=NativeDecodeControl::new(maximum.checked_sub(self.admitted).ok_or_else(||fault(refused()))?,&mut accepted);
        let result=(||{while !self.entries.has_reserved_slot(){let bytes=self.entries.next_allocation_bytes().map_err(fault)?;control.charge(bytes).map_err(fault)?;self.entries.reserve_one(bytes).map_err(|error|fault(error.refusal()))?;}control.charge(key.len().checked_add(extra).ok_or_else(||fault(refused()))?).map_err(fault)?;
            let mut owned=String::new();owned.try_reserve_exact(key.len()).map_err(fault)?;owned.push_str(key);self.pending_key=Some(owned);self.pending_value=Some(make()?);
            let entry=(self.pending_key.take().unwrap(),self.pending_value.take().unwrap(),true);self.entries.push_reserved(entry).map_err(|(key,value,_)|{self.pending_key=Some(key);self.pending_value=Some(value);fault(refused())})?;self.live+=1;Ok(true)
        })();self.admitted=self.admitted.checked_add(control.owned_bytes()).ok_or_else(||fault(refused()))?;result
    }
    pub(super) fn insert(&mut self,key:&str,value:T)->Result<bool,semio_framework::Fault>{self.insert_with(key,0,||Ok(value))}
}
impl Table<()>{pub(super) fn insert_key(&mut self,key:&str)->Result<bool,semio_framework::Fault>{self.insert(key,())}}
impl Table<String>{pub(super) fn insert_text(&mut self,key:&str,value:&str)->Result<bool,semio_framework::Fault>{if value.len()>MAX_ID_BYTES{return Err(fault(refused()));}self.insert_with(key,value.len(),||{let mut owned=String::new();owned.try_reserve_exact(value.len()).map_err(fault)?;owned.push_str(value);Ok(owned)})}}
impl<T:RetireAuthority> std::ops::Index<&String> for Table<T>{type Output=T;fn index(&self,key:&String)->&T{self.get(key.as_str()).expect("admitted clipboard metadata key")}}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
