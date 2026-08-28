#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🕳️ Mesh fixture recipes — the `topology` family: shapes spanning genus 0..3, disconnected components,
// thin shells and high-aspect-ratio features. Where `booleans` picks its operations for the shape they
// produce, this family picks its shapes for the TOPOLOGICAL INVARIANT or the numerically stressful
// proportion they exercise, independent of which boolean built it.
//
// A recipe DESCRIBES a shape; it computes nothing. `../📜️script.ts` builds it, exports it to four
// formats, re-imports and re-measures what it wrote, and records the bundle with its provenance.
//
// @see ../📜️script.ts — the generator that runs these

//#endregion 🧲️Header

//#region 🔌️Adapters
import type { Recipe } from "../📜️script.ts";
//#endregion 🔌️Adapters

//#region 🧪️Recipes
/** 🕳️ The `topology` recipes. */
export const RECIPES: readonly Recipe[] = [
  {
    id: "topology-genus0-sphere",
    family: "topology",
    tolerance: "mesh-tessellated",
    notes: "A plain sphere — genus 0, the baseline every other genus fixture in this family is measured against.",
    build: ({ Manifold }) => ({ result: Manifold.sphere(12, 32) }),
  },
  {
    id: "topology-genus1-single-bore",
    family: "topology",
    tolerance: "mesh-tessellated",
    notes: "A box with one through-bore — genus 1. The `topology`-family twin of `boolean-difference-cube-single-bore`, built to be picked out by genus rather than by which operation produced it.",
    build: ({ Manifold }) => ({ result: Manifold.cube([30, 16, 16], true).subtract(Manifold.cylinder(20, 4, 4, 32, true).rotate([0, 90, 0])) }),
  },
  {
    id: "topology-genus2-two-bores",
    family: "topology",
    tolerance: "mesh-tessellated",
    notes: "A box with two parallel, non-intersecting through-bores — genus 2.",
    build: ({ Manifold }) => {
      const bores = [-10, 10].map((x) => Manifold.cylinder(20, 4, 4, 32, true).rotate([0, 90, 0]).translate([x, 0, 0]));
      let result = Manifold.cube([50, 16, 16], true);
      for (const bore of bores) result = result.subtract(bore);
      return { result };
    },
  },
  {
    id: "topology-genus3-three-bores",
    family: "topology",
    tolerance: "mesh-tessellated",
    notes: "A box with three parallel, non-intersecting through-bores — genus 3, the top of this family's genus ladder and the direct cross-check for `boolean-difference-cube-three-bores`.",
    build: ({ Manifold }) => {
      const bores = [-16, 0, 16].map((x) => Manifold.cylinder(20, 4, 4, 32, true).rotate([0, 90, 0]).translate([x, 0, 0]));
      let result = Manifold.cube([50, 16, 16], true);
      for (const bore of bores) result = result.subtract(bore);
      return { result };
    },
  },
  {
    id: "topology-disconnected-two-components",
    family: "topology",
    tolerance: "mesh-tessellated",
    notes: "Two cubes placed far apart, unioned — `decompose()` recovers 2 components, each genus 0. The `topology`-family counterpart to the disjoint-spheres boolean fixture, built from planar rather than curved primitives.",
    build: ({ Manifold }) => ({ result: Manifold.cube([10, 10, 10], true).add(Manifold.cube([10, 10, 10], true).translate([60, 0, 0])) }),
  },
  {
    id: "topology-disconnected-three-components",
    family: "topology",
    tolerance: "mesh-tessellated",
    notes: "Three spheres placed far apart, unioned — `decompose()` recovers 3 components.",
    build: ({ Manifold }) => {
      const a = Manifold.sphere(6, 24);
      const b = Manifold.sphere(6, 24).translate([30, 0, 0]);
      const c = Manifold.sphere(6, 24).translate([0, 30, 0]);
      return { result: a.add(b).add(c) };
    },
  },
  {
    id: "topology-disconnected-mixed-sizes",
    family: "topology",
    tolerance: "mesh-tessellated",
    notes: "One large cube and one small cube — two orders of magnitude apart in edge length — placed far apart and unioned into a single 2-component result. Exercises whether a consumer's bounding-box-diagonal-relative tolerance stays correct when the two components it spans are wildly different sizes, distinct from the `scale` family's whole-shape rescaling.",
    build: ({ Manifold }) => ({ result: Manifold.cube([50, 50, 50], true).add(Manifold.cube([0.5, 0.5, 0.5], true).translate([200, 0, 0])) }),
  },
  {
    id: "topology-thin-shell-box",
    family: "topology",
    tolerance: "mesh-tessellated",
    notes: "A box shelled to a wall 1/30th of its own edge length (30mm outer, 1mm wall) by subtracting a slightly smaller box — thin enough that a weld grid sized wrong relative to the shape would merge the outer and inner surfaces into one degenerate sheet instead of keeping them the two disjoint boundary components they are.",
    build: ({ Manifold }) => ({ result: Manifold.cube([30, 30, 30], true).subtract(Manifold.cube([28, 28, 28], true)) }),
  },
  {
    id: "topology-thin-shell-sphere",
    family: "topology",
    tolerance: "mesh-tessellated",
    notes: "The same thin-wall-shell stress as `topology-thin-shell-box`, on doubly-curved rather than planar faces — a sphere shelled to a wall 1/20th of its own radius.",
    build: ({ Manifold }) => ({ result: Manifold.sphere(20, 48).subtract(Manifold.sphere(19, 48)) }),
  },
  {
    id: "topology-high-aspect-ratio-rod",
    family: "topology",
    tolerance: "mesh-tessellated",
    notes: "A cylindrical rod 100 times longer than it is wide (radius 1, length 100) — the high-aspect-ratio counterpart of the thin-shell fixtures: instead of one dimension being thin relative to the whole, one dimension is LONG relative to the other two, which stresses a bounding-box-diagonal-relative tolerance from the opposite direction.",
    build: ({ Manifold }) => ({ result: Manifold.cylinder(100, 1, 1, 24, true) }),
  },
  {
    id: "topology-high-aspect-ratio-plate",
    family: "topology",
    tolerance: "mesh-tessellated",
    notes: "A plate 400 times wider than it is thick (200×200mm, 0.5mm thick) — the planar high-aspect-ratio case, and close to the thinnest a single-digit-millimetre real part's web or fin ever gets.",
    build: ({ Manifold }) => ({ result: Manifold.cube([200, 200, 0.5], true) }),
  },
  {
    id: "topology-thin-wall-partition",
    family: "topology",
    tolerance: "mesh-tessellated",
    notes: "A thin-walled box shell (`topology-thin-shell-box`'s construction) with an internal partition wall fused across its hollow interior, splitting the one cavity into two chambers. The partition is sized to overlap the shell's own solid material at both ends rather than merely touch it face-to-face, so the fuse is a genuine volumetric union rather than a coincident-face join — MEASURED: the resulting SOLID is one contiguous piece of material (volume 6224mm³, 42mm³ less than the un-merged 5048+1218 sum, exactly the overlap the union correctly deduplicated), but its BOUNDARY still decomposes into 3 disjoint closed surfaces (the outer shell face plus one per now-separated chamber, each genus 0) — the same one-solid/many-boundary-components split `boolean-difference-nested-cavity-sphere` documents, here with the added twist that the partition is what CREATES the second inner boundary rather than a single pre-existing cavity being merely enclosed. This is the thin-partition-inside-a-cavity case the mechanical STEP pilot found interesting for exactly the reason it is here: a lost internal wall leaves the outer surface, and therefore a surface-only check, completely untouched.",
    build: ({ Manifold }) => {
      const shell = Manifold.cube([30, 30, 30], true).subtract(Manifold.cube([28, 28, 28], true));
      const partition = Manifold.cube([29, 1.5, 28], true);
      return { result: shell.add(partition) };
    },
  },
];
//#endregion 🧪️Recipes
