/** 🌲️ The Reingold–Tilford/Buchheim tidy tree and the dendrogram cluster layouts of the print hierarchy twin. */
import { VizHierarchyNode } from "../🟦️.ts";

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

/** 🌴️ The tidy tree layout of Reingold, Tilford and Buchheim. */
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

/** 📏️ The dendrogram cluster layout: every leaf on the same baseline. */
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
