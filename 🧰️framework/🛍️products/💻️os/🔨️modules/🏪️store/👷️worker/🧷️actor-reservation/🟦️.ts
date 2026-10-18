import { type ActorInstanceLifetime, type ActorInstanceOpenRequest, type ActorInstanceCloseRequest, actorInstanceLifetimeEquals, type ActorInstanceLifecycleReceipt } from "../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts";
import { type BrowserActorUiPatchOfferV1, type BrowserActorUiPatchResultV1, captureBrowserActorUiPatchV1, browserActorUiPatchOwnerMatchesV1 } from "@semio-tech/framework/browser-bundle/patch-handoff";
import { type ResolvedPluginViewState, windowViewContext, panelViewContext, sectionViewContext } from "@semio-tech/framework/manifest";
import { ActorDocumentBindingV1, documentBackboneEffectV1, encodeDocumentBackboneControlV1 } from "@semio-tech/framework/backbone/binding";
import { type BrowserActorEphemeralSnapshotV1, type BrowserActorCommandBackboneEnvelopeV1, type BrowserActorCommandPublicationV1, encodeBrowserActorHostEffectV1, decodeBrowserActorIntentPublicationV1, decodeBrowserActorCommandPublicationV1, decodeBrowserActorUnsolicitedPublicationV1, requireBrowserActorCommandBackboneProjectionV1 } from "@semio-tech/framework/browser-bundle/action-handoff/publication";
import { type BrowserActorChildValue, reserveBrowserActorChild } from "../../../🔌️plugin/🌐️browser-bundle/🧵️child/🟦️.ts";
import { type ShardCommandIngressPage, createShardCommandIngressPages } from "@semio-tech/framework-actor/shard-client";
import { driveSpawnedJob, spawnedJobCompletedEvent, wireSpawnJob, typedOperationResult, typedOperationPageAnswerV1 } from "@semio-tech/framework-actor/wire-turn";
import { faultDisplayMessage, decodePackValue, parseDocumentBackboneMessage, BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES, parseInboundDocumentBackboneMessage, decodeBackboneMessage, encodePackValue, decodePackWire, packWireNatural } from "../../../../🟦️";
import { BROWSER_ACTOR_ACTION_MUTATION_MAXIMUM, parseBrowserActorHostEffectBytesV1, type BrowserActorActionRequestV1, type BrowserActorActionResultV1, parseBrowserActorActionRequestV1, BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION, parseBrowserActorHistoryPatchBytesV1, browserActorGuestRefusalReasonV1, browserActorAdmissionRefusalReasonV1 } from "@semio-tech/framework/browser-bundle/action-handoff";
import { HISTORY_TRANSITION_DIFF_SCHEMA, DOCUMENT_BACKBONE_RETENTION_LIMITS } from "@semio-tech/framework-replication";
import { documentActorRecoveryStepV1 } from "../🚑️actor-recovery/🟦️.ts";
import { BROWSER_ACTOR_VISIBLE_SURFACES_V1, type BrowserActorVisibleTurnV1, browserActorVisibleSurfacesV1 } from "../🪟️visible-surfaces/🟦️.ts";
import type { DocumentExecutionTargetProgressV1 } from "@semio-tech/framework/directory/schema";
import type { PersistenceBinding } from "../../../../🟦️";
import { type ColdPairIngressStatus, coldDocumentPairCursorEquals } from "@semio-tech/framework-actor/cold-pair";
import { BROWSER_ACTOR_CHILD_LIMITS } from "@semio-tech/framework/browser-bundle/child/schema";
import { encodeActorUiPatchReceipt } from "../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🩹️patch/🟦️.ts";
import { type BrowserActorActionPublication, BrowserActorGuestRefusalV1, type BrowserActorSpawnedJob, DOCUMENT_BROWSER_ACTOR_RENDER_TURN_LIMIT, type DocumentBrowserActorChild, type DocumentBrowserActorGrant, browserActorActionDisposition, browserActorAppCommandV1, browserActorBytes, browserActorCapturedReceipt, browserActorCloseReceipt, browserActorColdStatus, browserActorCommandIngressStatus, browserActorRecord, browserActorTurnBudget, browserActorTurnResult, browserActorUiIntentV1, browserActorUiPatchReceipt, coldPairIngressStatusDiagnostic, hubBinding, retryColdPairBackpressure, settleColdPairLoading, wipeBrowserActorValue } from "../🧩️actor-wire-helpers/🟦️.ts";
import type { VerifiedColdDocumentPair } from "../🧊️cold-pair/🟦️.ts";
import { type ArtifactState, documentServices, post } from "../🧾️document-state/🟦️.ts";
import { documentBrowserActorLease } from "../🎭️actor-lifecycle/🟦️.ts";
import { emitExecutionTargetStatus, requestDocumentActorRecoveryV1 } from "../📥️execution-target-read/🟦️.ts";
import { handleLocalMsg } from "../♻️artifact-lifecycle/🟦️.ts";
import { documentCatchingUpV1 } from "../🛰️hub-relay/🟦️.ts";
import type { DocumentExecutionTargetLease } from "../🪪️execution-lease/🟦️.ts";

let documentBrowserActorGeneration = 0n;

/** 🧷️ Owns one document's reserved child; only the live private lease and exchanged grant select it. */
export class DocumentBrowserActorReservation {
  readonly generation: bigint;
  private readonly scope: Readonly<{ spaceId: string; documentId: string }>;
  private readonly windowKindId: string;
  /** 🪟️ Every window kind of the verified app; an app command is admitted in any of them. */
  private readonly windowKeys: ReadonlySet<string>;
  /** 🪟️ The window and panel bodies this child renders, by the key the shell's stores and the guest's surface refs share. */
  private readonly surfaceKeys: ReadonlySet<string>;
  private readonly abort = new AbortController();
  private child: DocumentBrowserActorChild | null = null;
  private activation: Promise<void> | null = null;
  private lifetime: ActorInstanceLifetime | null = null;
  private coldOwner: VerifiedColdDocumentPair | null = null;
  private coldApplied: VerifiedColdDocumentPair | null = null;
  private coldTransfer: Promise<void> | null = null;
  private socket: WebSocket | null = null;
  private linkSuspended = false;
  /** 🩹️ The one patch offer awaiting the main thread; `settled` mirrors its outcome so a queued action can wait on it without owning it. */
  private pendingUiPatch: { readonly offer: BrowserActorUiPatchOfferV1; readonly resolve: (result: BrowserActorUiPatchResultV1) => void; readonly reject: (error: Error) => void; readonly timer: ReturnType<typeof setTimeout>; readonly settled: Promise<void> } | null = null;
  private renderedUiPatch = false;
  /** 🩹️ The last revision the shell acknowledged per rendered surface; an intent is admitted against its own surface's. */
  private readonly renderedSurfaceRevisions = new Map<string, number>();
  private acknowledgedUiRevision = 0;
  private mountedUiRevision = 0;
  private renderedViewState: ResolvedPluginViewState | null = null;
  private viewRefresh: Promise<void> | null = null;
  private documentBinding: ActorDocumentBindingV1 | null = null;
  private documentBindingGeneration = 0n;
  private documentBackboneReady = false;
  private backboneDraining = false;
  private pendingBackboneBeforeBinding: Uint8Array[] = [];
  private pendingBackboneBeforeBindingBytes = 0;
  private pollTail: Promise<void> = Promise.resolve();
  private turnTail: Promise<void> = Promise.resolve();
  private localEffectTail: Promise<void> = Promise.resolve();
  private localEffectFailure: Error | null = null;
  private lastActionSequence = 0;
  private lastEphemeral: BrowserActorEphemeralSnapshotV1 | null = null;
  private retirement: Promise<"retired" | "unconfirmed"> | null = null;
  private closed = false;

  private get renderedUiRevision(): number {
    return this.renderedSurfaceRevisions.get(this.windowKindId) ?? 0;
  }

  get retirementOutcome(): Promise<"retired" | "unconfirmed"> | null {
    return this.retirement;
  }

  /** 👥️ The guest's last published presence pack and interaction slice — what this human's presence heartbeat carries
   * for the actor-bound document (the Shell's local instance never sees it). `null` until the guest published one. */
  get ephemeralSnapshot(): BrowserActorEphemeralSnapshotV1 | null {
    return this.lastEphemeral;
  }

  private invokeWithPages(child: DocumentBrowserActorChild, events: BrowserActorChildValue[], commandPage: ShardCommandIngressPage | null, coldPairPage: BrowserActorChildValue | null, assertCurrent: () => void, allowClosing = false): Promise<BrowserActorChildValue> {
    const work = this.pollTail.then(async () => {
      if ((!allowClosing && this.closed) || this.child !== child) throw new Error("document browser actor: closed poll lane");
      assertCurrent();
      if (commandPage) await child.invoke(["reactor", "stageCommandPage"], [commandPage.cursor as unknown as BrowserActorChildValue, commandPage.bytes as unknown as BrowserActorChildValue]);
      if (coldPairPage) await child.invoke(["reactor", "stageColdPairPage"], [coldPairPage]);
      const value = await child.invoke(["reactor", "poll"], [events, browserActorTurnBudget()]);
      assertCurrent();
      return value;
    });
    this.pollTail = work.then(() => {}, () => {});
    return work;
  }

  private invokeJobs(child: DocumentBrowserActorChild, name: "startJob" | "stepJob" | "cancelJob", args: BrowserActorChildValue[], assertCurrent: () => void): Promise<BrowserActorChildValue> {
    const work = this.pollTail.then(async () => {
      if (this.closed || this.child !== child) throw new Error("document browser actor: closed poll lane");
      assertCurrent();
      const value = await child.invoke(["jobs", name], args);
      assertCurrent();
      return value;
    });
    this.pollTail = work.then(() => {}, () => {});
    return work;
  }

  /** 🧵️ Runs each spawned job to a terminal step through the shared {@link driveSpawnedJob} pump and returns the
   * `job-completed` events the guest is owed, in spawn order. */
  private async driveSpawnedJobs(child: DocumentBrowserActorChild, jobs: readonly BrowserActorSpawnedJob[], assertCurrent: () => void): Promise<BrowserActorChildValue[]> {
    const events: BrowserActorChildValue[] = [];
    for (const spawned of jobs) {
      const completion = await driveSpawnedJob({
        ...spawned,
        port: {
          startJob: async (job, kind, input) => void (await this.invokeJobs(child, "startJob", [job, kind, input], assertCurrent)),
          stepJob: (job, budget) => this.invokeJobs(child, "stepJob", [job, { fuel: budget.fuel, deadlineMs: budget.deadlineMs }], assertCurrent),
        },
      });
      const event = spawnedJobCompletedEvent(spawned.job, completion);
      events.push({ tag: event.kind, val: event.payload } as unknown as BrowserActorChildValue);
    }
    return events;
  }

  private invokePoll(child: DocumentBrowserActorChild, events: BrowserActorChildValue[], coldPairPage: BrowserActorChildValue | null, assertCurrent: () => void, allowClosing = false): Promise<BrowserActorChildValue> {
    return this.invokeWithPages(child, events, null, coldPairPage, assertCurrent, allowClosing);
  }

  private invokeCommandPage(child: DocumentBrowserActorChild, page: ShardCommandIngressPage | null, assertCurrent: () => void): Promise<BrowserActorChildValue> {
    return this.invokeWithPages(child, [], page, null, assertCurrent);
  }

  private enqueueTurn<T>(work: () => Promise<T>, allowClosing = false): Promise<T> {
    const operation = this.turnTail.then(async () => {
      if (this.closed && !allowClosing) throw new Error("document browser actor: closed turn lane");
      return work();
    });
    this.turnTail = operation.then(() => {}, () => {});
    return operation;
  }

  private currentDocumentSource() {
    return { runtimeKey: this.state.runtimeKey, clientInstanceId: this.state.openClientInstanceId, scope: { ...this.scope } } as const;
  }

  private publishMountedIfReady(): void {
    if (!this.renderedUiPatch || this.renderedUiRevision < 1 || this.acknowledgedUiRevision !== this.renderedUiRevision || this.mountedUiRevision !== 0 || this.documentBinding === null || !this.documentBackboneReady) return;
    const cold = this.coldApplied;
    if (cold === null) return;
    cold.assertCurrent();
    const identity = this.lease.fields(),
      headEditOrdinal = cold.frontier.headEditOrdinal,
      lastCommitSeq = cold.frontier.lastCommitSeq;
    if (identity.browserActor.kind !== "closed-browser-actor") throw new Error("document browser actor: mounted identity mismatch");
    if (headEditOrdinal > BigInt(Number.MAX_SAFE_INTEGER) || lastCommitSeq > BigInt(Number.MAX_SAFE_INTEGER)) throw new Error("document browser actor: mounted frontier capacity");
    const frontier = {
      documentId: cold.frontier.documentId,
      headEditOrdinal: Number(headEditOrdinal),
      headEditId: cold.frontier.headEditId,
      lastCommitSeq: Number(lastCommitSeq),
      chainHash: Array.from(cold.frontier.chainSha256),
    };
    if (
      identity.checkpoint.baselineFrontier.documentId !== frontier.documentId ||
      identity.checkpoint.baselineFrontier.headEditOrdinal !== frontier.headEditOrdinal ||
      identity.checkpoint.baselineFrontier.headEditId !== frontier.headEditId ||
      identity.checkpoint.baselineFrontier.lastCommitSeq !== frontier.lastCommitSeq ||
      identity.checkpoint.baselineFrontier.chainHash.some((byte, index) => byte !== frontier.chainHash[index])
    ) throw new Error("document browser actor: mounted checkpoint mismatch");
    post({
      kind: "browser-actor-ui-mounted",
      scope: { ...identity.scope },
      clientInstanceId: this.state.openClientInstanceId,
      activationGeneration: this.generation.toString(),
      instanceId: 0,
      verifiedSurfaceId: identity.surface.surfaceId,
      windowKindId: identity.surface.windowKindId,
      catalogGenerationId: identity.catalog.generationId,
      componentSha256: identity.package.componentSha256,
      descriptorSha256: identity.package.descriptorByteSha256,
      browserActorSha256: identity.browserActor.sha256,
      activeCheckpointId: identity.checkpoint.checkpointId,
      descriptorDigestV1: identity.checkpoint.descriptorDigestV1,
      frontier,
      uiRevision: this.renderedUiRevision,
    });
    this.mountedUiRevision = this.renderedUiRevision;
  }

  refreshHostView(): void {
    if (this.closed || !this.activation || this.viewRefresh) return;
    const activation = this.activation;
    this.viewRefresh = (async () => {
      await activation;
      await this.coldTransfer;
      await this.enqueueTurn(async () => {
        const child = this.child;
        if (!child || !this.coldApplied) return;
        const assertCurrent = () => {
          if (this.closed || !this.linkCurrent() || this.state.browserActorReservation !== this || documentBrowserActorLease(this.state) !== this.lease) throw new Error("document browser actor: stale host view");
          this.lease.assertBrowserActorCurrent();
        };
        while (this.state.browserActorViewState !== null && this.state.browserActorViewState !== this.renderedViewState) {
          assertCurrent();
          await this.renderSurface(child, assertCurrent);
        }
      });
    })().catch((error: unknown) => {
      if (this.closed) return;
      const binding = hubBinding(this.state.config);
      if (binding) emitExecutionTargetStatus(this.state, binding, "renderer-unavailable", undefined, error instanceof Error ? error.message : String(error));
      this.close();
    }).finally(() => { this.viewRefresh = null; });
  }

  bindApprovalUndoIfMounted(): void {
    const owner = this.coldApplied;
    if (owner !== null && this.renderedUiPatch) documentServices.documentMounted({ state: this.state, reservation: this, pair: owner });
  }

  private assertDocumentOwnerCurrent(): void {
    if (this.closed || this.state.browserActorReservation !== this || documentBrowserActorLease(this.state) !== this.lease || this.lease.browserActorGrant() !== this.grant) throw new Error("actor-document-port.stale");
    this.lease.assertBrowserActorCurrent();
  }

  /** 🧭️ Routes one guest turn's WIT effects: backbone messages to the hub port, Shell messages to the typed-operation and
   * publication decoders, spawned jobs to the job driver, and only the closed host-intent set to the Shell. A
   * `publish-event` is the guest's own pub/sub publication and has no Shell consumer — the local shard lane's
   * `wireEffectToFriendly` yields nothing for it either — so it is neither forwarded nor fatal (a committed checkpoint
   * publishes one, and treating it as a foreign host effect closed the document child on every auto check-in:
   * `unsupported host effect publish-event`, hub 7800, ticket 26/09/23 C10). An app command's invocation projects the
   * OPERATION envelopes it sent; history transitions (undo, redo, checkpoint commit — `HISTORY_TRANSITION_DIFF_SCHEMA`)
   * are framework records no invocation projects, so a checkpoint's transition is routed without a projection. */
  private async routeTurnEffects(value: BrowserActorChildValue, mode: "ordinary" | "control" | BrowserActorActionPublication = "ordinary"): Promise<Readonly<{ receipts: readonly Uint8Array[]; mutations: number; publications: number; refusals: readonly string[]; hostEffects: readonly (readonly number[])[]; historyPatches: readonly (readonly number[])[]; jobs: readonly BrowserActorSpawnedJob[]; acknowledgements: readonly BrowserActorChildValue[] }>> {
    const result = browserActorTurnResult(value),
      effects = result.effects;
    if (effects !== undefined && !Array.isArray(effects)) throw new Error("document browser actor: invalid effects");
    const receipts: Uint8Array[] = [],
      hostEffects: (readonly number[])[] = [],
      jobs: BrowserActorSpawnedJob[] = [],
      historyPatches: (readonly number[])[] = [],
      acknowledgements: BrowserActorChildValue[] = [],
      messages: Readonly<{ payload: Uint8Array }>[] = [],
      commandEnvelopes: BrowserActorCommandBackboneEnvelopeV1[] = [];
    const refusals: string[] = [];
    let mutations = 0,
      publications = 0,
      artifactPages = 0,
      commandPublication: Extract<BrowserActorCommandPublicationV1, { readonly kind: "invocation" }> | null = null;
    const completions: Extract<BrowserActorCommandPublicationV1, { readonly kind: "completion" }>[] = [];
    for (const raw of (effects ?? []) as BrowserActorChildValue[]) {
      const effect = browserActorRecord(raw, "document browser actor: invalid effect");
      if (effect.tag === "publish-event") continue;
      if (effect.tag === "spawn-job" && mode !== "control") {
        const { job, kind, input } = wireSpawnJob({ tag: effect.tag, val: effect.val }).spawnJob;
        jobs.push({ job, kind, input: input.slice() });
        continue;
      }
      if (effect.tag !== "send-message") {
        if (typeof mode === "string") throw new Error("document browser actor: unsupported effect");
        hostEffects.push(encodeBrowserActorHostEffectV1(effect));
        continue;
      }
      const body = browserActorRecord(effect.val, "document browser actor: invalid message effect"),
        target = browserActorRecord(body.target, "document browser actor: invalid message target"),
        payload = browserActorBytes(body.payload, "document browser actor: invalid message payload");
      if (target.tag === "shell") {
        if (target.val !== 0) throw new Error("document browser actor: foreign shell effect");
        const page = mode === "control" ? null : typedOperationResult({ tag: "send-message", val: { target: { tag: "shell", val: target.val }, payload } });
        if (page !== null) {
          const answer = typedOperationPageAnswerV1(page);
          if (answer.refused) refusals.push(faultDisplayMessage(Array.from(page.payload), decodePackValue));
          if (answer.artifact) artifactPages += 1;
          acknowledgements.push({ tag: "message", val: { source: { tag: "shell", val: page.token.receiver }, payload: Uint8Array.from(answer.acknowledgement.payload.payload) } });
          continue;
        }
        if (mode === "control") receipts.push(payload);
        else if (typeof mode !== "string") {
          const publication = mode.kind === "ui-intent" ? decodeBrowserActorIntentPublicationV1(payload) : decodeBrowserActorCommandPublicationV1(payload, mode.sequence);
          if (publication.kind === "ephemeral") {
            this.lastEphemeral = publication.snapshot;
            continue;
          }
          if (publication.kind === "merge-report") continue;
          if (publication.kind === "error") {
            refusals.push(publication.detail);
            continue;
          }
          if (publication.kind !== "emit" && publication.historyPatch !== null) historyPatches.push(publication.historyPatch);
          if (publication.kind === "operation-completed") {
            if (mode.commit !== null) throw new Error("action-publication-mismatch");
            mode.commit = { operation: publication.operation.toString(10), revision: publication.revision.toString(10) };
            continue;
          }
          if (publication.kind === "completion") {
            completions.push(publication);
            continue;
          }
          if (mode.kind === "app-command") commandPublication = publication as Extract<BrowserActorCommandPublicationV1, { readonly kind: "invocation" }>;
          publications += 1;
        } else {
          const publication = decodeBrowserActorUnsolicitedPublicationV1(payload);
          if (publication.kind === "ephemeral") this.lastEphemeral = publication.snapshot;
          if (publication.kind === "completion" || publication.kind === "operation-completed") {
            if (publication.historyPatch !== null) historyPatches.push(publication.historyPatch);
            if (publication.kind === "completion") completions.push(publication);
          }
        }
        continue;
      }
      if (mode === "control") throw new Error("actor-document-control.data-before-receipt");
      if (target.tag !== "backbone" || typeof target.val !== "string") throw new Error("document browser actor: unsupported message target");
      if (!this.documentBackboneReady && !this.backboneDraining) throw new Error("actor-document-port.not-live");
      const binding = this.documentBinding;
      if (binding === null || target.val !== binding.port.uri) throw new Error("actor-document-port.foreign-uri");
      const kind = documentBackboneEffectV1(payload),
        parsed = kind === "mutations" ? parseDocumentBackboneMessage(payload) : null,
        messageMutations = parsed?.envelopes.length ?? 0;
      mutations += messageMutations;
      if (mutations > BROWSER_ACTOR_ACTION_MUTATION_MAXIMUM) throw new Error("document browser actor: mutation effect limit");
      if (typeof mode !== "string" && mode.kind === "app-command" && parsed !== null) commandEnvelopes.push(...parsed.envelopes);
      messages.push({ payload });
    }
    if (typeof mode !== "string" && mode.kind === "app-command") {
      const projections = [...(commandPublication === null ? [] : [commandPublication]), ...completions],
        carrying = projections.filter((publication) => publication.projection.mutations.length !== 0);
      if (carrying.length > 1) throw new Error("action-publication-mismatch");
      const owner = carrying[0] ?? projections[0];
      if (carrying.length === 0 && artifactPages !== 0) {
        for (const publication of projections) requireBrowserActorCommandBackboneProjectionV1(publication, []);
      } else {
        const operationEnvelopes = commandEnvelopes.filter((envelope) => envelope.diff.schema !== HISTORY_TRANSITION_DIFF_SCHEMA);
        if (owner === undefined && operationEnvelopes.length !== 0) throw new Error("action-publication-unprojected");
        for (const publication of projections) requireBrowserActorCommandBackboneProjectionV1(publication, publication === owner ? operationEnvelopes : []);
      }
    }
    const retainedHostEffects = parseBrowserActorHostEffectBytesV1(hostEffects);
    if (retainedHostEffects.length > 1) throw new Error("document browser actor: host effect limit");
    for (const message of messages) {
      const binding = this.documentBinding;
      if (binding === null || !binding.port.send(binding.port.uri, message.payload)) throw new Error("actor-document-port.retired");
    }
    await this.localEffectTail;
    if (this.localEffectFailure !== null) throw this.localEffectFailure;
    return { receipts, mutations, publications, refusals, hostEffects: retainedHostEffects, historyPatches, jobs, acknowledgements };
  }

  private async bindDocumentBackbone(child: DocumentBrowserActorChild): Promise<void> {
    if (this.documentBinding !== null) return this.documentBinding.bind();
    const owner = { ...this.currentDocumentSource(), actorId: this.grant.actorId, activationGeneration: this.generation, instanceId: 0 } as const,
      binding = new ActorDocumentBindingV1(owner, ++this.documentBindingGeneration, {
        current: () => {
          try { this.assertDocumentOwnerCurrent(); return true; } catch { return false; }
        },
        assertActive: () => this.assertDocumentOwnerCurrent(),
        limits: { messageBytes: BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES, pendingBytes: DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumBytes, pendingMessages: DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumMessages },
        send: payload => {
          const kind = documentBackboneEffectV1(payload);
          if (kind === "remote-ingest-receipt") return;
          const work = this.localEffectTail.then(async () => {
            if (this.localEffectFailure !== null) throw this.localEffectFailure;
            const priorBytes = this.state.pendingDocumentBackboneBytes,
              priorMessages = this.state.pendingDocumentBackboneMessages;
            await handleLocalMsg(this.state, { kind: "documentBackbone", message: payload });
            if (this.state.pendingDocumentBackboneMessages !== priorMessages + 1 || this.state.pendingDocumentBackboneBytes !== priorBytes + payload.byteLength) throw new Error("actor-document-port.admission-refused");
          });
          this.localEffectTail = work.then(
            () => {},
            error => { this.localEffectFailure = error instanceof Error ? error : new Error("actor-document-port.admission-refused"); },
          );
        },
        exchange: async command => {
          const exchange = async () => {
            let result: BrowserActorChildValue | null = null;
            try {
              result = await this.invokePoll(
                child,
                [{ tag: "message", val: { source: { tag: "shell", val: 0 }, payload: encodeDocumentBackboneControlV1(command) } }],
                null,
                () => {
                  if (this.child !== child || this.lifetime === null) throw new Error("actor-document-control.stale-child");
                  if (command.operation !== "retire") this.assertDocumentOwnerCurrent();
                },
                command.operation === "retire",
              );
              if (browserActorColdStatus(result).kind !== "idle" || this.captureUiPatch(result, this.lifetime!) !== null) throw new Error("actor-document-control.unexpected-turn-output");
              const routed = await this.routeTurnEffects(result, "control");
              if (routed.mutations !== 0) throw new Error("actor-document-control.data-before-receipt");
              return routed.receipts;
            } finally {
              if (result !== null) wipeBrowserActorValue(result);
            }
          };
          return command.operation === "retire" ? this.enqueueTurn(exchange, true) : exchange();
        },
        deliver: async payload => {
          await this.enqueueTurn(async () => {
            let result: BrowserActorChildValue | null = null;
            try {
              result = await this.invokePoll(child, [{ tag: "message", val: { source: { tag: "backbone", val: binding.port.uri }, payload } }], null, () => this.assertDocumentOwnerCurrent());
              if (browserActorColdStatus(result).kind !== "idle") throw new Error("actor-document-port.unexpected-cold-ingress");
              await this.driveTurnResult(result, child, () => this.assertDocumentOwnerCurrent());
              result = null;
              await this.refreshDocumentSurfaces(child, () => this.assertDocumentOwnerCurrent(), false);
            } finally {
              if (result !== null) wipeBrowserActorValue(result);
            }
          });
        },
      });
    this.documentBinding = binding;
    try {
      await binding.bind();
    } catch (error) {
      if (this.documentBinding === binding) this.documentBinding = null;
      void binding.port.retire().catch(() => {});
      throw error;
    }
  }

  /** 📥️ Delivers the backbone retained before the binding (a catch-up tail among it) in arrival order — reporting
   * `catching-up` progress per delivered message, which the mounted surface clears — then opens the
   * port for live traffic. The guest answers each ingested batch on the bound port (its `remote-ingest-receipt`), so the
   * bound port accepts the guest's sends while the retained backlog drains. */
  private async flushPendingBackbone(): Promise<void> {
    const binding = this.documentBinding;
    if (binding === null) throw new Error("actor-document-port.unbound");
    this.backboneDraining = true;
    try {
      const hub = hubBinding(this.state.config);
      let delivered = 0;
      while (this.pendingBackboneBeforeBinding.length > 0) {
        const queued = this.pendingBackboneBeforeBinding;
        this.pendingBackboneBeforeBinding = [];
        this.pendingBackboneBeforeBindingBytes = 0;
        const retained = delivered + queued.length;
        for (const payload of queued) {
          if (hub !== null) emitExecutionTargetStatus(this.state, hub, "catching-up", { stage: "catch-up", completedBytes: delivered, totalBytes: retained });
          if (!(await binding.port.receive(this.currentDocumentSource(), payload))) throw new Error("actor-document-port.stale-queued-message");
          delivered += 1;
        }
      }
    } finally {
      this.backboneDraining = false;
    }
    this.documentBackboneReady = true;
    this.publishMountedIfReady();
  }

  private retainBackboneBeforeBinding(bytes: Uint8Array): void {
    const message = this.admitInboundBackbone(bytes);
    if (this.pendingBackboneBeforeBinding.length >= DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumMessages || message.byteLength > DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumBytes - this.pendingBackboneBeforeBindingBytes) throw new Error("actor-document-port.capacity");
    this.pendingBackboneBeforeBinding.push(message);
    this.pendingBackboneBeforeBindingBytes += message.byteLength;
  }

  /** 🛂️ One canonical inbound message for this document's actor: a mutation batch scoped to this document, or the
   * retraction of history transitions the hub refused (`BackboneMessage::Retract`). */
  private admitInboundBackbone(bytes: Uint8Array): Uint8Array {
    const message = parseInboundDocumentBackboneMessage(bytes);
    if (decodeBackboneMessage(message).kind === "retract") return message;
    if (parseDocumentBackboneMessage(message).envelopes.some(envelope => envelope.document_id !== this.state.config.documentId)) throw new Error("actor-document-port.scope");
    return message;
  }

  async receiveBackbone(bytes: Uint8Array): Promise<void> {
    this.assertDocumentOwnerCurrent();
    const binding = this.documentBinding;
    if (binding === null || !this.documentBackboneReady) {
      this.retainBackboneBeforeBinding(bytes);
      return;
    }
    if (!(await binding.port.receive(this.currentDocumentSource(), this.admitInboundBackbone(bytes)))) throw new Error("actor-document-port.stale");
  }

  /** ⏳️ Resolves once no patch offer awaits the main thread and no host view refresh is outstanding — poll-free, by
   * wrapping the promises those two already own. It MUST run outside the turn lane: `refreshHostView` settles by
   * running its own turn on that lane, so a turn that awaited `viewRefresh` from inside would wait on the turn
   * queued behind itself. A patch deadline rejects through here (the action falls to `action-refused: <why>`). */
  private async awaitUiQuiescence(): Promise<void> {
    while (this.pendingUiPatch !== null || this.viewRefresh !== null) {
      if (this.pendingUiPatch !== null) await this.pendingUiPatch.settled;
      else await this.viewRefresh;
    }
  }

  /** 🎯️ Applies one authenticated action in issue order: the turn lane is the FIFO (L5 — an early input queues, it is
   * never refused as `action-busy`), a patch still being acknowledged is awaited rather than refused, and the
   * action is accepted against any revision this lifetime painted. */
  async dispatchAction(raw: BrowserActorActionRequestV1): Promise<BrowserActorActionResultV1> {
    const request = parseBrowserActorActionRequestV1(raw);
    let invoked = false;
    try {
      await this.awaitUiQuiescence();
      return await this.enqueueTurn(async () => {
        // 🩹️ Offers are acknowledged inside the turn that made them, so this is null at a turn boundary today; it is
        // awaited (never refused) so the contract holds if an offer ever outlives its turn.
        if (this.pendingUiPatch !== null) await this.pendingUiPatch.settled;
        const fields = this.lease.fields();
        this.assertDocumentOwnerCurrent();
        // 🪞️ The mailbox sends one action at a time and stamps each at issue, so a queued click carries the revision
        // on screen when it was made, which later acknowledged patches have since passed. Any revision this lifetime
        // painted on the action's own surface (1 ..= its last acknowledged revision: the window an app command was issued
        // in, the intent's window or panel body for a UI intent) is the user's; the guest judges an intent's geometry
        // staleness itself (`DEFAULT_REVISION_TOLERANCE`).
        const intent = request.payload.kind === "ui-intent" ? browserActorUiIntentV1(request) : null,
          command = intent === null ? browserActorAppCommandV1(request, fields, this.windowKeys) : null;
        const painted = request.surfaceRevision >= 1 && request.surfaceRevision <= (this.renderedSurfaceRevisions.get(intent?.surfaceKey ?? command!.surfaceKey) ?? 0);
        if (
          request.scope.spaceId !== fields.scope.spaceId ||
          request.scope.documentId !== fields.scope.documentId ||
          request.verifiedSurfaceId !== fields.surface.surfaceId ||
          request.appChannelVersion !== BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION ||
          request.activationGeneration !== this.generation.toString() ||
          request.instanceId !== 0 ||
          !painted ||
          request.actionSequence <= this.lastActionSequence
        ) throw new Error("action-owner-mismatch");
        if (
          documentCatchingUpV1({
            bound: this.documentBinding !== null,
            backboneReady: this.documentBackboneReady,
            bootstrap: this.state.artifactBootstrap !== null,
            rebootstrap: this.state.artifactRebootstrapRequired,
            tail: this.state.requiredTailFrontier !== null,
            coldApplied: this.coldApplied !== null,
            coldTransfer: this.coldTransfer !== null,
          })
        ) throw new Error("action-catching-up");
        const child = this.child;
        if (child === null) throw new Error("action-child-unavailable");
        this.lastActionSequence = request.actionSequence;
        const publication: BrowserActorActionPublication = { kind: request.payload.kind, sequence: request.actionSequence, frames: 0, commit: null, refusals: [], hostEffects: [], historyPatches: [] };
        let mutationCount = 0;
        if (intent !== null) {
          invoked = true;
          const result = await this.invokePoll(child, [{ tag: "ui-intent", val: { instance: 0, intent: intent.bytes } }], null, () => this.assertDocumentOwnerCurrent());
          mutationCount = await this.driveTurnResult(result, child, () => this.assertDocumentOwnerCurrent(), publication);
        } else {
          const pages = createShardCommandIngressPages({ owner: 0n, generation: this.generation, commandIndex: 0, commandCount: 1, instance: 0, seq: BigInt(request.actionSequence), command: command!.bytes });
          let terminal = "idle";
          for (const page of pages) {
            invoked = true;
            const result = await this.invokeCommandPage(child, page, () => this.assertDocumentOwnerCurrent());
            terminal = browserActorCommandIngressStatus(result);
            mutationCount += await this.driveTurnResult(result, child, () => this.assertDocumentOwnerCurrent(), publication);
            if (terminal === "backpressure" || terminal === "fault") throw new Error("action-command-ingress-refused");
          }
          for (let turn = 0; terminal !== "command-complete" && turn < 1_024; turn += 1) {
            const result = await this.invokeCommandPage(child, null, () => this.assertDocumentOwnerCurrent());
            terminal = browserActorCommandIngressStatus(result);
            mutationCount += await this.driveTurnResult(result, child, () => this.assertDocumentOwnerCurrent(), publication);
            if (terminal === "backpressure" || terminal === "fault") throw new Error("action-command-ingress-refused");
          }
          if (terminal !== "command-complete") throw new Error("action-command-ingress-unconfirmed");
        }
        if (publication.refusals.length !== 0) throw new BrowserActorGuestRefusalV1(publication.refusals[0] ?? null);
        if (publication.frames !== 1) throw new Error("action-publication-mismatch");
        await this.refreshDocumentSurfaces(child, () => this.assertDocumentOwnerCurrent(), command !== null || mutationCount > 0);
        this.state.actorRecovery = documentActorRecoveryStepV1(this.state.actorRecovery, { kind: "applied", atMs: Date.now() }).memory;
        return browserActorActionDisposition(request, "guest-applied", mutationCount, publication.commit, parseBrowserActorHostEffectBytesV1(publication.hostEffects), undefined, parseBrowserActorHistoryPatchBytesV1(publication.historyPatches));
      });
    } catch (error) {
      const explicitRefusal = error instanceof BrowserActorGuestRefusalV1;
      if (invoked && !explicitRefusal) {
        this.close();
        requestDocumentActorRecoveryV1(this.state, "action-unconfirmed");
      }
      const reason = explicitRefusal ? browserActorGuestRefusalReasonV1(error.detail) : error instanceof Error && /^(action-owner-mismatch|action-catching-up|action-child-unavailable)$/u.test(error.message) ? error.message : invoked ? "action-state-unconfirmed" : browserActorAdmissionRefusalReasonV1(error instanceof Error ? error.message : String(error));
      return browserActorActionDisposition(request, "rejected", 0, null, [], reason);
    }
  }
  private readonly retire = () => this.close();

  /** 🔌️ The link this child answers to is the document's current socket, or it is suspended by a short link loss. */
  private linkCurrent(): boolean {
    return this.linkSuspended || this.state.socket === this.socket;
  }

  /** 🔌️ Keeps a MOUNTED child through a short link loss (cold pair applied, surface painted): actions keep applying and
   * their backbone effects queue in the outbox. A child still activating is not resumable and returns `false`. */
  suspendLink(): boolean {
    if (this.closed || this.child === null || this.coldApplied === null || !this.renderedUiPatch || !this.documentBackboneReady || this.linkSuspended) return false;
    this.linkSuspended = true;
    this.socket = null;
    return true;
  }

  get suspended(): boolean {
    return this.linkSuspended && !this.closed;
  }

  /** 🔁️ Adopts the re-admitted grant of the same hub actor (see `DocumentExecutionTargetLease.readmitBrowserActor`). */
  adoptGrant(grant: DocumentBrowserActorGrant): void {
    if (!this.linkSuspended || grant.actorId !== this.grant.actorId) throw new Error("document browser actor: grant actor mismatch");
    this.grant = grant;
  }

  /** 🔁️ Binds the socket whose `Session` re-admitted this child's actor; the missed tail and the queued outbox then
   * travel through the ordinary resume path (`Welcome` None/Tail, `Commands`, `Session` flush). */
  resumeLink(socket: WebSocket): boolean {
    if (!this.suspended || this.lease.browserActorGrant() !== this.grant) return false;
    this.linkSuspended = false;
    this.socket = socket;
    return true;
  }

  constructor(
    private readonly state: ArtifactState,
    private readonly lease: DocumentExecutionTargetLease,
    private grant: DocumentBrowserActorGrant,
  ) {
    if (documentBrowserActorGeneration === 0xffffffffffffffffn) throw new Error("document browser actor: generation exhausted");
    const fields = lease.fields();
    this.scope = Object.freeze({ ...fields.scope });
    this.windowKindId = fields.surface.windowKindId;
    this.windowKeys = new Set(lease.renderSurfaces().windows.map(({ key }) => key));
    this.surfaceKeys = new Set([...this.windowKeys, ...lease.renderSurfaces().panels.map(({ key }) => key), ...BROWSER_ACTOR_VISIBLE_SURFACES_V1.sections.map(({ bodyKey }) => bodyKey)]);
    this.generation = ++documentBrowserActorGeneration;
    // ⏳️ A LIVE browser actor is bounded by its socket and its lease, never by the calendar the
    // admission plan was minted with. `grant.retireAtMs` is `plan.expiresAtUnixMs`, whose TTL is at
    // most `DOCUMENT_OPEN_PLAN_MAX_TTL_MS` (30 s) — the window in which the one-shot plan receipt may
    // be exchanged for a socket grant, not a lifetime. Arming a retirement on it closed the child of
    // every healthy hub document ~30 s after the plan was minted, with the socket still open and the
    // guest still answering (measured 2026-09-22 on hub 7681: `grant retired at … (admitted with
    // -10 ms left)` while the gis actor had been `actor-ready` for 21 s). The two signals below are
    // the real bounds: the document's abort and the lease's retirement.
    state.docAbort.signal.addEventListener("abort", this.retire, { once: true });
    lease.retirement.addEventListener("abort", this.retire, { once: true });
  }

  async reserve(): Promise<void> {
    const fields = this.lease.fields();
    if (fields.browserActor.kind !== "closed-browser-actor") throw new Error("document browser actor: unavailable");
    const child = await reserveBrowserActorChild({ actorId: this.grant.actorId, activationGeneration: this.generation, bundleSha256: fields.browserActor.sha256, bundleByteLength: fields.browserActor.byteLength }, this.abort.signal);
    this.child = child;
    if (this.closed || this.state.browserActorReservation !== this || documentBrowserActorLease(this.state) !== this.lease || this.lease.browserActorGrant() !== this.grant || Date.now() >= this.grant.reserveBeforeMs) {
      this.close();
      throw new Error("document browser actor: stale reservation");
    }
  }

  activate(socket: WebSocket): Promise<void> {
    if (this.activation) {
      if (this.socket !== socket) return Promise.reject(new Error("document browser actor: activation socket mismatch"));
      return this.activation;
    }
    this.socket = socket;
    this.activation = (async () => {
      const assertCurrent = () => {
        if (
          this.closed ||
          this.state.socket !== socket ||
          !this.state.hubActorReady ||
          this.state.actor !== this.grant.actorId ||
          this.state.pendingSocketActorId !== null ||
          this.state.browserActorReservation !== this ||
          documentBrowserActorLease(this.state) !== this.lease ||
          this.lease.browserActorGrant() !== this.grant
        )
          throw new Error("document browser actor: stale Session");
        this.lease.assertBrowserActorCurrent();
      };
      assertCurrent();
      const child = this.child,
        binding = hubBinding(this.state.config);
      if (!child || !binding) throw new Error("document browser actor: unavailable child");
      const stage = (name: DocumentExecutionTargetProgressV1["stage"], completedBytes = 0): void => {
        if (this.mountedUiRevision === 0) emitExecutionTargetStatus(this.state, binding, "verifying", { stage: name, completedBytes, totalBytes: 1 });
      };
      await this.lease.activateBrowserActor(
        child,
        this.abort.signal,
        assertCurrent,
        (progress) => {
          assertCurrent();
          emitExecutionTargetStatus(this.state, binding, "verifying", progress);
        },
        (progress) => emitExecutionTargetStatus(this.state, binding, "retrying", progress),
      );
      assertCurrent();
      stage("actor-open");
      this.lifetime = await this.openGuest(child, assertCurrent);
      assertCurrent();
      stage("actor-cold");
      const owner = this.state.verifiedColdPair;
      if (owner) await this.transferColdPair(owner, child, binding, assertCurrent);
      stage("actor-view");
      this.refreshHostView();
      stage("actor-ready", 1);
    })();
    return this.activation;
  }

  async installColdPair(owner: VerifiedColdDocumentPair, retainedBackbone: readonly Uint8Array[] = []): Promise<void> {
    if (this.closed) throw new Error("document browser actor: closed reservation");
    this.documentBackboneReady = false;
    for (const message of retainedBackbone) this.retainBackboneBeforeBinding(message);
    if (!this.child || !this.socket || !this.activation) return;
    await this.activation;
    if (!this.lifetime) return;
    await this.viewRefresh;
    const binding = hubBinding(this.state.config),
      socket = this.socket;
    if (!binding) throw new Error("document browser actor: missing hub binding");
    const assertCurrent = () => {
      if (this.closed || this.socket !== socket || !this.linkCurrent() || this.state.browserActorReservation !== this || documentBrowserActorLease(this.state) !== this.lease) throw new Error("document browser actor: stale cold transfer");
      owner.assertCurrent();
    };
    await this.transferColdPair(owner, this.child, binding, assertCurrent);
    if (!this.documentBackboneReady) await this.flushPendingBackbone();
    this.refreshHostView();
  }

  private async openGuest(child: DocumentBrowserActorChild, assertCurrent: () => void): Promise<ActorInstanceLifetime> {
    const fields = this.lease.fields();
    const request: ActorInstanceOpenRequest = { kind: "open", activationGeneration: this.generation, instanceId: 0, requestSequence: 1 };
    const config = new Uint8Array(0),
      quotas = new Uint8Array(0);
    let result: BrowserActorChildValue | null = null;
    try {
      result = await this.invokePoll(
        child,
        [
          {
              tag: "instance-open",
              val: {
                instance: request.instanceId,
                activationGeneration: request.activationGeneration,
                requestSequence: BigInt(request.requestSequence),
                appId: fields.surface.appId,
                actor: this.grant.actorId,
                config,
                assets: [],
                capabilities: [],
                quotas,
              },
          },
        ],
        null,
        assertCurrent,
      );
      assertCurrent();
      const captured = browserActorCapturedReceipt(result, request);
      if (browserActorColdStatus(result, true).kind !== "idle") throw new Error("document browser actor: cold ingress before ACK");
      if (this.captureUiPatch(result, captured.lifetime) !== null) throw new Error("document browser actor: patch before lifecycle ACK");
      const acknowledged = await this.invokePoll(child, [{ tag: "instance-lifecycle-ack", val: { tag: "captured", val: { lifetime: { ...captured.lifetime }, requestSequence: BigInt(captured.requestSequence) } } }], null, assertCurrent);
      wipeBrowserActorValue(result);
      result = acknowledged;
      assertCurrent();
      if (browserActorColdStatus(acknowledged).kind !== "idle") throw new Error("document browser actor: cold ingress during ACK");
      if (this.captureUiPatch(acknowledged, captured.lifetime) !== null) throw new Error("document browser actor: patch before cold load");
      return captured.lifetime;
    } finally {
      if (config.byteLength) config.fill(0);
      if (quotas.byteLength) quotas.fill(0);
      if (result !== null) wipeBrowserActorValue(result);
    }
  }

  private transferColdPair(owner: VerifiedColdDocumentPair, child: DocumentBrowserActorChild, binding: Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }>, assertCurrent: () => void): Promise<void> {
    if (this.coldApplied === owner) return Promise.resolve();
    if (this.coldOwner === owner && this.coldTransfer) return this.coldTransfer;
    if (this.coldTransfer) return Promise.reject(new Error("document browser actor: cold transfer already active"));
    const lifetime = this.lifetime;
    if (!lifetime) return Promise.reject(new Error("document browser actor: guest not live"));
    this.coldOwner = owner;
    this.coldApplied = null;
    this.renderedUiPatch = false;
    this.renderedSurfaceRevisions.clear();
    this.acknowledgedUiRevision = 0;
    this.mountedUiRevision = 0;
    this.lastActionSequence = 0;
    this.documentBackboneReady = false;
    this.coldTransfer = this.enqueueTurn(async () => {
      for (let pageIndex = 0; pageIndex < owner.pageCount; pageIndex += 1) {
        const sendPage = async (): Promise<BrowserActorChildValue> => {
          assertCurrent();
          const page = owner.page(lifetime, pageIndex);
          const pageRecord = browserActorRecord(page, "document browser actor: invalid cold page");
          const bytes = pageRecord.bytes;
          if (!(bytes instanceof Uint8Array)) throw new Error("document browser actor: invalid cold bytes");
          try {
            const sent = await this.invokePoll(child, [], page, assertCurrent);
            if (bytes.byteLength !== 0) {
              wipeBrowserActorValue(sent);
              throw new Error("document browser actor: cold page ownership not transferred");
            }
            return sent;
          } finally {
            if (bytes.byteLength) bytes.fill(0);
          }
        };
        let result: BrowserActorChildValue | null = null;
        try {
          result = await retryColdPairBackpressure(sendPage, (value) => browserActorColdStatus(value), lifetime, () => {
            assertCurrent();
            return this.invokePoll(child, [], null, assertCurrent);
          }, wipeBrowserActorValue);
          assertCurrent();
          if (pageIndex + 1 === owner.pageCount) {
            const cursor = { lifetime, transferGeneration: owner.transferGeneration, pageIndex, pageCount: owner.pageCount };
            const loadingStatus = (value: BrowserActorChildValue): ColdPairIngressStatus => {
              const answered = browserActorColdStatus(value);
              if (answered.kind === "loading" && this.captureUiPatch(value, lifetime) !== null) throw new Error("document browser actor: patch before cold pair applied");
              return answered;
            };
            result = await settleColdPairLoading(result, loadingStatus, cursor, () => {
              assertCurrent();
              return this.invokePoll(child, [], null, assertCurrent);
            }, wipeBrowserActorValue);
            assertCurrent();
            const status = browserActorColdStatus(result);
            if (status.kind !== "applied") throw new Error(`document browser actor: cold pair not applied (${coldPairIngressStatusDiagnostic(status)})`);
            owner.assertApplied(status, lifetime);
            this.coldApplied = owner;
            await this.bindDocumentBackbone(child);
            await this.reconcileUiPatches(result, child, assertCurrent);
            await this.renderSurface(child, assertCurrent);
          } else {
            const status = browserActorColdStatus(result);
            const expected = { lifetime, transferGeneration: owner.transferGeneration, pageIndex, pageCount: owner.pageCount };
            if (status.kind !== "pageAccepted" || !coldDocumentPairCursorEquals(status.cursor, expected))
              throw new Error(`document browser actor: invalid page receipt (page ${pageIndex + 1}/${owner.pageCount} answered ${coldPairIngressStatusDiagnostic(status)})`);
            if (this.captureUiPatch(result, lifetime) !== null) throw new Error("document browser actor: patch before cold pair applied");
          }
        } finally {
          if (result !== null) wipeBrowserActorValue(result);
        }
      }
      assertCurrent();
      this.coldApplied = owner;
      this.bindApprovalUndoIfMounted();
      if (!this.renderedUiPatch) emitExecutionTargetStatus(this.state, binding, "renderer-unavailable");
    }).then(() => this.flushPendingBackbone()).finally(() => {
      if (this.coldOwner === owner) this.coldOwner = null;
      this.coldTransfer = null;
    });
    return this.coldTransfer;
  }

  /** 🖼️ Makes every verified window (each in its own window context), every verified panel body (panel context) and every
   * reserved refresh section (section context; the static catalogue on the lifetime's first paint only) visible to the
   * child, exactly as the local refresh binds them, and reconciles the turn's patches until the render settles. Rendering
   * the panels here is what keeps an actor-bound document's inspector live (G-P1-4); rendering every window is what lets a
   * second window of the document show and command the live document; rendering the sections is what keeps its window
   * chrome and tool measures live (C13 P1). */
  private async renderSurface(child: DocumentBrowserActorChild, assertCurrent: () => void): Promise<void> {
    const lifetime = this.lifetime;
    if (lifetime === null) throw new Error("document browser actor: missing render lifetime");
    assertCurrent();
    const hostView = this.state.browserActorViewState;
    if (hostView === null) throw new Error("document browser actor: missing host view context");
    const visible = this.visibleEvents(lifetime, hostView, this.renderedViewState === null ? "mount" : "repaint");
    for (let turn = 0; turn < DOCUMENT_BROWSER_ACTOR_RENDER_TURN_LIMIT; turn += 1) {
      assertCurrent();
      let result: BrowserActorChildValue | null = await this.invokePoll(child, turn === 0 ? visible : [{ tag: "wake" }], null, assertCurrent);
      try {
        assertCurrent();
        if (browserActorColdStatus(result).kind !== "idle") throw new Error("document browser actor: unexpected cold ingress during render");
        const owned = result;
        result = null;
        if (!(await this.reconcileUiPatches(owned, child, assertCurrent)).moreWork) {
          this.renderedViewState = hostView;
          this.publishMountedIfReady();
          return;
        }
      } finally {
        if (result !== null) wipeBrowserActorValue(result);
      }
    }
    throw new Error("document browser actor: render turn limit");
  }

  /** 🪟️ One `surface-visible` per surface `turn` announces (`🪟️visible-surfaces`): a window in its own window context of
   * `hostView`, a panel in its panel context, a reserved refresh section in its section context — a fresh encoding per
   * event (one buffer shared by two events is a `value alias` the child boundary refuses). */
  private visibleEvents(lifetime: ActorInstanceLifetime, hostView: ResolvedPluginViewState, turn: BrowserActorVisibleTurnV1): BrowserActorChildValue[] {
    return browserActorVisibleSurfacesV1(this.lease.renderSurfaces(), turn).map(({ surface, bodyKey, context }): BrowserActorChildValue => {
      if (context === "window") {
        const viewState = windowViewContext(hostView, surface);
        if (!viewState || viewState.activeWindowKindId !== surface) throw new Error("document browser actor: unknown host window instance");
        return { tag: "surface-visible", val: { surface: { instance: lifetime.instanceId, surface }, bodyKey, viewState: encodePackValue(viewState) } };
      }
      return { tag: "surface-visible", val: { surface: { instance: lifetime.instanceId, surface }, bodyKey, viewState: encodePackValue(context === "panel" ? panelViewContext(hostView) : sectionViewContext(hostView)) } };
    });
  }

  /** 🗂️ Re-projects the document's rendered surfaces after a turn that may have changed the document — the actor lane's
   * twin of the Shell's refresh, which re-takes every window, panel and section after each local action. Every verified
   * panel body and every live section always (on a document change the guest re-renders only its mounted WINDOW: without
   * this an open inspector stayed at its last projection, ticket 26/09/23 C10 run `c10gp14e`, and draw's "N layers"
   * engagement stayed at the opening's count, C13 P1); every verified window too when `windows` — after an app command or
   * a mutating action, whose turn does not re-render the author's own window: the author saw its own edit only on its next
   * action (~20 s later, C11 `c11self4`) while the peer saw it at once from the relay. Unchanged surfaces answer no patch. */
  private async refreshDocumentSurfaces(child: DocumentBrowserActorChild, assertCurrent: () => void, windows: boolean): Promise<void> {
    const lifetime = this.lifetime,
      hostView = this.renderedViewState;
    if (lifetime === null || hostView === null) return;
    const events = this.visibleEvents(lifetime, hostView, windows ? "repaint" : "refresh");
    const result = await this.invokePoll(child, events, null, assertCurrent);
    if (browserActorColdStatus(result).kind !== "idle") {
      wipeBrowserActorValue(result);
      throw new Error("document browser actor: unexpected cold ingress during surface refresh");
    }
    if ((await this.driveTurnResult(result, child, assertCurrent)) !== 0) throw new Error("document browser actor: surface refresh mutated the document");
  }

  private captureUiPatch(value: BrowserActorChildValue, lifetime: ActorInstanceLifetime) {
    const result = browserActorTurnResult(value);
    return captureBrowserActorUiPatchV1(result.uiPatches, browserActorUiPatchReceipt(result.uiPatchReceipt), lifetime, this.surfaceKeys, { decodePack: decodePackWire, natural: packWireNatural });
  }

  /** 🩻️ Names a failure the reservation resolves by itself. `refreshHostView`'s catch reports only
   * while the reservation is still open, so a deadline that closes the child before its rejection
   * propagates left the shell on "Verifying document component…" forever with no reason anywhere. */
  private reportExecutionTargetFault(diagnostic: string): void {
    if (this.closed) return;
    const binding = hubBinding(this.state.config);
    if (binding) emitExecutionTargetStatus(this.state, binding, "integrity-failed", undefined, diagnostic);
  }

  private awaitUiPatchResult(offer: BrowserActorUiPatchOfferV1): Promise<BrowserActorUiPatchResultV1> {
    if (this.pendingUiPatch !== null) return Promise.reject(new Error("document browser actor: patch result already pending"));
    let resolve!: (result: BrowserActorUiPatchResultV1) => void, reject!: (error: Error) => void;
    const result = new Promise<BrowserActorUiPatchResultV1>((resolvePatch, rejectPatch) => {
      resolve = resolvePatch;
      reject = rejectPatch;
    });
    const timer = setTimeout(
      () => {
        if (this.pendingUiPatch?.offer !== offer) return;
        this.pendingUiPatch = null;
        this.reportExecutionTargetFault("document browser actor: patch result deadline");
        reject(new Error("document browser actor: patch result deadline"));
        this.close();
      },
      Math.min(15_000, BROWSER_ACTOR_CHILD_LIMITS.invokeMs),
    );
    // ⏳️ `settled` lets `awaitUiQuiescence` wait for this offer; its own no-op catch keeps an unobserved deadline
    // from surfacing as an unhandled rejection while awaiters still see the rejection.
    const settled = result.then(() => undefined);
    settled.catch(() => {});
    this.pendingUiPatch = { offer, resolve, reject, timer, settled };
    post({ ...offer, clientInstanceId: this.state.openClientInstanceId });
    return result;
  }

  settleUiPatch(result: BrowserActorUiPatchResultV1): void {
    const pending = this.pendingUiPatch;
    if (pending === null || !browserActorUiPatchOwnerMatchesV1(pending.offer, result)) return;
    clearTimeout(pending.timer);
    this.pendingUiPatch = null;
    pending.resolve(result);
  }

  private async reconcileUiPatches(
    initial: BrowserActorChildValue,
    child: DocumentBrowserActorChild,
    assertCurrent: () => void,
    publication: BrowserActorActionPublication | null = null,
  ): Promise<Readonly<{ moreWork: boolean; mutations: number }>> {
    const lifetime = this.lifetime;
    if (lifetime === null) throw new Error("document browser actor: missing patch lifetime");
    let value: BrowserActorChildValue | null = initial;
    let mutations = 0;
    let lastRejection: string | null = null;
    const jobs: BrowserActorSpawnedJob[] = [];
    const acknowledgements: BrowserActorChildValue[] = [];
    try {
      for (let patchCount = 0; patchCount < 8; patchCount += 1) {
        const routed = await this.routeTurnEffects(value, publication ?? "ordinary");
        mutations += routed.mutations;
        jobs.push(...routed.jobs);
        acknowledgements.push(...routed.acknowledgements);
        if (publication !== null) {
          publication.frames += routed.publications;
          publication.refusals.push(...routed.refusals);
          publication.hostEffects.push(...routed.hostEffects);
          publication.historyPatches.push(...routed.historyPatches);
          parseBrowserActorHistoryPatchBytesV1(publication.historyPatches);
          parseBrowserActorHostEffectBytesV1(publication.hostEffects);
          if (publication.frames > 1) throw new Error("action-publication-mismatch");
          if (publication.hostEffects.length > 1) throw new Error("document browser actor: host effect limit");
        }
        if (mutations > BROWSER_ACTOR_ACTION_MUTATION_MAXIMUM) throw new Error("document browser actor: mutation effect limit");
        const captured = this.captureUiPatch(value, lifetime);
        if (captured === null && (jobs.length !== 0 || acknowledgements.length !== 0)) {
          const completions = await this.driveSpawnedJobs(child, jobs.splice(0), assertCurrent);
          wipeBrowserActorValue(value);
          value = null;
          value = await this.invokePoll(child, [...acknowledgements.splice(0), ...completions], null, assertCurrent);
          continue;
        }
        if (captured === null) return { moreWork: browserActorRecord(browserActorTurnResult(value).status, "document browser actor: invalid render status").tag === "more-work", mutations };
        const fields = this.lease.fields();
        const offer: BrowserActorUiPatchOfferV1 = {
          kind: "browser-actor-ui-patch",
          scope: { ...fields.scope },
          verifiedSurfaceId: fields.surface.surfaceId,
          activationGeneration: this.generation.toString(),
          instanceId: captured.instanceId,
          patches: captured.patches,
          receipt: Array.from(encodeActorUiPatchReceipt(captured.receipt)),
        };
        wipeBrowserActorValue(value);
        value = null;
        const result = await this.awaitUiPatchResult(offer);
        assertCurrent();
        const feedback = result.verdicts.map((verdict, index) => {
          if (verdict.outcome === "acknowledged" && verdict.revision !== captured.patches[index]!.revision) throw new Error("document browser actor: acknowledged revision mismatch");
          if (verdict.outcome === "acknowledged") this.renderedSurfaceRevisions.set(verdict.surface, verdict.revision);
          else lastRejection = verdict.reason ?? "<unnamed>";
          return {
            tag: verdict.outcome === "acknowledged" ? "patch-ack" : "patch-rejected",
            val: {
              receipt: { lifetime: { ...captured.receipt.lifetime }, patchSequence: captured.receipt.patchSequence },
              surface: { instance: captured.instanceId, surface: verdict.surface },
              revision: BigInt(verdict.revision),
              ...(verdict.outcome === "rejected" ? { reason: verdict.reason } : {}),
            },
          };
        });
        const windowVerdict = result.verdicts.find((verdict) => verdict.surface === this.windowKindId);
        if (windowVerdict?.outcome === "acknowledged") this.renderedUiPatch = true;
        value = await this.invokePoll(child, [...feedback, ...acknowledgements.splice(0)], null, assertCurrent);
        assertCurrent();
        if (browserActorColdStatus(value).kind !== "idle") throw new Error("document browser actor: cold ingress after patch feedback");
        if (windowVerdict?.outcome === "acknowledged") this.acknowledgedUiRevision = windowVerdict.revision;
      }
      throw new Error(`document browser actor: patch feedback limit${lastRejection === null ? "" : ` (last rejection: ${lastRejection})`}`);
    } finally {
      if (value !== null) wipeBrowserActorValue(value);
    }
  }

  private async driveTurnResult(
    initial: BrowserActorChildValue,
    child: DocumentBrowserActorChild,
    assertCurrent: () => void,
    publication: BrowserActorActionPublication | null = null,
  ): Promise<number> {
    let value: BrowserActorChildValue | null = initial,
      mutations = 0;
    try {
      for (let turn = 0; turn < DOCUMENT_BROWSER_ACTOR_RENDER_TURN_LIMIT; turn += 1) {
        const owned = value;
        value = null;
        const settled = await this.reconcileUiPatches(owned, child, assertCurrent, publication);
        mutations += settled.mutations;
        if (mutations > BROWSER_ACTOR_ACTION_MUTATION_MAXIMUM) throw new Error("document browser actor: mutation effect limit");
        if (!settled.moreWork) {
          this.publishMountedIfReady();
          return mutations;
        }
        value = await this.invokePoll(child, [{ tag: "wake" }], null, assertCurrent);
      }
      throw new Error("document browser actor: turn continuation limit");
    } finally {
      if (value !== null) wipeBrowserActorValue(value);
    }
  }

  private async retireGuest(child: DocumentBrowserActorChild, lifetime: ActorInstanceLifetime): Promise<"retired" | "unconfirmed"> {
    const request: ActorInstanceCloseRequest = { kind: "close", lifetime, requestSequence: 2 },
      assertRetained = () => {
        if (this.child !== child || this.lifetime === null || !actorInstanceLifetimeEquals(this.lifetime, lifetime)) throw new Error("document browser actor: stale retirement owner");
      };
    let result: BrowserActorChildValue | null = null;
    try {
      result = await this.invokePoll(child, [{ tag: "instance-close", val: { lifetime: { ...lifetime }, requestSequence: BigInt(request.requestSequence) } }], null, assertRetained, true);
      let receipt = browserActorCloseReceipt(result, request, null);
      if (receipt?.kind !== "accepted" || this.captureUiPatch(result, lifetime) !== null || ((browserActorTurnResult(result).effects as readonly unknown[] | undefined)?.length ?? 0) !== 0) throw new Error("document browser actor: close not accepted");
      wipeBrowserActorValue(result);
      result = null;
      const accepted: ActorInstanceLifecycleReceipt = receipt;
      result = await this.invokePoll(
        child,
        [{ tag: "instance-lifecycle-ack", val: { tag: "accepted", val: { lifetime: { ...accepted.lifetime }, requestSequence: BigInt(accepted.requestSequence), closeGeneration: accepted.closeGeneration } } }],
        null,
        assertRetained,
        true,
      );
      for (let turn = 0; turn < DOCUMENT_BROWSER_ACTOR_RENDER_TURN_LIMIT; turn += 1) {
        receipt = browserActorCloseReceipt(result, request, accepted);
        if (this.captureUiPatch(result, lifetime) !== null || ((browserActorTurnResult(result).effects as readonly unknown[] | undefined)?.length ?? 0) !== 0) throw new Error("document browser actor: close emitted work");
        if (receipt?.kind === "retired") {
          const retired = receipt;
          wipeBrowserActorValue(result);
          result = await this.invokePoll(
            child,
            [{ tag: "instance-lifecycle-ack", val: { tag: "retired", val: { lifetime: { ...retired.lifetime }, requestSequence: BigInt(retired.requestSequence), closeGeneration: retired.closeGeneration } } }],
            null,
            assertRetained,
            true,
          );
          if (browserActorCloseReceipt(result, request, retired) !== null || this.captureUiPatch(result, lifetime) !== null || ((browserActorTurnResult(result).effects as readonly unknown[] | undefined)?.length ?? 0) !== 0) throw new Error("document browser actor: retired ACK not terminal");
          return "retired";
        }
        if (receipt !== null && receipt.kind !== "accepted") throw new Error("document browser actor: invalid close progress");
        wipeBrowserActorValue(result);
        result = await this.invokePoll(child, [], null, assertRetained, true);
      }
      return "unconfirmed";
    } catch {
      return "unconfirmed";
    } finally {
      if (result !== null) wipeBrowserActorValue(result);
    }
  }

  close(): void {
    if (this.closed) return;
    this.closed = true;
    this.state.docAbort.signal.removeEventListener("abort", this.retire);
    this.lease.retirement.removeEventListener("abort", this.retire);
    if (this.pendingUiPatch !== null) {
      const pending = this.pendingUiPatch;
      this.pendingUiPatch = null;
      clearTimeout(pending.timer);
      pending.reject(new Error("document browser actor: closed with pending patch"));
    }
    const child = this.child,
      lifetime = this.lifetime;
    this.coldOwner = null;
    this.coldApplied = null;
    this.renderedUiPatch = false;
    this.renderedSurfaceRevisions.clear();
    this.acknowledgedUiRevision = 0;
    this.mountedUiRevision = 0;
    this.renderedViewState = null;
    this.lastActionSequence = 0;
    this.pendingBackboneBeforeBinding = [];
    this.pendingBackboneBeforeBindingBytes = 0;
    this.documentBackboneReady = false;
    if (this.state.browserActorReservation === this) this.state.browserActorReservation = null;
    if (child === null || lifetime === null) {
      const outcome = child === null ? "retired" as const : "unconfirmed" as const;
      this.abort.abort();
      child?.close(outcome === "retired" ? "retired" : "retirement unconfirmed");
      this.child = null;
      this.lifetime = null;
      this.documentBinding = null;
      this.retirement = Promise.resolve(outcome);
      return;
    }
    const bindingRetirement = this.documentBinding?.port.retire() ?? Promise.resolve();
    this.retirement = bindingRetirement.then(() => this.enqueueTurn(() => this.retireGuest(child, lifetime), true), () => "unconfirmed" as const).then(outcome => {
      this.abort.abort();
      child?.close(outcome === "retired" ? "retired" : "retirement unconfirmed");
      this.child = null;
      this.lifetime = null;
      this.documentBinding = null;
      return outcome;
    });
  }
}
