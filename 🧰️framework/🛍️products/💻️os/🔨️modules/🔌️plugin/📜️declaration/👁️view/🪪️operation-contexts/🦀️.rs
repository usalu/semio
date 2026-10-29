//! 🪪️ The durable authority captured at command admission, the fixed render authority of a retained background job, the mounted snapshot read preparation and the publication revisions. See [`super`].

use super::*;

/// 🪪️ Durable authority captured once at public command admission and copied into every
/// plugin continuation record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppOperationContext {
    pub app_instance_id: u32,
    pub parent_document_id: String,
    pub operation_id: u64,
    pub generation: u64,
    pub canonical_base_revision: [u8; 32],
    pub retained:RetainedCloneGrant,
    /// 🪪️ Who admitted this command, and when: content-addressed over the admitting actor and the document
    /// store's hybrid logical clock ticked for the admission (`VcsArtifactApp::authoring_seed`). Every id a
    /// command mints is scoped by it, so two writers — or two sessions of one — never mint the same id at one base.
    pub authoring_seed: String,
}

impl semio_framework_value::retirement::RetireOwned for AppOperationContext {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{let Self{parent_document_id,authoring_seed,..}=self;semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(parent_document_id),semio_framework_value::retirement::deferred(authoring_seed)])}
    fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.parent_document_id),semio_framework_value::retirement::deferred_birth_bytes_for(&self.authoring_seed)])}
    fn controlled_retirement_supported()->bool{true}
}

/// 🎨️ Fixed authority for one live renderer-observed document revision. Unlike a command
/// operation, this carries no user invocation id: it binds an app-owned retained background job to
/// the exact instance, base revision and store generation observed by the renderer/refresh pass.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AppRenderOperationContext {
    pub app_instance_id: u32,
    pub base_revision: semio_framework_job::RevisionId,
    pub generation: semio_framework_job::Generation,
    pub canonical_base_revision: [u8; 32],
    pub mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1,
}

/// 🧾️ Original mounted snapshot preparation readiness and its actual retained effects.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MountedSnapshotReadPreparation {
    pub ready: bool,
    pub progress: RetainedCloneProgress,
}

impl MountedSnapshotReadPreparation {
    /// 📏️ Intersects mounted policy with the original caller's unspent physical and CPU authority.
    pub fn grant(policy: RetainedCloneGrant, cx: &semio_framework_job::StepContext<'_>) -> Option<RetainedCloneGrant> {
        if cx.is_cancelled() || cx.should_yield() { return None; }
        let mut grant = semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant(policy, cx.retained_grant());
        grant.maximum_items = grant.maximum_items.min(cx.fuel_remaining().try_into().unwrap_or(usize::MAX));
        Some(grant)
    }

    /// 🎟️ Records effects against the original caller before refusing excess authority.
    pub fn record(self, grant: RetainedCloneGrant, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, Fault> {
        let remaining = cx.retained_grant();
        let covered = grant.maximum_items <= remaining.maximum_items && grant.maximum_copy_bytes <= remaining.maximum_copy_bytes && grant.maximum_capacity_bytes <= remaining.maximum_capacity_bytes && grant.maximum_release_bytes <= remaining.maximum_release_bytes && grant.maximum_depth <= remaining.maximum_depth && self.progress.fits(grant) && (grant.maximum_depth != 0 || self.progress == RetainedCloneProgress::default()) && self.progress.copied_items as u64 <= cx.fuel_remaining();
        let receipt = cx.consume_retained(self.progress).map_err(|error| plugin_sdk_fault(error.into_message()));
        cx.consume_fuel(self.progress.copied_items as u64);
        receipt?;
        if !covered { return Err(plugin_sdk_fault("mounted snapshot preparation exceeds its original granted turn")); }
        Ok(self.ready)
    }
}

impl AppRenderOperationContext {
    /// 🔐️ Matches complete document authority independently of retained work grants.
    pub fn matches_revision(&self, app_instance_id: u32, base_revision: semio_framework_job::RevisionId, generation: semio_framework_job::Generation, canonical_base_revision: &[u8; 32]) -> bool {
        self.app_instance_id == app_instance_id && self.base_revision == base_revision && self.generation == generation && &self.canonical_base_revision == canonical_base_revision
    }
}

/// 🧾️ Projects the exact render-pinned store revision onto the decimal-u64 completion lane.
pub fn render_operation_publication_revision(operation: AppRenderOperationContext) -> UiPublicationRevision {
    UiPublicationRevision(u64::from_be_bytes(operation.canonical_base_revision[..8].try_into().expect("revision lane width")))
}

/// 🧾️ Reads the render-pinned store authority required by editable retained controls.
pub fn artifact_render_publication_revision<P>(doc: &ArtifactView<'_, P>) -> UiAssemblyResult<UiPublicationRevision> {
    doc.render_operation().map(render_operation_publication_revision).ok_or_else(|| PluginAssemblyError::new("framework.artifact-render.publication-revision", "editable rendering requires a render-pinned document revision"))
}

impl AppOperationContext {
    /// 👁️ Keeps the admitted read operation's exact identity and physical grant without granting entity authoring.
    pub fn for_read_only(app_instance_id:u32,parent_document_id:String,operation:semio_framework_job::Operation,canonical_base_revision:[u8;32],retained:RetainedCloneGrant)->Self{
        Self{app_instance_id,parent_document_id,operation_id:operation.operation.0,generation:operation.generation.0,canonical_base_revision,retained,authoring_seed:String::new()}
    }

    /// 🌱️ Mints a replay-stable entity identity from the document and actor-clock command admission.
    pub fn entity_id(&self, prefix: &str, local_key: &str) -> Result<String, Fault> {
        if self.authoring_seed.is_empty() {
            return Err(plugin_sdk_fault("entity creation requires an admitted authoring seed"));
        }
        let bytes = format!("{}:{}{}:{}{}:{}", self.parent_document_id.len(), self.parent_document_id, self.authoring_seed.len(), self.authoring_seed, local_key.len(), local_key);
        Ok(store::content_id(prefix, bytes.as_bytes()))
    }

    /// 🧬️ Fixed-width lowercase encoding for typed persisted continuation fields.
    pub fn canonical_base_revision_hex(&self) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut result = String::with_capacity(64);
        for byte in self.canonical_base_revision {
            result.push(HEX[usize::from(byte >> 4)] as char);
            result.push(HEX[usize::from(byte & 0x0f)] as char);
        }
        result
    }
}
