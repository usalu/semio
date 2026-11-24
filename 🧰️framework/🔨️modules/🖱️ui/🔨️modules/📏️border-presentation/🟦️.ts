// #region 🧲️Header
// 💻️ framework/ui/modules/📏️border-presentation/component.ts
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 📏️BorderPresentation
/** 📏️ Subtle normal control and divider stroke. */
export const borderNormalClass = "!border-normal";

/** ⬇️ Normal bottom divider stroke. */
export const borderNormalBottomClass = `border-b ${borderNormalClass}`;

/** 🌈️ Implicit element border color. */
export const borderElementClass = "border-element";
/** 🎚️ Paint precedence: persistent states first, then effects, then the transient interaction states focus and hover, so a focused box is never opened by a neighbour's selection. */
export type BorderState = "normal" | "active" | "invalid" | "waiting" | "loading" | "introduced" | "celebrated" | "focus" | "hover";

/** 📍 Coordinates in the caller's shared, snapped paint space. */
export interface BorderPoint { readonly x: number; readonly y: number }

/** 📐 One owned axis-aligned stroke before coincidence resolution. */
export interface BorderSegment {
  readonly id: string;
  readonly owner: string;
  readonly from: BorderPoint;
  readonly to: BorderPoint;
  readonly state: BorderState;
}

/** 🧩 Disjoint collinear span retaining all incident source identifiers. */
export interface ResolvedBorderSegment extends BorderSegment { readonly contributors: readonly string[] }

/** 🪜 Paint precedence rank of every state, hover highest. */
export const borderStatePriority: Record<BorderState, number> = {
  normal: 0, active: 1, invalid: 2, waiting: 3, loading: 4, introduced: 5, celebrated: 6, focus: 7, hover: 8,
};
const compareBorderText = (a: string, b: string): number => a < b ? -1 : a > b ? 1 : 0;

/** 🧮 Splits exact collinear overlaps once; hover wins, then owner and ID break ties independently of input order. */
export function resolveBorderSegments(segments: readonly BorderSegment[]): ResolvedBorderSegment[] {
  const ids = new Set<string>();
  const lines = new Map<string, { horizontal: boolean; coordinate: number; events: Map<number, { starts: BorderSegment[]; ends: BorderSegment[] }> }>();
  for (const segment of segments) {
    const { id, owner, from, to, state } = segment;
    if (!id || !owner || ids.has(id) || !Object.hasOwn(borderStatePriority, state)) throw new TypeError("Invalid border identity or state");
    ids.add(id);
    if (![from.x, from.y, to.x, to.y].every(Number.isFinite)) throw new TypeError("Border coordinates must be finite");
    if (from.x !== to.x && from.y !== to.y) throw new TypeError(`Border must be axis-aligned: ${JSON.stringify({ id, owner, from, to })}`);
    if (from.x === to.x && from.y === to.y) continue;
    const horizontal = from.y === to.y;
    const coordinate = horizontal ? from.y : from.x;
    const start = Math.min(horizontal ? from.x : from.y, horizontal ? to.x : to.y);
    const end = Math.max(horizontal ? from.x : from.y, horizontal ? to.x : to.y);
    const key = `${horizontal ? "h" : "v"}:${coordinate}`;
    let line = lines.get(key);
    if (!line) { line = { horizontal, coordinate, events: new Map() }; lines.set(key, line); }
    for (const [position, kind] of [[start, "starts"], [end, "ends"]] as const) {
      let event = line.events.get(position);
      if (!event) { event = { starts: [], ends: [] }; line.events.set(position, event); }
      event[kind].push(segment);
    }
  }
  const output: ResolvedBorderSegment[] = [];
  const orderedLines = [...lines.values()].sort((a, b) => Number(b.horizontal) - Number(a.horizontal) || a.coordinate - b.coordinate);
  for (const { horizontal, coordinate, events } of orderedLines) {
    const positions = [...events.keys()].sort((a, b) => a - b);
    const active = new Map<string, BorderSegment>();
    for (let index = 0; index < positions.length - 1; index++) {
      const start = positions[index], end = positions[index + 1];
      const event = events.get(start)!;
      for (const segment of event.ends) active.delete(segment.id);
      for (const segment of event.starts) active.set(segment.id, segment);
      if (!active.size) continue;
      const candidates = [...active.values()].sort((a, b) => borderStatePriority[b.state] - borderStatePriority[a.state] || compareBorderText(a.owner, b.owner) || compareBorderText(a.id, b.id));
      const winner = candidates[0];
      output.push({
        id: winner.id, owner: winner.owner, state: winner.state,
        from: horizontal ? { x: start, y: coordinate } : { x: coordinate, y: start },
        to: horizontal ? { x: end, y: coordinate } : { x: coordinate, y: end },
        contributors: candidates.map(segment => segment.id).sort(compareBorderText),
      });
    }
  }
  return output;
}
/** 📦 One box of the state model, with an optional outline that responds to hover within it. */
export interface StateBox {
  readonly id: string;
  readonly parent: string | null;
  readonly interactive: boolean;
  readonly hoverWithin?: boolean;
  readonly disabled: boolean;
  readonly marks: readonly BorderState[];
}

/** 🎚️ Hover goes to the innermost enabled control and to enabled ancestors that paint on hover within. Focus and markers stay local.
 *
 * @see ./states.cases.json */
export function resolveBoxStates(boxes: readonly StateBox[], pointer: string | null, focusVisible: string | null): Record<string, BorderState> {
  const byId = new Map(boxes.map(box => [box.id, box]));
  const innermost = (from: string | null, accepts: (box: StateBox) => boolean): string | null => {
    for (let box = from === null ? undefined : byId.get(from), steps = 0; box && steps <= boxes.length; box = box.parent === null ? undefined : byId.get(box.parent), steps++) if (accepts(box)) return box.id;
    return null;
  };
  const hovered = new Set<string>();
  let hasControl = false;
  for (let box = pointer === null ? undefined : byId.get(pointer), steps = 0; box && steps < boxes.length; box = box.parent === null ? undefined : byId.get(box.parent), steps++) {
    if (box.disabled) continue;
    if (box.hoverWithin || (!hasControl && box.interactive)) hovered.add(box.id);
    hasControl ||= box.interactive;
  }
  const focused = innermost(focusVisible, box => box.interactive);
  return Object.fromEntries(boxes.map(({ id, marks }) => {
    const candidates: BorderState[] = ["normal", ...marks];
    if (hovered.has(id)) candidates.push("hover");
    if (id === focused) candidates.push("focus");
    return [id, candidates.reduce((winner, state) => borderStatePriority[state] > borderStatePriority[winner] ? state : winner)];
  }));
}

/** 🔲 One box of the outline model: a rect, the edges it declares and its resolved state. */
export interface OutlineBox {
  readonly id: string;
  readonly parent: string | null;
  readonly owner: string;
  readonly rect: { readonly left: number; readonly top: number; readonly right: number; readonly bottom: number };
  readonly edges: readonly BorderSide[];
  readonly state: BorderState;
}

/** 🧭 A side of a box. */
export type BorderSide = "top" | "right" | "bottom" | "left";

/** 🖍️ Whether `state` closes the whole perimeter of its box. Hover does not: it emphasises the lines that lie on the perimeter without the hover and never adds one. */
export const borderStateCloses = (state: BorderState): boolean => state !== "normal" && state !== "hover";

/** 🫧 Sides of `box` that are drawn: a normal or hovered box keeps its declared sides except those flush with a side its parent leaves undrawn; a box in any other state closes all four. */
export function boxDrawnSides(box: OutlineBox, parent: OutlineBox | undefined): readonly BorderSide[] {
  if (borderStateCloses(box.state)) return ["top", "right", "bottom", "left"];
  return box.edges.filter(side => !parent || parent.edges.includes(side) || Math.abs(box.rect[side] - parent.rect[side]) >= 0.5);
}

/** 🧽 Resolved spans without those only emphasis contributes to: an emphasis segment recolours the lines it lies on and paints nothing where no other segment lies. */
export function withoutLoneEmphasis<Segment extends ResolvedBorderSegment>(resolved: readonly Segment[], emphasis: (id: string) => boolean): Segment[] {
  return resolved.filter(segment => !segment.contributors.every(emphasis));
}

/** 🔳 Resolved paint spans of every box outline: each drawn side is one `${id}:${side}` edge in the box's owner, resolved by state priority on all collinear overlaps. The sides a hovered box does not draw are emphasis: they take the hover state onto the lines of other boxes they lie on and add no line.
 *
 * @see ./outlines.cases.json */
export function boxOutlineSegments(boxes: readonly OutlineBox[]): ResolvedBorderSegment[] {
  const byId = new Map(boxes.map(box => [box.id, box]));
  const emphasis = new Set<string>();
  const sides: readonly BorderSide[] = ["top", "right", "bottom", "left"];
  return withoutLoneEmphasis(resolveBorderSegments(boxes.flatMap(box => {
    const drawn = boxDrawnSides(box, box.parent === null ? undefined : byId.get(box.parent));
    return (box.state === "hover" ? sides : drawn).map((side): BorderSegment => {
      const { left, top, right, bottom } = box.rect;
      const from = side === "right" ? { x: right, y: top } : side === "bottom" ? { x: left, y: bottom } : { x: left, y: top };
      const to = side === "top" ? { x: right, y: top } : side === "left" ? { x: left, y: bottom } : { x: right, y: bottom };
      if (!drawn.includes(side)) emphasis.add(`${box.id}:${side}`);
      return { id: `${box.id}:${side}`, owner: box.owner, from, to, state: box.state };
    });
  })), id => emphasis.has(id));
}
// #endregion 📏️BorderPresentation
