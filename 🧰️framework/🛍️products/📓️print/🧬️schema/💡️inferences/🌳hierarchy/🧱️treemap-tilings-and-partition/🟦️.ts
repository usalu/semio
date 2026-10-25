/** 🧱️ The six treemap tilings, the treemap layout and the icicle partition of the print hierarchy twin. */
import { VizHierarchyNode } from "../🟦️.ts";

//#region 🔖️Tiling
/** 🀄️ A treemap tiling: it positions the children of one node inside a rectangle. */
export type VizTiling<T> = (parent: VizHierarchyNode<T>, x0: number, y0: number, x1: number, y1: number) => void;

/** 🎲️ `dice`: children side by side across the width. */
export function tilingDice<T>(parent: VizHierarchyNode<T>, x0: number, y0: number, x1: number, y1: number): void {
  const nodes = parent.children ?? [];
  const k = parent.value ? (x1 - x0) / parent.value : 0;
  let cursor = x0;
  for (const node of nodes) {
    node.y0 = y0;
    node.y1 = y1;
    node.x0 = cursor;
    cursor += (node.value ?? 0) * k;
    node.x1 = cursor;
  }
}

/** 🍕️ `slice`: children stacked down the height. */
export function tilingSlice<T>(parent: VizHierarchyNode<T>, x0: number, y0: number, x1: number, y1: number): void {
  const nodes = parent.children ?? [];
  const k = parent.value ? (y1 - y0) / parent.value : 0;
  let cursor = y0;
  for (const node of nodes) {
    node.x0 = x0;
    node.x1 = x1;
    node.y0 = cursor;
    cursor += (node.value ?? 0) * k;
    node.y1 = cursor;
  }
}

/** 🔪️ `slice-dice`: alternating by depth. */
export function tilingSliceDice<T>(parent: VizHierarchyNode<T>, x0: number, y0: number, x1: number, y1: number): void {
  if (parent.depth & 1) tilingSlice(parent, x0, y0, x1, y1);
  else tilingDice(parent, x0, y0, x1, y1);
}

const PHI = (1 + Math.sqrt(5)) / 2;

type SquarifyRow<T> = { value: number; dice: boolean; children: VizHierarchyNode<T>[]; ratio?: number };

function squarifyRatio<T>(ratio: number, parent: VizHierarchyNode<T>, x0In: number, y0In: number, x1: number, y1: number): SquarifyRow<T>[] {
  const rows: SquarifyRow<T>[] = [];
  const nodes = parent.children ?? [];
  const n = nodes.length;
  let i0 = 0;
  let i1 = 0;
  let x0 = x0In;
  let y0 = y0In;
  let value = parent.value ?? 0;
  while (i0 < n) {
    const dx = x1 - x0;
    const dy = y1 - y0;
    let sumValue: number;
    do {
      sumValue = nodes[i1]!.value ?? 0;
      i1 += 1;
    } while (!sumValue && i1 < n);
    let minValue = sumValue;
    let maxValue = sumValue;
    const alpha = Math.max(dy / dx, dx / dy) / (value * ratio);
    let beta = sumValue * sumValue * alpha;
    let minRatio = Math.max(maxValue / beta, beta / minValue);
    for (; i1 < n; i1 += 1) {
      const nodeValue = nodes[i1]!.value ?? 0;
      sumValue += nodeValue;
      if (nodeValue < minValue) minValue = nodeValue;
      if (nodeValue > maxValue) maxValue = nodeValue;
      beta = sumValue * sumValue * alpha;
      const newRatio = Math.max(maxValue / beta, beta / minValue);
      if (newRatio > minRatio) {
        sumValue -= nodeValue;
        break;
      }
      minRatio = newRatio;
    }
    const row: SquarifyRow<T> = { value: sumValue, dice: dx < dy, children: nodes.slice(i0, i1) };
    rows.push(row);
    const pseudo = { value: row.value, children: row.children, depth: parent.depth } as unknown as VizHierarchyNode<T>;
    if (row.dice) {
      const nextY = value ? y0 + (dy * sumValue) / value : y1;
      tilingDice(pseudo, x0, y0, x1, nextY);
      y0 = nextY;
    } else {
      const nextX = value ? x0 + (dx * sumValue) / value : x1;
      tilingSlice(pseudo, x0, y0, nextX, y1);
      x0 = nextX;
    }
    value -= sumValue;
    i0 = i1;
  }
  return rows;
}

/** 🔲️ `squarify`: rows chosen to keep every rectangle as close to the golden ratio as possible. */
export function tilingSquarify<T>(parent: VizHierarchyNode<T>, x0: number, y0: number, x1: number, y1: number): void {
  squarifyRatio(PHI, parent, x0, y0, x1, y1);
}

const RESQUARIFY_ROWS = new WeakMap<object, SquarifyRow<unknown>[]>();

/** 🔁️ `resquarify`: squarify that reuses the row structure of the previous pass. */
export function tilingResquarify<T>(parent: VizHierarchyNode<T>, x0: number, y0: number, x1: number, y1: number): void {
  const cached = RESQUARIFY_ROWS.get(parent as unknown as object) as SquarifyRow<T>[] | undefined;
  if (cached !== undefined && cached.length > 0 && cached[0]!.ratio === PHI) {
    let cx0 = x0;
    let cy0 = y0;
    const dx = x1 - x0;
    const dy = y1 - y0;
    let value = parent.value ?? 0;
    for (const row of cached) {
      const pseudo = { value: row.value, children: row.children, depth: parent.depth } as unknown as VizHierarchyNode<T>;
      if (row.dice) {
        const nextY = value ? cy0 + (dy * row.value) / value : y1;
        tilingDice(pseudo, cx0, cy0, x1, nextY);
        cy0 = nextY;
      } else {
        const nextX = value ? cx0 + (dx * row.value) / value : x1;
        tilingSlice(pseudo, cx0, cy0, nextX, y1);
        cx0 = nextX;
      }
      value -= row.value;
    }
    return;
  }
  const rows = squarifyRatio(PHI, parent, x0, y0, x1, y1);
  for (const row of rows) row.ratio = PHI;
  RESQUARIFY_ROWS.set(parent as unknown as object, rows as SquarifyRow<unknown>[]);
}

/** ⚖️ `binary`: recursive halving by cumulative value, the balanced-partition tiling. */
export function tilingBinary<T>(parent: VizHierarchyNode<T>, x0: number, y0: number, x1: number, y1: number): void {
  const nodes = parent.children ?? [];
  const n = nodes.length;
  const sums = new Array<number>(n + 1);
  sums[0] = 0;
  for (let i = 0; i < n; i += 1) sums[i + 1] = sums[i]! + (nodes[i]!.value ?? 0);
  const partition = (i: number, j: number, value: number, px0: number, py0: number, px1: number, py1: number): void => {
    if (i >= j - 1) {
      const node = nodes[i]!;
      node.x0 = px0;
      node.y0 = py0;
      node.x1 = px1;
      node.y1 = py1;
      return;
    }
    const valueOffset = sums[i]!;
    const valueTarget = value / 2 + valueOffset;
    let k = i + 1;
    let hi = j - 1;
    while (k < hi) {
      const mid = (k + hi) >>> 1;
      if (sums[mid]! < valueTarget) k = mid + 1;
      else hi = mid;
    }
    if (valueTarget - sums[k - 1]! < sums[k]! - valueTarget && i + 1 < k) k -= 1;
    const valueLeft = sums[k]! - valueOffset;
    const valueRight = value - valueLeft;
    if (px1 - px0 > py1 - py0) {
      const xk = value ? (px0 * valueRight + px1 * valueLeft) / value : px1;
      partition(i, k, valueLeft, px0, py0, xk, py1);
      partition(k, j, valueRight, xk, py0, px1, py1);
    } else {
      const yk = value ? (py0 * valueRight + py1 * valueLeft) / value : py1;
      partition(i, k, valueLeft, px0, py0, px1, yk);
      partition(k, j, valueRight, px0, yk, px1, py1);
    }
  };
  partition(0, n, parent.value ?? 0, x0, y0, x1, y1);
}

/** 🔎️ Resolves a tiling name from the grammar onto its implementation. */
export function vizTiling<T>(name: string): VizTiling<T> {
  switch (name) {
    case "slice":
      return tilingSlice;
    case "dice":
      return tilingDice;
    case "slice-dice":
      return tilingSliceDice;
    case "binary":
      return tilingBinary;
    case "resquarify":
      return tilingResquarify;
    default:
      return tilingSquarify;
  }
}

//#endregion 🔖️Tiling
//#region 🔖️Treemap
/** ⚙️ Options of `\SemioVizLayout{treemap}`. */
export type VizTreemapOptions<T> = {
  readonly size?: readonly [number, number];
  readonly tile?: string | VizTiling<T>;
  readonly round?: boolean;
  readonly paddingInner?: number;
  readonly paddingOuter?: number;
  readonly paddingTop?: number;
  readonly paddingRight?: number;
  readonly paddingBottom?: number;
  readonly paddingLeft?: number;
};

function roundNode<T>(node: VizHierarchyNode<T>): void {
  node.x0 = Math.round(node.x0);
  node.y0 = Math.round(node.y0);
  node.x1 = Math.round(node.x1);
  node.y1 = Math.round(node.y1);
}

/** 🗺️ The treemap layout: nested rectangles proportional to `value`. */
export function vizTreemap<T>(root: VizHierarchyNode<T>, options: VizTreemapOptions<T> = {}): VizHierarchyNode<T> {
  const tile = typeof options.tile === "function" ? options.tile : vizTiling<T>(typeof options.tile === "string" ? options.tile : "squarify");
  const [dx, dy] = options.size ?? [1, 1];
  const paddingInner = options.paddingInner ?? 0;
  const paddingTop = options.paddingTop ?? options.paddingOuter ?? 0;
  const paddingRight = options.paddingRight ?? options.paddingOuter ?? 0;
  const paddingBottom = options.paddingBottom ?? options.paddingOuter ?? 0;
  const paddingLeft = options.paddingLeft ?? options.paddingOuter ?? 0;
  const paddingStack: number[] = [0];
  paddingStack[root.depth] = 0;
  root.x0 = 0;
  root.y0 = 0;
  root.x1 = dx;
  root.y1 = dy;
  root.eachBefore((node) => {
    let p = paddingStack[node.depth] ?? 0;
    let x0 = node.x0 + p;
    let y0 = node.y0 + p;
    let x1 = node.x1 - p;
    let y1 = node.y1 - p;
    if (x1 < x0) {
      x0 = (x0 + x1) / 2;
      x1 = x0;
    }
    if (y1 < y0) {
      y0 = (y0 + y1) / 2;
      y1 = y0;
    }
    node.x0 = x0;
    node.y0 = y0;
    node.x1 = x1;
    node.y1 = y1;
    if (node.children && node.children.length > 0) {
      p = paddingInner / 2;
      paddingStack[node.depth + 1] = p;
      x0 += paddingLeft - p;
      y0 += paddingTop - p;
      x1 -= paddingRight - p;
      y1 -= paddingBottom - p;
      if (x1 < x0) {
        x0 = (x0 + x1) / 2;
        x1 = x0;
      }
      if (y1 < y0) {
        y0 = (y0 + y1) / 2;
        y1 = y0;
      }
      tile(node, x0, y0, x1, y1);
    }
  });
  if (options.round === true) root.eachBefore(roundNode);
  return root;
}

/** 🧊️ The icicle partition: one band per depth, subdivided by value. */
export function vizPartition<T>(root: VizHierarchyNode<T>, options: { size?: readonly [number, number]; padding?: number; round?: boolean } = {}): VizHierarchyNode<T> {
  const [dx, dy] = options.size ?? [1, 1];
  const padding = options.padding ?? 0;
  const n = root.height + 1;
  root.x0 = padding;
  root.y0 = padding;
  root.x1 = dx;
  root.y1 = dy / n;
  root.eachBefore((node) => {
    if (node.children && node.children.length > 0) tilingDice(node, node.x0, (dy * (node.depth + 1)) / n, node.x1, (dy * (node.depth + 2)) / n);
    let x0 = node.x0;
    let y0 = node.y0;
    let x1 = node.x1 - padding;
    let y1 = node.y1 - padding;
    if (x1 < x0) {
      x0 = (x0 + x1) / 2;
      x1 = x0;
    }
    if (y1 < y0) {
      y0 = (y0 + y1) / 2;
      y1 = y0;
    }
    node.x0 = x0;
    node.y0 = y0;
    node.x1 = x1;
    node.y1 = y1;
  });
  if (options.round === true) root.eachBefore(roundNode);
  return root;
}
//#endregion 🔖️Treemap
