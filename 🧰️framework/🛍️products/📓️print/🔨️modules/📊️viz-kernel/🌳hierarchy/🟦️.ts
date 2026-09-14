/** 🌳 Hierarchies: the TypeScript twin of `semio-viz-hierarchy`. Node construction and stratify,
 * the traversal and aggregation vocabulary, the Reingold–Tilford/Buchheim tree, the dendrogram
 * cluster, all six treemap tilings, the icicle partition and the Welzl circle packing — the last
 * driven by the same seeded linear congruential generator as the LaTeX kernel.
 * @see ../../../🖋️latex/semio-viz-hierarchy.sty
 */

//#region 🔖️Random
/** 🎲 d3's linear congruential generator: `s ← (1664525·s + 1013904223) mod 2³²`, seeded with 1.
 * `semio-viz-hierarchy.sty` steps the identical recurrence, so packing consumes the same stream. */
export function vizLcg(seed = 1): () => number {
  let s = seed;
  return () => {
    s = (1664525 * s + 1013904223) % 4294967296;
    return s / 4294967296;
  };
}

function shuffleWith<T>(values: T[], random: () => number): T[] {
  let m = values.length;
  while (m) {
    const i = (random() * m--) | 0;
    const t = values[m]!;
    values[m] = values[i]!;
    values[i] = t;
  }
  return values;
}
//#endregion 🔖️Random

//#region 🔖️Node
/** 🌳 One node of a hierarchy, carrying every field the layouts write into. */
export class VizHierarchyNode<T> {
  data: T;
  depth = 0;
  height = 0;
  parent: VizHierarchyNode<T> | null = null;
  children?: VizHierarchyNode<T>[];
  value?: number;
  id?: string;
  x = 0;
  y = 0;
  r = 0;
  x0 = 0;
  y0 = 0;
  x1 = 0;
  y1 = 0;

  constructor(data: T) {
    this.data = data;
  }

  /** 🌳 Pre-order traversal: a node before its children, children left to right. */
  eachBefore(callback: (node: VizHierarchyNode<T>, index: number) => void): this {
    const stack: VizHierarchyNode<T>[] = [this];
    let index = -1;
    for (let node = stack.pop(); node !== undefined; node = stack.pop()) {
      index += 1;
      callback(node, index);
      const children = node.children;
      if (children) for (let i = children.length - 1; i >= 0; i -= 1) stack.push(children[i]!);
    }
    return this;
  }

  /** 🌳 Post-order traversal: every child before its parent. */
  eachAfter(callback: (node: VizHierarchyNode<T>, index: number) => void): this {
    const stack: VizHierarchyNode<T>[] = [this];
    const next: VizHierarchyNode<T>[] = [];
    for (let node = stack.pop(); node !== undefined; node = stack.pop()) {
      next.push(node);
      const children = node.children;
      if (children) for (const child of children) stack.push(child);
    }
    let index = -1;
    for (let node = next.pop(); node !== undefined; node = next.pop()) {
      index += 1;
      callback(node, index);
    }
    return this;
  }

  /** 🌳 Breadth-first traversal. */
  each(callback: (node: VizHierarchyNode<T>, index: number) => void): this {
    let index = -1;
    for (const node of this.descendants()) {
      index += 1;
      callback(node, index);
    }
    return this;
  }

  /** 🌳 Sums a leaf value into every ancestor, bottom up. */
  sum(value: (data: T) => number): this {
    return this.eachAfter((node) => {
      let total = +value(node.data) || 0;
      const children = node.children;
      if (children) for (let i = children.length - 1; i >= 0; i -= 1) total += children[i]!.value ?? 0;
      node.value = total;
    });
  }

  /** 🌳 Counts the leaves under every node into `value`. */
  count(): this {
    return this.eachAfter((node) => {
      let total = 0;
      const children = node.children;
      if (children) for (const child of children) total += child.value ?? 0;
      else total = 1;
      node.value = total;
    });
  }

  /** 🌳 Sorts the children of every node in place. */
  sort(compare: (a: VizHierarchyNode<T>, b: VizHierarchyNode<T>) => number): this {
    return this.eachBefore((node) => {
      if (node.children) node.children.sort(compare);
    });
  }

  /** 🌳 Every node of the subtree in breadth-first order. */
  descendants(): VizHierarchyNode<T>[] {
    const out: VizHierarchyNode<T>[] = [];
    let queue: VizHierarchyNode<T>[] = [this];
    while (queue.length > 0) {
      const next: VizHierarchyNode<T>[] = [];
      for (const node of queue) {
        out.push(node);
        if (node.children) next.push(...node.children);
      }
      queue = next;
    }
    return out;
  }

  /** 🌳 The childless nodes, in pre-order. */
  leaves(): VizHierarchyNode<T>[] {
    const out: VizHierarchyNode<T>[] = [];
    this.eachBefore((node) => {
      if (!node.children) out.push(node);
    });
    return out;
  }

  /** 🌳 This node and every ancestor up to the root. */
  ancestors(): VizHierarchyNode<T>[] {
    const out: VizHierarchyNode<T>[] = [];
    for (let node: VizHierarchyNode<T> | null = this; node !== null; node = node.parent) out.push(node);
    return out;
  }

  /** 🌳 The parent–child edges of the subtree. */
  links(): { readonly source: VizHierarchyNode<T>; readonly target: VizHierarchyNode<T> }[] {
    const out: { source: VizHierarchyNode<T>; target: VizHierarchyNode<T> }[] = [];
    for (const node of this.descendants()) if (node.parent !== null) out.push({ source: node.parent, target: node });
    return out;
  }
}

function computeHeight<T>(start: VizHierarchyNode<T>): void {
  let height = 0;
  let node: VizHierarchyNode<T> | null = start;
  do {
    node.height = height;
    height += 1;
    node = node.parent;
  } while (node !== null && node.height < height);
}

/** 🌳 Builds a hierarchy from nested data through a children accessor. */
export function vizHierarchy<T>(data: T, children: (data: T) => readonly T[] | null | undefined = (value) => (value as { children?: readonly T[] }).children): VizHierarchyNode<T> {
  const root = new VizHierarchyNode(data);
  const stack: VizHierarchyNode<T>[] = [root];
  for (let node = stack.pop(); node !== undefined; node = stack.pop()) {
    const kids = children(node.data);
    if (kids && kids.length > 0) {
      const built = kids.map((entry) => new VizHierarchyNode(entry));
      node.children = built;
      for (let i = built.length - 1; i >= 0; i -= 1) {
        const child = built[i]!;
        child.parent = node;
        child.depth = node.depth + 1;
        stack.push(child);
      }
    }
  }
  root.eachBefore(computeHeight);
  return root;
}

/** 🌳 `\SemioVizHierarchy`: builds a hierarchy from a flat id/parent table. */
export function vizStratify<T>(data: readonly T[], options: { id?: (row: T) => string | null | undefined; parentId?: (row: T) => string | null | undefined } = {}): VizHierarchyNode<T> {
  const id = options.id ?? ((row: T) => (row as { id?: string }).id);
  const parentId = options.parentId ?? ((row: T) => (row as { parentId?: string }).parentId);
  const nodes = data.map((row) => new VizHierarchyNode(row));
  const byId = new Map<string, VizHierarchyNode<T> | "ambiguous">();
  const parents = new Array<string | undefined>(nodes.length);
  nodes.forEach((node, i) => {
    const nodeId = id(data[i]!);
    if (nodeId !== null && nodeId !== undefined && `${nodeId}` !== "") {
      node.id = `${nodeId}`;
      byId.set(node.id, byId.has(node.id) ? "ambiguous" : node);
    }
    const parent = parentId(data[i]!);
    parents[i] = parent === null || parent === undefined || `${parent}` === "" ? undefined : `${parent}`;
  });
  let root: VizHierarchyNode<T> | undefined;
  nodes.forEach((node, i) => {
    const parentKey = parents[i];
    if (parentKey === undefined) {
      if (root !== undefined) throw new Error("multiple roots");
      root = node;
      return;
    }
    const parent = byId.get(parentKey);
    if (parent === undefined) throw new Error(`missing: ${parentKey}`);
    if (parent === "ambiguous") throw new Error(`ambiguous: ${parentKey}`);
    if (parent.children) parent.children.push(node);
    else parent.children = [node];
    node.parent = parent;
  });
  if (root === undefined) throw new Error("no root");
  root.eachBefore((node) => {
    node.depth = node.parent === null ? 0 : node.parent.depth + 1;
  });
  root.eachBefore(computeHeight);
  return root;
}
//#endregion 🔖️Node

//#region 🔖️Tree
type TreeShadow<T> = {
  node: VizHierarchyNode<T>;
  parent: TreeShadow<T> | null;
  children: TreeShadow<T>[] | null;
  ancestor: TreeShadow<T>;
  thread: TreeShadow<T> | null;
  z: number;
  m: number;
  c: number;
  s: number;
  index: number;
  defaultAncestor: TreeShadow<T> | null;
};

function shadowTree<T>(root: VizHierarchyNode<T>): TreeShadow<T> {
  const make = (node: VizHierarchyNode<T>, index: number): TreeShadow<T> => {
    const shadow: TreeShadow<T> = { node, parent: null, children: null, ancestor: null as unknown as TreeShadow<T>, thread: null, z: 0, m: 0, c: 0, s: 0, index, defaultAncestor: null };
    shadow.ancestor = shadow;
    return shadow;
  };
  const tree = make(root, 0);
  const stack: TreeShadow<T>[] = [tree];
  for (let current = stack.pop(); current !== undefined; current = stack.pop()) {
    const children = current.node.children;
    if (children) {
      current.children = children.map((child, i) => make(child, i));
      for (let i = current.children.length - 1; i >= 0; i -= 1) {
        current.children[i]!.parent = current;
        stack.push(current.children[i]!);
      }
    }
  }
  const preroot = make(root, 0);
  preroot.children = [tree];
  tree.parent = preroot;
  return tree;
}

function shadowEachAfter<T>(root: TreeShadow<T>, callback: (node: TreeShadow<T>) => void): void {
  const stack: TreeShadow<T>[] = [root];
  const next: TreeShadow<T>[] = [];
  for (let node = stack.pop(); node !== undefined; node = stack.pop()) {
    next.push(node);
    if (node.children) for (const child of node.children) stack.push(child);
  }
  for (let node = next.pop(); node !== undefined; node = next.pop()) callback(node);
}

function shadowEachBefore<T>(root: TreeShadow<T>, callback: (node: TreeShadow<T>) => void): void {
  const stack: TreeShadow<T>[] = [root];
  for (let node = stack.pop(); node !== undefined; node = stack.pop()) {
    callback(node);
    if (node.children) for (let i = node.children.length - 1; i >= 0; i -= 1) stack.push(node.children[i]!);
  }
}

function nextLeft<T>(v: TreeShadow<T>): TreeShadow<T> | null {
  return v.children && v.children.length > 0 ? v.children[0]! : v.thread;
}

function nextRight<T>(v: TreeShadow<T>): TreeShadow<T> | null {
  return v.children && v.children.length > 0 ? v.children[v.children.length - 1]! : v.thread;
}

function moveSubtree<T>(wm: TreeShadow<T>, wp: TreeShadow<T>, shift: number): void {
  const change = shift / (wp.index - wm.index);
  wp.c -= change;
  wp.s += shift;
  wm.c += change;
  wp.z += shift;
  wp.m += shift;
}

function executeShifts<T>(v: TreeShadow<T>): void {
  let shift = 0;
  let change = 0;
  const children = v.children ?? [];
  for (let i = children.length - 1; i >= 0; i -= 1) {
    const w = children[i]!;
    w.z += shift;
    w.m += shift;
    change += w.c;
    shift += w.s + change;
  }
}

function nextAncestor<T>(vim: TreeShadow<T>, v: TreeShadow<T>, ancestor: TreeShadow<T>): TreeShadow<T> {
  return vim.ancestor.parent === v.parent ? vim.ancestor : ancestor;
}

/** 🌳 Options of `\SemioVizLayout{tree}`; `nodeSize` switches from a normalised to an absolute layout. */
export type VizTreeOptions<T> = { readonly size?: readonly [number, number]; readonly nodeSize?: readonly [number, number]; readonly separation?: (a: VizHierarchyNode<T>, b: VizHierarchyNode<T>) => number };

/** 🌳 The tidy tree layout of Reingold, Tilford and Buchheim. */
export function vizTree<T>(root: VizHierarchyNode<T>, options: VizTreeOptions<T> = {}): VizHierarchyNode<T> {
  const separation = options.separation ?? ((a: VizHierarchyNode<T>, b: VizHierarchyNode<T>) => (a.parent === b.parent ? 1 : 2));
  const [dx, dy] = options.nodeSize ?? options.size ?? [1, 1];
  const tree = shadowTree(root);
  const apportion = (v: TreeShadow<T>, w: TreeShadow<T> | null, ancestorIn: TreeShadow<T>): TreeShadow<T> => {
    let ancestor = ancestorIn;
    if (w === null) return ancestor;
    let vip: TreeShadow<T> | null = v;
    let vop: TreeShadow<T> | null = v;
    let vim: TreeShadow<T> | null = w;
    let vom: TreeShadow<T> | null = vip.parent!.children![0]!;
    let sip = vip.m;
    let sop = vop.m;
    let sim = vim.m;
    let som = vom.m;
    for (;;) {
      vim = nextRight(vim!);
      vip = nextLeft(vip!);
      if (vim === null || vip === null) break;
      vom = nextLeft(vom!);
      vop = nextRight(vop!);
      vop!.ancestor = v;
      const shift = vim.z + sim - vip.z - sip + separation(vim.node, vip.node);
      if (shift > 0) {
        moveSubtree(nextAncestor(vim, v, ancestor), v, shift);
        sip += shift;
        sop += shift;
      }
      sim += vim.m;
      sip += vip.m;
      som += vom!.m;
      sop += vop!.m;
    }
    if (vim !== null && nextRight(vop!) === null) {
      vop!.thread = vim;
      vop!.m += sim - sop;
    }
    if (vip !== null && nextLeft(vom!) === null) {
      vom!.thread = vip;
      vom!.m += sip - som;
      ancestor = v;
    }
    return ancestor;
  };
  shadowEachAfter(tree, (v) => {
    const children = v.children;
    const siblings = v.parent!.children!;
    const w = v.index > 0 ? siblings[v.index - 1]! : null;
    if (children && children.length > 0) {
      executeShifts(v);
      const midpoint = (children[0]!.z + children[children.length - 1]!.z) / 2;
      if (w !== null) {
        v.z = w.z + separation(v.node, w.node);
        v.m = v.z - midpoint;
      } else v.z = midpoint;
    } else if (w !== null) v.z = w.z + separation(v.node, w.node);
    v.parent!.defaultAncestor = apportion(v, w, v.parent!.defaultAncestor ?? siblings[0]!);
  });
  tree.parent!.m = -tree.z;
  shadowEachBefore(tree, (v) => {
    v.node.x = v.z + v.parent!.m;
    v.m += v.parent!.m;
  });
  if (options.nodeSize !== undefined) {
    root.eachBefore((node) => {
      node.x *= dx;
      node.y = node.depth * dy;
    });
  } else {
    let left = root;
    let right = root;
    let bottom = root;
    root.eachBefore((node) => {
      if (node.x < left.x) left = node;
      if (node.x > right.x) right = node;
      if (node.depth > bottom.depth) bottom = node;
    });
    const s = left === right ? 1 : separation(left, right) / 2;
    const tx = s - left.x;
    const kx = dx / (right.x + s + tx);
    const ky = dy / (bottom.depth || 1);
    root.eachBefore((node) => {
      node.x = (node.x + tx) * kx;
      node.y = node.depth * ky;
    });
  }
  return root;
}

/** 🌳 The dendrogram cluster layout: every leaf on the same baseline. */
export function vizCluster<T>(root: VizHierarchyNode<T>, options: VizTreeOptions<T> = {}): VizHierarchyNode<T> {
  const separation = options.separation ?? ((a: VizHierarchyNode<T>, b: VizHierarchyNode<T>) => (a.parent === b.parent ? 1 : 2));
  const [dx, dy] = options.nodeSize ?? options.size ?? [1, 1];
  let previous: VizHierarchyNode<T> | null = null;
  let x = 0;
  root.eachAfter((node) => {
    const children = node.children;
    if (children && children.length > 0) {
      node.x = children.reduce((total, child) => total + child.x, 0) / children.length;
      node.y = 1 + children.reduce((best, child) => Math.max(best, child.y), 0);
    } else {
      if (previous !== null) {
        x += separation(node, previous);
        node.x = x;
      } else node.x = 0;
      node.y = 0;
      previous = node;
    }
  });
  const leafLeft = (node: VizHierarchyNode<T>): VizHierarchyNode<T> => {
    let current = node;
    while (current.children && current.children.length > 0) current = current.children[0]!;
    return current;
  };
  const leafRight = (node: VizHierarchyNode<T>): VizHierarchyNode<T> => {
    let current = node;
    while (current.children && current.children.length > 0) current = current.children[current.children.length - 1]!;
    return current;
  };
  const left = leafLeft(root);
  const right = leafRight(root);
  const x0 = left.x - separation(left, right) / 2;
  const x1 = right.x + separation(right, left) / 2;
  return root.eachAfter(
    options.nodeSize !== undefined
      ? (node) => {
          node.x = (node.x - root.x) * dx;
          node.y = (root.y - node.y) * dy;
        }
      : (node) => {
          node.x = ((node.x - x0) / (x1 - x0)) * dx;
          node.y = (1 - (root.y ? node.y / root.y : 1)) * dy;
        },
  );
}
//#endregion 🔖️Tree

//#region 🔖️Tiling
/** 🧱️ A treemap tiling: it positions the children of one node inside a rectangle. */
export type VizTiling<T> = (parent: VizHierarchyNode<T>, x0: number, y0: number, x1: number, y1: number) => void;

/** 🧱️ `dice`: children side by side across the width. */
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

/** 🧱️ `slice`: children stacked down the height. */
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

/** 🧱️ `slice-dice`: alternating by depth. */
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

/** 🧱️ `squarify`: rows chosen to keep every rectangle as close to the golden ratio as possible. */
export function tilingSquarify<T>(parent: VizHierarchyNode<T>, x0: number, y0: number, x1: number, y1: number): void {
  squarifyRatio(PHI, parent, x0, y0, x1, y1);
}

const RESQUARIFY_ROWS = new WeakMap<object, SquarifyRow<unknown>[]>();

/** 🧱️ `resquarify`: squarify that reuses the row structure of the previous pass. */
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

/** 🧱️ `binary`: recursive halving by cumulative value, the balanced-partition tiling. */
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

/** 🧱️ Resolves a tiling name from the grammar onto its implementation. */
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
/** 🧱️ Options of `\SemioVizLayout{treemap}`. */
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

/** 🧱️ The treemap layout: nested rectangles proportional to `value`. */
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

//#region 🔖️Pack
type Circle = { x: number; y: number; r: number };

function place(b: Circle, a: Circle, c: Circle): void {
  const dx = b.x - a.x;
  const dy = b.y - a.y;
  const d2 = dx * dx + dy * dy;
  if (d2) {
    let a2 = a.r + c.r;
    a2 *= a2;
    let b2 = b.r + c.r;
    b2 *= b2;
    if (a2 > b2) {
      const x = (d2 + b2 - a2) / (2 * d2);
      const y = Math.sqrt(Math.max(0, b2 / d2 - x * x));
      c.x = b.x - x * dx - y * dy;
      c.y = b.y - x * dy + y * dx;
    } else {
      const x = (d2 + a2 - b2) / (2 * d2);
      const y = Math.sqrt(Math.max(0, a2 / d2 - x * x));
      c.x = a.x + x * dx - y * dy;
      c.y = a.y + x * dy + y * dx;
    }
  } else {
    c.x = a.x + c.r;
    c.y = a.y;
  }
}

function intersects(a: Circle, b: Circle): boolean {
  const dr = a.r + b.r - 1e-6;
  const dx = b.x - a.x;
  const dy = b.y - a.y;
  return dr > 0 && dr * dr > dx * dx + dy * dy;
}

type ChainNode = { circle: Circle; next: ChainNode; previous: ChainNode };

function chainScore(node: ChainNode): number {
  const a = node.circle;
  const b = node.next.circle;
  const ab = a.r + b.r;
  const dx = (a.x * b.r + b.x * a.r) / ab;
  const dy = (a.y * b.r + b.y * a.r) / ab;
  return dx * dx + dy * dy;
}

function enclosesNot(a: Circle, b: Circle): boolean {
  const dr = a.r - b.r;
  const dx = b.x - a.x;
  const dy = b.y - a.y;
  return dr < 0 || dr * dr < dx * dx + dy * dy;
}

function enclosesWeak(a: Circle, b: Circle): boolean {
  const dr = a.r - b.r + Math.max(a.r, b.r, 1) * 1e-9;
  const dx = b.x - a.x;
  const dy = b.y - a.y;
  return dr > 0 && dr * dr > dx * dx + dy * dy;
}

function enclosesWeakAll(a: Circle, basis: readonly Circle[]): boolean {
  for (const circle of basis) if (!enclosesWeak(a, circle)) return false;
  return true;
}

function encloseBasis2(a: Circle, b: Circle): Circle {
  const x21 = b.x - a.x;
  const y21 = b.y - a.y;
  const r21 = b.r - a.r;
  const l = Math.sqrt(x21 * x21 + y21 * y21);
  return { x: (a.x + b.x + (x21 / l) * r21) / 2, y: (a.y + b.y + (y21 / l) * r21) / 2, r: (l + a.r + b.r) / 2 };
}

function encloseBasis3(a: Circle, b: Circle, c: Circle): Circle {
  const a2 = a.x - b.x;
  const a3 = a.x - c.x;
  const b2 = a.y - b.y;
  const b3 = a.y - c.y;
  const c2 = b.r - a.r;
  const c3 = c.r - a.r;
  const d1 = a.x * a.x + a.y * a.y - a.r * a.r;
  const d2 = d1 - b.x * b.x - b.y * b.y + b.r * b.r;
  const d3 = d1 - c.x * c.x - c.y * c.y + c.r * c.r;
  const ab = a3 * b2 - a2 * b3;
  const xa = (b2 * d3 - b3 * d2) / (ab * 2) - a.x;
  const xb = (b3 * c2 - b2 * c3) / ab;
  const ya = (a3 * d2 - a2 * d3) / (ab * 2) - a.y;
  const yb = (a2 * c3 - a3 * c2) / ab;
  const A = xb * xb + yb * yb - 1;
  const B = 2 * (a.r + xa * xb + ya * yb);
  const C = xa * xa + ya * ya - a.r * a.r;
  const r = -(Math.abs(A) > 1e-6 ? (B + Math.sqrt(B * B - 4 * A * C)) / (2 * A) : C / B);
  return { x: a.x + xa + xb * r, y: a.y + ya + yb * r, r };
}

function encloseBasis(basis: readonly Circle[]): Circle {
  if (basis.length === 1) return { x: basis[0]!.x, y: basis[0]!.y, r: basis[0]!.r };
  if (basis.length === 2) return encloseBasis2(basis[0]!, basis[1]!);
  return encloseBasis3(basis[0]!, basis[1]!, basis[2]!);
}

function extendBasis(basis: readonly Circle[], p: Circle): Circle[] {
  if (enclosesWeakAll(p, basis)) return [p];
  for (const candidate of basis) if (enclosesNot(p, candidate) && enclosesWeakAll(encloseBasis2(candidate, p), basis)) return [candidate, p];
  for (let i = 0; i < basis.length - 1; i += 1) {
    for (let j = i + 1; j < basis.length; j += 1) {
      if (enclosesNot(encloseBasis2(basis[i]!, basis[j]!), p) && enclosesNot(encloseBasis2(basis[i]!, p), basis[j]!) && enclosesNot(encloseBasis2(basis[j]!, p), basis[i]!) && enclosesWeakAll(encloseBasis3(basis[i]!, basis[j]!, p), basis)) {
        return [basis[i]!, basis[j]!, p];
      }
    }
  }
  throw new Error("no enclosing basis");
}

/** 🫧 The smallest enclosing circle by Welzl's move-to-front, shuffled by the seeded generator. */
export function vizPackEnclose(circles: readonly Circle[], random: () => number = vizLcg()): Circle | undefined {
  const shuffled = shuffleWith([...circles], random);
  let i = 0;
  let basis: Circle[] = [];
  let enclosing: Circle | undefined;
  while (i < shuffled.length) {
    const p = shuffled[i]!;
    if (enclosing !== undefined && enclosesWeak(enclosing, p)) i += 1;
    else {
      basis = extendBasis(basis, p);
      enclosing = encloseBasis(basis);
      i = 0;
    }
  }
  return enclosing;
}

/** 🫧 The front-chain sibling packing; returns the radius of the enclosing circle. */
export function vizPackSiblings(circles: readonly Circle[], random: () => number = vizLcg()): number {
  const n = circles.length;
  if (n === 0) return 0;
  let a = circles[0]!;
  a.x = 0;
  a.y = 0;
  if (n < 2) return a.r;
  const b0 = circles[1]!;
  a.x = -b0.r;
  b0.x = a.r;
  b0.y = 0;
  if (n < 3) return a.r + b0.r;
  place(b0, a, circles[2]!);
  let na: ChainNode = { circle: a, next: null as unknown as ChainNode, previous: null as unknown as ChainNode };
  let nb: ChainNode = { circle: b0, next: null as unknown as ChainNode, previous: null as unknown as ChainNode };
  let nc: ChainNode = { circle: circles[2]!, next: null as unknown as ChainNode, previous: null as unknown as ChainNode };
  na.next = nc.previous = nb;
  nb.next = na.previous = nc;
  nc.next = nb.previous = na;
  for (let i = 3; i < n; i += 1) {
    place(na.circle, nb.circle, circles[i]!);
    nc = { circle: circles[i]!, next: null as unknown as ChainNode, previous: null as unknown as ChainNode };
    let j = nb.next;
    let k = na.previous;
    let sj = nb.circle.r;
    let sk = na.circle.r;
    let restarted = false;
    do {
      if (sj <= sk) {
        if (intersects(j.circle, nc.circle)) {
          nb = j;
          na.next = nb;
          nb.previous = na;
          i -= 1;
          restarted = true;
          break;
        }
        sj += j.circle.r;
        j = j.next;
      } else {
        if (intersects(k.circle, nc.circle)) {
          na = k;
          na.next = nb;
          nb.previous = na;
          i -= 1;
          restarted = true;
          break;
        }
        sk += k.circle.r;
        k = k.previous;
      }
    } while (j !== k.next);
    if (restarted) continue;
    nc.previous = na;
    nc.next = nb;
    na.next = nc;
    nb.previous = nc;
    nb = nc;
    let best = chainScore(na);
    let cursor = nc;
    while ((cursor = cursor.next) !== nb) {
      const candidate = chainScore(cursor);
      if (candidate < best) {
        na = cursor;
        best = candidate;
      }
    }
    nb = na.next;
  }
  const chain: Circle[] = [nb.circle];
  let cursor = nb;
  while ((cursor = cursor.next) !== nb) chain.push(cursor.circle);
  const enclosing = vizPackEnclose(chain, random)!;
  for (const circle of circles) {
    circle.x -= enclosing.x;
    circle.y -= enclosing.y;
  }
  return enclosing.r;
}

/** 🫧 Options of `\SemioVizLayout{pack}`. */
export type VizPackOptions<T> = { readonly size?: readonly [number, number]; readonly padding?: number; readonly radius?: (node: VizHierarchyNode<T>) => number };

/** 🫧 The enclosure diagram: nested circles proportional to `value`, packed deterministically. */
export function vizPack<T>(root: VizHierarchyNode<T>, options: VizPackOptions<T> = {}): VizHierarchyNode<T> {
  const [dx, dy] = options.size ?? [1, 1];
  const padding = options.padding ?? 0;
  const random = vizLcg();
  root.x = dx / 2;
  root.y = dy / 2;
  const radiusLeaf = (radius: (node: VizHierarchyNode<T>) => number) => (node: VizHierarchyNode<T>) => {
    if (!node.children || node.children.length === 0) node.r = Math.max(0, +radius(node) || 0);
  };
  const packChildren = (pad: number, k: number) => (node: VizHierarchyNode<T>) => {
    const children = node.children;
    if (!children || children.length === 0) return;
    const r = pad * k || 0;
    if (r) for (const child of children) child.r += r;
    const enclosing = vizPackSiblings(children as unknown as Circle[], random);
    if (r) for (const child of children) child.r -= r;
    node.r = enclosing + r;
  };
  const translateChild = (k: number) => (node: VizHierarchyNode<T>) => {
    const parent = node.parent;
    node.r *= k;
    if (parent !== null) {
      node.x = parent.x + k * node.x;
      node.y = parent.y + k * node.y;
    }
  };
  if (options.radius !== undefined) {
    root.eachBefore(radiusLeaf(options.radius));
    root.eachAfter(packChildren(padding, 0.5));
    root.eachBefore(translateChild(1));
  } else {
    root.eachBefore(radiusLeaf((node) => Math.sqrt(node.value ?? 0)));
    root.eachAfter(packChildren(0, 1));
    root.eachAfter(packChildren(padding, root.r / Math.min(dx, dy)));
    root.eachBefore(translateChild(Math.min(dx, dy) / (2 * root.r)));
  }
  return root;
}
//#endregion 🔖️Pack
