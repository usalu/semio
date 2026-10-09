//! 📊️ Admitted computing identities shared by evaluation and graph chrome.
#[cfg(test)]
use serde::{Deserialize,Serialize};
use semio_framework_value_derive::{FromValue,ToValue};
#[derive(Clone,Debug,Default,PartialEq,FromValue,ToValue)]
#[cfg_attr(test,derive(Serialize,Deserialize))]
#[cfg_attr(test,serde(deny_unknown_fields))]
#[value(deny_unknown_fields)]
pub struct DagComputingProgress {pub active:Option<String>,pub stale:Vec<String>}
