/** 🕸️ Networks: the TypeScript twin of `semio-viz-network`. A Barnes–Hut quadtree, the deterministic
 * force simulation with the six d3 forces, the chord layout, and the arc, circular and layered
 * (Sugiyama) graph layouts. Every random draw comes from the same seeded generator as
 * `semio-viz-network.sty`, so a run is reproducible across implementations.
 * @see ../../../🖋️latex/semio-viz-network.sty
 */
import { vizLcg } from "../🌳hierarchy/🟦️.ts";

//#region 🔖️Quadtree
/** 🌲 An internal quadtree node: four quadrants, any of them absent. */
export type VizQuadInternal<T> = [VizQuadNode<T> | undefined, VizQuadNode<T> | undefined, VizQuadNode<T> | undefined, VizQuadNode<T> | undefined] & { x?: number; y?: number; value?: number; r?: number };

/** 🌲 A leaf of the quadtree: one datum plus the coincident data chained behind it. */
export type VizQuadLeaf<T> = { data: T; next?: VizQuadLeaf<T>; x?: number; y?: number; value?: number; r?: number };

export type VizQuadNode<T> = VizQuadInternal<T> | VizQuadLeaf<T>;

function isInternal<T>(node: VizQuadNode<T>): node is VizQuadInternal<T> {
  return Array.isArray(node);
}

/** 🌲 The point quadtree d3-force's Barnes–Hut approximation and collision detection walk. */
export class VizQuadtree<T> {
  private readonly accessX: (d: T) => number;
  private readonly accessY: (d: T) => number;
  root: VizQuadNode<T> | undefined;
  x0 = Number.NaN;
  y0 = Number.NaN;
  x1 = Number.NaN;
  y1 = Number.NaN;

  constructor(x: (d: T) => number, y: (d: T) => number) {
    this.accessX = x;
    this.accessY = y;
  }

  /** 🌲 Grows the covered square until it contains the point. */
  cover(x: number, y: number): this {
    if (Number.isNaN(x) || Number.isNaN(y)) return this;
    if (Number.isNaN(this.x0)) {
      this.x0 = Math.floor(x);
      this.x1 = this.x0 + 1;
      this.y0 = Math.floor(y);
      this.y1 = this.y0 + 1;
      return this;
    }
    let z = this.x1 - this.x0 || 1;
    let node = this.root;
    while (this.x0 > x || x >= this.x1 || this.y0 > y || y >= this.y1) {
      const i = ((y < this.y0 ? 1 : 0) << 1) | (x < this.x0 ? 1 : 0);
      const parent = [undefined, undefined, undefined, undefined] as VizQuadInternal<T>;
      parent[i] = node;
      node = parent;
      z *= 2;
      if (i === 0) {
        this.x1 = this.x0 + z;
        this.y1 = this.y0 + z;
      } else if (i === 1) {
        this.x0 = this.x1 - z;
        this.y1 = this.y0 + z;
      } else if (i === 2) {
        this.x1 = this.x0 + z;
        this.y0 = this.y1 - z;
      } else {
        this.x0 = this.x1 - z;
        this.y0 = this.y1 - z;
      }
    }
    if (this.root !== undefined && isInternal(this.root)) this.root = node;
    return this;
  }

  private insert(x: number, y: number, d: T): void {
    if (Number.isNaN(x) || Number.isNaN(y)) return;
    const leaf: VizQuadLeaf<T> = { data: d };
    if (this.root === undefined) {
      this.root = leaf;
      return;
    }
    let node = this.root;
    let parent: VizQuadInternal<T> | undefined;
    let x0 = this.x0;
    let y0 = this.y0;
    let x1 = this.x1;
    let y1 = this.y1;
    let i = 0;
    while (isInternal(node)) {
      const xm = (x0 + x1) / 2;
      const ym = (y0 + y1) / 2;
      const right = x >= xm;
      const bottom = y >= ym;
      if (right) x0 = xm;
      else x1 = xm;
      if (bottom) y0 = ym;
      else y1 = ym;
      parent = node;
      i = (bottom ? 2 : 0) + (right ? 1 : 0);
      const child = node[i];
      if (child === undefined) {
        parent[i] = leaf;
        return;
      }
      node = child;
    }
    const xp = +this.accessX(node.data);
    const yp = +this.accessY(node.data);
    if (x === xp && y === yp) {
      leaf.next = node;
      if (parent !== undefined) parent[i] = leaf;
      else this.root = leaf;
      return;
    }
    let j: number;
    do {
      const created = [undefined, undefined, undefined, undefined] as VizQuadInternal<T>;
      if (parent !== undefined) parent[i] = created;
      else this.root = created;
      parent = created;
      const xm = (x0 + x1) / 2;
      const ym = (y0 + y1) / 2;
      const right = x >= xm;
      const bottom = y >= ym;
      if (right) x0 = xm;
      else x1 = xm;
      if (bottom) y0 = ym;
      else y1 = ym;
      i = (bottom ? 2 : 0) + (right ? 1 : 0);
      j = ((yp >= ym ? 1 : 0) << 1) | (xp >= xm ? 1 : 0);
    } while (i === j);
    parent[j] = node;
    parent[i] = leaf;
  }

  /** 🌲 Adds every datum, covering their bounding square first exactly like d3. */
  addAll(data: readonly T[]): this {
    const n = data.length;
    const xz = new Array<number>(n);
    const yz = new Array<number>(n);
    let x0 = Number.POSITIVE_INFINITY;
    let y0 = Number.POSITIVE_INFINITY;
    let x1 = Number.NEGATIVE_INFINITY;
    let y1 = Number.NEGATIVE_INFINITY;
    for (let i = 0; i < n; i += 1) {
      const x = +this.accessX(data[i]!);
      const y = +this.accessY(data[i]!);
      if (Number.isNaN(x) || Number.isNaN(y)) continue;
      xz[i] = x;
      yz[i] = y;
      if (x < x0) x0 = x;
      if (x > x1) x1 = x;
      if (y < y0) y0 = y;
      if (y > y1) y1 = y;
    }
    if (x0 > x1 || y0 > y1) return this;
    this.cover(x0, y0).cover(x1, y1);
    for (let i = 0; i < n; i += 1) this.insert(xz[i]!, yz[i]!, data[i]!);
    return this;
  }

  /** 🌲 Pre-order visit; returning `true` prunes the subtree. */
  visit(callback: (node: VizQuadNode<T>, x0: number, y0: number, x1: number, y1: number) => boolean | void): this {
    const quads: { node: VizQuadNode<T>; x0: number; y0: number; x1: number; y1: number }[] = [];
    if (this.root !== undefined) quads.push({ node: this.root, x0: this.x0, y0: this.y0, x1: this.x1, y1: this.y1 });
    for (let q = quads.pop(); q !== undefined; q = quads.pop()) {
      const pruned = callback(q.node, q.x0, q.y0, q.x1, q.y1);
      if (!pruned && isInternal(q.node)) {
        const xm = (q.x0 + q.x1) / 2;
        const ym = (q.y0 + q.y1) / 2;
        if (q.node[3]) quads.push({ node: q.node[3]!, x0: xm, y0: ym, x1: q.x1, y1: q.y1 });
        if (q.node[2]) quads.push({ node: q.node[2]!, x0: q.x0, y0: ym, x1: xm, y1: q.y1 });
        if (q.node[1]) quads.push({ node: q.node[1]!, x0: xm, y0: q.y0, x1: q.x1, y1: ym });
        if (q.node[0]) quads.push({ node: q.node[0]!, x0: q.x0, y0: q.y0, x1: xm, y1: ym });
      }
    }
    return this;
  }

  /** 🌲 Post-order visit, the pass that accumulates charge and radius. */
  visitAfter(callback: (node: VizQuadNode<T>, x0: number, y0: number, x1: number, y1: number) => void): this {
    const quads: { node: VizQuadNode<T>; x0: number; y0: number; x1: number; y1: number }[] = [];
    const next: { node: VizQuadNode<T>; x0: number; y0: number; x1: number; y1: number }[] = [];
    if (this.root !== undefined) quads.push({ node: this.root, x0: this.x0, y0: this.y0, x1: this.x1, y1: this.y1 });
    for (let q = quads.pop(); q !== undefined; q = quads.pop()) {
      if (isInternal(q.node)) {
        const xm = (q.x0 + q.x1) / 2;
        const ym = (q.y0 + q.y1) / 2;
        if (q.node[0]) quads.push({ node: q.node[0]!, x0: q.x0, y0: q.y0, x1: xm, y1: ym });
        if (q.node[1]) quads.push({ node: q.node[1]!, x0: xm, y0: q.y0, x1: q.x1, y1: ym });
        if (q.node[2]) quads.push({ node: q.node[2]!, x0: q.x0, y0: ym, x1: xm, y1: q.y1 });
        if (q.node[3]) quads.push({ node: q.node[3]!, x0: xm, y0: ym, x1: q.x1, y1: q.y1 });
      }
      next.push(q);
    }
    for (let q = next.pop(); q !== undefined; q = next.pop()) callback(q.node, q.x0, q.y0, q.x1, q.y1);
    return this;
  }
}

/** 🌲 Builds a quadtree over the data with the given coordinate accessors. */
export function vizQuadtree<T>(data: readonly T[], x: (d: T) => number, y: (d: T) => number): VizQuadtree<T> {
  return new VizQuadtree(x, y).addAll(data);
}
//#endregion 🔖️Quadtree

//#region 🔖️ForceModel
/** 🧲 A simulation node; the layout writes position and velocity back into it. */
export type VizForceNode = { index?: number; x: number; y: number; vx: number; vy: number; fx?: number | null; fy?: number | null };

/** 🧲 A simulation link between two nodes, given by index or by reference. */
export type VizForceLink<N extends VizForceNode> = { source: N | number; target: N | number; index?: number };

/** 🧲 One force: it is initialised with the node array once and applied every tick. */
export type VizForce<N extends VizForceNode> = { initialize?(nodes: N[], random: () => number): void; apply(alpha: number): void };

const INITIAL_RADIUS = 10;
const INITIAL_ANGLE = Math.PI * (3 - Math.sqrt(5));

function jiggle(random: () => number): number {
  return (random() - 0.5) * 1e-6;
}

/** 🧲 Places nodes on d3's phyllotaxis spiral — the deterministic initial configuration. */
export function initializeVizForceNodes<N extends VizForceNode>(nodes: N[]): void {
  nodes.forEach((node, i) => {
    node.index = i;
    if (node.fx !== undefined && node.fx !== null) node.x = node.fx;
    if (node.fy !== undefined && node.fy !== null) node.y = node.fy;
    if (Number.isNaN(node.x) || Number.isNaN(node.y)) {
      const radius = INITIAL_RADIUS * Math.sqrt(0.5 + i);
      const angle = i * INITIAL_ANGLE;
      node.x = radius * Math.cos(angle);
      node.y = radius * Math.sin(angle);
    }
    if (Number.isNaN(node.vx) || Number.isNaN(node.vy)) {
      node.vx = 0;
      node.vy = 0;
    }
  });
}

/** 🧲 Options of `\SemioVizLayout{force}`; the defaults are d3's. */
export type VizSimulationOptions = { readonly alpha?: number; readonly alphaMin?: number; readonly alphaDecay?: number; readonly alphaTarget?: number; readonly velocityDecay?: number; readonly seed?: number };

/** 🧲 A deterministic force simulation: no clock, only an explicit number of ticks. */
export class VizForceSimulation<N extends VizForceNode> {
  readonly nodes: N[];
  readonly random: () => number;
  alpha: number;
  readonly alphaMin: number;
  readonly alphaDecay: number;
  readonly alphaTarget: number;
  readonly velocityDecay: number;
  private readonly forces = new Map<string, VizForce<N>>();

  constructor(nodes: N[], options: VizSimulationOptions = {}) {
    this.nodes = nodes;
    this.random = vizLcg(options.seed ?? 1);
    this.alpha = options.alpha ?? 1;
    this.alphaMin = options.alphaMin ?? 0.001;
    this.alphaDecay = options.alphaDecay ?? 1 - this.alphaMin ** (1 / 300);
    this.alphaTarget = options.alphaTarget ?? 0;
    this.velocityDecay = 1 - (options.velocityDecay ?? 0.4);
    initializeVizForceNodes(nodes);
  }

  /** 🧲 Registers a named force, in the order the simulation will apply it. */
  force(name: string, force: VizForce<N>): this {
    this.forces.set(name, force);
    force.initialize?.(this.nodes, this.random);
    return this;
  }

  /** 🧲 Advances the simulation by a fixed number of ticks. */
  tick(iterations = 1): this {
    for (let k = 0; k < iterations; k += 1) {
      this.alpha += (this.alphaTarget - this.alpha) * this.alphaDecay;
      for (const force of this.forces.values()) force.apply(this.alpha);
      for (const node of this.nodes) {
        if (node.fx === undefined || node.fx === null) {
          node.vx *= this.velocityDecay;
          node.x += node.vx;
        } else {
          node.x = node.fx;
          node.vx = 0;
        }
        if (node.fy === undefined || node.fy === null) {
          node.vy *= this.velocityDecay;
          node.y += node.vy;
        } else {
          node.y = node.fy;
          node.vy = 0;
        }
      }
    }
    return this;
  }

  /** 🧲 Runs until alpha falls below `alphaMin`, the settled configuration d3 would reach. */
  settle(): this {
    while (this.alpha >= this.alphaMin) this.tick(1);
    return this;
  }
}
//#endregion 🔖️ForceModel

//#region 🔖️Forces
/** 🧲 `link`: a spring pulling each edge toward its rest distance. */
export function forceVizLink<N extends VizForceNode>(links: VizForceLink<N>[], options: { distance?: number | ((link: VizForceLink<N>, index: number) => number); strength?: number | ((link: VizForceLink<N>, index: number) => number); iterations?: number } = {}): VizForce<N> {
  let nodes: N[] = [];
  let random: () => number = vizLcg();
  let distances: number[] = [];
  let strengths: number[] = [];
  let bias: number[] = [];
  const iterations = options.iterations ?? 1;
  const resolve = (endpoint: N | number): N => (typeof endpoint === "number" ? nodes[endpoint]! : endpoint);
  return {
    initialize(allNodes, source) {
      nodes = allNodes;
      random = source;
      const count = new Array<number>(nodes.length).fill(0);
      links.forEach((link, i) => {
        link.index = i;
        link.source = resolve(link.source);
        link.target = resolve(link.target);
        count[(link.source as N).index!] = (count[(link.source as N).index!] ?? 0) + 1;
        count[(link.target as N).index!] = (count[(link.target as N).index!] ?? 0) + 1;
      });
      bias = links.map((link) => {
        const s = count[(link.source as N).index!]!;
        const t = count[(link.target as N).index!]!;
        return s / (s + t);
      });
      strengths = links.map((link, i) => (typeof options.strength === "function" ? options.strength(link, i) : (options.strength ?? 1 / Math.min(count[(link.source as N).index!]!, count[(link.target as N).index!]!))));
      distances = links.map((link, i) => (typeof options.distance === "function" ? options.distance(link, i) : (options.distance ?? 30)));
    },
    apply(alpha) {
      for (let k = 0; k < iterations; k += 1) {
        links.forEach((link, i) => {
          const source = link.source as N;
          const target = link.target as N;
          let x = target.x + target.vx - source.x - source.vx || jiggle(random);
          let y = target.y + target.vy - source.y - source.vy || jiggle(random);
          let l = Math.sqrt(x * x + y * y);
          l = ((l - distances[i]!) / l) * alpha * strengths[i]!;
          x *= l;
          y *= l;
          let b = bias[i]!;
          target.vx -= x * b;
          target.vy -= y * b;
          b = 1 - b;
          source.vx += x * b;
          source.vy += y * b;
        });
      }
    },
  };
}

/** 🧲 `many-body`: the Barnes–Hut n-body force, repulsive at a negative strength. */
export function forceVizManyBody<N extends VizForceNode>(options: { strength?: number | ((node: N, index: number) => number); theta?: number; distanceMin?: number; distanceMax?: number } = {}): VizForce<N> {
  let nodes: N[] = [];
  let random: () => number = vizLcg();
  let strengths: number[] = [];
  const theta2 = (options.theta ?? 0.9) ** 2;
  const distanceMin2 = (options.distanceMin ?? 1) ** 2;
  const distanceMax2 = (options.distanceMax ?? Number.POSITIVE_INFINITY) ** 2;
  return {
    initialize(allNodes, source) {
      nodes = allNodes;
      random = source;
      strengths = nodes.map((node, i) => (typeof options.strength === "function" ? options.strength(node, i) : (options.strength ?? -30)));
    },
    apply(alpha) {
      const tree = vizQuadtree(
        nodes,
        (node) => node.x,
        (node) => node.y,
      );
      tree.visitAfter((quad) => {
        let strength = 0;
        if (isInternal(quad)) {
          let weight = 0;
          let x = 0;
          let y = 0;
          for (let i = 0; i < 4; i += 1) {
            const q = quad[i];
            if (q === undefined) continue;
            const c = Math.abs(q.value ?? 0);
            if (!c) continue;
            strength += q.value!;
            weight += c;
            x += c * q.x!;
            y += c * q.y!;
          }
          quad.x = x / weight;
          quad.y = y / weight;
        } else {
          let leaf: VizQuadLeaf<N> | undefined = quad;
          quad.x = quad.data.x;
          quad.y = quad.data.y;
          while (leaf !== undefined) {
            strength += strengths[leaf.data.index!]!;
            leaf = leaf.next;
          }
        }
        quad.value = strength;
      });
      for (const node of nodes) {
        tree.visit((quad, x1, _y1, x2) => {
          if (!quad.value) return true;
          let x = quad.x! - node.x;
          let y = quad.y! - node.y;
          const w = x2 - x1;
          let l = x * x + y * y;
          if ((w * w) / theta2 < l) {
            if (l < distanceMax2) {
              if (x === 0) {
                x = jiggle(random);
                l += x * x;
              }
              if (y === 0) {
                y = jiggle(random);
                l += y * y;
              }
              if (l < distanceMin2) l = Math.sqrt(distanceMin2 * l);
              node.vx += (x * quad.value! * alpha) / l;
              node.vy += (y * quad.value! * alpha) / l;
            }
            return true;
          }
          if (isInternal(quad) || l >= distanceMax2) return false;
          if (quad.data !== node || quad.next !== undefined) {
            if (x === 0) {
              x = jiggle(random);
              l += x * x;
            }
            if (y === 0) {
              y = jiggle(random);
              l += y * y;
            }
            if (l < distanceMin2) l = Math.sqrt(distanceMin2 * l);
          }
          let leaf: VizQuadLeaf<N> | undefined = quad;
          while (leaf !== undefined) {
            if (leaf.data !== node) {
              const k = (strengths[leaf.data.index!]! * alpha) / l;
              node.vx += x * k;
              node.vy += y * k;
            }
            leaf = leaf.next;
          }
          return false;
        });
      }
    },
  };
}

/** 🧲 `center`: recentres the whole configuration without changing velocities. */
export function forceVizCenter<N extends VizForceNode>(x = 0, y = 0, strength = 1): VizForce<N> {
  let nodes: N[] = [];
  return {
    initialize(allNodes) {
      nodes = allNodes;
    },
    apply() {
      const n = nodes.length;
      if (n === 0) return;
      let sx = 0;
      let sy = 0;
      for (const node of nodes) {
        sx += node.x;
        sy += node.y;
      }
      sx = (sx / n - x) * strength;
      sy = (sy / n - y) * strength;
      for (const node of nodes) {
        node.x -= sx;
        node.y -= sy;
      }
    },
  };
}

/** 🧲 `collide`: keeps circles of the given radius from overlapping. */
export function forceVizCollide<N extends VizForceNode>(options: { radius?: number | ((node: N, index: number) => number); strength?: number; iterations?: number } = {}): VizForce<N> {
  let nodes: N[] = [];
  let random: () => number = vizLcg();
  let radii: number[] = [];
  const strength = options.strength ?? 1;
  const iterations = options.iterations ?? 1;
  return {
    initialize(allNodes, source) {
      nodes = allNodes;
      random = source;
      radii = nodes.map((node, i) => (typeof options.radius === "function" ? options.radius(node, i) : (options.radius ?? 1)));
    },
    apply() {
      for (let k = 0; k < iterations; k += 1) {
        const tree = vizQuadtree(
          nodes,
          (node) => node.x + node.vx,
          (node) => node.y + node.vy,
        );
        tree.visitAfter((quad) => {
          if (!isInternal(quad)) {
            quad.r = radii[quad.data.index!]!;
            return;
          }
          quad.r = 0;
          for (let i = 0; i < 4; i += 1) {
            const child = quad[i];
            if (child !== undefined && (child.r ?? 0) > quad.r!) quad.r = child.r;
          }
        });
        for (const node of nodes) {
          const ri = radii[node.index!]!;
          const ri2 = ri * ri;
          const xi = node.x + node.vx;
          const yi = node.y + node.vy;
          tree.visit((quad, x0, y0, x1, y1) => {
            let rj = quad.r ?? 0;
            const r = ri + rj;
            if (!isInternal(quad)) {
              const data = quad.data;
              if (data.index! > node.index!) {
                let x = xi - data.x - data.vx;
                let y = yi - data.y - data.vy;
                let l = x * x + y * y;
                if (l < r * r) {
                  if (x === 0) {
                    x = jiggle(random);
                    l += x * x;
                  }
                  if (y === 0) {
                    y = jiggle(random);
                    l += y * y;
                  }
                  l = Math.sqrt(l);
                  l = ((r - l) / l) * strength;
                  x *= l;
                  y *= l;
                  rj *= rj;
                  let share = rj / (ri2 + rj);
                  node.vx += x * share;
                  node.vy += y * share;
                  share = 1 - share;
                  data.vx -= x * share;
                  data.vy -= y * share;
                }
              }
              return false;
            }
            return x0 > xi + r || x1 < xi - r || y0 > yi + r || y1 < yi - r;
          });
        }
      }
    },
  };
}

/** 🧲 `x`: pulls nodes toward a vertical line. */
export function forceVizX<N extends VizForceNode>(options: { x?: number | ((node: N, index: number) => number); strength?: number | ((node: N, index: number) => number) } = {}): VizForce<N> {
  let nodes: N[] = [];
  let xz: number[] = [];
  let strengths: number[] = [];
  return {
    initialize(allNodes) {
      nodes = allNodes;
      xz = nodes.map((node, i) => (typeof options.x === "function" ? options.x(node, i) : (options.x ?? 0)));
      strengths = nodes.map((node, i) => (typeof options.strength === "function" ? options.strength(node, i) : (options.strength ?? 0.1)));
    },
    apply(alpha) {
      nodes.forEach((node, i) => {
        node.vx += (xz[i]! - node.x) * strengths[i]! * alpha;
      });
    },
  };
}

/** 🧲 `y`: pulls nodes toward a horizontal line. */
export function forceVizY<N extends VizForceNode>(options: { y?: number | ((node: N, index: number) => number); strength?: number | ((node: N, index: number) => number) } = {}): VizForce<N> {
  let nodes: N[] = [];
  let yz: number[] = [];
  let strengths: number[] = [];
  return {
    initialize(allNodes) {
      nodes = allNodes;
      yz = nodes.map((node, i) => (typeof options.y === "function" ? options.y(node, i) : (options.y ?? 0)));
      strengths = nodes.map((node, i) => (typeof options.strength === "function" ? options.strength(node, i) : (options.strength ?? 0.1)));
    },
    apply(alpha) {
      nodes.forEach((node, i) => {
        node.vy += (yz[i]! - node.y) * strengths[i]! * alpha;
      });
    },
  };
}

/** 🧲 `radial`: pulls nodes toward a circle around a centre. */
export function forceVizRadial<N extends VizForceNode>(radius: number | ((node: N, index: number) => number), x = 0, y = 0, strength: number | ((node: N, index: number) => number) = 0.1): VizForce<N> {
  let nodes: N[] = [];
  let radii: number[] = [];
  let strengths: number[] = [];
  return {
    initialize(allNodes) {
      nodes = allNodes;
      radii = nodes.map((node, i) => (typeof radius === "function" ? radius(node, i) : radius));
      strengths = nodes.map((node, i) => (typeof strength === "function" ? strength(node, i) : strength));
    },
    apply(alpha) {
      nodes.forEach((node, i) => {
        const dx = node.x - x || 1e-6;
        const dy = node.y - y || 1e-6;
        const r = Math.sqrt(dx * dx + dy * dy);
        const k = ((radii[i]! - r) * strengths[i]! * alpha) / r;
        node.vx += dx * k;
        node.vy += dy * k;
      });
    },
  };
}
//#endregion 🔖️Forces

//#region 🔖️Chord
/** 🎻 One angular span of a chord diagram. */
export type VizChordSpan = { readonly index: number; readonly startAngle: number; readonly endAngle: number; readonly value: number };

/** 🎻 One chord: the two spans it joins, the larger one first. */
export type VizChordArc = { source: VizChordSpan; target: VizChordSpan };

/** 🎻 The chord layout of a square matrix. */
export type VizChordLayout = { readonly chords: readonly VizChordArc[]; readonly groups: readonly VizChordSpan[] };

const TAU = 2 * Math.PI;

/** 🎻 `\SemioVizLayout{chord}`: turns a flow matrix into groups and chords on the circle. */
export function vizChord(matrixIn: readonly (readonly number[])[], options: { padAngle?: number; sortGroups?: (a: number, b: number) => number; sortSubgroups?: (a: number, b: number) => number; sortChords?: (a: VizChordArc, b: VizChordArc) => number; transpose?: boolean } = {}): VizChordLayout {
  const n = matrixIn.length;
  const padAngle = options.padAngle ?? 0;
  const matrix = new Float64Array(n * n);
  for (let i = 0; i < n * n; i += 1) matrix[i] = options.transpose === true ? matrixIn[i % n]![(i / n) | 0]! : matrixIn[(i / n) | 0]![i % n]!;
  const groupSums = new Array<number>(n);
  const groupIndex = Array.from({ length: n }, (_, i) => i);
  const chords = new Array<VizChordArc | undefined>(n * n);
  const groups = new Array<VizChordSpan>(n);
  let k = 0;
  for (let i = 0; i < n; i += 1) {
    let total = 0;
    for (let j = 0; j < n; j += 1) total += matrix[i * n + j]!;
    groupSums[i] = total;
    k += total;
  }
  k = Math.max(0, TAU - padAngle * n) / k;
  const dx = k ? padAngle : TAU / n;
  let x = 0;
  if (options.sortGroups) groupIndex.sort((a, b) => options.sortGroups!(groupSums[a]!, groupSums[b]!));
  for (const i of groupIndex) {
    const x0 = x;
    const subgroupIndex = Array.from({ length: n }, (_, j) => j).filter((j) => matrix[i * n + j] || matrix[j * n + i]);
    if (options.sortSubgroups) subgroupIndex.sort((a, b) => options.sortSubgroups!(matrix[i * n + a]!, matrix[i * n + b]!));
    for (const j of subgroupIndex) {
      let chord: VizChordArc;
      if (i < j) {
        chord = chords[i * n + j] ?? { source: null as unknown as VizChordSpan, target: null as unknown as VizChordSpan };
        chords[i * n + j] = chord;
        const start = x;
        x += matrix[i * n + j]! * k;
        chord.source = { index: i, startAngle: start, endAngle: x, value: matrix[i * n + j]! };
      } else {
        chord = chords[j * n + i] ?? { source: null as unknown as VizChordSpan, target: null as unknown as VizChordSpan };
        chords[j * n + i] = chord;
        const start = x;
        x += matrix[i * n + j]! * k;
        chord.target = { index: i, startAngle: start, endAngle: x, value: matrix[i * n + j]! };
        if (i === j) chord.source = chord.target;
      }
      if (chord.source !== null && chord.target !== null && chord.source !== undefined && chord.target !== undefined && chord.source.value < chord.target.value) {
        const source = chord.source;
        chord.source = chord.target;
        chord.target = source;
      }
    }
    groups[i] = { index: i, startAngle: x0, endAngle: x, value: groupSums[i]! };
    x += dx;
  }
  const dense = chords.filter((chord): chord is VizChordArc => chord !== undefined);
  return { chords: options.sortChords ? [...dense].sort(options.sortChords) : dense, groups };
}
//#endregion 🔖️Chord

//#region 🔖️GraphLayouts
/** 🕸️ A plain graph the deterministic layouts consume. */
export type VizGraph = { readonly nodes: readonly string[]; readonly edges: readonly (readonly [string, string])[] };

/** 🕸️ A laid-out node: its identifier and its position in figure millimetres. */
export type VizPlacedNode = { readonly id: string; readonly x: number; readonly y: number; readonly layer?: number; readonly order?: number };

/** 🕸️ `circular`: nodes evenly spaced on a circle, in the given order. */
export function vizCircularLayout(graph: VizGraph, options: { radius?: number; cx?: number; cy?: number; startAngle?: number } = {}): VizPlacedNode[] {
  const radius = options.radius ?? 1;
  const cx = options.cx ?? 0;
  const cy = options.cy ?? 0;
  const startAngle = options.startAngle ?? 0;
  const n = graph.nodes.length;
  return graph.nodes.map((id, i) => {
    const angle = startAngle + (TAU * i) / n - Math.PI / 2;
    return { id, x: cx + radius * Math.cos(angle), y: cy + radius * Math.sin(angle), order: i };
  });
}

/** 🕸️ `arc`: nodes on one axis, edges drawn as semicircles above it. */
export function vizArcLayout(graph: VizGraph, options: { length?: number; origin?: number; vertical?: boolean } = {}): VizPlacedNode[] {
  const length = options.length ?? 1;
  const origin = options.origin ?? 0;
  const n = graph.nodes.length;
  const step = n > 1 ? length / (n - 1) : 0;
  return graph.nodes.map((id, i) => (options.vertical === true ? { id, x: origin, y: origin + i * step, order: i } : { id, x: origin + i * step, y: origin, order: i }));
}

/** 🕸️ The layered result: nodes per layer, plus the dummy chain of every long edge. */
export type VizLayeredLayout = { readonly nodes: readonly VizPlacedNode[]; readonly layers: readonly (readonly string[])[]; readonly dummies: readonly (readonly VizPlacedNode[])[] };

/** 🕸️ `layered` (Sugiyama): longest-path layering, barycentre ordering sweeps, then priority
 * coordinate assignment. Deterministic: the sweep count is fixed and ties break on the input order. */
export function vizLayeredLayout(graph: VizGraph, options: { layerGap?: number; nodeGap?: number; sweeps?: number } = {}): VizLayeredLayout {
  const layerGap = options.layerGap ?? 1;
  const nodeGap = options.nodeGap ?? 1;
  const sweeps = options.sweeps ?? 8;
  const outgoing = new Map<string, string[]>();
  const incoming = new Map<string, string[]>();
  for (const id of graph.nodes) {
    outgoing.set(id, []);
    incoming.set(id, []);
  }
  const acyclic: [string, string][] = [];
  const seen = new Set<string>();
  const order = new Map(graph.nodes.map((id, i) => [id, i] as const));
  for (const [source, target] of graph.edges) {
    const key = `${source} ${target}`;
    if (seen.has(key) || source === target) continue;
    seen.add(key);
    const forward = (order.get(source) ?? 0) <= (order.get(target) ?? 0);
    const edge: [string, string] = forward ? [source, target] : [target, source];
    acyclic.push(edge);
    outgoing.get(edge[0])!.push(edge[1]);
    incoming.get(edge[1])!.push(edge[0]);
  }
  const layer = new Map<string, number>();
  const assign = (id: string, guard: Set<string>): number => {
    const known = layer.get(id);
    if (known !== undefined) return known;
    if (guard.has(id)) return 0;
    guard.add(id);
    let best = 0;
    for (const parent of incoming.get(id) ?? []) best = Math.max(best, assign(parent, guard) + 1);
    guard.delete(id);
    layer.set(id, best);
    return best;
  };
  for (const id of graph.nodes) assign(id, new Set());
  const depth = Math.max(0, ...graph.nodes.map((id) => layer.get(id)!)) + 1;
  const layers: string[][] = Array.from({ length: depth }, () => []);
  for (const id of graph.nodes) layers[layer.get(id)!]!.push(id);
  const position = new Map<string, number>();
  for (const rows of layers) rows.forEach((id, i) => position.set(id, i));
  const barycentre = (id: string, neighbours: Map<string, string[]>): number => {
    const list = neighbours.get(id) ?? [];
    if (list.length === 0) return position.get(id)!;
    return list.reduce((total, other) => total + (position.get(other) ?? 0), 0) / list.length;
  };
  for (let sweep = 0; sweep < sweeps; sweep += 1) {
    const neighbours = sweep % 2 === 0 ? incoming : outgoing;
    const range = sweep % 2 === 0 ? layers : [...layers].reverse();
    for (const rows of range) {
      const scored = rows.map((id, i) => ({ id, key: barycentre(id, neighbours), tie: i }));
      scored.sort((a, b) => a.key - b.key || a.tie - b.tie);
      rows.splice(0, rows.length, ...scored.map((entry) => entry.id));
      rows.forEach((id, i) => position.set(id, i));
    }
  }
  const widest = Math.max(1, ...layers.map((rows) => rows.length));
  const nodes: VizPlacedNode[] = [];
  layers.forEach((rows, l) => {
    const offset = ((widest - rows.length) * nodeGap) / 2;
    rows.forEach((id, i) => nodes.push({ id, x: offset + i * nodeGap, y: l * layerGap, layer: l, order: i }));
  });
  const byId = new Map(nodes.map((node) => [node.id, node] as const));
  const dummies = acyclic
    .filter(([source, target]) => layer.get(target)! - layer.get(source)! > 1)
    .map(([source, target]) => {
      const from = byId.get(source)!;
      const to = byId.get(target)!;
      const gap = layer.get(target)! - layer.get(source)!;
      return Array.from({ length: gap - 1 }, (_, step) => {
        const t = (step + 1) / gap;
        return { id: `${source}→${target}#${step}`, x: from.x + (to.x - from.x) * t, y: from.y + (to.y - from.y) * t, layer: layer.get(source)! + step + 1 };
      });
    });
  return { nodes, layers, dummies };
}
//#endregion 🔖️GraphLayouts
