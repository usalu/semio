import type { ReservationSession } from "../🗃️state/🟦️.ts";
import type { ReservationHub } from "../🧩️hub/🟦️.ts";
import type { ActorInstanceLifetime } from "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts";
import { type BrowserActorUiPatchOfferV1, type BrowserActorUiPatchResultV1, captureBrowserActorUiPatchV1, browserActorUiPatchOwnerMatchesV1 } from "@semio-tech/framework/browser-bundle/patch-handoff";
import type { BrowserActorChildValue } from "@semio-tech/framework-plugin/browser-bundle/child";
import { decodePackWire, packWireNatural } from "../../../../../🟦️";
import { BROWSER_ACTOR_ACTION_MUTATION_MAXIMUM, parseBrowserActorHostEffectBytesV1, parseBrowserActorHistoryPatchBytesV1 } from "@semio-tech/framework/browser-bundle/action-handoff";
import { BROWSER_ACTOR_CHILD_LIMITS } from "@semio-tech/framework/browser-bundle/child/schema";
import { encodeActorUiPatchReceipt } from "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🩹️patch/🟦️.ts";
import { type BrowserActorActionPublication, type BrowserActorSpawnedJob, DOCUMENT_BROWSER_ACTOR_RENDER_TURN_LIMIT, type DocumentBrowserActorChild, browserActorColdStatus, browserActorRecord, browserActorTurnResult, browserActorUiPatchReceipt, hubBinding, wipeBrowserActorValue } from "../../🧩️actor-wire-helpers/🟦️.ts";
import { post } from "../../🧾️document-state/🟦️.ts";
import { emitExecutionTargetStatus } from "../../📥️execution-target-read/🟦️.ts";

declare module "../🧩️hub/🟦️.ts" {
  interface ReservationCollaborators { patches: ReservationUiPatches }
}

/** 🩹️ Offers the guest's UI patches to the main thread, feeds the verdicts back and drives a turn to its settled result. */
export class ReservationUiPatches {
  private readonly session: ReservationSession;
  constructor(private readonly hub: ReservationHub) { this.session = hub.session; }

  captureUiPatch(value: BrowserActorChildValue, lifetime: ActorInstanceLifetime) {
    const result = browserActorTurnResult(value);
    return captureBrowserActorUiPatchV1(result.uiPatches, browserActorUiPatchReceipt(result.uiPatchReceipt), lifetime, this.session.surfaceKeys, { decodePack: decodePackWire, natural: packWireNatural });
  }

  /** 🩻️ Names a failure the reservation resolves by itself. `refreshHostView`'s catch reports only
   * while the reservation is still open, so a deadline that closes the child before its rejection
   * propagates left the shell on "Verifying document component…" forever with no reason anywhere. */
  private reportExecutionTargetFault(diagnostic: string): void {
    if (this.session.closed) return;
    const binding = hubBinding(this.session.artifact.config);
    if (binding) emitExecutionTargetStatus(this.session.artifact, binding, "integrity-failed", undefined, diagnostic);
  }

  private awaitUiPatchResult(offer: BrowserActorUiPatchOfferV1): Promise<BrowserActorUiPatchResultV1> {
    if (this.session.pendingUiPatch !== null) return Promise.reject(new Error("document browser actor: patch result already pending"));
    let resolve!: (result: BrowserActorUiPatchResultV1) => void, reject!: (error: Error) => void;
    const result = new Promise<BrowserActorUiPatchResultV1>((resolvePatch, rejectPatch) => {
      resolve = resolvePatch;
      reject = rejectPatch;
    });
    const timer = setTimeout(
      () => {
        if (this.session.pendingUiPatch?.offer !== offer) return;
        this.session.pendingUiPatch = null;
        this.reportExecutionTargetFault("document browser actor: patch result deadline");
        reject(new Error("document browser actor: patch result deadline"));
        this.hub.retirement.close();
      },
      Math.min(15_000, BROWSER_ACTOR_CHILD_LIMITS.invokeMs),
    );
    // ⏳️ `settled` lets `awaitUiQuiescence` wait for this offer; its own no-op catch keeps an unobserved deadline
    // from surfacing as an unhandled rejection while awaiters still see the rejection.
    const settled = result.then(() => undefined);
    settled.catch(() => {});
    this.session.pendingUiPatch = { offer, resolve, reject, timer, settled };
    post({ ...offer, clientInstanceId: this.session.artifact.openClientInstanceId });
    return result;
  }

  settleUiPatch(result: BrowserActorUiPatchResultV1): void {
    const pending = this.session.pendingUiPatch;
    if (pending === null || !browserActorUiPatchOwnerMatchesV1(pending.offer, result)) return;
    clearTimeout(pending.timer);
    this.session.pendingUiPatch = null;
    pending.resolve(result);
  }

  async reconcileUiPatches(
    initial: BrowserActorChildValue,
    child: DocumentBrowserActorChild,
    assertCurrent: () => void,
    publication: BrowserActorActionPublication | null = null,
  ): Promise<Readonly<{ moreWork: boolean; mutations: number }>> {
    const lifetime = this.session.lifetime;
    if (lifetime === null) throw new Error("document browser actor: missing patch lifetime");
    let value: BrowserActorChildValue | null = initial;
    let mutations = 0;
    let lastRejection: string | null = null;
    const jobs: BrowserActorSpawnedJob[] = [];
    const acknowledgements: BrowserActorChildValue[] = [];
    try {
      for (let patchCount = 0; patchCount < 8; patchCount += 1) {
        const routed = await this.hub.effects.routeTurnEffects(value, publication ?? "ordinary");
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
          const completions = await this.hub.lanes.driveSpawnedJobs(child, jobs.splice(0), assertCurrent);
          wipeBrowserActorValue(value);
          value = null;
          value = await this.hub.lanes.invokePoll(child, [...acknowledgements.splice(0), ...completions], null, assertCurrent);
          continue;
        }
        if (captured === null) return { moreWork: browserActorRecord(browserActorTurnResult(value).status, "document browser actor: invalid render status").tag === "more-work", mutations };
        const fields = this.session.lease.fields();
        const offer: BrowserActorUiPatchOfferV1 = {
          kind: "browser-actor-ui-patch",
          scope: { ...fields.scope },
          verifiedSurfaceId: fields.surface.surfaceId,
          activationGeneration: this.session.generation.toString(),
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
          if (verdict.outcome === "acknowledged") this.session.renderedSurfaceRevisions.set(verdict.surface, verdict.revision);
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
        const windowVerdict = result.verdicts.find((verdict) => verdict.surface === this.session.windowKindId);
        if (windowVerdict?.outcome === "acknowledged") this.session.renderedUiPatch = true;
        value = await this.hub.lanes.invokePoll(child, [...feedback, ...acknowledgements.splice(0)], null, assertCurrent);
        assertCurrent();
        if (browserActorColdStatus(value).kind !== "idle") throw new Error("document browser actor: cold ingress after patch feedback");
        if (windowVerdict?.outcome === "acknowledged") this.session.acknowledgedUiRevision = windowVerdict.revision;
      }
      throw new Error(`document browser actor: patch feedback limit${lastRejection === null ? "" : ` (last rejection: ${lastRejection})`}`);
    } finally {
      if (value !== null) wipeBrowserActorValue(value);
    }
  }

  async driveTurnResult(
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
          this.hub.surfaces.publishMountedIfReady();
          return mutations;
        }
        value = await this.hub.lanes.invokePoll(child, [{ tag: "wake" }], null, assertCurrent);
      }
      throw new Error("document browser actor: turn continuation limit");
    } finally {
      if (value !== null) wipeBrowserActorValue(value);
    }
  }
}
