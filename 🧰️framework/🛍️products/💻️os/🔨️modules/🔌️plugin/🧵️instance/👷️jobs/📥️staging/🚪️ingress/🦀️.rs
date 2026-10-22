//! 🔧️ Staged slice of the `ingress` band of the mounted artifact instance: its `instance-jobs` members (WP6 step S0 of the instance family).
//!
//! 🚪️ Ingress of the mounted artifact instance: framework-reserved job driving, constructors, store-constructor units, envelope ingress and decode, document archive genesis and merge, child members and packs.
use semio_framework_plugin_ownership::bounded_ownership::{ARTIFACT_LIVE_OUTPUT_SLOTS, plugin_retirement_fault};
use semio_framework::{ActionKind, Fault, FaultCode, FaultFrom, FaultOrigin};
use semio_framework_plugin_decl_emission::{ActionMeta, Emit};
use semio_framework_plugin_command::command_admission::{AdmittedToolCommand, TypedCommandWireWriter};
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use std::collections::{BTreeMap, BTreeSet};
use semio_framework_plugin_instance_children::children_components::ChildComposition;
use semio_framework_plugin_instance_history::history_components::{CommandHistory, HostEvents};
use semio_framework_value::{DslValue, RetirementDemand};
use semio_framework_plugin_job::reserved_jobs::{FrameworkReservedCommitPermit, QualifiedToolProof};
use std::future::Future;
use semio_framework_plugin_instance_stores::stores_components::InstanceStores;
use store::{MemberFactory, Mutation, ReferenceSpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_operation::ToolCancellationHandle;
use crate::jobs_components::ToolOperations;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_plugin_window_kits::validate_ui_dispatch_classification;
use semio_framework_ui_contract::*;

impl<A: ArtifactApp> ToolOperations<A> {
    #[cfg(test)]
    pub fn test_registered_tool_keys(&self) -> BTreeSet<(String, String)> {
        self.tool_jobs.keys().into_iter().map(|key| (key.controller_id, key.tool_id)).collect()
    }

    #[cfg(test)]
    pub fn test_framework_tool_registrations(&self) -> BTreeMap<String, (semio_framework::ToolFactoryKey, String, std::any::TypeId, &'static str, semio_framework::ToolExecutionContract)> {
        self.framework_tool_registrations
            .iter()
            .map(|(tool_id, registration)| (tool_id.clone(), (registration.key.clone(), registration.schema_id.clone(), registration.factory_type_id, registration.factory_type_name, registration.contract)))
            .collect()
    }

    pub fn tool_cancellation_handle(&self) -> &ToolCancellationHandle {
        &self.tool_cancellations
    }

    /// 🧷️ Clears only the original reserved publication identity without changing a foreign slot.
    pub fn release_reserved_emit_slot(&mut self, operation: u64) {
        let reservation = &mut self.typed_operation_reservations[operation as usize % ARTIFACT_LIVE_OUTPUT_SLOTS];
        if *reservation == Some(operation) { *reservation = None; }
    }

    /// 🏁️ Returns a reserved producer's original slot and keyed cancellation authority after ordinary completion.
    pub fn finish_reserved_emit_producer(&mut self, permit: FrameworkReservedCommitPermit) {
        self.release_reserved_emit_slot(permit.operation.operation.0);
        permit.finish();
    }

    pub fn qualified_tool_proof(&self, stores: &InstanceStores<A>, verb: &str) -> Result<QualifiedToolProof, Fault> {
        if let Some(lane) = stores.unsupported_publication_contracts.get(verb) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.publication-authority-missing"), format!("typed command '{verb}' declares the unsupported {lane} publication lane")));
        }
        if let Some(registration) = self.app_tool_registrations.get(verb) {
            return Ok(QualifiedToolProof::AppOwned(registration.clone()));
        }
        if let Some(registration) = self.framework_tool_registrations.get(verb) {
            return Ok(QualifiedToolProof::FrameworkOwned(registration.clone()));
        }
        if self.bounded_tool_proofs.contains_key(verb) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-owned-reducer"), format!("typed command '{verb}' has only a generic proof and no exact app-owned retained decoder/reducer factory")));
        }
        Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-factory"), format!("typed command '{verb}' has no exact controller/owner/factory/tool/schema proof")))
    }

    pub(crate) fn qualified_host_configuration_tool_proof(&self, stores: &InstanceStores<A>, verb: &str) -> Result<QualifiedToolProof, Fault> {
        if let Some(lane) = stores.unsupported_publication_contracts.get(verb) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.publication-authority-missing"), format!("host configuration command '{verb}' declares the unsupported {lane} publication lane")));
        }
        if let Some(registration) = self.app_tool_registrations.get(verb) {
            return Ok(QualifiedToolProof::AppOwned(registration.clone()));
        }
        if let Some(registration) = self.framework_tool_registrations.get(verb) {
            return Ok(QualifiedToolProof::FrameworkOwned(registration.clone()));
        }
        if let Some(proof) = self.bounded_tool_proofs.get(verb) {
            return Ok(QualifiedToolProof::Bounded(proof.clone()));
        }
        Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-factory"), format!("host configuration command '{verb}' has no exact controller/owner/factory/tool/schema proof")))
    }

    /// 🚦️ The UI-safety backstop, read from the app's own manifest declaration: a verb that is not
    /// `Migrated` is refused by NAME here, before any factory, proof or wire admission is resolved.
    /// Resolving the proof first answered `interactive-job.missing-factory` for a verb whose real
    /// refusal is `interactive-job.not-ui-safe` — a non-migrated verb has no factory precisely
    /// BECAUSE it is not migrated, so the proof lookup reported the consequence, not the cause.
    pub(crate) fn require_ui_safe_declaration(&self, verb: &str) -> Result<(), Fault> {
        let declaration = match self.registry.get(verb) {
            Some(definition) => definition.semantics.execution.interactive_job,
            None => self
                .registry
                .get_command(verb)
                .map(|definition| definition.semantics.execution.interactive_job)
                .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unknown-key"), format!("typed command '{verb}' has no exact manifest declaration")))?,
        };
        validate_ui_dispatch_classification("owner", verb, declaration)
    }

    pub(crate) async fn admit_command_wire_with_proof(&self, proof: QualifiedToolProof, verb: &str, payload: &[u8], decoded_items: usize) -> Result<AdmittedToolCommand, Fault> {
        self.require_ui_safe_declaration(verb)?;
        let key = proof.key();
        let (admission, input) =
            self.tool_jobs.begin_exact_wire(key.controller_id, key.tool_id, proof.schema_id(), payload.len()).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), error.to_string()))?;
        let mut writer = TypedCommandWireWriter { input, page: [0; semio_framework::action_bus::TOOL_WIRE_PAGE_BYTES], len: 0 };
        std::io::Write::write_all(&mut writer, payload).map_err(|error| plugin_sdk_fault(error.to_string()))?;
        let raw_wire = writer.finish()?;
        if decoded_items > proof.contract().max_decoded_items {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.decoded-limit"), format!("decoded command contains {decoded_items} items; maximum is {}", proof.contract().max_decoded_items)));
        }
        if !proof.admits::<A>(&admission) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.factory-identity"), format!("typed command '{verb}' resolved outside its exact controller/owner/factory/tool/schema proof")));
        }
        Ok(AdmittedToolCommand { verb: verb.to_string(), proof, wire_admission: admission, decoded_items, raw_wire })
    }

    pub async fn admit_command_wire(&self, stores: &InstanceStores<A>, verb: &str, payload: &[u8], decoded_items: usize) -> Result<AdmittedToolCommand, Fault> {
        self.require_ui_safe_declaration(verb)?;
        let proof = self.qualified_tool_proof(stores, verb)?;
        self.admit_command_wire_with_proof(proof, verb, payload, decoded_items).await
    }

    pub(crate) async fn admit_command_json_with_proof(&self, proof: QualifiedToolProof, verb: &str, args: Option<&DslValue>) -> Result<AdmittedToolCommand, Fault> {
        self.require_ui_safe_declaration(verb)?;
        let key = proof.key();
        let (admission, input) = self
            .tool_jobs
            .begin_exact_wire(key.controller_id, key.tool_id, proof.schema_id(), proof.contract().max_raw_wire_bytes)
            .map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), error.to_string()))?;
        let mut writer = TypedCommandWireWriter { input, page: [0; semio_framework::action_bus::TOOL_WIRE_PAGE_BYTES], len: 0 };
        let wire_args = args.cloned();
        let encoded = semio_framework_pack_json::to_json_string(&(verb, wire_args));
        std::io::Write::write_all(&mut writer, encoded.as_bytes()).map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.raw-wire-limit"), "typed command raw JSON exceeds its registered retained-page admission"))?;
        let raw_wire = writer.finish_admitted_prefix()?;
        if !proof.admits::<A>(&admission) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.factory-identity"), format!("typed command '{verb}' resolved outside its exact controller/owner/factory/tool/schema proof")));
        }
        Ok(AdmittedToolCommand { verb: verb.to_string(), proof, wire_admission: admission, decoded_items: 0, raw_wire })
    }

    pub async fn admit_host_configuration_json(&self, stores: &InstanceStores<A>, verb: &str, args: Option<&DslValue>) -> Result<AdmittedToolCommand, Fault> {
        self.require_ui_safe_declaration(verb)?;
        let proof = self.qualified_host_configuration_tool_proof(stores, verb)?;
        self.admit_command_json_with_proof(proof, verb, args).await
    }

    /// 🧾️ A settled typed operation that published no document lane and no shared-config lane still
    /// owes the history panel one row, on the admitting completion: `dispatch_emit`'s empty
    /// `artifact_mutations` branch files the verb under its declared kind (`View` for `select`,
    /// `Mutation` for a reducer that emitted nothing, `edit_id: None`). Recording that row only from
    /// retirement lands it after [`Self::take_typed_operation_completion`] has built `history_patch`,
    /// so the row surfaced on the next command. Both the completion take and
    /// [`Self::retire_typed_operation_unit`] call this; `command_logged` makes the second a no-op.
    /// Silent for a faulted operation, exactly as `dispatch_emit`'s `Err` path is, and for a `View`
    /// whose only durable emission is the per-window config lane — an orbit tick is not a command.
    pub fn record_settled_typed_operation_command(&mut self, history: &mut CommandHistory<A>, host_events: &mut HostEvents, operation_id: u64) {
        if host_events.host_event_operations.remove(&operation_id) {
            if let Some(operation) = self.tool_operations.get_mut(operation_id) {
                operation.command_logged = true;
            }
            return;
        }
        let logged = {
            let Some(operation) = self.tool_operations.get(operation_id) else { return };
            if operation.command_logged || operation.terminal_fault.is_some() || operation.published_artifact || operation.published_config {
                return;
            }
            let kind = self.typed_operation_command_kind(host_events, &operation.verb, false);
            if operation.published_window_config && matches!(kind, ActionKind::View) {
                return;
            }
            (operation.verb.clone(), kind)
        };
        let (verb, kind) = logged;
        if let Some(operation) = self.tool_operations.get_mut(operation_id) {
            operation.command_logged = true;
        }
        self.record_command(history, &verb, kind, None, None, Vec::new(), None);
    }

    /// 🧾️ The declared [`ActionKind`] one migrated verb logs under — the same three-step resolution
    /// [`Self::dispatch_emit`] runs, with the same last resort: an unresolved verb that reached a
    /// document store is a `Mutation`, one that reached nothing is a `View`.
    pub fn typed_operation_command_kind(&self, host_events: &HostEvents, verb: &str, artifact_lane: bool) -> ActionKind {
        match host_events.invocation_kind {
            Some(kind) => kind,
            None => match self.registry.get(verb).map(|definition| definition.kind) {
                Some(kind) => kind,
                None => match self.registry.get_command(verb) {
                    Some(definition) => definition.kind,
                    None if artifact_lane => ActionKind::Mutation,
                    None => ActionKind::View,
                },
            },
        }
    }

    /// 📏️ Quotes the next retiring typed-operation turn: its private child group's close, one owner hand-off, or the operation's own retirement.
    pub fn typed_operation_retirement_demands<M: ReferenceSpaceMember + MemberFactory + 'static>(&self, child_composition: &ChildComposition<M>, operation_id: u64, body: usize) -> Result<RetirementDemand, Fault> {
        let operation = self.tool_operations.get(operation_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.typed-operation-retirement-authority"), "typed operation owner changed before its retirement quote"))?;
        if operation.close_fault_pending(){return operation.close_fault_demands(body).map_err(plugin_retirement_fault);}
        if let Some(group) = operation.owned_child_group {
            if child_composition.private_child_groups.get(group).is_some() { return child_composition.private_child_group_operation_close_demands(group, body).map_err(plugin_retirement_fault); }
            return Ok(RetirementDemand { depth: 1, ..Default::default() });
        }
        if operation.lease_retirement_ready(){return operation.lease_retirement_demands(&self.tool_cancellations,body).map_err(plugin_retirement_fault);}
        operation.retirement_demands(body).map_err(plugin_retirement_fault)
    }
}
