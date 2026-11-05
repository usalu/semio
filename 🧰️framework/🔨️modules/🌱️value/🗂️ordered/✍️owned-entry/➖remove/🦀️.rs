//! ➖️ Native owned removal reuses the exact original key, map, update engine and retirement.
use super::*;
impl<V:RetireOwned+Sync> OriginalOwnedEntry<V>{
 /// 🗑️ Transfers actual original map and key while the existing updater receives no value.
 pub fn remove(base:OrderedMap<V>,key:String)->Self{Self{fields:ManuallyDrop::new(Fields{base:Some(base),key:Some(key),value:None,shared_key:None,shared_value:None,update:None}),retirement:Default::default(),phase:0,closing:false,progress:Default::default(),last_work:0}}
}
