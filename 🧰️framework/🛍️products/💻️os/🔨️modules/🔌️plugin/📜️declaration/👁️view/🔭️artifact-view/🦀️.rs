//! 🔭️ The read-only view of an app document, the pending view of a poll and the materialized snapshot, history and undo reads of the view. See [`super`].

use super::*;
use semio_framework_plugin_operation::ArtifactOperationProgress;

/// 🧾️ Read-only view of an app's document handed to `ArtifactApp::handle_action`/`render`:
/// the materialized snapshot plus the history metadata (checkpoints/alternatives/undo state)
/// derived from the owning {@link VcsArtifactApp}'s persistent {@link ArtifactStore}.
pub struct ArtifactView<'a, P> {
    pub snapshot: &'a P,
    pub history: &'a HistoryView,
    /// 🧸️ Read access to this document's OWNED CHILDREN, each of which is its own envelope with
    /// its own `ArtifactVcs` history (never an inline subtree). See {@link ChildContentView}.
    pub children: ChildContentView,
    pub(super) operation: Option<AppOperationContext>,
    pub(super) render_operation: Option<AppRenderOperationContext>,
    pub(super) parent_document_id: Option<&'a str>,
    snapshot_read: std::sync::Mutex<Option<store::os_store::snapshot_read::SnapshotRead<P>>>,
    tool_run: Option<ToolRunView>,
    original_tool_run:Option<semio_framework_tool_run_presentation::ToolRunView<'a>>,
    operations: &'a [ArtifactOperationProgress],
    provisional_generation: u64,
    instance: Option<ArtifactInstanceOperationOwnerHandle>,
}

/// 🧵️ A poll borrows original committed inputs without materializing a renderer history projection.
 pub struct ArtifactPendingView<'a,P>{pub snapshot:&'a P,pub children:&'a ChildContentView,pub(super) render_operation:AppRenderOperationContext,pub(super) parent_document_id:&'a str,snapshot_read:std::sync::Mutex<Option<store::os_store::snapshot_read::SnapshotRead<P>>>,tool_run:Option<crate::ToolRunPollView<'a>>,provisional_generation:u64,instance:&'a ArtifactInstanceOperationOwnerHandle}

 impl<'a,P> ArtifactPendingView<'a,P>{
 /// 🌱️ Binds the original store, admitted read and live run for one mounted poll.
 pub fn new(snapshot:&'a P,children:&'a ChildContentView,render_operation:AppRenderOperationContext,parent_document_id:&'a str,snapshot_read:Option<store::os_store::snapshot_read::SnapshotRead<P>>,tool_run:Option<crate::ToolRunPollView<'a>>,provisional_generation:u64,instance:&'a ArtifactInstanceOperationOwnerHandle)->Self{Self{snapshot,children,render_operation,parent_document_id,snapshot_read:std::sync::Mutex::new(snapshot_read),tool_run,provisional_generation,instance}}
 pub fn render_operation(&self)->AppRenderOperationContext{self.render_operation}
 pub fn parent_document_id(&self)->&str{self.parent_document_id}
 pub fn instance(&self)->&ArtifactInstanceOperationOwnerHandle{self.instance}
 pub fn provisional_generation(&self)->u64{self.provisional_generation}
 pub fn tool_run(&self)->Option<crate::ToolRunPollView<'a>>{self.tool_run}
 pub fn source_version(&self)->semio_framework_value_registry::fixed_request_owners::RequestSourceVersion{semio_framework_value_registry::fixed_request_owners::RequestSourceVersion{base_revision:self.render_operation.base_revision.0,generation:self.render_operation.generation.0,canonical_revision:self.render_operation.canonical_base_revision}}
 pub fn take_snapshot_read(&self)->Result<store::os_store::snapshot_read::SnapshotRead<P>,Fault>{let mut owner=self.snapshot_read.try_lock().map_err(|_|plugin_sdk_fault("original poll snapshot-read lease is busy"))?;owner.take().ok_or_else(||plugin_sdk_fault("original poll has no admitted snapshot-read lease"))}
 }

impl<'a, P> ArtifactView<'a, P> {
    /// 🏗️ A view over a document with no composed children — the overwhelming majority, and the
    /// shape every leaf app and test uses. Prefer this over a struct literal: lane views grow
    /// over time, and a constructor absorbs that growth without touching every call site.
    pub fn new(snapshot: &'a P, history: &'a HistoryView) -> Self {
        Self { snapshot, history, children: ChildContentView::EMPTY, operation: None, render_operation: None, parent_document_id: None, snapshot_read: std::sync::Mutex::new(None), tool_run: None, original_tool_run:None, operations: &[], provisional_generation: 0, instance: None }
    }

    /// 🪪️ Synchronous retained-worker view bound to one exact admitted operation.
    pub fn with_operation(snapshot: &'a P, history: &'a HistoryView, operation: AppOperationContext) -> Self {
        Self {
            snapshot,
            history,
            children: ChildContentView::EMPTY,
            operation: Some(operation),
            render_operation: None,
            parent_document_id: None,
            snapshot_read: std::sync::Mutex::new(None),
            tool_run: None, original_tool_run:None,
            operations: &[],
            provisional_generation: 0,
            instance: None,
        }
    }

    /// 🧩️ App-side composing view for retained command work, which observes its children through
    /// [`ArtifactOwnedToolJobContext::children`] rather than through a host render pass and so
    /// carries no render identity. Synchronous on purpose: it only moves owned fields.
    pub fn with_children(snapshot: &'a P, history: &'a HistoryView, children: ChildContentView) -> Self {
        Self { snapshot, history, children, operation: None, render_operation: None, parent_document_id: None, snapshot_read: std::sync::Mutex::new(None), tool_run: None, original_tool_run:None, operations: &[], provisional_generation: 0, instance: None }
    }

    /// 🪪️ Binds a composing retained view to the exact admitted operation it runs under, so its handlers mint ids
    /// and tool transactions from the admission's authoring seed exactly like a leaf view bound by `with_operation`.
    pub fn bound_to_operation(mut self, operation: AppOperationContext) -> Self {
        self.operation = Some(operation);
        self
    }

    /// 🏗️ A view over a composing document, wired to its live child stores AND to the identity the
    /// live revision is observed under. Every host entry point that hands an app a composing view —
    /// `render`, `pending_effects`, `window_engagements`, `window_measures`, `tool_measures`,
    /// `context_menu`, `interaction_topology`, `export_media`, `media_fingerprint` — goes through
    /// here, so [`ArtifactView::render_operation`] is total on the host and an app that leases a
    /// per-instance session off it (puzzle 3d's `puzzle3d_view_session_key`) never runs cold.
    pub async fn with_render_context(snapshot: &'a P, history: &'a HistoryView, children: ChildContentView, render_operation: AppRenderOperationContext, parent_document_id: &'a str, snapshot_read: Option<store::os_store::snapshot_read::SnapshotRead<P>>) -> Self {
        Self {
            snapshot,
            history,
            children,
            operation: None,
            render_operation: Some(render_operation),
            parent_document_id: Some(parent_document_id),
            snapshot_read: std::sync::Mutex::new(snapshot_read),
            tool_run: None, original_tool_run:None,
            operations: &[],
            provisional_generation: 0,
            instance: None,
        }
    }

    /// 🔮️ Binds the mounted instance this view belongs to, whose handle owns the instance's derived-state sessions.
    pub fn with_instance(mut self, instance: ArtifactInstanceOperationOwnerHandle) -> Self {
        self.instance = Some(instance);
        self
    }

    /// 🔮️ The mounted instance this view belongs to, `None` for a view no instance hosts (a fixture, a one-shot read).
    pub fn instance(&self) -> Option<&ArtifactInstanceOperationOwnerHandle> {
        self.instance.as_ref()
    }

    /// ⏯️ Binds the tool run this document instance shows; `snapshot` is then committed ⊕ provisional.
    pub fn with_tool_run(mut self, tool_run: Option<ToolRunView>) -> Self {
        self.tool_run = tool_run;
        self
    }

    /// ⏯️ The run whose provisional entities render with the `provisional` style token and stay
    /// out of interaction topology (contract §4.1 layer 1).
    pub fn tool_run(&self) -> Option<&ToolRunView> {
        self.tool_run.as_ref()
    }

    /// 🔢️ Binds the instance's continuous-tool overlay generation, which moves on every tick, release and zero-trace
    /// abort of an open press or typing run.
    pub fn with_provisional_generation(mut self, generation: u64) -> Self {
        self.provisional_generation = generation;
        self
    }

    /// 🔢️ The continuous-tool overlay generation as of this view: a derived view (a preview) whose input follows the
    /// open transactions (`ArtifactOwnedToolJobContext::provisional`) owes a fresh evaluation whenever it moves, even
    /// when no edit landed (a zero-trace abort).
    pub fn provisional_generation(&self) -> u64 {
        self.provisional_generation
    }

    /// ⏳️ Current local command work; observation never enters artifact history.
    pub fn operations(&self) -> &[ArtifactOperationProgress] {
        self.operations
    }

    pub fn with_operations(mut self, operations: &'a [ArtifactOperationProgress]) -> Self {
        self.operations = operations;
        self
    }

    /// 🎯️ Borrows a real typed child under this same original parent read and canonical operation revision.
    pub fn child_editor_read<S:pack::snapshot::ArtifactPack+store::os_store::serialization::ArtifactStoreSnapshot+Send+Sync+'static>(&self,key:crate::child_view::MemberKeyRef<'_>)->Result<crate::child_view::editor::read::ChildEditorRead<'_,S>,Fault>{let(parent,revision)=if let Some(operation)=self.operation.as_ref(){if self.parent_document_id.is_some_and(|parent|parent!=operation.parent_document_id){return Err(plugin_sdk_fault("child editor parent identity differs from original command"));}(operation.parent_document_id.as_str(),&operation.canonical_base_revision)}else if let Some(operation)=self.render_operation.as_ref(){(self.parent_document_id.ok_or_else(||plugin_sdk_fault("child editor requires its real parent artifact identity"))?,&operation.canonical_base_revision)}else{return Err(plugin_sdk_fault("child editor requires an original parent command or render operation"));};self.children.editor_read_at(key,self.snapshot as*const P as usize,parent,revision)}

    /// 🪪️ Returns the actual public job authority for command handlers.
    pub fn operation(&self) -> Result<&AppOperationContext, Fault> {
        self.operation.as_ref().ok_or_else(|| plugin_sdk_fault("the artifact view is not bound to a public command operation"))
    }

    /// 🎟️ Returns the original explicitly admitted command or render maintenance authority.
    pub fn retained_grant(&self)->Result<RetainedCloneGrant,Fault>{
        if let Some(operation)=&self.operation{return Ok(operation.retained);}
        if let Some(operation)=self.render_operation{return Ok(operation.mounted_policy.maintenance);}
        Err(plugin_sdk_fault("artifact view has no original admitted retained authority"))
    }

    /// 🪪️ Optional form for render/test-only views which do not enter command authority.
    pub fn operation_optional(&self) -> Option<&AppOperationContext> {
        self.operation.as_ref()
    }

    /// 🎨️ Returns the immutable identity observed by a live render/refresh pass.
    /// 🪞️ Lends the actual render operation metadata to bounded source readers without copying its context.
    pub fn render_operation_ref(&self)->Option<&AppRenderOperationContext>{self.render_operation.as_ref()}

    pub fn render_operation(&self) -> Option<AppRenderOperationContext> {
        self.render_operation
    }

    /// 🧾️ Returns the exact live store envelope id bound to this host render pass.
    pub fn parent_document_id(&self) -> Option<&str> {
        self.parent_document_id.as_deref()
    }

    /// 🧵️ Transfers the one registered immutable snapshot-read lease to a retained worker
    /// owner. Contention and a second take fail closed without cloning the underlying snapshot.
    pub fn take_snapshot_read(&self) -> Result<store::os_store::snapshot_read::SnapshotRead<P>, Fault> {
        let mut owner = self.snapshot_read.try_lock().map_err(|_| plugin_sdk_fault("the live snapshot-read lease is busy"))?;
        owner.take().ok_or_else(|| plugin_sdk_fault("the artifact view has no transferable live snapshot-read lease"))
    }
}

impl<'a,P> ArtifactView<'a,P>{
 /// 🪟️ Borrows the original immutable presentation pages for this mounted render without materializing metadata or result bytes.
 pub fn with_original_tool_run(mut self,view:Option<semio_framework_tool_run_presentation::ToolRunView<'a>>)->Self{self.original_tool_run=view;self}
 /// 🔎️ Reads the same retained presentation that the native tick receiver published.
 pub fn original_tool_run(&self)->Option<semio_framework_tool_run_presentation::ToolRunView<'_>>{self.original_tool_run}
}
