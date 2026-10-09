//! 📤️ Known host effect fields keep original typed owners through exact deferred retirement.
use crate::Effect;
use semio_framework_value::{retirement::{RetireOwned,RetirementCursor,RetirementStep},retained_clone::RetainedCloneGrant};
use std::mem::ManuallyDrop;
pub struct CompletionEffect(pub Effect);
macro_rules! birth{($($value:expr),*$(,)?)=>{semio_framework_value::retirement::sequence_birth_bytes(&[$(semio_framework_value::retirement::deferred_birth_bytes_for($value)),*])};}
macro_rules! sequence{($($value:expr),*$(,)?)=>{semio_framework_value::retirement::sequence(vec![$(semio_framework_value::retirement::deferred($value)),*])};}
impl CompletionEffect{pub fn supported(effect:&Effect)->bool{matches!(effect,Effect::Notify{..}|Effect::Navigate{..}|Effect::LoadDocument{..}|Effect::OpenExternalUrl{..}|Effect::SetPanel{..}|Effect::DownloadMediaExport{..}|Effect::RequestFileOpen{..}|Effect::SetActiveUtility{..}|Effect::SetActiveTool{..}|Effect::OpenDialog{..}|Effect::DispatchAction{..}|Effect::ReplayShellCommand{..}|Effect::ClipboardWrite{..})}}
impl CompletionEffect{pub fn original_birth(effect:&Effect)->Option<usize>{match effect{Effect::Notify{message}=>birth!(message),Effect::Navigate{uri}=>birth!(uri),Effect::LoadDocument{pack,spr}=>birth!(pack,spr),Effect::OpenExternalUrl{url}=>birth!(url),Effect::SetPanel{panel_json}=>birth!(panel_json),Effect::DownloadMediaExport{filename,mime_type,data,encoding}=>birth!(filename,mime_type,data,encoding),Effect::RequestFileOpen{req,accept,read_as,import_action,args,multiple}=>birth!(&req.0,accept,read_as,import_action,args,multiple),Effect::SetActiveUtility{window_id,utility_id}=>birth!(window_id,utility_id),Effect::SetActiveTool{tool_id}=>birth!(tool_id),Effect::OpenDialog{req,dialog_id,args}=>birth!(&req.0,dialog_id,args),Effect::DispatchAction{req,action,args,delay_ms}=>birth!(&req.0,action,args,delay_ms),Effect::ReplayShellCommand{action_id,args}=>birth!(action_id,args),Effect::ClipboardWrite{fragment}=>birth!(fragment),_=>None}}}
struct Unsupported{original:ManuallyDrop<Effect>}
impl RetirementCursor for Unsupported{
 fn close_step(&mut self,_grant:RetainedCloneGrant)->RetirementStep{RetirementStep::Failure(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original effect has no exact completion retirement facet"))}
 fn terminal_is_empty(&self)->bool{false}
 fn next_birth_bytes(&self,_copy:usize)->Option<usize>{None}
}
impl Drop for Unsupported{fn drop(&mut self){assert!(std::thread::panicking(),"unsupported original host effect lost retained custody");}}
impl RetireOwned for CompletionEffect{
 fn retirement(self)->Box<dyn RetirementCursor>{match self.0{Effect::Notify{message}=>sequence!(message),
Effect::Navigate{uri}=>sequence!(uri),
Effect::LoadDocument{pack,spr}=>sequence!(pack,spr),
Effect::OpenExternalUrl{url}=>sequence!(url),
Effect::SetPanel{panel_json}=>sequence!(panel_json),
Effect::DownloadMediaExport{filename,mime_type,data,encoding}=>sequence!(filename,mime_type,data,encoding),
Effect::RequestFileOpen{req,accept,read_as,import_action,args,multiple}=>sequence!(req.0,accept,read_as,import_action,args,multiple),
Effect::SetActiveUtility{window_id,utility_id}=>sequence!(window_id,utility_id),
Effect::SetActiveTool{tool_id}=>sequence!(tool_id),
Effect::OpenDialog{req,dialog_id,args}=>sequence!(req.0,dialog_id,args),
Effect::DispatchAction{req,action,args,delay_ms}=>sequence!(req.0,action,args,delay_ms),
Effect::ReplayShellCommand{action_id,args}=>sequence!(action_id,args),
Effect::ClipboardWrite{fragment}=>sequence!(fragment),original=>Box::new(Unsupported{original:ManuallyDrop::new(original)})}}
 fn retirement_birth_bytes(&self)->Option<usize>{match self{Self(Effect::Notify{message})=>birth!(message),
Self(Effect::Navigate{uri})=>birth!(uri),
Self(Effect::LoadDocument{pack,spr})=>birth!(pack,spr),
Self(Effect::OpenExternalUrl{url})=>birth!(url),
Self(Effect::SetPanel{panel_json})=>birth!(panel_json),
Self(Effect::DownloadMediaExport{filename,mime_type,data,encoding})=>birth!(filename,mime_type,data,encoding),
Self(Effect::RequestFileOpen{req,accept,read_as,import_action,args,multiple})=>birth!(&req.0,accept,read_as,import_action,args,multiple),
Self(Effect::SetActiveUtility{window_id,utility_id})=>birth!(window_id,utility_id),
Self(Effect::SetActiveTool{tool_id})=>birth!(tool_id),
Self(Effect::OpenDialog{req,dialog_id,args})=>birth!(&req.0,dialog_id,args),
Self(Effect::DispatchAction{req,action,args,delay_ms})=>birth!(&req.0,action,args,delay_ms),
Self(Effect::ReplayShellCommand{action_id,args})=>birth!(action_id,args),
Self(Effect::ClipboardWrite{fragment})=>birth!(fragment),_=>None}}
 fn controlled_retirement_supported()->bool{true}
}

