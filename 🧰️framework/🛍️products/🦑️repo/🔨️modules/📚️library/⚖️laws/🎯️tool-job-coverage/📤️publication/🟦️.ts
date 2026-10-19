/** 📤️ Tool-job coverage: import preparation, dispatch, publication freshness and bounded publication laws.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts
 */
import { readSourceFamilySource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { policyReadFileSafe } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { toolJobProductionSource, toolJobRustBlock } from "../📋️inventory/🟦️.ts";

const PLUGIN = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const STORE = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store";

const withoutStrings = (text: string): string => text.replace(/"(?:[^"\\]|\\.)*"/gu, '""');

/** 🧪️ Replaces the first `before` inside the block that starts at the last `owner` needle, for the hostile mutations of the source laws; a target missing from the owner is a failed self-test. */
export function toolJobSourceMutation(source: string, owner: string, before: string, after: string): string {
  const start = source.lastIndexOf(owner);
  const open = start < 0 ? -1 : source.indexOf("{", start);
  const block = open < 0 ? undefined : toolJobRustBlock(source, open);
  if (!block || !block.body.includes(before)) throw new Error(`[verify interactivity tool-jobs] self-test hostile target "${before}" is missing from ${owner}.`);
  const at = source.indexOf(before, open);
  return source.slice(0, at) + after + source.slice(at + before.length);
}

/** 🧵️ Reads the text of the mounted-instance family the typed-command publication laws bind: the composer and every use-case crate of the instance, the retained command shell and the lifted full-operation job, each crate family in path order. */
export function toolJobPluginInstanceSource(root: string): string {
  return ["🧵️instance", "🪜️maintenance-ladder", "♻️cancellation", "🧵️retained-command", "👷️job"].map((folder) => readSourceFamilySource(`${root}/${PLUGIN}/${folder}`)).join("\n");
}

/** 🏪️ Reads the owning files of the Store one-item publication: the item fold first, then the engine bundle, the publication owner, its contract, the cursor and log operations and the ephemeral publications. The first match of a name is the production owner. */
export function toolJobStoreSource(root: string): string {
  return [
    "⚙️engine/🧱️store-components/📥️lifted/🏪️store/📬️store-owners-batch-fold-and-history/🦀️.rs",
    "⚙️engine/🧳️bundle/🦀️.rs",
    "📬️one-item-owners/📥️lifted/📬️publication/🦀️.rs",
    "🎮️one-item/🦀️.rs",
    "🎮️one-item/📬️publication/🦀️.rs",
    "⚙️engine/🧱️store-components/📥️lifted/🏪️store/🖱️store-cursor-operations/🦀️.rs",
    "⚙️engine/🧱️store-components/📥️lifted/🏪️store/📒️store-log-operations/🦀️.rs",
    "🫧️ephemeral/📤️ephemeral-one-item-publication/🦀️.rs",
    "🫧️ephemeral/👥️presence-store-and-peers-publication/🦀️.rs",
    "🫧️ephemeral/🌫️transient-and-interaction-stores/🦀️.rs",
  ].map((file) => policyReadFileSafe(root, `${STORE}/${file}`)).join("\n");
}

export function toolJobImportPreparationBounded(source: string): boolean {
  const start = source.lastIndexOf("async fn dispatch_import_media");
  const open = start < 0 ? -1 : source.indexOf("{", start);
  const block = open < 0 ? undefined : toolJobRustBlock(source, open);
  return !!block && !block.body.includes("serde_json::to_vec(&(port, media))") && !block.body.includes("raw.clone()") && block.body.includes("build_artifact_reserved_media_job(port, media");
}

export function toolJobTypedCaptureBody(source: string): string | undefined {
  const start = source.lastIndexOf("async fn capture_typed_command_roots(");
  if (start < 0 || source.split("async fn capture_typed_command_roots(").length !== 2) return undefined;
  const body = toolJobRustBlock(source, source.indexOf("{", start))?.body;
  const expected = `
        self.as_render().refresh_cache().await?;
        let (snapshot, config, history) = self.history.command_cache_inputs();
        let canonical_base_revision = self.stores.store.content_revision();
        let window_config_authority = self.stores.window_config_store.capture(meta.view_state.as_ref()).await?;
        let targeted_window_transient_view = match A::retained_window_transient_target(command) {
            Some((window_id, expected_kind)) => {
                let view = meta.view_state.as_ref().ok_or_else(|| plugin_sdk_fault("targeted window transient capture requires an exact ViewModel roster"))?;
                let window = view.window_instances.iter().find(|window| window.id == window_id).ok_or_else(|| plugin_sdk_fault("targeted window transient capture requires an attached window instance"))?;
                if window.window_kind_id != expected_kind {
                    return Err(plugin_sdk_fault("targeted window transient capture does not match the command owner's expected window kind"));
                }
                Some(view.for_window_instance(window_id).ok_or_else(|| plugin_sdk_fault("targeted window transient capture lost its attached window instance"))?)
            }
            None => None,
        };
        let window_transient_authority = self.stores.window_transient_store.capture(targeted_window_transient_view.as_ref().or(meta.view_state.as_ref()))?;
        Ok(TypedCommandRoots {
            snapshot,
            config,
            history,
            children: std::sync::Arc::new(ChildContentView::clone(&self.child_composition.child_content_root)),
            draft_snapshot: self.stores.draft_store.snapshot_root(),
            interaction_state: self.stores.interaction_store.snapshot_root(),
            interaction_hover: self.interaction.interaction_hover.clone(),
            presence_peers: self.stores.presence_store.peers_root(),
            transient: self.stores.transient_store.current_root(),
            peer_presence: std::sync::Arc::clone(&self.roster.peer_presence),
            canonical_base_revision,
            base_revision: semio_framework_job::RevisionId(u64::from_be_bytes(canonical_base_revision[..8].try_into().expect("revision lane width"))),
            generation: semio_framework_job::Generation(self.stores.store.generation()),
            config_generation: self.stores.config_store.generation(),
            draft_generation: self.stores.draft_store.generation(),
            presence_generation: self.stores.presence_store.generation().await,
            transient_generation: self.stores.transient_store.generation().await,
            window_config_authority,
            window_transient_authority,
        })
    `;
  return body && body.replace(/\s+/gu, "") === expected.replace(/\s+/gu, "") ? body : undefined;
}

/** 🧵️ The typed-command seam: dispatch admits the verb into the latest-wins queue behind the full-operation gate and the fixed slot admission, and the latest-wins advance is the only caller of the setup that captures the roots, reserves the operation slot and mounts the worker. Returns the setup body. */
export function toolJobRetainedDispatchSetup(source: string): string | undefined {
  const body = (name: string): string | undefined => {
    const start = source.lastIndexOf(`async fn ${name}(`);
    return start < 0 ? undefined : toolJobRustBlock(source, source.indexOf("{", start))?.body;
  };
  const dispatch = body("dispatch_typed_command_inner");
  const setup = body("start_typed_command_operation");
  const advance = body("advance_latest_wins_command_one");
  if (!dispatch || !setup || !advance) return undefined;
  const guard = dispatch.indexOf("self.media.require_complete_tool_operation_pipeline(&admission)?");
  const slot = dispatch.indexOf("self.media.admit_typed_operation_slot(");
  const retained = dispatch.indexOf("self.tools.latest_wins_commands.insert_admitted(");
  const reservation = setup.indexOf("assert!(self.tools.tool_operations.can_insert(operation_id.0)");
  const capture = toolJobTypedCaptureBody(source);
  const prepare = setup.indexOf("self.capture_typed_command_roots(command.as_ref(), meta).await?");
  const callers = source.split("start_typed_command_operation(").length - 1;
  return guard >= 0 && slot > guard && retained > slot && !dispatch.includes("refresh_cache().await") && !dispatch.includes("start_typed_command_operation(") && !dispatch.includes("tool_jobs.dispatch")
    && callers === 2 && advance.includes("self.as_dispatch().start_typed_command_operation(command, admission, &pending.meta, pending.operation.operation, lease, cx).await")
    && !!capture && reservation >= 0 && prepare > reservation && setup.includes("self.tools.typed_operation_reservations[") ? setup : undefined;
}

/** 🧵️ Typed dispatch performs no worker turn: it admits into the latest-wins queue, the setup mounts the dispatched job as a pending original worker admission behind its operation slot, and the mounted operation's `drive_worker_step` admits that session once and pumps it once through the exact work loan. */
export function toolJobMountedDispatchOneTurnExact(raw: string): boolean {
  const source = toolJobProductionSource(raw);
  const count = (text: string, token: string): number => text.split(token).length - 1;
  if (["dispatch_typed_command_inner", "start_typed_command_operation"].some((name) => count(source, `async fn ${name}(`) !== 1)) return false;
  const start = source.lastIndexOf("async fn dispatch_typed_command_inner(");
  const inner = start < 0 ? undefined : toolJobRustBlock(source, source.indexOf("{", start));
  const setup = toolJobRetainedDispatchSetup(source);
  const workerSignature = "pub fn drive_worker_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<PluginLifecycleStep, semio_framework_job::OriginalTurnFailure>";
  if (!inner || !setup || count(source, workerSignature) !== 1) return false;
  const driveOpen = source.indexOf("{", source.indexOf(workerSignature) + workerSignature.length);
  const drive = toolJobRustBlock(source, driveOpen);
  if (!drive) return false;
  const stages = ["dispatch_wire_retained_with_spec", "RefusedWorkerSource::pending(", "self.tools.tool_operations.insert_admitted("];
  const offsets = stages.map((token) => setup.indexOf(token));
  const turns = ["drive_worker_step(", "admit_pending(", "pump_one", "run_to_completion"];
  const forbidden = ["run_on_worker_async", "run_to_completion", "thread_local!", "A::handle(", "A::ephemeral(", "MountedWorkerJobSession::try_new"];
  return stages.every((token) => count(setup, token) === 1) && count(setup, "self.tools.tool_jobs.dispatch(operation_spec)") === 1
    && offsets.every((offset, index) => offset >= 0 && (index === 0 || offset > offsets[index - 1]!))
    && !["tool_jobs.dispatch", "RefusedWorkerSource", ...turns].some((token) => inner.body.includes(token))
    && !turns.some((token) => setup.includes(token))
    && !forbidden.some((token) => inner.body.includes(token) || setup.includes(token) || drive.body.includes(token))
    && count(drive.body, "rejected.admit_pending(config,cx)") === 1 && count(drive.body, "session.pump_one_for_interactive_turn(same)") === 1 && count(drive.body, "pump_one") === 1
    && drive.body.includes("cx.with_original_work_loan(step_fuel,step_deadline_us,") && !/\b(?:while|loop)\b/.test(withoutStrings(drive.body));
}

/** 🧵️ The typed-command full operation is bounded end to end: dispatch only admits, the roots are captured by the retained setup, the generic bounded job prepares its roots under cancellation and yield checks and then faults, because only an app-owned retained job owns a reducer, and the mounted operation publishes one funded unit per turn through its pending publication, its result page and its ACK. */
export function toolJobFullOperationBounded(source: string): boolean {
  const start = source.lastIndexOf("async fn dispatch_typed_command_inner");
  const open = start < 0 ? -1 : source.indexOf("{", start);
  const block = open < 0 ? undefined : toolJobRustBlock(source, open);
  if (!block) return false;
  const preparationWork = ["refresh_cache().await", "draft_store.snapshot().await", "interaction_store.snapshot().await", "ChildContentView::new", "presence_store.peers().await", "transient_store.current().await", "store.content_revision().await", "store.envelope().await", "presence_store.apply", "transient_store.apply", "dispatch_emit(", "WorkerJobSession::new"];
  const operationStart = source.indexOf("impl<A: ArtifactApp> semio_framework_job::InteractiveJob for TypedCommandFullOperationJob<A>");
  const operationOpen = operationStart < 0 ? -1 : source.indexOf("{", operationStart);
  const operation = operationOpen < 0 ? undefined : toolJobRustBlock(source, operationOpen);
  const operationJob = source.includes("struct TypedCommandFullOperationJob") && !!operation;
  const exactStages = ["typed-command-prepare", "typed-command-reducer"].every((stage) => source.includes(`cx.set_stage("${stage}")`));
  const boundedControl = !!operation && operation.body.includes("cx.is_cancelled()") && operation.body.includes("cx.should_yield()") && operation.body.includes("max_decoded_items") && operation.body.includes("max_output_bytes") && operation.body.includes("ReservedOutcomeKind::Preview") && source.includes("ReservedOutcomeKind::Checkpoint");
  const genericReducerFaults = !!operation && operation.body.includes("generic bounded proof has no resumable app-owned reducer job");
  const shortcuts = ["A::handle", "A::ephemeral", "bounded_command_output_bytes", "serde_json::to_vec", "for child in emit.child_emits", "ActiveToolCommand", "BoundedFirstStepCommandJob"];
  const monolithic = !!operation && shortcuts.some((needle) => operation.body.includes(needle));
  const publisherStart = source.lastIndexOf("fn publish_mounted_typed_operation_unit");
  const publisherOpen = publisherStart < 0 ? -1 : source.indexOf("{", publisherStart);
  const publisher = publisherOpen < 0 ? undefined : toolJobRustBlock(source, publisherOpen);
  const retainedSetup = toolJobRetainedDispatchSetup(source);
  const capturedRoots = toolJobTypedCaptureBody(source);
  return !!(
    preparationWork.every((needle) => !block.body.includes(needle)) &&
    !!retainedSetup &&
    retainedSetup.includes("RefusedWorkerSource::pending(") &&
    retainedSetup.includes("dispatch_wire_retained_with_spec") &&
    retainedSetup.includes("self.tools.tool_operations.insert_admitted(") &&
    capturedRoots?.includes("draft_snapshot: self.stores.draft_store.snapshot_root()") &&
    capturedRoots?.includes("interaction_state: self.stores.interaction_store.snapshot_root()") &&
    retainedSetup.includes("presence_local: Some(self.stores.presence_store.local_read().map_err(Fault::from)?)") &&
    capturedRoots?.includes("presence_peers: self.stores.presence_store.peers_root()") &&
    capturedRoots?.includes("transient: self.stores.transient_store.current_root()") &&
    source.includes("MountedTypedCommandFullOperationStage::AwaitingAck") &&
    source.includes("fn acknowledge_result_page") &&
    source.includes("if page.token != token") &&
    source.includes("advance_typed_operation_publication_one") &&
    source.includes("let mut mounted =") &&
    source.includes(".remove(operation_id)") &&
    source.includes("self.tools.tool_operations.insert_admitted(operation_id, mounted)") &&
    !!publisher &&
    !source.includes("async fn publish_mounted_typed_operation_unit") &&
    ["presence", "transient"].every((lane) => publisher.body.includes(`${lane}.pop()`)) &&
    !publisher.body.includes(".last().cloned()") &&
    !publisher.body.includes(".await") &&
    publisher.body.includes("pending_artifact_publication") &&
    publisher.body.includes("admit_apply_batch") &&
    publisher.body.includes("begin_publish_one") &&
    publisher.body.includes("advance_apply_batch") &&
    publisher.body.includes("advance_publish_one") &&
    !publisher.body.includes(".apply_one(") &&
    publisher.body.includes(".pop()") &&
    toolJobPublicationFreshnessBeforeEveryTurn(source) &&
    publisher.body.includes("content_revision_now()") &&
    !/\bwhile\b/.test(withoutStrings(publisher.body)) &&
    operationJob &&
    exactStages &&
    boundedControl &&
    genericReducerFaults &&
    !monolithic
  );
}

/** 🔒️ Requires document freshness before either resuming or beginning one retained publication turn. */
export function toolJobPublicationFreshnessBeforeEveryTurn(source: string): boolean {
  const publisherStart = source.lastIndexOf("fn publish_mounted_typed_operation_unit");
  const publisherOpen = publisherStart < 0 ? -1 : source.indexOf("{", publisherStart);
  const publisher = publisherOpen < 0 ? undefined : toolJobRustBlock(source, publisherOpen);
  if (!publisher) return false;
  const directFreshness = publisher.body.indexOf("validate_commit(&mounted.operation");
  const helperFreshness = publisher.body.indexOf("typed_operation_document_is_fresh(&mounted.operation");
  const helperStart = source.lastIndexOf("fn typed_operation_document_is_fresh");
  const helperOpen = helperStart < 0 ? -1 : source.indexOf("{", helperStart);
  const helper = helperOpen < 0 ? undefined : toolJobRustBlock(source, helperOpen);
  const helperExact =
    !!helper &&
    helper.body.includes("canonical_revision == live_revision") &&
    helper.body.includes("validate_commit(operation") &&
    helper.body.includes("Generation(live_generation)") &&
    helper.body.includes("CommitValidation::Accepted");
  const freshness = directFreshness >= 0 ? directFreshness : helperExact ? helperFreshness : -1;
  const pendingPublication = publisher.body.indexOf("if let Some(pending) = mounted.pending_artifact_publication.as_mut()");
  const beginTurns = [
    "self.stores.store.admit_apply_batch",
    "self.stores.config_store.admit_apply_batch",
    "self.stores.draft_store.admit_apply_batch",
    "self.stores.window_config_store.admit_begin",
    "self.stores.presence_store.begin_publish_one",
    "self.stores.transient_store.begin_publish_one",
    "self.stores.window_transient_store.begin(",
  ]
    .map((needle) => publisher.body.indexOf(needle))
    .filter((index) => index >= 0);
  return (
    freshness >= 0 &&
    pendingPublication >= 0 &&
    beginTurns.length > 0 &&
    freshness < pendingPublication &&
    beginTurns.every((begin) => freshness < begin) &&
    (publisher.body.includes("mounted.canonical_revision != live_revision") || helperExact)
  );
}

/** 🧵️ Rejects the last generic typed-command false green: a batched publisher that
 * enters whole mutation replay, history construction, or outbound serialization in one turn.
 * ONE gesture stages N mutations into ONE `Edit` and ONE ledger slot, one fold per turn.
 * `storeSource` is the text of {@link toolJobStoreSource}: the item fold first, then the engine bundle, the publication owner and the cursor and log operations. */
export function toolJobStoreBatchPublicationBounded(storeSource: string, pluginSource: string): boolean {
  const blockOf = (source: string, needle: string, fromEnd = false) => {
    const start = fromEnd ? source.lastIndexOf(needle) : source.indexOf(needle);
    const open = start < 0 ? -1 : source.indexOf("{", start);
    return open < 0 ? undefined : toolJobRustBlock(source, open);
  };
  const admission = blockOf(storeSource, "pub fn admit_apply_batch(");
  const advance = blockOf(storeSource, "pub fn advance_apply_batch<P, Mutation>(");
  const cancel = blockOf(storeSource, "pub fn cancel_apply_batch(");
  const fold = blockOf(storeSource, "pub fn fold_batch_item(");
  const base = blockOf(storeSource, "fn batch_item_base<P, Mutation>(");
  const publicationState = blockOf(storeSource, "pub struct ArtifactStoreBatchPublication<P, Mutation>");
  const publication = blockOf(storeSource, "impl<P, Mutation> ArtifactStoreBatchPublication<P, Mutation>");
  const close = publication ? blockOf(publication.body, "pub fn close_step(&mut self, grant: RetainedCloneGrant)") : undefined;
  const terminal = publication ? blockOf(publication.body, "pub fn terminal_is_empty(&self)") : undefined;
  const publisher = blockOf(pluginSource, "fn publish_mounted_typed_operation_unit", true);
  const monolithic = ["replay_mutations(", "apply_command(", "flush_outbound(", "mutation.diff(", "diff.apply(", "try_reserve", ".collect::<Vec", ".to_vec("];
  const publisherText = publisher?.body.replace(/\s+/gu, "") ?? "";
  const lanes = { store: "artifact_mutations", config_store: "config_mutations", draft_store: "draft_mutations" } as const;
  const retiredByProposal = (lane: string): boolean => lane === "artifact_mutations" && publisherText.includes("pending_transaction_proposal=Some(TransactionProposalDraft{local_ops,foreign});emit.artifact_mutations.clear();");
  const admitted = (store: keyof typeof lanes): string => publisherText.match(new RegExp(`self\\.stores\\.${store}\\.admit_apply_batch\\((.*?)\\)\\.map_err`, "u"))?.[1] ?? "";
  return (
    storeSource.includes("pub struct ArtifactStoreBatchPublication") &&
    storeSource.includes("pub enum ArtifactStoreOneItemPublicationPhase") &&
    storeSource.includes("pub trait ArtifactStoreOneItemPreparationFactory") &&
    storeSource.includes("pub trait ArtifactStoreOneItemPreparation") &&
    storeSource.includes("pub struct ArtifactStoreOneItemPrepared") &&
    storeSource.includes("trait ArtifactStoreBatchSource") &&
    !!admission &&
    !!advance &&
    !!cancel &&
    !!fold &&
    !!base &&
    !!publicationState &&
    !!publication &&
    !!close &&
    !!terminal &&
    admission.body.includes("ArtifactStoreBatchPublication") &&
    admission.body.includes("std::mem::take(mutations)") &&
    admission.body.includes("grant.maximum_copy_bytes<copied_bytes") &&
    admission.body.includes("original_is_untouched()") &&
    publication.body.includes("fn begin_close") &&
    close.body.includes("maximum_items: 1") &&
    close.body.includes("maximum_copy_bytes") &&
    close.body.includes("maximum_capacity_bytes") &&
    close.body.includes("maximum_release_bytes") &&
    !close.body.includes(".clear()") &&
    terminal.body.includes("ArtifactStoreOneItemPublicationPhase::Complete") &&
    (terminal.body.includes(".is_empty()") || terminal.body.includes(".is_none()")) &&
    publication.body.includes("maximum_items") &&
    publication.body.includes("maximum_capacity_bytes") &&
    /\b(?:expected_)?generation\b/.test(publicationState.body) &&
    storeSource.includes("impl<P, Mutation> Drop for ArtifactStoreBatchPublication") &&
    monolithic.every((needle) => !admission.body.includes(needle) && !advance.body.includes(needle)) &&
    fold.body.includes("authority.validate_prepared(candidate)") &&
    fold.body.includes("original.inverse.pop()") &&
    fold.body.includes("target.inverse.push_reserved(") &&
    !fold.body.includes("replay_mutations(") &&
    !fold.body.includes("apply_command(") &&
    !advance.body.includes("while ") &&
    !fold.body.includes("while ") &&
    pluginSource.includes("pending_artifact_publication") &&
    pluginSource.includes("artifact_one_item_factory: Option<") &&
    pluginSource.includes("config_one_item_factory: Option<") &&
    pluginSource.includes("draft_one_item_factory: Option<") &&
    pluginSource.includes("unsupported_publication_contracts") &&
    !!publisher &&
    publisher.body.includes("admit_apply_batch") &&
    publisher.body.includes("advance_apply_batch") &&
    (Object.keys(lanes) as (keyof typeof lanes)[]).every((store) => admitted(store).includes(`&mutemit.${lanes[store]},HistoryLane::Document`) && !publisherText.includes(`${lanes[store]}.last().cloned()`) && !publisherText.includes(`emit.${lanes[store]}.clone()`) && publisherText.split(`emit.${lanes[store]}.clear()`).length - 1 === Number(retiredByProposal(lanes[store]))) &&
    publisher.body.includes("self.stores.artifact_one_item_factory.as_ref()") &&
    publisher.body.includes("self.stores.config_one_item_factory.as_ref()") &&
    publisher.body.includes("self.stores.draft_one_item_factory.as_ref()") &&
    !publisher.body.includes("A::build_artifact_store_one_item_preparation_factory") &&
    !publisher.body.includes("A::build_config_store_one_item_preparation_factory") &&
    !publisher.body.includes("A::build_draft_store_one_item_preparation_factory") &&
    !publisher.body.includes(".apply_one(") &&
    !publisher.body.includes(".await")
  );
}

export function toolJobEphemeralOneItemPublicationBounded(storeSource: string, pluginSource: string): boolean {
  const blocks = (source: string, needle: string) => {
    const result: { body: string; end: number }[] = [];
    let cursor = 0;
    for (;;) {
      const start = source.indexOf(needle, cursor);
      if (start < 0) return result;
      const open = source.indexOf("{", start);
      const block = open < 0 ? undefined : toolJobRustBlock(source, open);
      if (!block) return result;
      result.push(block);
      cursor = block.end;
    }
  };
  const advances = blocks(storeSource, "pub fn advance_publish_one(");
  const begins = blocks(storeSource, "pub fn begin_publish_one(");
  const cancels = blocks(storeSource, "pub fn cancel_publish_one(");
  const publication = storeSource.indexOf("impl<P, Mutation> ArtifactEphemeralOneItemPublication<P, Mutation>");
  const publicationOpen = publication < 0 ? -1 : storeSource.indexOf("{", publication);
  const publicationBlock = publicationOpen < 0 ? undefined : toolJobRustBlock(storeSource, publicationOpen);
  const forbidden = ["mutation.diff(", "diff.apply(", "apply_command(", "replay_mutations(", "try_reserve", ".to_vec(", "drop(previous)"];
  const publisherStart = pluginSource.lastIndexOf("fn publish_mounted_typed_operation_unit");
  const publisherOpen = publisherStart < 0 ? -1 : pluginSource.indexOf("{", publisherStart);
  const publisher = publisherOpen < 0 ? undefined : toolJobRustBlock(pluginSource, publisherOpen);
  return (
    storeSource.includes("pub trait ArtifactEphemeralOneItemPreparationFactory") &&
    storeSource.includes("pub trait ArtifactEphemeralOneItemPreparation") &&
    storeSource.includes("pub struct ArtifactEphemeralOneItemPrepared") &&
    storeSource.includes("pub struct ArtifactEphemeralOneItemPublication") &&
    begins.length === 2 &&
    advances.length === 2 &&
    cancels.length === 2 &&
    advances.every((block) => forbidden.every((needle) => !block.body.includes(needle)) && block.body.includes("retirement")) &&
    !!publicationBlock &&
    publicationBlock.body.includes("fn close_step") &&
    publicationBlock.body.includes("fn terminal_is_empty") &&
    storeSource.includes("impl<P, Mutation> Drop for ArtifactEphemeralOneItemPublication") &&
    !!publisher &&
    pluginSource.includes("presence_one_item_factory: Option<") &&
    pluginSource.includes("transient_one_item_factory: Option<") &&
    pluginSource.includes("presence_local_root_retirement_factory: Option<") &&
    pluginSource.includes("transient_local_root_retirement_factory: Option<") &&
    pluginSource.includes("unsupported_publication_contracts") &&
    pluginSource.includes("if let Some(lane) = stores.unsupported_publication_contracts.get(verb)") &&
    publisher.body.includes("PendingArtifactStorePublication::Presence") &&
    publisher.body.includes("PendingArtifactStorePublication::Transient") &&
    (publisher.body.match(/begin_publish_one/g) ?? []).length >= 2 &&
    (publisher.body.match(/advance_publish_one/g) ?? []).length >= 2 &&
    publisher.body.includes("self.stores.presence_one_item_factory.as_deref()") &&
    publisher.body.includes("self.stores.transient_one_item_factory.as_deref()") &&
    publisher.body.includes("self.stores.presence_local_root_retirement_factory.clone()") &&
    publisher.body.includes("self.stores.transient_local_root_retirement_factory.clone()") &&
    !publisher.body.includes("A::build_presence_store_one_item_preparation_factory") &&
    !publisher.body.includes("A::build_transient_store_one_item_preparation_factory") &&
    !publisher.body.includes("presence_store.apply_one") &&
    !publisher.body.includes("transient_store.apply_one") &&
    !publisher.body.includes("presence_store.apply(") &&
    !publisher.body.includes("transient_store.apply(")
  );
}

export function toolJobTypedRouteFailsClosedBeforePreparation(source: string): boolean {
  const start = source.lastIndexOf("async fn dispatch_typed_command_inner");
  const open = start < 0 ? -1 : source.indexOf("{", start);
  const block = open < 0 ? undefined : toolJobRustBlock(source, open);
  const guard = block?.body.indexOf("require_complete_tool_operation_pipeline(&admission)?") ?? -1;
  const preparation = block?.body.indexOf("self.tools.latest_wins_commands.insert_admitted(") ?? -1;
  const authorityStart = source.indexOf("fn require_complete_tool_operation_pipeline");
  const authorityOpen = authorityStart < 0 ? -1 : source.indexOf("{", authorityStart);
  const authority = authorityOpen < 0 ? undefined : toolJobRustBlock(source, authorityOpen);
  if (guard < 0 || preparation <= guard || !toolJobRetainedDispatchSetup(source) || !authority) return false;
  const rejected = authority.body.indexOf("QualifiedToolProof::FrameworkOwned(_) | QualifiedToolProof::Bounded(_) =>");
  const rejection = rejected < 0 ? undefined : toolJobRustBlock(authority.body, authority.body.indexOf("{", rejected));
  return authority.body.includes("self.require_tool_operation_authority(admission)?") && authority.body.includes("QualifiedToolProof::AppOwned(_) => Ok(())")
    && !!rejection && rejection.body.includes('FaultCode::new("interactive-job.missing-owned-reducer")') && rejection.body.includes("Err(") && !rejection.body.includes("Ok(");
}

/** 🧵️ The typed-operation persistence seam: dispatch admits behind the full-operation gate and the fixed slot, the setup captures the roots before it mounts the pending worker session and the operation, the mounted operation steps its worker once per turn through the exact work loan, the maintenance ladder selects the next worker by a bounded fair scan, and one host turn spends its grant on a bounded number of worker steps. */
export function toolJobTypedPersistentFoundation(source: string): boolean {
  const blockOf = (needle: string) => {
    const start = source.lastIndexOf(needle);
    const open = start < 0 ? -1 : source.indexOf("{", start);
    return open < 0 ? undefined : toolJobRustBlock(source, open);
  };
  const dispatch = blockOf("async fn dispatch_typed_command_inner(");
  const active = blockOf("impl<A: ArtifactApp> MountedTypedCommandFullOperation<A>");
  const drive = blockOf("pub fn drive_worker_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>)");
  const selection = blockOf("fn maintenance_tool_selection(&self) -> MaintenanceTool");
  const worker = blockOf("pub fn drive_typed_operation_worker(");
  const setup = toolJobRetainedDispatchSetup(source);
  if (!dispatch || !active || !drive || !selection || !worker || !setup) return false;
  const guard = dispatch.body.indexOf("require_complete_tool_operation_pipeline(&admission)?");
  const slot = dispatch.body.indexOf("self.media.admit_typed_operation_slot(");
  const capture = setup.indexOf("self.capture_typed_command_roots(");
  const session = setup.indexOf("RefusedWorkerSource::pending(");
  const retained = setup.indexOf("self.tools.tool_operations.insert_admitted(");
  return (
    guard >= 0 &&
    slot > guard &&
    capture >= 0 &&
    session > capture &&
    retained > session &&
    dispatch.body.includes('DslValue::String(operation_id.0.to_string())') &&
    !setup.includes("session.step(") &&
    !setup.includes("let outcome = loop") &&
    source.includes("tool_operations: ArtifactFixedRegistry<MountedTypedCommandFullOperation<A>>") &&
    drive.body.includes("session.pump_one_for_interactive_turn(same)") &&
    drive.body.includes("session.checked_out_outcome()") &&
    drive.body.includes("MountedTypedCommandFullOperationStage::Publishing") &&
    drive.body.includes('Ok(_)=>Ok(PluginLifecycleStep::AwaitingInput{reason:"typed operation mounted worker awaits its next outcome"})') &&
    active.body.includes("MountedTypedCommandFullOperationStage::AwaitingAck") &&
    active.body.includes("fn terminal_is_empty") &&
    selection.body.includes("for offset in 0..ARTIFACT_LIVE_OUTPUT_SLOTS") &&
    selection.body.includes("self.tools.tool_operations.entry(index)") &&
    selection.body.includes("every_operation_awaits_presented_ack") &&
    selection.body.includes("MaintenanceTool::Awaiting") &&
    worker.body.includes("for _ in 0..INTERACTIVE_TURN_WORKER_PUMPS") &&
    worker.body.includes("operation.drive_worker_step(same)") &&
    worker.body.includes("INTERACTIVE_TURN_WORKER_WALL_US")
  );
}
