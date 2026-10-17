/** 🌳 Hierarchies: the TypeScript twin of `semio-viz-hierarchy`. Node construction and stratify, the traversal and aggregation vocabulary and the seeded random stream shared with the layouts.
 * @see ../../../🖋️latex/semio-viz-hierarchy.sty
 * @see ./🌲️tidy-tree-and-cluster/🟦️.ts — the Reingold–Tilford/Buchheim tree and the dendrogram cluster
 * @see ./🧱️treemap-tilings-and-partition/🟦️.ts — all six treemap tilings and the icicle partition
 * @see ./🫧️circle-packing/🟦️.ts — the Welzl circle packing, driven by the same seeded linear congruential generator as the LaTeX kernel
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

export function shuffleWith<T>(values: T[], random: () => number): T[] {
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
