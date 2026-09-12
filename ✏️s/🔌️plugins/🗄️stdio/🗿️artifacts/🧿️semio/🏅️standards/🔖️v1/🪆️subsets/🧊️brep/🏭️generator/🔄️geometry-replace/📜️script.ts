#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// ➰️🗺️ BRep fixture recipes — the `geometry-replace` family: `replace-curve` (an edge's underlying
// curve changes while its two endpoint vertices stay put) and `replace-surface` (a face's underlying
// surface changes while its boundary wire stays put).
//
// brepjs has no in-place "swap this edge's curve" call — every pair below instead builds BEFORE and
// AFTER as two independently-constructed, kernel-valid shapes that share the same boundary geometry
// (same endpoints for a curve swap, the SAME wire OBJECT for most surface swaps) but differ in exactly
// the one property the mutation targets. `face()` (a planar-only analytic builder) vs `filledFace()`
// (a general surface-filling algorithm) turned out, MEASURED, to be the natural `replace-surface`
// pair: called on the identical planar wire, `face()` returns a `PLANE` and `filledFace()` returns a
// `BSPLINE_SURFACE` of the same area — a genuine surface-type swap with no geometric change at all.
//
// @see ../📜️script.ts — the generator that runs these

//#endregion 🧲️Header

//#region 🔌️Adapters
import { type Kernel, type Recipe, call } from "../📜️script.ts";
//#endregion 🔌️Adapters

//#region 🧪️Recipes
/** ➰️🗺️ The `geometry-replace` recipes: `replace-curve` then `replace-surface`. */
export const RECIPES: readonly Recipe[] = [
  //#region ➰replace-curve
  {
    id: "geometry-replace-curve-line-to-arc-square-small",
    family: "geometry-replace",
    kind: "replace-curve",
    outcome: "applied",
    tolerance: "geometry-tessellated",
    notes: "Mirrors this subset's own oracle scenario `replace-curve/swaps-the-first-edges-line-for-a-circular-arc`: BEFORE is a square whose first edge is a straight LINE; AFTER rebuilds the same closed loop with that edge replaced by a circular ARC bulging outward through the same two endpoints. Both endpoints and the other three edges are unchanged — only the one curve's type and the loop's enclosed area move.",
    build: (b) => {
      const p1: [number, number, number] = [0, 0, 0];
      const p2: [number, number, number] = [10, 0, 0];
      const p3: [number, number, number] = [10, 10, 0];
      const p4: [number, number, number] = [0, 10, 0];
      const before = call(b, "face", call(b, "wireLoop", [call(b, "line", p1, p2), call(b, "line", p2, p3), call(b, "line", p3, p4), call(b, "line", p4, p1)]));
      const arcMid: [number, number, number] = [5, -2, 0];
      const after = call(b, "filledFace", call(b, "wireLoop", [call(b, "threePointArc", p1, arcMid, p2), call(b, "line", p2, p3), call(b, "line", p3, p4), call(b, "line", p4, p1)]));
      return { operands: [{ role: "operand-a-step", shape: before }], result: after };
    },
  },
  {
    id: "geometry-replace-curve-line-to-arc-square-large",
    family: "geometry-replace",
    kind: "replace-curve",
    outcome: "applied",
    tolerance: "geometry-tessellated",
    notes: "The line-to-arc curve swap at 5e2 scale.",
    build: (b) => {
      const p1: [number, number, number] = [0, 0, 0];
      const p2: [number, number, number] = [500, 0, 0];
      const p3: [number, number, number] = [500, 500, 0];
      const p4: [number, number, number] = [0, 500, 0];
      const before = call(b, "face", call(b, "wireLoop", [call(b, "line", p1, p2), call(b, "line", p2, p3), call(b, "line", p3, p4), call(b, "line", p4, p1)]));
      const arcMid: [number, number, number] = [250, -100, 0];
      const after = call(b, "filledFace", call(b, "wireLoop", [call(b, "threePointArc", p1, arcMid, p2), call(b, "line", p2, p3), call(b, "line", p3, p4), call(b, "line", p4, p1)]));
      return { operands: [{ role: "operand-a-step", shape: before }], result: after };
    },
  },
  {
    id: "geometry-replace-curve-arc-to-spline-square",
    family: "geometry-replace",
    kind: "replace-curve",
    outcome: "applied",
    tolerance: "geometry-tessellated",
    notes: "The SECOND leg of the line → arc → spline progression: BEFORE is the arc-bulged square from `geometry-replace-curve-line-to-arc-square-small`; AFTER replaces that same edge again, this time with a B-spline (`bsplineApprox`) threaded through three points sharing the arc's endpoints and midpoint height but not its exact circular shape — a curve-TYPE change (`ARC → BSPLINE`) rather than a further shape change.",
    build: (b) => {
      const p1: [number, number, number] = [0, 0, 0];
      const p2: [number, number, number] = [10, 0, 0];
      const p3: [number, number, number] = [10, 10, 0];
      const p4: [number, number, number] = [0, 10, 0];
      const arcMid: [number, number, number] = [5, -2, 0];
      const before = call(b, "filledFace", call(b, "wireLoop", [call(b, "threePointArc", p1, arcMid, p2), call(b, "line", p2, p3), call(b, "line", p3, p4), call(b, "line", p4, p1)]));
      const splineEdge = call(b, "bsplineApprox", [p1, [2.5, -2.6, 0], arcMid, [7.5, -2.6, 0], p2]);
      const after = call(b, "filledFace", call(b, "wireLoop", [splineEdge, call(b, "line", p2, p3), call(b, "line", p3, p4), call(b, "line", p4, p1)]));
      return { operands: [{ role: "operand-a-step", shape: before }], result: after };
    },
  },
  {
    id: "geometry-replace-curve-line-to-spline-rectangle",
    family: "geometry-replace",
    kind: "replace-curve",
    outcome: "applied",
    tolerance: "geometry-tessellated",
    notes: "A direct LINE → BSPLINE swap (skipping the intermediate arc) on a non-square rectangle, exercising `replace-curve` against an aspect ratio the square fixtures don't.",
    build: (b) => {
      const p1: [number, number, number] = [0, 0, 0];
      const p2: [number, number, number] = [20, 0, 0];
      const p3: [number, number, number] = [20, 8, 0];
      const p4: [number, number, number] = [0, 8, 0];
      const before = call(b, "face", call(b, "wireLoop", [call(b, "line", p1, p2), call(b, "line", p2, p3), call(b, "line", p3, p4), call(b, "line", p4, p1)]));
      const splineEdge = call(b, "bsplineApprox", [p1, [7, -3, 0], [13, 2, 0], p2]);
      const after = call(b, "filledFace", call(b, "wireLoop", [splineEdge, call(b, "line", p2, p3), call(b, "line", p3, p4), call(b, "line", p4, p1)]));
      return { operands: [{ role: "operand-a-step", shape: before }], result: after };
    },
  },
  {
    id: "geometry-replace-curve-prism-line-to-arc-small",
    family: "geometry-replace",
    kind: "replace-curve",
    outcome: "applied",
    tolerance: "geometry-tessellated",
    notes: "`replace-curve` on an actual SOLID rather than a flat face: BEFORE extrudes a straight-edged quad profile into a prism; AFTER extrudes the SAME profile with one edge swapped for an arc into an equal-height prism. Volume and face count both move as a direct consequence of the one curve swap, which a flat-face fixture cannot demonstrate.",
    build: (b) => {
      const p1: [number, number, number] = [0, 0, 0];
      const p2: [number, number, number] = [10, 0, 0];
      const p3: [number, number, number] = [10, 10, 0];
      const p4: [number, number, number] = [0, 10, 0];
      const straightProfile = call(b, "polygon", [p1, p2, p3, p4]);
      const before = call(b, "extrude", straightProfile, [0, 0, 6]);
      const arcMid: [number, number, number] = [5, -2, 0];
      const arcProfile = call(b, "filledFace", call(b, "wireLoop", [call(b, "threePointArc", p1, arcMid, p2), call(b, "line", p2, p3), call(b, "line", p3, p4), call(b, "line", p4, p1)]));
      const after = call(b, "extrude", arcProfile, [0, 0, 6]);
      return { operands: [{ role: "operand-a-step", shape: before }], result: after };
    },
  },
  {
    id: "geometry-replace-curve-prism-line-to-arc-large",
    family: "geometry-replace",
    kind: "replace-curve",
    outcome: "applied",
    tolerance: "geometry-tessellated",
    notes: "The prism line-to-arc curve swap at 2e2 scale.",
    build: (b) => {
      const p1: [number, number, number] = [0, 0, 0];
      const p2: [number, number, number] = [200, 0, 0];
      const p3: [number, number, number] = [200, 200, 0];
      const p4: [number, number, number] = [0, 200, 0];
      const straightProfile = call(b, "polygon", [p1, p2, p3, p4]);
      const before = call(b, "extrude", straightProfile, [0, 0, 120]);
      const arcMid: [number, number, number] = [100, -40, 0];
      const arcProfile = call(b, "filledFace", call(b, "wireLoop", [call(b, "threePointArc", p1, arcMid, p2), call(b, "line", p2, p3), call(b, "line", p3, p4), call(b, "line", p4, p1)]));
      const after = call(b, "extrude", arcProfile, [0, 0, 120]);
      return { operands: [{ role: "operand-a-step", shape: before }], result: after };
    },
  },
  {
    id: "geometry-replace-curve-tangent-arc-fillet-like",
    family: "geometry-replace",
    kind: "replace-curve",
    outcome: "applied",
    tolerance: "geometry-tessellated",
    notes: "Uses `tangentArc` rather than `threePointArc` — the replacement curve is tangent to the PREVIOUS edge's direction at the shared vertex rather than merely passing through it, the smoother of the two ways this kernel can compute a replacement curve through the same endpoint.",
    build: (b) => {
      const p1: [number, number, number] = [0, 0, 0];
      const p2: [number, number, number] = [10, 0, 0];
      const p3: [number, number, number] = [10, 10, 0];
      const p4: [number, number, number] = [0, 10, 0];
      const before = call(b, "face", call(b, "wireLoop", [call(b, "line", p1, p2), call(b, "line", p2, p3), call(b, "line", p3, p4), call(b, "line", p4, p1)]));
      const tangentEdge = call(b, "tangentArc", p4, [1, 0, 0], p1);
      const after = call(b, "filledFace", call(b, "wireLoop", [call(b, "line", p1, p2), call(b, "line", p2, p3), call(b, "line", p3, p4), tangentEdge]));
      return { operands: [{ role: "operand-a-step", shape: before }], result: after };
    },
  },
  //#endregion ➰replace-curve

  //#region 🗺️replace-surface
  {
    id: "geometry-replace-surface-plane-to-bspline-square-small",
    family: "geometry-replace",
    kind: "replace-surface",
    outcome: "applied",
    tolerance: "geometry-tessellated",
    notes: "Mirrors this subset's own oracle scenario `replace-surface/swaps-the-faces-plane-for-a-cylinder` in spirit: the SAME closed wire is capped twice — `face()` (analytic planar builder) for BEFORE and `filledFace()` (general surface-filling algorithm) for AFTER. MEASURED: BEFORE's surface type is `PLANE`, AFTER's is `BSPLINE_SURFACE`, and their areas agree to floating-point noise (100 vs 99.99999999999993) — a pure surface-representation swap with no boundary change at all.",
    build: (b) => {
      const wire = call(b, "wireLoop", [call(b, "line", [0, 0, 0], [10, 0, 0]), call(b, "line", [10, 0, 0], [10, 10, 0]), call(b, "line", [10, 10, 0], [0, 10, 0]), call(b, "line", [0, 10, 0], [0, 0, 0])]);
      const before = call(b, "face", wire);
      const after = call(b, "filledFace", wire);
      return { operands: [{ role: "operand-a-step", shape: before }], result: after };
    },
  },
  {
    id: "geometry-replace-surface-plane-to-bspline-square-large",
    family: "geometry-replace",
    kind: "replace-surface",
    outcome: "applied",
    tolerance: "geometry-tessellated",
    notes: "The plane-to-bspline surface swap at 5e2 scale.",
    build: (b) => {
      const wire = call(b, "wireLoop", [call(b, "line", [0, 0, 0], [500, 0, 0]), call(b, "line", [500, 0, 0], [500, 500, 0]), call(b, "line", [500, 500, 0], [0, 500, 0]), call(b, "line", [0, 500, 0], [0, 0, 0])]);
      const before = call(b, "face", wire);
      const after = call(b, "filledFace", wire);
      return { operands: [{ role: "operand-a-step", shape: before }], result: after };
    },
  },
  {
    id: "geometry-replace-surface-plane-to-bspline-pentagon",
    family: "geometry-replace",
    kind: "replace-surface",
    outcome: "applied",
    tolerance: "geometry-tessellated",
    notes: "The plane-to-bspline surface swap against a five-sided boundary rather than a square, checking the pair isn't an artefact of exactly four edges.",
    build: (b) => {
      const pts: [number, number, number][] = [[0, 0, 0], [10, 0, 0], [13, 8, 0], [5, 13, 0], [-3, 8, 0]];
      const edges = pts.map((p, i) => call(b, "line", p, pts[(i + 1) % pts.length]!));
      const wire = call(b, "wireLoop", edges);
      const before = call(b, "face", wire);
      const after = call(b, "filledFace", wire);
      return { operands: [{ role: "operand-a-step", shape: before }], result: after };
    },
  },
  {
    id: "geometry-replace-surface-plane-to-bspline-triangle",
    family: "geometry-replace",
    kind: "replace-surface",
    outcome: "applied",
    tolerance: "geometry-tessellated",
    notes: "The plane-to-bspline surface swap against the simplest possible boundary — a triangle, always planar regardless of vertex position — isolating the surface swap from any boundary-planarity question.",
    build: (b) => {
      const p1: [number, number, number] = [0, 0, 0];
      const p2: [number, number, number] = [12, 0, 0];
      const p3: [number, number, number] = [6, 10, 0];
      const wire = call(b, "wireLoop", [call(b, "line", p1, p2), call(b, "line", p2, p3), call(b, "line", p3, p1)]);
      const before = call(b, "face", wire);
      const after = call(b, "filledFace", wire);
      return { operands: [{ role: "operand-a-step", shape: before }], result: after };
    },
  },
  {
    id: "geometry-replace-surface-disk-plane-to-general",
    family: "geometry-replace",
    kind: "replace-surface",
    outcome: "applied",
    tolerance: "geometry-tessellated",
    notes: "The plane-to-general surface swap against a CIRCULAR boundary (a single closed circular edge as its own wire) rather than a straight-edged polygon — whichever surface type `filledFace` measures for a circular disk, the pair still proves the same wire capped two ways.",
    build: (b) => {
      const circleEdge = call(b, "circle", 8);
      const wire = call(b, "wire", [circleEdge]);
      const before = call(b, "face", wire);
      const after = call(b, "filledFace", wire);
      return { operands: [{ role: "operand-a-step", shape: before }], result: after };
    },
  },
  {
    id: "geometry-replace-surface-cylindrical-strip-to-bspline-small",
    family: "geometry-replace",
    kind: "replace-surface",
    outcome: "applied",
    tolerance: "geometry-tessellated",
    notes: "A genuinely CURVED replace-surface pair: the boundary (two axial lines + two circular arcs) sits exactly on a cylinder's own lateral surface, since a cylinder is ruled along its axis. `subFace(cylinderLateralFace, wire)` — MEASURED — reuses that CYLINDRE surface for BEFORE; `filledFace(wire)` rebuilds the SAME wire as a BSPLINE_SURFACE for AFTER. Where the square/pentagon/triangle fixtures swap PLANE↔BSPLINE, this one swaps CYLINDRE↔BSPLINE.",
    build: (b) => {
      const radius = 6;
      const height = 10;
      const angle = Math.PI / 2;
      const bottomStart: [number, number, number] = [radius, 0, 0];
      const bottomEnd: [number, number, number] = [radius * Math.cos(angle), radius * Math.sin(angle), 0];
      const topStart: [number, number, number] = [radius, 0, height];
      const topEnd: [number, number, number] = [radius * Math.cos(angle), radius * Math.sin(angle), height];
      const wire = call(b, "wireLoop", [call(b, "line", bottomStart, topStart), call(b, "threePointArc", topStart, [radius * Math.cos(angle / 2), radius * Math.sin(angle / 2), height], topEnd), call(b, "line", topEnd, bottomEnd), call(b, "threePointArc", bottomEnd, [radius * Math.cos(angle / 2), radius * Math.sin(angle / 2), 0], bottomStart)]);
      const cylinder = call(b, "cylinder", radius, height);
      const cylinderFaces = call(b, "getFaces", cylinder) as unknown[];
      let lateral: unknown = null;
      for (const f of cylinderFaces) if (call(b, "getSurfaceType", f) === "CYLINDRE") lateral = f;
      const before = call(b, "subFace", lateral, wire);
      const after = call(b, "filledFace", wire);
      return { operands: [{ role: "operand-a-step", shape: before }], result: after };
    },
  },
  {
    id: "geometry-replace-surface-cylindrical-strip-to-bspline-large",
    family: "geometry-replace",
    kind: "replace-surface",
    outcome: "applied",
    tolerance: "geometry-tessellated",
    notes: "The cylindrical-strip-to-bspline surface swap at 5e1 scale (5e1× the small variant's 6mm radius).",
    build: (b) => {
      const radius = 300;
      const height = 500;
      const angle = Math.PI / 2;
      const bottomStart: [number, number, number] = [radius, 0, 0];
      const bottomEnd: [number, number, number] = [radius * Math.cos(angle), radius * Math.sin(angle), 0];
      const topStart: [number, number, number] = [radius, 0, height];
      const topEnd: [number, number, number] = [radius * Math.cos(angle), radius * Math.sin(angle), height];
      const wire = call(b, "wireLoop", [call(b, "line", bottomStart, topStart), call(b, "threePointArc", topStart, [radius * Math.cos(angle / 2), radius * Math.sin(angle / 2), height], topEnd), call(b, "line", topEnd, bottomEnd), call(b, "threePointArc", bottomEnd, [radius * Math.cos(angle / 2), radius * Math.sin(angle / 2), 0], bottomStart)]);
      const cylinder = call(b, "cylinder", radius, height);
      const cylinderFaces = call(b, "getFaces", cylinder) as unknown[];
      let lateral: unknown = null;
      for (const f of cylinderFaces) if (call(b, "getSurfaceType", f) === "CYLINDRE") lateral = f;
      const before = call(b, "subFace", lateral, wire);
      const after = call(b, "filledFace", wire);
      return { operands: [{ role: "operand-a-step", shape: before }], result: after };
    },
  },
  //#endregion 🗺️replace-surface
];
//#endregion 🧪️Recipes
