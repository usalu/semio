//! 🏷️ Hands the retained opening app id into its original close slot.

use semio_framework_plugin_reactor_tasks::INSTANCE_METADATA;

/// 📮️ Hands the exact retained opening app id into its original close slot before metadata release.
pub(crate) fn take_original_instance_metadata_app_id(instance:u32,target:&mut Option<String>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<bool>,semio_framework_job::OriginalTurnFailure>{
 if target.is_some(){return Ok(Some(false))}
 INSTANCE_METADATA.with(|metadata|{
  let mut metadata=metadata.try_borrow_mut().map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original opening metadata source is borrowed"))?;
  if metadata.get(instance).is_none(){return Ok(Some(true))}
  if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(None)}
  cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);
  *target=metadata.remove(instance).map(|metadata|metadata.app_id);
  Ok(Some(false))
 })
}
