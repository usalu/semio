/** 🫧️ The Welzl circle packing of the print hierarchy twin: sibling placement, enclosing circles and the seeded pack layout. */
import { shuffleWith, VizHierarchyNode, vizLcg } from "../🟦️.ts";

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
