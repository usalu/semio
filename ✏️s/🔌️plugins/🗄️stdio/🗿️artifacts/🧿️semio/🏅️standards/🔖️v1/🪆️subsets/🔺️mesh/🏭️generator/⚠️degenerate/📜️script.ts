#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🧨 Mesh fixture recipes — the `degenerate` family: sliver triangles, near-coincident vertices and
// features far smaller than the shape they sit on. Every shape manifold-3d itself agrees to build here
// IS a valid manifold — the library refuses genuinely invalid topology outright — so "degenerate" in
// this family means geometrically EXTREME rather than topologically broken: a feature small enough,
// relative to the rest of the shape, that a fixed-size or carelessly-chosen tolerance anywhere
// downstream (a weld grid, a simplification pass, a display LOD) could plausibly destroy it without
// anyone noticing. Where the `scale` family rescales one whole shape, this family holds the WHOLE shape
// at one ordinary size and shrinks a FEATURE inside it instead.
//
// A recipe DESCRIBES a shape; it computes nothing. `../📜️script.ts` builds it, exports it to four
// formats, re-imports and re-measures what it wrote, and records the bundle with its provenance. A
// recipe that manifold-3d or the re-import/weld pipeline refuses is a MEASURED finding, reported by
// `../📜️script.ts`'s per-recipe try/catch rather than quietly dropped.
//
// @see ../📜️script.ts — the generator that runs these

//#endregion 🧲️Header

//#region 🔌️Adapters
import type { Recipe } from "../📜️script.ts";
//#endregion 🔌️Adapters

//#region 🧪️Recipes
/** 🧨 The `degenerate` recipes. */
export const RECIPES: readonly Recipe[] = [
  {
    id: "degenerate-sliver-thin-slab",
    family: "degenerate",
    tolerance: "mesh-degenerate",
    notes: "A 50×50mm plate 0.001mm thick — a 50,000:1 aspect ratio, thin enough that the two large faces sit closer together than the weld grid many fixed-tolerance pipelines would choose, and a real test of whether this generator's own bounding-box-diagonal-relative weld keeps the top and bottom faces distinct rather than collapsing the slab to a single sheet.",
    build: ({ Manifold }) => ({ result: Manifold.cube([50, 50, 0.001], true) }),
  },
  {
    id: "degenerate-needle-cone",
    family: "degenerate",
    tolerance: "mesh-degenerate",
    notes: "A cone with a 0.05mm base radius and a 60mm height — a needle whose tessellated side wall is almost entirely sliver triangles (each far longer than it is wide), the opposite failure mode from a thin slab's flat near-coincident faces.",
    build: ({ Manifold }) => ({ result: Manifold.cylinder(60, 0.05, 0, 24, true) }),
  },
  {
    id: "degenerate-near-coincident-union",
    family: "degenerate",
    tolerance: "mesh-degenerate",
    notes: "Two 20mm boxes overlapping by only 0.0001mm along one axis — as close to merely touching as this generator could place them while still guaranteeing genuine volumetric overlap. Tests whether the boolean and the subsequent weld agree on a single connected solid rather than one of them treating the near-coincident seam as two separate pieces.",
    build: ({ Manifold }) => {
      const a = Manifold.cube([20, 20, 20], true);
      const b = Manifold.cube([20, 20, 20], true).translate([19.9999, 0, 0]);
      return { operands: [{ role: "operand-a", shape: a }, { role: "operand-b", shape: b }], result: a.add(b) };
    },
  },
  {
    id: "degenerate-tiny-bore-below-tolerance",
    family: "degenerate",
    tolerance: "mesh-degenerate",
    notes: "A 100mm cube with a through-bore only 0.0005mm in radius — a feature roughly seven orders of magnitude smaller than the block's own bounding-box diagonal. This is the direct probe for whether a bounding-box-diagonal-relative weld grid (this generator's own tolerance rule) is itself capable of destroying a feature that is legitimately present but far smaller than the WHOLE shape, which a purely scale-relative rule cannot distinguish from noise the way an absolute floor tuned to the feature itself could.",
    build: ({ Manifold }) => ({ result: Manifold.cube([100, 100, 100], true).subtract(Manifold.cylinder(120, 0.0005, 0.0005, 24, true)) }),
  },
  {
    id: "degenerate-high-tessellation-sphere",
    family: "degenerate",
    tolerance: "mesh-degenerate",
    notes: "A 10mm-radius sphere at 128 circular segments — an edge length around 0.5mm on an object with a ~20mm bounding-box diagonal, densely tessellated enough (double `primitives-sphere-64`'s densest rung) to be a real stress test of the weld grid's ability to keep genuinely distinct neighbouring vertices apart rather than merging fine tessellation into itself, without the ASCII-text export ballooning into tens of megabytes the way a further doubling would.",
    build: ({ Manifold }) => ({ result: Manifold.sphere(10, 128) }),
  },
  {
    id: "degenerate-thin-fin",
    family: "degenerate",
    tolerance: "mesh-degenerate",
    notes: "A 50×50×5mm plate with a single fin 0.002mm thick fused onto its top face — a protruding thin feature rather than a shell wall, testing the same near-coincident-face weld risk as `topology-thin-shell-box` but on an ADDITIVE feature instead of a subtractive cavity.",
    build: ({ Manifold }) => {
      const plate = Manifold.cube([50, 50, 5], true);
      const fin = Manifold.cube([40, 0.002, 15], true).translate([0, 0, 10]);
      return { operands: [{ role: "operand-a", shape: plate }], result: plate.add(fin) };
    },
  },
  {
    id: "degenerate-microscopic-cube",
    family: "degenerate",
    tolerance: "mesh-degenerate",
    notes: "A cube 1e-9mm on a side, deliberately sized to sit ON TOP OF this generator's own weld absolute floor rather than safely above or below it. MEASURED, not fixed afterward: the cube's re-imported corners land at ±4.999999858590343e-10 (manifold-3d's Float32 mesh output, not a further precision loss from this generator), and dividing by the 1e-9 grid gives ±0.4999999…, which `Math.round` sends to 0 on BOTH signs — every one of the 8 corners collapses onto the same lattice point, every triangle becomes degenerate and is dropped, and `Manifold.ofMesh` on the empty remainder throws `Not manifold`. This is `../📜️script.ts`'s per-recipe try/catch reporting a genuine finding, deliberately kept rather than resized away: an absolute floor is, definitionally, a size below which THIS PARTICULAR generator's own re-measurement pipeline cannot see a shape at all, and this fixture is the floor made visible rather than assumed.",
    build: ({ Manifold }) => ({ result: Manifold.cube([1e-9, 1e-9, 1e-9], true) }),
  },
  {
    id: "degenerate-hairline-groove",
    family: "degenerate",
    tolerance: "mesh-degenerate",
    notes: "A 50×50×10mm block with a groove only 0.0002mm wide cut across its top face — a subtractive feature narrow enough that its two opposing walls sit closer together than many fixed weld grids would resolve as distinct, the groove counterpart to `degenerate-tiny-bore-below-tolerance`'s round hole.",
    build: ({ Manifold }) => {
      const block = Manifold.cube([50, 50, 10], true);
      const groove = Manifold.cube([60, 0.0002, 4], true).translate([0, 0, 3]);
      return { operands: [{ role: "operand-a", shape: block }], result: block.subtract(groove) };
    },
  },
  {
    id: "degenerate-coplanar-faces-union",
    family: "degenerate",
    tolerance: "mesh-degenerate",
    notes: "Two 20mm cubes placed edge-to-edge so one whole face of each is EXACTLY coplanar with the other's, then unioned — the classic coincident-face boolean edge case, distinct from `degenerate-near-coincident-union`'s epsilon-offset overlap because here the shared face is mathematically identical between the two operands rather than merely close.",
    build: ({ Manifold }) => {
      const a = Manifold.cube([20, 20, 20], true);
      const b = Manifold.cube([20, 20, 20], true).translate([20, 0, 0]);
      return { operands: [{ role: "operand-a", shape: a }, { role: "operand-b", shape: b }], result: a.add(b) };
    },
  },
  {
    id: "degenerate-tiny-boss-on-large-plate",
    family: "degenerate",
    tolerance: "mesh-degenerate",
    notes: "A 300×300×10mm plate carrying one boss 0.01mm in radius — a single shape spanning six orders of magnitude between its overall footprint and its smallest feature, the WITHIN-ONE-SHAPE counterpart to the `scale` family's whole-shape rescaling and to `topology-disconnected-mixed-sizes`'s two-separate-components version of the same size disparity.",
    build: ({ Manifold }) => {
      const plate = Manifold.cube([300, 300, 10], true);
      const boss = Manifold.cylinder(2, 0.01, 0.01, 16).translate([100, 100, 5]);
      return { operands: [{ role: "operand-a", shape: plate }], result: plate.add(boss) };
    },
  },
];
//#endregion 🧪️Recipes
