/** 🌊️ The air around a set of meshes (C31): every triangle is a wall, the space between the walls is flooded from outside the bounding box, and a point is reached when the flood arrives at it without crossing a triangle.
 *
 * The space is a sparse grid of two levels: blocks of {@link FLOOD_BLOCK}³ cells; a block no triangle touches is one node of the flood, a block with triangles keeps its cells as rows of bits along x, and the flood runs row by row. A cell that a triangle touches (separating-axis test of the triangle against the closed cell) is a wall cell, so a gap of at most one cell is always closed and a gap of two cells is always open. A point in a wall cell is reached when it sees the centre of a reached cell at most {@link FLOOD_LIMITS}.sightCells away along a straight segment that no triangle crosses (exact, so a wall thinner than a cell still separates).
 *
 * THE REFERENCE is the connected-component labelling of scipy on a trimesh voxelisation in `🧪️tests/🧊️volumes-flood/🐍️.py`.
 *
 * @see ../🧮️measures/🟦️.ts — `outsideOf`, which decides with it what counts as outer surface (C31) and which wall of a region lies behind a face in a closed cavity (E3)
 * @see https://fileadmin.cs.lth.se/cs/Personal/Tomas_Akenine-Moller/code/tribox_tam.pdf — the triangle-box overlap test */

type Vec = readonly [number, number, number];

/** 🧱️ A mesh of the flood: three coordinates per vertex, three vertex indices per triangle. */
export type FloodMesh = { readonly positions: Float64Array; readonly indices: Uint32Array };

/** 🧊️ How many cells a block spans along each axis. */
export const FLOOD_BLOCK = 8;

/** 🎚️ The budget of the flood: at most `fineBlocks` blocks with cells and `coarseBlocks` blocks in all (the cell grows until both hold), and how many cells away a point in a wall cell looks for a free cell. */
export const FLOOD_LIMITS = { fineBlocks: 600_000, coarseBlocks: 8_000_000, sightCells: 2 } as const;

/** 🌊️ The flooded space: `cellM` is the cell the budget allowed, `reaches` says whether the air from outside arrives at the point `p` in front of a face with the outward unit normal `n`, `firstHit` is the index of the mesh whose triangle the segment from `p` to `q` crosses first (-1 when none), `crossings` are the fractions along the segment from `p` to `q` at which it crosses a triangle of a mesh whose index `keep` accepts, ascending (E3: the wall of a region behind a face). */
export type Flood = { readonly cellM: number; readonly reaches: (p: Vec, n: Vec) => boolean; readonly firstHit: (p: Vec, q: Vec) => number; readonly crossings: (p: Vec, q: Vec, keep: (mesh: number) => boolean) => number[] };

const ROWS = FLOOD_BLOCK * FLOOD_BLOCK;
const FULL = (1 << FLOOD_BLOCK) - 1;
const LAST = 1 << (FLOOD_BLOCK - 1);
const CHUNK = 1024;
const JITTER: Vec = [0.3719, 0.6143, 0.4327];
const EDGE = 1e-7;
const [FREE, WALL, REACHED] = [0, 1, 2];

const OFFSETS: readonly (readonly [number, number, number])[] = (() => {
  const r = FLOOD_LIMITS.sightCells;
  const out: [number, number, number][] = [];
  for (let k = -r; k <= r; k++) for (let j = -r; j <= r; j++) for (let i = -r; i <= r; i++) if (i !== 0 || j !== 0 || k !== 0) out.push([i, j, k]);
  return out.sort((a, b) => a[0] * a[0] + a[1] * a[1] + a[2] * a[2] - (b[0] * b[0] + b[1] * b[1] + b[2] * b[2]));
})();

/** 🥞️ A growing stack of integers. */
class Stack {
  data = new Int32Array(1 << 16);
  top = 0;
  push(value: number): void {
    if (this.top === this.data.length) {
      const grown = new Int32Array(this.data.length * 2);
      grown.set(this.data);
      this.data = grown;
    }
    this.data[this.top++] = value;
  }
}

/** 📐️ The area in m² of the triangles of a mesh before the index `end`. */
function meshArea(mesh: FloodMesh, end: number): number {
  const q = mesh.positions;
  let twice = 0;
  for (let t = 0; t < end; t += 3) {
    const a = mesh.indices[t]! * 3, b = mesh.indices[t + 1]! * 3, c = mesh.indices[t + 2]! * 3;
    const ux = q[b]! - q[a]!, uy = q[b + 1]! - q[a + 1]!, uz = q[b + 2]! - q[a + 2]!, vx = q[c]! - q[a]!, vy = q[c + 1]! - q[a + 1]!, vz = q[c + 2]! - q[a + 2]!;
    twice += Math.hypot(uy * vz - uz * vy, uz * vx - ux * vz, ux * vy - uy * vx);
  }
  return twice / 2;
}

/** 🏗️ The flood at the cell `h`, or the factor by which the cell has to grow to fit the budget. */
function build(walls: readonly FloodMesh[], h: number, tick: () => void, limits: { readonly fineBlocks: number; readonly coarseBlocks: number }): Flood | number {
  const min = [Infinity, Infinity, Infinity];
  const max = [-Infinity, -Infinity, -Infinity];
  const base: number[] = [];
  let triangles = 0;
  let surface = 0;
  for (const wall of walls) {
    base.push(triangles);
    triangles += wall.indices.length / 3;
    surface += meshArea(wall, wall.indices.length);
    for (let i = 0; i < wall.indices.length; i++) for (let k = 0; k < 3; k++) {
      const v = wall.positions[wall.indices[i]! * 3 + k]!;
      if (v < min[k]!) min[k] = v;
      if (v > max[k]!) max[k] = v;
    }
  }
  if (triangles === 0) return { cellM: h, reaches: () => true, firstHit: () => -1, crossings: () => [] };
  const s = FLOOD_BLOCK * h;
  const origin = [0, 1, 2].map((k) => min[k]! - s - JITTER[k]! * h);
  const [nx, ny, nz] = [0, 1, 2].map((k) => Math.floor((max[k]! - origin[k]!) / s) + 2) as [number, number, number];
  if (nx * ny * nz > limits.coarseBlocks) return Math.cbrt((nx * ny * nz) / limits.coarseBlocks) * 1.05;
  const slot = new Int32Array(nx * ny * nz).fill(-1);
  const wallRows: Uint8Array[] = [];
  const reachedRows: Uint8Array[] = [];
  const blockOf: number[] = [];
  const lastTriangle: number[] = [];
  const pairs = new Stack();
  const owner = new Int32Array(triangles);
  let count = 0;
  let over = false;
  let g = 0;
  const [a, e, n] = [new Float64Array(9), new Float64Array(9), new Float64Array(3)];
  const overlaps = (cx: number, cy: number, cz: number, hx: number, hy: number, hz: number): boolean => {
    const x0 = a[0]! - cx, y0 = a[1]! - cy, z0 = a[2]! - cz, x1 = a[3]! - cx, y1 = a[4]! - cy, z1 = a[5]! - cz, x2 = a[6]! - cx, y2 = a[7]! - cy, z2 = a[8]! - cz;
    if ((x0 > hx && x1 > hx && x2 > hx) || (x0 < -hx && x1 < -hx && x2 < -hx) || (y0 > hy && y1 > hy && y2 > hy) || (y0 < -hy && y1 < -hy && y2 < -hy) || (z0 > hz && z1 > hz && z2 > hz) || (z0 < -hz && z1 < -hz && z2 < -hz)) return false;
    if (Math.abs(n[0]! * x0 + n[1]! * y0 + n[2]! * z0) > hx * Math.abs(n[0]!) + hy * Math.abs(n[1]!) + hz * Math.abs(n[2]!)) return false;
    for (let k = 0; k < 9; k += 3) {
      const ex = e[k]!, ey = e[k + 1]!, ez = e[k + 2]!;
      const ax = Math.abs(ex), ay = Math.abs(ey), az = Math.abs(ez);
      let p0 = ey * z0 - ez * y0, p1 = ey * z1 - ez * y1, p2 = ey * z2 - ez * y2, r = hy * az + hz * ay;
      if ((p0 > r && p1 > r && p2 > r) || (p0 < -r && p1 < -r && p2 < -r)) return false;
      p0 = ez * x0 - ex * z0;
      p1 = ez * x1 - ex * z1;
      p2 = ez * x2 - ex * z2;
      r = hx * az + hz * ax;
      if ((p0 > r && p1 > r && p2 > r) || (p0 < -r && p1 < -r && p2 < -r)) return false;
      p0 = ex * y0 - ey * x0;
      p1 = ex * y1 - ey * x1;
      p2 = ex * y2 - ey * x2;
      r = hx * ay + hy * ax;
      if ((p0 > r && p1 > r && p2 > r) || (p0 < -r && p1 < -r && p2 < -r)) return false;
    }
    return true;
  };
  const mark = (i: number, j: number, k: number): void => {
    const b = ((k >> 3) * ny + (j >> 3)) * nx + (i >> 3);
    let at = slot[b]!;
    if (at < 0) {
      if (count === limits.fineBlocks) {
        over = true;
        return;
      }
      at = count++;
      slot[b] = at;
      blockOf.push(b);
      lastTriangle.push(-1);
      if (at % CHUNK === 0) {
        wallRows.push(new Uint8Array(CHUNK * ROWS));
        reachedRows.push(new Uint8Array(CHUNK * ROWS));
      }
    }
    wallRows[(at / CHUNK) | 0]![(at % CHUNK) * ROWS + (((k & 7) << 3) | (j & 7))]! |= 1 << (i & 7);
    if (lastTriangle[at] !== g) {
      lastTriangle[at] = g;
      pairs.push(at);
      pairs.push(g);
    }
  };
  const raster = (i0: number, i1: number, j0: number, j1: number, k0: number, k1: number): void => {
    if (over || !overlaps((i0 + i1 + 1) / 2, (j0 + j1 + 1) / 2, (k0 + k1 + 1) / 2, (i1 - i0 + 1) / 2 + EDGE, (j1 - j0 + 1) / 2 + EDGE, (k1 - k0 + 1) / 2 + EDGE)) return;
    const di = i1 - i0, dj = j1 - j0, dk = k1 - k0;
    if (di === 0 && dj === 0 && dk === 0) return mark(i0, j0, k0);
    if (di >= dj && di >= dk) {
      const m = (i0 + i1) >> 1;
      raster(i0, m, j0, j1, k0, k1);
      raster(m + 1, i1, j0, j1, k0, k1);
    } else if (dj >= dk) {
      const m = (j0 + j1) >> 1;
      raster(i0, i1, j0, m, k0, k1);
      raster(i0, i1, m + 1, j1, k0, k1);
    } else {
      const m = (k0 + k1) >> 1;
      raster(i0, i1, j0, j1, k0, m);
      raster(i0, i1, j0, j1, m + 1, k1);
    }
  };
  for (let w = 0; w < walls.length && !over; w++) {
    const { positions: q, indices } = walls[w]!;
    for (let t = 0; t < indices.length && !over; t += 3, g++) {
      if ((g & 0x3ff) === 0) tick();
      owner[g] = w;
      for (let c = 0; c < 3; c++) for (let k = 0; k < 3; k++) a[c * 3 + k] = (q[indices[t + c]! * 3 + k]! - origin[k]!) / h;
      for (let k = 0; k < 3; k++) {
        e[k] = a[3 + k]! - a[k]!;
        e[3 + k] = a[6 + k]! - a[3 + k]!;
        e[6 + k] = a[k]! - a[6 + k]!;
      }
      n[0] = e[1]! * e[5]! - e[2]! * e[4]!;
      n[1] = e[2]! * e[3]! - e[0]! * e[5]!;
      n[2] = e[0]! * e[4]! - e[1]! * e[3]!;
      raster(Math.floor(Math.min(a[0]!, a[3]!, a[6]!) - EDGE), Math.floor(Math.max(a[0]!, a[3]!, a[6]!) + EDGE), Math.floor(Math.min(a[1]!, a[4]!, a[7]!) - EDGE), Math.floor(Math.max(a[1]!, a[4]!, a[7]!) + EDGE), Math.floor(Math.min(a[2]!, a[5]!, a[8]!) - EDGE), Math.floor(Math.max(a[2]!, a[5]!, a[8]!) + EDGE));
    }
  }
  if (over) {
    let seen = 0;
    for (let w = 0; w < walls.length && base[w]! < g; w++) seen += meshArea(walls[w]!, Math.min(walls[w]!.indices.length, (g - base[w]!) * 3));
    return Math.min(4, Math.max(1.15, Math.sqrt(surface / Math.max(seen, 1e-12)) * 1.05));
  }
  const start = new Int32Array(count + 1);
  for (let i = 0; i < pairs.top; i += 2) start[pairs.data[i]! + 1]!++;
  for (let i = 0; i < count; i++) start[i + 1]! += start[i]!;
  const listed = new Int32Array(pairs.top / 2);
  const fill = start.slice(0, count);
  for (let i = 0; i < pairs.top; i += 2) listed[fill[pairs.data[i]!]!++] = pairs.data[i + 1]!;
  const stack = new Stack();
  const layer = nx * ny;
  const seed = (at: number, row: number, bits: number): void => {
    const index = (at % CHUNK) * ROWS + row;
    const reached = reachedRows[(at / CHUNK) | 0]!;
    const add = bits & ~wallRows[(at / CHUNK) | 0]![index]! & ~reached[index]!;
    if (add !== 0) {
      reached[index]! |= add;
      stack.push(at * ROWS + row);
    }
  };
  const cross = (nb: number, row: number, bits: number): void => {
    const sn = slot[nb]!;
    if (sn === -1) {
      slot[nb] = -2;
      stack.push(-(nb + 1));
    } else if (sn >= 0) seed(sn, row, bits);
  };
  const enter = (nb: number, axis: number, side: number): void => {
    const sn = slot[nb]!;
    if (sn === -1) {
      slot[nb] = -2;
      stack.push(-(nb + 1));
    } else if (sn >= 0) {
      const fixed = side > 0 ? 0 : FLOOD_BLOCK - 1;
      if (axis === 0) for (let row = 0; row < ROWS; row++) seed(sn, row, side > 0 ? 1 : LAST);
      else for (let r = 0; r < FLOOD_BLOCK; r++) seed(sn, axis === 1 ? (r << 3) | fixed : (fixed << 3) | r, FULL);
    }
  };
  slot[0] = -2;
  stack.push(-1);
  let popped = 0;
  while (stack.top > 0) {
    if ((++popped & 0xffff) === 0) tick();
    const v = stack.data[--stack.top]!;
    if (v < 0) {
      const b = -v - 1;
      const bx = b % nx, by = ((b / nx) | 0) % ny, bz = (b / layer) | 0;
      if (bx > 0) enter(b - 1, 0, -1);
      if (bx + 1 < nx) enter(b + 1, 0, 1);
      if (by > 0) enter(b - nx, 1, -1);
      if (by + 1 < ny) enter(b + nx, 1, 1);
      if (bz > 0) enter(b - layer, 2, -1);
      if (bz + 1 < nz) enter(b + layer, 2, 1);
      continue;
    }
    const at = (v / ROWS) | 0;
    const row = v % ROWS;
    const index = (at % CHUNK) * ROWS + row;
    const reached = reachedRows[(at / CHUNK) | 0]!;
    const free = ~wallRows[(at / CHUNK) | 0]![index]! & FULL;
    let m = reached[index]!;
    for (let grown = (m | (m << 1) | (m >> 1)) & free; grown !== m; grown = (m | (m << 1) | (m >> 1)) & free) m = grown;
    reached[index] = m;
    const b = blockOf[at]!;
    const j = row & 7, k = row >> 3;
    if ((m & 1) !== 0) cross(b - 1, row, LAST);
    if ((m & LAST) !== 0) cross(b + 1, row, 1);
    if (j > 0) seed(at, row - 1, m);
    else cross(b - nx, row + 7, m);
    if (j < 7) seed(at, row + 1, m);
    else cross(b + nx, row - 7, m);
    if (k > 0) seed(at, row - 8, m);
    else cross(b - layer, row + 56, m);
    if (k < 7) seed(at, row + 8, m);
    else cross(b + layer, row - 56, m);
  }
  const state = (i: number, j: number, k: number): number => {
    if (i < 0 || j < 0 || k < 0 || i >= nx * FLOOD_BLOCK || j >= ny * FLOOD_BLOCK || k >= nz * FLOOD_BLOCK) return REACHED;
    const at = slot[((k >> 3) * ny + (j >> 3)) * nx + (i >> 3)]!;
    if (at < 0) return at === -2 ? REACHED : FREE;
    const index = (at % CHUNK) * ROWS + (((k & 7) << 3) | (j & 7));
    const bit = 1 << (i & 7);
    return (wallRows[(at / CHUNK) | 0]![index]! & bit) !== 0 ? WALL : (reachedRows[(at / CHUNK) | 0]![index]! & bit) !== 0 ? REACHED : FREE;
  };
  const scan = (px: number, py: number, pz: number, qx: number, qy: number, qz: number, visit: (wall: number, along: number) => boolean): void => {
    const dx = qx - px, dy = qy - py, dz = qz - pz;
    const dd = dx * dx + dy * dy + dz * dz;
    const bx0 = Math.max(0, Math.floor((Math.min(px, qx) - origin[0]!) / s - EDGE)), bx1 = Math.min(nx - 1, Math.floor((Math.max(px, qx) - origin[0]!) / s + EDGE));
    const by0 = Math.max(0, Math.floor((Math.min(py, qy) - origin[1]!) / s - EDGE)), by1 = Math.min(ny - 1, Math.floor((Math.max(py, qy) - origin[1]!) / s + EDGE));
    const bz0 = Math.max(0, Math.floor((Math.min(pz, qz) - origin[2]!) / s - EDGE)), bz1 = Math.min(nz - 1, Math.floor((Math.max(pz, qz) - origin[2]!) / s + EDGE));
    for (let bz = bz0; bz <= bz1; bz++)
      for (let by = by0; by <= by1; by++)
        for (let bx = bx0; bx <= bx1; bx++) {
          const at = slot[(bz * ny + by) * nx + bx]!;
          if (at < 0) continue;
          for (let i = start[at]!; i < start[at + 1]!; i++) {
            const triangle = listed[i]!;
            const w = owner[triangle]!;
            const wall = walls[w]!;
            const q = wall.positions;
            const t = (triangle - base[w]!) * 3;
            const ia = wall.indices[t]! * 3, ib = wall.indices[t + 1]! * 3, ic = wall.indices[t + 2]! * 3;
            const ax = q[ia]!, ay = q[ia + 1]!, az = q[ia + 2]!;
            const e1x = q[ib]! - ax, e1y = q[ib + 1]! - ay, e1z = q[ib + 2]! - az, e2x = q[ic]! - ax, e2y = q[ic + 1]! - ay, e2z = q[ic + 2]! - az;
            const hx = dy * e2z - dz * e2y, hy = dz * e2x - dx * e2z, hz = dx * e2y - dy * e2x;
            const det = e1x * hx + e1y * hy + e1z * hz;
            if (det * det < 1e-24 * (e1x * e1x + e1y * e1y + e1z * e1z) * (e2x * e2x + e2y * e2y + e2z * e2z) * dd) continue;
            const sx = px - ax, sy = py - ay, sz = pz - az;
            const u = (sx * hx + sy * hy + sz * hz) / det;
            if (u < -EDGE || u > 1 + EDGE) continue;
            const rx = sy * e1z - sz * e1y, ry = sz * e1x - sx * e1z, rz = sx * e1y - sy * e1x;
            const v = (dx * rx + dy * ry + dz * rz) / det;
            if (v < -EDGE || u + v > 1 + EDGE) continue;
            const along = (e2x * rx + e2y * ry + e2z * rz) / det;
            if (along < 0 || along > 1) continue;
            if (visit(w, along)) return;
          }
        }
  };
  const nearest = (p: Vec, q: Vec): number => {
    let best = Infinity;
    let found = -1;
    scan(p[0], p[1], p[2], q[0], q[1], q[2], (w, along) => {
      if (along < best) [best, found] = [along, w];
      return false;
    });
    return found;
  };
  return {
    cellM: h,
    reaches: (p, normal) => {
      const gx = (p[0] - origin[0]!) / h, gy = (p[1] - origin[1]!) / h, gz = (p[2] - origin[2]!) / h;
      const i = Math.floor(gx), j = Math.floor(gy), k = Math.floor(gz);
      const here = state(i, j, k);
      if (here !== WALL) return here === REACHED;
      for (let o = 0; o < OFFSETS.length; o++) {
        const offset = OFFSETS[o]!;
        const cx = i + offset[0] + 0.5, cy = j + offset[1] + 0.5, cz = k + offset[2] + 0.5;
        if ((cx - gx) * normal[0] + (cy - gy) * normal[1] + (cz - gz) * normal[2] <= 0) continue;
        const there = state(i + offset[0], j + offset[1], k + offset[2]);
        if (there === WALL) continue;
        let blocked = false;
        scan(p[0], p[1], p[2], origin[0]! + cx * h, origin[1]! + cy * h, origin[2]! + cz * h, () => (blocked = true));
        if (!blocked) return there === REACHED;
      }
      return false;
    },
    firstHit: nearest,
    crossings: (p, q, keep) => {
      const out: number[] = [];
      scan(p[0], p[1], p[2], q[0], q[1], q[2], (w, along) => {
        if (keep(w)) out.push(along);
        return false;
      });
      return out.sort((u, v) => u - v);
    },
  };
}

/** 🌊️ Floods the space around the meshes from outside with cells of `cellM` metres; the cell grows only while the grid would exceed the budget of {@link FLOOD_LIMITS} (the result names the cell that was used). `tick` is called regularly and may throw to cancel. */
export function floodOutside(walls: readonly FloodMesh[], cellM: number, tick: () => void = () => {}, limits: { readonly fineBlocks: number; readonly coarseBlocks: number } = FLOOD_LIMITS): Flood {
  let h = cellM;
  for (;;) {
    const built = build(walls, h, tick, limits);
    if (typeof built !== "number") return built;
    h *= built;
  }
}
