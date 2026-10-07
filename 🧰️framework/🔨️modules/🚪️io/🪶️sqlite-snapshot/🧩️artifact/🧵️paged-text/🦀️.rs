//! 🧵️ Borrowed paged text pays only its actual owned SQLite output and yields cancellation at every chunk.
use super::{SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind};
use semio_framework_value::{NativeEncodeControl,native_encoding::NativeEncodeProgress,paged::Utf8Text};

pub(super) fn copy_text(text:&dyn Utf8Text,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<String,ValueError>{
 control.allocation_stage(phase,|remaining,progress|{
  let mut callback=|event:NativeEncodeProgress|progress(event.completed,event.total);
  let mut native=NativeEncodeControl::new(remaining,&mut callback);
  let result=native.scoped_stage(|native|{
   let length=text.text_bytes();native.begin_stage(length)?;native.charge(length)?;
   let mut output=String::new();output.try_reserve_exact(length).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"paged SQLite text allocation failed"))?;
   for index in 0..text.text_chunk_count(){
    let chunk=text.text_chunk(index).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"paged SQLite text omitted a declared chunk"))?;
    if output.len().checked_add(chunk.len()).is_none_or(|next|next>length){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"paged SQLite text exceeds its declared bytes"));}
    output.push_str(chunk);native.advance(chunk.len())?;
   }
   if output.len()!=length{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"paged SQLite text differs from its declared bytes"));}Ok(output)
  });(result,native.owned_bytes())
 })?
}
