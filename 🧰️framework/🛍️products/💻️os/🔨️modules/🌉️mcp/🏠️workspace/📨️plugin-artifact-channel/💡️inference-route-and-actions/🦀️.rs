//! 💡️ Inference route and actions of the plugin artifact channel: the inference route, real inference, the addressed action invocation and the typed faults. See [`super`].

use super::*;

//#region 🔖️ArtifactChannel
#[cfg(not(target_arch = "wasm32"))]
impl PluginArtifactChannel {
    /// 🏭️ Opens the actual compiled actor through synchronous slices of its retained lifetime source.
    pub(super) fn open_original_instance(&mut self,actor:semio_framework_actor::ActorId,driver:&Arc<semio_plugin_host::GuestRelayWakeDriver>,scope:&ActivationScope)->Result<semio_plugin_host::GuestInstance,Fault>{
        use semio_plugin_host::GuestRelayAdmissionCall;
        if self.startup_custody.is_some()||self.startup_instance_custody.is_some()||self.startup_failure.is_some(){return Err(Self::not_wired("original startup","an earlier opening still owns its refused source"))}
        scope.enter(ActivationPhase::OpeningGuest)?;
        let runtime=Arc::clone(&self.runtime);
        let compiled=self.compiled.clone();
        let mut opening=None;
        let begin=driver.dispatch_admission_step(false,|_,cx|GuestRelayAdmissionCall{result:runtime.begin_original_actor_step(&compiled,actor,cx).map(|owner|{opening=owner;}),execution:None});
        match begin{Ok(true) if opening.is_some()=>{},Ok(_)=>return Err(Self::budget_fault("original startup admission")),Err(error)=>{let fault=Self::not_wired("original startup admission",format!("{error:?}"));self.startup_failure=Some(error);if let Some(owner)=opening.take(){self.startup_custody=Some((owner,Arc::clone(driver)))}return Err(fault)}}
        loop{
            if let Err(mut fault)=scope.checkpoint(){
                loop{
                    let mut closed=false;
                    let step=driver.dispatch_admission_step(false,|_,cx|GuestRelayAdmissionCall{result:runtime.close_original_actor_step(&mut opening,&compiled,cx).map(|value|{closed=value;}),execution:None});
                    match step{Ok(true) if closed=>return Err(fault),Ok(true)=>continue,Err(error)=>{fault.message=format!("{}; original startup cleanup refused: {error:?}",fault.message);self.startup_failure=Some(error);},Ok(false)=>{}}
                    if let Some(owner)=opening.take(){self.startup_custody=Some((owner,Arc::clone(driver)))}
                    return Err(fault)
                }
            }
            if opening.as_ref().is_some_and(|owner|owner.ready()){
                let mut instance=None;
                let take=driver.dispatch_admission_step(false,|_,cx|GuestRelayAdmissionCall{result:(||{let input=semio_framework_actor::RetainedTurnInput::borrow_original(cx)?;runtime.take_original_actor_step(&mut opening,&compiled,input,cx).map(|value|{instance=value;})})(),execution:None});
                match take{Ok(true) if instance.is_some()=>return Ok(instance.take().expect("funded instance transfer")),Err(error)=>{let fault=Self::not_wired("original startup transfer",format!("{error:?}"));self.startup_failure=Some(error);if let Some(owner)=opening.take(){self.startup_custody=Some((owner,Arc::clone(driver)))}if let Some(instance)=instance.take(){self.startup_instance_custody=Some((instance,Arc::clone(driver)))}return Err(fault)},Ok(_)=>{}}
            }else{
                let execution=opening.as_ref().is_some_and(|owner|owner.requires_execution());
                let advance=driver.dispatch_admission_step(execution,|execution,cx|match runtime.advance_original_actor_step(opening.as_mut().expect("retained startup"),&compiled,execution,cx){Ok(call)=>GuestRelayAdmissionCall{result:call.result.map(|_|()),execution:call.execution},Err(error)=>GuestRelayAdmissionCall{result:Err(error),execution:None}});
                match advance{Ok(true)=>continue,Err(error)=>{let fault=Self::not_wired("original startup step",format!("{error:?}"));self.startup_failure=Some(error);if let Some(owner)=opening.take(){self.startup_custody=Some((owner,Arc::clone(driver)))}return Err(fault)},Ok(false)=>{}}
            }
            if let Some(owner)=opening.take(){self.startup_custody=Some((owner,Arc::clone(driver)))}
            return Err(Self::budget_fault("original startup"))
        }
    }

    /// 💡️ Registers the committed inference roster over a separate actual actor on this compiled runtime.
    fn ensure_inference_route(&mut self,cancel:&semio_framework_async::CancelToken,funding:&semio_plugin_host::GuestRelayWakeAuthority) -> Result<&PluginInferenceRoute, Fault> {
        if self.inference.is_none() {
            let roster: Vec<_> = self
                .descriptor
                .contributions
                .inference_services
                .iter()
                .chain(self.descriptor.contributions.artifact_contributions.iter().flat_map(|contribution| contribution.inferences.iter()))
                .cloned()
                .collect();
            if roster.is_empty() {
                return Err(Fault { code: "capability.not-found".to_string(), message: format!("plugin `{}` declares no inference service in its committed descriptor", self.plugin_id) });
            }
            let roster_bytes = semio_framework_pack_json::to_json_string(&roster).into_bytes();
            let actor = ::semio_framework_async::poll::resolve_ready(semio_framework_actor::ActorId::new(INFERENCE_ACTOR_ORDINAL, 0, 1, 0));
            let runtimes = Arc::clone(&self.runtime);
            let driver=Arc::clone(&funding.receiver);
            let instance=self.open_original_instance(actor,&driver,&ActivationScope::new(cancel.clone(),|_,_|{}))?;
            let handle = Arc::new(semio_framework_async::block_on(semio_plugin_host::PluginInstanceHandle::new(actor, Arc::clone(&runtimes), self.compiled.clone(), instance,funding.clone())));
            let router = semio_plugin_host::ArtifactInferenceRouter::new();
            semio_framework_async::block_on(router.register_plugin(&self.plugin_id, &self.descriptor.manifest.dependencies, handle, &roster_bytes)).map_err(|error| Self::not_wired("inference route registration", error))?;
            self.inference = Some(PluginInferenceRoute { router });
        }
        Ok(self.inference.as_ref().expect("inference route was just opened"))
    }

    /// 🧠️ Runs the declared router with caller freshness data and a fresh finite native operation source.
    pub(crate) fn infer_real(&mut self, command: &InferCommand) -> Result<ArtifactInferenceResultV1, Fault> {
        if self.inference.is_some(){return Err(Self::not_wired("inference route","the previous request still owns its guest route until funded retirement"))}
        if self.operation_refusal.is_some(){return Err(Self::not_wired("inference operation","an earlier call retains its refused original cleanup source"))}
        let declared = self
            .descriptor
            .contributions
            .inference_services
            .iter()
            .chain(self.descriptor.contributions.artifact_contributions.iter().flat_map(|contribution| contribution.inferences.iter()))
            .find(|item| item.artifact_kind == command.artifact_kind && item.inference_schema == command.inference_schema)
            .cloned()
            .ok_or_else(|| Fault {
                code: "capability.not-found".to_string(),
                message: format!("plugin `{}` declares no inference `{}` on artifact kind `{}`", self.plugin_id, command.inference_schema, command.artifact_kind),
            })?;
        let request = ArtifactInferenceRequestV1 {
            wire_version: ARTIFACT_INFERENCE_WIRE_VERSION,
            owner: declared.owner.clone(),
            artifact_kind: declared.artifact_kind.clone(),
            artifact_schema: declared.artifact_schema.clone(),
            artifact_schema_version: declared.artifact_schema_version,
            inference_schema: declared.inference_schema.clone(),
            inference_schema_version: declared.inference_schema_version,
            algorithm_version: declared.algorithm_version,
            policy_version: declared.policy_version,
            revision: command.revision,
            generation: command.document_generation,
            source_dialect: declared.artifact_schema.clone(),
            policy: Vec::new(),
            budgets: ArtifactInferenceBudgetV1 { allocation_bytes: INFERENCE_ALLOCATION_BYTES, work_units: command.work_units, recursion_depth: INFERENCE_RECURSION_DEPTH },
            retained: command.retained_policy,
            cancellation_id: command.cancellation_id.clone(),
            previous_state: None,
            requested_cache_mode: ArtifactInferenceCacheModeV1::Cold,
            canonical_payload: bind_inference_document(&declared, command)?,
            dependencies: match command.artifact_document {
                Some(_) => self.inference_child_dependencies(&command.artifact_id)?,
                None => Vec::new(),
            },
        };
        let policy=self.operation_policy;
        let reserve=policy.cleanup;
        let caps=self.host_driver_policy.operation_caps(semio_plugin_host::GuestRelayOperationReserve{retained:reserve.retained,work_fuel:reserve.work_fuel,execution_fuel:reserve.execution_fuel,maximum_microseconds:reserve.maximum_microseconds},semio_plugin_host::GuestRelayOperationTransit{retained:policy.transit.retained,work_fuel:policy.transit.work_fuel},semio_plugin_host::GuestRelayOperationInfrastructure{retained:policy.infrastructure.retained,work_fuel:policy.infrastructure.work_fuel,maximum_cleanup_microseconds:policy.infrastructure.maximum_cleanup_microseconds});
        let requested=semio_plugin_host::GuestRelayOperationRequest{work_units:command.work_units,maximum_elapsed_microseconds:command.maximum_elapsed_milliseconds.saturating_mul(1000),retained:command.retained_policy};
        let request_bytes=semio_framework_pack_json::to_json_string(&request).into_bytes();
        let cancel=command.cancel.0.clone();
        let driver=Arc::new(caps.issue(requested,semio_framework_job::default_now_us,Box::new(move||cancel.is_cancelled_now())).map_err(|error|Self::not_wired("inference operation admission",error))?);
        let policy=match driver.policy(){Ok(policy)=>policy,Err(error)=>return self.settle_inference_operation(driver,Err(Self::not_wired("inference operation policy",error))).and_then(|_|unreachable!())};
        let funding=semio_plugin_host::GuestRelayWakeAuthority{drive_policy:policy.drive,policy:policy.wake,issuer:semio_framework_job_worker::admit_original_thread_worker_wake,receiver:Arc::clone(&driver)};
        if let Err(error)=self.ensure_inference_route(&command.cancel.0,&funding){drop(funding);return self.settle_inference_operation(driver,Err(error)).and_then(|_|unreachable!())}
        let route=self.inference.as_ref().expect("inference route was activated");
        let result=semio_framework_async::block_on(route.router.infer(&request_bytes, &command.cancel.0, &funding)).map_err(|error|Fault{code:"app.command.rejected".into(),message:format!("inference refused: {error}")});
        drop(funding);
        let result_bytes=self.settle_inference_operation(driver,result)?;
        let result_text = std::str::from_utf8(&result_bytes).map_err(|error| Self::not_wired("decoding the inference result", error))?;
        semio_framework_pack_json::from_json_str(result_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| Self::not_wired("decoding the inference result", error))
    }

    /// 🪢️ A failed close retains the exact source and both body and cleanup failures.
    fn settle_inference_operation(&mut self,driver:Arc<semio_plugin_host::GuestRelayWakeDriver>,result:Result<Vec<u8>,Fault>)->Result<Vec<u8>,Fault>{
        match semio_framework_async::block_on(driver.close_operation()){
            Ok(())=>result,
            Err(error)=>{
                let message=format!("inference cleanup refused: {error:?}; body: {:?}",result.as_ref().err());
                self.operation_refusal=Some((driver,error,result.err()));
                Err(Self::not_wired("inference operation retirement",message))
            }
        }
    }

    /// 🎯️ Resolves an action from its declared owner; PureCommandRequest binds the actual caller surface.
    pub(crate) fn addressed_action_invocation(&self, capability_id: &str, arguments: impl FnOnce() -> Result<std::collections::BTreeMap<String, semio_framework_value::DslValue>, String>) -> Result<semio_framework::manifest::ActionInvocation, Fault> {
        let plugin_id = self.plugin_id.as_str();
        let mut address: Option<semio_framework::manifest::ActionAddress> = None;
        'apps: for app in self.descriptor.manifest.apps.iter() {
            let base_window = app.window_kinds.iter().next().map(|kind| kind.id.clone());
            for window_kind in app.window_kinds.iter() {
                for action in window_kind.actions.iter() {
                    if capability_id == format!("{plugin_id}.{}.{}", app.id, action.id) {
                        address = Some(semio_framework::manifest::ActionAddress {
                            plugin_id: plugin_id.to_string(),
                            app_id: app.id.clone(),
                            mode_id: app.default_mode_id.clone(),
                            window_kind_id: window_kind.id.clone(),
                            window_instance_id: window_kind.id.clone(),
                            action_id: action.id.clone(),
                        });
                        break 'apps;
                    }
                }
            }
            for action in app.actions.iter() {
                if capability_id == format!("{plugin_id}.{}.{}", app.id, action.id) {
                    let Some(window_instance_id) = base_window.clone() else {
                        return Err(Self::not_wired("PureCommand", format!("app `{}` declares action `{}` but no window kind at all, so there is no pane an agent could address it in", app.id, action.id)));
                    };
                    address = Some(semio_framework::manifest::ActionAddress {
                        plugin_id: plugin_id.to_string(),
                        app_id: app.id.clone(),
                        mode_id: app.default_mode_id.clone(),
                        window_kind_id: "*".to_string(),
                        window_instance_id,
                        action_id: action.id.clone(),
                    });
                    break 'apps;
                }
            }
        }
        let address = address.ok_or_else(|| Fault {
            code: "capability.not-found".to_string(),
            message: format!("plugin `{plugin_id}` declares no app action named by capability `{capability_id}` — its committed manifest publishes {} app(s)", self.descriptor.manifest.apps.len()),
        })?;
        let arguments = arguments().map_err(|detail| Self::not_wired("PureCommand", detail))?;
        Ok(semio_framework::manifest::ActionInvocation { expected_document_revision:None, address, arguments })
    }

    /// ⌨️ Resolves commands through their committed app, mode or plugin declaration.
    pub(crate) fn addressed_command_invocation(&self, capability_id: &str, arguments: std::collections::BTreeMap<String, semio_framework_value::DslValue>) -> Option<semio_framework::manifest::CommandInvocation> {
        Self::command_invocation_in_manifest(&self.plugin_id, &self.descriptor.manifest, capability_id, arguments)
    }

    /// 🧾️ The declaration, rather than capability punctuation, determines a command owner.
    pub(crate) fn command_invocation_in_manifest(plugin_id: &str, manifest: &semio_framework::manifest::PluginManifest, capability_id: &str, arguments: std::collections::BTreeMap<String, semio_framework_value::DslValue>) -> Option<semio_framework::manifest::CommandInvocation> {
        use semio_framework::manifest::CommandInvocation;
        for app in &manifest.apps {
            for command in &app.commands {
                if let Some(address) = Self::declared_command_address(plugin_id, Some(&app.id), None, &command.id, capability_id) {
                    return Some(CommandInvocation { address, arguments });
                }
            }
            for mode in &app.modes {
                for command in &mode.commands {
                    if let Some(address) = Self::declared_command_address(plugin_id, Some(&app.id), Some(&mode.id), &command.id, capability_id) {
                        return Some(CommandInvocation { address, arguments });
                    }
                }
            }
        }
        manifest.commands.iter().find_map(|command| Self::declared_command_address(plugin_id, None, None, &command.id, capability_id)).map(|address| CommandInvocation { address, arguments })
    }

    /// 🏷️ Projects one declaration into the catalog identity and structural command address.
    pub fn declared_command_address(plugin_id: &str, app_id: Option<&str>, mode_id: Option<&str>, command_id: &str, capability_id: &str) -> Option<semio_framework::manifest::CommandAddress> {
        use semio_framework::manifest::{CommandAddress, CommandOwnerAddress};
        let (id, owner) = match (app_id, mode_id) {
            (Some(app), Some(mode)) => (format!("{plugin_id}.{app}.mode.{mode}.{command_id}"), CommandOwnerAddress::Mode { plugin_id: plugin_id.into(), app_id: app.into(), mode_id: mode.into() }),
            (Some(app), None) => (format!("{plugin_id}.{app}.cmd.{command_id}"), CommandOwnerAddress::App { plugin_id: plugin_id.into(), app_id: app.into() }),
            (None, None) => (format!("{plugin_id}.cmd.{command_id}"), CommandOwnerAddress::Plugin { plugin_id: plugin_id.into() }),
            (None, Some(_)) => return None,
        };
        (id == capability_id).then(|| CommandAddress { owner, command_id: command_id.into() })
    }

    pub(crate) fn not_wired(what: &str, detail: impl std::fmt::Display) -> Fault {
        Fault { code: "channel.not-wired".to_string(), message: format!("{what}: {detail}") }
    }

    pub(super) fn budget_fault(what: &str) -> Fault {
        Fault { code: "budget.exceeded".to_string(), message: format!("{what} reached its wall budget before it settled") }
    }
}
//#endregion 🔖️ArtifactChannel
