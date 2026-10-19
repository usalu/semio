/** 🧺️ Phase 4 of `loadPluginModule`: retained-UI patch intake, surface projection and the instance close ladder of one plugin handle.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | PluginRuntime}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { type PluginUiRefreshRequest, type PluginUiRefreshResponse, type PluginUiRefreshSectionResponse, type UiRefreshSectionKey } from "@semio-tech/framework";
import { type BuiltNode } from "@semio-tech/framework";
import { DEFAULT_UI_DOCUMENT_LIMITS } from "../../📃️UiDocumentStore/🟦️.tsx";
import { OwnedUiPatchIntake } from "../../📃️UiDocumentStore/📥️intake/🟦️.ts";
import { type OwnedUiPatchAcknowledgementEntry } from "@semio-tech/framework-actor/shard-client/ui-patch";
import { type ShardInstanceLifecycleLease } from "@semio-tech/framework-actor/shard-client/activation";
import { OwnedUiInstance, type OwnedUiInstanceRetirement, type OwnedUiInstanceSurface, type OwnedUiPatchAcknowledgement } from "../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts";
import type { RetainedUiNodeRecord } from "../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts";
import { hostContinuations } from "@semio-tech/framework/async/continuation";
import { hopTrace } from "@semio-tech/framework/trace";
import { retainedSurfaceId, wirePatchSurfaceId, type WireTurnResult, type WireVariant } from "../🪝️wire-codec/🟦️.ts";
import { ownedUiComponentToBuilt } from "../🪟️retained-ui-patch/🟦️.ts";
import { serializePerActor, submitPluginLifecycleTurn } from "../🧵️turn-dispatch/🟦️.ts";
import {
  createRetainedUiCloseLadderV1,
  PLUGIN_UI_CONTINUATION_LIMIT,
  PLUGIN_UI_INTAKE_STEP_CEILING,
  type PluginPatchAcceptance,
  uiIntakeOwesYieldV1,
  uiPatchAdmissionRoundsV1,
  yieldPluginUiContinuation
} from "../🔁️turn-settle/🟦️.ts";
import { retainedUiRefreshEffects, sectionValueFromBuiltNode, uiRefreshSectionTargets, uiRefreshSectionUnchanged } from "../🖼️surface-refresh/🟦️.ts";
import { createInstanceTable, type PluginHandleScope } from "../🪢️handle-scope/🟦️.ts";

//#region 🔖️plugin-runtime
/** 🧺️ Phase 4a of {@link loadPluginModule}: retained-UI patch admission — the per-instance intake lane, the round-wise
 * admission and acknowledgement of a turn's patch page, surface installation and publication announcements. */
function createUiPatchAdmission(scope: PluginHandleScope, instances: ReturnType<typeof createInstanceTable>) {
  const { pluginId, lifecycleByInstance, uiOwnerByInstance, uiSurfaceByInstance, uiIntakesByInstance } = scope;
  const { requireActorId } = instances;
  /** 🩺️ When each retained surface last became CURRENT — the instant its newest patch finished
   * installing. The projection that first carries that surface onward closes `patch.paint` against it,
   * which is what makes "the guest published it" → "the shell holds it" a measured interval rather
   * than an inference from whichever refresh happened to be in flight. */
  const uiSurfaceInstalledAtMs = new Map<string, number>();
  /** 📣️ Listeners for {@link announceSurfacePublications} — the host's own "this surface is current
   * now" door, answered by a projection-only pass that crosses to no guest. */
  const surfacePublicationListeners = new Set<(instanceId: number, surfaceIds: readonly string[]) => void>();
  /** 📣️ Says that a patch has finished installing and its surface is now CURRENT in the host's own
   * retained store.
   *
   * 🐛️ Without this door a published surface reached the DOM only when the shell NEXT asked for a
   * refresh, because the projection that paints it is request-driven and the patch that makes it
   * current arrives on whatever turn the guest happened to finish it on — a drain turn, a job
   * completion, a later continuation of somebody else's settle. A peer measured the consequence on a
   * live tool run (ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS phase 8): the shell asked every
   * 100–400 ms, the world patch cost ~30 ms of intake, and it landed 250–800 ms after the request
   * that wanted it — about one scene update per second. The publication is the event; the refresh
   * request is not. Announced AFTER the whole batch is installed and dispatched through the host's one
   * {@link hostContinuations} scheduler, so a listener can never re-enter the turn loop that is still
   * running. */
  const announceSurfacePublications = (instanceId: number, surfaceIds: readonly string[]): void => {
    if (surfaceIds.length === 0 || surfacePublicationListeners.size === 0) return;
    const announced = [...surfaceIds];
    hostContinuations.schedule(() => {
      for (const listener of surfacePublicationListeners) {
        try {
          listener(instanceId, announced);
        } catch (error) {
        }
      }
    }, 0);
  };
  /** 🎟️ Per-step intake slice. One item per step made the 4 096-step continuation budget a hard
   * 4 096-item ceiling on a single surface patch (a 180-object world scene exhausts it); 256 items /
   * 64 KiB per step keeps each slice sub-millisecond while the budget bounds a patch at ~1 M items. */
  const uiGrant = Object.freeze({ maxItems: 256, maxBytes: 65_536 });
  /** 🚦 The instance's ONE owning intake. Every path that accepts UI patches for an instance — the
   * `settlePluginTurn` accept callback of a command turn, the owed/drain settle, the ShellHost progress
   * subscription's refresh, a job completion, the hot-swap close — enters through here, so a patch
   * offered while another is open WAITS instead of meeting a busy surface cell. Keyed by the
   * plugin-qualified instance id (never reused) so a hot swap that replaces the handle keeps the same
   * queue, and backed by the file's own bounded {@link serializePerActor} rather than a second
   * mechanism. The recursion over each round's acknowledgement turn deliberately does NOT re-enter:
   * it is already inside this queue's one admission. */
  const serializeInstanceUiIntake = <T,>(instanceId: number, run: () => Promise<T>): Promise<T> => serializePerActor(`ui-intake:${pluginId}#${instanceId}`, run);
  const closeIntake = async (instanceId: number, intake: OwnedUiPatchIntake): Promise<void> => {
    intake.beginClose();
    for (let step = 1; !intake.terminalIsEmpty(); step += 1) {
      if (step > PLUGIN_UI_CONTINUATION_LIMIT) throw new Error("plugin-ui.intake-close-budget-exhausted");
      const current = intake.closeStep(uiGrant);
      if (current.kind === "blocked" || current.kind === "rejected") throw new Error(`plugin-ui.intake-close-${current.kind}:${current.phase}`);
      if (uiIntakeOwesYieldV1(step)) await yieldPluginUiContinuation();
    }
    uiIntakesByInstance.get(instanceId)?.delete(intake);
  };
  type AdmittedUiPatch = { readonly surfaceId: string; readonly intake: OwnedUiPatchIntake; readonly entry: OwnedUiPatchAcknowledgementEntry; readonly startedMs: number };
  /** 🧺️ Admits one round's patches, acknowledges the WHOLE round in one crossing, installs each surface
   * and closes its intake — the shape the patch-batching lane bought, now applied per round instead of
   * per turn. `submitUiAcknowledgements` admits any non-empty SUBSET of a turn's patches (it is the same
   * path the one-patch case took before batching), so a round is a legal acknowledgement unit. */
  const installUiPatchRound = async (instanceId: number, lease: ShardInstanceLifecycleLease, admitted: readonly AdmittedUiPatch[]): Promise<WireTurnResult> => {
    const acknowledged = await submitPluginLifecycleTurn(lease, { kind: "issued-ui-acks", entries: admitted.map(({ entry }) => entry) }, "Interactive");
    if (!acknowledged.submissions || acknowledged.submissions.length !== admitted.length) throw new Error("plugin-ui.acknowledgement-refused");
    const installed: string[] = [];
    for (const [index, { surfaceId, intake, startedMs }] of admitted.entries()) {
      if (!intake.acceptAcknowledgement(acknowledged.submissions[index]!)) throw new Error("plugin-ui.acknowledgement-refused");
      for (let step = 1; ; step += 1) {
        if (step > PLUGIN_UI_INTAKE_STEP_CEILING) throw new Error(`plugin-ui.publication-close-budget-exhausted:${surfaceId}:${PLUGIN_UI_INTAKE_STEP_CEILING}`);
        const current = intake.advance(uiGrant);
        if (current.kind === "ready") break;
        if (current.kind === "blocked" || current.kind === "rejected") throw new Error(`plugin-ui.intake-${current.kind}:${current.phase}`);
        if (uiIntakeOwesYieldV1(step)) await yieldPluginUiContinuation();
      }
      const surface = intake.takeSurface();
      if (!surface) throw new Error("plugin-ui.surface-missing");
      const surfaces = uiSurfaceByInstance.get(instanceId) ?? new Map<string, OwnedUiInstanceSurface>();
      surfaces.set(surfaceId, surface); uiSurfaceByInstance.set(instanceId, surfaces);
      const installedAtMs = performance.now();
      uiSurfaceInstalledAtMs.set(`${instanceId}:${surfaceId}`, installedAtMs);
      hopTrace.record("patch.install", startedMs, installedAtMs - startedMs, { instanceId, surfaceId });
      installed.push(surfaceId);
      await closeIntake(instanceId, intake);
    }
    announceSurfacePublications(instanceId, installed);
    return acknowledged.turn;
  };
  /** 🧯️ Retires patches this round already admitted when a LATER one in the same round failed: their
   * acknowledgement still crosses (a subset is a legal batch) so every cell they hold is released. One
   * admission fault can therefore never leave a surface with an open wire, input page or receipt outbox —
   * the state that made `beginPatch` answer busy for the rest of the instance's life. */
  const retireAdmittedUiPatches = async (instanceId: number, lease: ShardInstanceLifecycleLease, admitted: readonly AdmittedUiPatch[]): Promise<void> => {
    if (admitted.length === 0) return;
    try {
      await installUiPatchRound(instanceId, lease, admitted);
    } catch (error) {
      console.error(`plugin-ui.intake-retirement-failed: instance ${instanceId} could not retire ${admitted.length} admitted patch(es) (${admitted.map(({ surfaceId }) => surfaceId).join(", ")})`, error);
    }
  };
  const acceptUiPatches = async (instanceId: number, turn: WireTurnResult): Promise<PluginPatchAcceptance> => {
    if (turn.uiPatches.length === 0) return { acknowledgements: [], turns: [] };
    return serializeInstanceUiIntake(instanceId, () => acceptOwnedUiPatches(instanceId, turn));
  };
  /** 📥️ Installs one turn's whole patch page. Runs INSIDE {@link serializeInstanceUiIntake}, so it —
   * and its own recursion over each round's acknowledgement turn — is the instance's single intake. */
  const acceptOwnedUiPatches = async (instanceId: number, turn: WireTurnResult): Promise<PluginPatchAcceptance> => {
    if (turn.uiPatches.length === 0) return { acknowledgements: [], turns: [] };
    requireActorId(instanceId);
    const lease = lifecycleByInstance.get(instanceId);
    const owner = uiOwnerByInstance.get(instanceId);
    if (!lease || !owner || !turn.original || !turn.uiPatchReceipt) throw new Error("plugin-ui.native-owner-required");
    const surfaceIds = turn.uiPatches.map(wirePatchSurfaceId);
    const supplemental: WireTurnResult[] = [];
    for (const round of uiPatchAdmissionRoundsV1(surfaceIds)) {
      const admitted: AdmittedUiPatch[] = [];
      let pending: OwnedUiPatchIntake | null = null;
      try {
        for (const index of round) {
          const surfaceId = surfaceIds[index];
          if (!surfaceId) throw new Error("plugin-ui.projection-surface-required");
          const source = lease.captureUiPatchAuthority(turn.original, index);
          const startedMs = performance.now();
          const intake = new OwnedUiPatchIntake(owner, source);
          pending = intake;
          const intakes = uiIntakesByInstance.get(instanceId) ?? new Set<OwnedUiPatchIntake>();
          intakes.add(intake); uiIntakesByInstance.set(instanceId, intakes);
          let token: OwnedUiPatchAcknowledgement | null = null;
          for (let step = 1; token === null; step += 1) {
            if (step > PLUGIN_UI_INTAKE_STEP_CEILING) throw new Error(`plugin-ui.intake-budget-exhausted:${surfaceId}:${PLUGIN_UI_INTAKE_STEP_CEILING}`);
            const current = intake.advance(uiGrant);
            token = intake.peekAcknowledgement();
            if (current.kind === "rejected") throw new Error(`plugin-ui.intake-rejected:${current.phase}:${intake.failure ?? "unknown"}`);
            if (current.kind === "blocked" && token === null) throw new Error(`plugin-ui.intake-blocked:${current.phase}`);
            if (uiIntakeOwesYieldV1(step)) await yieldPluginUiContinuation();
          }
          admitted.push({ surfaceId, intake, entry: { source, token }, startedMs });
          pending = null;
        }
      } catch (error) {
        if (pending) await closeIntake(instanceId, pending).catch((closeError) => console.error(`plugin-ui.intake-retirement-failed: instance ${instanceId} could not close the intake that faulted`, closeError));
        await retireAdmittedUiPatches(instanceId, lease, admitted);
        throw error;
      }
      const acknowledgedTurn = await installUiPatchRound(instanceId, lease, admitted);
      supplemental.push(acknowledgedTurn);
      const nested = await acceptOwnedUiPatches(instanceId, acknowledgedTurn);
      supplemental.push(...nested.turns);
      if (nested.acknowledgements.length > 0) throw new Error("plugin-ui.unexpected-generic-acknowledgement");
    }
    return { acknowledgements: [], turns: supplemental };
  };
  return { uiGrant, uiSurfaceInstalledAtMs, serializeInstanceUiIntake, closeIntake, acceptUiPatches };
}

/** 🧺️ Phase 4b of {@link loadPluginModule}: reads the retained surfaces back as built nodes and answers a refresh request. */
function createUiSurfaceProjection(scope: PluginHandleScope, admission: ReturnType<typeof createUiPatchAdmission>) {
  const { state, actorIdByInstance, closingInstances, lifecycleByInstance, uiOwnerByInstance, uiSurfaceByInstance, uiReadsByInstance, turnOutcomes } = scope;
  const { uiGrant, uiSurfaceInstalledAtMs } = admission;
  const advanceUiMaintenance = async (owner: OwnedUiInstance, budget: { steps: number }, phase: string): Promise<void> => {
    while (owner.maintenancePending) {
      if (++budget.steps > DEFAULT_UI_DOCUMENT_LIMITS.maxNodes * 64) throw new Error(`plugin-ui.${phase}-budget-exhausted`);
      const current = owner.advanceMaintenance(uiGrant);
      if (current.kind === "blocked" || current.kind === "rejected") throw new Error(`plugin-ui.${phase}-${current.kind}:${current.phase}`);
      if (uiIntakeOwesYieldV1(budget.steps)) await yieldPluginUiContinuation();
    }
  };
  const projectOwnedUiSurface = async (instanceId: number, actorId: string, lease: ShardInstanceLifecycleLease, owner: OwnedUiInstance, surface: OwnedUiInstanceSurface): Promise<{ readonly hash: string; readonly value: BuiltNode } | null> => {
    const current = () => !state.disposing && !closingInstances.has(instanceId) && actorIdByInstance.get(instanceId) === actorId && lifecycleByInstance.get(instanceId) === lease && uiOwnerByInstance.get(instanceId) === owner;
    const view = surface.view;
    if (view.root === null) return null;
    if (!view.hash) throw new Error("plugin-ui.surface-hash-required");
    const visited = new Set<number>();
    const budget = { steps: 0 };
    const build = async (id: number, depth: number): Promise<BuiltNode> => {
      if (!current()) throw new Error("plugin-ui.read-stale");
      if (depth > DEFAULT_UI_DOCUMENT_LIMITS.maxDepth || visited.size >= DEFAULT_UI_DOCUMENT_LIMITS.maxNodes || visited.has(id)) throw new Error("plugin-ui.read-graph-invalid");
      visited.add(id);
      const subscription = surface.subscribeNode(id, () => {});
      let record: RetainedUiNodeRecord | null = null;
      try {
        await advanceUiMaintenance(owner, budget, "read");
        const snapshot = subscription.snapshot;
        if (!snapshot || snapshot.version !== view.revision || !snapshot.record) throw new Error("plugin-ui.read-snapshot-missing");
        record = snapshot.record;
        surface.acknowledgeRead(subscription, snapshot);
      } finally {
        surface.unsubscribeNode(subscription);
        await advanceUiMaintenance(owner, budget, "read-retirement");
      }
      if (!record) throw new Error("plugin-ui.read-snapshot-missing");
      const children: BuiltNode[] = [];
      for (const child of record.children) children.push(await build(child, depth + 1));
      return { key: record.key, component: ownedUiComponentToBuilt(record.component), layout: record.layout, style: record.style, activity: record.activity, disabled: record.disabled, accessibility: record.accessibility, bindings: record.bindings, menu: record.menu, children };
    };
    const value = await build(view.root, 1);
    if (!current() || surface.view !== view) throw new Error("plugin-ui.read-stale");
    return { hash: view.hash, value };
  };
  const ownedUiRefreshResponse = async (instanceId: number, actorId: string, request: PluginUiRefreshRequest, effects: readonly WireVariant[]): Promise<PluginUiRefreshResponse> => {
    const lease = lifecycleByInstance.get(instanceId);
    const owner = uiOwnerByInstance.get(instanceId);
    const surfaces = uiSurfaceByInstance.get(instanceId);
    if (!lease || !owner || !surfaces) throw new Error("plugin-ui.native-owner-required");
    const notePaint = (surfaceId: string, key: string): void => {
      const installedAtMs = uiSurfaceInstalledAtMs.get(`${instanceId}:${surfaceId}`);
      if (installedAtMs === undefined) return;
      uiSurfaceInstalledAtMs.delete(`${instanceId}:${surfaceId}`);
      hopTrace.record("patch.paint", installedAtMs, performance.now() - installedAtMs, { instanceId, surfaceId, key });
    };
    const project = async (targets: PluginUiRefreshRequest["windows"]): Promise<PluginUiRefreshSectionResponse[]> => {
      const result: PluginUiRefreshSectionResponse[] = [];
      for (const target of targets ?? []) {
        const surface = surfaces.get(retainedSurfaceId(instanceId, target.key));
        if (surface && uiRefreshSectionUnchanged(target.hash, surface.view)) {
          result.push({ key: target.key, hash: target.hash! });
          continue;
        }
        // 🧯️ A requested body that has no retained surface at all, or one the guest has not rooted yet,
        // is omitted from the response — and the shell's own merge keeps whatever it had (its loading
        // placeholder, on a first refresh). That is indistinguishable from a healthy refresh, so it is
        // recorded: one console record per dropped body, permanent (not a `[TRACE]` trace), the same
        // way `AppRouter` records a plugin it excluded. A panel that renders forever empty in the
        // browser (measured 2026-09-09 21:05 on `framework.panel.inspection`) is either named here or
        // is a real guest render — no third possibility.
        if (!surface) {
          console.error(`refreshUi dropped requested body ${JSON.stringify(target.key)}: no retained surface on instance ${instanceId}`);
          continue;
        }
        const projected = await projectOwnedUiSurface(instanceId, actorId, lease, owner, surface);
        if (projected) { notePaint(retainedSurfaceId(instanceId, target.key), target.key); result.push({ key: target.key, ...projected }); }
        else console.error(`refreshUi dropped requested body ${JSON.stringify(target.key)}: retained surface has no root on instance ${instanceId}`);
      }
      return result;
    };
    const projectSections = async (): Promise<{ [key in UiRefreshSectionKey]?: PluginUiRefreshSectionResponse }> => {
      const sections: { [key in UiRefreshSectionKey]?: PluginUiRefreshSectionResponse } = {};
      for (const section of uiRefreshSectionTargets(request)) {
        const surface = surfaces.get(retainedSurfaceId(instanceId, section.bodyKey));
        if (!surface) continue;
        const cached = request[section.key]?.hash;
        if (uiRefreshSectionUnchanged(cached, surface.view)) {
          sections[section.key] = { key: section.key, hash: cached! };
          continue;
        }
        const projected = await projectOwnedUiSurface(instanceId, actorId, lease, owner, surface);
        if (projected) { notePaint(retainedSurfaceId(instanceId, section.bodyKey), section.key); sections[section.key] = { key: section.key, hash: projected.hash, value: sectionValueFromBuiltNode(section.bodyKey, projected.value, `${actorId} instance ${instanceId}`) }; }
      }
      return sections;
    };
    const read = (async () => ({ windows: await project(request.windows), panels: await project(request.panels), ...(await projectSections()) }))();
    const reads = uiReadsByInstance.get(instanceId) ?? new Set<Promise<unknown>>();
    reads.add(read); uiReadsByInstance.set(instanceId, reads);
    try {
      const projected = await read;
      return { ...projected, requestedEffects: retainedUiRefreshEffects(instanceId, effects, (frames) => turnOutcomes.push({ instanceId, frames })) };
    } finally {
      reads.delete(read);
    }
  };
  return { ownedUiRefreshResponse };
}

/** 🧺️ Phase 4c of {@link loadPluginModule}: retires one instance — its UI owner behind the intake queue, then its lifecycle lease. */
function createUiInstanceClose(scope: PluginHandleScope, admission: ReturnType<typeof createUiPatchAdmission>) {
  const { uiRetirementByInstance, uiSurfaceByInstance, uiIntakesByInstance, uiReadsByInstance } = scope;
  const { uiGrant, serializeInstanceUiIntake, closeIntake } = admission;
  const closeUiOwner = (instanceId: number, owner: OwnedUiInstance): Promise<OwnedUiInstanceRetirement> => serializeInstanceUiIntake(instanceId, () => closeOwnedUiInstance(instanceId, owner));
  /** 🧹 Runs inside {@link serializeInstanceUiIntake}: a hot swap can never tear an instance's surfaces
   * down while one of its patches is mid-admission. */
  const closeOwnedUiInstance = async (instanceId: number, owner: OwnedUiInstance): Promise<OwnedUiInstanceRetirement> => {
    const retained = uiRetirementByInstance.get(instanceId);
    if (retained) return retained;
    if ((uiReadsByInstance.get(instanceId)?.size ?? 0) !== 0) throw new Error("plugin-ui.read-retirement-pending");
    for (const intake of uiIntakesByInstance.get(instanceId) ?? []) await closeIntake(instanceId, intake);
    const retainedSurfaces = uiSurfaceByInstance.get(instanceId)?.size ?? 0;
    uiSurfaceByInstance.get(instanceId)?.clear();
    const ladder = createRetainedUiCloseLadderV1(retainedSurfaces);
    owner.beginClose();
    let steps = 0;
    const phaseCensus = new Map<string, number>();
    for (let step = 1; !owner.terminalIsEmpty(); step += 1) {
      steps = step;
      const current = owner.closeStep(uiGrant);
      if (current.kind === "blocked" || current.kind === "rejected") throw new Error(`plugin-ui.owner-close-${current.kind}:${current.phase}`);
      phaseCensus.set(current.phase, (phaseCensus.get(current.phase) ?? 0) + 1);
      const fault = ladder.admit(current);
      if (fault) throw new Error(fault);
      if (uiIntakeOwesYieldV1(step)) await yieldPluginUiContinuation();
    }
    const witness = owner.takeRetirementWitness();
    if (!witness) throw new Error("plugin-ui.retirement-witness-missing");
    uiRetirementByInstance.set(instanceId, witness);
    return witness;
  };
  const assertClosingTurn = (turn: WireTurnResult): void => {
    if (turn.uiPatches.length > 0 || turn.uiPatchReceipt !== undefined) throw new Error("plugin-ui.patch-after-lifecycle-close");
  };
  const retireInstanceLifecycle = async (instanceId: number, lease: ShardInstanceLifecycleLease, owner: OwnedUiInstance): Promise<void> => {
    const captured = lease.pendingReceipt;
    if (captured?.kind === "captured") {
      const acknowledged = await submitPluginLifecycleTurn(lease, { kind: "receipt-ack", receipt: captured }, "Interactive");
      assertClosingTurn(acknowledged.turn);
    }
    lease.beginClose();
    for (let step = 1; lease.progress().kind !== "complete"; step += 1) {
      if (step > PLUGIN_UI_CONTINUATION_LIMIT) throw new Error("plugin-ui.lifecycle-close-budget-exhausted");
      const receipt = lease.pendingReceipt;
      if (receipt?.kind === "accepted") {
        const acknowledged = await submitPluginLifecycleTurn(lease, { kind: "receipt-ack", receipt }, "Interactive");
        assertClosingTurn(acknowledged.turn);
      } else if (receipt?.kind === "retired") {
        const retirement = await closeUiOwner(instanceId, owner);
        const acknowledged = await submitPluginLifecycleTurn(lease, { kind: "receipt-ack", receipt, retirement }, "Interactive");
        assertClosingTurn(acknowledged.turn);
      } else {
        const progress = lease.progress();
        if (progress.kind === "blocked") throw new Error(`plugin-ui.lifecycle-close-blocked:${progress.failure ?? "unknown"}`);
        const current = progress.kind === "closing"
          ? await submitPluginLifecycleTurn(lease, { kind: "close" }, "Interactive")
          : await submitPluginLifecycleTurn(lease, { kind: "poll" }, "Interactive");
        assertClosingTurn(current.turn);
      }
      if (uiIntakeOwesYieldV1(step)) await yieldPluginUiContinuation();
    }
    lease.dispose();
  };
  return { retireInstanceLifecycle };
}
//#endregion 🔖️plugin-runtime

/** 🧺️ Phase 4 of {@link loadPluginModule}: retained-UI patch intake, projection and close — every path that accepts,
 * reads or retires an instance's retained surfaces. */
export function createRetainedUiIntake(scope: PluginHandleScope, instances: ReturnType<typeof createInstanceTable>) {
  const admission = createUiPatchAdmission(scope, instances);
  const { ownedUiRefreshResponse } = createUiSurfaceProjection(scope, admission);
  const { retireInstanceLifecycle } = createUiInstanceClose(scope, admission);
  return { acceptUiPatches: admission.acceptUiPatches, ownedUiRefreshResponse, retireInstanceLifecycle };
}
