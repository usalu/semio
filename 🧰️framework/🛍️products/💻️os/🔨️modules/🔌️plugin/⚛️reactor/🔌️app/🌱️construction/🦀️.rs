//! 🌱️ Mounted instance creation retains its concrete original owner through every admission and cancellation turn.
use semio_framework::{AppDefinition};
use semio_framework_job::{StepContext,OriginalTurnFailure};
use semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1;
use semio_framework_value::{RetirementDemand,ValueError};
/// 🪪️ The retained opening borrows its immutable registered definition and exact original request identity each turn.
pub struct OriginalPluginAppSource<'a>{pub definition:&'a AppDefinition,pub definition_source:semio_framework_plugin_window_kits::RegisteredDefinitionSource<'a>,pub registration:&'a dyn OriginalPluginAppRegistration,pub actor:&'a str,pub instance_id:u32,pub policy:MountedOwnerPolicyV1}
/// 🧳️ A concrete registered producer owns partial Stores and services until genuine handoff or original closure.
pub trait OriginalPluginAppConstruction<PA>:Send{
 fn advance(&mut self,source:OriginalPluginAppSource<'_>,cx:&mut StepContext<'_>,output:&mut Option<PA>)->Result<bool,OriginalTurnFailure>;
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>;
 fn terminal_is_empty(&self)->bool;
 fn boxed_release_demands(&self)->Result<RetirementDemand,ValueError>;
}
/// 🏛️ Declaration commit produces one concrete immutable registration source before mounted requests begin.
pub trait OriginalPluginAppRegistration:Send{
 fn as_any(&self)->&dyn std::any::Any;
 fn commit_producer(&mut self);
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>;
 fn terminal_is_empty(&self)->bool;
 fn boxed_release_demands(&self)->RetirementDemand;
}
/// 🎟️ This bare declaration port admits the actual retained construction frame using its original caller.
pub struct OriginalPluginAppFactory<PA>{pub registration:fn(semio_framework_plugin_window_kits::RegisteredDefinitionSource<'_>)->Result<Box<dyn OriginalPluginAppRegistration>,semio_framework::Fault>,pub begin:fn(&mut StepContext<'_>,&mut Option<Box<dyn OriginalPluginAppConstruction<PA>>>)->Result<bool,OriginalTurnFailure>}
impl<PA> Copy for OriginalPluginAppFactory<PA>{}
impl<PA> Clone for OriginalPluginAppFactory<PA>{fn clone(&self)->Self{*self}}
