/** 🔄️ Flow session task tracking and payload delivery: shared payload digests and coalescing, scene to session synchronisation, camera startup and refit.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | NodeGraph}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { serializeCanvasThemeJson } from "@semio-tech/ui-styling";
import { type NodeGraphHover, type NodeGraphScene, type AppCatalogue } from "@semio-tech/framework";
import { parseSceneJsonField } from "../../🗣️Interpreter/🌉️scene-host-contract/🟦️.ts";
import { type FlowTask, type FlowWasmSession } from "../../🪪️WasmSessionLoader/🟦️.tsx";
import { DEFAULT_NODE_GRAPH_VIEWPORT } from "../🧰️graph-core/🟦️.ts";
import {
  DAG_CONTENT_REFIT_MAX_COVERAGE,
  type DagCameraState,
  dagContentBounds,
  dagContentCoverage,
  dagFitCamera,
  dagStartupCamera
} from "../🖼️dag-overlays/🟦️.tsx";

//#region 🔖️NodeGraphHost
//#region 🔖️flow-graph-canvas-host
//#region Sync
// 🎥️ The camera is NEVER copied from `scene.viewport` on a resync: live pan/zoom lives in the
// FlowWasmSession (and plugin runtime via `nodeGraphViewport`), while `scene.viewport` often lags.
// Applying it on hover/eval/edit-triggered synchronization would snap the camera; document
// preserves the live camera so fixture content reloads never reset the view. The ONE moment a
// stored camera is considered at all is the surface's first attach, and even there it is a
// decision, not a copy — see `applyFlowStartupCamera`.
const activeFlowTasks = new WeakMap<FlowWasmSession, Map<string, FlowTask<unknown>>>();

export function observeFlowTask<T>(session: FlowWasmSession, feature: string, task: FlowTask<T>, consume?: (value: T) => void, settled?: (delivered: boolean) => void): () => void {
  let features = activeFlowTasks.get(session);
  if (!features) {
    features = new Map();
    activeFlowTasks.set(session, features);
  }
  features.get(feature)?.cancel();
  features.set(feature, task as FlowTask<unknown>);
  const unsubscribe = task.subscribe(() => {});
  let delivered = false;
  void task.result
    .then((value) => {
      delivered = true;
      consume?.(value);
    })
    .catch(() => {})
    .finally(() => {
      unsubscribe();
      settled?.(delivered);
      if (features?.get(feature) === task) features.delete(feature);
    });
  return () => {
    unsubscribe();
    task.cancel();
    if (features?.get(feature) === task) features.delete(feature);
  };
}

/** 🧊️ Whether an overlay read produced the same picture as the last pass. Every pass re-parses its
 * JSON into fresh objects, so a plain `setState` would change the reference 60 times a second during
 * a gesture and commit the surface's whole React subtree for a picture that did not move. */
export function sameOverlayValue(left: unknown, right: unknown): boolean {
  return left === right || JSON.stringify(left ?? null) === JSON.stringify(right ?? null);
}

/** 🖱️ Issues one gesture STEP — a wheel tick, a pointer down/move/up, a slider write — and never
 * pre-empts the previous one.
 *
 * `observeFlowTask` keeps one task per feature key and cancels the previous, which is right for a
 * query whose answer is superseded and wrong for an input event: a cancelled `wheelScreen` that had
 * not reached the session yet is a zoom tick the board NEVER applies, so a fast scroll silently
 * loses travel, and a cancelled `pointerMoveScreen` is a drag step the gesture never sees. A step is
 * an increment on board state; it has no successor that could carry its delta.
 *
 * The session's own operation order is the serialization, so steps stay in the order the user made
 * them; a closed session rejects them all and the whole surface is going away anyway. */
export function issueFlowGestureStep<T>(task: FlowTask<T>, consume?: (value: T) => void): void {
  const unsubscribe = task.subscribe(() => {});
  void task.result
    .then((value) => consume?.(value))
    .catch(() => {})
    .finally(unsubscribe);
}

export async function readFlowTask<T>(task: FlowTask<T>): Promise<T> {
  const unsubscribe = task.subscribe(() => {});
  try {
    return await task.result;
  } finally {
    unsubscribe();
  }
}

export async function readObservedFlowTask<T>(session: FlowWasmSession, feature: string, task: FlowTask<T>): Promise<T> {
  observeFlowTask(session, feature, task);
  return task.result;
}

/** 📄️ Payloads a session has already been handed, and the ones being handed to it right now.
 *
 * `observeFlowTask` keeps ONE task per feature key and cancels the previous — right for a query whose
 * answer is superseded, fatal for a payload the guest has to APPLY. The scene changes on every
 * `flowEvalTick` (measured roughly 700 ms apart on a live graph), so an unguarded re-issue cancelled
 * the document sync and the operator-kind table before the guest ever applied them: the session kept
 * its default three-widget fixture and its kind-less operator names for the whole session, while the
 * scene had carried seven nodes since the first frame. The same pre-emption defect the paint lane
 * fixed for `renderCanvas`, on the state half of the sync.
 *
 * An identical payload is therefore never re-sent — and a send that did NOT deliver clears the
 * record, so a cancelled or failed send is retried by the next scene pass rather than lost. */
const flowSessionDeliveredPayloads = new WeakMap<FlowWasmSession, Map<string, string>>();
const flowSessionSendingPayloads = new WeakMap<FlowWasmSession, Map<string, string>>();

function flowSessionPayloadRecord(map: WeakMap<FlowWasmSession, Map<string, string>>, session: FlowWasmSession): Map<string, string> {
  let record = map.get(session);
  if (!record) {
    record = new Map();
    map.set(session, record);
  }
  return record;
}

//#region 📦️SharedFlowPayloads
/** 📦️ Marks a content-addressed flow payload. `@<digest>\n<body>` CARRIES a body and registers it in
 * the guest process under that digest; `@<digest>` alone REFERENCES one the guest already holds.
 * Twin of `FLOW_SHARED_PAYLOAD_PREFIX` / `resolve_flow_shared_payload` in `🌊️flow/🖥️host/🦀️.rs`. */
const FLOW_SHARED_PAYLOAD_REFERENCE_PREFIX = "@";

/** 🧩️ Separates the PARTS of one composed payload. The operator table is the app-static catalogue
 * plus whatever records the scene derived; carried part by part, appending 1 805 B of scene operators
 * names the 98 642 B of app operators the guest already holds instead of re-crossing them. Twin of
 * `FLOW_SHARED_PAYLOAD_PART_SEPARATOR`. */
const FLOW_SHARED_PAYLOAD_PART_SEPARATOR = "";

/** 🗂️ Digests this page has confirmed the guest PROCESS holds.
 *
 * Module-scoped, not per session, because every flow session in a page lives in ONE wasm module with
 * one linear memory, while the app-static catalogue is the same bytes for all of them: the operator
 * kind infos are 88 438 B and the palette sections 22 849 B, and both crossed the ABI again for every
 * board that attached (`📓️flow-scroll-render-perf-2026-09-15.md` §9). A catalogue GENERATION that
 * really changed hashes differently and therefore carries its body; a second surface on the same
 * generation names it. */
const flowSharedPayloadDigests = new Set<string>();

/** 🔢 A content address for a flow payload — FNV-1a over the body in two independent lanes, tagged
 * with its length. Not a security digest: the guest treats it as an opaque key and answers
 * `unknown shared payload` when it does not hold it, which puts the body back on the wire. */
function flowContentDigest(body: string): string {
  let low = 0x811c9dc5;
  let high = 0x01000193;
  for (let index = 0; index < body.length; index += 1) {
    const code = body.charCodeAt(index);
    low = Math.imul(low ^ code, 0x01000193) >>> 0;
    high = Math.imul(high ^ (code + index), 0x85ebca6b) >>> 0;
  }
  return `${body.length.toString(36)}.${low.toString(36)}.${high.toString(36)}`;
}

/** ⏲️ How long an app-static payload waits for a better version of itself before it crosses.
 *
 * The app catalogue arrives in STAGES: the operator table crossed at 98 644 B and again 8 ms later at
 * 100 449 B, because a later refresh pass carried more operators than the first
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). Content addressing cannot help — the two bodies really
 * are different — so the payload waits out the stage instead. */
export const FLOW_SHARED_PAYLOAD_COALESCE_MS = 24;

/** 🔌️ What the coalescer needs from its host, injected so a law drives the real rule over a virtual
 * clock instead of a browser. */
export type FlowSharedPayloadCoalescerPorts = Readonly<{
  readonly send: (feature: string, parts: readonly string[]) => void;
  readonly schedule: (run: () => void, delayMs: number) => unknown;
  readonly cancel: (handle: unknown) => void;
}>;

/** 📦️ Holds an app-static payload for one settle window so a catalogue that arrives in stages crosses
 * ONCE, carrying its final content, instead of once per stage. Each feature settles on its own clock:
 * a new offer for the same feature replaces the pending one and restarts its window. */
export function createFlowSharedPayloadCoalescer(ports: FlowSharedPayloadCoalescerPorts, settleMs: number = FLOW_SHARED_PAYLOAD_COALESCE_MS) {
  const pending = new Map<string, readonly string[]>();
  const timers = new Map<string, unknown>();
  const flush = (feature: string) => {
    timers.delete(feature);
    const parts = pending.get(feature);
    if (!parts) return;
    pending.delete(feature);
    ports.send(feature, parts);
  };
  return {
    offer(feature: string, parts: readonly string[]): void {
      pending.set(feature, parts);
      const running = timers.get(feature);
      if (running !== undefined) ports.cancel(running);
      timers.set(feature, ports.schedule(() => flush(feature), settleMs));
    },
    pendingFeatures(): readonly string[] {
      return [...pending.keys()];
    },
    dispose(): void {
      for (const handle of timers.values()) ports.cancel(handle);
      timers.clear();
      pending.clear();
    },
  };
}

const flowSessionSharedCoalescers = new WeakMap<FlowWasmSession, ReturnType<typeof createFlowSharedPayloadCoalescer>>();
const flowSessionSharedIssues = new WeakMap<FlowWasmSession, Map<string, (payload: string) => FlowTask<unknown>>>();
//#endregion 📦️SharedFlowPayloads

/** 📤️ Sends `payload` under `feature` exactly once per distinct value — see the docstring above.
 *
 * A `shared` payload is app-static and identical for every session in the page, so it crosses the ABI
 * as a content-addressed reference once the guest process has confirmed the body. A reference the
 * guest cannot resolve fails the send, which both clears the digest and clears this session's
 * delivery record, so the next scene pass carries the body again. */
function sendFlowPayloadOnce(session: FlowWasmSession, feature: string, payload: string, issue: (payload: string) => FlowTask<unknown>): void {
  crossFlowPayload(session, feature, [payload], issue, false);
}

/** 📦️ Sends a payload composed of parts, each content-addressed: a part the guest process already
 * holds crosses as its digest alone, and only the parts it has never seen carry their bytes.
 *
 * This is where the app-static operator catalogue stops re-crossing. The table is built from the
 * app catalogue and the scene's own derived records, and the two arrive a few milliseconds apart, so
 * the whole 98 642 B table used to cross twice — once for the app half, once to append 1 805 B of
 * scene records. A part the guest cannot resolve fails the send, which clears both the digest and
 * this session's delivery record so the next scene pass carries the bytes again. */
function sendFlowPayloadPartsOnce(session: FlowWasmSession, feature: string, parts: readonly string[], issue: (payload: string) => FlowTask<unknown>): void {
  let issues = flowSessionSharedIssues.get(session);
  if (!issues) {
    issues = new Map();
    flowSessionSharedIssues.set(session, issues);
  }
  issues.set(feature, issue);
  let coalescer = flowSessionSharedCoalescers.get(session);
  if (!coalescer) {
    coalescer = createFlowSharedPayloadCoalescer({
      send: (sentFeature, sentParts) => {
        const sentIssue = flowSessionSharedIssues.get(session)?.get(sentFeature);
        if (sentIssue) crossFlowPayload(session, sentFeature, sentParts, sentIssue, true);
      },
      schedule: (run, delayMs) => setTimeout(run, delayMs),
      cancel: (handle) => clearTimeout(handle as ReturnType<typeof setTimeout>),
    });
    flowSessionSharedCoalescers.set(session, coalescer);
  }
  coalescer.offer(feature, parts);
}

/** 📮️ Puts one composed payload on the wire, naming every part the guest process already holds. */
function crossFlowPayload(session: FlowWasmSession, feature: string, parts: readonly string[], issue: (payload: string) => FlowTask<unknown>, shared: boolean): void {
  const delivered = flowSessionPayloadRecord(flowSessionDeliveredPayloads, session);
  const sending = flowSessionPayloadRecord(flowSessionSendingPayloads, session);
  const body = parts.join(FLOW_SHARED_PAYLOAD_PART_SEPARATOR);
  if (delivered.get(feature) === body || sending.get(feature) === body) return;
  sending.set(feature, body);
  const digests = shared ? parts.map(flowContentDigest) : [];
  const named = digests.filter((digest) => flowSharedPayloadDigests.has(digest)).length;
  const wire = shared
    ? parts
        .map((part, index) => (flowSharedPayloadDigests.has(digests[index]!) ? `${FLOW_SHARED_PAYLOAD_REFERENCE_PREFIX}${digests[index]}` : `${FLOW_SHARED_PAYLOAD_REFERENCE_PREFIX}${digests[index]}\n${part}`))
        .join(FLOW_SHARED_PAYLOAD_PART_SEPARATOR)
    : body;
  observeFlowTask(session, feature, issue(wire), undefined, (landed) => {
    if (sending.get(feature) !== body) return;
    sending.delete(feature);
    if (landed) {
      delivered.set(feature, body);
      for (const digest of digests) flowSharedPayloadDigests.add(digest);
    } else {
      delivered.delete(feature);
      for (const digest of digests) flowSharedPayloadDigests.delete(digest);
    }
  });
}

export function cancelFlowTasks(session: FlowWasmSession): void {
  for (const task of activeFlowTasks.get(session)?.values() ?? []) task.cancel();
  activeFlowTasks.delete(session);
  flowSessionSharedCoalescers.get(session)?.dispose();
  flowSessionSharedCoalescers.delete(session);
  flowSessionSharedIssues.delete(session);
}

export function syncFlowCanvasTheme(session: FlowWasmSession): void {
  observeFlowTask(session, "setCanvasThemeJson", session.setCanvasThemeJson(serializeCanvasThemeJson()));
}

function applyNodeGraphHoverFromScene(session: FlowWasmSession, hover: NodeGraphHover | undefined): void {
  if (hover === undefined) return;
  if (hover.nodeId && hover.portId) {
    observeFlowTask(session, "setHoverChannel", session.setHoverChannel(hover.nodeId, hover.portId));
  } else {
    observeFlowTask(session, "setHover", session.setHover(hover.nodeId ?? null));
  }
}

function syncFlowSessionEvalFromScene(session: FlowWasmSession, scene: NodeGraphScene): void {
  if (scene.evalJson) observeFlowTask(session, "applyEvalOutputsJson", session.applyEvalOutputsJson(scene.evalJson));
  if (scene.statusJson) observeFlowTask(session, "setNodeStatuses", session.setNodeStatuses(scene.statusJson));
  else if (scene.computingJson) observeFlowTask(session, "setComputingProgress", session.setComputingProgress(scene.computingJson));
}

/** 🔌️ The operator kind infos a flow session lays node ports out from: the app-static registered
 * catalogue first, then whatever DOCUMENT-derived records this particular scene carries (the OS
 * workflow window derives one per workflow node). `setNeuronKindInfosJson` replaces the session's whole
 * table, so the two sources are always pushed together, never one after the other. */
function syncFlowOperatorInfos(session: FlowWasmSession, catalogue: AppCatalogue, scene: NodeGraphScene): void {
  // 🧩️ The two sources are carried as two PARTS, not concatenated into one body: they arrive
  // milliseconds apart, so a single body made the whole app-static table cross again just to append
  // the scene's own records.
  const parts = [catalogue.operators ?? [], scene.operators ?? []].filter((source) => source.length > 0).map((source) => JSON.stringify(source));
  if (parts.length === 0) return;
  sendFlowPayloadPartsOnce(session, "setNeuronKindInfosJson", parts, (json) => session.setNeuronKindInfosJson(json));
}

/** 🛍️ Installs the app-static catalogue on a flow session: the operator kind infos the canvas lays
 * ports out from, and the palette sections its spotlight ranks. Its own pass, run once per app instance
 * rather than per scene sync — see {@link AppCatalogueContext}. */
export function syncFlowSessionAppCatalogue(session: FlowWasmSession, catalogue: AppCatalogue, scene: NodeGraphScene): void {
  syncFlowOperatorInfos(session, catalogue, scene);
  if (catalogue.sections) sendFlowPayloadPartsOnce(session, "setCatalogueJson", [JSON.stringify(catalogue.sections)], (json) => session.setCatalogueJson(json));
}

export function syncFlowSessionStructureFromScene(session: FlowWasmSession, scene: NodeGraphScene, catalogue: AppCatalogue, skipFixture = false): void {
  if (scene.operators?.length) syncFlowOperatorInfos(session, catalogue, scene);
  if (!skipFixture && scene.hostSnapshotJson) sendFlowPayloadOnce(session, "synchronizeSnapshotJson", scene.hostSnapshotJson, (json) => session.synchronizeSnapshotJson(json));
  if (scene.selection) observeFlowTask(session, "setSelection", session.setSelection(JSON.stringify({nodes:scene.selection,edges:[],handles:[]})));
  applyNodeGraphHoverFromScene(session, scene.hover);
  if (scene.previewOffJson) observeFlowTask(session, "setPreviewOff", session.setPreviewOff(scene.previewOffJson));
  if (scene.lodJson) {
    try {
      const lod = parseSceneJsonField<{ readonly automatic?: boolean; readonly forcedLabel?: string }>(scene.lodJson);
      observeFlowTask(session, "setAutomaticLod", session.setAutomaticLod(lod.automatic !== false));
      if (lod.forcedLabel) observeFlowTask(session, "setForcedDrawLodLabel", session.setForcedDrawLodLabel(lod.forcedLabel));
    } catch {
      /* ignore */
    }
  }
}

/** 🖼️ The opening camera of a node-graph surface: the stored one when it already frames this graph,
 * the fit otherwise. Returns the camera it installed together with whether the fit won — the caller
 * persists a fit exactly the way it persists a pan or a zoom gesture, so the next open honours it.
 *
 * Twin of `DagHost::adopt_camera_or_fit`; the law itself is {@link dagStartupCamera}, pinned by
 * `🧰️framework/🔨️modules/🖼️canvas/🧫️fixtures/📷️camera-fit/🔣️.json`. */
export function applyFlowStartupCamera(session: FlowWasmSession, scene: NodeGraphScene, width: number, height: number): { readonly camera: DagCameraState; readonly fitted: boolean } {
  const stored = scene.viewport ?? DEFAULT_NODE_GRAPH_VIEWPORT;
  const decision = dagStartupCamera(stored, dagContentBounds(scene.nodes), width, height);
  observeFlowTask(session, "setCamera", session.setCamera(decision.camera.x, decision.camera.y, decision.camera.zoom));
  return decision;
}

/** 🔀️ A graph that CHANGED under a live camera — an example switch — is re-framed only when the
 * change left essentially nothing on screen. Twin of `DagHost::refit_camera_if_content_left_view`. */
export function refitFlowCameraIfContentLeftView(session: FlowWasmSession, scene: NodeGraphScene, camera: DagCameraState, width: number, height: number): DagCameraState | null {
  const content = dagContentBounds(scene.nodes);
  if (!content || dagContentCoverage(content, camera, width, height) > DAG_CONTENT_REFIT_MAX_COVERAGE) return null;
  const fitted = dagFitCamera(content, width, height);
  observeFlowTask(session, "setCamera", session.setCamera(fitted.x, fitted.y, fitted.zoom));
  return fitted;
}

export function syncFlowSessionFromScene(session: FlowWasmSession, scene: NodeGraphScene, catalogue: AppCatalogue): void {
  syncFlowSessionStructureFromScene(session, scene, catalogue);
  // 🧵️ Apply results from the plugin's off-main-thread `flowEvalTick` chain BEFORE computingJson —
  // applyEvalOutputsJson clears computing chrome, so applying computingJson first would have it
  // immediately wiped by this call on the same sync pass.
  syncFlowSessionEvalFromScene(session, scene);
}
//#endregion Sync
//#endregion 🔖️flow-graph-canvas-host
//#endregion 🔖️NodeGraphHost
