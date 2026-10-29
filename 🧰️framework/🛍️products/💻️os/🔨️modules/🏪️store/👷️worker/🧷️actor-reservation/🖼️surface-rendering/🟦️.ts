import type { ReservationSession } from "../🗃️state/🟦️.ts";
import type { ReservationHub } from "../🧩️hub/🟦️.ts";
import type { ActorInstanceLifetime } from "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts";
import { type ResolvedPluginViewState, windowViewContext, panelViewContext, sectionViewContext } from "@semio-tech/framework-manifest";
import type { BrowserActorChildValue } from "@semio-tech/framework-plugin/browser-bundle/child";
import { encodePackValue } from "../../../../../🟦️";
import { type BrowserActorVisibleTurnV1, browserActorVisibleSurfacesV1 } from "../../🪟️visible-surfaces/🟦️.ts";
import { DOCUMENT_BROWSER_ACTOR_RENDER_TURN_LIMIT, type DocumentBrowserActorChild, browserActorColdStatus, hubBinding, wipeBrowserActorValue } from "../../🧩️actor-wire-helpers/🟦️.ts";
import { documentServices, post } from "../../🧾️document-state/🟦️.ts";
import { documentBrowserActorLease } from "../../🎭️actor-lifecycle/🟦️.ts";
import { emitExecutionTargetStatus } from "../../📥️execution-target-read/🟦️.ts";

declare module "../🧩️hub/🟦️.ts" {
  interface ReservationCollaborators { surfaces: ReservationSurfaceRendering }
}

/** 🖼️ Renders and refreshes the verified windows, panels and sections of the child and publishes the mounted state. */
export class ReservationSurfaceRendering {
  private readonly session: ReservationSession;
  constructor(private readonly hub: ReservationHub) { this.session = hub.session; }

  publishMountedIfReady(): void {
    if (!this.session.renderedUiPatch || this.session.renderedUiRevision < 1 || this.session.acknowledgedUiRevision !== this.session.renderedUiRevision || this.session.mountedUiRevision !== 0 || this.session.documentBinding === null || !this.session.documentBackboneReady) return;
    const cold = this.session.coldApplied;
    if (cold === null) return;
    cold.assertCurrent();
    const identity = this.session.lease.fields(),
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
      clientInstanceId: this.session.artifact.openClientInstanceId,
      activationGeneration: this.session.generation.toString(),
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
      uiRevision: this.session.renderedUiRevision,
    });
    this.session.mountedUiRevision = this.session.renderedUiRevision;
  }

  refreshHostView(): void {
    if (this.session.closed || !this.session.activation || this.session.viewRefresh) return;
    const activation = this.session.activation;
    this.session.viewRefresh = (async () => {
      await activation;
      await this.session.coldTransfer;
      await this.hub.lanes.enqueueTurn(async () => {
        const child = this.session.child;
        if (!child || !this.session.coldApplied) return;
        const assertCurrent = () => {
          if (this.session.closed || !this.hub.link.linkCurrent() || this.session.artifact.browserActorReservation !== this.hub.shell || documentBrowserActorLease(this.session.artifact) !== this.session.lease) throw new Error("document browser actor: stale host view");
          this.session.lease.assertBrowserActorCurrent();
        };
        while (this.session.artifact.browserActorViewState !== null && this.session.artifact.browserActorViewState !== this.session.renderedViewState) {
          assertCurrent();
          await this.renderSurface(child, assertCurrent);
        }
      });
    })().catch((error: unknown) => {
      if (this.session.closed) return;
      const binding = hubBinding(this.session.artifact.config);
      if (binding) emitExecutionTargetStatus(this.session.artifact, binding, "renderer-unavailable", undefined, error instanceof Error ? error.message : String(error));
      this.hub.retirement.close();
    }).finally(() => { this.session.viewRefresh = null; });
  }

  bindApprovalUndoIfMounted(): void {
    const owner = this.session.coldApplied;
    if (owner !== null && this.session.renderedUiPatch) documentServices.documentMounted({ state: this.session.artifact, reservation: this.hub.shell, pair: owner });
  }

  /** 🖼️ Makes every verified window (each in its own window context), every verified panel body (panel context) and every
   * reserved refresh section (section context; the static catalogue on the lifetime's first paint only) visible to the
   * child, exactly as the local refresh binds them, and reconciles the turn's patches until the render settles. Rendering
   * the panels here is what keeps an actor-bound document's inspector live (G-P1-4); rendering every window is what lets a
   * second window of the document show and command the live document; rendering the sections is what keeps its window
   * chrome and tool measures live (C13 P1). */
  async renderSurface(child: DocumentBrowserActorChild, assertCurrent: () => void): Promise<void> {
    const lifetime = this.session.lifetime;
    if (lifetime === null) throw new Error("document browser actor: missing render lifetime");
    assertCurrent();
    const hostView = this.session.artifact.browserActorViewState;
    if (hostView === null) throw new Error("document browser actor: missing host view context");
    const owed = this.visibleEvents(lifetime, hostView, this.session.renderedViewState === null ? "mount" : "repaint");
    for (let turn = 0; turn < DOCUMENT_BROWSER_ACTOR_RENDER_TURN_LIMIT; turn += 1) {
      assertCurrent();
      const events: BrowserActorChildValue[] = owed.length !== 0 ? owed.splice(0, 1) : turn === 0 ? [] : [{ tag: "wake" }];
      let result: BrowserActorChildValue | null = await this.hub.lanes.invokePoll(child, events, null, assertCurrent);
      try {
        assertCurrent();
        if (browserActorColdStatus(result).kind !== "idle") throw new Error("document browser actor: unexpected cold ingress during render");
        const owned = result;
        result = null;
        if (!(await this.hub.patches.reconcileUiPatches(owned, child, assertCurrent)).moreWork && owed.length === 0) {
          this.session.renderedViewState = hostView;
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
    return browserActorVisibleSurfacesV1(this.session.lease.renderSurfaces(), turn).map(({ surface, bodyKey, context }): BrowserActorChildValue => {
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
  async refreshDocumentSurfaces(child: DocumentBrowserActorChild, assertCurrent: () => void, windows: boolean): Promise<void> {
    const lifetime = this.session.lifetime,
      hostView = this.session.renderedViewState;
    if (lifetime === null || hostView === null) return;
    for (const event of this.visibleEvents(lifetime, hostView, windows ? "repaint" : "refresh")) {
      const result = await this.hub.lanes.invokePoll(child, [event], null, assertCurrent);
      if (browserActorColdStatus(result).kind !== "idle") {
        wipeBrowserActorValue(result);
        throw new Error("document browser actor: unexpected cold ingress during surface refresh");
      }
      if ((await this.hub.patches.driveTurnResult(result, child, assertCurrent)) !== 0) throw new Error("document browser actor: surface refresh mutated the document");
    }
  }
}
