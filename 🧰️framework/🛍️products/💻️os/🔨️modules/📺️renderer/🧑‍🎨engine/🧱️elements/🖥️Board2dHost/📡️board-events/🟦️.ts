/** 📡️ Board2dHost events: the transient and flush-now event names, the gesture id and row camera, the event coalescing and the live mirror mutations collected from the board events. */
import type { BoardCamera, BoardEventRow } from "../🔬️board-parsing/🟦️.ts";

//#region 🔖️Board2dHost
//#region BoardEvents
// 🐁️ `hover` is the highest-frequency row the engine emits and it is NOT an `applyBoardEvents` payload:
// it travels on the framework's own `interactionHover` lane through {@link latestBoard2dHoverId}, so a
// pointermove never queues a retained board-events job. `nodeMove` and `transformPreview` are live-preview
// frames for sibling panes only: a finished gesture is ONE `gesture` record, never its frames.
const BOARD_TRANSIENT_EVENT_NAMES = new Set(["nodeMove", "transformPreview", "preselect", "brushPreview", "linkCompatibleNodes", "linkTargetRing", "hover"]);
const BOARD_FLUSH_NOW_EVENT_NAMES = new Set(["gesture", "select", "preselectCancel", "brushCandidates", "brushPlace", "edgeCreate", "edgeDelete", "nodeDelete", "regionCreate", "regionResize"]);

/** 🪪️ The `gestureId` a `select` or `gesture` row carries, `undefined` for an untagged row. */
function board2dGestureId(row: BoardEventRow): string | undefined {
  const id = (row.payload as { readonly gestureId?: unknown } | undefined)?.gestureId;
  return typeof id === "string" ? id : undefined;
}

/** 🎥️ The camera a `camera` row carries; `null` for a row that carries none. */
function board2dRowCamera(row: BoardEventRow): BoardCamera | null {
  const payload = row.payload as Partial<BoardCamera> | undefined;
  return typeof payload?.x === "number" && typeof payload.y === "number" && typeof payload.zoom === "number" ? { x: payload.x, y: payload.y, zoom: payload.zoom } : null;
}

/**
 * 📬️ Drops transient rows, takes the latest `camera` out of the batch, keeps every other row in order, and flags
 * whether the buffer should flush now: any terminal row does, except a `select` tagged with a `gestureId`
 * whose `gesture` record the batch does not carry — an open gesture's selection leaves with its record, never
 * mid-gesture. The camera is view state: it leaves on the view lane (`setCamera`) and never inside
 * `applyBoardEvents`, so a pan is never a command row. The shared corpus
 * `🧪️tests/🧫️fixtures/🧫️board-event-coalescing/🔣️.json` pins this and the wgpu twin `coalesce_owned_board_events`.
 */
export function coalesceBoard2dEvents(rows: readonly BoardEventRow[]): { readonly flushNow: boolean; readonly eventsJson: string; readonly camera: BoardCamera | null } {
  const recorded = new Set(rows.filter((row) => row.name === "gesture").map(board2dGestureId));
  let flushNow = false;
  let camera: BoardCamera | null = null;
  const rest: BoardEventRow[] = [];
  for (const row of rows) {
    if (BOARD_TRANSIENT_EVENT_NAMES.has(row.name)) continue;
    if (row.name === "camera") {
      camera = board2dRowCamera(row) ?? camera;
      continue;
    }
    const gestureId = row.name === "select" ? board2dGestureId(row) : undefined;
    if (BOARD_FLUSH_NOW_EVENT_NAMES.has(row.name) && (gestureId === undefined || recorded.has(gestureId))) flushNow = true;
    rest.push(row);
  }
  return { flushNow, eventsJson: JSON.stringify(rest), camera };
}

/** 🐢️ Live cross-pane mirror payload extracted from a batch of freshly-drained rows — positions/selection/preselect only, everything else (camera, brush/link chrome, hover) stays pane-local. */
export type BoardLiveMirrorMutations = {
  readonly positions: readonly { readonly id: string; readonly x: number; readonly y: number }[];
  readonly selectionIds: readonly string[] | null;
  readonly preselect: { readonly ids: readonly string[]; readonly removedIds: readonly string[] } | null;
  readonly clearPreselect: boolean;
};

function stringArray(value: unknown): readonly string[] {
  return Array.isArray(value) ? value.filter((entry): entry is string => typeof entry === "string") : [];
}

/**
 * 🐢️ Classifies a batch of raw board-event rows (as seen straight off `drainEventsJson`, before
 * the transient-event filter/coalescer runs) into the subset worth mirroring imperatively into sibling
 * panes: latest node position per id (from `nodeMove` drag frames and `transformPreview` rotate frames), and the
 * live selection/preselect state (`select`/`preselectCancel` commit or restore selection and clear
 * preselect; `preselect` sets the live marquee highlight). Multiple rows of the same kind in one batch
 * collapse to the latest.
 */
export function collectBoardLiveMirrorMutations(rows: readonly BoardEventRow[]): BoardLiveMirrorMutations {
  const positionsById = new Map<string, { readonly id: string; readonly x: number; readonly y: number }>();
  let selectionIds: readonly string[] | null = null;
  let preselect: { readonly ids: readonly string[]; readonly removedIds: readonly string[] } | null = null;
  let clearPreselect = false;

  for (const row of rows) {
    const payload = row.payload as Record<string, unknown> | undefined;
    switch (row.name) {
      case "nodeMove": {
        const id = payload?.id;
        const x = payload?.x;
        const y = payload?.y;
        if (typeof id === "string" && typeof x === "number" && typeof y === "number") positionsById.set(id, { id, x, y });
        break;
      }
      case "transformPreview": {
        const moves = payload?.moves;
        if (!Array.isArray(moves)) break;
        for (const move of moves as readonly Record<string, unknown>[]) {
          const id = move.id;
          const x = move.x;
          const y = move.y;
          if (typeof id === "string" && typeof x === "number" && typeof y === "number") positionsById.set(id, { id, x, y });
        }
        break;
      }
      case "preselect": {
        preselect = { ids: stringArray(payload?.ids), removedIds: stringArray(payload?.removedIds) };
        clearPreselect = false;
        break;
      }
      case "preselectCancel": {
        selectionIds = stringArray(payload?.ids);
        preselect = null;
        clearPreselect = true;
        break;
      }
      case "select": {
        selectionIds = stringArray(payload?.ids);
        preselect = null;
        clearPreselect = true;
        break;
      }
      default:
        break;
    }
  }

  return { positions: [...positionsById.values()], selectionIds, preselect, clearPreselect };
}
//#endregion BoardEvents
//#endregion 🔖️Board2dHost
