//! 🎯️ Completion and owned context of the tool job contract: the tool completion operations, the owned job snapshots and context, the context identity digest and the context operations. See [`super`].

use super::*;

impl<A: ArtifactApp> ArtifactToolCompletion<A> {
    /// 🪪️ Runs app-owned physical IO with the mounted operation's original lease and receipt.
    pub fn with_authoring_identity<T>(&self, operation: impl FnOnce(&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<T, Fault>) -> Result<T, Fault> {
        let mut owner = self.inner.try_lock_authoring().map_err(|_| plugin_sdk_fault("original app-owned effect authority is busy or poisoned"))?;
        owner.as_mut().ok_or_else(|| plugin_sdk_fault("app-owned creation requires its original mounted effect authority"))?.run(operation)
    }

    pub fn install_authoring_identity(&self, receipt: semio_framework_value::native_encoding::NativeEncodeContinuation, lease: &ToolCancellationLease) -> Result<(), Fault> {
        let mut owner = self.inner.try_lock_authoring().map_err(|_| plugin_sdk_fault("original app-owned effect authority is busy or poisoned"))?;
        if owner.is_some() { return Err(plugin_sdk_fault("original app-owned effect authority was already installed")); }
        *owner = Some(OriginalAuthoringEffect { receipt: Some(receipt), token: lease.cancel_token().clone(), publication: lease.publication_claim().clone(), progress: semio_framework_value::native_encoding::NativeEncodeProgress { completed: 0, total: 0, owned_bytes: 0 } });
        Ok(())
    }

    pub fn take_authoring_identity(&self) -> Result<Option<(semio_framework_value::native_encoding::NativeEncodeContinuation, semio_framework_value::native_encoding::NativeEncodeProgress)>, Fault> {
        let mut owner = self.inner.try_lock_authoring().map_err(|_| plugin_sdk_fault("original app-owned effect authority is busy or poisoned"))?;
        let Some(mut original) = owner.take() else { return Ok(None) };
        let receipt = original.receipt.take().ok_or_else(|| plugin_sdk_fault("original app-owned effect receipt is absent"))?;
        Ok(Some((receipt, original.progress)))
    }

    pub(crate) fn retirement_cursor(self)->crate::completion_retirement::CompletionRetirement<A>{crate::completion_retirement::CompletionRetirement::new(self.inner)}
    /// 🔎️ Compares genuine completion identity without exposing an Arc or Weak capability.
    fn same_original_cell(&self,other:&Self)->bool{self.inner.same_cell(&other.inner)}
    /// 🧪️ Creates a completion cell for an external artifact's registered factory laws.
    #[cfg(feature = "artifact-app-testing")]
    pub fn test_new() -> Self {
        Self::new()
    }

    /// 🧪️ Observes a factory result without granting production code completion consumption.
    #[cfg(feature = "artifact-app-testing")]
    pub fn test_take_emit(&self) -> Result<Option<ArtifactToolEmission<A>>, Fault> {
        self.take_emit()
    }

    pub fn new() -> Self {
        Self { inner: crate::completion_retirement::CompletionCell::new() }
    }

    pub fn has_mounted_consumer(&self) -> bool {
        self.inner.strong_count() > 1
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the original fixed-capacity input so its caller can retry or retire that exact owner without an extra allocation.")]
    pub fn complete(&self, emit: ArtifactMutationOutcome<A::Mutation, A::ConfigMutation, A::DraftMutation>, ephemeral: EphemeralEmit<A>) -> Result<(), ArtifactToolCompletionRejection<A>> {
        let mut value = match self.inner.try_lock() {
            Ok(value) => value,
            Err(_) => {
                return Err(ArtifactToolCompletionRejection { emit, ephemeral, fault: Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.media-completion-busy"), "media completion authority is busy or poisoned") });
            }
        };
        if value.is_some() {
            return Err(ArtifactToolCompletionRejection { emit, ephemeral, fault: Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.duplicate-output"), "app-owned tool job attempted to complete more than once") });
        }
        *value = Some(ArtifactToolCompletionValue::Emit(emit.map_err(ArtifactCompletionFault::new), ephemeral));
        Ok(())
    }

    #[doc(hidden)]
    pub fn with_busy_test_lock<R>(&self, test: impl FnOnce() -> R) -> R {
        let _guard = self.inner.lock().expect("completion test lock");
        test()
    }

    pub fn complete_download(&self,download:Result<ArtifactDownloadOutput,Fault>,ephemeral:EphemeralEmit<A>)->Result<(),ArtifactDownloadCompletionRejection<A>>{
        let mut value=match self.inner.try_lock(){Ok(value)=>value,Err(_)=>return Err(ArtifactDownloadCompletionRejection{download,ephemeral,fault:Fault::new(FaultOrigin::Framework,FaultCode::new("interactive-job.tool-completion-busy"),"tool completion authority is busy or poisoned")})};
        if value.is_some(){return Err(ArtifactDownloadCompletionRejection{download,ephemeral,fault:Fault::new(FaultOrigin::Framework,FaultCode::new("interactive-job.duplicate-output"),"app-owned tool job attempted to complete more than once")})}
        *value=Some(ArtifactToolCompletionValue::Download(download.map_err(ArtifactCompletionFault::new),ephemeral));Ok(())
    }

    #[doc(hidden)]
    pub fn test_take_download(&self)->Result<Option<(Result<ArtifactDownloadOutput,Fault>,EphemeralEmit<A>)>,Fault>{match self.take()?{Some(ArtifactToolCompletionValue::Download(output,ephemeral))=>Ok(Some((output.map_err(ArtifactCompletionFault::into_fault),ephemeral))),None=>Ok(None),_=>Err(Fault::from("expected original download completion"))}}

    pub fn take(&self) -> Result<Option<ArtifactToolCompletionValue<A>>, Fault> {
        self.inner.try_lock().map(|mut value| value.take()).map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.tool-completion-busy"), "tool completion authority is busy or poisoned"))
    }

    pub fn take_emit(&self) -> Result<Option<ArtifactToolEmission<A>>, Fault> {
        match self.take()? {
            Some(ArtifactToolCompletionValue::Emit(emit, ephemeral)) => Ok(Some((emit.map_err(ArtifactCompletionFault::into_fault), ephemeral))),
            Some(ArtifactToolCompletionValue::Download(_, _)) => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unexpected-download-output"), "reserved clipboard/import route completed with a segmented download")),
            None => Ok(None),
        }
    }
}

/// 📸️ State snapshots retained together by one app-owned tool job.
pub struct ArtifactOwnedToolJobSnapshots<A: ArtifactApp> {
    pub children: std::sync::Arc<ChildContentView>,
    pub draft: std::sync::Arc<A::Draft>,
    pub transient: std::sync::Arc<A::Transient>,
    pub window_config: Option<WindowConfigSnapshot>,
    pub window_transient: Option<WindowTransientSnapshot>,
}

/// 🧬 Immutable app-owned state captured by the scheduler for one retained job lifetime.
pub struct ArtifactOwnedToolJobContext<A: ArtifactApp> {
    presence_local: Option<store::os_store::snapshot_read::SnapshotRead<A::Presence>>,
    presence_peers: Option<std::sync::Arc<store::os_store::presence::PresencePeersRoot<A::Presence>>>,
    presence_peer_factory: Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<A::Presence>>>,
    pub app_instance_id: u32,
    pub view_state: Option<ViewModel>,
    pub canonical_base_revision: [u8; 32],
    pub draft_generation: u64,
    pub transient_generation: u64,
    pub children: std::sync::Arc<ChildContentView>,
    pub draft: ArtifactContextRoot<A::Draft>,
    pub transient: ArtifactContextRoot<A::Transient>,
    pub window_config: Option<WindowConfigSnapshot>,
    pub window_transient: Option<WindowTransientSnapshot>,
    source_capture_started: bool,
    draft_source_fault: Option<String>,
    transient_source_fault: Option<String>,
    refused_source_reads: [Option<store::os_store::snapshot_read::ErasedSnapshotRead>; 2],
    identity_digest: u64,
    tool_run: Option<ToolRunView>,
    original_tool_run:Option<semio_framework_tool_run_presentation::ToolRunCapturedView>,
    provisional: Vec<DslValue>,
    provisional_generation: u64,
    gesture: GestureCapture<A::Mutation>,
}

pub fn artifact_owned_tool_job_context_identity_digest(
    app_instance_id: u32,
    view_state: Option<&ViewModel>,
    canonical_base_revision: [u8; 32],
    draft_generation: u64,
    transient_generation: u64,
    children_digest: u64,
    windows: (Option<&WindowConfigSnapshot>, Option<&WindowTransientSnapshot>),
) -> u64 {
    let (window_config, window_transient) = windows;
    fn extend(mut digest: u64, bytes: impl IntoIterator<Item = u8>) -> u64 {
        for byte in bytes {
            digest = (digest ^ u64::from(byte)).wrapping_mul(0x1000_0000_01b3);
        }
        digest
    }

    let mut digest = extend(0xcbf2_9ce4_8422_2325, b"ARC-CONTEXT-4".iter().copied());
    digest = extend(digest, app_instance_id.to_le_bytes());
    digest = extend(digest, view_state.map(semio_framework_pack_json::to_json_string).unwrap_or_default().bytes());
    digest = extend(digest, canonical_base_revision);
    digest = extend(digest, draft_generation.to_le_bytes());
    digest = extend(digest, transient_generation.to_le_bytes());
    digest = extend(digest, children_digest.to_le_bytes());
    digest = extend(digest, [u8::from(window_config.is_some())]);
    if let Some(window) = window_config {
        for value in [window.window_id(), window.window_kind_id()] {
            digest = extend(digest, (value.len() as u64).to_le_bytes());
            digest = extend(digest, value.bytes());
        }
        digest = extend(digest, window.generation().to_le_bytes());
        digest = extend(digest, window.revision());
    }
    digest = extend(digest, [u8::from(window_transient.is_some())]);
    if let Some(window) = window_transient {
        for value in [window.window_id(), window.window_kind_id()] {
            digest = extend(digest, (value.len() as u64).to_le_bytes());
            digest = extend(digest, value.bytes());
        }
        digest = extend(digest, window.generation().to_le_bytes());
        digest = extend(digest, window.document_generation().to_le_bytes());
    }
    digest
}

impl<A: ArtifactApp> ArtifactOwnedToolJobContext<A> {
    pub fn new(app_instance_id: u32, view_state: Option<ViewModel>, canonical_base_revision: [u8; 32], draft_generation: u64, transient_generation: u64, snapshots: ArtifactOwnedToolJobSnapshots<A>) -> Self {
        let ArtifactOwnedToolJobSnapshots { children, draft, transient, window_config, window_transient } = snapshots;
        let identity_digest =
            artifact_owned_tool_job_context_identity_digest(app_instance_id, view_state.as_ref(), canonical_base_revision, draft_generation, transient_generation, children.identity_digest(), (window_config.as_ref(), window_transient.as_ref()));
        Self {
            app_instance_id,
            view_state,
            canonical_base_revision,
            draft_generation,
            transient_generation,
            children,
            draft: ArtifactContextRoot::new(draft),
            transient: ArtifactContextRoot::new(transient),
            window_config,
            window_transient,
            source_capture_started: false,
            draft_source_fault: None,
            transient_source_fault: None,
            refused_source_reads: [None, None],
            identity_digest,
            tool_run: None,original_tool_run:None,
            presence_local: None,
            presence_peers: None,
            presence_peer_factory: None,
            provisional: Vec::new(),
            provisional_generation: 0,
            gesture: GestureCapture::detached(),
        }
    }

    /// 🧳️ Transfers every original captured field into its genuine known or unresolved owner group.
    pub(crate) fn into_retirement_parts(self)->crate::owned_context::ContextRetirementParts<A>{
        let Self{presence_local,presence_peers,presence_peer_factory,app_instance_id:_,view_state,canonical_base_revision:_,draft_generation:_,transient_generation:_,children,draft,transient,window_config,window_transient,source_capture_started:_,draft_source_fault,transient_source_fault,refused_source_reads,identity_digest:_,tool_run,original_tool_run,provisional,provisional_generation:_,gesture}=self;
        let (tool_run,tool_run_sources)=tool_run.map(|view|{let ToolRunView{tool_id,identity,state,provisional_entities,progress,payload}=view;(crate::owned_context::ContextToolRunMetadata{tool_id,identity,state,progress},crate::owned_context::ContextToolRunSources{entities:provisional_entities,payload})}).unzip();
        crate::owned_context::ContextRetirementParts{metadata:crate::owned_context::ContextOwnedFields{view_state,tool_run,original_tool_run,draft,transient,presence:presence_local.map(store::os_store::snapshot_read::SnapshotRead::into_erased),window_config,window_transient,provisional,gesture,refused_reads:refused_source_reads,transient_source_fault,draft_source_fault},residual:crate::owned_context::ContextOriginalResidual{children,peers:presence_peers,peer_factory:presence_peer_factory,tool_run:tool_run_sources}}
    }
    /// 🔗️ Captures the exact draft and transient source issuers while preserving every refusal.
    pub fn with_source_issuers(mut self,draft:&store::DraftStore<A::Draft,A::DraftMutation>,transient:&store::TransientStore<A::Transient,A::TransientMutation>)->Self {
        if self.source_capture_started{return self;}
        self.source_capture_started=true;
        match draft.snapshot_read(){Ok(read)=>{if let Err(read)=self.draft.bind(read){self.refused_source_reads[0]=Some(read.into_erased());}},Err(error)=>self.draft_source_fault=Some(format!("{error:?}"))}
        match transient.current_read(){Ok(read)=>{if let Err(read)=self.transient.bind(read){self.refused_source_reads[1]=Some(read.into_erased());}},Err(error)=>self.transient_source_fault=Some(error)}
        self
    }
    /// 🪪️ Reports only successfully captured original source authority.
    pub fn source_issuers_ready(&self)->bool{self.source_capture_started&&self.draft_source_fault.is_none()&&self.transient_source_fault.is_none()&&self.refused_source_reads.iter().all(Option::is_none)&&self.draft.has_original_issuer()&&self.transient.has_original_issuer()}

    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }

    /// 👥️ Retains the exact O(1) presence read authority for an app-owned worker stage.
    pub fn with_presence(mut self, local: store::os_store::snapshot_read::SnapshotRead<A::Presence>, peers: std::sync::Arc<store::os_store::presence::PresencePeersRoot<A::Presence>>, peer_factory: Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<A::Presence>>>) -> Self {
        self.presence_local = Some(local);
        self.presence_peers = Some(peers);
        self.presence_peer_factory = peer_factory;
        self
    }

    /// 👥️ Borrows immutable captured presence without projecting or granting publication authority.
    pub fn presence_view(&self) -> Option<PresenceView<'_, A::Presence>> {
        Some(PresenceView { local: self.presence_local.as_ref()?.get(), peers: PresencePeersView::new(self.presence_peers.as_deref()?) })
    }

    /// ⏯️ Binds the instance's tool run as of command admission (identity, state, provisional entities).
    pub fn with_tool_run(mut self, tool_run: Option<ToolRunView>) -> Self {
        self.tool_run = tool_run;
        self
    }

    /// ⏯️ The instance's tool run as of command admission, the identity a command targets a run action at —
    /// never a renderer-echoed cursor. Ephemeral local state, so it is not part of the identity digest.
    pub fn tool_run(&self) -> Option<&ToolRunView> {
        self.tool_run.as_ref()
    }

    /// 📸️ Retains a paid immutable original presentation for this command's lifetime.
    pub fn with_original_tool_run(mut self,view:Option<semio_framework_tool_run_presentation::ToolRunCapturedView>)->Self{self.original_tool_run=view;self}
    /// 🪟️ Borrows the same captured original pages and progress without a result copy.
    pub fn original_tool_run(&self)->Option<semio_framework_tool_run_presentation::ToolRunView<'_>>{self.original_tool_run.as_ref().map(|view|view.borrowed_view())}

    /// 🪞️ Binds the instance's open continuous-tool transactions as of command admission: every open press's and
    /// run's provisional leaves in value form (overlay fold order) and the overlay generation. Ephemeral local state,
    /// folded into the identity digest so a job captured on another overlay is never mistaken for this one.
    pub fn with_provisional(mut self, provisional: Vec<DslValue>, generation: u64) -> Self {
        self.identity_digest = (self.identity_digest ^ generation).wrapping_mul(0x1000_0000_01b3);
        self.provisional = provisional;
        self.provisional_generation = generation;
        self
    }

    /// 🪞️ The provisional leaves a derived view (a preview a retained job evaluates) folds onto the committed
    /// snapshot it was handed, so it follows an open scrub or typing run instead of freezing until the release;
    /// empty while nothing is open. Commands decide over the committed document and never read it.
    pub fn provisional(&self) -> &[DslValue] {
        &self.provisional
    }

    /// 🔢️ The overlay generation as of admission: it moves on every tick, release and zero-trace abort.
    pub fn provisional_generation(&self) -> u64 {
        self.provisional_generation
    }

    /// 🎰️ Binds the dispatching window's gesture slot as of command admission (design §22.10). Ephemeral local tool
    /// state, so it is not part of the identity digest.
    pub fn with_gesture(mut self, gesture: GestureCapture<A::Mutation>) -> Self {
        self.gesture = gesture;
        self
    }

    /// 🖐️ The dispatching window's gesture slot: a streamed tool drives its window's ONE gesture through it
    /// (`GestureSlot::drive`) instead of persisting a gesture of its own; the runtime keeps what the dispatch decided
    /// when it publishes and ends the gesture on host facts.
    pub fn gesture(&self) -> &GestureSlot<A::Mutation> {
        &self.gesture
    }
}
