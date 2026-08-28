#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🧊 Mesh fixture recipes — the `primitives` family: cube, sphere, cylinder, cone and torus at several
// tessellation levels. Every recipe here is deliberately exact-volume-checkable in closed form, which is
// what makes this family the corpus's baseline — a fixture whose own answer can be hand-verified before
// any of the other families lean on the same weld/measure pipeline.
//
// A recipe DESCRIBES a shape; it computes nothing. `../📜️script.ts` builds it, exports it to four
// formats, re-imports and re-measures what it wrote, and records the bundle with its provenance.
//
// @see ../📜️script.ts — the generator that runs these

//#endregion 🧲️Header

//#region 🔌️Adapters
import type { Recipe, Toolkit } from "../📜️script.ts";
//#endregion 🔌️Adapters

//#region 🧪️Recipes
/** 🧊 The `primitives` recipes: cube, sphere, cylinder, cone, torus — the last two built from
 * `Manifold.cylinder`'s two-radius form and `CrossSection.circle().revolve()` respectively, since
 * manifold-3d has no dedicated cone/torus constructor. */
export const RECIPES: readonly Recipe[] = [
  {
    id: "primitives-cube-unit",
    family: "primitives",
    tolerance: "mesh-exact",
    notes: "A unit cube, centered at the origin — the simplest possible closed-form check: volume 1000, 6 quad faces triangulated as 12 triangles, genus 0.",
    build: ({ Manifold }) => ({ result: Manifold.cube([10, 10, 10], true) }),
  },
  {
    id: "primitives-cube-rectangular",
    family: "primitives",
    tolerance: "mesh-exact",
    notes: "A non-cubic box with three distinct edge lengths — catches an axis-swap bug a perfect cube can hide.",
    build: ({ Manifold }) => ({ result: Manifold.cube([20, 35, 8], true) }),
  },
  {
    id: "primitives-sphere-8",
    family: "primitives",
    tolerance: "mesh-tessellated",
    notes: "A coarse geodesic sphere (8 circular segments, rounded up to the nearest factor of four by manifold-3d) — the low end of the tessellation ladder this family sweeps.",
    build: ({ Manifold }) => ({ result: Manifold.sphere(10, 8) }),
  },
  {
    id: "primitives-sphere-16",
    family: "primitives",
    tolerance: "mesh-tessellated",
    notes: "Same sphere as `primitives-sphere-8`, doubled tessellation — the pair exists to make the volume converge toward 4/3πr³ measurably as segment count rises.",
    build: ({ Manifold }) => ({ result: Manifold.sphere(10, 16) }),
  },
  {
    id: "primitives-sphere-32",
    family: "primitives",
    tolerance: "mesh-tessellated",
    notes: "Same sphere, 32 segments — a resolution dense enough that most downstream consumers would treat it as visually smooth.",
    build: ({ Manifold }) => ({ result: Manifold.sphere(10, 32) }),
  },
  {
    id: "primitives-sphere-64",
    family: "primitives",
    tolerance: "mesh-tessellated",
    notes: "Same sphere, 64 segments — the high end of the ladder, and the densest primitive mesh in this family before `degenerate` takes tessellation density to the point of stressing the weld itself.",
    build: ({ Manifold }) => ({ result: Manifold.sphere(10, 64) }),
  },
  {
    id: "primitives-cylinder-8",
    family: "primitives",
    tolerance: "mesh-tessellated",
    notes: "An octagonal-approximation cylinder (8 circular segments) — coarse enough that its side faces are visibly flat facets rather than a smooth barrel.",
    build: ({ Manifold }) => ({ result: Manifold.cylinder(30, 10, 10, 8, true) }),
  },
  {
    id: "primitives-cylinder-16",
    family: "primitives",
    tolerance: "mesh-tessellated",
    notes: "Same cylinder, 16 segments.",
    build: ({ Manifold }) => ({ result: Manifold.cylinder(30, 10, 10, 16, true) }),
  },
  {
    id: "primitives-cylinder-32",
    family: "primitives",
    tolerance: "mesh-tessellated",
    notes: "Same cylinder, 32 segments — a resolution dense enough to read as round.",
    build: ({ Manifold }) => ({ result: Manifold.cylinder(30, 10, 10, 32, true) }),
  },
  {
    id: "primitives-cylinder-64",
    family: "primitives",
    tolerance: "mesh-tessellated",
    notes: "Same cylinder, 64 segments — the high end of this family's cylinder ladder.",
    build: ({ Manifold }) => ({ result: Manifold.cylinder(30, 10, 10, 64, true) }),
  },
  {
    id: "primitives-cone-8",
    family: "primitives",
    tolerance: "mesh-tessellated",
    notes: "`Manifold.cylinder`'s two-radius form with the top radius at zero gives an exact cone — 8 segments, the coarse end of the cone ladder.",
    build: ({ Manifold }) => ({ result: Manifold.cylinder(30, 12, 0, 8, true) }),
  },
  {
    id: "primitives-cone-16",
    family: "primitives",
    tolerance: "mesh-tessellated",
    notes: "Same cone, 16 segments.",
    build: ({ Manifold }) => ({ result: Manifold.cylinder(30, 12, 0, 16, true) }),
  },
  {
    id: "primitives-cone-32",
    family: "primitives",
    tolerance: "mesh-tessellated",
    notes: "Same cone, 32 segments — the smooth end of the cone ladder. The apex is a single degenerate ring of coincident vertices before welding, which is exactly the case `weldToManifold` has to collapse back down correctly.",
    build: ({ Manifold }) => ({ result: Manifold.cylinder(30, 12, 0, 32, true) }),
  },
  {
    id: "primitives-torus-16",
    family: "primitives",
    tolerance: "mesh-tessellated",
    notes: "A genus-1 torus — `CrossSection.circle(tubeRadius).translate(majorRadius, 0).revolve(segments, 360)`, manifold-3d's real revolve-of-a-2D-profile constructor rather than a hand-built ring of quads. 16 segments in both the tube circle and the revolve sweep.",
    build: ({ CrossSection }) => ({ result: CrossSection.circle(4, 16).translate(14, 0).revolve(16, 360) }),
  },
  {
    id: "primitives-torus-32",
    family: "primitives",
    tolerance: "mesh-tessellated",
    notes: "Same torus, 32 segments.",
    build: ({ CrossSection }) => ({ result: CrossSection.circle(4, 32).translate(14, 0).revolve(32, 360) }),
  },
  {
    id: "primitives-torus-64",
    family: "primitives",
    tolerance: "mesh-tessellated",
    notes: "Same torus, 64 segments — the densest genus-1 primitive in this family, and a direct cross-check for the `topology` family's genus-1 fixtures built by drilling a bore instead of revolving a profile.",
    build: ({ CrossSection }) => ({ result: CrossSection.circle(4, 64).translate(14, 0).revolve(64, 360) }),
  },
];
//#endregion 🧪️Recipes
