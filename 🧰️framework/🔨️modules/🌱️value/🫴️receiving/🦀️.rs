//! 🫴️ Foreign construction borrows one original directional native admission ledger.
use crate::{ValueError,ValueRefusalKind};
pub(crate) fn invariant()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"foreign native receiving ledger disagrees with its original directional owner")}

macro_rules! scoped_encoding_receiver {
 ($original:ident,$operation:ident,$native_allocation:ident,$native_progress:ident)=>{{
  $original.checkpoint()?;
  let start=$original.owned_bytes;let ceiling=$original.maximum_bytes;let maximum=ceiling.checked_sub(start).ok_or_else(crate::value::native_receiving::invariant)?;
  let seen=std::sync::atomic::AtomicUsize::new(start);let refusal=std::sync::Mutex::new(None::<ValueError>);let mut work=None::<(usize,usize)>;
  let callback=&mut $original.callback;let completed=&mut $original.completed;let total=&mut $original.total;let started=&mut $original.started;let stage=&mut $original.stage;
  let mut observer=|progress:crate::native_encoding::NativeEncodeProgress|{
   if refusal.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(){return false}
   let result=(||{if start.checked_add(progress.owned_bytes)!=Some(seen.load(std::sync::atomic::Ordering::Relaxed))||(progress.total!=0&&progress.completed>progress.total){return Err(crate::value::native_receiving::invariant())}
    if work.is_none_or(|(previous,previous_total)|previous_total!=progress.total||previous>progress.completed){*stage=stage.checked_add(1).ok_or_else(crate::value::native_receiving::invariant)?;}
    *completed=progress.completed;*total=progress.total;*started=true;work=Some((progress.completed,progress.total));
    if (**callback)($native_progress{completed:*completed,total:*total,owned_bytes:seen.load(std::sync::atomic::Ordering::Relaxed)}){Ok(())}else{Err(ValueError::literal(crate::ValueRefusalKind::Canceled,"original native receiving observer canceled"))}
   })();match result{Ok(())=>true,Err(error)=>{let mut first=refusal.lock().unwrap_or_else(std::sync::PoisonError::into_inner);if first.is_none(){*first=Some(error)}false}}
  };
  let original_owned=&mut $original.owned_bytes;let original_port=&mut $original.allocation;
  let mut allocate=|request:crate::native_encoding::NativeEncodeAllocation|{
   if refusal.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(){return Err(crate::value::native_receiving::invariant())}
   let result=(||{let local=original_owned.checked_sub(start).ok_or_else(crate::value::native_receiving::invariant)?;if request.maximum_bytes>maximum||request.owned_bytes!=local||local.checked_add(request.bytes)!=Some(request.next_owned_bytes)||request.next_owned_bytes>request.maximum_bytes{return Err(crate::value::native_receiving::invariant())}
    let next=original_owned.checked_add(request.bytes).filter(|next|*next<=ceiling).ok_or_else(crate::value::native_receiving::invariant)?;
    if let allocation::Binding::Forwarded(port)=original_port{port($native_allocation{bytes:request.bytes,owned_bytes:*original_owned,next_owned_bytes:next,maximum_bytes:ceiling})?;}
    *original_owned=next;seen.store(next,std::sync::atomic::Ordering::Relaxed);Ok(())
   })();if let Err(error)=result{let mut first=refusal.lock().unwrap_or_else(std::sync::PoisonError::into_inner);if first.is_none(){*first=Some(error)}return Err(crate::value::native_receiving::invariant())}Ok(())
  };
  let result=$operation(maximum,&mut observer,&mut allocate);if let Some(error)=refusal.into_inner().unwrap_or_else(std::sync::PoisonError::into_inner){return Err(E::from(error))}result
 }};
}
pub(crate) use scoped_encoding_receiver;

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
