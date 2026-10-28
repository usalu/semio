import type { ReservationSession } from "../🗃️state/🟦️.ts";
import type { ReservationHub } from "../🧩️hub/🟦️.ts";
import { documentBackboneEffectV1 } from "@semio-tech/framework/backbone/binding";
import { type BrowserActorCommandBackboneEnvelopeV1, type BrowserActorCommandPublicationV1, encodeBrowserActorHostEffectV1, decodeBrowserActorIntentPublicationV1, decodeBrowserActorCommandPublicationV1, decodeBrowserActorUnsolicitedPublicationV1, requireBrowserActorCommandBackboneProjectionV1 } from "@semio-tech/framework/browser-bundle/action-handoff/publication";
import type { BrowserActorChildValue } from "@semio-tech/framework-plugin/browser-bundle/child";
import { wireSpawnJob, typedOperationResult, typedOperationPageAnswerV1 } from "@semio-tech/framework-actor/wire-turn";
import { faultDisplayMessage, decodePackValue, parseDocumentBackboneMessage } from "../../../../../🟦️";
import { BROWSER_ACTOR_ACTION_MUTATION_MAXIMUM, parseBrowserActorHostEffectBytesV1 } from "@semio-tech/framework/browser-bundle/action-handoff";
import { HISTORY_TRANSITION_DIFF_SCHEMA } from "@semio-tech/framework-replication";
import { type BrowserActorActionPublication, type BrowserActorSpawnedJob, browserActorBytes, browserActorRecord, browserActorTurnResult } from "../../🧩️actor-wire-helpers/🟦️.ts";

declare module "../🧩️hub/🟦️.ts" {
  interface ReservationCollaborators { effects: ReservationTurnEffects }
}

/** 📤️ Routes the WIT effects of one guest turn to the backbone port, the typed-operation and publication decoders and the job driver. */
export class ReservationTurnEffects {
  private readonly session: ReservationSession;
  constructor(private readonly hub: ReservationHub) { this.session = hub.session; }

  /** 🧭️ Routes one guest turn's WIT effects: backbone messages to the hub port, Shell messages to the typed-operation and
   * publication decoders, spawned jobs to the job driver, and only the closed host-intent set to the Shell. A
   * `publish-event` is the guest's own pub/sub publication and has no Shell consumer — the local shard lane's
   * `wireEffectToFriendly` yields nothing for it either — so it is neither forwarded nor fatal (a committed checkpoint
   * publishes one, and treating it as a foreign host effect closed the document child on every auto check-in:
   * `unsupported host effect publish-event`, hub 7800, ticket 26/09/23 C10). An app command's invocation projects the
   * OPERATION envelopes it sent; history transitions (undo, redo, checkpoint commit — `HISTORY_TRANSITION_DIFF_SCHEMA`)
   * are framework records no invocation projects, so a checkpoint's transition is routed without a projection. */
  async routeTurnEffects(value: BrowserActorChildValue, mode: "ordinary" | "control" | BrowserActorActionPublication = "ordinary", receiveReply?: (payload: Uint8Array) => boolean): Promise<Readonly<{ receipts: readonly Uint8Array[]; mutations: number; publications: number; refusals: readonly string[]; hostEffects: readonly (readonly number[])[]; historyPatches: readonly (readonly number[])[]; jobs: readonly BrowserActorSpawnedJob[]; acknowledgements: readonly BrowserActorChildValue[] }>> {
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
      if (effect.tag === "link-resolve") { this.hub.links.schedule(effect.val); continue; }
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
        if (receiveReply?.(payload)) continue;
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
            this.session.lastEphemeral = publication.snapshot;
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
          if (publication.kind === "ephemeral") this.session.lastEphemeral = publication.snapshot;
          if (publication.kind === "completion" || publication.kind === "operation-completed") {
            if (publication.historyPatch !== null) historyPatches.push(publication.historyPatch);
            if (publication.kind === "completion") completions.push(publication);
          }
        }
        continue;
      }
      if (mode === "control") throw new Error("actor-document-control.data-before-receipt");
      if (target.tag !== "backbone" || typeof target.val !== "string") throw new Error("document browser actor: unsupported message target");
      if (!this.session.documentBackboneReady && !this.session.backboneDraining) throw new Error("actor-document-port.not-live");
      const binding = this.session.documentBinding;
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
      const binding = this.session.documentBinding;
      if (binding === null || !binding.port.send(binding.port.uri, message.payload)) throw new Error("actor-document-port.retired");
    }
    await this.session.localEffectTail;
    if (this.session.localEffectFailure !== null) throw this.session.localEffectFailure;
    return { receipts, mutations, publications, refusals, hostEffects: retainedHostEffects, historyPatches, jobs, acknowledgements };
  }
}
