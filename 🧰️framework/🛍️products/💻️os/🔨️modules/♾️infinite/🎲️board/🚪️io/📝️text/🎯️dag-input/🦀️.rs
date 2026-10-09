//! 🎯️ Controlled physical admission and publication for typed DAG input facts.
#[path="📤️selection/🦀️.rs"]
pub mod selection;
use crate::infinite::board::schema::dag_input::{DagChannelRef,DagNodeStatuses,DagSelectionDomains,DagHoverFacts};
use semio_framework_value::{DslValue,FromValue,NativeDecodeControl,NativeEncodeControl,ValueError,ValueRefusalKind};

fn refuse()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"invalid DAG input facts")}
fn closed(value:&DslValue,names:&[&str])->Result<(),ValueError>{let fields=value.as_object().ok_or_else(refuse)?;if fields.len()!=names.len()||fields.iter().any(|(key,_)|!names.contains(&key.as_str())){return Err(refuse())}Ok(())}
fn identifier(value:&DslValue)->Result<(),ValueError>{if value.as_str().is_none_or(str::is_empty){return Err(refuse())}Ok(())}
fn identifiers(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{for entry in value.as_array().ok_or_else(refuse)?{control.step()?;identifier(entry)?;}Ok(())}
fn channel(value:&DslValue)->Result<(),ValueError>{closed(value,&["widgetId","port","direction"])?;identifier(value.get("widgetId").ok_or_else(refuse)?)?;identifier(value.get("port").ok_or_else(refuse)?)?;if !matches!(value.get("direction").and_then(DslValue::as_str),Some("in"|"out")){return Err(refuse())}Ok(())}

/// 📥️ Refuses malformed selection before any semantic host changes.
pub fn decode_dag_selection_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<DagSelectionDomains,ValueError>{
    let raw=semio_framework_pack_json::from_json_str_controlled::<DslValue>(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)?.guard_decoded();
    admit_dag_selection_value(raw.get(),control)
}

fn admit_dag_selection_value(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<DagSelectionDomains,ValueError>{
    closed(value,&["nodes","edges","handles"])?;
    for key in ["nodes","edges","handles"]{identifiers(value.get(key).ok_or_else(refuse)?,control)?;}
    DagSelectionDomains::from_value_controlled(value,control)
}

/// 🔌️ Admits a complete port list independently of graph selection mutation.
pub fn decode_dag_channels_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<Vec<DagChannelRef>,ValueError>{
    let raw=semio_framework_pack_json::from_json_str_controlled::<DslValue>(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)?.guard_decoded();
    admit_dag_channels_value(raw.get(),control)
}

fn admit_dag_channels_value(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Vec<DagChannelRef>,ValueError>{
    for row in value.as_array().ok_or_else(refuse)?{control.step()?;channel(row)?;}
    Vec::<DagChannelRef>::from_value_controlled(value,control)
}

/// 🚦️ Admits every status row before replacing accepted host chrome.
pub fn decode_dag_node_statuses_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<DagNodeStatuses,ValueError>{
    let raw=semio_framework_pack_json::from_json_str_controlled::<DslValue>(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)?.guard_decoded();
    admit_dag_node_statuses_value(raw.get(),control)
}

fn admit_dag_node_statuses_value(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<DagNodeStatuses,ValueError>{
    for (id,row)in value.as_object().ok_or_else(refuse)?{
        control.step()?;if id.is_empty(){return Err(refuse())}
        match row.get("status").and_then(DslValue::as_str){
            Some("ok"|"queued"|"computing")=>closed(row,&["status"] )?,
            Some("error")=>{closed(row,&["status","message"])?;if row.get("message").and_then(DslValue::as_str).is_none(){return Err(refuse())}},
            Some("blocked")=>{closed(row,&["status","ports"])?;identifiers(row.get("ports").ok_or_else(refuse)?,control)?},
            _=>return Err(refuse()),
        }
    }
    DagNodeStatuses::from_value_controlled(value,control)
}

/// 📤️ Publishes typed selection through the caller's cumulative output authority.
pub fn encode_dag_selection_json(value:&DagSelectionDomains,control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{semio_framework_pack_json::to_json_string_controlled(value,control)}
/// 📤️ Publishes typed channels through the same explicit output authority.
pub fn encode_dag_channels_json(value:&Vec<DagChannelRef>,control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{semio_framework_pack_json::to_json_string_controlled(value,control)}
/// 🎯️ Publishes projected semantic identities under explicit output authority.
pub fn encode_dag_node_ids_json(value:&Vec<String>,control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{semio_framework_pack_json::to_json_string_controlled(value,control)}

/// 🖱️ Admits complete hover facts before publishing an interaction projection.
pub fn decode_dag_hover_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<DagHoverFacts,ValueError>{
    let raw=semio_framework_pack_json::from_json_str_controlled::<DslValue>(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)?.guard_decoded();closed(raw.get(),&["channel","refusal"])?;
    let hovered=raw.get().get("channel").ok_or_else(refuse)?;if !hovered.is_null(){control.step()?;channel(hovered)?;}
    let refusal=raw.get().get("refusal").ok_or_else(refuse)?;if !refusal.is_null(){control.step()?;closed(refusal,&["source","sourceTypes","target","targetTypes"])?;for key in ["source","target"]{identifier(refusal.get(key).ok_or_else(refuse)?)?;}for key in ["sourceTypes","targetTypes"]{identifiers(refusal.get(key).ok_or_else(refuse)?,control)?;}}
    DagHoverFacts::from_value_controlled(raw.get(),control)
}

/// 📤️ Publishes the declared hover and refusal record under cumulative output authority.
pub fn encode_dag_hover_json(value:&DagHoverFacts,control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{semio_framework_pack_json::to_json_string_controlled(value,control)}

#[path="📊️progress/🦀️.rs"]
pub mod progress;

#[path="📤️output/🦀️.rs"]
pub mod output;

#[path="🧵️retained/🦀️.rs"]
pub mod retained;
