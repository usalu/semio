//! 🏪️ Workflow store of the plugin host: the workflow store over the backbone with its authoring, undo, redo and catalog wire. See [`super`].

use super::*;

// #region host
//#region 🔖️OsWorkflowStore
pub struct OsWorkflowStore {
    pub(super) inner: ArtifactStore<workflow::WorkflowSnapshot, workflow::WorkflowMutation>,
    pub(super) name: String,
}

impl OsWorkflowStore {
    pub fn new(document: OsWorkflowArtifactDocument, actor: protocol::ActorId) -> Result<Self, VcsError> {
        let (name, envelope) = take_backbone_envelope(document);
        let mut inner = resolve_kernel_future(ArtifactStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, actor))?;
        inner.install_document_store_owners_exact(store::os_store::document_store_owners::funded_bounded_artifact_store_owners().map_err(|error| VcsError::ValidationFailed(error.to_string()))?).map_err(|(error, owners)| { std::mem::forget(owners); VcsError::ValidationFailed(error.to_string()) })?;
        Ok(Self { inner, name })
    }

    pub fn generation(&self) -> u64 {
        self.inner.generation()
    }

    pub fn snapshot(&self) -> Result<workflow::WorkflowSnapshot, VcsError> {
        self.inner.snapshot()
    }

    /// 🤝️ Fresh replay plus `reconcile_workflow_snapshot`'s whole 4(+1)-rule pipeline —
    /// invoked explicitly here rather than through `Mutation::reconcile` (a no-op default at the
    /// kernel-crate layer, since two of those rules need the os-core plugin/artifact registry).
    pub fn snapshot_with_conflicts(&self) -> Result<(workflow::WorkflowSnapshot, Vec<protocol::MutationMessage>), VcsError> {
        let document = self.inner.snapshot()?;
        Ok(reconcile_workflow_snapshot(document))
    }

    pub fn document(&self) -> OsWorkflowArtifactDocument {
        let envelope = self.inner.envelope();
        BackboneDocument {
            schema: envelope.schema.clone(),
            id: envelope.id.clone(),
            name: self.name.clone(),
            vcs: envelope.vcs.clone(),
            transitions: envelope.transitions.clone(),
            edit_messages: envelope.edit_messages.iter().cloned().collect(),
            conflicts: envelope.conflicts.clone(),
            backbone: envelope.backbone.clone(),
        }
    }

    pub fn dispatch_text(&mut self, command_text: &str, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        resolve_kernel_future(self.inner.dispatch_text(command_text, identity)).map(|_| ())
    }

    pub fn dispatch_binary(&mut self, command_bytes: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        resolve_kernel_future(self.inner.dispatch_binary(command_bytes, identity)).map(|_| ())
    }

    pub fn dispatch_apply(&mut self, mutations: Vec<workflow::WorkflowMutation>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        resolve_kernel_future(self.inner.dispatch(ArtifactCommand::Apply { mutations, transaction: None }, identity)).map(|_| ())
    }

    pub fn set_workflow_name(&mut self, name: &str) {
        self.name = name.into();
        let _ = self.inner.generation();
    }

    /// 🆔️ Mints a fresh `WorkflowNode` (id, ports, document/config refs — everything) via
    /// `workflow::workflow_node_for_app`, at dispatch time, so replay never re-derives it. Also
    /// dispatches `space::SpaceMutation::InstallProgram` against `space_store` — the owning space's
    /// `programs` list moved off the dissolved `OsSnapshot` onto `space::SpaceSnapshot` (see
    /// `## The inversion` in the plan), so spawning a node into the workflow graph and installing its
    /// plugin into the space are now two operations against two separate documents.
    pub fn add_workflow_node(&mut self, plugin_id: &str, app_id: &str, label: Option<&str>, x: f64, y: f64, space_store: &mut OsSpaceStore, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, VcsError> {
        let app = resolve_os_app_definition(plugin_id, app_id).ok_or_else(|| VcsError::Deserialize(format!("unknown app {plugin_id}/{app_id}")))?;
        let node_id = create_os_id("node");
        let position = workflow::WorkflowPosition { x, y, width: 0.0, height: 0.0 };
        let mut node = resolve_kernel_future(workflow::workflow_node_for_app(&app, plugin_id, &node_id, &position));
        if let Some(label) = label {
            node.label = label.into();
        }
        self.dispatch_apply(vec![workflow::WorkflowMutation::AddNode(workflow::AddNode { node })], identity)?;
        resolve_kernel_future(space_store.dispatch(ArtifactCommand::Apply { mutations: vec![space::SpaceMutation::InstallProgram { plugin_id: plugin_id.into(), index: None }], transaction: None }, identity))?;
        Ok(node_id)
    }

    pub fn add_parameter(&mut self, parameter_type: &workflow::WorkflowParameterType, name: &str, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, VcsError> {
        let parameter = resolve_kernel_future(workflow::create_default_workflow_parameter(parameter_type, name, None));
        let parameter_id_value = workflow::workflow_parameter_id(&parameter).to_string();
        self.dispatch_apply(vec![workflow::WorkflowMutation::AddParameter(workflow::AddParameter { parameter: Box::new(parameter) })], identity)?;
        Ok(parameter_id_value)
    }

    pub fn patch_parameter(&mut self, target_parameter_id: &str, patch: &workflow::WorkflowParameterPatch, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let document = self.snapshot()?;
        let current = document.parameters.iter().find(|parameter| workflow::workflow_parameter_id(parameter) == target_parameter_id).cloned().ok_or_else(|| VcsError::Deserialize(format!("unknown parameter {target_parameter_id}")))?;
        let next = resolve_kernel_future(workflow::patch_workflow_parameter(&current, patch));
        self.dispatch_apply(vec![workflow::WorkflowMutation::ChangeParameter(workflow::ChangeParameter { parameter_id: target_parameter_id.into(), parameter: Box::new(next) })], identity)
    }

    /// 📡️ Pumps any queued inbound backbone messages into the edit timeline.
    pub fn tick(&mut self) -> Result<bool, VcsError> {
        resolve_kernel_future(self.inner.tick())
    }

    /// 🔗️ Resolves and attaches a backbone by uri. Only available inside the wasm sandbox
    /// (every scheme forwards to the host over the injected `BackboneChannelPort`, a pure queue) —
    /// see {@link attach_backbone} for the native counterpart, which takes an explicit
    /// `Box<dyn store::os_store::backbone::Backbone>` since native has no URI→IO auto-resolution anymore (`framework/sync`'s
    /// `ArtifactHost` module owns constructing the real endpoint via `ArtifactHost`).
    #[cfg(target_arch = "wasm32")]
    pub fn attach_backbone(&mut self, uri: &str) -> Result<(), VcsError> {
        resolve_kernel_future(self.inner.attach_backbone_uri(uri))
    }

    /// 🔗️ Attaches an explicit native backbone channel (typically a `channel_backbone` handed
    /// out by `framework/sync`'s `ArtifactHost::open`, per `ArtifactHost`'s canonical sequence).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn attach_backbone(&mut self, backbone: store::os_store::backbone::Backbones) -> Result<(), VcsError> {
        resolve_kernel_future(self.inner.attach_backbone(backbone))
    }

    pub fn detach_backbone(&mut self) -> Result<Option<store::os_store::backbone::Backbones>, VcsError> {
        self.inner.detach_backbone()
    }

    pub fn backbone_ref(&self) -> Option<&ArtifactBackboneRef> {
        self.inner.backbone_ref()
    }
}
//#endregion 🔖️OsWorkflowStore
// #endregion host
