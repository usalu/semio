//! 🤝️ Original actor job results and registered ownership have a first-party neutral provider.
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_value_derive::{FromValue,ToValue};

/// 📬️ Original output backings remain in their receiving owner until caller-funded handback.
#[derive(Debug,serde::Deserialize,FromValue,serde::Serialize,ToValue,semio_framework_value::RetireOwned)]
pub enum OriginalJobStep{Running(Option<Vec<u8>>),Done(Vec<u8>),Failed(Vec<u8>)}

/// 🎟️ Every registry opportunity borrows the caller's original context and its exact identity.
pub trait OriginalActorJobProvider{
 fn identity(&self)->(u64,u64);
 fn next_owned(&self)->Option<u64>;
 fn is_cancelled(&self,job:u64)->Option<bool>;
 fn cancel(&mut self,job:u64,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>;
 fn step(&mut self,job:u64,cx:&mut StepContext<'_>)->Result<Option<OriginalJobStep>,OriginalTurnFailure>;

}

/// 🪪️ Inline metadata names the same retained job output without transferring its body.
#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize,ToValue,FromValue)]
#[serde(rename_all="camelCase")]
#[value(rename_all="camelCase")]
pub enum OriginalJobOutputState{Running,Done,Failed}
semio_framework_value::artifact_retire_leaf!(OriginalJobOutputState);
/// 📬️ The body remains in its original Entry owner until read and paid full retirement complete.
#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize,ToValue,FromValue,semio_framework_value::RetireOwned)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(rename_all="camelCase")]
pub struct OriginalJobOutcome{pub job:u64,pub operation:u64,pub generation:u64,pub state:OriginalJobOutputState,pub byte_length:u64}
/// 📦️ One paid body span advances its original offset and acknowledges actual source closure.
#[derive(Debug,serde::Serialize,serde::Deserialize,ToValue,FromValue,semio_framework_value::RetireOwned)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(rename_all="camelCase")]
pub struct OriginalJobOutputChunk{pub offset:u64,pub bytes:Option<Vec<u8>>,pub output_closed:bool}

/// 🧾️ Returns exact child spending before optional original output or complete-source acknowledgement.
#[derive(Debug,serde::Serialize,serde::Deserialize,semio_framework_value::RetireOwned)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct OriginalJobOutputReply{pub input:crate::RetainedTurnInput,pub spent:semio_framework_value::RetainedCloneProgress,pub work_units_spent:u64,pub chunk:Option<OriginalJobOutputChunk>,pub source_closed:bool,pub refusal:Option<crate::OriginalJobCallFailure>}

/// 🤝️ A bounded original acknowledgement retires only the exact offered output source identity.
#[derive(Debug,serde::Serialize,serde::Deserialize,semio_framework_value::RetireOwned)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct OriginalJobOutputAckReply{pub input:crate::RetainedTurnInput,pub spent:semio_framework_value::RetainedCloneProgress,pub work_units_spent:u64,pub acknowledged:bool,pub source_closed:bool,pub refusal:Option<crate::OriginalJobCallFailure>}
