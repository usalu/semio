//! 🧩️ Extension resources, topic and command of the wasm extension glue: the extension evaluation resources and their plugin owner impl, the topic contribution and the command JSON. See [`super`].

use super::*;

// #region ⏱️EvaluationJobs
/// 🔌️ Retains the original extension registry and its existing exact retirement authority.
pub struct ExtensionEvaluationResources {
    pub invoke: std::sync::Mutex<Option<EvaluationInvokeCursor>>,
    original_registry: Option<Registry>,
    registry_issuer: neural_engine::SharedRegistryIssuer,
    registry_source_close: Option<Box<dyn ErasedSnapshotRetirement>>,
    pub registry: Option<neural_engine::SharedRegistry>,
    pub(super) lease: Option<neural_engine::RegistryLeaseRetirement>,
    pub(super) retirement: Option<neural_engine::RegistryRetirement>,
    pub(super) close_failure: Option<original_failure::EvaluationCloseFailure>,
    pub(super) closing: bool,
}

impl ExtensionEvaluationResources {
    pub fn new(registry: Registry) -> Self {
        Self { invoke: std::sync::Mutex::new(None), original_registry: Some(registry), registry_issuer: neural_engine::SharedRegistryIssuer::new(), registry_source_close: None, registry: None, lease: None, retirement: None, close_failure: None, closing: false }
    }
    pub fn registry(&self) -> Option<&neural_engine::SharedRegistry> { self.registry.as_ref() }
    /// 🧪️ Names the fields that still hold a value.
    #[cfg(test)]
    pub fn debug_live_fields(&self)->String{format!("invoke={} original_registry={} issuer_empty={} source_close={} registry={} lease={} retirement={} failure={}",self.invoke.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(),self.original_registry.is_some(),self.registry_issuer.terminal_is_empty(),self.registry_source_close.is_some(),self.registry.is_some(),self.lease.is_some(),self.retirement.is_some(),self.close_failure.is_some())}
    pub fn admit_registry_on_original(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<bool>, semio_framework_job::OriginalTurnFailure> {
        if self.registry.is_some() { return Ok(Some(true)); }
        if self.closing { return Ok(None); }
        let admitted = self.registry_issuer.step(&mut self.original_registry, cx)?;
        if let Some((registry, retirement)) = admitted.ready {
            self.registry = Some(registry);
            self.retirement = Some(retirement);
        }
        Ok(if admitted.progress == RetainedCloneProgress::default() { None } else { Some(false) })
    }
    pub fn source_close(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,EvaluationFailure>{
        let empty=RetainedCloneProgress::default();
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        let pending=self.invoke.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(cursor)=pending {let step=match self.registry.as_ref(){Some(registry)=>cursor.close_step_with_registry(registry,grant)?,None=>cursor.close_step(grant)?};if cursor.terminal_is_empty(){*pending=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
        if let Some(registry)=&self.registry{
            if evaluation_retirement_pending(registry){return retire_cancelled_evaluations_close_step(registry,grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
            if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"evaluation resource lease handoff requires admitted depth").into());}
            self.lease=Some(self.registry.take().unwrap().into_retirement());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..empty}));
        }
        if let Some(lease)=&mut self.lease{
            let step=lease.close_step(grant)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,lease.terminal_is_empty(),"extension evaluation source lease")?;
            if lease.terminal_is_empty(){self.lease=None;}return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.registry_source_close.is_some(){return evaluation_child_close(&mut self.registry_source_close,grant).map_err(Into::into);}
        if self.original_registry.is_some(){return evaluation_admit_original(&mut self.original_registry,&mut self.registry_source_close,grant).map_err(Into::into);}
        let Some(retirement)=self.retirement.as_mut() else{return Ok(RetainedCloneStep::Complete(empty));};
        let step=retirement.close_step(grant)?;
        if retirement.terminal_is_empty(){self.retirement=None;}
        Ok(step)
    }
}

impl semio_framework_plugin::ExtensionResourceOwner for ExtensionEvaluationResources {
    fn retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        let pending=self.invoke.lock().unwrap_or_else(std::sync::PoisonError::into_inner);if self.close_failure.is_some(){return Ok(semio_framework_job::original_domain_failure_demand::<original_failure::EvaluationCloseFailure>());}if let Some(cursor)=pending.as_ref(){return match self.registry.as_ref(){Some(registry)=>cursor.retirement_demands_with_registry(registry,copy),None=>cursor.retirement_demands(copy)};}
        if let Some(registry)=&self.registry{if evaluation_retirement_pending(registry){return evaluation_retirement_demands(registry,copy);}return Ok(RetirementDemand {depth:1,..Default::default()});}
        if let Some(lease)=&self.lease{return Ok(RetirementDemand {copy_bytes:lease.next_copy_byte_demand()?,capacity_bytes:lease.next_capacity_byte_demand(copy)?,release_bytes:lease.next_release_byte_demand()?,depth:lease.next_depth_demand()?});}
        if !self.registry_issuer.terminal_is_empty(){return self.registry_issuer.next_demand(copy);}
        if self.registry_source_close.is_some(){return evaluation_child_demands(&self.registry_source_close,copy);}
        if self.original_registry.is_some(){return Ok(evaluation_owned_birth::<Registry>());}
        let Some(retirement)=self.retirement.as_ref() else{return Ok(Default::default());};
        Ok(RetirementDemand {copy_bytes:retirement.next_copy_byte_demand()?,capacity_bytes:retirement.next_capacity_byte_demand(copy)?,release_bytes:retirement.next_release_byte_demand()?,depth:retirement.next_depth_demand()?})
    }
    fn invoke(&mut self,capability:&str,request:&[u8],cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_plugin::ExtensionInvokeStep,semio_framework::Fault>{
        match capability {"evaluate"=>evaluate_invoke_json(self,request,cx),_=>Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin,semio_framework::FaultCode::new("extension.unknown-capability"),"unknown flow evaluation capability"))}
    }

    fn cancel_inference(&self,cancellation_id:&str)->bool{self.registry.as_ref().is_some_and(|registry|cancel_inference_evaluation(registry,cancellation_id))}
    fn begin_close(&mut self){self.closing=true;self.registry_issuer.begin_close();if let Some(cursor)=self.invoke.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner){cursor.begin_close();}if let Some(registry)=&self.registry{cancel_all_evaluations(registry);}}
    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_plugin::PluginLifecycleStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_plugin::PluginLifecycleStep;
        if self.terminal_is_empty(){return Ok(PluginLifecycleStep::Complete(Default::default()))}
        if self.close_failure.is_some(){return match semio_framework_job::admit_original_domain_failure(&mut self.close_failure,cx)?{Some(failure)=>Err(failure),None=>Ok(PluginLifecycleStep::Progress(Default::default()))}}
        if !self.registry_issuer.terminal_is_empty(){
            if self.invoke.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_none_or(EvaluationInvokeCursor::terminal_is_empty){
                let grant=cx.retained_grant();
                if self.invoke.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(){
                    if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(PluginLifecycleStep::Progress(Default::default()));}
                    let progress=RetainedCloneProgress{copied_items:1,..Default::default()};
                    cx.consume_retained(progress)?;cx.consume_fuel(1);
                    self.invoke.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                    return Ok(PluginLifecycleStep::Progress(progress));
                }
                let step=self.registry_issuer.close_step(&mut self.original_registry,cx)?;
                return Ok(PluginLifecycleStep::Progress(step.progress()));
            }
        }
        let remaining=cx.retained_grant();let demand=self.retirement_demands(remaining.maximum_copy_bytes)?;
        if cx.should_yield()||remaining.maximum_items==0||remaining.maximum_copy_bytes<demand.copy_bytes||remaining.maximum_capacity_bytes<demand.capacity_bytes||remaining.maximum_release_bytes<demand.release_bytes||remaining.maximum_depth<demand.depth{return Ok(PluginLifecycleStep::Progress(Default::default()))}
        cx.consume_fuel(1);let grant=RetainedCloneGrant{maximum_items:1,..remaining};
        match self.source_close(grant){
            Ok(step)=>{let progress=step.progress();if let Err(refusal)=cx.consume_retained(progress){self.close_failure=Some(original_failure::EvaluationCloseFailure{initial:None,closure:refusal.into(),receipt_refusal:None});return Ok(PluginLifecycleStep::Progress(progress))}Ok(PluginLifecycleStep::retained(step,self.terminal_is_empty()))}
            Err(closure)=>{let progress=closure.retained_progress();let receipt_refusal=cx.consume_retained(progress).err();self.close_failure=Some(original_failure::EvaluationCloseFailure{initial:None,closure,receipt_refusal});Ok(PluginLifecycleStep::Progress(progress))}
        }
    }
    fn terminal_is_empty(&self)->bool{self.close_failure.is_none()&&self.invoke.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()&&self.registry.is_none()&&self.lease.is_none()&&self.registry_issuer.terminal_is_empty()&&self.original_registry.is_none()&&self.registry_source_close.is_none()&&self.retirement.as_ref().is_none_or(neural_engine::RegistryRetirement::terminal_is_empty)}
}
// #endregion ⏱️EvaluationJobs

// #region 🔖️TopicContribution
/// 🗺️ Builds the `"flow.extension"` topic contribution one host app (`flow-play`,
/// `procedural3d-play`) consumes — factored out of every flow extension's own guest bundle so those
/// crates never need `serde_json` themselves. See
/// `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs::TopicContribution`.
pub fn flow_extension_topic_contribution(app_id: &str, extension_id: &str, label: &str, icon_id: &str, manifest_json: &str) -> semio_framework::TopicContribution {
    semio_framework::TopicContribution::new(
        "flow.extension",
        DslValue::object([
            ("appId".to_string(), DslValue::String(app_id.to_string())),
            ("extensionId".to_string(), DslValue::String(extension_id.to_string())),
            ("label".to_string(), DslValue::String(label.to_string())),
            ("iconId".to_string(), DslValue::String(icon_id.to_string())),
            ("manifestJson".to_string(), DslValue::String(manifest_json.to_string())),
        ]),
    )
}
// #endregion 🔖️TopicContribution

// #region 🔖️Command
/// ⌘️ Stub command handler returning acknowledgement JSON.
pub fn command_json(command_id: &str, args_json: &str) -> String {
    semio_framework_pack_json::to_json_string(&DslValue::object([
        ("ok".to_string(), true.to_value()),
        ("commandId".to_string(), command_id.to_value()),
        ("args".to_string(), args_json.to_value()),
    ]))
}
// #endregion 🔖️Command
