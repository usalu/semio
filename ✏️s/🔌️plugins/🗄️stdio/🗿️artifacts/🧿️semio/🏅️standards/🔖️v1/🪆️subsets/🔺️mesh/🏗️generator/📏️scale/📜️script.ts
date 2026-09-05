#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬 Mesh fixture recipes — the `scale` family: the SAME shape at 1e-3, 1, 1e3 and 1e6× its reference
// size. This is what catches a fixed-tolerance bug. The BRep pilot's own qualification found a fixed
// absolute tessellation tolerance consume 2.4 GB and twelve minutes on a part scaled to 1e6 units, while
// the underlying exact boolean had finished in under a second — the measuring tool was consumed by the
// boundary it existed to measure. `../📜️script.ts`'s weld grid is scale-relative (`weldGridFor`,
// `max(absoluteFloor, relative × boundingBoxDiagonal)`) specifically so this family passes at every
// scale with an identical relative error rather than blowing up or silently losing precision at either
// end.
//
// Each shape here is built ONCE at its natural reference size and then `Manifold.scale`d — never
// rebuilt with different absolute dimensions — so any measured difference between the four fixtures in
// a group is attributable to scale alone.
//
// A recipe DESCRIBES a shape; it computes nothing. `../📜️script.ts` builds it, exports it to four
// formats, re-imports and re-measures what it wrote, and records the bundle with its provenance.
//
// @see ../📜️script.ts — the generator that runs these

//#endregion 🧲️Header

//#region 🔌️Adapters
import type { Manifold } from "manifold-3d";
import type { Recipe, Toolkit } from "../📜️script.ts";
//#endregion 🔌️Adapters

//#region 🧰️Reference shapes
/** 🧊 A cube with a through-bore and a fused spherical boss — the exact shape the mesh toolchain spike
 * proved the whole export/re-import/weld/re-measure chain on, at its natural 20mm reference size. */
function referenceBoreBoss({ Manifold }: Toolkit): Manifold {
  return Manifold.cube([20, 20, 20], true)
    .subtract(Manifold.cylinder(30, 5, 5, 64, true))
    .add(Manifold.sphere(8, 64).translate([10, 10, 10]));
}

/** 🍩 A genus-1 torus at its natural reference size — the doubly-curved counterpart to the bore-boss
 * reference shape, so this family's scale sweep is not only exercising planar/cylindrical geometry. */
function referenceTorus({ CrossSection }: Toolkit): Manifold {
  return CrossSection.circle(4, 32).translate(14, 0).revolve(32, 360);
}

const SCALES: readonly { suffix: string; factor: number }[] = [
  { suffix: "1e-3", factor: 1e-3 },
  { suffix: "1", factor: 1 },
  { suffix: "1e3", factor: 1e3 },
  { suffix: "1e6", factor: 1e6 },
];
//#endregion 🧰️Reference shapes

//#region 🧪️Recipes
/** 🔬 The `scale` recipes: two reference shapes, each at four scales spanning nine orders of magnitude. */
export const RECIPES: readonly Recipe[] = [
  ...SCALES.map(({ suffix, factor }): Recipe => ({
    id: `scale-bore-boss-${suffix}`,
    family: "scale",
    tolerance: "mesh-scale-relative",
    notes: `The bore-boss reference shape at ${factor}× its natural 20mm size (built once, then \`Manifold.scale(${factor})\`d — never rebuilt with different absolute dimensions). Volume and surface area must scale by ${factor}³ and ${factor}² respectively relative to \`scale-bore-boss-1\`; the weld grid this generator applies before re-measuring is bounding-box-diagonal-relative specifically so it neither over-merges this fixture at 1e-3 nor runs away in cost or memory at 1e6, the exact failure mode a fixed absolute tolerance produced in the sibling BRep pilot.`,
    build: (t) => ({ result: referenceBoreBoss(t).scale(factor) }),
  })),
  ...SCALES.map(({ suffix, factor }): Recipe => ({
    id: `scale-torus-${suffix}`,
    family: "scale",
    tolerance: "mesh-scale-relative",
    notes: `The reference torus at ${factor}× its natural size (major radius 14mm) — the doubly-curved half of this family's scale sweep, exercising the same scale-relative weld tolerance on a genus-1 shape rather than the bore-boss's genus-1-by-subtraction construction.`,
    build: (t) => ({ result: referenceTorus(t).scale(factor) }),
  })),
];
//#endregion 🧪️Recipes
