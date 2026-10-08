//! 🗂️ Owned u16 record fields in one exact admitted slot buffer.
use super::FieldValue;
#[derive(Clone,Debug,Default,PartialEq)]
pub struct RecordFields{entries:Vec<(u16,FieldValue)>}
impl RecordFields{
    /// 📦️ Retains a producer-admitted empty buffer without another allocation.
    pub fn from_empty_slots(entries:Vec<(u16,FieldValue)>)->Self{assert!(entries.is_empty());Self{entries}}
    /// 🔎️ Returns the number of committed fields.
    pub fn len(&self)->usize{self.entries.len()}
    /// 🕳️ Reports whether any fields have been committed.
    pub fn is_empty(&self)->bool{self.entries.is_empty()}
    /// 📏️ Reports actual allocated slot capacity.
    pub fn capacity(&self)->usize{self.entries.capacity()}
    /// 🔎️ Borrows one field by its declared identity.
    pub fn get(&self,id:&u16)->Option<&FieldValue>{self.entries.binary_search_by_key(id,|(id,_)|*id).ok().map(|index|&self.entries[index].1)}
    /// 🔧️ Borrows one field payload for a declared owner mutation.
    pub fn get_mut(&mut self,id:&u16)->Option<&mut FieldValue>{self.entries.binary_search_by_key(id,|(id,_)|*id).ok().map(|index|&mut self.entries[index].1)}
    /// 📍️ Reports whether a declared identity is present.
    pub fn contains_key(&self,id:&u16)->bool{self.entries.binary_search_by_key(id,|(id,_)|*id).is_ok()}
    /// 📥️ Commits one field in identity order and transfers a replaced value.
    pub fn insert(&mut self,id:u16,value:FieldValue)->Option<FieldValue>{match self.entries.binary_search_by_key(&id,|(id,_)|*id){Ok(index)=>Some(std::mem::replace(&mut self.entries[index].1,value)),Err(index)=>{self.entries.insert(index,(id,value));None}}}
    /// 📤️ Removes one field and transfers its payload.
    pub fn remove(&mut self,id:&u16)->Option<FieldValue>{self.entries.binary_search_by_key(id,|(id,_)|*id).ok().map(|index|self.entries.remove(index).1)}
    /// 🧭️ Borrows identities in their canonical numeric order.
    pub fn keys(&self)->impl ExactSizeIterator<Item=&u16>+DoubleEndedIterator{self.entries.iter().map(|(id,_)|id)}
    /// 🧭️ Borrows payloads in their canonical identity order.
    pub fn values(&self)->impl ExactSizeIterator<Item=&FieldValue>+DoubleEndedIterator{self.entries.iter().map(|(_,value)|value)}
    /// 🔧️ Borrows payloads for a declared owner mutation.
    pub fn values_mut(&mut self)->impl ExactSizeIterator<Item=&mut FieldValue>+DoubleEndedIterator{self.entries.iter_mut().map(|(_,value)|value)}
    /// 🧭️ Borrows complete fields without materializing a mirror.
    pub fn iter(&self)->impl ExactSizeIterator<Item=(&u16,&FieldValue)>+DoubleEndedIterator{self.entries.iter().map(|(id,value)|(id,value))}
    /// ♻️ Moves all fields into a newly admitted buffer without implicit growth.
    pub fn replace_empty_slots(&mut self,mut entries:Vec<(u16,FieldValue)>){assert!(entries.is_empty()&&entries.capacity()>=self.entries.len());entries.append(&mut self.entries);self.entries=entries;}
    /// 📤️ Transfers each field payload to its owning consumer.
    pub fn into_values(self)->impl ExactSizeIterator<Item=FieldValue>+DoubleEndedIterator{self.entries.into_iter().map(|(_,value)|value)}
}
impl FromIterator<(u16,FieldValue)> for RecordFields{fn from_iter<T:IntoIterator<Item=(u16,FieldValue)>>(items:T)->Self{let mut fields=Self::default();for(id,value)in items{fields.insert(id,value);}fields}}
impl IntoIterator for RecordFields{type Item=(u16,FieldValue);type IntoIter=std::vec::IntoIter<Self::Item>;fn into_iter(self)->Self::IntoIter{self.entries.into_iter()}}
fn field_ref(pair:&(u16,FieldValue))->(&u16,&FieldValue){(&pair.0,&pair.1)}
impl<'a> IntoIterator for &'a RecordFields{type Item=(&'a u16,&'a FieldValue);type IntoIter=std::iter::Map<std::slice::Iter<'a,(u16,FieldValue)>,fn(&'a(u16,FieldValue))->Self::Item>;fn into_iter(self)->Self::IntoIter{self.entries.iter().map(field_ref)}}

impl semio_framework_value::retirement::RetireOwned for RecordFields{
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::RetireOwned::retirement(self.entries)}
    fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&self.entries)}
    fn controlled_retirement_supported()->bool{true}
}
