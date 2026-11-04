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
/** 🎚️ Paint precedence, from neutral through effects to pointer hover. */
export type BorderState = "normal" | "focus" | "active" | "waiting" | "loading" | "introduced" | "celebrated" | "hover";

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

const borderStatePriority: Record<BorderState, number> = {
  normal: 0, focus: 1, active: 2, waiting: 3, loading: 4, introduced: 5, celebrated: 6, hover: 7,
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
    if (from.x !== to.x && from.y !== to.y) throw new TypeError("Border must be axis-aligned");
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
// #endregion 📏️BorderPresentation
