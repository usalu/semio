/** 🫱️ The interaction ledger: which selection and hover marks a surface has already published to the plugin.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | NodeGraph}, which re-exports every public name below so import specifiers stay unchanged.
 */

import type { NodeGraphHover } from "@semio-tech/framework";

//#region 🔖️NodeGraphHost
//#region 🔖️flow-graph-canvas-host
//#region FlowGraphCanvasHost
//#region 🫱️InteractionPublication
/** 🫱️ What a surface can tell the plugin it selected. */
export type NodeGraphSelectionMarks = Readonly<{
  readonly nodeIds: readonly string[];
  readonly edgeIds?: readonly string[];
  readonly handleIds?: readonly string[];
}>;

/** 🫱️ What a surface can tell the plugin the pointer is over. */
export type NodeGraphHoverMark = Readonly<{ readonly hoveredId?: string; readonly portId?: string; readonly portDirection?: "in" | "out"; readonly portIds?: readonly string[]; readonly edgeId?: string }>;

/** 🔑️ The exact selection a publication would carry, as one comparable value. Order is the domain's
 * own — the guest answers its ids in its own order and the plugin stores them that way, so a reorder
 * IS a different mark. */
export function nodeGraphSelectionMarkKey(marks: NodeGraphSelectionMarks): string {
  return JSON.stringify([marks.nodeIds, marks.edgeIds ?? [], marks.handleIds ?? []]);
}

/** 🔑️ The exact hover a publication would carry, as one comparable value. */
export function nodeGraphHoverMarkKey(mark: NodeGraphHoverMark): string {
  return JSON.stringify([mark.hoveredId, mark.portId, mark.portDirection, mark.edgeId, mark.portIds]);
}

/** 🧾️ What this surface has last told the plugin about its interaction marks, and therefore which
 * lanes a new reading actually OWES it.
 *
 * `emitInteractionState` published selection AND hover on every non-pan pointer-up, including a plain
 * click that changed neither — and the same pointer-up reaches it twice (the pick hook's
 * `onSelectTarget` and the surface's own `onPointerUp`), so one click on a node cost four guest
 * invocations and one click on empty canvas cost two. Each is a `performInvocation` → `refreshUi` →
 * React commit over the whole interaction scope (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️interaction-scope-narrowing-2026-09-15.md` §5).
 *
 * `adopt*` is the other half: marks that arrive FROM the plugin on a scene are what the plugin
 * already holds, so they set the baseline without owing a hop — and a plugin-side change (a keyboard
 * `selectAll`, an outline pick) can never be shadowed by a stale note of what we last sent. */
export function createNodeGraphInteractionLedger(windowInstanceId?: string | null) {
  let selection: string | null = null;
  const emptyHover = nodeGraphHoverMarkKey({});
  let hover: string | null = null;
  let sceneHover: string | undefined;
  let sceneHoverSource: string | null | undefined;
  let localOwnsHover = false;
  let sceneOwnsHover = false;
  let sceneHoverRevision = 0;
  return {
    publishSelection(marks: NodeGraphSelectionMarks): boolean {
      const key = nodeGraphSelectionMarkKey(marks);
      if (key === selection) return false;
      selection = key;
      return true;
    },
    publishHover(mark: NodeGraphHoverMark, revision = sceneHoverRevision): boolean {
      if (revision !== sceneHoverRevision) return false;
      const key = nodeGraphHoverMarkKey(mark);
      if (key === hover) return false;
      hover = key;
      localOwnsHover = true;
      sceneOwnsHover = false;
      return true;
    },
    adoptSelection(marks: NodeGraphSelectionMarks): void {
      selection = nodeGraphSelectionMarkKey(marks);
    },
    beginLocalHover(): void {
      if (sceneOwnsHover && sceneHoverSource !== undefined) hover = null;
      localOwnsHover = true;
      sceneOwnsHover = false;
    },
    get sceneHoverRevision(): number {
      return sceneHoverRevision;
    },
    reconcileSceneHover(next: NodeGraphHover | undefined): NodeGraphHoverMark | undefined {
      const mark: NodeGraphHoverMark = next?.edgeId
        ? { edgeId: next.edgeId }
        : next?.nodeId
          ? { hoveredId: next.nodeId, portId: next.portId ?? undefined, ...(next.portIds?.length ? { portIds: next.portIds } : {}) }
          : {};
      const key = next === undefined ? undefined : nodeGraphHoverMarkKey(mark);
      const source = next?.hoverSourceWindowId;
      if (key === sceneHover && source === sceneHoverSource) return undefined;
      sceneHover = key;
      sceneHoverSource = source;
      const ownEcho = typeof source === "string" && source === windowInstanceId;
      if ((ownEcho && localOwnsHover) || (key === undefined && !sceneOwnsHover)) return undefined;
      const foreign = source !== undefined && !ownEcho;
      if (key === hover && !foreign) return undefined;
      hover = key ?? emptyHover;
      localOwnsHover = false;
      sceneOwnsHover = hover !== emptyHover;
      sceneHoverRevision += 1;
      return mark;
    },
    retire(): void {
      selection = null;
      hover = null;
      sceneHover = undefined;
      sceneHoverSource = undefined;
      localOwnsHover = false;
      sceneOwnsHover = false;
      sceneHoverRevision += 1;
    },
  };
}
//#endregion 🫱️InteractionPublication
//#endregion FlowGraphCanvasHost
//#endregion 🔖️flow-graph-canvas-host
//#endregion 🔖️NodeGraphHost
