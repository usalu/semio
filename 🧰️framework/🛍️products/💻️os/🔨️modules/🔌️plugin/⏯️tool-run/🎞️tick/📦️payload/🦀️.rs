//! 🎞️ Original tick wire chunks enter the same preadmitted payload and caller context one paid unit at a time.
use semio_framework_job::{StepContext};
use semio_framework_job_payload::{RetainedPayloadBuilder};
use semio_framework_tool_run::{ToolRunTickWriter};
use semio_framework_tool_run_tick_wire::{ToolRunTickWireCursor, ToolRunTickPayloadSource};
use semio_framework_value::ValueError;
use semio_framework_value::retained_clone::RetainedCloneProgress;

#[doc(hidden)]
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
pub mod tests;

/// 🤝️ Borrows the original writer, cursor, preadmitted payload storage and append position throughout the turn.
pub fn advance_original_tick_payload(writer:&ToolRunTickWriter,source:Option<&dyn ToolRunTickPayloadSource>,cursor:&mut ToolRunTickWireCursor,payload:&mut RetainedPayloadBuilder,position:&mut usize,cx:&mut StepContext<'_>)->Result<bool,ValueError>{
 if !payload.is_initialized(){let turn=payload.advance_initialization(cx);cx.consume_work(turn.work);turn.result?;return Ok(false);}
 if !cursor.chunk().is_empty(){
  if *position<cursor.chunk().len(){let turn=payload.append_original(cx,cursor.chunk(),position);cx.consume_work(turn.work);turn.result?;return Ok(false);}
  let receipt=cursor.acknowledge_chunk(cx.retained_grant());cx.consume_retained(receipt)?;if receipt.copied_items>0{*position=0;}return Ok(false);
 }
 if !cursor.is_complete(){let receipt=cursor.advance(writer,source,cx.retained_grant())?;cx.consume_retained(receipt)?;return Ok(false);}
 if payload.published().is_none(){let turn=payload.seal(cx);cx.consume_work(turn.work);return Ok(matches!(turn.result?,semio_framework_value::RetainedCloneStep::Complete(_)));}
 Ok(true)
}

/// ♻️ Cancellation keeps the original chunk and physical payload owners until each full receipt is recorded.
pub fn close_original_tick_payload(cursor:&mut ToolRunTickWireCursor,payload:&mut RetainedPayloadBuilder,position:&mut usize,cx:&mut StepContext<'_>)->Result<bool,ValueError>{
 if !cursor.terminal_is_empty(){let receipt=cursor.close_step(cx.retained_grant());cx.consume_retained(receipt)?;return Ok(false);}
 if !payload.terminal_is_empty(){let grant=cx.retained_grant();let Some((turn,performed))=cx.with_original_effect(grant,|_,effect|payload.close_step(effect))?else{return Ok(false)};cx.consume_work(performed);let step=turn.result?;cx.consume_retained(step.progress())?;return Ok(false);}
 if *position>0{let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(false);}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;*position=0;return Ok(false);}
 Ok(true)
}
