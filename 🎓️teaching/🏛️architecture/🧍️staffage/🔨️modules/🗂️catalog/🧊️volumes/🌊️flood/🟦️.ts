/** 🌊️ The air around a set of meshes (C31): every triangle is a wall, the space between the walls is flooded from outside the bounding box, and a point is reached when the flood arrives at it without crossing a triangle.
 *
 * The space is a sparse grid of two levels: blocks of {@link FLOOD_BLOCK}³ cells; a block no triangle touches is one node of the flood, a block with triangles keeps its cells. A cell that a triangle touches (separating-axis test of the triangle against the closed cell) is a wall cell, so a gap of at most one cell is always closed and a gap of two cells is always open. A point in a wall cell is reached when it sees the centre of a reached cell at most {@link FLOOD_LIMITS}.sightCells away along a straight segment that no triangle crosses (exact, so a wall thinner than a cell still separates).
 *
 * THE REFERENCE is the connected-component labelling of scipy on a trimesh voxelisation in `🧪️tests/🧊️volumes-flood/🐍️.py`.
 *
 * @see ../🧮️measures/🟦️.ts — `outsideOf`, which decides with it what counts as outer surface
 * @see https://fileadmin.cs.lth.se/cs/Personal/Tomas_Akenine-Moller/code/tribox_tam.pdf — the triangle-box overlap test */

type Vec = readonly [number, number, number];

/** 🧱️ A mesh of the flood: three coordinates per vertex, three vertex indices per triangle. */
export type FloodMesh = { readonly positions: Float64Array; readonly indices: Uint32Array };

/** 🧊️ How many cells a block spans along each axis. */
export const FLOOD_BLOCK = 8;

/** 🎚️ The budget of the flood: at most `fineBlocks` blocks with cells and `coarseBlocks` blocks in all (the cell grows until both hold), and how many cells away a point in a wall cell looks for a free cell. */
export const FLOOD_LIMITS = { fineBlocks: 200_000, coarseBlocks: 8_000_000, sightCells: 2 } as const;

/** 🌊️ The flooded space: `cellM` is the cell the budget allowed, `reaches` says whether the air from outside arrives at the point `p` in front of a face with the outward unit normal `n`, `firstHit` is the index of the mesh whose triangle the segment from `p` to `q` crosses first (-1 when none). */
export type Flood = { readonly cellM: number; readonly reaches: (p: Vec, n: Vec) => boolean; readonly firstHit: (p: Vec, q: Vec) => number };

const CELLS = FLOOD_BLOCK * FLOOD_BLOCK * FLOOD_BLOCK;
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

function build(walls: readonly FloodMesh[], h: number, tick: () => void, limits: { readonly fineBlocks: number; readonly coarseBlocks: number }): Flood | number {
  const min = [Infinity, Infinity, Infinity];
  const max = [-Infinity, -Infinity, -Infinity];
  const base: number[] = [];
  let triangles = 0;
  for (const wall of walls) {
    base.push(triangles);
    triangles += wall.indices.length / 3;
    for (let i = 0; i < wall.indices.length; i++) for (let k = 0; k < 3; k++) {
      const v = wall.positions[wall.indices[i]! * 3 + k]!;
      if (v < min[k]!) min[k] = v;
      if (v > max[k]!) max[k] = v;
    }
  }
  if (triangles === 0) return { cellM: h, reaches: () => true, firstHit: () => -1 };
  const s = FLOOD_BLOCK * h;
  const origin = [0, 1, 2].map((k) => min[k]! - s - JITTER[k]! * h);
  const [nx, ny, nz] = [0, 1, 2].map((k) => Math.floor((max[k]! - origin[k]!) / s) + 2) as [number, number, number];
  if (nx * ny * nz > limits.coarseBlocks) return Math.cbrt((nx * ny * nz) / limits.coarseBlocks) * 1.05;
  const slot = new Int32Array(nx * ny * nz).fill(-1);
  const chunks: Uint8Array[] = [];
  const blockOf: number[] = [];
  const lastTriangle: number[] = [];
  const pairs = new Stack();
  const owner = new Int32Array(triangles);
  let count = 0;
  let over = false;
  let g = 0;
  const [a, e, n] = [new Float64Array(9), new Float64Array(9), new Float64Array(3)];
  const overlaps = (cx: number, cy: number, cz: number, hx: number, hy: number, hz: number): boolean => {
    const [x0, y0, z0, x1, y1, z1, x2, y2, z2] = [a[0]! - cx, a[1]! - cy, a[2]! - cz, a[3]! - cx, a[4]! - cy, a[5]! - cz, a[6]! - cx, a[7]! - cy, a[8]! - cz];
    if (Math.min(x0, x1, x2) > hx || Math.max(x0, x1, x2) < -hx || Math.min(y0, y1, y2) > hy || Math.max(y0, y1, y2) < -hy || Math.min(z0, z1, z2) > hz || Math.max(z0, z1, z2) < -hz) return false;
    if (Math.abs(n[0]! * x0 + n[1]! * y0 + n[2]! * z0) > hx * Math.abs(n[0]!) + hy * Math.abs(n[1]!) + hz * Math.abs(n[2]!)) return false;
    for (let k = 0; k < 9; k += 3) {
      const [ex, ey, ez] = [e[k]!, e[k + 1]!, e[k + 2]!];
      let [p0, p1, p2] = [ey * z0 - ez * y0, ey * z1 - ez * y1, ey * z2 - ez * y2];
      let r = hy * Math.abs(ez) + hz * Math.abs(ey);
      if (Math.min(p0, p1, p2) > r || Math.max(p0, p1, p2) < -r) return false;
      [p0, p1, p2] = [ez * x0 - ex * z0, ez * x1 - ex * z1, ez * x2 - ex * z2];
      r = hx * Math.abs(ez) + hz * Math.abs(ex);
      if (Math.min(p0, p1, p2) > r || Math.max(p0, p1, p2) < -r) return false;
      [p0, p1, p2] = [ex * y0 - ey * x0, ex * y1 - ey * x1, ex * y2 - ey * x2];
      r = hx * Math.abs(ey) + hy * Math.abs(ex);
      if (Math.min(p0, p1, p2) > r || Math.max(p0, p1, p2) < -r) return false;
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
      if (at % CHUNK === 0) chunks.push(new Uint8Array(CHUNK * CELLS));
    }
    chunks[(at / CHUNK) | 0]![(at % CHUNK) * CELLS + (((k & 7) << 6) | ((j & 7) << 3) | (i & 7))] = WALL;
    if (lastTriangle[at] !== g) {
      lastTriangle[at] = g;
      pairs.push(at);
      pairs.push(g);
    }
  };
  const raster = (i0: number, i1: number, j0: number, j1: number, k0: number, k1: number): void => {
    if (over || !overlaps((i0 + i1 + 1) / 2, (j0 + j1 + 1) / 2, (k0 + k1 + 1) / 2, (i1 - i0 + 1) / 2 + EDGE, (j1 - j0 + 1) / 2 + EDGE, (k1 - k0 + 1) / 2 + EDGE)) return;
    const [di, dj, dk] = [i1 - i0, j1 - j0, k1 - k0];
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
  if (over) return Math.min(4, Math.max(1.25, Math.sqrt(triangles / Math.max(1, g)) * 1.1));
  const start = new Int32Array(count + 1);
  for (let i = 0; i < pairs.top; i += 2) start[pairs.data[i]! + 1]!++;
  for (let i = 0; i < count; i++) start[i + 1]! += start[i]!;
  const listed = new Int32Array(pairs.top / 2);
  const fill = start.slice(0, count);
  for (let i = 0; i < pairs.top; i += 2) listed[fill[pairs.data[i]!]!++] = pairs.data[i + 1]!;
  const cellAt = (at: number, local: number): number => chunks[(at / CHUNK) | 0]![(at % CHUNK) * CELLS + local]!;
  const reachCell = (at: number, local: number, stack: Stack): void => {
    const chunk = chunks[(at / CHUNK) | 0]!;
    const index = (at % CHUNK) * CELLS + local;
    if (chunk[index] === FREE) {
      chunk[index] = REACHED;
      stack.push(at * CELLS + local);
    }
  };
  const stack = new Stack();
  const strides = [1, nx, nx * ny];
  const dims = [nx, ny, nz];
  slot[0] = -2;
  stack.push(-1);
  let popped = 0;
  while (stack.top > 0) {
    if ((++popped & 0xffff) === 0) tick();
    const v = stack.data[--stack.top]!;
    if (v < 0) {
      const b = -v - 1;
      const at = [b % nx, ((b / nx) | 0) % ny, (b / (nx * ny)) | 0];
      for (let axis = 0; axis < 3; axis++)
        for (const side of [-1, 1]) {
          if (at[axis]! + side < 0 || at[axis]! + side >= dims[axis]!) continue;
          const nb = b + side * strides[axis]!;
          const sn = slot[nb]!;
          if (sn === -1) {
            slot[nb] = -2;
            stack.push(-(nb + 1));
          } else if (sn >= 0) {
            const fixed = side > 0 ? 0 : FLOOD_BLOCK - 1;
            for (let p = 0; p < FLOOD_BLOCK; p++)
              for (let r = 0; r < FLOOD_BLOCK; r++) reachCell(sn, axis === 0 ? (p << 6) | (r << 3) | fixed : axis === 1 ? (p << 6) | (fixed << 3) | r : (fixed << 6) | (p << 3) | r, stack);
          }
        }
    } else {
      const [at, local] = [(v / CELLS) | 0, v % CELLS];
      const b = blockOf[at]!;
      for (let axis = 0; axis < 3; axis++) {
        const shift = axis * 3;
        const c = (local >> shift) & 7;
        for (const side of [-1, 1]) {
          const to = c + side;
          if (to >= 0 && to < FLOOD_BLOCK) {
            reachCell(at, local + (side << shift), stack);
            continue;
          }
          const nb = b + side * strides[axis]!;
          const sn = slot[nb]!;
          if (sn === -1) {
            slot[nb] = -2;
            stack.push(-(nb + 1));
          } else if (sn >= 0) reachCell(sn, (local & ~(7 << shift)) | ((side > 0 ? 0 : FLOOD_BLOCK - 1) << shift), stack);
        }
      }
    }
  }
  const state = (i: number, j: number, k: number): number => {
    if (i < 0 || j < 0 || k < 0 || i >= nx * FLOOD_BLOCK || j >= ny * FLOOD_BLOCK || k >= nz * FLOOD_BLOCK) return REACHED;
    const at = slot[((k >> 3) * ny + (j >> 3)) * nx + (i >> 3)]!;
    return at === -2 ? REACHED : at === -1 ? FREE : cellAt(at, ((k & 7) << 6) | ((j & 7) << 3) | (i & 7));
  };
  const crossing = (px: number, py: number, pz: number, qx: number, qy: number, qz: number, nearest: boolean): number => {
    const [dx, dy, dz] = [qx - px, qy - py, qz - pz];
    const lo = [Math.min(px, qx), Math.min(py, qy), Math.min(pz, qz)].map((v, k) => Math.max(0, Math.floor((v - origin[k]!) / s - EDGE)));
    const hi = [Math.max(px, qx), Math.max(py, qy), Math.max(pz, qz)].map((v, k) => Math.min(dims[k]! - 1, Math.floor((v - origin[k]!) / s + EDGE)));
    let [best, found] = [Infinity, -1];
    for (let bz = lo[2]!; bz <= hi[2]!; bz++)
      for (let by = lo[1]!; by <= hi[1]!; by++)
        for (let bx = lo[0]!; bx <= hi[0]!; bx++) {
          const at = slot[(bz * ny + by) * nx + bx]!;
          if (at < 0) continue;
          for (let i = start[at]!; i < start[at + 1]!; i++) {
            const triangle = listed[i]!;
            const w = owner[triangle]!;
            const { positions: q, indices } = walls[w]!;
            const t = (triangle - base[w]!) * 3;
            const [ia, ib, ic] = [indices[t]! * 3, indices[t + 1]! * 3, indices[t + 2]! * 3];
            const [ax, ay, az] = [q[ia]!, q[ia + 1]!, q[ia + 2]!];
            const [e1x, e1y, e1z, e2x, e2y, e2z] = [q[ib]! - ax, q[ib + 1]! - ay, q[ib + 2]! - az, q[ic]! - ax, q[ic + 1]! - ay, q[ic + 2]! - az];
            const [hx, hy, hz] = [dy * e2z - dz * e2y, dz * e2x - dx * e2z, dx * e2y - dy * e2x];
            const det = e1x * hx + e1y * hy + e1z * hz;
            if (det * det < 1e-24 * (e1x * e1x + e1y * e1y + e1z * e1z) * (e2x * e2x + e2y * e2y + e2z * e2z) * (dx * dx + dy * dy + dz * dz)) continue;
            const [sx, sy, sz] = [px - ax, py - ay, pz - az];
            const u = (sx * hx + sy * hy + sz * hz) / det;
            if (u < -EDGE || u > 1 + EDGE) continue;
            const [rx, ry, rz] = [sy * e1z - sz * e1y, sz * e1x - sx * e1z, sx * e1y - sy * e1x];
            const v = (dx * rx + dy * ry + dz * rz) / det;
            if (v < -EDGE || u + v > 1 + EDGE) continue;
            const along = (e2x * rx + e2y * ry + e2z * rz) / det;
            if (along < 0 || along > 1) continue;
            if (!nearest) return w;
            if (along < best) [best, found] = [along, w];
          }
        }
    return found;
  };
  return {
    cellM: h,
    reaches: (p, normal) => {
      const [gx, gy, gz] = [(p[0] - origin[0]!) / h, (p[1] - origin[1]!) / h, (p[2] - origin[2]!) / h];
      const [i, j, k] = [Math.floor(gx), Math.floor(gy), Math.floor(gz)];
      const here = state(i, j, k);
      if (here !== WALL) return here === REACHED;
      for (const [di, dj, dk] of OFFSETS) {
        const there = state(i + di, j + dj, k + dk);
        if (there === WALL) continue;
        const [cx, cy, cz] = [i + di + 0.5, j + dj + 0.5, k + dk + 0.5];
        if ((cx - gx) * normal[0] + (cy - gy) * normal[1] + (cz - gz) * normal[2] <= 0) continue;
        if (crossing(p[0], p[1], p[2], origin[0]! + cx * h, origin[1]! + cy * h, origin[2]! + cz * h, false) < 0) return there === REACHED;
      }
      return false;
    },
    firstHit: (p, q) => crossing(p[0], p[1], p[2], q[0], q[1], q[2], true),
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
