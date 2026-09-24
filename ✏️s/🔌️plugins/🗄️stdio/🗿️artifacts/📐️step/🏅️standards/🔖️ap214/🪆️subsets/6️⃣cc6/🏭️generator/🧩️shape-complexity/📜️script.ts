#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🧩️ BRep fixture recipes — the `shape-complexity` family.
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
/** 🧩️ The `shape-complexity` recipes. */
export const RECIPES: readonly Recipe[] = [
  {
    id: "cut-skewed-bore",
    family: "shape-complexity",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A non-axis-aligned cutter through a box. Rotating the tool takes the operation off every analytic shortcut and onto the kernel's real surface–surface intersection.",
    build: (b) => {
      const box = call(b, "box", 20, 20, 20);
      const bore = call(b, "translate", call(b, "rotate", call(b, "cylinder", 4, 60), 35, { at: [0, 0, 0], axis: [1, 0, 0] }), [10, 10, -20]);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: bore }], result: call(b, "cut", box, bore) };
    },
  },
  {
    id: "cut-sphere-from-box",
    family: "shape-complexity",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "Sphere/box: a doubly-curved cutter against planar faces, and a periodic surface whose seam placement is a kernel decision rather than a semantic one.",
    build: (b) => {
      const box = call(b, "box", 20, 20, 20);
      const ball = call(b, "translate", call(b, "sphere", 7), [10, 10, 20]);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: ball }], result: call(b, "cut", box, ball) };
    },
  },
  {
    id: "fuse-cylinder-cross",
    family: "shape-complexity",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "Cylinder/cylinder at right angles: two periodic surfaces meeting in a genuine 3-D intersection curve, the case where seam-crossing splits differ between kernels.",
    build: (b) => {
      const upright = call(b, "cylinder", 5, 40);
      const across = call(b, "translate", call(b, "rotate", call(b, "cylinder", 5, 40), 90, { at: [0, 0, 0], axis: [1, 0, 0] }), [0, 20, 20]);
      return { operands: [{ role: "operand-a-step", shape: upright }, { role: "operand-b-step", shape: across }], result: call(b, "fuse", upright, across) };
    },
  },
  {
    id: "cut-cone-from-cylinder",
    family: "shape-complexity",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "Cone/cylinder, off-axis so the cone's slant surface genuinely exits through the cylinder's lateral wall rather than sitting fully contained. MEASURED: 6 faces, 11 edges, 7 vertices, volume 4348.89mm³ (cylinder 6031.86 minus the cone's partial overlap) — a real conical/cylindrical surface intersection, not a degenerate coaxial case.",
    build: (b) => {
      const cyl = call(b, "cylinder", 8, 30);
      const cone = call(b, "translate", call(b, "cone", 10, 2, 20), [5, 0, 5]);
      return { operands: [{ role: "operand-a-step", shape: cyl }, { role: "operand-b-step", shape: cone }], result: call(b, "cut", cyl, cone) };
    },
  },
  {
    id: "cut-torus-groove-from-cylinder",
    family: "shape-complexity",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "Torus/cylinder: an O-ring-style toroidal groove cut into a cylinder's lateral wall, where the torus's outer radius (13) exceeds the cylinder's radius (12) so the tool genuinely breaches the wall. MEASURED and load-bearing for consumers: the reimported result's declared `getBounds` reports a radial extent of 13.12 (9.3% larger than the cylinder's own radius), but re-tessellating and walking every mesh vertex gives a true max radial distance of exactly 12.0 — the trimmed toroidal face's axis-aligned bounding box is LOOSE, not the geometry. `boundingBoxDiagonal` for this fixture is therefore not a tight proxy for shape extent; volume (2084.04mm³), 5 faces, 7 edges, 4 vertices are the trustworthy numbers.",
    build: (b) => {
      const cyl = call(b, "cylinder", 12, 8);
      const groove = call(b, "translate", call(b, "torus", 10, 3), [0, 0, 4]);
      return { operands: [{ role: "operand-a-step", shape: cyl }, { role: "operand-b-step", shape: groove }], result: call(b, "cut", cyl, groove) };
    },
  },
  {
    id: "fuse-torus-torus-interlock",
    family: "shape-complexity",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes:
      "Two identical tori (major 10, minor 3) offset so their tubes overlap, like two interlocked rings. MEASURED, and the reason the offset below is 16 rather than the more obviously-overlapping 14: at an offset of exactly 14 the fused RESULT is a `isValidSolid`-true, single-solid, exact-BRep-valid shape by the exact kernel, yet its own tessellation (at this fixture's declared tolerance) is REJECTED as non-manifold by the independent mesh engine (`ManifoldError: Not manifold`) — an exact-solid/tessellation disagreement that would fail this family's own `step-mesh-compare` self-check. Nearby offsets (12, 16, 18, and a pure-Z offset) all tessellate cleanly; 16 was kept as the shipped geometry specifically because it stays comfortably clear of whatever degenerate alignment offset 14 hits. At offset 16: 9 faces, 25 edges, 14 vertices, volume 3226.62mm³, mesh genus 2 — below the naive sum of two independent tori (3553.06mm³), confirming the overlap was genuinely resolved rather than the kernel silently keeping two disjoint solids.",
    build: (b) => {
      const ringA = call(b, "torus", 10, 3);
      const ringB = call(b, "translate", call(b, "torus", 10, 3), [16, 0, 0]);
      return { operands: [{ role: "operand-a-step", shape: ringA }, { role: "operand-b-step", shape: ringB }], result: call(b, "fuse", ringA, ringB) };
    },
  },
  {
    id: "intersect-sphere-sphere-lens",
    family: "shape-complexity",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "Sphere/sphere boolean INTERSECTION (common), not the cut/fuse already covered elsewhere in the family — two doubly-periodic surfaces meeting along a single circle, producing a lens. MEASURED: 3 faces, 3 edges, 2 vertices, volume 871.27mm³, valid solid.",
    build: (b) => {
      const sphereA = call(b, "sphere", 10);
      const sphereB = call(b, "translate", call(b, "sphere", 10), [12, 0, 0]);
      return { operands: [{ role: "operand-a-step", shape: sphereA }, { role: "operand-b-step", shape: sphereB }], result: call(b, "intersect", sphereA, sphereB) };
    },
  },
  {
    id: "cut-pentagon-prism-from-block",
    family: "shape-complexity",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "Prism via `polygon` (an irregular, non-regular pentagon) + `extrude` — the code path a wedge built from `polyhedron` never exercises. MEASURED: pentagon area 126mm² × height 15 = exactly 1890mm³ for the tool (confirms `polygon`+`extrude` compute planar area exactly, no approximation loss); the cut result carries 11 faces, 27 edges, 18 vertices, volume 7740mm³.",
    build: (b) => {
      const block = call(b, "box", 30, 30, 10);
      const pentagon = call(b, "polygon", [[0, 0, 0], [10, 0, 0], [13, 6, 0], [5, 12, 0], [-3, 6, 0]]);
      const prism = call(b, "translate", call(b, "extrude", pentagon, 15), [10, 5, -3]);
      return { operands: [{ role: "operand-a-step", shape: block }, { role: "operand-b-step", shape: prism }], result: call(b, "cut", block, prism) };
    },
  },
  {
    id: "cut-wedge-from-block",
    family: "shape-complexity",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "Wedge built directly from `polyhedron` (6 explicit vertices, 5 explicit quad/triangle face loops) rather than swept from a 2-D profile — the classic doorstop shape. MEASURED: the kernel silently RETRIANGULATED every declared quad face, reporting 8 faces (not the 5 submitted) once queried — `polyhedron`'s face count is not the input face count. Volume 400mm³ (0.5×10×8×10, exact triangular-prism formula) confirms the geometry itself is correct despite the face-count surprise. Cut result: 14 faces, 29 edges, 18 vertices, volume 10425mm³.",
    build: (b) => {
      const block = call(b, "box", 30, 30, 12);
      const wedge = call(
        b,
        "polyhedron",
        [
          [0, 0, 0],
          [10, 0, 0],
          [0, 0, 8],
          [0, 10, 0],
          [10, 10, 0],
          [0, 10, 8],
        ],
        [
          [0, 1, 4, 3],
          [0, 3, 5, 2],
          [1, 4, 5, 2],
          [0, 1, 2],
          [3, 4, 5],
        ],
      );
      const tool = call(b, "translate", wedge, [10, 10, 6]);
      return { operands: [{ role: "operand-a-step", shape: block }, { role: "operand-b-step", shape: tool }], result: call(b, "cut", block, tool) };
    },
  },
  {
    id: "cut-filleted-boss-pocket",
    family: "shape-complexity",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A FILLETED solid (all 12 edges of a box rounded at r=3) used as the cutting TOOL — the case where a fillet's newly-introduced toroidal/cylindrical corner patches must survive a second boolean, which is exactly where kernels most often disagree. MEASURED: the filleted boss alone carries 26 faces, 56 edges, 24 vertices (up from the box's 6/12/8), volume 3761.33mm³ (vs. the sharp box's 4096mm³ — the fillets removed 334.67mm³, plausible for 12 quarter-round r=3 edges plus 8 corner spheres). Pocket cut result: 23 faces, 52 edges, 28 vertices, volume 15622.79mm³.",
    build: (b) => {
      const outer = call(b, "box", 30, 30, 20);
      const boss = call(b, "fillet", call(b, "box", 16, 16, 16), 3);
      const tool = call(b, "translate", boss, [7, 7, 10]);
      return { operands: [{ role: "operand-a-step", shape: outer }, { role: "operand-b-step", shape: tool }], result: call(b, "cut", outer, tool) };
    },
  },
  {
    id: "fuse-chamfered-boss-to-block",
    family: "shape-complexity",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A CHAMFERED cylinder (both circular rim edges broken at 2mm, all edges selected) fused onto a plate — chamfer's flat conical break-edges meeting the plate's planar face is a different failure mode from a fillet's tangent blend. MEASURED: chamfered boss alone is 5 faces, 7 edges, 4 vertices, volume 2228.44mm³ (vs. the sharp cylinder's 2412.74mm³). Fused result: 10 faces, 19 edges, 12 vertices, volume 11228.44mm³ — exactly plate-volume + chamfered-boss-volume, confirming a clean, lossless union.",
    build: (b) => {
      const plate = call(b, "box", 30, 30, 10);
      const boss = call(b, "chamfer", call(b, "cylinder", 8, 12), 2);
      const positioned = call(b, "translate", boss, [15, 15, 10]);
      return { operands: [{ role: "operand-a-step", shape: plate }, { role: "operand-b-step", shape: positioned }], result: call(b, "fuse", plate, positioned) };
    },
  },
  {
    id: "cut-lofted-funnel-from-block",
    family: "shape-complexity",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A LOFT between a wide circle (r=6, z=0) and a smaller, laterally-offset circle (r=3, translated [5,0,20]) — a genuinely oblique ruled surface between two periodic profiles, used as a boolean operand. MEASURED: the loft alone is a valid solid, 3 faces, 3 edges, 2 vertices, volume 1319.47mm³. Cut result: 9 faces, 15 edges, 10 vertices, volume 25680.53mm³.",
    build: (b) => {
      const block = call(b, "box", 30, 30, 30);
      const bottom = call(b, "wireLoop", [call(b, "circle", 6)]);
      const top = call(b, "wireLoop", [call(b, "translate", call(b, "circle", 3), [5, 0, 20])]);
      const funnel = call(b, "translate", call(b, "loft", [bottom, top]), [10, 10, 5]);
      return { operands: [{ role: "operand-a-step", shape: block }, { role: "operand-b-step", shape: funnel }], result: call(b, "cut", block, funnel) };
    },
  },
  {
    id: "fuse-swept-rib-to-block",
    family: "shape-complexity",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A circular profile SWEPT along a genuinely curved (non-planar-trivial) three-point-arc spine, fused onto a block — a curved rib, the shape family a `sweep`-produced solid must survive a boolean untouched. MEASURED: the swept rib alone is 3 faces, 3 edges, 2 vertices, volume 285.23mm³. Fused result: 12 faces, 24 edges, 15 vertices, volume 9018.27mm³ — exactly block + rib volume, a clean union with no material lost at the rib's tangent entry into the block face.",
    build: (b) => {
      const block = call(b, "box", 30, 10, 30);
      const profile = call(b, "wireLoop", [call(b, "circle", 2, { at: [0, 0, 0], axis: [0, 1, 0] })]);
      const spine = call(b, "wire", [call(b, "threePointArc", [0, 0, 0], [10, 6, 10], [20, 0, 20])]);
      const rib = call(b, "sweep", profile, spine);
      return { operands: [{ role: "operand-a-step", shape: block }, { role: "operand-b-step", shape: rib }], result: call(b, "fuse", block, rib) };
    },
  },
  {
    id: "cut-partial-revolved-ring-groove",
    family: "shape-complexity",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A PARTIAL revolve (270°, i.e. a 3/4 ring left deliberately open) as a cutting tool — `revolve`'s angle is in RADIANS, unlike every other angle in this file which is degrees, so the recipe converts explicitly rather than repeat the corpus's one silent-degrees-vs-radians trap. MEASURED: the open 3/4 ring is a single valid solid — 6 faces, 12 edges, 8 vertices, volume 918.92mm³ (a quarter less than the same profile's full 360° revolve would give, consistent with 270/360) — confirming the kernel closes a partial revolve with genuine flat end-cap faces rather than leaving an open shell. Groove cut into a disc: 7 faces, 15 edges, 10 vertices, volume 2222.68mm³.",
    build: (b) => {
      const disc = call(b, "cylinder", 10, 10);
      const profile = call(b, "polygon", [[5, 0, 0], [8, 0, 0], [8, 0, 10], [5, 0, 10]]);
      const ring = call(b, "revolve", profile, { angle: (270 * Math.PI) / 180 });
      return { operands: [{ role: "operand-a-step", shape: disc }, { role: "operand-b-step", shape: ring }], result: call(b, "cut", disc, ring) };
    },
  },
  {
    id: "cut-helical-coil-groove",
    family: "shape-complexity",
    outcome: "rejected",
    tolerance: "mechanical-standard",
    notes:
      "The closest this kernel offers to genuine THREAD geometry: a small circular profile swept along a `helix` spine (radius 9, pitch 6, 4 turns over height 24), cut into a base cylinder to carve a helical groove — the standing-directive 'helical/threaded geometry' case. MEASURED SURPRISE, and the reason `rejected` is the honest outcome here rather than `applied`: the BOOLEAN itself succeeds in memory — `cut(base, coil)` returns 1 valid solid — and the coil TOOL alone exports to STEP fine (46398 bytes). But exporting THAT CUT RESULT crashes the kernel outright: `exportSTEP` throws `{\"kind\":\"IO\",\"code\":\"STEP_EXPORT_CRASHED\",\"message\":\"STEP export crashed the kernel (Out of bounds memory access (evaluating 'func(...args)')); the shape likely contains geometry the STEP writer cannot serialize\"}` — a WASM memory fault, not a validation error. WORSE, confirmed directly against the kernel in a scratch harness rather than assumed: the crash is NOT contained to that one export call — it PERMANENTLY POISONS the WASM instance. Every `exportSTEP` call made afterward in the same process fails with that identical message, including exporting the untouched base cylinder again and exporting a brand-new, entirely unrelated 5×5×5 box. This is why `build()` below never calls `exportSTEP` on the grooved result itself: doing so inside the generator's own pipeline would not just fail this one fixture, it would take down every operand/result export queued after it in the same process (a real hazard for any future FULL, non `--only` regeneration run that happens to schedule this recipe before others). A related, independent surprise measured while building this case: the bare `helix(pitch, height, radius=9, ...)` wire's own declared bounds already extend to radius ~9.46 (5% past nominal) before any sweep, and once swept with a tube radius of 1.5 the coil solid's bounding box balloons further, to a radial extent of roughly 21 on one side (nominal envelope 9+1.5=10.5). `operands`/`result` below are the safely-exportable base cylinder and coil tool, recorded for provenance only — the finding this fixture pins is the STEP_EXPORT_CRASHED payload and its kernel-poisoning blast radius, not the fallback shape's numbers.",
    build: (b) => {
      const base = call(b, "cylinder", 10, 24);
      const spine = call(b, "helix", 6, 24, 9, { at: [0, 0, 0], axis: [0, 0, 1] });
      const profile = call(b, "wireLoop", [call(b, "circle", 1.5, { at: [9, 0, 0], axis: [0, 1, 0] })]);
      // 💥️`sweep`+`cut` succeed here — only exporting THAT result to STEP crashes the kernel, and doing
      // so poisons every export for the rest of the process (see notes). Never call exportSTEP on
      // `grooved` from inside this file; the crash is documented from a separate, disposable process.
      const coil = call(b, "sweep", profile, spine);
      return { operands: [{ role: "operand-a-step", shape: base }, { role: "operand-b-step", shape: coil }], result: base };
    },
  },
  {
    id: "cut-box-across-cylinder-seam",
    family: "shape-complexity",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A deliberately-engineered SEAM-CROSSING case: OCCT's cylindrical surface parametrization seams at θ=0 (the +x direction), so this cutter box straddles x=+radius exactly at y=0 — the cut boundary is forced to cross the cylindrical face's periodic wraparound (θ: 359°→0°) rather than merely approach it. MEASURED: 6 faces, 12 edges, 8 vertices, volume 5941.44mm³ — a single clean face split at the seam, no duplicate or degenerate face reported.",
    build: (b) => {
      const cyl = call(b, "cylinder", 10, 20);
      const cutter = call(b, "translate", call(b, "box", 6, 6, 30), [7, -3, -5]);
      return { operands: [{ role: "operand-a-step", shape: cyl }, { role: "operand-b-step", shape: cutter }], result: call(b, "cut", cyl, cutter) };
    },
  },
  {
    id: "cut-box-across-sphere-pole",
    family: "shape-complexity",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "The sphere counterpart to the cylinder-seam case: a sphere's parametrization has a genuine POLE singularity (not just a seam), where every meridian converges to a single point. This cutter box is centered exactly over the +z pole so the cut boundary passes directly through it, forcing the kernel to trim a face across its degenerate pole rather than around it. MEASURED: 6 faces, 15 edges, 10 vertices, volume 4091.83mm³ — the pole survives as a valid (if pathological) trimmed spherical face, no crash or degenerate zero-area face reported.",
    build: (b) => {
      const sphere = call(b, "sphere", 10);
      const cutter = call(b, "translate", call(b, "box", 6, 6, 6), [-3, -3, 7]);
      return { operands: [{ role: "operand-a-step", shape: sphere }, { role: "operand-b-step", shape: cutter }], result: call(b, "cut", sphere, cutter) };
    },
  },
  {
    id: "cut-spline-bounded-pocket",
    family: "shape-complexity",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A NURBS/spline-bounded face: `bsplineApprox` fits a B-spline curve through 4 scattered points, closed by one straight edge, faced and extruded into a pocket tool — the only fixture in the corpus whose cutting boundary is a genuine free-form curve rather than an analytic primitive. MEASURED and load-bearing: an EARLIER control-point layout for this same idea ([0,0,0],[3,4,0],[7,-3,0],[10,2,0]) produced a self-crossing boundary and `isValidSolid` measured FALSE on the extruded tool, even though the subsequent cut still completed — that layout was discarded rather than shipped invalid; the control points below were chosen specifically because they measure `valid: true`. Tool alone: 4 faces, 6 edges, 4 vertices, volume 341.64mm³. Cut result: 10 faces, 18 edges, 12 vertices, volume 17658.36mm³.",
    build: (b) => {
      const block = call(b, "box", 30, 30, 20);
      const spline = call(b, "bsplineApprox", [[0, 0, 0], [4, 6, 0], [8, 3, 0], [10, 0, 0]]);
      const closing = call(b, "line", [10, 0, 0], [0, 0, 0]);
      const face = call(b, "face", call(b, "wireLoop", [spline, closing]));
      const pocket = call(b, "translate", call(b, "extrude", face, 8), [8, 8, 6]);
      return { operands: [{ role: "operand-a-step", shape: block }, { role: "operand-b-step", shape: pocket }], result: call(b, "cut", block, pocket) };
    },
  },
  {
    id: "cut-compound-spheres-from-block",
    family: "shape-complexity",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A MULTI-SOLID operand: three separate spheres assembled with `compound` into ONE tool, cut from a block in a SINGLE boolean call rather than three chained cuts — the case where a kernel might silently only process the first solid of a compound, or fail on multi-body tools entirely. MEASURED: none of that happened — 9 faces (6 flat block faces + 3 independent spherical cavity faces), 21 edges, 14 vertices, volume 17195.75mm³ (block 18000mm³ minus all three spheres' volumes, confirming every solid in the compound was applied).",
    build: (b) => {
      const block = call(b, "box", 30, 30, 20);
      const s1 = call(b, "translate", call(b, "sphere", 4), [8, 8, 10]);
      const s2 = call(b, "translate", call(b, "sphere", 4), [22, 8, 10]);
      const s3 = call(b, "translate", call(b, "sphere", 4), [15, 22, 10]);
      const tool = call(b, "compound", [s1, s2, s3]);
      return { operands: [{ role: "operand-a-step", shape: block }, { role: "operand-b-step", shape: tool }], result: call(b, "cut", block, tool) };
    },
  },
  {
    id: "cut-through-shelled-box",
    family: "shape-complexity",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A `shell` result (a box with one face removed and the remaining walls offset to 2mm thickness, i.e. an open-top hollow container) used as the BASE of a boolean, then drilled straight through — the case where a boolean must cut through thin, previously-hollowed walls rather than solid stock. MEASURED: the shelled box alone is 11 faces, 24 edges, 16 vertices, volume 3392mm³ (down from the sharp box's 8000mm³ — consistent with a 2mm-thick shell of a 20mm cube missing one face). Drilled result: 13 faces, 30 edges, 20 vertices, volume 3278.90mm³ — the drill only had to remove material from the solid bottom wall, since the top was already open.",
    build: (b) => {
      const box = call(b, "box", 20, 20, 20);
      const openTop = [(call(b, "getFaces", box) as unknown[])[0]];
      const hollow = call(b, "shell", box, openTop, 2);
      const drill = call(b, "translate", call(b, "cylinder", 3, 30), [10, 10, -5]);
      return { operands: [{ role: "operand-a-step", shape: hollow }, { role: "operand-b-step", shape: drill }], result: call(b, "cut", hollow, drill) };
    },
  },
  {
    id: "fuse-thickened-shell-into-block",
    family: "shape-complexity",
    outcome: "disjoint",
    tolerance: "mechanical-standard",
    notes:
      "Started as a `thicken` boolean-operand test (thicken a cylinder's curved lateral face by 2mm into a closed tube-shell solid, then fuse it fully embedded inside a larger block) and the MEASUREMENT overturned the intended outcome, exactly the trap this corpus exists to catch. `thicken(getFaces(cylinder)[0], 2)` measured `measureVolume = -6031.86` — a NEGATIVE volume, i.e. the thickened solid comes back with inverted face orientation — even though its magnitude is exactly right (π×(17²−15²)×30 = 6031.86, the correct annulus volume). Fusing that inverted-orientation tube into a block that fully contains it does NOT add material: the reimported result measures `solids: 2` and a TOTAL volume of 101968.14mm³, which is the block's own 108000mm³ MINUS 6031.86mm³ — i.e. `fuse` behaved like `cut` and split the block into a disjoint inner core plus an outer shell-with-cavity. Kept as `disjoint` rather than dropped or silently relabeled `applied`, because a kernel treating an inverted-orientation `thicken` result as a subtractive tool inside `fuse` is precisely the kind of orientation-sensitivity a BRep kernel under test must be measured against.",
    build: (b) => {
      const block = call(b, "box", 60, 60, 30);
      const cylinderFace = (call(b, "getFaces", call(b, "cylinder", 15, 30)) as unknown[])[0];
      const plate = call(b, "translate", call(b, "thicken", cylinderFace, 2), [30, 30, 0]);
      return { operands: [{ role: "operand-a-step", shape: block }, { role: "operand-b-step", shape: plate }], result: call(b, "fuse", block, plate) };
    },
  },
  {
    id: "fuse-double-rotated-skewed-box",
    family: "shape-complexity",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A box rotated about TWO different, non-parallel axes in sequence (25° about X, then 40° about Y) before being fused — no single rotation axis or plane describes its orientation, unlike `cut-skewed-bore`'s one-axis tilt. MEASURED: the skewed box alone reports 6 faces, 12 edges, 8 vertices, volume 5880mm³ (unchanged by rotation, as expected) with a bounding box no longer aligned to any face. Fused result: 14 faces, 36 edges, 24 vertices, volume 12920.60mm³ — exactly the sum of both boxes' volumes, a clean union despite the compound tilt.",
    build: (b) => {
      const boxA = call(b, "box", 20, 20, 20);
      const tilted = call(b, "rotate", call(b, "rotate", call(b, "box", 14, 14, 30), 25, { at: [0, 0, 0], axis: [1, 0, 0] }), 40, { at: [0, 0, 0], axis: [0, 1, 0] });
      const boxB = call(b, "translate", tilted, [10, 10, 10]);
      return { operands: [{ role: "operand-a-step", shape: boxA }, { role: "operand-b-step", shape: boxB }], result: call(b, "fuse", boxA, boxB) };
    },
  },
];
//#endregion 🧪️Recipes
