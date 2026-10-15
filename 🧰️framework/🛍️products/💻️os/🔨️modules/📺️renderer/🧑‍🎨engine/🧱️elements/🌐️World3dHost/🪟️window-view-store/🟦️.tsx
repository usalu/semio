/** 🪟️ The orbit projection switch pane, window instance id context, retained viewport view store and registry, and the pending projection registry.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | World3dHost}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { createContext, useState } from "react";
import type { ThreeCamera } from "@semio-tech/ui-react";
// #region 🔌️Adapters
import {
  childElementId,
  Pane,
  useLabel,
  usePaneSlot,
  windowMeasuresBodyClass,
  type Anchor,
  type IconName,
  Vector3
} from "@semio-tech/ui-react";
// #endregion 🔌️Adapters
import { worldProjectionSpecIconId, WorldProjectionKindSwitch, type WorldCameraState, type WorldProjectionSpec } from "@semio-tech/infinite-world-r3f";
import { type WorldParsedCameraState } from "../🔭️scene-schema/🟦️.ts";
import { worldRayFromNdc } from "../🎯️marquee-and-bridges/🟦️.tsx";


//#region 🔖️World3dHost
/** 🖱️➡️ Signed distance along `axis` (unit vector) from `origin` to the point on that line closest to the
 * camera ray through the current pointer position — the standard closest-point-between-two-lines
 * construction, used so a face-normal drag tracks naturally instead of needing a ground/tangent-plane
 * intersection (which is undefined for motion parallel to the plane, i.e. exactly along the normal). */
export function axisDragParam(clientX: number, clientY: number, hostRect: DOMRect, camera: ThreeCamera, origin: readonly [number, number, number], axis: readonly [number, number, number]): number | null {
  const ndcX = ((clientX - hostRect.left) / hostRect.width) * 2 - 1;
  const ndcY = -(((clientY - hostRect.top) / hostRect.height) * 2 - 1);
  const ray = worldRayFromNdc(ndcX, ndcY, camera);
  if (!ray) return null;
  const axisOrigin = new Vector3(origin[0], origin[1], origin[2]);
  const axisDirection = new Vector3(axis[0], axis[1], axis[2]).normalize();
  const originDelta = ray.origin.clone().sub(axisOrigin);
  const a = ray.direction.dot(ray.direction);
  const b = ray.direction.dot(axisDirection);
  const c = axisDirection.dot(axisDirection);
  const d = ray.direction.dot(originDelta);
  const e = axisDirection.dot(originDelta);
  const denominator = a * c - b * b;
  if (Math.abs(denominator) < 1e-9) return null;
  return (a * e - b * d) / denominator;
}

/** 🪪️ Element id of one world surface's projection pane. A world surface is mounted once per OPEN WINDOW
 * INSTANCE of its kind (`WindowInstanceIdContext`), and {@link Pane} renders its `id` on the pane root plus two
 * derived control ids (`…pane.fold`, `…pane.foldControl`), so the bare `framework.worldOrbit.projection` it used
 * to carry put three DUPLICATE DOM ids in the document per extra pane — invalid HTML, and it makes the pane
 * unaddressable through `<label for>`, `aria-labelledby` and any automation. The instance segment is the same
 * camelCase {@link childElementId} convention the window element ids use (`…projection.puzzle3dMainPerspective`). */
export function world3dProjectionPaneElementId(windowElementSegment: string): string {
  return childElementId("framework.worldOrbit.projection", windowElementSegment);
}

/** 🔀️ Portals the world's projection-kind switch into the enclosing window's pane host (see `usePaneSlot`), defaulting to bottom-right under the navigation cube — the cube sits above the folded chrome and the unfolded pane grows over it. Falls back to a local overlay when no pane host is mounted yet (or outside one). */
export function WorldOrbitProjectionSwitchPane({ spec, onSpecChange, windowElementSegment }: { readonly spec: WorldProjectionSpec; readonly onSpecChange: (spec: WorldProjectionSpec) => void; readonly windowElementSegment: string }) {
  const [anchor, setAnchor] = useState<Anchor>("bottom-right");
  const [folded, setFolded] = useState(true);
  const projectionLabel = useLabel("ui.host.projection");
  const paneId = world3dProjectionPaneElementId(windowElementSegment);
  const pane = (
    <Pane
      id={paneId}
      anchor={anchor}
      onAnchorChange={setAnchor}
      folded={folded}
      onFoldToggle={() => setFolded((value) => !value)}
      icon={worldProjectionSpecIconId(spec) as IconName}
      label={projectionLabel}
      bodyClassName={windowMeasuresBodyClass}
      stackDataAttrs={{ "data-level": "pane" }}
      dimWhenOpen
    >
      <WorldProjectionKindSwitch id={paneId} spec={spec} onSpecChange={onSpecChange} />
    </Pane>
  );
  const portaled = usePaneSlot(pane);
  if (portaled) return portaled;
  return (
    <div className="pointer-events-none absolute inset-0" data-slot="world-orbit-projection-switch-fallback" data-world-projection-kind-switch-host="">
      {pane}
    </div>
  );
}

//#region WorldWindowInstance
export type World3dRetainedViewportV1 = Readonly<{
  camera: WorldParsedCameraState | null;
  viewportOwned: boolean;
  userMovedFitRevision: number | null;
  detachEpoch: number;
  projectionContentFrameSeeded: boolean;
  lastDispatchedCamera: WorldCameraState | null;
  observedSceneCameraJson: string;
  sceneCameraAttachJson: string;
  pendingProjectionSpec: WorldProjectionSpec | null;
}>;

/** 📷️ Shell-session owner of live per-window world viewports. */
export class World3dWindowViewStoreV1 {
  readonly #views = new Map<string, World3dRetainedViewportV1>();
  #owner: string | null = null;

  admitOwner(owner: string | null): void {
    if (this.#owner === owner) return;
    this.#owner = owner;
    this.#views.clear();
  }

  read(windowId: string | null): World3dRetainedViewportV1 | null {
    return windowId ? (this.#views.get(windowId) ?? null) : null;
  }

  write(windowId: string | null, viewport: World3dRetainedViewportV1): void {
    if (windowId) this.#views.set(windowId, viewport);
  }

  retire(windowId: string | null): void {
    if (windowId) this.#views.delete(windowId);
  }
}

/** 🪪️ Keeps independently-lived primary and spawned program viewport owners apart. */
export class World3dWindowViewRegistryV1 {
  readonly #stores = new Map<string, World3dWindowViewStoreV1>();

  scope(owner: string): World3dWindowViewStoreV1 {
    const retained = this.#stores.get(owner);
    if (retained) return retained;
    const created = new World3dWindowViewStoreV1();
    created.admitOwner(owner);
    this.#stores.set(owner, created);
    return created;
  }

  retireOwner(owner: string | null): void {
    if (owner) this.#stores.delete(owner);
  }

  retireWindow(windowId: string | null): void {
    for (const store of this.#stores.values()) store.retire(windowId);
  }
}

/** 🐚️ The current shell session's live world viewport owner. */
export const World3dWindowViewStoreContext = createContext<World3dWindowViewStoreV1 | null>(null);

/** 🪟️ One-shot initial camera pose, keyed by window instance id, consumed by {@link World3dHost} on
 * mount and then discarded — the side-channel a Display "Windows" drag-and-drop template uses to seed a
 * freshly-opened pane's projection (dragging "Top" opens a pane that starts in the Top view, etc.). */
const pendingWorldProjectionByWindowId = new Map<string, WorldProjectionSpec>();

/** 🪟️ Registers `spec` to be consumed once by the next {@link World3dHost} mounted for `windowId`. */
export function registerPendingWorldProjection(windowId: string, spec: WorldProjectionSpec): void {
  pendingWorldProjectionByWindowId.set(windowId, spec);
}

/** 🪟️ Peeks the sticky initial projection for `windowId` — kept until the pane is closed so React
 * Strict Mode's mount→unmount→remount pass still seeds both Top and Perspective (take-on-read / clear-on-apply
 * left the second mount on the shared scene camera and empty transparent canvases). */
export function peekPendingWorldProjection(windowId: string | null): WorldProjectionSpec | null {
  if (!windowId) return null;
  return pendingWorldProjectionByWindowId.get(windowId) ?? null;
}

/** 🪟️ Drops the initial projection seed when a pane is closed (not after first apply). */
export function clearPendingWorldProjection(windowId: string | null): void {
  if (!windowId) return;
  pendingWorldProjectionByWindowId.delete(windowId);
}

/** 🪦️ Retires projection seeds owned by a session that is leaving the shell. */
export function clearPendingWorldProjections(windowIds: readonly string[]): void {
  for (const windowId of windowIds) pendingWorldProjectionByWindowId.delete(windowId);
}
//#endregion WorldWindowInstance
//#endregion 🔖️World3dHost
