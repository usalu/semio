//! 🔬️ Test-only observations borrow genuine value roots and never issue or transfer arena authority.
use super::*;
pub struct UiValueObservation{roots:Vec<UiCollectionHandle>}
impl UiValueObservation{
    pub fn capture(value:&UiValue)->Result<Self,&'static str>{
        let arena=UI_VALUE_ARENA.try_lock().map_err(|_|"value observation arena unavailable")?;
        let handle=|value:&UiValue|match value{UiValue::List(value)=>value.handle,UiValue::Map(value)=>value.handle,_=>None};
        let mut pending:Vec<_>=handle(value).into_iter().collect();let mut roots=Vec::new();let mut pages=0;
        while let Some(root)=pending.pop(){
            if roots.contains(&root){continue}
            if roots.len()>=UI_VALUE_ADMISSION_SLOTS{return Err("bounded observation root census exhausted")}
            roots.push(root);let mut page=arena.collection(root).ok_or("observation source stale")?.head;
            while page!=UI_VALUE_NONE{
                if pages>=UI_VALUE_AGGREGATE_ITEMS{return Err("bounded observation page census exhausted")}
                pages+=1;let current=arena.pages.get(page).ok_or("observation page stale")?;
                let value=match current.value.as_ref().ok_or("observation page empty")?{UiPageValue::List(value)|UiPageValue::Map(_,value)=>value};
                if let Some(nested)=handle(value){pending.push(nested)}page=current.next;
            }
        }
        Ok(Self{roots})
    }
    pub fn all_retired(&self)->Result<bool,&'static str>{let arena=UI_VALUE_ARENA.try_lock().map_err(|_|"value observation arena unavailable")?;Ok(self.roots.iter().all(|root|arena.collection(*root).is_none()))}
    pub fn all_live(&self)->Result<bool,&'static str>{let arena=UI_VALUE_ARENA.try_lock().map_err(|_|"value observation arena unavailable")?;Ok(self.roots.iter().all(|root|arena.collection(*root).is_some()))}
}
/// 🔒️ Holds the actual private arena while a native law observes a nonblocking refusal.
pub fn with_value_arena_held<R>(run:impl FnOnce()->R)->R{let _held=UI_VALUE_ARENA.lock().expect("actual fixture arena available");run()}
