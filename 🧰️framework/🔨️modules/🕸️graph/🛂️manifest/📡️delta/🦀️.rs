//! 📡️ Graph property deltas settle the actual typed property owner directly.
use super::{PropertyBag,PropertyValue};
use semio_framework_value::FromValue;

impl protocol::mutation::MapDeltaTarget<PropertyValue> for PropertyBag {
    fn contains_key(&self,key:&str)->bool{PropertyBag::contains_key(self,key)}
    fn set_owned(&mut self,key:String,value:PropertyValue){if let Some(previous)=self.insert(key,value){PropertyValue::retire_decoded(previous)}}
    fn remove_owned(&mut self,key:&str){if let Some(previous)=self.remove(key){PropertyValue::retire_decoded(previous)}}
}
