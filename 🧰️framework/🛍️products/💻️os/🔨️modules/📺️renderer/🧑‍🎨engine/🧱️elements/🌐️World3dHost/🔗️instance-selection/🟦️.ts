import type { InteractionTarget } from "@semio-tech/framework";
/** 🔗️ Instance, selection and leftover-overlay parsing and residency, interaction and LOD parsing, context-menu targets, raycast helpers and the world interaction domain.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | World3dHost}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { leftoverWorldGumballPoseV1 } from "../../🛠️ShellHelpers/🔩️plumbing/🎮️leftover-views/🟦️.ts";
import { parseJsonArray } from "../../🗣️Interpreter/🌉️scene-host-contract/🟦️.ts";
import {
  type World3dMarkerInteractionFields,
  type WorldEngagementPreviewItem,
  type WorldInstanceRecord,
  type WorldInteractionRecord,
  type WorldLodRecord,
  type WorldMeshRecord,
  type WorldSelectionRecord
} from "../🔭️scene-schema/🟦️.ts";


//#region 🔖️World3dHost
//#region WorldSceneParsing
function parseInstances(instancesJson: string): WorldInstanceRecord[] {
  try {
    const parsed = JSON.parse(instancesJson);
    return Array.isArray(parsed) ? (parsed as WorldInstanceRecord[]) : [];
  } catch {
    return [];
  }
}

const WORLD_INSTANCE_RECORD_TEXT = new WeakMap<WorldInstanceRecord, string>();

/** 🧾️ A record's serialized JSON, memoized per record object so a retained record is serialized once. */
function worldInstanceRecordText(record: WorldInstanceRecord): string {
  let text = WORLD_INSTANCE_RECORD_TEXT.get(record);
  if (text === undefined) {
    text = JSON.stringify(record);
    WORLD_INSTANCE_RECORD_TEXT.set(record, text);
  }
  return text;
}

/** 🪪️ `parsed`, with every record whose retained twin (same id) serializes identically replaced by that twin. */
function internWorldInstanceRecords(retained: readonly WorldInstanceRecord[] | null, parsed: WorldInstanceRecord[]): WorldInstanceRecord[] {
  if (!retained || retained.length === 0) return parsed;
  const byId = new Map(retained.map((record) => [record.id, record]));
  return parsed.map((record) => {
    const twin = byId.get(record.id);
    return twin !== undefined && worldInstanceRecordText(twin) === worldInstanceRecordText(record) ? twin : record;
  });
}

/** 🚚️ The `instancesDeltaJson` lane's declared shape — see the Rust `World3dScene::instances_delta_json`. */
export type WorldInstanceDeltaV1 = {
  readonly base: number;
  readonly revision: number;
  readonly count: number;
  readonly changed: readonly WorldInstanceRecord[];
  readonly removed: readonly string[];
};

/** 🧾️ One consumer's retained instance set and the delta revision it stands at. */
export type WorldInstanceResidencyV1 = {
  readonly revision: number;
  readonly records: readonly WorldInstanceRecord[];
};

export function parseWorldInstanceDelta(deltaJson: string | null | undefined): WorldInstanceDeltaV1 | null {
  if (!deltaJson) return null;
  try {
    const parsed = JSON.parse(deltaJson) as Partial<WorldInstanceDeltaV1>;
    if (typeof parsed?.revision !== "number" || typeof parsed?.base !== "number" || !Array.isArray(parsed.changed)) return null;
    return { base: parsed.base, revision: parsed.revision, count: typeof parsed.count === "number" ? parsed.count : parsed.changed.length, changed: parsed.changed, removed: Array.isArray(parsed.removed) ? parsed.removed : [] };
  } catch {
    return null;
  }
}

/**
 * 🚚️ Advances one consumer's retained instance set by the publication it just received.
 *
 * ⏱️ Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B44. A pose edit on a large document republishes the
 * whole `instancesJson` lane — 55 KiB / 180 records on Nakagin — and every consumer used to re-parse
 * all of it to learn that ONE instance moved. When the producer also publishes
 * `instancesDeltaJson` and this consumer's retained set is exactly at that delta's `base`, the
 * changed records are substituted BY ID and every untouched record keeps its object identity, so the
 * downstream instanced-mesh memos see only the instances that actually moved.
 *
 * 🧯️ In-place application is deliberately restricted to pure UPDATES of ids already retained.
 * `instancesJson` is index-addressed (`worldPick` resolves a hit by array position, which is why a
 * hidden instance stays in the array at zero scale), so an addition or a removal — anything that can
 * reorder — falls back to the authoritative full parse. Same for a delta whose `base` does not match,
 * a `count` that disagrees with the result, or a producer that publishes no delta at all: the full
 * lane is always correct, so ignoring the delta can only cost time, never correctness.
 *
 * 🪪️ A fallback still keeps the object identity of every retained record whose re-parsed record is
 * value-equal (same serialized JSON). A running fill APPENDS placements, so every one of its refreshes
 * falls back; re-issuing all records re-rendered every instance of the document per refresh, where only
 * the appended ones changed.
 */
export function advanceWorldInstanceResidency(previous: WorldInstanceResidencyV1 | null, instancesJson: string, deltaJson: string | null | undefined): WorldInstanceResidencyV1 {
  const delta = parseWorldInstanceDelta(deltaJson);
  const retained = previous?.records ?? null;
  const applicable =
    delta !== null &&
    retained !== null &&
    previous?.revision === delta.base &&
    delta.removed.length === 0 &&
    delta.count === retained.length &&
    delta.changed.every((record) => retained.some((existing) => existing.id === record.id));
  if (!applicable) return { revision: delta?.revision ?? -1, records: internWorldInstanceRecords(retained, parseInstances(instancesJson)) };
  if (delta.changed.length === 0) return { revision: delta.revision, records: retained };
  const replacements = new Map(delta.changed.map((record) => [record.id, record]));
  return { revision: delta.revision, records: retained.map((existing) => replacements.get(existing.id) ?? existing) };
}

export function parseSelection(selectionJson: string): WorldSelectionRecord {
  try {
    return JSON.parse(selectionJson) as WorldSelectionRecord;
  } catch {
    return { method: "rectangle", ids: [] };
  }
}

export type LeftoverWorldSelectionOverlayV1 = {
  readonly ids: readonly string[];
  readonly hoveredId: string | null;
  readonly hoveredDomain?: string | null;
  readonly gumballActive: boolean;
  readonly gumballAnchorId: string | null;
  readonly activeUtility?: string | null;
  readonly activeToolId?: string | null;
  /** 🧹️ Guest published an explicit emptying pick — the host must not resurrect prior overlay ids. */
  readonly selectionCleared?: boolean;
};

/** 🪟️ The overlay fields ONE pane owns. Everything else in the overlay is the document's, shared by
 * every pane of it: `ids`/`gumball*` are a document selection (B20) and `activeToolId` is a
 * mode-level tool. Hover and the armed window utility belong to the window
 * INSTANCE that resolved them (wave B9), so one pane arming Brush must never read back in another. */
const LEFTOVER_WORLD_WINDOW_FIELDS = ["hoveredId", "hoveredDomain", "activeUtility"] as const;

/** 🪟️ Where ONE leftover publication has authority: a single pane, the document's shared fields only,
 * or every pane at once — the last is what a mode-level tool activation is (it clears every window's
 * utility, `🏛️ShellHost`'s `clearAllWindowUtilities`). */
export type LeftoverWorldOverlayScopeV1 = { readonly kind: "window"; readonly windowId: string } | { readonly kind: "document" } | { readonly kind: "allWindows" };

const leftoverWorldOverlayByWindow = new Map<string, LeftoverWorldSelectionOverlayV1>();
let leftoverWorldDocumentOverlay: LeftoverWorldSelectionOverlayV1 | null = null;
const leftoverWorldSelectionListeners = new Set<() => void>();
/** 🈳️ The window fields of a document slot that no document-wide publication has spoken for: no hover, no armed utility. */
const LEFTOVER_WORLD_NO_WINDOW_FIELDS: LeftoverWorldSelectionOverlayV1 = { ids: [], hoveredId: null, hoveredDomain: null, gumballActive: false, gumballAnchorId: null, activeUtility: null };
const leftoverWorldWindowOverlayMemo = new WeakMap<LeftoverWorldSelectionOverlayV1, { readonly document: LeftoverWorldSelectionOverlayV1 | null; readonly merged: LeftoverWorldSelectionOverlayV1 }>();

/** 🪪️ `document`'s own shared fields with `window`'s per-pane fields laid over them. */
function leftoverWorldOverlayForWindowFieldsV1(document: LeftoverWorldSelectionOverlayV1, window: LeftoverWorldSelectionOverlayV1): LeftoverWorldSelectionOverlayV1 {
  const merged: Record<string, unknown> = { ...document };
  for (const field of LEFTOVER_WORLD_WINDOW_FIELDS) merged[field] = window[field];
  return merged as unknown as LeftoverWorldSelectionOverlayV1;
}

/** 🟰️ Two overlays carry the same fields with the same values (`ids` element-wise): a republished equal overlay is the SAME
 * overlay, so the slot keeps its object and no reader re-renders for it (ticket 26/09/23 F3: every hover republished a fresh
 * `ids` array and re-rendered every pane of the document twice per hover transition). */
function leftoverWorldOverlaySameV1(left: LeftoverWorldSelectionOverlayV1 | null | undefined, right: LeftoverWorldSelectionOverlayV1 | null | undefined): boolean {
  if (left === right) return true;
  if (!left || !right) return false;
  const keys = Object.keys(left) as (keyof LeftoverWorldSelectionOverlayV1)[];
  return (
    keys.length === Object.keys(right).length &&
    keys.every((key) => {
      const [a, b] = [left[key], right[key]];
      return key in right && (a === b || (Array.isArray(a) && Array.isArray(b) && a.length === b.length && a.every((id, index) => id === b[index])));
    })
  );
}

/** 🕹️ Host leftover InteractionView overlay — vite-live until guest scene.selectionJson republishes.
 *
 * A `window` publication owns only that pane's window fields; the document slot takes the
 * publication's shared fields and KEEPS its own window fields, so arming Brush in one pane leaves
 * every other pane exactly as it was (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B39) — also when it is the
 * first publication of all: a pane's hover never seeds the document slot, where every sibling pane without
 * an overlay of its own would paint it and keep it (ticket 26/09/23 F3, puzzle3d MainTop). A slot whose
 * value does not change keeps its object, and a publication that changes no slot notifies nobody. */
export function publishLeftoverWorldSelectionV1(overlay: LeftoverWorldSelectionOverlayV1 | null, scope: LeftoverWorldOverlayScopeV1): void {
  let changed = scope.kind === "allWindows" && leftoverWorldOverlayByWindow.size > 0;
  if (scope.kind === "allWindows") leftoverWorldOverlayByWindow.clear();
  if (scope.kind === "window") {
    if (!overlay) changed = leftoverWorldOverlayByWindow.delete(scope.windowId);
    else if (!leftoverWorldOverlaySameV1(leftoverWorldOverlayByWindow.get(scope.windowId), overlay)) {
      leftoverWorldOverlayByWindow.set(scope.windowId, overlay);
      changed = true;
    }
  }
  const nextDocument = !overlay
    ? null
    : scope.kind === "allWindows" || (scope.kind === "document" && !leftoverWorldDocumentOverlay)
      ? overlay
      : leftoverWorldOverlayForWindowFieldsV1(overlay, leftoverWorldDocumentOverlay ?? LEFTOVER_WORLD_NO_WINDOW_FIELDS);
  if (!leftoverWorldOverlaySameV1(leftoverWorldDocumentOverlay, nextDocument)) {
    leftoverWorldDocumentOverlay = nextDocument;
    changed = true;
  }
  if (!changed) return;
  for (const listener of leftoverWorldSelectionListeners) listener();
}

/** 🗂️ The document-scoped overlay: the shared selection plus the window fields of the last
 * document-wide publication. Readers that are about the DOCUMENT (the Outliner/Inspection tree, the
 * armed mode-level tool) read this; a pane reads {@link leftoverWorldWindowOverlayV1}. */
export function leftoverWorldSelectionOverlayV1(): LeftoverWorldSelectionOverlayV1 | null {
  return leftoverWorldDocumentOverlay;
}

/** 🪟️ What ONE pane's record must be overlaid with: the document's shared fields under this pane's
 * own window fields, falling back to the document-wide ones for a pane that has published none. The
 * answer keeps its object while its value holds, so a pane can subscribe to exactly its own overlay. */
export function leftoverWorldWindowOverlayV1(windowId: string | null | undefined): LeftoverWorldSelectionOverlayV1 | null {
  const own = windowId ? leftoverWorldOverlayByWindow.get(windowId) : undefined;
  if (!own) return leftoverWorldDocumentOverlay;
  const memo = leftoverWorldWindowOverlayMemo.get(own);
  if (memo?.document === leftoverWorldDocumentOverlay) return memo.merged;
  const next = leftoverWorldOverlayForWindowFieldsV1(leftoverWorldDocumentOverlay ?? own, own);
  const merged = memo && leftoverWorldOverlaySameV1(memo.merged, next) ? memo.merged : next;
  leftoverWorldWindowOverlayMemo.set(own, { document: leftoverWorldDocumentOverlay, merged });
  return merged;
}

/** 🖌️ The pane whose own overlay is armed with a brush, for the host's brush-preview refresh lane —
 * an arm lives in exactly one pane now, so "is anything armed" is a search, not a global read. */
export function leftoverWorldArmedWindowOverlayV1(): LeftoverWorldSelectionOverlayV1 | null {
  for (const [windowId, overlay] of leftoverWorldOverlayByWindow) {
    if (leftoverOverlayArmedBrushUtilityV1(overlay.activeUtility)) return leftoverWorldWindowOverlayV1(windowId);
  }
  return leftoverWorldDocumentOverlay;
}

/** 🖌️ The window instance id whose own overlay is armed with a brush, or `undefined`. */
export function leftoverWorldArmedWindowIdV1(): string | undefined {
  for (const [windowId, overlay] of leftoverWorldOverlayByWindow) {
    if (leftoverOverlayArmedBrushUtilityV1(overlay.activeUtility)) return windowId;
  }
  return undefined;
}

/**
 * 🧰️ The armed window utility is host session state owned by `SET_ACTIVE_UTILITY_ACTION_ID`, NOT part of
 * the guest's `InteractionView` leftover — so an `interactionSelect`/`interactionHover`/`setCamera`
 * leftover republish, which carries no `activeUtility` at all, must CARRY the armed one forward instead of
 * replacing the overlay with an utility-less one (that dropped the lane back to the guest's `select` and
 * disarmed Brush on the first pointer move after arming it).
 */
export function leftoverOverlayArmedBrushUtilityV1(utility: string | null | undefined): utility is "brush" | "volumeBrush" {
  return utility === "brush" || utility === "volumeBrush";
}

export function leftoverOverlayCarryingUtilityV1(next: LeftoverWorldSelectionOverlayV1, prior: LeftoverWorldSelectionOverlayV1 | null): LeftoverWorldSelectionOverlayV1 {
  const nextUtility = next.activeUtility === undefined ? prior?.activeUtility ?? null : next.activeUtility;
  const activeUtility = nextUtility === "select" && leftoverOverlayArmedBrushUtilityV1(prior?.activeUtility) ? prior?.activeUtility : nextUtility;
  const carried = nextUtility === next.activeUtility && activeUtility === next.activeUtility ? next : { ...next, activeUtility };
  return carried.activeToolId === undefined ? { ...carried, activeToolId: prior?.activeToolId ?? null } : carried;
}

/** 🕹️ Hover leftover may publish empty `ids` while selection stays live — unless the guest marked an explicit clear. */
export function leftoverOverlayCarryingSelectionV1(next: LeftoverWorldSelectionOverlayV1, prior: LeftoverWorldSelectionOverlayV1 | null, selectionCleared = false): LeftoverWorldSelectionOverlayV1 {
  const carried = leftoverOverlayCarryingUtilityV1(next, prior);
  const utility = leftoverOverlayArmedBrushUtilityV1(prior?.activeUtility) && carried.hoveredId && carried.activeUtility === "select"
    ? { ...carried, activeUtility: prior?.activeUtility }
    : carried;
  const cleared = selectionCleared || utility.selectionCleared === true;
  if (cleared) return { ...utility, ids: [], gumballActive: false, gumballAnchorId: null, selectionCleared: true };
  if (utility.ids.length > 0 || !prior?.ids.length) return utility;
  return {
    ...utility,
    ids: prior.ids,
    gumballActive: utility.gumballActive || prior.gumballActive,
    gumballAnchorId: utility.gumballAnchorId ?? prior.gumballAnchorId ?? prior.ids[0] ?? null,
  };
}

/** 📄️ Outliner/Inspection tree keys are the entity id; leftover.ids never use hoveredId. */
export function leftoverTreeItemSelectedV1(itemId: string, leftoverIds: readonly string[] | undefined): boolean {
  if (!leftoverIds?.length) return false;
  return leftoverIds.some((id) => id === itemId || itemId.endsWith(`/${id}`) || itemId.endsWith(`.${id}`));
}

/** 🕹️ True when a leftover wrongly treats hover as selection (empty background pick must not do this). */
export function leftoverSelectIdsMustNameHoverPickV1(selectedIds: readonly string[] | undefined, hoverId: string | null | undefined): boolean {
  return hoverId !== null && hoverId !== undefined && (selectedIds ?? []).includes(hoverId);
}

export function subscribeLeftoverWorldSelectionV1(listener: () => void): () => void {
  leftoverWorldSelectionListeners.add(listener);
  return () => leftoverWorldSelectionListeners.delete(listener);
}

/** 🗺️ Hover-only leftover still overlays — selectedIds empty is the interactionHover leftover shape.
 *
 * 🧹️ `selectionCleared` is the one overlay whose whole content is an ABSENCE: the guest's own
 * retirement of a selection, published either by an emptying pick or by a document change
 * (`interaction_after_document_change_v1`, `🔌️plugin/🦀️.rs`). Without it in this guard a retirement
 * carrying no ids, no hover and no utility answered "nothing to overlay" and
 * {@link mergeWorldSelectionWithLeftoverV1} returned the pane's base record untouched — so the one
 * publication that exists to REMOVE a selection was the one the host could not act on. */
export function leftoverWorldOverlayAppliesV1(leftover: LeftoverWorldSelectionOverlayV1 | null | undefined): boolean {
  return Boolean(leftover && (leftover.ids.length > 0 || leftover.hoveredId || leftover.activeUtility || leftover.activeToolId || leftover.selectionCleared));
}

/** 👆️ Vortex-domain leftover hover id (`objectId:vortexId`) for hoveredVortexFullId. */
export function leftoverHoveredVortexFullIdV1(leftover: Pick<LeftoverWorldSelectionOverlayV1, "hoveredId" | "hoveredDomain"> | null | undefined): string | undefined {
  const id = leftover?.hoveredId;
  if (!id) return undefined;
  if (leftover.hoveredDomain && leftover.hoveredDomain !== "vortex") return undefined;
  return id.includes(":") ? id : undefined;
}

/** 🖌️ Armed leftover brush must not publish an empty interactionHover — that clears the guest hover
 * map and the next SurfaceVisible / refreshUi re-projects preview=0 over the 252–313 byte lane. */
export function leftoverBrushRetainGuestHoverV1(activeUtility: string | null | undefined, leftover: LeftoverWorldSelectionOverlayV1 | null | undefined): string | undefined {
  return leftoverOverlayArmedBrushUtilityV1(activeUtility ?? leftover?.activeUtility) ? leftoverHoveredVortexFullIdV1(leftover) : undefined;
}

/** 🧹️ The overlay ids this pane's CURRENT document can still be covering.
 *
 * The leftover overlay exists to carry a pick for the one window between the dispatch and the
 * guest's own lane answering — so it is a cover over the document the pick was made in, and nothing
 * beyond it. Nothing retires it: it is written only by the pick route and by a utility arm
 * (`publishLeftoverWorldSelectionV1` callers in `🏛️ShellHost/🟦️.tsx`), and an app command that
 * REPLACES the whole document cannot publish one either — a job-routed command answers through
 * `TypedOperationCompletion`, which carries revision, ui scope and history patch and has no leftover
 * `output` field at all. Measured on the generation3d React playground 2026-09-14: after picking
 * `shell@solid` in `Box Shell Preview`, `data-guest-selection-json` was `[]` for every later example
 * — the guest had pruned — while `data-selection-json` still painted `["shell@solid"]`, and the next
 * plain REPLACE click read back `["fuse@solid","shell@solid"]`.
 *
 * The pane owns the answer without asking anyone: it knows which objects it is drawing. An id no
 * instance of this pane offers — neither as its own `id` nor as the coarser `interactionId` the
 * topology declares nor as the mesh it draws — is covering nothing. A pane with NO instances yet has
 * no membership information and keeps every id, the same rule the guest's own topology pruning uses
 * for a domain it has no entry for. A just-picked id is always among them, because the pick landed
 * on that instance's geometry — so the cover the overlay exists for is never the one dropped. */
export function leftoverWorldOverlayIdsInDocumentV1(ids: readonly string[], instances: readonly WorldInstanceRecord[]): readonly string[] {
  if (!instances.length || !ids.length) return ids;
  const offered = new Set<string>();
  for (const instance of instances) {
    if (instance.id) offered.add(instance.id);
    if (instance.interactionId) offered.add(instance.interactionId);
    if (instance.meshId) offered.add(instance.meshId);
  }
  return ids.filter((id) => offered.has(id) || [...offered].some((prefix) => id.startsWith(`${prefix}.`)));
}

/** 🪪️ The rendered instance id an id names — itself, or the instance whose `interactionId` it is. */
export function worldActiveInstanceIdV1(instances: readonly WorldInstanceRecord[], id: string | null | undefined): string | undefined {
  if (!id) return undefined;
  return instances.find((instance) => instance.id === id)?.id ?? instances.find((instance) => instance.interactionId === id)?.id ?? id;
}

function worldIdsCoverInstanceV1(instances: readonly WorldInstanceRecord[], ids: readonly string[], instanceId: string): boolean {
  const interactionId = instances.find((instance) => instance.id === instanceId)?.interactionId;
  return ids.includes(instanceId) || (interactionId != null && ids.includes(interactionId));
}

export function mergeWorldSelectionWithLeftoverV1(base: WorldSelectionRecord, leftover: LeftoverWorldSelectionOverlayV1 | null, instances: readonly WorldInstanceRecord[] = []): WorldSelectionRecord {
  if (!leftoverWorldOverlayAppliesV1(leftover) || !leftover) return base;
  const covered = leftoverWorldOverlayIdsInDocumentV1(leftover.ids, instances);
  if (covered.length !== leftover.ids.length) leftover = { ...leftover, ids: covered, gumballAnchorId: leftover.gumballAnchorId && covered.includes(leftover.gumballAnchorId) ? leftover.gumballAnchorId : covered[0] ?? null };
  if (!leftoverWorldOverlayAppliesV1(leftover)) return base;
  const pose = leftoverWorldGumballPoseV1(leftover, instances);
  if (leftover.selectionCleared) {
    return {
      ...base,
      ids: [],
      activeObjectId: null,
      hoveredId: leftover.hoveredId ?? base.hoveredId,
      gumballActive: false,
      gumballTarget: undefined,
      transformMode: base.transformMode ?? pose.transformMode,
    };
  }
  return {
    ...base,
    ...(leftover.ids.length > 0 ? { ids: leftover.ids, activeObjectId: worldActiveInstanceIdV1(instances, base.activeObjectId && worldIdsCoverInstanceV1(instances, leftover.ids, base.activeObjectId) ? base.activeObjectId : leftover.ids[0]) } : {}),
    hoveredId: leftover.hoveredId ?? base.hoveredId,
    gumballActive: leftover.gumballActive || Boolean(base.gumballActive),
    gumballTarget: pose.gumballTarget ?? base.gumballTarget,
    transformMode: base.transformMode ?? pose.transformMode,
  };
}

export function mergeWorldInteractionWithLeftoverV1(base: WorldInteractionRecord, leftover: LeftoverWorldSelectionOverlayV1 | null): WorldInteractionRecord {
  const hoveredVortexFullId = leftoverHoveredVortexFullIdV1(leftover);
  const activeUtility = leftover?.activeUtility;
  if (!hoveredVortexFullId && !activeUtility) return base;
  return {
    ...base,
    ...(hoveredVortexFullId ? { hoveredVortexFullId } : {}),
    ...(activeUtility ? { activeUtility } : {}),
  };
}

/** 🔦️ What ONE pane publishes as `data-selection-json` — the state the pane actually PAINTS, after
 * `mergeWorldSelectionWithLeftoverV1` has laid the host's leftover overlay over the guest's own
 * `selectionJson` lane. Until this attribute existed the pane published its geometry, its camera and
 * its interaction record but never its selection: `WorldInteractionRecord` carries neither
 * `selectedIds` nor a hover target (it is the utility/brush record), `data-instances-json` is the
 * guest's geometry cache which deliberately never bakes selection into an instance
 * (`world_instances_geometry_json`), and `data-status-json` is the off-thread COMPUTE status
 * (`{computing,label}`) — so every outside reader of "what is selected in this pane" was reading
 * fields that never held it (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B20 defect 1).
 *
 * 🪟️ Selection is per DOCUMENT and hover is per WINDOW, so `selectedIds` is the same list in every
 * pane of one document while `hoverTarget` is whatever THIS pane resolved; `activeUtility` rides along
 * because it is per window instance too (wave B9). */
export function worldSurfaceSelectionDomV1(selection: WorldSelectionRecord, interaction: WorldInteractionRecord): Record<string, unknown> {
  const hoveredId = selection.hoveredId ?? null;
  return {
    selectedIds: selection.ids ?? [],
    activeObjectId: selection.activeObjectId ?? null,
    targetVolumeIds: selection.targetVolumeIds ?? [],
    referenceSelectedId: selection.referenceSelectedId ?? null,
    hoverTarget: hoveredId ? { domain: hoveredId.includes(":") ? "vortex" : "object", id: hoveredId } : null,
    hoveredVortexFullId: interaction.hoveredVortexFullId ?? null,
    hoveredKindId: selection.hoveredKindId ?? null,
    gumballActive: Boolean(selection.gumballActive),
    gumballTarget: selection.gumballTarget ?? null,
    transformMode: selection.transformMode ?? null,
    activeUtility: interaction.activeUtility ?? "select",
    showEdges: selection.showEdges ?? null,
    selectionMode: selection.selectionMode ?? selection.granularity ?? null,
    componentIds: selection.componentIds ?? [],
    targets: selection.targets ?? null,
    hoveredComponent: selection.hoveredComponent ?? null,
    gumballConfig: selection.gumballConfig ?? null,
  };
}

/** 🛰️ What the GUEST itself sent on this pane's own selection lane, BEFORE
 * {@link mergeWorldSelectionWithLeftoverV1} lays the host's leftover overlay over it.
 *
 * `data-selection-json` publishes the merged result — the state the pane PAINTS — so a guest that
 * renders from an empty interaction and a guest that renders from the picked one are byte-identical
 * from outside as soon as the host's own leftover overlay carries the ids. That is precisely the
 * divergence the selection-scoped defect family lives in (ticket 26/09/02/PUZZLE-3D-END-TO-END wave
 * B23): on wasm #48 the panes painted `selectedIds:["seed-left-001"]` while the guest's Inspection
 * body rendered the empty-document summary in the same window of time, and no attribute anywhere
 * separated "the guest has the pick" from "the host is painting it for the guest".
 *
 * Kept deliberately minimal — ids, hover, gumball — because its ONE job is to be comparable against
 * `data-selection-json`'s same three fields. */
export function worldSurfaceGuestSelectionDomV1(selectionJson: string | undefined): Record<string, unknown> {
  const guest = parseSelection(selectionJson ?? "{}");
  return {
    selectedIds: guest.ids ?? [],
    activeObjectId: guest.activeObjectId ?? null,
    hoveredId: guest.hoveredId ?? null,
    gumballActive: Boolean(guest.gumballActive),
  };
}

export function parseInteraction(interactionJson: string | undefined): WorldInteractionRecord {
  if (!interactionJson) return {};
  try {
    return JSON.parse(interactionJson) as WorldInteractionRecord;
  } catch {
    return {};
  }
}

export function parseLod(lodJson: string | undefined): WorldLodRecord {
  if (!lodJson) return {};
  try {
    return JSON.parse(lodJson) as WorldLodRecord;
  } catch {
    return {};
  }
}

type WorldContextMenuTarget = { readonly kind: "vortex" | "object" | "reference"; readonly id: string };

/** 🖱️ Resolves which entity a plain right-click should select-then-open a menu for, by priority: hovered vortex, object component, reference, then object. */
export function resolveWorldContextMenuTarget(interaction: WorldInteractionRecord, selection: WorldSelectionRecord): WorldContextMenuTarget | null {
  if (interaction.hoveredVortexFullId) return { kind: "vortex", id: interaction.hoveredVortexFullId };
  if (selection.hoveredComponent?.objectId) return { kind: "object", id: selection.hoveredComponent.objectId };
  const hoveredId = selection.hoveredId;
  if (hoveredId?.startsWith("reference:")) return { kind: "reference", id: hoveredId.slice("reference:".length) };
  if (hoveredId) return { kind: "object", id: hoveredId };
  return null;
}

/** 🪧️ The `surface` half of one world right-click's {@link PluginContextMenuRequest}: the entity under
 * the pointer as the single `hits` row, and the painted selection as per-domain `selection` groups. The hit
 * target rides THIS request — an editor reads it straight off `ContextMenuSurfaceTarget.hits` — so no
 * separate target-recording dispatch precedes the menu. `contextMenuAt`, the action that used to carry it,
 * no longer exists in any world-3d app (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM moved the
 * target onto the request); dispatching it anyway only produced an `undeclaredActionDiagnostic` drop on
 * every right-click. */
export function world3dContextMenuSurfaceV1(
  target: WorldContextMenuTarget | null,
  selection: Pick<WorldSelectionRecord, "ids" | "componentIds">,
): { readonly hits: readonly { readonly domain: string; readonly id: string }[]; readonly selection: readonly { readonly domain: string; readonly ids: readonly string[] }[] } {
  const groups: { readonly domain: string; readonly ids: readonly string[] }[] = [];
  if ((selection.ids?.length ?? 0) > 0) groups.push({ domain: "object", ids: [...(selection.ids ?? [])] });
  if ((selection.componentIds?.length ?? 0) > 0) groups.push({ domain: "feature", ids: (selection.componentIds ?? []).map(String) });
  return { hits: target ? [{ domain: target.kind, id: target.id }] : [], selection: groups };
}

/** 🚫️ Instance-mesh picking must be disabled for brush engagements — otherwise a click meant for a vortex marker or a voxel gesture falls through and selects/gumballs the underlying object instead. */
export function worldInstancePickBlocked(activeUtility: string | undefined): boolean {
  return activeUtility === "brush" || activeUtility === "volumeBrush" || activeUtility === "surfaceBrush";
}

/** 🎯️ The instance lane's resolved hover target, retained verbatim for a missed canvas pick.
 * Marker lanes never publish to this lane; target ids remain opaque across every domain. */
export function hoveredInteractionTargetV1(target: InteractionTarget | null | undefined): InteractionTarget | null {
  return target?.id ? target : null;
}

/** 🚫️ `undefined` keeps the default mesh raycast; a no-op replaces it so a blocked instance cannot steal sibling vortex hits. */
export function worldInstanceMeshRaycast(pickEnabled: boolean): (() => null) | undefined {
  return pickEnabled ? undefined : () => null;
}

/** 🚫️ Walks a GLB/instance root and disables mesh raycasts when pick is blocked. */
export function applyWorldInstanceMeshRaycast(
  root: { readonly traverse: (fn: (object: { readonly isMesh?: boolean; raycast: unknown }) => void) => void },
  pickEnabled: boolean,
  meshRaycast: unknown,
): void {
  const raycast = pickEnabled ? meshRaycast : () => null;
  root.traverse((object) => {
    if (!object.isMesh) return;
    object.raycast = raycast;
  });
}

const WORLD_VORTEX_DEFAULT_RADIUS = 0.36;

/** 🎯 Hit-proxy sphere stays `visible` so Three's raycaster does not skip it; radius is at least the published marker radius. */
export function worldVortexHitProxy(radius?: number): { readonly visible: true; readonly radius: number } {
  const published = radius ?? WORLD_VORTEX_DEFAULT_RADIUS;
  return { visible: true, radius: Math.max(published, WORLD_VORTEX_DEFAULT_RADIUS) };
}

/** 🖱️ In brush mode or vertex selection mode, pointer-down on a vortex selects immediately; otherwise a click selects and a drag starts connect. */
export function resolveVortexPointerDownIntent(brushMode: boolean, selectionMode?: string): "select" | "click-or-drag" {
  return brushMode || selectionMode === "vertex" ? "select" : "click-or-drag";
}

export function parseEngagementPreview(engagementPreviewJson: string | undefined): readonly WorldEngagementPreviewItem[] {
  return parseJsonArray<WorldEngagementPreviewItem>(engagementPreviewJson);
}
//#endregion WorldSceneParsing

//#region CatalogueDrop
export function clientPointOverHost(clientX: number, clientY: number, hostRect: DOMRect): boolean {
  return clientX >= hostRect.left && clientX <= hostRect.right && clientY >= hostRect.top && clientY <= hostRect.bottom;
}
//#endregion CatalogueDrop

//#region 🎯️WorldInteractionDomain
/** 🎯️ Granularity a plain world instance hit reports when the scene declares no
 * `domainGranularityId` — matches the node graph's own port granularity, which is what an
 * instance standing for one output channel resolves to. */
export const WORLD3D_DEFAULT_INTERACTION_GRANULARITY = "handle";

/** ✨ Alt+right-click opens suggestions only when a vortex marker is already hovered. */
export function world3dSuggestionsGestureArmed(altKey: boolean, hoveredVortexFullId: string | null | undefined): boolean {
  return Boolean(altKey && hoveredVortexFullId);
}

/** ✨ Playwright/macOS often omit `event.altKey` on reconstructed pointer events — track the key. */
export function world3dSuggestionsAltHeld(eventAltKey: boolean, altHeld: boolean): boolean {
  return Boolean(eventAltKey || altHeld);
}

/** ✨ Guest `interactionHover` may stay null after a spawn-admit miss — keep the local marker hover for Alt+right-click. */
export function world3dRetainLocalVortexHover(local: string | null | undefined, guest: string | null | undefined): string | null {
  return guest || local || null;
}

/** ✨ Alt+right-click is the suggestions gesture — consume contextmenu so workspace chrome cannot steal it. */
export function world3dSuggestionsGestureConsumesContextMenu(altKey: boolean): boolean {
  return altKey;
}

/** ✨ Chrome overlays steal bubbling button=2 before orbit's canvas listener — window capture over the host is the route. */
export function world3dSuggestionsRightDownRoutesOnWindowCapture(button: number, overHost: boolean): boolean {
  return button === 2 && overHost;
}

export type World3dSuggestionsRightDownRoute = {
  readonly host: () => HTMLElement | null;
  readonly handle: (event: PointerEvent) => boolean;
};
export const world3dSuggestionsRightDownRoutes = new Set<World3dSuggestionsRightDownRoute>();

function world3dSuggestionsEventOverHost(node: HTMLElement | null, clientX: number, clientY: number): boolean {
  if (!node) return false;
  const rects = [node.getBoundingClientRect(), ...Array.from(node.querySelectorAll("canvas")).map((canvas) => canvas.getBoundingClientRect())];
  return rects.some((rect) => rect.width > 1 && rect.height > 1 && clientPointOverHost(clientX, clientY, rect));
}

function world3dSuggestionsDispatchWindowRightDown(event: PointerEvent): void {
  if (event.button !== 2) return;
  for (const route of world3dSuggestionsRightDownRoutes) {
    const overHost = world3dSuggestionsEventOverHost(route.host(), event.clientX, event.clientY);
    if (!world3dSuggestionsRightDownRoutesOnWindowCapture(event.button, overHost)) continue;
    if (route.handle(event) !== false) continue;
    event.preventDefault();
    event.stopPropagation();
    return;
  }
}

// 🧭️ `globalThis.` because this file imports React's synthetic `MouseEvent`/`DragEvent` for JSX props,
// which shadow the globals — and a real `window.addEventListener` listener is handed the DOM event.
function world3dSuggestionsDispatchWindowContextMenu(event: globalThis.MouseEvent): void {
  for (const route of world3dSuggestionsRightDownRoutes) {
    if (!world3dSuggestionsEventOverHost(route.host(), event.clientX, event.clientY)) continue;
    const asPointer = event as unknown as PointerEvent;
    if (route.handle(asPointer) !== false) continue;
    event.preventDefault();
    event.stopPropagation();
    return;
  }
}

if (typeof window !== "undefined") {
  window.addEventListener("pointerdown", world3dSuggestionsDispatchWindowRightDown, true);
  window.addEventListener("contextmenu", world3dSuggestionsDispatchWindowContextMenu, true);
}

/** 🖱️ Mirrors `nodeGraphSelectionActionArgs` for a world window bound to a framework interaction
 * domain — `merge` is passed through as already resolved at the call site (marquee/click modifier
 * state), not recomputed here. */

/** 🖱️ Instance picks join a scene interaction domain only when the instance names a topology id. */
export function world3dInstancePickUsesInteractionDomain(record: { readonly interactionId?: string } | undefined): boolean {
  return typeof record?.interactionId === "string" && record.interactionId.length > 0;
}

/** 🎯️ Resolves rendered instance ids onto the interaction targets they stand for, deduplicated and
 * order-preserving — several instances can share one target (procedural3d renders one instance per
 * geometry item of a channel, all of which resolve to that channel's port). */
export function interactionTargetsForInstances(instances: readonly WorldInstanceRecord[], ids: readonly string[]): readonly string[] {
  const seen = new Set<string>();
  const targets: string[] = [];
  for (const id of ids) {
    const target = instances.find((entry) => entry.id === id)?.interactionId ?? id;
    if (seen.has(target)) continue;
    seen.add(target);
    targets.push(target);
  }
  return targets;
}

/** 🎯️ The `{ granularity, id }` interaction target one rendered instance stands for — the record's own
 * `interactionGranularityId`/`interactionId` win, else the scene's default granularity and the id itself. */
export function world3dInstanceInteractionTarget(instances: readonly WorldInstanceRecord[], id: string, defaultGranularity: string): InteractionTarget {
  const record = instances.find((entry) => entry.id === id);
  return { granularity: record?.interactionGranularityId ?? defaultGranularity, id: record?.interactionId ?? id };
}

/** 🎯️ Every rendered instance id resolved onto its interaction target, deduplicated and order-preserving,
 * each under its OWN granularity — the marquee's shape of {@link interactionTargetsForInstances}. */
export function world3dInstanceInteractionTargets(instances: readonly WorldInstanceRecord[], ids: readonly string[], defaultGranularity: string): readonly { readonly granularity: string; readonly id: string }[] {
  const seen = new Set<string>();
  const targets: { readonly granularity: string; readonly id: string }[] = [];
  for (const id of ids) {
    const target = world3dInstanceInteractionTarget(instances, id, defaultGranularity);
    const key = `${target.granularity}\u0000${target.id}`;
    if (seen.has(key)) continue;
    seen.add(key);
    targets.push(target);
  }
  return targets;
}

/** 🎯️ The interaction target of one mesh COMPONENT (face/edge/vertex) of a rendered instance under a
 * domain: `<objectInteractionId>.<granularity>.<componentId>`, the object's own target id extended by the
 * component address (the lowpoly "mesh" domain's `lowpoly-document.<objectId>.<granularity>.<id>` rows). */
export function world3dComponentInteractionTarget(instances: readonly WorldInstanceRecord[], meshes: readonly WorldMeshRecord[], objectId: string, granularity: string, componentId: number): { readonly granularity: string; readonly id: string } | undefined {
  const record = instances.find((entry) => entry.id === objectId) ?? instances.find((entry) => entry.interactionId === objectId);
  const id = `${record?.interactionId ?? objectId}.${granularity}.${componentId}`;
  const source = record?.componentSource;
  if (!source) return { granularity, id };
  if (!Number.isInteger(componentId) || componentId < 0 || !["vertex", "edge", "face"].includes(granularity) || !/^[0-9a-f]{64}$/.test(source.handle) || !/^[0-9a-f]{64}$/.test(source.revision)) return undefined;
  const references = meshes.find(mesh => mesh.id === record?.meshId)?.data?.componentReferences?.[granularity];
  const label = references?.[componentId];
  if (!label || !/^[1-9][0-9]{0,19}$/.test(label) || BigInt(label) > 18446744073709551615n || references!.filter(value => value === label).length !== 1) return undefined;
  return { granularity, id: `${id}~${source.handle}~${label}~${source.revision}` };
}

/** 🧿️ The pickable non-instance layers of a `World3dScene`, each named after the scene field it is
 * parsed from (`vorticesJson`, `attractionsJson`, `targetVolumesJson`, `referencesJson`). */
export type World3dMarkerLayer = "vortex" | "attraction" | "targetVolume" | "reference";

/** 🧿️ Granularity a marker hit reports when its scene record declares no `interactionGranularityId`
 * — the layer's own name, the same way {@link WORLD3D_DEFAULT_INTERACTION_GRANULARITY} names the
 * instance layer's default. Keeps the host free of any per-app granularity vocabulary: an app that
 * calls its marker rows something else overrides it per record from the scene JSON. */
export const WORLD3D_DEFAULT_MARKER_GRANULARITY: Readonly<Record<World3dMarkerLayer, string>> = {
  vortex: "vortex",
  attraction: "attraction",
  targetVolume: "targetVolume",
  reference: "reference",
};

/** 🧿️ Resolves the `{ granularity, id }` interaction target one marker hit stands for — the record's
 * own scene-JSON fields win, else the record's own id at its layer's default granularity. */
export function world3dMarkerInteractionTarget(layer: World3dMarkerLayer, id: string, record?: World3dMarkerInteractionFields): { readonly granularity: string; readonly id: string } {
  return { granularity: record?.interactionGranularityId ?? WORLD3D_DEFAULT_MARKER_GRANULARITY[layer], id: record?.interactionId ?? id };
}
//#endregion 🎯️WorldInteractionDomain
//#endregion 🔖️World3dHost
