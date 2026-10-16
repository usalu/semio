/** 🎯️ Connect rubber band, voxel preview, attraction lines, status panes, engagement preview, suggestion menu items, marquee and click picking math, and the camera, introduction and hit-stamp bridges.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | World3dHost}, which re-exports every public name below so import specifiers stay unchanged.
 */

import React, {
  useCallback,
  useEffect,
  useLayoutEffect,
  useMemo,
  useRef,
  useSyncExternalStore
} from "react";
import type { ThreeCamera } from "@semio-tech/ui-react";
// #region 🔌️Adapters
import {
  cn,
  marqueeModeFromModifiers,
  ndcToViewportPoint,
  registerIntroductionSurfaceResolver,
  Spinner,
  type IntroductionResolvedGeometry,
  type SelectionMarqueeCoverage,
  type SelectionMarqueeMethod,
  type SelectionMarqueePoint,
  type UiLabel,
  BufferAttribute,
  BufferGeometry,
  OrthographicCamera,
  Quaternion,
  Vector3
} from "@semio-tech/ui-react";
// #endregion 🔌️Adapters
import { windowElementId, world3dComputeStatusV1, type ContextMenuItemSpec, type MergeMode } from "@semio-tech/framework";
import { cadVec3ToThree } from "@semio-tech/infinite-world-r3f";
import {
  type WorldAttractionRecord,
  type WorldEngagementPreviewItem,
  type WorldInstanceRecord,
  type WorldMeshRecord,
  type WorldSuggestionMenuRecord,
  type WorldVortexRecord
} from "../🔭️scene-schema/🟦️.ts";
import {
  buildVertexPickData,
  disposeWorldMeshAppearance,
  type MeshVisuals,
  useFrame,
  useThree,
  world3dInstanceLocalCorners
} from "../🎨️mesh-render/🟦️.tsx";

//#region 🔖️World3dHost
/** 🧲️ Rubber-band line drawn from the drag-connect source vortex to the currently hovered vortex (or itself, if hovering nothing). */
export function WorldConnectRubberBand({ from, to }: { readonly from: readonly [number, number, number]; readonly to: readonly [number, number, number] }) {
  const geometry = useMemo(() => {
    const positions = new Float32Array([from[0], from[1], from[2], to[0], to[1], to[2]]);
    const geom = new BufferGeometry();
    geom.setAttribute("position", new BufferAttribute(positions, 3));
    return geom;
  }, [from, to]);
  return (
    <lineSegments geometry={geometry} raycast={() => null}>
      <lineBasicMaterial color="#f59e0b" linewidth={2} />
    </lineSegments>
  );
}

/** 🧊️ Cursor-follow ghost box previewing the target volume that Alt+click would place, sized by the engagement's W/D/H steppers. */
export function WorldVoxelPreviewBox({ origin, dims, gridFactor }: { readonly origin: readonly [number, number, number]; readonly dims: readonly [number, number, number]; readonly gridFactor: number }) {
  return (
    <mesh position={origin as [number, number, number]} raycast={() => null}>
      <boxGeometry args={[dims[0] * gridFactor, dims[1] * gridFactor, dims[2] * gridFactor]} />
      <meshStandardMaterial color="#38bdf8" transparent opacity={0.48} />
    </mesh>
  );
}

export function WorldAttractionLines({ attractions }: { readonly attractions: readonly WorldAttractionRecord[] }) {
  if (!attractions.length) return null;
  return (
    <group>
      {attractions.map((attraction) => {
        const positions = new Float32Array([attraction.from[0], attraction.from[1], attraction.from[2], attraction.to[0], attraction.to[1], attraction.to[2]]);
        const geometry = new BufferGeometry();
        geometry.setAttribute("position", new BufferAttribute(positions, 3));
        return (
          <lineSegments key={attraction.id} geometry={geometry} raycast={() => null}>
            <lineBasicMaterial color={attraction.color ?? "#60a5fa"} linewidth={2} />
          </lineSegments>
        );
      })}
    </group>
  );
}

/** 👻️ GLB URL for a catalogue-drop ghost — scene mesh match when present, else the preview's own `meshUrl`, so a kind not placed yet still renders. */
export function worldGhostMeshUrl(preview: { readonly meshUrl?: string }, meshes: readonly Pick<WorldMeshRecord, "url">[]): string | undefined {
  const meshUrl = preview.meshUrl;
  if (!meshUrl) return undefined;
  return meshes.find((mesh) => mesh.url === meshUrl)?.url ?? meshUrl;
}

/** 🎞️ Demand-frameloop kick when a token changes — box fallbacks and already-cached GLBs otherwise leave a ghost invisible until the next orbit tick. */
export function DemandInvalidateOnToken({ token }: { readonly token: string }) {
  const invalidate = useThree((state) => state.invalidate);
  useLayoutEffect(() => {
    invalidate();
  }, [invalidate, token]);
  return null;
}

/**
 * ⏳️ The compute-status pane: what the producer says it is doing, how far it has got, and —
 * while it says so — a real button that stops it.
 *
 * 🛑️ The affordance is a `<button>`, not a clickable div, so it is in the tab order, answers Enter
 * and Space for free and reads as a control to a screen reader. Its label comes from the shell's own
 * `ui.common.cancel` bundle (both languages, no default) while the PHASE label comes from the
 * producer's `{en, de}` pair — the surface never invents either. Interaction-friendliness is a
 * CLAUDE.md law, and an expensive operation with progress but no stop is only half of it
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
 */
/** 🎨️ Announces the current owned surface publication without copying mesh or appearance state. */
export function WorldSurfaceStatusPane({visuals,locale,glassClass}:{readonly visuals:ReadonlyMap<string,MeshVisuals>;readonly locale?:string;readonly glassClass:string}){
  const snapshot=()=>{let pending=0,failed=0,cancelled=0;for(const visual of visuals.values()){pending+=visual.appearance?.closed?0:visual.appearance?.pending ?? 0;if(visual.fault||visual.appearance?.fault && visual.appearance.fault!=="cancelled")failed++;if(visual.appearance?.fault==="cancelled")cancelled++;}return `${pending}:${failed}:${cancelled}`;};
  const version=useSyncExternalStore(useCallback(listener=>{const appearances=[...visuals.values()].flatMap(visual=>visual.appearance?[visual.appearance]:[]);for(const appearance of appearances)appearance.listeners.add(listener);return()=>{for(const appearance of appearances)appearance.listeners.delete(listener);};},[visuals]),snapshot,snapshot);
  const [pending,failed,cancelled]=version.split(":").map(Number);if(pending===undefined||failed===undefined||cancelled===undefined)throw Error("Complete owned surface status required");const language=locale?.split(/[-_]/)[0];const labels=language==="en"?{loading:"Loading surface textures",failed:"Surface could not be displayed",cancelled:"Surface texture loading cancelled",cancel:"Cancel"}:language==="de"?{loading:"Oberflächentexturen werden geladen",failed:"Oberfläche konnte nicht dargestellt werden",cancelled:"Laden der Oberflächentexturen abgebrochen",cancel:"Abbrechen"}:undefined;
  const active=Boolean(pending||failed||cancelled);
  return <div role="status" aria-live="polite" aria-busy={pending>0 || undefined} data-slot="world-surface-status" data-surface-pending={pending} data-surface-failed={failed} data-surface-cancelled={cancelled} className={active?cn("pointer-events-auto rounded px-single py-half text-xs shadow-sm",glassClass):"sr-only"}>{active&&<><span>{failed?labels?.failed:pending?labels?.loading:labels?.cancelled}</span>{pending>0&&<><progress value={[...visuals.values()].reduce((sum,visual)=>sum+(visual.appearance?.textures.size ?? 0),0)-pending} max={[...visuals.values()].reduce((sum,visual)=>sum+(visual.appearance?.textures.size ?? 0),0)} aria-label={labels?.loading}/><button type="button" onClick={()=>{for(const visual of visuals.values()){const appearance=visual.appearance;if(appearance&&!appearance.closed&&appearance.pending){appearance.fault="cancelled";disposeWorldMeshAppearance(appearance);}}}}>{labels?.cancel}</button></>}</>}</div>;
}

export function WorldComputeStatusPane({
  status,
  glassClass,
  locale,
  onCancel,
}: {
  readonly status: ReturnType<typeof world3dComputeStatusV1>;
  readonly glassClass: string;
  readonly locale: string | undefined;
  readonly onCancel: () => void;
}) {
  const idle = !status.computing && !status.cancellable && status.phase !== "cancelled";
  const language = locale?.toLowerCase().split(/[-_]/)[0];
  const labels = language === "en" ? {loading: "Loading", cancel: "Cancel"} : language === "de" ? {loading: "Wird geladen", cancel: "Abbrechen"} : null;
  const phaseText = status.phaseLabel && labels ? (language === "de" ? status.phaseLabel.de : status.phaseLabel.en) : null;
  const percent = status.unitsTotal > 0 ? Math.round(status.ratio * 100) : null;
  // 📢️ The live region STAYS MOUNTED while idle, rendering nothing.
  //
  // 🐛️ This used to `return null` at idle, which is the classic ARIA live-region trap: a reader only
  // announces changes INSIDE a region it was already observing, so a `role="status"` element that
  // first appears at the moment it has something to say is announced by nothing. An evaluation going
  // idle → "Sampling edges" → "Evaluated" was therefore completely silent, which is the whole point
  // of the region. Keeping an empty, zero-size region in the tree makes every later phase a
  // mutation of an observed subtree, which IS announced.
  // 🕳️ An idle surface with a HINT is not silent: the producer said why it is empty, in its own
  // locale, and that sentence is painted. Without this the hint reached `data-status-json` and stopped
  // there — a user looking at an empty generate preview was told nothing
  // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `panel-i18n · de:preview_hint`).
  if (idle) {
    if (!status.hint) return <div className="sr-only" data-slot="world-compute-status" data-compute-phase={status.phase} role="status" aria-busy={undefined} />;
    return (
      <div className={cn("pointer-events-none flex items-center gap-single rounded px-single py-half text-xs text-muted-foreground shadow-sm", glassClass)} data-level="pane" data-slot="world-compute-status" data-compute-phase={status.phase} role="status" aria-busy={undefined}>
        <span data-slot="world-compute-hint">{status.hint}</span>
      </div>
    );
  }
  if (!labels) return <div className="sr-only" data-slot="world-compute-status" data-compute-phase={status.phase} data-language-required="" role="status" aria-busy={status.computing ? "true" : undefined} />;
  return (
    <div
      className={cn("pointer-events-auto flex items-center gap-single rounded px-single py-half text-xs shadow-sm", glassClass)}
      data-level="pane"
      data-slot="world-compute-status"
      data-compute-phase={status.phase}
      data-compute-cancellable={status.cancellable ? "" : undefined}
      data-compute-ratio={status.unitsTotal > 0 ? String(status.ratio) : undefined}
      data-compute-units-done={status.unitsDone}
      data-compute-units-total={status.unitsTotal}
      role="status"
      aria-busy={status.computing ? "true" : undefined}
    >
      {status.computing ? <Spinner size="small" /> : null}
      <span data-slot="world-compute-phase">{phaseText ?? labels.loading}</span>
      {percent === null ? null : (
        <span data-slot="world-compute-progress">
          {status.unitsDone}/{status.unitsTotal} ({percent}%)
        </span>
      )}
      {status.cancellable ? (
        <button type="button" data-slot="world-compute-cancel" data-cancel-action={status.cancelAction} data-cancel-run-id={status.cancelArgs.runId} data-cancel-generation={status.cancelArgs.generation} className="rounded border px-half py-0 text-xs" onClick={onCancel}>
          {labels.cancel}
        </button>
      ) : null}
    </div>
  );
}

export function EngagementPreviewLayer({ items, color }: { readonly items: readonly WorldEngagementPreviewItem[]; readonly color: string }) {
  if (!items.length) return null;
  return (
    <group>
      {items.map((item, index) => {
        if (item.kind === "point") {
          return (
            <mesh key={`preview-point-${index}`} position={item.position as [number, number, number]} raycast={() => null}>
              <sphereGeometry args={[0.08, 12, 12]} />
              <meshStandardMaterial color={color} />
            </mesh>
          );
        }
        if (item.kind === "segment") {
          const positions = new Float32Array([item.from[0], item.from[1], item.from[2], item.to[0], item.to[1], item.to[2]]);
          const geometry = new BufferGeometry();
          geometry.setAttribute("position", new BufferAttribute(positions, 3));
          return (
            <lineSegments key={`preview-segment-${index}`} geometry={geometry} raycast={() => null}>
              <lineBasicMaterial color={color} linewidth={2} />
            </lineSegments>
          );
        }
        if (item.kind === "box-preview" && item.cornerA && item.cornerB) {
          const [ax, ay, az] = item.cornerA;
          const [bx, by] = item.cornerB;
          const width = Math.max(Math.abs(bx - ax), 0.05);
          const depth = Math.max(Math.abs(by - ay), 0.05);
          // `height` is a separate vertical extrusion from the footprint plane (az), not derived
          // from cornerB's z — the interaction specs author cornerA/cornerB as ground-plane points.
          const height = Math.max(Math.abs(item.height ?? 0.05), 0.05);
          return (
            <mesh key={`preview-box-${index}`} position={[(ax + bx) * 0.5, (ay + by) * 0.5, az + height * 0.5]} raycast={() => null}>
              <boxGeometry args={[width, depth, height]} />
              <meshBasicMaterial color={color} transparent opacity={0.35} depthWrite={false} wireframe />
            </mesh>
          );
        }
        if (item.kind === "linear-handle") {
          const [ox, oy, oz] = item.origin;
          const [dx, dy, dz] = item.axis;
          const length = Math.max(Math.hypot(dx, dy, dz), 0.05);
          const direction = new Vector3(dx, dy, dz).normalize();
          const quaternion = new Quaternion().setFromUnitVectors(new Vector3(0, 1, 0), direction);
          return (
            <mesh key={`preview-handle-${index}`} position={[ox + dx * 0.5, oy + dy * 0.5, oz + dz * 0.5]} quaternion={quaternion} raycast={() => null}>
              <cylinderGeometry args={[0.02, 0.02, length, 8]} />
              <meshBasicMaterial color={color} transparent opacity={0.6} depthWrite={false} />
            </mesh>
          );
        }
        return null;
      })}
    </group>
  );
}

/** 🧭️ Floating per-vortex candidate popup opened by Alt+right-click or the context menu's "Suggest objects" — a one-shot placement picker that does not switch the active utility into brush mode; hovering a row previews the ghost, clicking places it. Icon + active highlight only (no color swatch — object-kind color stays on the 3D ghost). */
export function suggestionMenuItems(
  menu: Pick<WorldSuggestionMenuRecord, "pending" | "candidates" | "vortexFullId">,
  activeIndex: number,
  labels: { readonly checkingPlacement: UiLabel; readonly noPlacement: UiLabel },
): ContextMenuItemSpec[] {
  if (menu.pending) {
    return [{ id: "pending", label: labels.checkingPlacement, disabled: true }];
  }
  if (menu.candidates.length === 0) {
    return [{ id: "empty", label: labels.noPlacement, disabled: true }];
  }
  return menu.candidates.map((candidate) => ({
    id: `suggestion-${candidate.index}`,
    label: `${candidate.objectLabel} · ${candidate.vortexLabel}`,
    icon: candidate.icon ?? "box",
    checked: candidate.index === activeIndex,
    action: "acceptSuggestion",
    args: { index: candidate.index, ...(menu.vortexFullId ? { fullId: menu.vortexFullId } : {}) },
    hoverAction: "hoverSuggestion",
    hoverArgs: { index: candidate.index },
  }));
}

export const WORLD3D_MARQUEE_DRAG_THRESHOLD_PX = 4;
export const MARQUEE_DRAG_THRESHOLD_PX = WORLD3D_MARQUEE_DRAG_THRESHOLD_PX;

/** 🖱️ Marquee may steal the pointer only after the click slop — otherwise r3f never sees the click. */
export function world3dMarqueePointerCaptureArmed(distancePx: number): boolean {
  return distancePx > MARQUEE_DRAG_THRESHOLD_PX;
}

/** 🎯️ The five-word `MergeMode` set algebra for non-string id sets — mirrors `selectionMergeIds`
 * from `@semio-tech/ui-react`, which is string-only. `range` has no ordered topology in a world
 * viewport, so it replaces (see `🕹️interaction/🧫️fixtures/🎯️merge-modes.json`, `range.unorderedDomains`). */
export function mergeIdSet<T>(mode: MergeMode, current: readonly T[], incoming: readonly T[]): T[] {
  const currentSet = new Set(current);
  const incomingSet = new Set(incoming);
  if (mode === "replace" || mode === "range") return [...incomingSet];
  if (mode === "additive") {
    for (const id of incomingSet) currentSet.add(id);
    return [...currentSet];
  }
  if (mode === "subtractive") {
    for (const id of incomingSet) currentSet.delete(id);
    return [...currentSet];
  }
  for (const id of incomingSet) {
    if (currentSet.has(id)) currentSet.delete(id);
    else currentSet.add(id);
  }
  return [...currentSet];
}

/** 🎯️ Resolves a world surface's declared selection mode before falling back to the shared selection toolbar. */
export function resolveWorldMergeMode(
  configuredMode: MergeMode | undefined,
  modifiers: { readonly shiftKey?: boolean; readonly ctrlKey?: boolean; readonly metaKey?: boolean },
  persistentMode?: MergeMode,
): MergeMode {
  if (configuredMode === undefined) return marqueeModeFromModifiers(modifiers, persistentMode);
  if (configuredMode !== "replace") return configuredMode;
  const shift = modifiers.shiftKey === true;
  const ctrl = modifiers.ctrlKey === true || modifiers.metaKey === true;
  if (shift && ctrl) return "invertive";
  if (shift) return "additive";
  if (ctrl) return "subtractive";
  return "replace";
}

/** 🖱️ The component (vertex/edge/face) pick's one deviation from the whole-instance pick: a BARE
 * click toggles instead of replacing, because component picking is an accumulate gesture. Every other
 * chord resolves exactly as {@link resolveWorldMergeMode} says. The result is a `MergeMode` — the ONE
 * merge vocabulary of `🕹️interaction/🧬️schema/🔣️.json`, pinned by
 * `🕹️interaction/🧫️fixtures/🎯️merge-modes.json`. Ticket 26/09/09/PROCEDURAL-3D-END-TO-END deleted the
 * `instanceMergeArg`/`componentMergeArg` translation into a private `add`/`remove`/`toggle` spelling:
 * the framework's `parse_merge_mode` rejects those words, so every modifier-click on a domain-bound
 * world scene faulted with `interactionSelect: unknown merge '…'`. */
export function componentPickMergeMode(mode: MergeMode): MergeMode {
  return mode === "replace" ? "invertive" : mode;
}

function pointInMarqueeRect(sx: number, sy: number, marquee: readonly SelectionMarqueePoint[]): boolean {
  if (marquee.length < 2) return false;
  const start = marquee[0]!;
  const end = marquee[marquee.length - 1]!;
  const minX = Math.min(start.x, end.x);
  const maxX = Math.max(start.x, end.x);
  const minY = Math.min(start.y, end.y);
  const maxY = Math.max(start.y, end.y);
  return sx >= minX && sx <= maxX && sy >= minY && sy <= maxY;
}

/** 🎯️ Even-odd point-in-polygon test for lasso selection. */
function pointInPolygon(sx: number, sy: number, polygon: readonly SelectionMarqueePoint[]): boolean {
  let inside = false;
  for (let i = 0, j = polygon.length - 1; i < polygon.length; j = i++) {
    const a = polygon[i]!;
    const b = polygon[j]!;
    const intersects = a.y > sy !== b.y > sy && sx < ((b.x - a.x) * (sy - a.y)) / (b.y - a.y) + a.x;
    if (intersects) inside = !inside;
  }
  return inside;
}

function pointInMarqueeRegion(sx: number, sy: number, method: SelectionMarqueeMethod, marquee: readonly SelectionMarqueePoint[]): boolean {
  if (method === "lasso" && marquee.length >= 3) return pointInPolygon(sx, sy, marquee);
  return pointInMarqueeRect(sx, sy, marquee);
}

/** 🎯️ Window (full containment, all points) vs crossing (partial, any point) semantics for multi-point elements. */
function pointsSatisfyMarquee(points: readonly (readonly [number, number])[], method: SelectionMarqueeMethod, marquee: readonly SelectionMarqueePoint[], coverage: SelectionMarqueeCoverage): boolean {
  if (points.length === 0) return false;
  const test = (point: readonly [number, number]) => pointInMarqueeRegion(point[0], point[1], method, marquee);
  return coverage === "full" ? points.every(test) : points.some(test);
}

function projectWorldPoint(point: readonly [number, number, number], offset: readonly [number, number, number], camera: ThreeCamera, rect: DOMRect): { readonly x: number; readonly y: number } {
  const projected = new Vector3(point[0] + offset[0], point[1] + offset[1], point[2] + offset[2]).project(camera);
  return {
    x: ((projected.x + 1) / 2) * rect.width,
    y: ((-projected.y + 1) / 2) * rect.height,
  };
}

export function resolveMarqueeComponentIds(
  instances: readonly WorldInstanceRecord[],
  meshes: readonly WorldMeshRecord[],
  selectionMode: string,
  activeObjectId: string | undefined,
  marquee: readonly SelectionMarqueePoint[],
  rect: DOMRect,
  camera: ThreeCamera,
  method: SelectionMarqueeMethod,
  coverage: SelectionMarqueeCoverage,
): readonly number[] {
  const active = instances.find((instance) => instance.id === activeObjectId) ?? instances.find((instance) => instance.interactionId === activeObjectId);
  if (!active) return [];
  const meshId = active.meshId ?? active.id;
  const meshData = meshes.find((mesh) => mesh.id === meshId)?.data;
  if (!meshData) return [];
  const offset = (active.position ?? [0, 0, 0]) as [number, number, number];
  const project = (point: readonly [number, number, number]): readonly [number, number] => {
    const screen = projectWorldPoint(point, offset, camera, rect);
    return [screen.x, screen.y];
  };
  const hits = new Set<number>();
  if (selectionMode === "vertex") {
    const pick = buildVertexPickData(meshData);
    if (!pick) return [];
    const positions = pick.geometry.attributes.position!;
    for (let index = 0; index < pick.vertexIds.length; index += 1) {
      const point = project([positions.getX(index), positions.getY(index), positions.getZ(index)]);
      if (pointsSatisfyMarquee([point], method, marquee, coverage)) hits.add(pick.vertexIds[index]!);
    }
  } else if (selectionMode === "edge" && meshData.edgeIds && meshData.edgePositions) {
    for (let edgeIndex = 0; edgeIndex < meshData.edgeIds.length; edgeIndex += 1) {
      const base = edgeIndex * 6;
      const a = project([meshData.edgePositions[base]!, meshData.edgePositions[base + 1]!, meshData.edgePositions[base + 2]!]);
      const b = project([meshData.edgePositions[base + 3]!, meshData.edgePositions[base + 4]!, meshData.edgePositions[base + 5]!]);
      if (pointsSatisfyMarquee([a, b], method, marquee, coverage)) hits.add(meshData.edgeIds[edgeIndex]!);
    }
  } else if (selectionMode === "face" && meshData.faceIds && meshData.indices.length) {
    for (let faceIndex = 0; faceIndex < meshData.faceIds.length; faceIndex += 1) {
      const i0 = meshData.indices[faceIndex * 3] ?? 0;
      const i1 = meshData.indices[faceIndex * 3 + 1] ?? 0;
      const i2 = meshData.indices[faceIndex * 3 + 2] ?? 0;
      const p0 = project([meshData.positions[i0 * 3]!, meshData.positions[i0 * 3 + 1]!, meshData.positions[i0 * 3 + 2]!]);
      const p1 = project([meshData.positions[i1 * 3]!, meshData.positions[i1 * 3 + 1]!, meshData.positions[i1 * 3 + 2]!]);
      const p2 = project([meshData.positions[i2 * 3]!, meshData.positions[i2 * 3 + 1]!, meshData.positions[i2 * 3 + 2]!]);
      if (pointsSatisfyMarquee([p0, p1, p2], method, marquee, coverage)) hits.add(meshData.faceIds[faceIndex]!);
    }
  }
  return [...hits];
}

export function resolveMarqueeInstanceIds(
  instances: readonly WorldInstanceRecord[],
  meshes: readonly WorldMeshRecord[],
  marquee: readonly SelectionMarqueePoint[],
  rect: DOMRect,
  camera: ThreeCamera,
  method: SelectionMarqueeMethod,
  coverage: SelectionMarqueeCoverage,
): readonly string[] {
  const meshById = new Map(meshes.map((mesh) => [mesh.id, mesh]));
  const hits: string[] = [];
  instances.forEach((instance, index) => {
    const meshId = instance.meshId ?? instance.id;
    const meshData = meshById.get(meshId)?.data;
    const position = (instance.position ?? [instance.x ?? index, instance.y ?? 0, instance.z ?? 0]) as [number, number, number];
    const scale = (instance.scale ?? [1, 1, 1]) as [number, number, number];
    const rotation = instance.rotation;
    const quaternion = rotation ? new Quaternion(rotation[0], rotation[1], rotation[2], rotation[3]) : undefined;
    const localCorners = world3dInstanceLocalCorners(meshData, meshById.get(meshId)?.url);
    const worldCorners = localCorners.map((corner) => {
      const v = new Vector3(corner[0] * scale[0], corner[1] * scale[1], corner[2] * scale[2]);
      if (quaternion) v.applyQuaternion(quaternion);
      v.add(new Vector3(position[0], position[1], position[2]));
      return [v.x, v.y, v.z] as const;
    });
    const points = worldCorners.map((corner) => {
      const screen = projectWorldPoint(corner, [0, 0, 0], camera, rect);
      return [screen.x, screen.y] as const;
    });
    if (pointsSatisfyMarquee(points, method, marquee, coverage)) hits.push(instance.id);
  });
  return hits;
}

/** 🖱️ Whether a host-local click sits inside the axis-aligned screen box of projected mesh corners. */
export function world3dProjectedAabbContainsClick(
  click: { readonly x: number; readonly y: number },
  corners: readonly (readonly [number, number])[],
): boolean {
  if (corners.length === 0) return false;
  let minX = Infinity;
  let minY = Infinity;
  let maxX = -Infinity;
  let maxY = -Infinity;
  for (const [x, y] of corners) {
    if (x < minX) minX = x;
    if (y < minY) minY = y;
    if (x > maxX) maxX = x;
    if (y > maxY) maxY = y;
  }
  return click.x >= minX && click.x <= maxX && click.y >= minY && click.y <= maxY;
}

/** 🖱️ Nearest projected AABB that contains the click; empty space returns null. */
export function resolveClickInstanceIdFromProjected(
  click: { readonly x: number; readonly y: number },
  candidates: readonly { readonly id: string; readonly corners: readonly (readonly [number, number])[]; readonly depth: number }[],
): string | null {
  let bestId: string | null = null;
  let bestDepth = Infinity;
  for (const candidate of candidates) {
    if (!world3dProjectedAabbContainsClick(click, candidate.corners)) continue;
    if (candidate.depth < bestDepth) {
      bestDepth = candidate.depth;
      bestId = candidate.id;
    }
  }
  return bestId;
}

export function resolveClickInstanceId(
  instances: readonly WorldInstanceRecord[],
  meshes: readonly WorldMeshRecord[],
  click: SelectionMarqueePoint,
  rect: DOMRect,
  camera: ThreeCamera,
): string | null {
  const meshById = new Map(meshes.map((mesh) => [mesh.id, mesh]));
  const cam = camera.position;
  const candidates: { id: string; corners: readonly (readonly [number, number])[]; depth: number }[] = [];
  instances.forEach((instance, index) => {
    const meshId = instance.meshId ?? instance.id;
    const meshData = meshById.get(meshId)?.data;
    const position = (instance.position ?? [instance.x ?? index, instance.y ?? 0, instance.z ?? 0]) as [number, number, number];
    const scale = (instance.scale ?? [1, 1, 1]) as [number, number, number];
    const rotation = instance.rotation;
    const quaternion = rotation ? new Quaternion(rotation[0], rotation[1], rotation[2], rotation[3]) : undefined;
    const localCorners = world3dInstanceLocalCorners(meshData, meshById.get(meshId)?.url);
    const worldCorners = localCorners.map((corner) => {
      const v = new Vector3(corner[0] * scale[0], corner[1] * scale[1], corner[2] * scale[2]);
      if (quaternion) v.applyQuaternion(quaternion);
      v.add(new Vector3(position[0], position[1], position[2]));
      return [v.x, v.y, v.z] as const;
    });
    const points = worldCorners.map((corner) => {
      const screen = projectWorldPoint(corner, [0, 0, 0], camera, rect);
      return [screen.x, screen.y] as const;
    });
    candidates.push({
      id: instance.id,
      corners: points,
      depth: Math.hypot(position[0] - cam.x, position[1] - cam.y, position[2] - cam.z),
    });
  });
  return resolveClickInstanceIdFromProjected(click, candidates);
}

export function CameraRefBridge({ cameraRef }: { readonly cameraRef: React.MutableRefObject<ThreeCamera | null> }) {
  const camera = useThree((state) => state.camera);
  useEffect(() => {
    cameraRef.current = camera;
  }, [camera, cameraRef]);
  return null;
}

/** 🧭️ Registers this window's live camera and entity data as the introduction demonstration
 * engine's resolver for `windowElementId(windowInstanceId)` — the same element id `windowElementId(kind.id)`
 * builds for authoring (see `mit-bestand/demonstrator/🪧️brand.ts`). Only the base (non-split) window instance
 * registers under its exact kind id this way, since `windowInstanceId` equals `kind.id` verbatim for it;
 * a split/spawned extra instance registers under its own instance id instead (last-write-wins if multiple
 * instances of a kind are open — acceptable for a single demonstration target, not a general
 * multi-instance broadcast). Supports `scenePoint` (raw 3D → screen), `canvasPoint` (ground-plane 2D →
 * screen, z = 0), and `entity` for domains `"vortex"` (`WorldVortexRecord.fullId`), `"object"`
 * (`WorldInstanceRecord.id`, reveal-cutoff-hidden instances resolve to null), and `"attraction"`
 * (`WorldAttractionRecord.id`, exposes a `polyline` of its projected endpoints for `Curve` targeting) —
 * `entity: "*"` picks whichever visible record projects nearest the window's own viewport center. Reads
 * the latest arrays via a ref so the registration effect (keyed on camera/gl/window id, not on scene data)
 * never has to re-run just because the scene updated. */
export function IntroductionWorldResolverBridge({
  windowInstanceId,
  vortices,
  instances,
  attractions,
}: {
  readonly windowInstanceId: string;
  readonly vortices: readonly WorldVortexRecord[];
  readonly instances: readonly WorldInstanceRecord[];
  readonly attractions: readonly WorldAttractionRecord[];
}) {
  const camera = useThree((state) => state.camera);
  const gl = useThree((state) => state.gl);
  const dataRef = useRef({ vortices, instances, attractions });
  dataRef.current = { vortices, instances, attractions };

  useEffect(() => {
    const windowId = windowElementId(windowInstanceId);
    const project = (position: readonly [number, number, number]): { readonly x: number; readonly y: number; readonly visible: boolean } => {
      const projected = new Vector3(...cadVec3ToThree(position)).project(camera);
      if (projected.z >= 1 || Math.abs(projected.x) > 1.05 || Math.abs(projected.y) > 1.05) return { x: 0, y: 0, visible: false };
      const rect = gl.domElement.getBoundingClientRect();
      const { x, y } = ndcToViewportPoint(projected, rect);
      return { x, y, visible: true };
    };
    const nearestToCenter = <T,>(
      records: readonly T[],
      positionOf: (record: T) => readonly [number, number, number],
    ): { readonly record: T; readonly projected: { readonly x: number; readonly y: number } } | null => {
      const rect = gl.domElement.getBoundingClientRect();
      const centerX = rect.left + rect.width / 2;
      const centerY = rect.top + rect.height / 2;
      let best: { record: T; projected: { x: number; y: number } } | null = null;
      let bestDistance = Infinity;
      for (const record of records) {
        const projected = project(positionOf(record));
        if (!projected.visible) continue;
        const distance = Math.hypot(projected.x - centerX, projected.y - centerY);
        if (distance < bestDistance) {
          bestDistance = distance;
          best = { record, projected };
        }
      }
      return best;
    };
    const instancePosition = (instance: WorldInstanceRecord): readonly [number, number, number] => instance.position ?? [instance.x ?? 0, instance.y ?? 0, instance.z ?? 0];
    const attractionMidpoint = (attraction: WorldAttractionRecord): readonly [number, number, number] => [
      (attraction.from[0] + attraction.to[0]) / 2,
      (attraction.from[1] + attraction.to[1]) / 2,
      (attraction.from[2] + attraction.to[2]) / 2,
    ];

    return registerIntroductionSurfaceResolver(windowId, {
      scenePoint: project,
      canvasPoint: (x, y) => project([x, y, 0]),
      entity: (domain, entity): IntroductionResolvedGeometry | null => {
        const { vortices: liveVortices, instances: liveInstances, attractions: liveAttractions } = dataRef.current;
        if (domain === "vortex") {
          if (entity === "*") {
            const nearest = nearestToCenter(liveVortices, (vortex) => vortex.position);
            return nearest ? { point: nearest.projected, visible: true } : null;
          }
          const vortex = liveVortices.find((candidate) => candidate.fullId === entity);
          if (!vortex) return null;
          const projected = project(vortex.position);
          return projected.visible ? { point: projected, visible: true } : null;
        }
        if (domain === "object") {
          const visibleInstances = liveInstances;
          if (entity === "*") {
            const nearest = nearestToCenter(visibleInstances, instancePosition);
            return nearest ? { point: nearest.projected, visible: true } : null;
          }
          const instance = visibleInstances.find((candidate) => candidate.id === entity);
          if (!instance) return null;
          const projected = project(instancePosition(instance));
          return projected.visible ? { point: projected, visible: true } : null;
        }
        if (domain === "attraction") {
          const attraction = entity === "*" ? nearestToCenter(liveAttractions, attractionMidpoint)?.record : liveAttractions.find((candidate) => candidate.id === entity);
          if (!attraction) return null;
          const from = project(attraction.from);
          const to = project(attraction.to);
          if (!from.visible || !to.visible) return null;
          return { point: { x: (from.x + to.x) / 2, y: (from.y + to.y) / 2 }, polyline: [from, to], visible: true };
        }
        return null;
      },
    });
  }, [camera, gl, windowInstanceId]);
  return null;
}

/** 🎯️ Widens Line/Points raycast hit area so thin edge/vertex geometry is reliably pickable without stealing face clicks. */
export function RaycasterPickTuning() {
  const raycaster = useThree((state) => state.raycaster);
  useEffect(() => {
    raycaster.params.Line = { threshold: 0.12 };
    raycaster.params.Points = { threshold: 0.08 };
  }, [raycaster]);
  return null;
}

export function WorldGumballHitStamp({
  target,
  active,
  hostRef,
}: {
  readonly target?: readonly [number, number, number];
  readonly active: boolean;
  readonly hostRef: React.RefObject<HTMLDivElement | null>;
}) {
  const camera = useThree((state) => state.camera);
  const size = useThree((state) => state.size);
  useFrame(() => {
    const el = hostRef.current;
    if (!el) return;
    if (!active || !target) {
      if (el.getAttribute("data-gumball-hits") !== "[]") el.setAttribute("data-gumball-hits", "[]");
      return;
    }
    const origin = new Vector3(target[0], target[1], target[2]);
    const scale = Math.max(camera.position.distanceTo(origin) / 8, 1e-4);
    const tip = 0.85;
    const handles = [
      { kind: "moveX", x: target[0] + scale * tip, y: target[1], z: target[2] },
      { kind: "moveY", x: target[0], y: target[1] + scale * tip, z: target[2] },
      { kind: "moveZ", x: target[0], y: target[1], z: target[2] + scale * tip },
      { kind: "origin", x: target[0], y: target[1], z: target[2] },
    ];
    const hits = handles.map((handle) => {
      const projected = new Vector3(handle.x, handle.y, handle.z).project(camera);
      return {
        kind: handle.kind,
        sx: Math.round((projected.x * 0.5 + 0.5) * size.width),
        sy: Math.round((-projected.y * 0.5 + 0.5) * size.height),
        ndcZ: Number(projected.z.toFixed(3)),
      };
    });
    const next = JSON.stringify(hits);
    if (el.getAttribute("data-gumball-hits") !== next) el.setAttribute("data-gumball-hits", next);
  });
  return null;
}

export function WorldVortexHitStamp({
  vortices,
  hostRef,
}: {
  readonly vortices: readonly WorldVortexRecord[];
  readonly hostRef: React.RefObject<HTMLDivElement | null>;
}) {
  const camera = useThree((state) => state.camera);
  const size = useThree((state) => state.size);
  useFrame(() => {
    const el = hostRef.current;
    if (!el) return;
    const hits = vortices.map((vortex) => {
      const projected = new Vector3(vortex.position[0], vortex.position[1], vortex.position[2]).project(camera);
      return {
        fullId: vortex.fullId,
        x: vortex.position[0],
        y: vortex.position[1],
        z: vortex.position[2],
        sx: Math.round((projected.x * 0.5 + 0.5) * size.width),
        sy: Math.round((-projected.y * 0.5 + 0.5) * size.height),
        ndcZ: Number(projected.z.toFixed(3)),
      };
    });
    const next = JSON.stringify(hits);
    if (el.getAttribute("data-vortex-hits") !== next) el.setAttribute("data-vortex-hits", next);
  });
  return null;
}

export function paneSuffixFromSurfaceId(surfaceId?: string): string | undefined {
  if (!surfaceId) return undefined;
  const slash = surfaceId.lastIndexOf("/");
  return slash >= 0 ? surfaceId.slice(slash + 1) : surfaceId;
}

/** 📡️ World-space camera ray through an NDC point — orthographic uses parallel near→far unproject rays;
 * perspective uses the pinhole from `camera.position`. Duck-types `isOrthographicCamera` (not `instanceof`) so
 * R3F-swapped cameras stay correct. */
export function worldRayFromNdc(ndcX: number, ndcY: number, camera: ThreeCamera): { origin: Vector3; direction: Vector3 } | null {
  const ortho = camera as OrthographicCamera & { readonly isOrthographicCamera?: boolean };
  if (ortho.isOrthographicCamera) {
    const origin = new Vector3(ndcX, ndcY, -1).unproject(camera);
    const direction = new Vector3(ndcX, ndcY, 1).unproject(camera).sub(origin);
    if (direction.lengthSq() < 1e-12) return null;
    return { origin, direction: direction.normalize() };
  }
  const origin = camera.position.clone();
  const direction = new Vector3(ndcX, ndcY, 0.5).unproject(camera).sub(origin);
  if (direction.lengthSq() < 1e-12) return null;
  return { origin, direction: direction.normalize() };
}

/** 🎯️ Intersects the camera ray through a client point with the world Z=0 ground plane (catalogue drop + face drag). */
export function raycastGroundPoint(clientX: number, clientY: number, hostRect: DOMRect, camera: ThreeCamera): [number, number, number] | null {
  const ndcX = ((clientX - hostRect.left) / hostRect.width) * 2 - 1;
  const ndcY = -(((clientY - hostRect.top) / hostRect.height) * 2 - 1);
  const ray = worldRayFromNdc(ndcX, ndcY, camera);
  if (!ray) return null;
  if (Math.abs(ray.direction.z) < 1e-6) return null;
  const t = -ray.origin.z / ray.direction.z;
  if (t < 0) return null;
  const hit = ray.origin.add(ray.direction.multiplyScalar(t));
  return [hit.x, hit.y, hit.z];
}
//#endregion 🔖️World3dHost
