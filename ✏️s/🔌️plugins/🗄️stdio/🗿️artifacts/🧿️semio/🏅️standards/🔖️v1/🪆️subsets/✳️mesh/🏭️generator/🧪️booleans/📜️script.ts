#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// ⚗️ Mesh fixture recipes — the `booleans` family: union/difference/intersection producing bores,
// grooves, disjoint results, tangent contacts and nested voids. This is where interesting genus and
// solid-count values come from — a single sphere is always genus 0 and always one solid; a boolean is
// what makes those numbers move.
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
/** ⚗️ The `booleans` recipes. */
export const RECIPES: readonly Recipe[] = [
  {
    id: "boolean-union-overlapping-spheres",
    family: "booleans",
    tolerance: "mesh-tessellated",
    notes: "Two spheres overlapping by half a radius, fused into one connected blob — the simplest non-trivial union: one solid, genus 0.",
    build: ({ Manifold }) => {
      const a = Manifold.sphere(10, 32);
      const b = Manifold.sphere(10, 32).translate([12, 0, 0]);
      return { operands: [{ role: "operand-a", shape: a }, { role: "operand-b", shape: b }], result: a.add(b) };
    },
  },
  {
    id: "boolean-union-disjoint-spheres",
    family: "booleans",
    tolerance: "mesh-tessellated",
    notes: "Two spheres far enough apart that they never touch. `add()` on non-overlapping input is still a valid manifold — MEASURED as `decompose().length === 2` — so this is the corpus's plainest disjoint-result fixture, distinct from a fused blob only in solid count.",
    build: ({ Manifold }) => {
      const a = Manifold.sphere(8, 32);
      const b = Manifold.sphere(8, 32).translate([40, 0, 0]);
      return { operands: [{ role: "operand-a", shape: a }, { role: "operand-b", shape: b }], result: a.add(b) };
    },
  },
  {
    id: "boolean-union-tangent-spheres",
    family: "booleans",
    tolerance: "mesh-tessellated",
    notes: "Two spheres placed exactly `radiusA + radiusB` apart — touching at a single point in the ideal continuous geometry. MEASURED at 32-segment tessellation: manifold-3d still reports `decompose().length === 2`, because the tessellated surfaces meet, if at all, at an isolated vertex rather than sharing a coincident face the boolean engine merges. A single-point contact declares itself as still-disjoint rather than silently fusing.",
    build: ({ Manifold }) => {
      const a = Manifold.sphere(10, 32);
      const b = Manifold.sphere(10, 32).translate([20, 0, 0]);
      return { operands: [{ role: "operand-a", shape: a }, { role: "operand-b", shape: b }], result: a.add(b) };
    },
  },
  {
    id: "boolean-difference-cube-single-bore",
    family: "booleans",
    tolerance: "mesh-tessellated",
    notes: "A box with one cylindrical through-bore — the canonical genus-1 difference, and the direct cross-check for `topology-genus1-single-bore` and `primitives-torus-32` (three different constructions of the same topological invariant).",
    build: ({ Manifold }) => {
      const box = Manifold.cube([30, 30, 20], true);
      const bore = Manifold.cylinder(30, 6, 6, 32, true);
      return { operands: [{ role: "operand-a", shape: box }, { role: "operand-b", shape: bore }], result: box.subtract(bore) };
    },
  },
  {
    id: "boolean-difference-cube-blind-bore",
    family: "booleans",
    tolerance: "mesh-tessellated",
    notes: "A box with a BLIND bore — a cylindrical pocket that does not reach the far face. The cavity opens to the outside on one end only, so the topology stays genus 0 despite the removed volume; the direct contrast case for `boolean-difference-cube-single-bore`.",
    build: ({ Manifold }) => {
      const box = Manifold.cube([30, 30, 20], true);
      const pocket = Manifold.cylinder(12, 6, 6, 32).translate([0, 0, 4]);
      return { operands: [{ role: "operand-a", shape: box }, { role: "operand-b", shape: pocket }], result: box.subtract(pocket) };
    },
  },
  {
    id: "boolean-difference-cube-groove",
    family: "booleans",
    tolerance: "mesh-tessellated",
    notes: "A box with an open channel cut straight across its top face. A groove open on two sides is still genus 0 — removing a trench does not add a handle the way a fully enclosed tunnel does.",
    build: ({ Manifold }) => {
      const box = Manifold.cube([40, 20, 12], true);
      const channel = Manifold.cube([50, 5, 5], true).translate([0, 0, 3.5]);
      return { operands: [{ role: "operand-a", shape: box }, { role: "operand-b", shape: channel }], result: box.subtract(channel) };
    },
  },
  {
    id: "boolean-difference-cube-three-bores",
    family: "booleans",
    tolerance: "mesh-tessellated",
    notes: "A box with three parallel, non-intersecting through-bores — MEASURED genus 3 (each independent through-hole adds one handle), and the boolean-family cross-check for `topology-genus3-three-bores`.",
    build: ({ Manifold }) => {
      const box = Manifold.cube([50, 16, 16], true);
      const bores = [-16, 0, 16].map((x) => Manifold.cylinder(20, 4, 4, 32, true).rotate([0, 90, 0]).translate([x, 0, 0]));
      let result = box;
      for (const bore of bores) result = result.subtract(bore);
      return { operands: [{ role: "operand-a", shape: box }], result };
    },
  },
  {
    id: "boolean-difference-nested-cavity-sphere",
    family: "booleans",
    tolerance: "mesh-tessellated",
    notes: "A sphere with a concentric spherical cavity fully inside it — a hollow shell. MEASURED (this generator's own qualification probe): `decompose().length === 2`, because the outer and inner boundary surfaces are two disjoint closed surfaces even though the solid material between them is one connected region — `genus()` on the whole shape returns manifold-3d's own sentinel `-1` for exactly this reason, which is why `../📜️script.ts`'s `topology()` helper decomposes before asking for genus.",
    build: ({ Manifold }) => {
      const outer = Manifold.sphere(15, 32);
      const inner = Manifold.sphere(9, 32);
      return { operands: [{ role: "operand-a", shape: outer }, { role: "operand-b", shape: inner }], result: outer.subtract(inner) };
    },
  },
  {
    id: "boolean-difference-nested-cavity-box",
    family: "booleans",
    tolerance: "mesh-tessellated",
    notes: "A box with a concentric box-shaped cavity fully inside it — the same nested-void topology as `boolean-difference-nested-cavity-sphere`, built from planar rather than doubly-curved faces.",
    build: ({ Manifold }) => {
      const outer = Manifold.cube([30, 30, 30], true);
      const inner = Manifold.cube([20, 20, 20], true);
      return { operands: [{ role: "operand-a", shape: outer }, { role: "operand-b", shape: inner }], result: outer.subtract(inner) };
    },
  },
  {
    id: "boolean-intersection-cube-sphere",
    family: "booleans",
    tolerance: "mesh-tessellated",
    notes: "The intersection of a cube and a sphere whose radius exceeds the cube's half-diagonal on the faces but not the corners — a rounded-cube result with six flat faces and eight spherical corners, genus 0.",
    build: ({ Manifold }) => {
      const box = Manifold.cube([20, 20, 20], true);
      const sphere = Manifold.sphere(13, 32);
      return { operands: [{ role: "operand-a", shape: box }, { role: "operand-b", shape: sphere }], result: box.intersect(sphere) };
    },
  },
  {
    id: "boolean-intersection-cylinder-cylinder",
    family: "booleans",
    tolerance: "mesh-tessellated",
    notes: "Two equal-radius cylinders on perpendicular axes, intersected — the classic Steinmetz solid (a bicylinder), whose closed-form volume 16r³/3 makes this fixture independently checkable against the tessellated measurement.",
    build: ({ Manifold }) => {
      const a = Manifold.cylinder(40, 10, 10, 48, true);
      const b = Manifold.cylinder(40, 10, 10, 48, true).rotate([90, 0, 0]);
      return { operands: [{ role: "operand-a", shape: a }, { role: "operand-b", shape: b }], result: a.intersect(b) };
    },
  },
  {
    id: "boolean-union-cube-cylinder-boss",
    family: "booleans",
    tolerance: "mesh-tessellated",
    notes: "A box with a cylindrical boss fused onto one face — the additive counterpart to a bore, and a shape whose result has a genuinely mixed planar/curved boundary.",
    build: ({ Manifold }) => {
      const box = Manifold.cube([30, 30, 10], true);
      const boss = Manifold.cylinder(8, 6, 6, 32).translate([0, 0, 5]);
      return { operands: [{ role: "operand-a", shape: box }, { role: "operand-b", shape: boss }], result: box.add(boss) };
    },
  },
  {
    id: "boolean-difference-torus-minus-cube",
    family: "booleans",
    tolerance: "mesh-tessellated",
    notes: "A genus-1 torus with an external notch cut by a box that does not reach the torus's own hole — the topology stays genus 1, the cross-check that an unrelated external cut does not disturb a shape's existing handle.",
    build: ({ CrossSection, Manifold }) => {
      const torus = CrossSection.circle(4, 32).translate(14, 0).revolve(32, 360);
      const notch = Manifold.cube([6, 6, 20], true).translate([14, 0, 0]);
      return { operands: [{ role: "operand-a", shape: torus }], result: torus.subtract(notch) };
    },
  },
  {
    id: "boolean-union-three-spheres-chain",
    family: "booleans",
    tolerance: "mesh-tessellated",
    notes: "Three overlapping spheres fused in a row — each pair overlaps by half a radius, so the union is one connected blob (genus 0), the multi-operand counterpart to the two-sphere union case.",
    build: ({ Manifold }) => {
      const a = Manifold.sphere(9, 32);
      const b = Manifold.sphere(9, 32).translate([11, 0, 0]);
      const c = Manifold.sphere(9, 32).translate([22, 0, 0]);
      return { operands: [{ role: "operand-a", shape: a }, { role: "operand-b", shape: b }], result: a.add(b).add(c) };
    },
  },
  {
    id: "boolean-difference-cube-countersink",
    family: "booleans",
    tolerance: "mesh-tessellated",
    notes: "A box with a countersunk through-hole — a cone fused onto a cylinder as ONE cutting tool, then subtracted in a single pass, rather than two separate cuts. Genus 1, and the case that exercises a compound cutter built from two different primitive families.",
    build: ({ Manifold }) => {
      const box = Manifold.cube([30, 30, 16], true);
      const shaft = Manifold.cylinder(16, 4, 4, 32, true);
      const sink = Manifold.cylinder(3, 7, 4, 32).translate([0, 0, 6.5]);
      const tool = shaft.add(sink);
      return { operands: [{ role: "operand-a", shape: box }, { role: "operand-b", shape: tool }], result: box.subtract(tool) };
    },
  },
  {
    id: "boolean-intersection-sphere-sphere-lens",
    family: "booleans",
    tolerance: "mesh-tessellated",
    notes: "The intersection of two equal, heavily overlapping spheres — a lens (vesica) solid, convex, genus 0, and the intersection-family counterpart to `boolean-union-overlapping-spheres`'s union of the same two operands.",
    build: ({ Manifold }) => {
      const a = Manifold.sphere(10, 32);
      const b = Manifold.sphere(10, 32).translate([8, 0, 0]);
      return { operands: [{ role: "operand-a", shape: a }, { role: "operand-b", shape: b }], result: a.intersect(b) };
    },
  },
  {
    id: "boolean-union-cube-sphere-tangent-contact",
    family: "booleans",
    tolerance: "mesh-tessellated",
    notes: "A sphere placed so it touches one face of a box at exactly one point without overlapping it. Like the tangent-spheres fixture, tessellated tangency does not fuse into one solid — declared and measured as a 2-component result, not assumed.",
    build: ({ Manifold }) => {
      const box = Manifold.cube([20, 20, 20], true);
      const sphere = Manifold.sphere(8, 32).translate([18, 0, 0]);
      return { operands: [{ role: "operand-a", shape: box }, { role: "operand-b", shape: sphere }], result: box.add(sphere) };
    },
  },
  {
    id: "boolean-difference-cube-two-crossing-bores",
    family: "booleans",
    tolerance: "mesh-tessellated",
    notes: "A box with two through-bores on perpendicular axes that CROSS each other inside the material, merging into one combined internal cavity rather than two separate tunnels. MEASURED genus 3, not the 2 a naive per-hole count would predict — the crossing junction itself adds a third handle beyond the one each bore would contribute independently, which is exactly why this fixture exists rather than trusting the naive count.",
    build: ({ Manifold }) => {
      const box = Manifold.cube([30, 30, 30], true);
      const boreX = Manifold.cylinder(40, 5, 5, 32, true).rotate([0, 90, 0]);
      const boreY = Manifold.cylinder(40, 5, 5, 32, true).rotate([90, 0, 0]);
      return { operands: [{ role: "operand-a", shape: box }], result: box.subtract(boreX).subtract(boreY) };
    },
  },
  {
    id: "boolean-difference-sphere-crescent",
    family: "booleans",
    tolerance: "mesh-tessellated",
    notes: "A sphere with a second, off-center sphere subtracted so the cutter pokes through the outer surface — the removed cavity opens to the outside rather than staying enclosed, leaving a single-solid crescent/dent rather than a nested void. The direct contrast case for `boolean-difference-nested-cavity-sphere`, where the same operation with the cutter fully enclosed produces two disjoint boundary components instead of one.",
    build: ({ Manifold }) => {
      const outer = Manifold.sphere(15, 32);
      const cutter = Manifold.sphere(10, 32).translate([12, 0, 0]);
      return { operands: [{ role: "operand-a", shape: outer }, { role: "operand-b", shape: cutter }], result: outer.subtract(cutter) };
    },
  },
  {
    id: "boolean-difference-cube-slot-through",
    family: "booleans",
    tolerance: "mesh-tessellated",
    notes: "A box with a rectangular slot cut all the way through — a through-hole built from planar faces instead of a cylinder, genus 1, and the rectangular-cutter cross-check for the round-bore fixtures.",
    build: ({ Manifold }) => {
      const box = Manifold.cube([40, 24, 12], true);
      const slot = Manifold.cube([10, 40, 6], true);
      return { operands: [{ role: "operand-a", shape: box }, { role: "operand-b", shape: slot }], result: box.subtract(slot) };
    },
  },
];
//#endregion 🧪️Recipes
