/** 🎲️ The board 2d surface the scenario drives: positions, selection, camera and pointer gestures. */
import { renderer, page } from "@semio-tech/framework/time-travel/testing/state";
import type { Renderer, Point } from "@semio-tech/framework/time-travel/testing/state";
import { evalSafe, sleep } from "@semio-tech/framework/time-travel/testing/page";
import { chromeHits } from "@semio-tech/framework/time-travel/testing/wgpu-mirror";
import { inside } from "@semio-tech/framework/time-travel/testing/chrome";

//#region 🔖️Arguments

export type Positions = Record<string, [number, number]>;

export const OVERVIEW = "2d-overview";

export const PLUGIN_VARIANT = "puzzle2d";

export const DEFAULT_SERVES: Readonly<Record<Renderer, string>> = { react: "http://127.0.0.1:6012/", wgpu: "http://127.0.0.1:6112/" };
//#endregion 🔖️Arguments
//#region 🔖️Wgpu

type Board2dSurface = { surfaceId: string; windowId: string; rect: [number, number, number, number]; camera: { x: number; y: number; zoom: number } | null; positions: Positions; selection: string[]; highlighted?: string[]; nodes: number; edges: number; handles: number; parsed: boolean };

/** 🎲️ The overview board as `dumpBoard2d` publishes it, or null while the renderer lacks the export (a named prerequisite). */
export const wgpuBoard = async () => {
  const json = await evalSafe(
    async (windowId) => {
      const introspection = (window as unknown as { semioWgpuIntrospection?: { dumpBoard2d?: (windowId?: string) => Promise<string> } }).semioWgpuIntrospection;
      return typeof introspection?.dumpBoard2d === "function" ? await introspection.dumpBoard2d(windowId) : null;
    },
    null as string | null,
    OVERVIEW,
  );
  if (!json) return null;
  try {
    const dump = JSON.parse(json) as { surfaces?: Board2dSurface[] };
    return dump.surfaces?.find((surface) => surface.windowId === OVERVIEW) ?? dump.surfaces?.[0] ?? null;
  } catch {
    return null;
  }
};

/** 📜️ The contract `dumpBoard2d` is asked to answer — the wgpu twin of React's Board2dHost `data-board-*` vitals. */
export const WGPU_BOARD_CONTRACT = "semioWgpuIntrospection.dumpBoard2d(windowId?) → {surfaces:[{surfaceId, windowId, rect:[x,y,w,h] (page CSS px of the board canvas), camera:{x,y,zoom}, positions:{nodeId:[x,y]} (the published fixture), selection:[id], highlighted:[id] (`highlighted_ids_json`, the ids an open time-travel draft references — React `data-board-highlighted-ids-json`), nodes, edges, handles, parsed}]} — read-only, from the Board2d scene the frame worker already holds (snapshot_json, camera_json, selection_json, highlighted_ids_json)";
//#endregion 🔖️Wgpu
//#region 🔖️Board

type Vitals = { surface: string; nodes: number; edges: number; handles: number; parsed: string; selection: string; camera: string; transform: string; utility: string; highlighted: string | null };

/** 🩻️ The overview pane's board vitals, read without a guest round trip: React's `data-board-*` attributes, wgpu's
 * `dumpBoard2d`. */
export const vitals = async (): Promise<Vitals | null> => {
  if (renderer === "wgpu") {
    const board = await wgpuBoard();
    return board ? { surface: board.surfaceId, nodes: board.nodes, edges: board.edges, handles: board.handles, parsed: String(board.parsed), selection: JSON.stringify(board.selection), camera: JSON.stringify(board.camera), transform: "", utility: "", highlighted: board.highlighted ? JSON.stringify(board.highlighted) : null } : null;
  }
  return reactVitals();
};

/** 💓️ React's Board2dHost `data-board-*` vitals of the overview pane. */
const reactVitals = () =>
  evalSafe(
    (surface) => {
      const el = document.querySelector(`[data-surface-id="${surface}"]`);
      if (!el) return null;
      return {
        surface,
        nodes: Number(el.getAttribute("data-board-nodes") ?? "-1"),
        edges: Number(el.getAttribute("data-board-edges") ?? "-1"),
        handles: Number(el.getAttribute("data-board-handles") ?? "-1"),
        parsed: el.getAttribute("data-board-snapshot-parsed") ?? "",
        selection: el.getAttribute("data-board-selection-json") ?? "",
        camera: el.getAttribute("data-board-camera-json") ?? "",
        transform: el.getAttribute("data-board-transform-json") ?? "",
        utility: el.getAttribute("data-board-active-utility") ?? "",
        highlighted: el.getAttribute("data-board-highlighted-ids-json"),
      };
    },
    null as Vitals | null,
    `window:${OVERVIEW}`,
  );

/** 📍️ Every node position the overview pane publishes (React `data-board-positions-json`, wgpu `dumpBoard2d`). */
export const positions = async (): Promise<Positions> => (renderer === "wgpu" ? ((await wgpuBoard())?.positions ?? {}) : reactPositions());

const reactPositions = async () => JSON.parse(await evalSafe((surface) => document.querySelector(`[data-surface-id="${surface}"]`)?.getAttribute("data-board-positions-json") ?? "{}", "{}", `window:${OVERVIEW}`)) as Positions;

export const selectionIds = (v: Vitals | null) => {
  try {
    return JSON.parse(v?.selection || "[]") as string[];
  } catch {
    return [] as string[];
  }
};

/** 🔦️ The ids the board highlights for an open draft's references, or null while the renderer publishes none. */
export const highlightedIds = (v: Vitals | null) => {
  if (v?.highlighted === null || v?.highlighted === undefined) return null;
  try {
    return JSON.parse(v.highlighted || "[]") as string[];
  } catch {
    return null;
  }
};

export const cameraOf = (v: Vitals | null) => {
  try {
    const camera = (JSON.parse(v?.camera || "{}") ?? {}) as { x?: number; y?: number; zoom?: number };
    return { x: camera.x ?? 0, y: camera.y ?? 0, zoom: camera.zoom && camera.zoom > 0 ? camera.zoom : 1 };
  } catch {
    return { x: 0, y: 0, zoom: 1 };
  }
};

/** 🪞 The overview board's rect in page CSS px. React: the pane's canvas. wgpu: the chrome hit registry's `ScrollRegion` of the
 * window (measured on B2w: [6.4, 60.8, 787.2, 907.2]) — `dumpBoard2d().rect` is window-local ([0, 0, w, h]), so aiming with
 * it lands 6 px left of and 61 px above the node. */
export const paneBox = async () => {
  if (renderer === "wgpu") {
    const region = ((await chromeHits()).hits ?? []).find((hit) => hit.kind === "ScrollRegion" && hit.controlId === OVERVIEW)?.rect;
    const rect = region ?? (await wgpuBoard())?.rect;
    return rect ? { x: rect[0], y: rect[1], width: rect[2], height: rect[3] } : { x: 0, y: 0, width: 1, height: 1 };
  }
  return (await page.locator(`[data-surface-id="window:${OVERVIEW}"] canvas`).first().boundingBox().catch(() => null)) ?? { x: 0, y: 0, width: 1, height: 1 };
};

/** 📌️ World → screen through the published camera: the pane centre is the camera position. */
export const toScreen = (world: [number, number], camera: { x: number; y: number; zoom: number }, box: { x: number; y: number; width: number; height: number }): Point => ({
  x: box.x + box.width / 2 + (world[0] - camera.x) * camera.zoom,
  y: box.y + box.height / 2 + (world[1] - camera.y) * camera.zoom,
});

/** 🗺️ The screen layout of every node: its point, whether it is inside the pane and its nearest neighbour in pixels. */
export const layout = async () => {
  const [v, p, box] = [await vitals(), await positions(), await paneBox()];
  const camera = cameraOf(v);
  const points = Object.entries(p).map(([id, world]) => ({ id, world, at: toScreen(world, camera, box) }));
  const rows = points.map((row) => ({ ...row, clearance: Math.min(...points.filter((other) => other.id !== row.id).map((other) => Math.hypot(other.at.x - row.at.x, other.at.y - row.at.y)), 1e9) }));
  return { camera, box, rows };
};

/** 🔭️ Zooms out at the pane centre until at least `minimum` nodes are inside the pane with some clearance. */
export const frameBoard = async (minimum: number) => {
  for (let attempt = 0; attempt < 6; attempt++) {
    const { box, rows } = await layout();
    const usable = rows.filter((row) => inside(row.at, box, 90) && row.clearance >= 14);
    if (usable.length >= minimum) return usable.length;
    await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
    await page.mouse.wheel(0, 300);
    await sleep(1200);
  }
  return 0;
};

/** 🍒️ Picks `count` distinct nodes that are inside the pane, clickable (≥ 14 px from any other node), whose
 * `offsets` (world units) keep them inside the pane, and that are ≥ `spread` px apart from each other. */
export const pickNodes = async (count: number, offsets: [number, number][], exclude: string[], spread = 40) => {
  const { camera, box, rows } = await layout();
  const picked: typeof rows = [];
  const candidates = rows
    .filter((row) => !exclude.includes(row.id) && row.clearance >= 14 && inside(row.at, box, 90))
    .filter((row) => offsets.every(([dx, dy]) => inside({ x: row.at.x + dx * camera.zoom, y: row.at.y + dy * camera.zoom }, box, 60)))
    .sort((a, b) => b.clearance - a.clearance);
  for (const row of candidates) {
    if (picked.length === count) break;
    if (picked.every((other) => Math.hypot(other.at.x - row.at.x, other.at.y - row.at.y) >= spread && Math.hypot(other.at.x - row.at.x, other.at.y - row.at.y) <= 420)) picked.push(row);
  }
  return { camera, box, picked };
};

/** 🧬️ A node to duplicate whose clone (`duplicate_selection_in_fixture` offsets it by +24,+24 world units) and a drag
 * destination for that clone are both ≥ 55 world units from every other node — the press on the clone lands on the
 * clone, never on a neighbour's free handle. Zooms out until one is inside the pane. */
export const pickCloneSource = async (exclude: string[]) => {
  const moves: [number, number][] = [[70, 70], [90, 0], [0, 90], [-90, 0], [0, -90], [70, -70], [-70, 70], [-70, -70]];
  for (let attempt = 0; attempt < 6; attempt++) {
    const { camera, box, rows } = await layout();
    const clear = (world: [number, number], skip: string[]) => rows.every((other) => skip.includes(other.id) || Math.hypot(other.world[0] - world[0], other.world[1] - world[1]) >= 55);
    for (const row of rows) {
      if (exclude.includes(row.id) || row.clearance < 14 || !inside(row.at, box, 90)) continue;
      const clone: [number, number] = [row.world[0] + 24, row.world[1] + 24];
      if (!clear(clone, [row.id])) continue;
      const move = moves.find(([dx, dy]) => {
        const destination: [number, number] = [clone[0] + dx, clone[1] + dy];
        return clear(destination, []) && Math.hypot(destination[0] - row.world[0], destination[1] - row.world[1]) >= 55 && inside(toScreen(destination, camera, box), box, 60);
      });
      if (move) return { row, move, zoom: camera.zoom };
    }
    await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
    await page.mouse.wheel(0, 300);
    await sleep(1200);
  }
  return null;
};

/** 🖐️ A press–move–release of `steps` pointer moves from `from` by `(dx, dy)` screen pixels. */
export const dragBy = async (from: Point, dx: number, dy: number, steps = 12) => {
  await page.mouse.move(from.x, from.y);
  await page.mouse.down();
  for (let index = 1; index <= steps; index++) await page.mouse.move(from.x + (dx * index) / steps, from.y + (dy * index) / steps);
  await page.mouse.up();
};

/** 🧮️ `after[id] - before[id]`, or null when either is missing. */
export const offsetOf = (before: Positions, after: Positions, id: string): [number, number] | null => (before[id] && after[id] ? [after[id][0] - before[id][0], after[id][1] - before[id][1]] : null);

export const near = (a: number, b: number, eps: number) => Math.abs(a - b) <= eps;

/** 📐️ Whether `id` sits at `origin + (dx, dy)` within `eps`. */
export const placed = (p: Positions, id: string, origin: [number, number] | undefined, dx: number, dy: number, eps = 0.02) => Boolean(origin && p[id] && near(p[id][0], origin[0] + dx, eps) && near(p[id][1], origin[1] + dy, eps));

/** 🚩️ The ids whose positions differ between two readings (beyond `eps`), plus ids present in only one. */
export const movedIds = (a: Positions, b: Positions, eps = 1e-6) => {
  const ids = new Set([...Object.keys(a), ...Object.keys(b)]);
  return [...ids].filter((id) => !a[id] || !b[id] || !near(a[id][0], b[id][0], eps) || !near(a[id][1], b[id][1], eps));
};
//#endregion 🔖️Board
