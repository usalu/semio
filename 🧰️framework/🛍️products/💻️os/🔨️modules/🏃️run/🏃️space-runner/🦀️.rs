//! 🏃️ Space runner: the headless run driver that walks the workflow and drives each node through the channel.
//!
//! Holds the `🔖️SpaceRunner` region of the module; the root re-exports its items.

use super::*;

//#region 🔖️SpaceRunner
/// 📊️ What actually happened in one `run()` call — which nodes were recomputed and which were left
/// untouched because neither their document, inputs, nor config changed.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RunReport {
    pub recomputed: Vec<String>,
    pub clean: Vec<String>,
}

/// 🩺️ Computes which nodes `SpaceRunner::run` would recompute, without instantiating a single host
/// — the `--dry` plan. Reuses exactly the dirty check `run` applies, so the plan can never drift from
/// what an actual run would do. `documents`/`configs` map a node's `artifact_ref`/`config_ref` string
/// to its current `(pack, spr)` artifact bytes — missing/absent means "never persisted".
/// `prior_node_records` is the PRIOR SEALED `semio_framework_artifact_workflow_run::RunArtifact.node_records`, keyed by node id
/// (empty for a first-ever run — every node then plans as recomputed).
pub async fn plan(
    graph: &Workflow,
    documents: &BTreeMap<String, (Vec<u8>, Vec<u8>)>,
    configs: &BTreeMap<String, (Vec<u8>, Vec<u8>)>,
    parameter_values: &[RunParameterValue],
    parameter_bindings: &[WorkflowParameterBinding],
    prior_node_records: &BTreeMap<String, RunNodeRecord>,
) -> Result<RunReport, RunError> {
    validate_edge_kinds(graph).await?;
    let order = topological_order(graph)?;
    let node_by_id: HashMap<&str, &WorkflowNode> = graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    let mut incoming: HashMap<&str, Vec<&WorkflowEdge>> = HashMap::new();
    for edge in &graph.edges {
        incoming.entry(edge.target_node_id.as_str()).or_default().push(edge);
    }
    let mut report = RunReport::default();
    for node_id in &order {
        let node = *node_by_id.get(node_id.as_str()).ok_or_else(|| RunError::UnknownNode(node_id.clone()))?;
        let document = documents.get(&node.artifact_ref).cloned().unwrap_or_default();
        let config = configs.get(&node.config_ref).cloned().unwrap_or_default();
        let (document_fingerprint, config_fingerprint) = node_fingerprints(node_id, &document.1, &config.1, parameter_bindings, parameter_values);
        let mut input_fingerprints: BTreeMap<String, String> = BTreeMap::new();
        for edge in incoming.get(node_id.as_str()).into_iter().flatten() {
            let fingerprint = prior_node_records.get(&edge.source_node_id).and_then(|record| record.output_fingerprints.iter().find(|entry| entry.port_id == edge.source_port_id)).map(|entry| entry.fingerprint.clone()).unwrap_or_default();
            input_fingerprints.insert(edge.target_port_id.clone(), fingerprint);
        }
        let dirty = match prior_node_records.get(node_id.as_str()) {
            None => true,
            Some(record) => {
                let previous_inputs: BTreeMap<String, String> = record.input_fingerprints.iter().map(|entry| (entry.port_id.clone(), entry.fingerprint.clone())).collect();
                record.document_fingerprint != document_fingerprint || previous_inputs != input_fingerprints || record.config_fingerprint != config_fingerprint
            }
        };
        if dirty {
            report.recomputed.push(node_id.clone());
        } else {
            report.clean.push(node_id.clone());
        }
    }
    Ok(report)
}

/// 🕸️ Computes one studio's workflow against an `AppChannelHost`. Node dirtiness is decided purely
/// from the PRIOR SEALED run's `semio_framework_artifact_workflow_run::RunNodeRecord`: the document's own fingerprint (did the
/// app's document change since last run — e.g. a UI edit), its resolved input fingerprints (did
/// anything upstream change), and its config's fingerprint (folding in any bound `--param` overlay —
/// see `node_parameter_overlay_bytes`). A clean node is never opened at all; its cached output
/// fingerprints feed straight into its consumers. `run()` is READONLY over `documents`/`configs` — see
/// this crate's module doc — every byte it produces lands in the caller-supplied `RunSink` instead.
// 🔀️ `B` is the pluggable `BlobStore` backend — dedyn-fw-os-misc, R11(a): `BlobStore` has 5 impls
// spread across independently-owned modules/crates (`🏪️store::NoBlobStore`/`FolderEventLogStorage`,
// `🪐️space::TestBlobStore`, this crate's own `FileBlobStore`/`InMemoryBlobStore`) — a genuinely open,
// cross-crate extension point (`🏃️run` cannot name a `🪐️space`/`🏪️store::sync` type without a reverse
// dependency), so `dyn_enum_close!` cannot close it; every caller already supplies a concrete value
// explicitly (`SpaceRunner::new`'s own `blob_store` parameter), so `B` is inferred at every call
// site and the `NoBlobStore` default only matters for a bare `SpaceRunner<H>` mention (none exist
// outside doc comments, verified).
pub struct SpaceRunner<H: AppChannelHost, B: BlobStore + 'static = NoBlobStore> {
    pub(super) host: H,
    pub(super) blob_store: Arc<B>,
    /// ⚖️ This run's local/authority `MergePolicy` (contract-freeze.md §C3) — sent as
    /// `AppCommand::SetMergePolicy` right after every node's `Hello`, in the same batched exchange
    /// (see `compute_node`'s doc). `--policy`/config default: `Normal`.
    merge_policy: protocol::MergePolicy,
    /// 🛑️ This run's own cancellation root — `CancelToken::root()` by default (never cancelled
    /// unless a caller holds a clone via `cancel_token()` and calls `.cancel()` on it), a child of
    /// which becomes every `compute_node` call's `OperationContext.cancel`. One token for the whole
    /// run, not one per node: cancelling it stops the run before its NEXT node (see
    /// `RunError::Cancelled`'s doc) without needing a separate per-node cancel wire.
    pub(super) cancel: CancelToken,
    /// ⏰️ Optional wall-clock deadline (epoch ms) applied to every `OperationContext` this run
    /// builds — `None` (the default) means no deadline, matching every pre-async-rewrite caller's
    /// behavior exactly.
    pub(super) deadline_ms: Option<u64>,
    /// 📊️ Hears every node's polled whole-document load progress — see [`Self::with_document_load_progress`].
    document_load_progress: Option<Box<dyn FnMut(&NodeDocumentLoadProgress) + Send>>,
}

impl<H: AppChannelHost, B: BlobStore + 'static> SpaceRunner<H, B> {
    pub fn new(host: H, blob_store: Arc<B>, merge_policy: protocol::MergePolicy, original_cancel:CancelToken) -> Self {
        Self { host, blob_store, merge_policy, cancel: original_cancel, deadline_ms: None, document_load_progress: None }
    }

    pub fn into_host(self) -> H {
        self.host
    }

    /// 🛑️ A clone of this run's cancellation root — call `.cancel()` on it from another thread (or
    /// a UI cancel button's handler) to stop the run before its next node. `SpaceRunner` itself
    /// never cancels this token.
    pub fn cancel_token(&self) -> CancelToken {
        self.cancel.clone()
    }

    /// ⏰️ Applies a wall-clock deadline (epoch ms) to every `OperationContext` this run builds from
    /// here on. Builder-style: `SpaceRunner::new(..).with_deadline_ms(..)`.
    pub fn with_deadline_ms(mut self, deadline_ms: Option<u64>) -> Self {
        self.deadline_ms = deadline_ms;
        self
    }

    /// 📊️ Hands every node's polled whole-document load progress to `observer` — ephemeral and local-only
    /// ([`NodeDocumentLoadProgress`]), never recorded into the run document. Builder-style, like
    /// [`Self::with_deadline_ms`].
    pub fn with_document_load_progress(mut self, observer: impl FnMut(&NodeDocumentLoadProgress) + Send + 'static) -> Self {
        self.document_load_progress = Some(Box::new(observer));
        self
    }

    /// 🪪️ Builds one node's `OperationContext`: `actor` is the node's own `AppChannelHost` handle
    /// (so a real host can tell which instance an operation belongs to), `cancel` a child of this
    /// run's root (see the `cancel` field's own doc — cancelling the run cancels every node's
    /// context transitively via `CancelToken::child`'s max-severity fold).
    fn node_ctx(&self, node_handle: u32) -> OperationContext {
        OperationContext { actor: node_handle as u64, generation: 0, trace: TraceId(node_handle as u64), lane: 0, deadline_ms: self.deadline_ms, cancel: self.cancel.child_now(), capability: None }
    }

    /// 🔌️ Returns `node`'s already-open handle, opening it (`host.open(node.plugin_id, node.app_id)`)
    /// and caching the handle in `live` on first use. Lazy by construction (unlike a plain
    /// `HashMap::entry(..).or_insert(expr)`, which would evaluate `expr` — and so call `host.open` —
    /// unconditionally even when the entry already exists).
    async fn open_node(&mut self, live: &mut HashMap<String, u32>, node: &WorkflowNode) -> Result<u32, RunError> {
        if let Some(handle) = live.get(&node.id) {
            return Ok(*handle);
        }
        let handle = self.host.open(&node.plugin_id, &node.app_id, &node.artifact_ref).await?;
        live.insert(node.id.clone(), handle);
        Ok(handle)
    }

    /// 🎬️ Runs one node's whole frame script — `SetMergePolicy` (this run's `merge_policy`, sent
    /// FIRST so the instance's local/authority policy is established before any other command
    /// reaches it — channel v12 retires `Hello`/`Welcome` entirely, `host.open` is what now
    /// establishes the instance, via `Event::InstanceOpen` on the reactor ABI, not a wire command),
    /// `LoadConfig` and the admission of the node's whole-document load in one batched exchange; the
    /// load then runs to its acknowledged terminal one poll per exchange ([`Self::drive_document_load`]);
    /// then one `MediaIn` per resolved input, one `MediaOut`+`MediaFingerprint` pair per output port,
    /// `ReadDocument` and finally `ReadConfig` to persist whatever the imports mutated on either artifact
    /// (see this file's header doc: "importing media is emitting operations") in a second batched
    /// exchange. A structured (`Document` wire) input closes its own import batch
    /// ([`Self::import_media_batch`]): a whole document of the node's schema loads as a stepped archive load
    /// before anything after it is sent. A node with no stored document (`SpaceBundle::read_artifact` answers an empty pair)
    /// loads nothing and runs on its app's genesis document. Returns the node's mutated document bytes,
    /// its mutated config bytes, and per output port the exported `Media` plus its wire fingerprint
    /// string.
    async fn compute_node(
        &mut self,
        live: &mut HashMap<String, u32>,
        node: &WorkflowNode,
        document: &(Vec<u8>, Vec<u8>),
        config: &(Vec<u8>, Vec<u8>),
        input_media: &BTreeMap<String, Media>,
    ) -> Result<((Vec<u8>, Vec<u8>), (Vec<u8>, Vec<u8>), BTreeMap<String, (Media, String)>), RunError> {
        // 🛑️ Checked BEFORE `open`/`exchange` — a cancelled run stops before its NEXT node rather
        // than mid-exchange (see `RunError::Cancelled`'s own doc).
        if self.cancel.is_cancelled().await {
            return Err(RunError::Cancelled);
        }
        let handle = self.open_node(live, node).await?;
        let ctx = self.node_ctx(handle);

        let mut seq: u64 = 0;
        let mut next_seq = move || {
            seq += 1;
            seq
        };

        let mut commands = Vec::new();

        let set_policy_seq = next_seq();
        commands.push(AppCommand::SetMergePolicy { seq: set_policy_seq, policy: self.merge_policy.as_u8() });

        let load_config_seq = next_seq();
        commands.push(AppCommand::LoadConfig { seq: load_config_seq, pack: config.0.clone(), spr: config.1.clone() });

        let mut document_load = (!document.0.is_empty() || !document.1.is_empty()).then(|| protocol::DocumentArchiveLoadHost::new(protocol::DocumentArchivePack { parent_pack: document.0.clone(), parent_spr: document.1.clone(), members: Vec::new() }));
        let admission = match document_load.as_mut().map(|load| load.step(&mut next_seq)) {
            Some(protocol::DocumentArchiveLoadStep::Send { seq, command }) => {
                commands.push(command);
                Some(seq)
            }
            _ => None,
        };

        let frames = self.host.exchange(&ctx, handle, commands).await?;
        refuse_unsolicited(&node.app_id, &frames)?;
        expect_done(&node.app_id, set_policy_seq, node_reply(&frames, &node.app_id, set_policy_seq)?)?;
        expect_done(&node.app_id, load_config_seq, node_reply(&frames, &node.app_id, load_config_seq)?)?;
        if let (Some(load), Some(admit_seq)) = (document_load.as_mut(), admission) {
            let admitted = (admit_seq, node_reply(&frames, &node.app_id, admit_seq)?.clone());
            self.drive_document_load(&ctx, handle, node, load, Some(admitted), &mut next_seq).await?;
        }

        let mut commands = Vec::new();
        let mut media_in_seqs = Vec::with_capacity(input_media.len());
        for (port, media) in input_media {
            let (descriptor, data) = media_to_artifact(media, self.blob_store.as_ref()).await?;
            let this_seq = next_seq();
            commands.push(AppCommand::MediaIn { seq: this_seq, port: port.clone(), descriptor, data });
            media_in_seqs.push(this_seq);
            if matches!(media.payload, MediaPayload::Structured { .. }) {
                self.import_media_batch(&ctx, handle, node, std::mem::take(&mut commands), std::mem::take(&mut media_in_seqs), &mut next_seq).await?;
            }
        }

        let mut output_seqs = Vec::with_capacity(node.outputs.len());
        for port in &node.outputs {
            let media_out_seq = next_seq();
            commands.push(AppCommand::MediaOut { seq: media_out_seq, port: port.id.clone(), request: Vec::new() });
            let fingerprint_seq = next_seq();
            commands.push(AppCommand::MediaFingerprint { seq: fingerprint_seq, port: port.id.clone() });
            output_seqs.push((port.id.clone(), media_out_seq, fingerprint_seq));
        }

        let read_artifact_seq = next_seq();
        commands.push(AppCommand::ReadDocument { seq: read_artifact_seq });

        let read_config_seq = next_seq();
        commands.push(AppCommand::ReadConfig { seq: read_config_seq });

        let frames = self.host.exchange(&ctx, handle, commands).await?;
        refuse_unsolicited(&node.app_id, &frames)?;

        let reply_to = |seq: u64| node_reply(&frames, &node.app_id, seq);
        for this_seq in &media_in_seqs {
            expect_done(&node.app_id, *this_seq, reply_to(*this_seq)?)?;
        }

        let mut outputs = BTreeMap::new();
        for (port_id, media_out_seq, fingerprint_seq) in &output_seqs {
            let media = match reply_to(*media_out_seq)? {
                AppFrame::Media { descriptor, data, .. } => media_from_document(descriptor, data.clone(), self.blob_store.as_ref()).await?,
                AppFrame::Error { fault, report, .. } => return Err(RunError::Host(dispatch_error_message(&node.app_id, &format!("failed to produce media on `{port_id}`"), fault, report))),
                other => return Err(RunError::Host(format!("`{}` sent an unexpected frame for media-out `{port_id}`: {other:?}", node.app_id))),
            };
            let fingerprint = match reply_to(*fingerprint_seq)? {
                AppFrame::MediaFingerprint { fingerprint, .. } => decode_fingerprint_wire(fingerprint)?,
                AppFrame::Error { fault, report, .. } => return Err(RunError::Host(dispatch_error_message(&node.app_id, &format!("failed to fingerprint `{port_id}`"), fault, report))),
                other => return Err(RunError::Host(format!("`{}` sent an unexpected frame for media-fingerprint `{port_id}`: {other:?}", node.app_id))),
            };
            outputs.insert(port_id.clone(), (media, fingerprint));
        }

        let mutated_document = match reply_to(read_artifact_seq)? {
            AppFrame::Document { pack, spr, .. } => (pack.clone(), spr.clone()),
            AppFrame::Error { fault, report, .. } => return Err(RunError::Host(dispatch_error_message(&node.app_id, "failed to read its document", fault, report))),
            other => return Err(RunError::Host(format!("`{}` sent an unexpected frame reading its document: {other:?}", node.app_id))),
        };

        let mutated_config = match reply_to(read_config_seq)? {
            AppFrame::Config { pack, spr, .. } => (pack.clone(), spr.clone()),
            AppFrame::Error { fault, report, .. } => return Err(RunError::Host(dispatch_error_message(&node.app_id, "failed to read its config", fault, report))),
            other => return Err(RunError::Host(format!("`{}` sent an unexpected frame reading its config: {other:?}", node.app_id))),
        };

        Ok((mutated_document, mutated_config, outputs))
    }

    /// 🗃️ Drives one node's whole-document load, admitted in [`Self::compute_node`]'s first batch, to its acknowledged terminal
    /// through the stepped archive load (`protocol::DocumentArchiveLoadHost`, `📓️api-stepped-document-load.md` §2): one command
    /// per exchange, every polled status handed to the [`Self::with_document_load_progress`] observer, so a long history folds
    /// over many guest turns. A cancelled run token cancels the load in the guest and keeps polling until the guest restored the
    /// node's previous document with zero trace, then answers [`RunError::Cancelled`], as does a load the guest cancelled itself.
    /// A guest fault, terminal or refused, is the node's host error carrying the guest's own fault code.
    async fn drive_document_load(&mut self, ctx: &OperationContext, handle: u32, node: &WorkflowNode, load: &mut protocol::DocumentArchiveLoadHost, admitted: Option<(u64, AppFrame)>, next_seq: &mut impl FnMut() -> u64) -> Result<(), RunError> {
        let mut answered = admitted;
        loop {
            if let Some((seq, frame)) = answered.take() {
                match load.answer(seq, &frame) {
                    Ok(Some(status)) => {
                        if let Some(observer) = self.document_load_progress.as_mut() {
                            observer(&NodeDocumentLoadProgress { node_id: node.id.clone(), completed: status.completed, total: status.total });
                        }
                    }
                    Ok(None) => {}
                    Err(protocol::DocumentArchiveLoadRefusal::Refused(fault)) => return Err(RunError::Host(dispatch_error_message(&node.app_id, "refused its document load", &fault, &[]))),
                    Err(protocol::DocumentArchiveLoadRefusal::Unanswered) => return Err(RunError::Host(format!("`{}` answered document load seq {seq} with a frame that answers another command", node.app_id))),
                }
            }
            if ctx.cancel.is_cancelled().await {
                load.request_cancel();
            }
            let (seq, command) = match load.step(&mut *next_seq) {
                protocol::DocumentArchiveLoadStep::Send { seq, command } => (seq, command),
                protocol::DocumentArchiveLoadStep::Finished(protocol::DocumentArchiveLoadOutcome::Ready) => return Ok(()),
                protocol::DocumentArchiveLoadStep::Finished(protocol::DocumentArchiveLoadOutcome::Cancelled) => return Err(RunError::Cancelled),
                protocol::DocumentArchiveLoadStep::Finished(protocol::DocumentArchiveLoadOutcome::Fault(fault)) => return Err(RunError::Host(dispatch_error_message(&node.app_id, "failed to load its document", &fault, &[]))),
            };
            let frames = self.host.exchange(ctx, handle, vec![command]).await?;
            refuse_unsolicited(&node.app_id, &frames)?;
            answered = Some((seq, node_reply(&frames, &node.app_id, seq)?.clone()));
        }
    }

    /// 🎞️ Sends one batch of media imports that ends in a structured (`Document` wire) input and answers once the node holds its
    /// result: every earlier import answers `Done`; the last one either imported (`Done`) or carried a whole document of the
    /// node's own schema, which the guest admitted as a stepped archive load under that import's own sequence
    /// (`AppFrame::DocumentArchiveLoad`, `📓️api-stepped-document-load.md` §4) — driven here to its acknowledged terminal like the
    /// node's own document, so no later import, export or read ever observes the document being replaced.
    async fn import_media_batch(&mut self, ctx: &OperationContext, handle: u32, node: &WorkflowNode, commands: Vec<AppCommand>, seqs: Vec<u64>, next_seq: &mut impl FnMut() -> u64) -> Result<(), RunError> {
        let frames = self.host.exchange(ctx, handle, commands).await?;
        refuse_unsolicited(&node.app_id, &frames)?;
        let Some((last, earlier)) = seqs.split_last() else { return Ok(()) };
        for seq in earlier {
            expect_done(&node.app_id, *seq, node_reply(&frames, &node.app_id, *seq)?)?;
        }
        match node_reply(&frames, &node.app_id, *last)? {
            AppFrame::DocumentArchiveLoad { status, .. } if status.operation == *last => self.drive_document_load(ctx, handle, node, &mut protocol::DocumentArchiveLoadHost::admitted(*last), None, next_seq).await,
            frame => expect_done(&node.app_id, *last, frame),
        }
    }

    /// 🕸️ Runs every dirty node in `graph`'s topological order, importing media across each edge
    /// (applying `convert_media` per edge's negotiated `contract` first). `documents`/`configs` stay
    /// READONLY for the whole call — a mutated document/config's bytes go into `sink` (keyed by node
    /// id), never back into these two source maps (see this crate's module doc, "non-destructive
    /// rework"). `sink.record` is called for every `NodeStarted`/`NodeFinished`; the caller owns
    /// emitting `Start` before and `Seal` after this call.
    pub async fn run(
        &mut self,
        graph: &Workflow,
        documents: &BTreeMap<String, (Vec<u8>, Vec<u8>)>,
        configs: &BTreeMap<String, (Vec<u8>, Vec<u8>)>,
        parameter_values: &[RunParameterValue],
        parameter_bindings: &[WorkflowParameterBinding],
        prior_node_records: &BTreeMap<String, RunNodeRecord>,
        cache: &mut dyn MediaCache,
        sink: &mut RunSink,
    ) -> Result<RunReport, RunError> {
        validate_edge_kinds(graph).await?;
        let order = topological_order(graph)?;
        let node_by_id: HashMap<&str, &WorkflowNode> = graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
        let mut incoming: HashMap<&str, Vec<&WorkflowEdge>> = HashMap::new();
        for edge in &graph.edges {
            incoming.entry(edge.target_node_id.as_str()).or_default().push(edge);
        }

        let mut report = RunReport::default();
        let mut live: HashMap<String, u32> = HashMap::new();
        // 🧷️ Progressively filled AS this run computes each node, in topological order — the within-run
        // counterpart to `prior_node_records` (which stays fixed: the prior SEALED run's ground truth).
        // A downstream node's input fingerprint must reflect what its upstream producer computed THIS
        // run, not last time — mirrors the old `RunState.nodes`'s dual role (both "prior run" and "just
        // computed this run"), split into two maps here on purpose so "prior sealed run" (read-only
        // memoization ground truth) and "this run's own progress" (write-as-we-go) can never be confused.
        let mut current_run_records: BTreeMap<String, RunNodeRecord> = BTreeMap::new();

        for node_id in &order {
            let node = *node_by_id.get(node_id.as_str()).ok_or_else(|| RunError::UnknownNode(node_id.clone()))?;
            let document = documents.get(&node.artifact_ref).cloned().unwrap_or_default();
            let config = configs.get(&node.config_ref).cloned().unwrap_or_default();
            let (document_fingerprint, config_fingerprint) = node_fingerprints(node_id, &document.1, &config.1, parameter_bindings, parameter_values);

            let mut input_fingerprints: BTreeMap<String, String> = BTreeMap::new();
            for edge in incoming.get(node_id.as_str()).into_iter().flatten() {
                let fingerprint = current_run_records.get(&edge.source_node_id).and_then(|record| record.output_fingerprints.iter().find(|entry| entry.port_id == edge.source_port_id)).map(|entry| entry.fingerprint.clone()).unwrap_or_default();
                input_fingerprints.insert(edge.target_port_id.clone(), fingerprint);
            }

            let previous = prior_node_records.get(node_id.as_str());
            let dirty = match previous {
                None => true,
                Some(record) => {
                    let previous_inputs: BTreeMap<String, String> = record.input_fingerprints.iter().map(|entry| (entry.port_id.clone(), entry.fingerprint.clone())).collect();
                    record.document_fingerprint != document_fingerprint || previous_inputs != input_fingerprints || record.config_fingerprint != config_fingerprint
                }
            };

            if !dirty {
                report.clean.push(node_id.clone());
                // ⚡️ Cache hit: reuse the prior sealed run's own record verbatim (fingerprints/outputs),
                // just relabeled `CacheHit` — this node is never opened, never mutates anything, and its
                // outputs stay exactly what they already were.
                let previous_record = previous.expect("dirty=false implies a prior record exists").clone();
                let node_record = RunNodeRecord { status: RunNodeStatus::CacheHit, ..previous_record };
                current_run_records.insert(node_id.clone(), node_record.clone());
                sink.record(RunMutation::FinishRunNode(FinishRunNode { node_record })).await?;
                continue;
            }
            report.recomputed.push(node_id.clone());
            sink.record(RunMutation::StartRunNode(StartRunNode { node_id: node_id.clone() })).await?;

            let mut input_media: BTreeMap<String, Media> = BTreeMap::new();
            for edge in incoming.get(node_id.as_str()).into_iter().flatten() {
                let fingerprint = MediaFingerprint(input_fingerprints.get(&edge.target_port_id).cloned().unwrap_or_default());
                let media = match cache.get(&fingerprint) {
                    Some(media) => media,
                    None => {
                        // 🩹️ Defensive one-hop fallback (mirrors the pre-`AppChannelHost` runner's own
                        // behavior): a clean upstream node's output should already be in `cache` from a
                        // prior run; reaching here means it genuinely isn't (e.g. an evicted media
                        // cache dir) — recompute the source node directly, WITHOUT recursively
                        // resolving ITS OWN inputs (a clean node's inputs are, by definition, unchanged
                        // since it was last fully computed). Still fully readonly over `documents`/
                        // `configs`: this recompute's own mutated bytes are simply discarded (`_source_*`)
                        // rather than written anywhere, matching "run never writes over source" even in
                        // this fallback path.
                        let source_node = *node_by_id.get(edge.source_node_id.as_str()).ok_or_else(|| RunError::UnknownNode(edge.source_node_id.clone()))?;
                        let source_document = documents.get(&source_node.artifact_ref).cloned().unwrap_or_default();
                        let source_config = configs.get(&source_node.config_ref).cloned().unwrap_or_default();
                        let (_source_document, _source_config, source_outputs) = self.compute_node(&mut live, source_node, &source_document, &source_config, &BTreeMap::new()).await?;
                        let (media, _fresh_fingerprint) = source_outputs.get(&edge.source_port_id).cloned().ok_or_else(|| RunError::Host(format!("upstream node `{}` produced no output on port `{}`", edge.source_node_id, edge.source_port_id)))?;
                        cache.put(&fingerprint, &media);
                        media
                    }
                };
                let converted = convert_media(&edge.contract, media)?;
                input_media.insert(edge.target_port_id.clone(), converted);
            }

            let started_at = std::time::Instant::now();
            let (mutated_document, mutated_config, outputs) = self.compute_node(&mut live, node, &document, &config, &input_media).await?;
            let duration_ms = started_at.elapsed().as_secs_f64() * 1000.0;

            // 🔒️ THE non-destructive rework's write side: mutated document/config bytes go into `sink`'s
            // run-owned area (keyed by node id), never back into `documents`/`configs` — those stay
            // read-only source maps for the whole call.
            sink.write_node_artifact(node_id, mutated_document.0, mutated_document.1);
            sink.write_node_config(node_id, mutated_config.0, mutated_config.1);

            let mut output_fingerprints = Vec::new();
            let mut run_outputs = Vec::new();
            for (port_id, (media, fingerprint)) in &outputs {
                output_fingerprints.push(PortFingerprint { port_id: port_id.clone(), fingerprint: fingerprint.clone() });
                cache.put(&MediaFingerprint(fingerprint.clone()), media);
                // 📤️ The media cache is the one place this crate already persists exported output bytes
                // durably (`<space>/cache/media/<fingerprint>.json`) — `RunOutputArtifact`
                // points at that real, existing location rather than inventing a second one this wave.
                run_outputs.push(RunOutputArtifact { port_id: port_id.clone(), artifact_id: format!("cache/media/{fingerprint}"), path: format!("cache/media/{fingerprint}.json") });
            }
            let node_record = RunNodeRecord {
                node_id: node_id.clone(),
                status: RunNodeStatus::Computed,
                document_fingerprint,
                config_fingerprint,
                input_fingerprints: input_fingerprints.iter().map(|(port_id, fingerprint)| PortFingerprint { port_id: port_id.clone(), fingerprint: fingerprint.clone() }).collect(),
                output_fingerprints,
                outputs: run_outputs,
                duration_ms,
            };
            current_run_records.insert(node_id.clone(), node_record.clone());
            sink.record(RunMutation::FinishRunNode(FinishRunNode { node_record })).await?;
        }

        Ok(report)
    }
}
//#endregion 🔖️SpaceRunner
