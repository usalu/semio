//! 📊️ Closed physical computing-progress admission before semantic host mutation.
use crate::infinite::board::schema::dag_input::DagComputingProgress;
use semio_framework_value::{DslValue,FromValue,NativeDecodeControl,ValueError};
pub(super) fn admit(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<DagComputingProgress,ValueError>{
    super::closed(value,&["active","stale"])?;
    let active=value.get("active").ok_or_else(super::refuse)?;if !active.is_null(){super::identifier(active)?;}
    super::identifiers(value.get("stale").ok_or_else(super::refuse)?,control)?;
    DagComputingProgress::from_value_controlled(value,control)
}
/// 📥️ Rejects incomplete, malformed and duplicate physical facts under the original caller control.
pub fn decode_dag_computing_progress_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<DagComputingProgress,ValueError>{
    let raw=semio_framework_pack_json::from_json_str_controlled::<DslValue>(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)?.guard_decoded();admit(raw.get(),control)
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
