//! 🎯️ Controlled physical admission and publication for typed DAG input facts.
use crate::infinite::board::schema::dag_input::{DagChannelRef,DagNodeStatuses,DagSelectionDomains};
use semio_framework_value::{DslValue,FromValue,NativeDecodeControl,NativeEncodeControl,ValueError,ValueRefusalKind};

fn refuse()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"invalid DAG input facts")}
fn closed(value:&DslValue,names:&[&str])->Result<(),ValueError>{let fields=value.as_object().ok_or_else(refuse)?;if fields.len()!=names.len()||fields.iter().any(|(key,_)|!names.contains(&key.as_str())){return Err(refuse())}Ok(())}
fn identifier(value:&DslValue)->Result<(),ValueError>{if value.as_str().is_none_or(str::is_empty){return Err(refuse())}Ok(())}
fn identifiers(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{for entry in value.as_array().ok_or_else(refuse)?{control.step()?;identifier(entry)?;}Ok(())}

/// 📥️ Refuses malformed selection before any semantic host changes.
pub fn decode_dag_selection_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<DagSelectionDomains,ValueError>{
    let raw=semio_framework_pack_json::from_json_str_controlled::<DslValue>(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)?.guard_decoded();
    closed(raw.get(),&["nodes","edges","handles"])?;
    for key in ["nodes","edges","handles"]{identifiers(raw.get().get(key).ok_or_else(refuse)?,control)?;}
    DagSelectionDomains::from_value_controlled(raw.get(),control)
}

/// 🔌️ Admits a complete port list independently of graph selection mutation.
pub fn decode_dag_channels_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<Vec<DagChannelRef>,ValueError>{
    let raw=semio_framework_pack_json::from_json_str_controlled::<DslValue>(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)?.guard_decoded();
    for row in raw.get().as_array().ok_or_else(refuse)?{control.step()?;closed(row,&["widgetId","port","direction"])?;identifier(row.get("widgetId").ok_or_else(refuse)?)?;identifier(row.get("port").ok_or_else(refuse)?)?;if !matches!(row.get("direction").and_then(DslValue::as_str),Some("in"|"out")){return Err(refuse())}}
    Vec::<DagChannelRef>::from_value_controlled(raw.get(),control)
}

/// 🚦️ Admits every status row before replacing accepted host chrome.
pub fn decode_dag_node_statuses_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<DagNodeStatuses,ValueError>{
    let raw=semio_framework_pack_json::from_json_str_controlled::<DslValue>(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)?.guard_decoded();
    for (id,row)in raw.get().as_object().ok_or_else(refuse)?{
        control.step()?;if id.is_empty(){return Err(refuse())}
        match row.get("status").and_then(DslValue::as_str){
            Some("ok"|"queued"|"computing")=>closed(row,&["status"] )?,
            Some("error")=>{closed(row,&["status","message"])?;if row.get("message").and_then(DslValue::as_str).is_none(){return Err(refuse())}},
            Some("blocked")=>{closed(row,&["status","ports"])?;identifiers(row.get("ports").ok_or_else(refuse)?,control)?},
            _=>return Err(refuse()),
        }
    }
    DagNodeStatuses::from_value_controlled(raw.get(),control)
}

/// 📤️ Publishes typed selection through the caller's cumulative output authority.
pub fn encode_dag_selection_json(value:&DagSelectionDomains,control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{semio_framework_pack_json::to_json_string_controlled(value,control)}
/// 📤️ Publishes typed channels through the same explicit output authority.
pub fn encode_dag_channels_json(value:&Vec<DagChannelRef>,control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{semio_framework_pack_json::to_json_string_controlled(value,control)}
