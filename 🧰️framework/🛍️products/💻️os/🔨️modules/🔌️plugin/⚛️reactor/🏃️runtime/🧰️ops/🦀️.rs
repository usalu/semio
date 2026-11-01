//! ⚙️ Runtime operations of the plugin SDK: wire limits, codecs, document ops, media, surfaces, render, refresh, context menu, command ingress and typed operations.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;

/// 🧮️ Weighs the allocator events that the paid-receipt laws of this crate's own test binary compare with their receipts.
#[cfg(test)]
#[global_allocator]
static RUNTIME_OPS_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

pub(crate) fn built_with<O>(cx:&mut semio_framework_job::StepContext<'_>,output:&mut Option<O>,demand:semio_framework_job::OriginalConstructionDemand,construct:impl FnOnce(&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(O,semio_framework_value::RetainedCloneProgress),semio_framework_value::ValueError>)->Result<bool,semio_framework_job::OriginalTurnFailure>{semio_framework_job::admit_original_construction(cx,output,demand,|control|semio_framework_value::RetainedWorkOutcome{result:construct(control),work:1})}
pub(crate) fn built<O>(cx:&mut semio_framework_job::StepContext<'_>,output:&mut Option<O>,demand:semio_framework_job::OriginalConstructionDemand,construct:impl FnOnce()->Result<(O,semio_framework_value::RetainedCloneProgress),semio_framework_value::ValueError>)->Result<bool,semio_framework_job::OriginalTurnFailure>{semio_framework_job::admit_original_construction(cx,output,demand,|_|semio_framework_value::RetainedWorkOutcome{result:construct(),work:1})}
pub(crate) fn retired(cx:&mut semio_framework_job::StepContext<'_>,demand:semio_framework_value::RetirementDemand,advance:impl FnOnce(semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{semio_framework_job::close_original_retained_step(cx,demand,|grant|semio_framework_value::RetainedWorkOutcome{result:advance(grant),work:1})}

#[path = "🛡️wire-limits/🦀️.rs"]
pub mod ops_wire_limits;
#[path = "📄️document/🦀️.rs"]
pub mod ops_document;
#[path = "🎞️media/🦀️.rs"]
pub mod ops_media;
#[path = "🖥️surface-render/🦀️.rs"]
pub mod ops_surface_render;
#[path = "📥️ingress/🦀️.rs"]
pub mod ops_ingress;
#[path = "🧾️typed-operations/🦀️.rs"]
pub mod ops_typed_operations;

#[doc(hidden)]
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub mod tests;
#[cfg(test)]
extern crate semio_framework_os_kernel as dsl;
