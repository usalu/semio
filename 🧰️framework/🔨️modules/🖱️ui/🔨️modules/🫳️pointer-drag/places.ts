/** 📍️ Drop places of a drag: which places a dragged element can be dropped on, which one the pointer is over, where the travelling copy lies and when a press becomes a drag.
 *
 * Pure geometry and policy shared by every drag of the design system; the cases in `places.cases.json` are language neutral.
 * @see ./places.schema.json */

/** 📏️ Pointer travel in CSS pixels before a press becomes a drag. */
export const DRAG_START_THRESHOLD_PX = 4;

/** 🎨️ Share in percent of the active colour that tints a place: every place the element can be dropped on, and the one under the pointer. */
export const DROP_TINT_PERCENT = { available: 10, active: 32 } as const;

/** 🔲️ An axis aligned rectangle in viewport pixels. */
export interface DropRect {
  readonly left: number;
  readonly top: number;
  readonly right: number;
  readonly bottom: number;
}

/** 🎯️ One place the dragged element can be dropped on. */
export interface DropPlace {
  readonly id: string;
  readonly rect: DropRect;
}

/** 🚦️ A place is `available` while the element travels and `active` while the pointer is over it. */
export type DropPlaceState = "available" | "active";

/** 📍️ A point in viewport pixels. */
export interface DragPoint {
  readonly x: number;
  readonly y: number;
}

const area = ({ rect }: DropPlace): number => (rect.right - rect.left) * (rect.bottom - rect.top);

const contains = ({ rect }: DropPlace, x: number, y: number): boolean => x >= rect.left && x <= rect.right && y >= rect.top && y <= rect.bottom;

/** 🎯️ The place under the point: the smallest one that contains it, of equal ones the later (the one painted on top). */
export function dropPlaceAt(places: readonly DropPlace[], x: number, y: number): string | null {
  let best: DropPlace | null = null;
  for (const place of places) if (contains(place, x, y) && (best === null || area(place) <= area(best))) best = place;
  return best?.id ?? null;
}

/** 🚦️ The state of every place for a pointer at the point, in the order of `places`. */
export function dropPlaceStates(places: readonly DropPlace[], x: number, y: number): readonly { readonly id: string; readonly state: DropPlaceState }[] {
  const active = dropPlaceAt(places, x, y);
  return places.map(({ id }) => ({ id, state: id === active ? "active" : "available" }));
}

/** 👻️ Offset in CSS pixels of the travelling copy from the pointer, so the copy never hides the place under the pointer. */
export const DRAG_GHOST_OFFSET: DragPoint = { x: 8, y: 10 };

/** 👻️ Where the travelling copy lies for a pointer at `point`. */
export function dragGhostPosition(point: DragPoint): DragPoint {
  return { x: point.x + DRAG_GHOST_OFFSET.x, y: point.y + DRAG_GHOST_OFFSET.y };
}

/** 🚀️ Whether the pointer has travelled far enough from where it was pressed for the press to become a drag. */
export function dragStarts(from: DragPoint, to: DragPoint, threshold: number = DRAG_START_THRESHOLD_PX): boolean {
  return Math.hypot(to.x - from.x, to.y - from.y) >= threshold;
}

/** ⎋ Whether the key abandons the drag. */
export function dragCancelKey(key: string): boolean {
  return key === "Escape";
}

/** ↔️ The rectangle inside a row where an element dropped at `index` among `items` would lie: the first half of the item that follows it, at the end the second half of the last one, an empty row is the whole slot. */
export function insertSlot(row: DropRect, items: readonly DropRect[], index: number): DropRect {
  if (items.length === 0) return row;
  const at = Math.min(Math.max(0, index), items.length);
  const next = items[at];
  if (next !== undefined) return { left: next.left, top: row.top, right: (next.left + next.right) / 2, bottom: row.bottom };
  const last = items[items.length - 1]!;
  return { left: (last.left + last.right) / 2, top: row.top, right: last.right, bottom: row.bottom };
}

/** 🌳️ The rectangle inside a tree row where a drop lands: the upper quarter before it, the lower quarter after it, the whole row inside it. */
export function treeDropBand(row: DropRect, position: "before" | "after" | "inside"): DropRect {
  const quarter = (row.bottom - row.top) / 4;
  if (position === "before") return { ...row, bottom: row.top + quarter };
  if (position === "after") return { ...row, top: row.bottom - quarter };
  return row;
}
