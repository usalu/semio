//! 🎯️ Typed selection and evaluation facts shared by DAG host receivers.
use serde::{Deserialize,Serialize};
use semio_framework_value_derive::{FromValue,ToValue};
use std::collections::BTreeMap;

#[derive(Clone,Debug,Default,PartialEq,Serialize,Deserialize,ToValue,FromValue)]
#[serde(deny_unknown_fields)]
pub struct DagSelectionDomains {pub nodes:Vec<String>,pub edges:Vec<String>,pub handles:Vec<String>}

#[derive(Clone,Debug,PartialEq,Serialize,Deserialize,ToValue,FromValue)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(rename_all="camelCase")]
pub struct DagChannelRef {pub widget_id:String,pub port:String,pub direction:String}
impl DagChannelRef {
    pub fn is_input(&self)->bool{self.direction=="in"}
    pub fn is_output(&self)->bool{self.direction=="out"}
}

#[derive(Clone,Debug,PartialEq,Serialize,Deserialize,ToValue,FromValue)]
#[serde(tag="status",rename_all="camelCase",deny_unknown_fields)]
#[value(tag="status",rename_all="camelCase")]
pub enum DagNodeEvaluationStatus {Ok,Queued,Computing,Error{message:String},Blocked{ports:Vec<String>}}

/// 🚦️ Holds admitted per-node evaluation facts independently of physical field spelling.
pub type DagNodeStatuses=BTreeMap<String,DagNodeEvaluationStatus>;
