//! 💡️ Inference route and actions of the plugin artifact channel: the inference route, real inference, the addressed action invocation and the typed faults. See [`super`].

use super::*;

//#region 🔖️ArtifactChannel
#[cfg(not(target_arch = "wasm32"))]
impl PluginArtifactChannel {
    /// 💡️ Opens (once) this plugin's real inference route: a fresh `GuestRuntimes` over the SAME
    /// compiled component, one `PluginInstanceHandle`, and an `ArtifactInferenceRouter` registered
    /// with the plugin's own declared roster — the same rows `🏃️run/🦀️.rs` builds
    /// (`contributions.inference_services` chained with every
    /// `artifact_contributions[].inferences`), written by the owned JSON codec. A plugin that declares no inference at all is a
    /// typed refusal here rather than an empty registration that would fail later with a worse
    /// message.
    ///
    /// 🐎️ The SAME runtime the command lane uses, not a second one: `self.compiled` carries a
    /// `wasmtime::Component` that only its own `Engine` can instantiate. The inference lane is
    /// still a SEPARATE guest instance on its own actor ordinal — `PluginInstanceHandle` takes
    /// ownership of the instance it drives — it just shares the engine and the compiled code.
    fn ensure_inference_route(&mut self) -> Result<&PluginInferenceRoute, Fault> {
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
            let caps: Vec<semio_framework::kernel::BrokerCapabilityGrant> =
                self.descriptor.capability_requests.iter().map(|request| semio_framework::kernel::BrokerCapabilityGrant { token: semio_framework::CapabilityToken(0), id: request.id.clone(), scope: request.scope.clone(), expires_ms: None }).collect();
            let runtimes = Arc::clone(&self.runtime);
            let budget = headless_inference_budget();
            let instance = semio_framework_async::block_on(semio_framework_plugin_host::GuestRuntime::instantiate(runtimes.as_ref(), &self.compiled, actor, &caps, &budget)).map_err(|error| Self::not_wired("inference instantiate", error))?;
            let handle = Arc::new(semio_framework_async::block_on(semio_framework_plugin_host::PluginInstanceHandle::new(actor, Arc::clone(&runtimes), instance,semio_framework_plugin_host::GuestRelayWakeAuthority{drive_policy:self.host_driver_policy.drive,policy:self.host_driver_policy.wake,issuer:semio_framework_job_worker::admit_original_thread_worker_wake,receiver:self.host_driver.clone()})));
            let router = semio_framework_plugin_host::ArtifactInferenceRouter::new();
            semio_framework_async::block_on(router.register_plugin(&self.plugin_id, &self.descriptor.manifest.dependencies, handle, &roster_bytes)).map_err(|error| Self::not_wired("inference route registration", error))?;
            self.inference = Some(PluginInferenceRoute { router });
        }
        Ok(self.inference.as_ref().expect("inference route was just opened"))
    }

    /// 💡️ Runs ONE declared inference through the real router — the general replacement for the old
    /// blanket `channel.not-wired`. The router resolves the route, toposorts and injects
    /// `dependsOn` results, drives the guest's `semio.infer` cold job, validates the guest's echo
    /// field-for-field and re-checks commit freshness; this function only builds the request wire
    /// and decodes the result, and reports whatever the router answers verbatim.
    ///
    /// 🗣️ The artifact schema's own canonical identity, exactly as the plugin committed it.
    /// It is an `ArtifactIdentity`, which the guest validates with `ArtifactIdentity::parse`
    /// (`🔌️plugin/🦀️.rs`, `validate_wire_request_resources`): dot-delimited segments of
    /// `[a-z0-9_-]` only. The `<kind>@<schemaVersion>/*` form this used to send is the
    /// CAPABILITY-id grammar, not an identity — `@`, `/` and `*` are all non-canonical
    /// segment bytes, so every real inference died `artifact-inference.source-dialect:
    /// identity "s.wfc.bitmap@1/*" has a non-canonical segment` before the guest ran a
    /// single step (measured 2026-09-22, slice CE3). The version is not lost by dropping the
    /// suffix: `artifact_schema_version` is its own field on this same request.
    pub(crate) fn infer_real(&mut self, command: &InferCommand) -> Result<ArtifactInferenceResultV1, Fault> {
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
            generation: command.turn.generation,
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
        let request_bytes = semio_framework_pack_json::to_json_string(&request).into_bytes();
        let route = self.ensure_inference_route()?;
        let result_bytes = semio_framework_async::block_on(route.router.infer(&request_bytes, &command.cancel.0)).map_err(|error| Fault { code: "app.command.rejected".to_string(), message: format!("`{}` refused: {error}", request.inference_schema) })?;
        let result_text = std::str::from_utf8(&result_bytes).map_err(|error| Self::not_wired("decoding the inference result", error))?;
        semio_framework_pack_json::from_json_str(result_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| Self::not_wired("decoding the inference result", error))
    }

    /// 🎯️ The owner-qualified address a typed command frame needs, resolved out of the plugin's OWN
    /// committed manifest rather than parsed out of the capability id's punctuation.
    ///
    /// 🗂️ The walk is `🗂️catalog`'s own `app_action_verbs` walk, in the same order and with the same
    /// precedence, so the address this builds always names the window kind the catalog published the
    /// verb under: window-kind declarations first, then APP-scope actions (`🗒️note` declares all 48
    /// of its verbs this way) under the catalog's own `"*"` marker for "belongs to no particular
    /// window". Recomputing the catalog's id (`{plugin}.{app}.{action}`) and comparing is what keeps
    /// the two in step — an app id is itself a dotted canonical surface ref
    /// (`s.draw.drawing@1/*#editor`), so splitting the id on `.` could never have recovered it.
    ///
    /// 🪟️ `window_instance_id` is the window kind's own id, which is exactly what the React shell's
    /// `sessionWindowInstances` mints for a freshly opened app (`app.windowKinds.map(kind => ({id:
    /// kind.id, …}))`) — an agent therefore addresses the very pane a human's first window is,
    /// rather than a synthetic id no shell would ever produce.
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
        Ok(semio_framework::manifest::ActionInvocation { address, arguments })
    }

    pub(crate) fn not_wired(what: &str, detail: impl std::fmt::Display) -> Fault {
        Fault { code: "channel.not-wired".to_string(), message: format!("{what}: {detail}") }
    }

    pub(super) fn budget_fault(what: &str) -> Fault {
        Fault { code: "budget.exceeded".to_string(), message: format!("{what} reached its wall budget before it settled") }
    }
}
//#endregion 🔖️ArtifactChannel
