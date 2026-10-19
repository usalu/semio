/** 🔬️ Board2dHost parsing: the board camera, event row and drop payload types, the vitals, kind domain and hover helpers, the status json, the float32 decimal, the camera, catalogue drag, transform flag, brush size and granularity parsers, the hover and camera action args and the suggestion menu shapes. */
import { type ContextMenuItemSpec } from "@semio-tech/framework";

//#region 🔖️Board2dHost
//#region Types
export type BoardCamera = { readonly x: number; readonly y: number; readonly zoom: number };
export type BoardEventRow = { readonly name: string; readonly payload?: unknown };
export type BoardDropPayload = {
  readonly kindId: string;
  readonly catalogSlice: string;
  readonly shape?: string;
  readonly radius?: number;
  readonly width?: number;
  readonly height?: number;
  readonly iconKind?: string;
};

//#endregion Types
//#region Parsing
/** 🩺️ The document vitals this host publishes as `data-board-*` attributes on its container — node/edge counts
 * and every node's world position — so a headless probe (and the shell's own tests) can read what the guest last
 * painted without a guest round trip, the board twin of `World3dHost`'s `data-instances-json`. */
export function board2dVitals(snapshotJson: string): { readonly nodes: number; readonly edges: number; readonly handles: number; readonly positionsJson: string } {
  try {
    const snapshot = JSON.parse(snapshotJson) as { nodes?: { id?: string; x?: number; y?: number; handles?: unknown[] }[]; edges?: unknown[] };
    const positions: Record<string, [number, number]> = {};
    let handles = 0;
    for (const node of snapshot.nodes ?? []) {
      if (typeof node.id === "string" && typeof node.x === "number" && typeof node.y === "number") positions[node.id] = [node.x, node.y];
      handles += node.handles?.length ?? 0;
    }
    return { nodes: snapshot.nodes?.length ?? 0, edges: snapshot.edges?.length ?? 0, handles, positionsJson: JSON.stringify(positions) };
  } catch {
    return { nodes: -1, edges: -1, handles: -1, positionsJson: "{}" };
  }
}

/** 🗂️ The kind ids this board knows, by the engine's own hover domain (`resolveElementKindHover`
 * answers `"node" | "handle" | "edge" | "wire"`). Read straight off the scene's kind catalogs, so the
 * host hard-codes no panel's id scheme and no owning plugin. */
export function board2dKindDomainById(glyphCatalogsJson: string): ReadonlyMap<string, string> {
  const out = new Map<string, string>();
  let catalogs: unknown;
  try {
    catalogs = JSON.parse(glyphCatalogsJson);
  } catch {
    return out;
  }
  if (typeof catalogs !== "object" || catalogs === null) return out;
  const slices: readonly (readonly [string, string])[] = [
    ["nodeKinds", "node"],
    ["handleKinds", "handle"],
    ["edgeKinds", "edge"],
    ["wireKinds", "wire"],
  ];
  for (const [key, domain] of slices) {
    const rows = (catalogs as Record<string, unknown>)[key];
    if (!Array.isArray(rows)) continue;
    for (const row of rows) {
      const id = (row as { readonly id?: unknown })?.id;
      if (typeof id === "string" && id.length > 0 && !out.has(id)) out.set(id, domain);
    }
  }
  return out;
}

/** 🖱️ Resolves a hovered chrome element's DOM id to `(domain, kindId)`. A catalogue row is named
 * `<sectionId>.<kindId>` (for example `catalog-kinds.nodes.beam`), so the trailing dot segment — or
 * the whole id, for a bare-kind row — is the candidate, and it counts only when this
 * board's OWN catalogs know it. That keeps the host generic: no panel id prefix, no plugin name. */
export function board2dKindHoverFromElementId(elementId: string | null | undefined, kindDomainById: ReadonlyMap<string, string>): { readonly domain: string; readonly kindId: string } | null {
  if (!elementId) return null;
  for (const candidate of [elementId.slice(elementId.lastIndexOf(".") + 1), elementId]) {
    const domain = kindDomainById.get(candidate);
    if (domain) return { domain, kindId: candidate };
  }
  return null;
}

/** 🩺️ The board's boot/sync verdict as one probe row — whether the last snapshot parsed, how big it
 * was, why it was refused, how many drained rows are still waiting for a flush, and which guest scene
 * revision this pane last applied. The board twin of `World3dHost`'s `data-status-json`. */
export type Board2dStatus = {
  snapshotParsed: boolean | null;
  snapshotChars: number;
  refusalReason: string;
  pendingEvents: number;
  guestRevision: number;
};

export function board2dStatusJson(status: Board2dStatus): string {
  return JSON.stringify(status);
}

/** ✂️ `value` as the shortest decimal that round-trips its f32. Browsers report pointer coordinates in f32, so the
 * digits below that precision are representation noise a recorded drag offset must never inherit; the board engine
 * records offsets in the same form (`board_pointer_offset`), both replaying the `🧫️float32-decimal` corpus. */
export function board2dFloat32Decimal(value: number): number {
  const narrowed = Math.fround(value);
  if (!Number.isFinite(narrowed)) return value;
  for (let digits = 1; digits <= 9; digits += 1) {
    const candidate = Number(narrowed.toPrecision(digits));
    if (Math.fround(candidate) === narrowed) return candidate;
  }
  return narrowed;
}

export function parseBoardCamera(json: string): BoardCamera | null {
  try {
    const parsed = JSON.parse(json) as Partial<BoardCamera>;
    if (typeof parsed.x !== "number" || typeof parsed.y !== "number" || typeof parsed.zoom !== "number") return null;
    return { x: parsed.x, y: parsed.y, zoom: parsed.zoom };
  } catch {
    return null;
  }
}

export function board2dCameraActionArgs(cameraJson: string): { readonly camera: BoardCamera } | null {
  const camera = parseBoardCamera(cameraJson);
  return camera ? { camera } : null;
}

export function parseBoardCatalogueDragPayload(encoded: string | null | undefined): BoardDropPayload | null {
  if (!encoded) return null;
  try {
    const parsed = JSON.parse(encoded) as Partial<BoardDropPayload>;
    if (typeof parsed.kindId !== "string") return null;
    return {
      kindId: parsed.kindId,
      catalogSlice: typeof parsed.catalogSlice === "string" ? parsed.catalogSlice : "nodes",
      shape: typeof parsed.shape === "string" ? parsed.shape : undefined,
      radius: typeof parsed.radius === "number" ? parsed.radius : undefined,
      width: typeof parsed.width === "number" ? parsed.width : undefined,
      height: typeof parsed.height === "number" ? parsed.height : undefined,
      iconKind: typeof parsed.iconKind === "string" ? parsed.iconKind : undefined,
    };
  } catch {
    return null;
  }
}
/** 🕹️ Reads `Board2dScene.transformFlags` (`{"move":boolean,"rotate":boolean}`); anything missing
 * or malformed leaves both handles on rather than silently disarming the gumball. */
export function parseBoard2dTransformFlags(encoded: string | null | undefined): { readonly move: boolean; readonly rotate: boolean } {
  if (!encoded) return { move: true, rotate: true };
  try {
    const parsed = JSON.parse(encoded) as { move?: unknown; rotate?: unknown };
    return { move: typeof parsed.move === "boolean" ? parsed.move : true, rotate: typeof parsed.rotate === "boolean" ? parsed.rotate : true };
  } catch {
    return { move: true, rotate: true };
  }
}
/** 🖍️ Reads `Board2dScene.areaBrushSize` (`{"width":number,"height":number}`). A missing or
 * malformed payload answers `0`, which the engine reads as "keep the extent you have" — an area brush
 * that silently collapsed to a zero-area rectangle would paint regions no fill placement can satisfy. */
export function parseBoard2dAreaBrushSize(encoded: string | null | undefined): { readonly width: number; readonly height: number } {
  if (!encoded) return { width: 0, height: 0 };
  try {
    const parsed = JSON.parse(encoded) as { width?: unknown; height?: unknown };
    return { width: typeof parsed.width === "number" && parsed.width > 0 ? parsed.width : 0, height: typeof parsed.height === "number" && parsed.height > 0 ? parsed.height : 0 };
  } catch {
    return { width: 0, height: 0 };
  }
}
/** 🐁️ Classifies every entity id the snapshot carries into the `vortex`-domain granularity a
 * pick or hover reports it under — the client twin of the owning guest's selection-target classifier. A
 * `node:handle` id nested under a node is a `handle`, an id in `edges` is an `edge`, everything else
 * (including an id the document does not carry yet) is a `node`, so a just-painted entity is never
 * dropped on the way to the framework. */
export function board2dGranularityById(snapshotJson: string): ReadonlyMap<string, string> {
  const byId = new Map<string, string>();
  try {
    const snapshot = JSON.parse(snapshotJson) as { nodes?: { id?: unknown; handles?: { id?: unknown }[] }[]; edges?: { id?: unknown }[] };
    for (const node of snapshot.nodes ?? []) {
      if (typeof node.id === "string") byId.set(node.id, "node");
      for (const handle of node.handles ?? []) if (typeof handle.id === "string") byId.set(handle.id, "handle");
    }
    for (const edge of snapshot.edges ?? []) if (typeof edge.id === "string") byId.set(edge.id, "edge");
  } catch {}
  return byId;
}

/** 🐁️ The `interactionHover` wire shape — mirrors `world3dHoverActionArgs`, so one id and one
 * granularity is all a board pointermove costs. An empty `targets` clears the domain's hover. */
export function board2dHoverActionArgs(domainId: string, granularity: string, id: string | null | undefined) {
  return { domainId, channel: "pointer", targets: JSON.stringify(id ? [{ granularity, id }] : []) };
}

/** 🐁️ The LAST `hover` row of a drained batch — the engine's live answer; `undefined` when the
 * batch carries none (leave the current hover alone), `null` when the pointer left every entity. */
export function latestBoard2dHoverId(rows: readonly BoardEventRow[]): string | null | undefined {
  let hovered: string | null | undefined;
  for (const row of rows) {
    if (row.name !== "hover") continue;
    const id = (row.payload as { readonly id?: unknown } | undefined)?.id;
    hovered = typeof id === "string" && id.length > 0 ? id : null;
  }
  return hovered;
}

/** 💡️ One placement candidate row of the handle-suggestions popup. */
export type Board2dSuggestionCandidate = {
  readonly index: number;
  /** 🔦️ The candidate's key in the board's `toolRunTrace` lane — hovering its row focuses exactly that placement.
   * Decimal text when it exceeds 2^53 (a planner twin key sets bit 62). */
  readonly key?: number | string;
  readonly nodeLabel: string;
  readonly handleLabel: string;
  readonly icon?: string;
  readonly color?: string;
};

/** 💡️ The open handle-suggestions popup the guest published, or `null` when this board has none.
 * `pending` means the slot has not resolved yet; an empty `candidates` on a resolved slot is the polite
 * refusal a document with no free handle (Nakagin) gives. */
export type Board2dSuggestionMenu = {
  readonly open: boolean;
  readonly x: number;
  readonly y: number;
  readonly windowId?: string;
  readonly handleId?: string;
  readonly hoveredIndex: number;
  /** 📂️ The list lives inside the regular context menu as the "suggest" row's submenu instead of floating as its own popup. */
  readonly submenu: boolean;
  readonly pending: boolean;
  readonly candidates: readonly Board2dSuggestionCandidate[];
};

export function parseBoard2dSuggestionMenu(encoded: string | null | undefined): Board2dSuggestionMenu | null {
  if (!encoded) return null;
  try {
    const parsed = JSON.parse(encoded) as Partial<Board2dSuggestionMenu>;
    if (parsed.open !== true) return null;
    const candidates = Array.isArray(parsed.candidates)
      ? parsed.candidates.filter((candidate): candidate is Board2dSuggestionCandidate => typeof candidate?.index === "number" && typeof candidate?.nodeLabel === "string")
      : [];
    return {
      open: true,
      x: typeof parsed.x === "number" ? parsed.x : 0,
      y: typeof parsed.y === "number" ? parsed.y : 0,
      windowId: typeof parsed.windowId === "string" && parsed.windowId.length > 0 ? parsed.windowId : undefined,
      handleId: typeof parsed.handleId === "string" && parsed.handleId.length > 0 ? parsed.handleId : undefined,
      hoveredIndex: typeof parsed.hoveredIndex === "number" ? parsed.hoveredIndex : 0,
      submenu: parsed.submenu === true,
      pending: parsed.pending === true,
      candidates,
    };
  } catch {
    return null;
  }
}

/** 🪟️ True when THIS pane owns the open popup — a menu naming no window is owned by whichever
 * pane renders it, exactly like `worldSuggestionMenuOwnsWindow`. */
export function board2dSuggestionMenuOwnsWindow(menu: Board2dSuggestionMenu | null, windowInstanceId: string | undefined): boolean {
  if (!menu?.open) return false;
  return !menu.windowId || menu.windowId === windowInstanceId;
}

/** 💡️ The popup's rows: hovering one PREVIEWS it (`hoverSuggestion`), clicking one places it
 * (`acceptSuggestion`). `closeOnSelect={false}` plus these two actions is what makes the preview a
 * "just looking" state distinct from the commit. */
export function board2dSuggestionMenuItems(menu: Board2dSuggestionMenu, labels: { readonly checkingPlacement: string; readonly noPlacement: string }): ContextMenuItemSpec[] {
  if (menu.pending) return [{ id: "pending", label: labels.checkingPlacement, disabled: true }];
  if (menu.candidates.length === 0) return [{ id: "empty", label: labels.noPlacement, disabled: true }];
  return menu.candidates.map((candidate) => ({
    id: `suggestion-${candidate.index}`,
    label: `${candidate.nodeLabel} · ${candidate.handleLabel}`,
    icon: candidate.icon ?? "circle-dot",
    checked: candidate.index === menu.hoveredIndex,
    action: "acceptSuggestion",
    args: { index: candidate.index, ...(menu.handleId ? { handleId: menu.handleId } : {}) },
    hoverAction: "hoverSuggestion",
    hoverArgs: { index: candidate.index, ...(menu.handleId ? { handleId: menu.handleId } : {}) },
  }));
}
//#endregion Parsing
//#endregion 🔖️Board2dHost
