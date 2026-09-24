#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// ⚠️ BRep fixture recipes — the `robustness` family.
//
// A recipe DESCRIBES a case; it computes nothing. `../📜️script.ts` runs each `build`, exports the
// operands and the result as STEP, re-imports what it wrote and measures THAT, and records the bundle
// with its provenance. Every expected answer therefore comes out of the third-party kernel, and this
// file's job is to say which shapes and which declared outcome.
//
// 📐️ The three kernel conventions every recipe here depends on, all MEASURED rather than assumed:
//    `box(dx, dy, dz)` sits CORNER-at-origin.
//    `cylinder(r, h)` sits AXIS-at-origin, extending +z.
//    `rotate(shape, angleDEGREES, { at, axis })` takes ONE options object.
//
// @see ../📜️script.ts — the generator that runs these
// @see ../../🔬️probes/📜️script.ts — the probes that measure what they produced

//#endregion 🧲️Header

//#region 🔌️Adapters
import { type Kernel, type Recipe, call } from "../📜️script.ts";
//#endregion 🔌️Adapters

//#region 🧪️Recipes
/** ⚠️ The `robustness` recipes. */
export const RECIPES: readonly Recipe[] = [
  {
    id: "fuse-edge-touching-boxes",
    family: "robustness",
    outcome: "disjoint",
    tolerance: "contact-sensitive",
    notes: "Edge contact only — the degenerate case between 'joined' and 'two bodies'. MEASURED: this kernel leaves TWO solids (12 faces, 23 edges, total volume 2000), so the declared class is DISJOINT, not applied. The volume is identical either way; only the component count separates the two answers, which is why component count is asserted and face count is not.",
    build: (b) => {
      const left = call(b, "box", 10, 10, 10);
      const diagonal = call(b, "translate", call(b, "box", 10, 10, 10), [10, 10, 0]);
      return { operands: [{ role: "operand-a-step", shape: left }, { role: "operand-b-step", shape: diagonal }], result: call(b, "fuse", left, diagonal) };
    },
  },
  {
    id: "cut-tangent-cylinder-epsilon-below",
    family: "robustness",
    outcome: "no-op",
    tolerance: "epsilon-degenerate",
    notes: "Epsilon BELOW contact: the cutter misses by 1e-6. Declared no-op. The lower rung of the three-rung contact bracket.",
    build: (b) => {
      const box = call(b, "box", 20, 20, 20);
      const cutter = call(b, "translate", call(b, "cylinder", 5, 40), [-5 - 1e-6, 10, -10]);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: cutter }], result: call(b, "cut", box, cutter) };
    },
  },
  {
    id: "cut-tangent-cylinder-exact",
    family: "robustness",
    outcome: "applied",
    tolerance: "epsilon-degenerate",
    notes: "EXACT tangency, the middle rung of the contact bracket. MEASURED: the kernel removes ZERO volume (8000 to within 1.8e-12) but IMPRINTS the tangent line, taking the shape from 6 faces / 12 edges to 7 / 15. So the class is APPLIED, not no-op: a volume-only comparison cannot tell this apart from the epsilon-below rung, and the epsilon-below rung genuinely leaves 6 faces untouched.",
    build: (b) => {
      const box = call(b, "box", 20, 20, 20);
      const cutter = call(b, "translate", call(b, "cylinder", 5, 40), [-5, 10, -10]);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: cutter }], result: call(b, "cut", box, cutter) };
    },
  },
  {
    id: "cut-tangent-cylinder-epsilon-above",
    family: "robustness",
    outcome: "applied",
    tolerance: "epsilon-degenerate",
    notes: "Epsilon ABOVE contact: the cutter bites 1e-6 into the face. MEASURED: a sliver of 8.44e-8 is removed and the shape reaches 9 faces / 21 edges — the upper rung of the contact bracket, and the one a tolerance sized in millimetres would swallow whole.",
    build: (b) => {
      const box = call(b, "box", 20, 20, 20);
      const cutter = call(b, "translate", call(b, "cylinder", 5, 40), [-5 + 1e-6, 10, -10]);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: cutter }], result: call(b, "cut", box, cutter) };
    },
  },
  {
    id: "cut-coplanar-face-cutter",
    family: "robustness",
    outcome: "applied",
    tolerance: "contact-sensitive",
    notes: "Coplanar faces: the cutter's top face lies exactly on the box's mid-plane. Coplanarity is the classic source of kernel-dependent face splitting, which is why face counts are NOT asserted here.",
    build: (b) => {
      const box = call(b, "box", 20, 20, 20);
      const cutter = call(b, "translate", call(b, "box", 30, 30, 10), [-5, -5, 0]);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: cutter }], result: call(b, "cut", box, cutter) };
    },
  },
  {
    id: "cut-thin-wall-shell",
    family: "robustness",
    outcome: "applied",
    tolerance: "epsilon-degenerate",
    notes: "A 0.2 mm wall left behind by a nested cut. Thin walls are where tessellation-only comparison passes while the exact shape is wrong.",
    build: (b) => {
      const outer = call(b, "box", 20, 20, 20);
      const inner = call(b, "translate", call(b, "box", 19.6, 19.6, 19.6), [0.2, 0.2, 0.2]);
      return { operands: [{ role: "operand-a-step", shape: outer }, { role: "operand-b-step", shape: inner }], result: call(b, "cut", outer, inner) };
    },
  },
  {
    id: "cut-micro-scale-bore",
    family: "robustness",
    outcome: "applied",
    tolerance: "micro-scale",
    notes: "Sub-millimetre geometry. An absolute tolerance sized for millimetres would swallow the whole model, which is precisely what the scale-relative resolution rule exists to prevent.",
    build: (b) => {
      const box = call(b, "box", 0.02, 0.02, 0.02);
      const bore = call(b, "translate", call(b, "cylinder", 0.005, 0.04), [0.01, 0.01, -0.01]);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: bore }], result: call(b, "cut", box, bore) };
    },
  },
  {
    id: "cut-large-coordinate-bore",
    family: "robustness",
    outcome: "applied",
    tolerance: "large-coordinate",
    notes: "The same bored box translated a kilometre from the origin. Absolute error grows with the coordinate, so only the relative term is meaningful here.",
    build: (b) => {
      const box = call(b, "translate", call(b, "box", 20, 20, 20), [1000000, 0, 0]);
      const bore = call(b, "translate", call(b, "cylinder", 5, 40), [1000010, 10, -10]);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: bore }], result: call(b, "cut", box, bore) };
    },
  },

  // 🔻️VERTEX-touching bracket: two 10³ cubes sharing only a single corner point. Three rungs at ±1e-6.
  {
    id: "fuse-vertex-touching-boxes-epsilon-below",
    family: "robustness",
    outcome: "disjoint",
    tolerance: "epsilon-degenerate",
    notes: "Vertex-contact bracket, lower rung: the diagonal cube's corner misses by 1e-6 in all three axes. MEASURED: 2 solids, 12 faces / 24 edges / 16 vertices, total volume 2000.0000000000002.",
    build: (b) => {
      const a = call(b, "box", 10, 10, 10);
      const c = call(b, "translate", call(b, "box", 10, 10, 10), [10 + 1e-6, 10 + 1e-6, 10 + 1e-6]);
      return { operands: [{ role: "operand-a-step", shape: a }, { role: "operand-b-step", shape: c }], result: call(b, "fuse", a, c) };
    },
  },
  {
    id: "fuse-vertex-touching-boxes-exact",
    family: "robustness",
    outcome: "disjoint",
    tolerance: "epsilon-degenerate",
    notes: "Vertex-contact bracket, middle rung: the diagonal cube's corner touches the base cube's corner EXACTLY — a single shared point, the most degenerate contact this corpus can express. MEASURED: 2 solids, 12 faces / 24 edges / 16 vertices, volume 1999.999999999999 — numerically IDENTICAL topology to the epsilon-below rung. A single tangent point produces no imprint at all: the kernel does not even split a face at the touching corner, unlike full-edge or full-face contact.",
    build: (b) => {
      const a = call(b, "box", 10, 10, 10);
      const c = call(b, "translate", call(b, "box", 10, 10, 10), [10, 10, 10]);
      return { operands: [{ role: "operand-a-step", shape: a }, { role: "operand-b-step", shape: c }], result: call(b, "fuse", a, c) };
    },
  },
  {
    id: "fuse-vertex-touching-boxes-epsilon-above",
    family: "robustness",
    outcome: "applied",
    tolerance: "epsilon-degenerate",
    notes: "Vertex-contact bracket, upper rung: the diagonal cube overlaps the base cube by a 1e-6³ corner cube. MEASURED: 1 solid, 12 faces / 30 edges / 20 vertices, volume 1999.9999999999993 — merges cleanly per the exact kernel. CAVEAT: the default-tolerance (1e-3) tessellation of this exact result is rejected as 'Not manifold' by the independent manifold-3d mesh engine, at every tessellation tolerance tried down to 1e-8 — the exact BRep is valid but its standard mesh is not, the sharpest disagreement this bracket family produced. See the report's mesh-verification section.",
    build: (b) => {
      const a = call(b, "box", 10, 10, 10);
      const c = call(b, "translate", call(b, "box", 10, 10, 10), [10 - 1e-6, 10 - 1e-6, 10 - 1e-6]);
      return { operands: [{ role: "operand-a-step", shape: a }, { role: "operand-b-step", shape: c }], result: call(b, "fuse", a, c) };
    },
  },

  // 🔲️COPLANAR partial-face contact bracket: two slabs whose contact plane is shared but only HALF
  // the face area overlaps in-plane — unlike `fuse-face-touching-boxes`, which shares the whole face.
  {
    id: "fuse-coplanar-partial-face-epsilon-below",
    family: "robustness",
    outcome: "disjoint",
    tolerance: "epsilon-degenerate",
    notes: "Coplanar partial-face bracket, lower rung: the lid sits 1e-6 above the base's top plane, half out of alignment in x. MEASURED: 2 solids, 12 faces / 24 edges / 16 vertices, volume 8000.000000000001.",
    build: (b) => {
      const base = call(b, "box", 20, 20, 10);
      const lid = call(b, "translate", call(b, "box", 20, 20, 10), [10, 0, 10 + 1e-6]);
      return { operands: [{ role: "operand-a-step", shape: base }, { role: "operand-b-step", shape: lid }], result: call(b, "fuse", base, lid) };
    },
  },
  {
    id: "fuse-coplanar-partial-face-exact",
    family: "robustness",
    outcome: "applied",
    tolerance: "epsilon-degenerate",
    notes: "Coplanar partial-face bracket, middle rung: the lid's bottom face is EXACTLY coplanar with the base's top face, overlapping only the x:[10,20] half of it — a partial, coplanar, non-degenerate-in-area contact distinct from full-face touching. MEASURED: 1 solid, 12 faces / 26 edges / 16 vertices, volume 7999.999999999999, area 2800 (down from 3200 disjoint-total — the coincident half-faces are consumed, not doubled). Unlike the vertex and sphere-tangent brackets, a partial but non-zero-area coplanar contact DOES merge into one solid.",
    build: (b) => {
      const base = call(b, "box", 20, 20, 10);
      const lid = call(b, "translate", call(b, "box", 20, 20, 10), [10, 0, 10]);
      return { operands: [{ role: "operand-a-step", shape: base }, { role: "operand-b-step", shape: lid }], result: call(b, "fuse", base, lid) };
    },
  },
  {
    id: "fuse-coplanar-partial-face-epsilon-above",
    family: "robustness",
    outcome: "applied",
    tolerance: "epsilon-degenerate",
    notes: "Coplanar partial-face bracket, upper rung: the lid embeds 1e-6 into the base across the shared half-face. MEASURED: 1 solid, 14 faces / 32 edges / 20 vertices, volume 7999.9998000000005 (Δ≈2e-4 from the exact rung, matching the 1e-6-thick overlap slab). CAVEAT: like the vertex-touching upper rung, this exact result's default-tolerance mesh is rejected as 'Not manifold' by the independent manifold-3d engine at every tessellation tolerance tried — see the report.",
    build: (b) => {
      const base = call(b, "box", 20, 20, 10);
      const lid = call(b, "translate", call(b, "box", 20, 20, 10), [10, 0, 10 - 1e-6]);
      return { operands: [{ role: "operand-a-step", shape: base }, { role: "operand-b-step", shape: lid }], result: call(b, "fuse", base, lid) };
    },
  },

  // 🛢️COAXIAL cylinders bracket: two r=5 cylinders sharing one axis, stacked cap-to-cap.
  {
    id: "fuse-coaxial-cylinders-epsilon-below",
    family: "robustness",
    outcome: "disjoint",
    tolerance: "epsilon-degenerate",
    notes: "Coaxial-cylinder bracket, lower rung: the upper cylinder's base cap sits 1e-6 above the lower cylinder's top cap, same axis, same radius. MEASURED: 2 solids, 6 faces / 6 edges / 4 vertices (3 faces / 3 edges / 2 vertices per cylinder), volume 3141.5926535897934.",
    build: (b) => {
      const lower = call(b, "cylinder", 5, 20);
      const upper = call(b, "translate", call(b, "cylinder", 5, 20), [0, 0, 20 + 1e-6]);
      return { operands: [{ role: "operand-a-step", shape: lower }, { role: "operand-b-step", shape: upper }], result: call(b, "fuse", lower, upper) };
    },
  },
  {
    id: "fuse-coaxial-cylinders-exact",
    family: "robustness",
    outcome: "applied",
    tolerance: "epsilon-degenerate",
    notes: "Coaxial-cylinder bracket, middle rung: the two r=5 cylinders share one axis and touch cap-to-cap EXACTLY — a full circular face in contact, the cylindrical analogue of `fuse-face-touching-boxes`. MEASURED: merges to 1 solid, 4 faces / 5 edges / 3 vertices, volume 3141.5926535897934 — IDENTICAL volume to the epsilon-below rung, the contact plane is IMPRINTED (a residual seam between the two cylindrical side faces) rather than fully healed into a single continuous side face. The direct cylindrical analogue of `cut-tangent-cylinder-exact`'s zero-volume imprint.",
    build: (b) => {
      const lower = call(b, "cylinder", 5, 20);
      const upper = call(b, "translate", call(b, "cylinder", 5, 20), [0, 0, 20]);
      return { operands: [{ role: "operand-a-step", shape: lower }, { role: "operand-b-step", shape: upper }], result: call(b, "fuse", lower, upper) };
    },
  },
  {
    id: "fuse-coaxial-cylinders-epsilon-above",
    family: "robustness",
    outcome: "applied",
    tolerance: "epsilon-degenerate",
    notes: "Coaxial-cylinder bracket, upper rung: the two same-radius, same-axis cylinders overlap by 1e-6 along their shared axis. MEASURED: 1 solid, 5 faces / 7 edges / 4 vertices, volume 3141.5925750499773 — Δ≈7.85e-5 from the exact rung's volume, matching the analytic overlap π·5²·1e-6=7.854e-5 to 4 significant figures. Distinguishable from the exact rung by both volume and topology.",
    build: (b) => {
      const lower = call(b, "cylinder", 5, 20);
      const upper = call(b, "translate", call(b, "cylinder", 5, 20), [0, 0, 20 - 1e-6]);
      return { operands: [{ role: "operand-a-step", shape: lower }, { role: "operand-b-step", shape: upper }], result: call(b, "fuse", lower, upper) };
    },
  },

  // 🔮️SPHERE-tangent-to-plane bracket: an r=5 sphere resting on the flat top face of a slab.
  {
    id: "fuse-sphere-tangent-plane-epsilon-below",
    family: "robustness",
    outcome: "disjoint",
    tolerance: "epsilon-degenerate",
    notes: "Sphere/plane tangency bracket, lower rung: the sphere's pole sits 1e-6 above the slab's top face (z=10). MEASURED: 2 solids, 7 faces / 15 edges / 10 vertices, volume 4523.598775598298.",
    build: (b) => {
      const slab = call(b, "box", 20, 20, 10);
      const ball = call(b, "translate", call(b, "sphere", 5), [10, 10, 15 + 1e-6]);
      return { operands: [{ role: "operand-a-step", shape: slab }, { role: "operand-b-step", shape: ball }], result: call(b, "fuse", slab, ball) };
    },
  },
  {
    id: "fuse-sphere-tangent-plane-exact",
    family: "robustness",
    outcome: "disjoint",
    tolerance: "epsilon-degenerate",
    notes: "Sphere/plane tangency bracket, middle rung: the sphere's pole touches the slab's flat top face at a SINGLE point (z=15 puts the pole exactly on z=10). A doubly-curved surface meeting a plane in a point is the most degenerate contact a curved/planar pair can have. MEASURED: 2 solids, 7 faces / 15 edges / 10 vertices, volume 4523.598775598298 — numerically IDENTICAL to the epsilon-below rung down to the last measured digit. Confirms the vertex-touching bracket's finding on a curved surface: a single tangent point produces NO imprint and no merge, unlike the coaxial-cylinder bracket's full-circle cap contact, which does imprint.",
    build: (b) => {
      const slab = call(b, "box", 20, 20, 10);
      const ball = call(b, "translate", call(b, "sphere", 5), [10, 10, 15]);
      return { operands: [{ role: "operand-a-step", shape: slab }, { role: "operand-b-step", shape: ball }], result: call(b, "fuse", slab, ball) };
    },
  },
  {
    id: "fuse-sphere-tangent-plane-epsilon-above",
    family: "robustness",
    outcome: "applied",
    tolerance: "epsilon-degenerate",
    notes: "Sphere/plane tangency bracket, upper rung: the sphere sinks 1e-6 into the slab, producing a minute spherical cap of intersection. MEASURED: 1 solid, 7 faces / 15 edges / 10 vertices (same counts as the other two rungs — the cap intersection doesn't add topology here, since the merged single face where the sphere meets the top face is not a new distinct face at this tessellation-independent BRep level), volume 4523.598775598248, Δ≈5e-11 from the exact rung. Distinguished from the exact rung by solid count (1 vs 2) and this minute volume delta, not by faces/edges.",
    build: (b) => {
      const slab = call(b, "box", 20, 20, 10);
      const ball = call(b, "translate", call(b, "sphere", 5), [10, 10, 15 - 1e-6]);
      return { operands: [{ role: "operand-a-step", shape: slab }, { role: "operand-b-step", shape: ball }], result: call(b, "fuse", slab, ball) };
    },
  },

  // 🗡️EDGE-on-face bracket: a 20-long diamond-section blade whose bottom EDGE (not a face, not a
  // point) runs the full length of a slab's flat top face. The blade is a box rotated 45° about the
  // x-axis so its square cross-section becomes a diamond; its lower vertex, extruded along x, is a
  // line — the geometry needed for a genuine edge/face contact rather than vertex or face contact.
  {
    id: "fuse-edge-on-face-epsilon-below",
    family: "robustness",
    outcome: "disjoint",
    tolerance: "epsilon-degenerate",
    notes: "Edge-on-face bracket, lower rung: the blade's bottom edge sits 1e-6 above the slab's top face along its entire 20-unit length. MEASURED: 2 solids, 12 faces / 24 edges / 16 vertices, volume 5279.999999999904.",
    build: (b) => {
      const slab = call(b, "box", 20, 20, 10);
      const half = 4;
      const bottomZ = half - half * Math.sqrt(2);
      const lift = 10 - bottomZ;
      const diamond = call(b, "rotate", call(b, "box", 20, 8, 8), 45, { at: [0, half, half], axis: [1, 0, 0] });
      const blade = call(b, "translate", diamond, [0, 6, lift + 1e-6]);
      return { operands: [{ role: "operand-a-step", shape: slab }, { role: "operand-b-step", shape: blade }], result: call(b, "fuse", slab, blade) };
    },
  },
  {
    id: "fuse-edge-on-face-exact",
    family: "robustness",
    outcome: "disjoint",
    tolerance: "epsilon-degenerate",
    notes: "Edge-on-face bracket, middle rung: the blade's diamond-section bottom vertex, extruded along its 20-unit length into a straight EDGE, rests EXACTLY on the slab's flat top face. MEASURED: this stays TWO solids (12→13 faces, 24→27 edges, 16→18 vertices versus the epsilon-below rung — the contact line IS imprinted onto the slab's top face) but the imprint does not merge the bodies, so the declared class is DISJOINT, not applied — the same reasoning `fuse-edge-touching-boxes` established for full-edge contact between two boxes, now confirmed for a genuine line contact between a curved-cross-section blade and a flat face. Volume is identical to the epsilon-below rung (5279.999999999904) — only the topology changes.",
    build: (b) => {
      const slab = call(b, "box", 20, 20, 10);
      const half = 4;
      const bottomZ = half - half * Math.sqrt(2);
      const lift = 10 - bottomZ;
      const diamond = call(b, "rotate", call(b, "box", 20, 8, 8), 45, { at: [0, half, half], axis: [1, 0, 0] });
      const blade = call(b, "translate", diamond, [0, 6, lift]);
      return { operands: [{ role: "operand-a-step", shape: slab }, { role: "operand-b-step", shape: blade }], result: call(b, "fuse", slab, blade) };
    },
  },
  {
    id: "fuse-edge-on-face-epsilon-above",
    family: "robustness",
    outcome: "applied",
    tolerance: "epsilon-degenerate",
    notes: "Edge-on-face bracket, upper rung: the blade's bottom edge embeds 1e-6 into the slab along its full length. MEASURED: 1 solid, 15 faces / 33 edges / 20 vertices, volume 5279.99999999992.",
    build: (b) => {
      const slab = call(b, "box", 20, 20, 10);
      const half = 4;
      const bottomZ = half - half * Math.sqrt(2);
      const lift = 10 - bottomZ;
      const diamond = call(b, "rotate", call(b, "box", 20, 8, 8), 45, { at: [0, half, half], axis: [1, 0, 0] });
      const blade = call(b, "translate", diamond, [0, 6, lift - 1e-6]);
      return { operands: [{ role: "operand-a-step", shape: slab }, { role: "operand-b-step", shape: blade }], result: call(b, "fuse", slab, blade) };
    },
  },

  // 🩹️Slivers and degeneracy — five distinct ways a shape can sit at or below the kernel's own
  // working tolerance rather than merely near another shape's contact surface.
  {
    id: "cut-sliver-intersection",
    family: "robustness",
    outcome: "rejected",
    tolerance: "epsilon-degenerate",
    notes: "A sliver cutter box(1e-7, 20, 20) shaving a full-face, near-zero-depth slice off a 20³ box. MEASURED: the kernel does not reach the boolean at all — `box(1e-7, ...)` ITSELF throws a bare `WebAssembly.Exception` with no message, no name and no enumerable properties (`Object.keys(e).length === 0`, `String(e) === '[object WebAssembly.Exception]'`) the instant any box dimension is exactly 1e-7. A scratch bisection (not part of this pipeline) found the boundary is exactly between 1e-7 (throws) and 2e-7 (succeeds, box constructs and cuts cleanly) — the kernel's own working tolerance is a hard wall for `box()`, not a soft one: there is no degraded-but-valid regime below it, only success or an opaque crash. `result`/`operands` fall back to an ordinary disjoint cut (recorded for provenance only); the finding is the exception, not the fallback numbers.",
    build: (b) => {
      let caught = "none";
      try {
        call(b, "box", 1e-7, 20, 20);
      } catch (error) {
        caught = String(error);
      }
      if (caught === "none") throw new Error("expected box(1e-7, 20, 20) to throw — the sliver boundary this fixture pins has moved, recipe needs re-deriving");
      const box = call(b, "box", 20, 20, 20);
      const away = call(b, "translate", call(b, "box", 10, 10, 10), [500, 0, 0]);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: away }], result: call(b, "cut", box, away) };
    },
  },
  {
    id: "cut-tiny-edge-below-tolerance",
    family: "robustness",
    outcome: "rejected",
    tolerance: "epsilon-degenerate",
    notes: "A corner notch box(1e-7, 1e-7, 1e-7) meant to shave the very corner of a 20³ box, producing edges at the kernel's own 1e-7 working tolerance. MEASURED: same root cause as `cut-sliver-intersection` — `box(1e-7, 1e-7, 1e-7)` itself throws the identical bare, message-less `WebAssembly.Exception` before any boolean runs. The two fixtures independently hit the SAME constructor guard (any box dimension ≤1e-7 throws, ≥2e-7 succeeds), which is itself a finding: two structurally different degeneracies (a full-face sliver, a corner notch) turn out to share one root cause in `box()`, not two. `result`/`operands` fall back to an ordinary disjoint cut (recorded for provenance only).",
    build: (b) => {
      let caught = "none";
      try {
        call(b, "box", 1e-7, 1e-7, 1e-7);
      } catch (error) {
        caught = String(error);
      }
      if (caught === "none") throw new Error("expected box(1e-7, 1e-7, 1e-7) to throw — the tiny-edge boundary this fixture pins has moved, recipe needs re-deriving");
      const box = call(b, "box", 20, 20, 20);
      const away = call(b, "translate", call(b, "box", 10, 10, 10), [500, 0, 0]);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: away }], result: call(b, "cut", box, away) };
    },
  },
  {
    id: "cut-narrow-channel",
    family: "robustness",
    outcome: "disjoint",
    tolerance: "epsilon-degenerate",
    notes: "A 1e-6-wide slot cut clean through a 30×20×10 slab, edge-to-edge and top-to-bottom, splitting it into two halves separated by a gap four orders of magnitude below a millimetre. MEASURED: the kernel DOES separate the two halves — 2 solids, 12 faces / 24 edges / 16 vertices (identical topology to two independent boxes), volume 5999.9996999999985 (Δ≈3e-4 from the undivided 6000, matching the tiny channel volume). No merge-tolerance artifact: a 1e-6 gap is enough to keep the halves apart.",
    build: (b) => {
      const slab = call(b, "box", 30, 20, 10);
      const channel = call(b, "translate", call(b, "box", 30, 1e-6, 15), [0, 10 - 5e-7, -2.5]);
      return { operands: [{ role: "operand-a-step", shape: slab }, { role: "operand-b-step", shape: channel }], result: call(b, "cut", slab, channel) };
    },
  },
  {
    id: "fuse-near-coplanar-faces-1e-9-radians",
    family: "robustness",
    outcome: "applied",
    tolerance: "epsilon-degenerate",
    notes: "Two 20×20×10 slabs share a bottom-edge hinge line, but the lid is tilted 1e-9 RADIANS (5.72957795e-8°) about that edge rather than sitting flat — so the two faces are coplanar at one edge and diverge to ~2e-8 units of overlap at the far edge, 20 units away. MEASURED: 1 solid, 10 faces / 20 edges / 12 vertices, volume 7999.999996 (Δ≈4e-6 from the undivided 8000, consistent with a wedge-shaped sliver of that magnitude). The kernel treats the whole continuously-varying overlap as one contact rather than splitting it at some internal angular threshold.",
    build: (b) => {
      const base = call(b, "box", 20, 20, 10);
      const flatLid = call(b, "translate", call(b, "box", 20, 20, 10), [0, 0, 10]);
      const tiltedLid = call(b, "rotate", flatLid, -(1e-9 * (180 / Math.PI)), { at: [0, 0, 10], axis: [1, 0, 0] });
      return { operands: [{ role: "operand-a-step", shape: base }, { role: "operand-b-step", shape: tiltedLid }], result: call(b, "fuse", base, tiltedLid) };
    },
  },
  {
    id: "cut-high-aspect-ratio-bore",
    family: "robustness",
    outcome: "applied",
    tolerance: "epsilon-degenerate",
    notes: "A hair-thin bore of radius 5e-4 (diameter 1e-3) through a 1000-unit cube: a 1e6:1 aspect ratio between the bore's length and its diameter, the extreme end of what surface–surface intersection is expected to resolve. MEASURED: the exact kernel resolves it cleanly — 1 solid, 7 faces / 15 edges / 10 vertices, volume 999999999.9992146 (Δ≈7.85e-4 from the undivided 1e9, matching the analytic bore volume π·(5e-4)²·1000≈7.854e-4). CAVEAT: this exact result's default-tolerance (1e-3) tessellation is rejected as 'Not manifold' by the independent manifold-3d engine — the bore's own diameter (1e-3) is comparable to the tessellation chord tolerance, and no tolerance retried resolves it. The exact BRep and its own standard mesh disagree on validity; see the report.",
    build: (b) => {
      const cube = call(b, "box", 1000, 1000, 1000);
      const bore = call(b, "translate", call(b, "cylinder", 5e-4, 1200), [500, 500, -100]);
      return { operands: [{ role: "operand-a-step", shape: cube }, { role: "operand-b-step", shape: bore }], result: call(b, "cut", cube, bore) };
    },
  },

  // 📏️Scale sweep — the same bored-box operation at 1e3 and 1e6. The 1e-3 and 1 rungs already exist
  // in this corpus as `cut-micro-scale-bore` and `cut-bored-box-through` (spatial-relationship); this
  // adds the two missing rungs rather than re-declaring identical geometry under a new name.
  {
    id: "cut-bore-scale-1e3",
    family: "robustness",
    outcome: "applied",
    tolerance: "large-coordinate",
    notes: "The bored-box operation at 1e3× — box 20000³, bore r=5000/h=40000 — the third of a planned four-point scale sweep whose other points are `cut-micro-scale-bore` (1e-3) and `cut-bored-box-through` (1); the fourth (1e6) was ATTEMPTED and ABANDONED, see the comment immediately below this recipe for why. MEASURED: 1 solid, 7 faces / 15 edges / 10 vertices, volume 6429203673205.049 — topologically identical to the base-scale rung, confirming the exact Boolean itself is scale-invariant up to 1e3×. The sweep's real finding is at 1e6×, where the generator's own fixed-absolute-tolerance MESHING stage — not the Boolean — stopped being tractable.",
    build: (b) => {
      const box = call(b, "box", 20000, 20000, 20000);
      const bore = call(b, "translate", call(b, "cylinder", 5000, 40000), [10000, 10000, -10000]);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: bore }], result: call(b, "cut", box, bore) };
    },
  },
  // 🚫️`cut-bore-scale-1e6` — box 2e7³ (20 kilometres), bore r=5e6/h=4e7 — was ATTEMPTED as the top rung
  // of this sweep and ABANDONED. The exact Boolean cut itself completes in under a second (`expected.step`
  // was written to disk within the first second of the run, and its 20 KB size is unremarkable), but the
  // generator's own subsequent MEASUREMENT stage then hung: `expected.metrics.json` and
  // `expected.mesh.json` never appeared, and the process was still running, still climbing in memory
  // (1.2-2.4 GB and rising), when it was killed after 12+ minutes. The generator meshes every non-empty
  // result at a single ABSOLUTE 1e-3 tessellation tolerance — at a 2e7-unit shape that is a relative
  // tolerance of 5e-11, which asks for a triangulation fine enough to be intractable. This is kept out of
  // the corpus rather than declared `rejected`, because a hang is not a clean refusal and leaving it in
  // this file would hang any future unscoped `generate` run. See the report's "sharpest boundary" section.



  {
    id: "cut-tiny-bore-far-from-origin",
    family: "robustness",
    outcome: "applied",
    tolerance: "large-coordinate",
    notes: "Combines the two axes `cut-large-coordinate-bore` and `cut-micro-scale-bore` each exercise alone: a normal-size 20³ box translated 1e6 units from the origin, bored by a 1e-3-radius channel — a feature seven orders of magnitude smaller than its own coordinate offset. MEASURED: 1 solid, 7 faces / 15 edges / 10 vertices, volume 7999.999937168143 — Δ≈6.28e-5 from the undivided 8000, matching the analytic bore volume π·(1e-3)²·20≈6.28e-5 almost exactly. A negative result worth recording: this kernel's double-precision arithmetic still resolves a 1e-3 feature at a 1e6 coordinate offset without visible degradation beyond the expected analytic value.",
    build: (b) => {
      const box = call(b, "translate", call(b, "box", 20, 20, 20), [1000000, 0, 0]);
      const bore = call(b, "translate", call(b, "cylinder", 1e-3, 40), [1000010, 10, -10]);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: bore }], result: call(b, "cut", box, bore) };
    },
  },

  // ⛓️Repeated boolean chains and operand order.
  {
    id: "cut-chain-ten-sequential",
    family: "robustness",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "Ten sequential through-bores drilled one at a time down a 100×20×20 bar, each cut applied to the RESULT of the previous one — the chain-depth case the single-cutter fixtures cannot exercise. Like the `mechanical` family's own chains, only the base is exported, since a 10-step chain has no two-operand reproduction path. MEASURED: 1 solid, 16 faces (6 base + 10 bore side faces — matches 6+10 exactly) / 42 edges / 28 vertices, volume 34345.1332235384 (matching the analytic 40000−10·π·3²·20=34345.13… to 6 significant figures). Ten independent, non-overlapping cuts compound cleanly with no accumulated defect.",
    build: (b) => {
      let bar = call(b, "box", 100, 20, 20);
      for (const x of [5, 14, 23, 32, 41, 50, 59, 68, 77, 86]) bar = call(b, "cut", bar, call(b, "translate", call(b, "cylinder", 3, 40), [x, 10, -10]));
      return { operands: [{ role: "operand-a-step", shape: call(b, "box", 100, 20, 20) }], result: bar };
    },
  },
  {
    id: "cut-chain-order-b-then-c",
    family: "robustness",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A−B−C: base minus cutter B, then minus cutter C, where B and C mutually overlap (centres 10 apart, radius 6 each). Paired with `cut-chain-order-c-then-b`, which applies the same two cutters in the opposite order — Boolean subtraction is order-independent in set theory (A−B−C = A−C−B) and this pair checks whether the kernel actually agrees with itself down to volume and topology when the intermediate SHAPE differs between the two orders. MEASURED: 1 solid, 9 faces / 21 edges / 14 vertices, volume 13656.168803337727, area 4992.452293197475 — EXACTLY equal to `cut-chain-order-c-then-b` on every one of these numbers (see that fixture's notes). Order-independence holds.",
    build: (b) => {
      const base = call(b, "box", 30, 30, 20);
      const cutterB = call(b, "translate", call(b, "cylinder", 6, 40), [10, 15, -10]);
      const cutterC = call(b, "translate", call(b, "cylinder", 6, 40), [20, 15, -10]);
      const result = call(b, "cut", call(b, "cut", base, cutterB), cutterC);
      return { operands: [{ role: "operand-a-step", shape: base }, { role: "operand-b-step", shape: cutterB }, { role: "operand-c-step", shape: cutterC }], result };
    },
  },
  {
    id: "cut-chain-order-c-then-b",
    family: "robustness",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A−C−B: the same base and the same two mutually-overlapping cutters as `cut-chain-order-b-then-c`, applied in the opposite order. MEASURED: 1 solid, 9 faces / 21 edges / 14 vertices, volume 13656.168803337727, area 4992.452293197475 — EXACTLY equal to `cut-chain-order-b-then-c` on every measured number, despite the two orders producing a genuinely different intermediate shape after the first cut. Boolean subtraction's order-independence is confirmed at the kernel level, not just set-theoretically.",
    build: (b) => {
      const base = call(b, "box", 30, 30, 20);
      const cutterB = call(b, "translate", call(b, "cylinder", 6, 40), [10, 15, -10]);
      const cutterC = call(b, "translate", call(b, "cylinder", 6, 40), [20, 15, -10]);
      const result = call(b, "cut", call(b, "cut", base, cutterC), cutterB);
      return { operands: [{ role: "operand-a-step", shape: base }, { role: "operand-b-step", shape: cutterB }, { role: "operand-c-step", shape: cutterC }], result };
    },
  },

  // 🕳️Many cutters at once and nested cavities.
  {
    id: "cutall-many-cutters",
    family: "robustness",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "25 non-overlapping through-bores removed from a 100×20×10 plate in ONE `cutAll(base, tools)` batch call, rather than 25 sequential pairwise cuts — the N-way batch-boolean path the rest of this corpus never exercises with more than a handful of tools. The 25 cutters are also exported as one compound STEP so the reproduction path stays a single extra file. MEASURED: 1 solid, 31 faces (6 base + 25 bore side faces — matches 6+25 exactly) / 87 edges / 58 vertices, volume 18869.02664470766. `cutAll`'s single-batch result is topologically clean at 25 tools, same pattern the 10-tool sequential chain showed.",
    build: (b) => {
      const plate = call(b, "box", 100, 20, 10);
      const tools = Array.from({ length: 25 }, (_, i) => call(b, "translate", call(b, "cylinder", 1.2, 20), [2 + i * 3.9, 10, -5]));
      return { operands: [{ role: "operand-a-step", shape: plate }, { role: "operand-tools-step", shape: call(b, "compound", tools) }], result: call(b, "cutAll", plate, tools) };
    },
  },
  {
    id: "fuse-nested-void-in-void",
    family: "robustness",
    outcome: "disjoint",
    tolerance: "mechanical-standard",
    notes: "A void inside a void: the outer operand is a 30³ shell with a 20³ internal cavity; the inner operand is a separate, disjoint 8³ shell with its OWN 4³ internal cavity, floating entirely inside the outer cavity with 6 units of clearance on every side. MEASURED: 2 solids, 4 SHELLS (2 per solid — the outer boundary and the inner-cavity boundary of each, exactly as designed), 24 faces (12 per solid) / 48 edges / 32 vertices, volume 19447.999999999993 (=[30³−20³]+[8³−4³]=19000+448=19448, matching to 6 significant figures). Both independent cavities survive the fuse and the export/reimport round-trip with their own shell intact.",
    build: (b) => {
      const outer = call(b, "cut", call(b, "box", 30, 30, 30), call(b, "translate", call(b, "box", 20, 20, 20), [5, 5, 5]));
      const inner = call(b, "translate", call(b, "cut", call(b, "box", 8, 8, 8), call(b, "translate", call(b, "box", 4, 4, 4), [2, 2, 2])), [11, 11, 11]);
      return { operands: [{ role: "operand-a-step", shape: outer }, { role: "operand-b-step", shape: inner }], result: call(b, "fuse", outer, inner) };
    },
  },

  // 📐️Unit-conversion boundary: a feature positioned exactly astride the 1000 mm / 1 m crossover.
  {
    id: "cut-unit-boundary-slot",
    family: "robustness",
    outcome: "disjoint",
    tolerance: "epsilon-degenerate",
    notes: "A 0.001-unit-wide through-slot centred exactly on x=1000 (spanning 999.9995 to 1000.0005) — the millimetre/metre crossover coordinate — cut through a 2000×100×100 bar. MEASURED: 2 solids, 12 faces / 24 edges / 16 vertices, volume 19999989.999999996 (=2000·100·100−0.001·100·100=20000000−10=19999990, matching to 9 significant figures). The kernel splits cleanly at this boundary coordinate with no rounding artifact at the mm/m crossover.",
    build: (b) => {
      const bar = call(b, "box", 2000, 100, 100);
      const slot = call(b, "translate", call(b, "box", 0.001, 100, 100), [999.9995, 0, 0]);
      return { operands: [{ role: "operand-a-step", shape: bar }, { role: "operand-b-step", shape: slot }], result: call(b, "cut", bar, slot) };
    },
  },
];
//#endregion 🧪️Recipes
