#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🧭️ BRep fixture recipes — the `spatial-relationship` family.
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
/** 🧭️ The `spatial-relationship` recipes. */
export const RECIPES: readonly Recipe[] = [
  {
    id: "cut-bored-box-through",
    family: "spatial-relationship",
    outcome: "applied",
    tolerance: "analytic-strict",
    notes: "Partial volumetric overlap: a cylinder bored clean through a box. The analytic answer 20³ − π·5²·20 is known in closed form, so this fixture also pins the kernel's own exactness.",
    build: (b) => {
      const box = call(b, "box", 20, 20, 20);
      const bore = call(b, "translate", call(b, "cylinder", 5, 40), [10, 10, -10]);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: bore }], result: call(b, "cut", box, bore) };
    },
  },
  {
    id: "cut-disjoint-operands",
    family: "spatial-relationship",
    outcome: "no-op",
    tolerance: "analytic-strict",
    notes: "Completely disjoint operands. The declared outcome is a NO-OP: subtracting something that touches nothing must return the base unchanged, not merely 'something that does not crash'.",
    build: (b) => {
      const box = call(b, "box", 10, 10, 10);
      const away = call(b, "translate", call(b, "box", 10, 10, 10), [100, 0, 0]);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: away }], result: call(b, "cut", box, away) };
    },
  },
  {
    id: "cut-contained-operand",
    family: "spatial-relationship",
    outcome: "applied",
    tolerance: "analytic-strict",
    notes: "One operand fully contained in the other, producing an internal cavity. A cavity is exactly what mesh similarity alone cannot see, which is why volume and validity are asserted beside it.",
    build: (b) => {
      const outer = call(b, "box", 20, 20, 20);
      const inner = call(b, "translate", call(b, "box", 6, 6, 6), [7, 7, 7]);
      return { operands: [{ role: "operand-a-step", shape: outer }, { role: "operand-b-step", shape: inner }], result: call(b, "cut", outer, inner) };
    },
  },
  {
    id: "cut-full-subtraction",
    family: "spatial-relationship",
    outcome: "empty",
    tolerance: "analytic-strict",
    notes: "The tool completely swallows the base. The declared outcome is EMPTY, so a kernel that returned the base unchanged would fail rather than pass quietly.",
    build: (b) => {
      const small = call(b, "translate", call(b, "box", 5, 5, 5), [5, 5, 5]);
      const large = call(b, "box", 20, 20, 20);
      return { operands: [{ role: "operand-a-step", shape: small }, { role: "operand-b-step", shape: large }], result: call(b, "cut", small, large) };
    },
  },
  {
    id: "fuse-face-touching-boxes",
    family: "spatial-relationship",
    outcome: "applied",
    tolerance: "contact-sensitive",
    notes: "Exact face contact. Two boxes sharing a whole face must fuse into ONE solid; a kernel that left two components would be caught by the component-count assertion, not by the volume, which is identical either way.",
    build: (b) => {
      const left = call(b, "box", 10, 10, 10);
      const right = call(b, "translate", call(b, "box", 10, 10, 10), [10, 0, 0]);
      return { operands: [{ role: "operand-a-step", shape: left }, { role: "operand-b-step", shape: right }], result: call(b, "fuse", left, right) };
    },
  },
  {
    id: "intersect-overlapping-boxes",
    family: "spatial-relationship",
    outcome: "applied",
    tolerance: "analytic-strict",
    notes: "Partial overlap intersection with the closed-form answer 5³. Supplies the A∩B term of the Boolean volume identity V(A∪B) + V(A∩B) = V(A) + V(B).",
    build: (b) => {
      const left = call(b, "box", 10, 10, 10);
      const right = call(b, "translate", call(b, "box", 10, 10, 10), [5, 5, 5]);
      return { operands: [{ role: "operand-a-step", shape: left }, { role: "operand-b-step", shape: right }], result: call(b, "intersect", left, right) };
    },
  },
  {
    id: "cut-disconnected-result",
    family: "spatial-relationship",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A cut that splits the base into TWO bodies. The component count is the assertion; volume alone cannot tell one body from two.",
    build: (b) => {
      const bar = call(b, "box", 40, 10, 10);
      const chop = call(b, "translate", call(b, "box", 6, 30, 30), [17, -10, -10]);
      return { operands: [{ role: "operand-a-step", shape: bar }, { role: "operand-b-step", shape: chop }], result: call(b, "cut", bar, chop) };
    },
  },

  // 🧭️ The 7 recipes above cover only SOME of the (arrangement × operation) matrix. Everything below
  // fills the remaining cells the ticket calls out by name, MEASURED against the actual re-imported
  // STEP the way every recipe in this file must be — see 📓️corpus-spatial-and-failure.md for the full
  // matrix table and every surprise the measurement turned up.

  {
    id: "fuse-disjoint-boxes",
    family: "spatial-relationship",
    outcome: "disjoint",
    tolerance: "analytic-strict",
    notes: "Completely disjoint operands, fused. MEASURED: two solids, total volume 2000 (1000+1000) — a boolean union of shapes that share no boundary stays disjoint rather than becoming one compound-but-still-two-bodies shape; component count is the only thing that would catch a kernel that merged them into a single bounding shell.",
    build: (b) => {
      const a = call(b, "box", 10, 10, 10);
      const away = call(b, "translate", call(b, "box", 10, 10, 10), [100, 0, 0]);
      return { operands: [{ role: "operand-a-step", shape: a }, { role: "operand-b-step", shape: away }], result: call(b, "fuse", a, away) };
    },
  },
  {
    id: "intersect-disjoint-boxes",
    family: "spatial-relationship",
    outcome: "empty",
    tolerance: "analytic-strict",
    notes: "Completely disjoint operands, intersected. MEASURED: zero solids — the intersect side of the same disjoint arrangement `fuse-disjoint-boxes` and `cut-disjoint-operands` already cover for fuse and cut.",
    build: (b) => {
      const a = call(b, "box", 10, 10, 10);
      const away = call(b, "translate", call(b, "box", 10, 10, 10), [100, 0, 0]);
      return { operands: [{ role: "operand-a-step", shape: a }, { role: "operand-b-step", shape: away }], result: call(b, "intersect", a, away) };
    },
  },
  {
    id: "fuse-contained-operand",
    family: "spatial-relationship",
    outcome: "no-op",
    tolerance: "analytic-strict",
    notes: "One operand fully contained in the other, FUSED. MEASURED: the result is byte-for-byte the same shape as the outer box alone — same volume (8000), same 6 faces / 12 edges / 8 vertices, no imprint at all. Declared NO-OP rather than applied: fusing a fully-swallowed shape changes nothing a re-importer could observe, unlike `cut-contained-operand`'s cavity.",
    build: (b) => {
      const outer = call(b, "box", 20, 20, 20);
      const inner = call(b, "translate", call(b, "box", 6, 6, 6), [7, 7, 7]);
      return { operands: [{ role: "operand-a-step", shape: outer }, { role: "operand-b-step", shape: inner }], result: call(b, "fuse", outer, inner) };
    },
  },
  {
    id: "intersect-contained-operand",
    family: "spatial-relationship",
    outcome: "applied",
    tolerance: "analytic-strict",
    notes: "One operand fully contained in the other, INTERSECTED. MEASURED: the result equals the inner box exactly — volume 216 (6³), 6 faces / 12 edges / 8 vertices, single solid. Complements `cut-contained-operand` (cavity) and `fuse-contained-operand` (no-op): the three operations give three genuinely different answers for the same pair of solids.",
    build: (b) => {
      const outer = call(b, "box", 20, 20, 20);
      const inner = call(b, "translate", call(b, "box", 6, 6, 6), [7, 7, 7]);
      return { operands: [{ role: "operand-a-step", shape: outer }, { role: "operand-b-step", shape: inner }], result: call(b, "intersect", outer, inner) };
    },
  },
  {
    id: "fuse-partial-overlap-boxes",
    family: "spatial-relationship",
    outcome: "applied",
    tolerance: "analytic-strict",
    notes: "Partial volumetric overlap, fused — the union term of V(A∪B)+V(A∩B)=V(A)+V(B) that `intersect-overlapping-boxes` already supplies the other half of. MEASURED: single solid, volume 1875 (1000+1000−125, matching the closed form).",
    build: (b) => {
      const left = call(b, "box", 10, 10, 10);
      const right = call(b, "translate", call(b, "box", 10, 10, 10), [5, 5, 5]);
      return { operands: [{ role: "operand-a-step", shape: left }, { role: "operand-b-step", shape: right }], result: call(b, "fuse", left, right) };
    },
  },
  {
    id: "cut-face-touching-boxes",
    family: "spatial-relationship",
    outcome: "no-op",
    tolerance: "contact-sensitive",
    notes: "Two boxes flush against a shared FULL face (zero gap, zero overlap), CUT. MEASURED: the result is byte-identical to the untouched base — volume 1000, 6 faces / 12 edges / 8 vertices, no imprint whatsoever. This is the cut sibling of `fuse-face-touching-boxes`; unlike the tangent-cylinder rung in `robustness`, a flat face-on-face touch leaves no trace at all, not even a face split.",
    build: (b) => {
      const left = call(b, "box", 10, 10, 10);
      const right = call(b, "translate", call(b, "box", 10, 10, 10), [10, 0, 0]);
      return { operands: [{ role: "operand-a-step", shape: left }, { role: "operand-b-step", shape: right }], result: call(b, "cut", left, right) };
    },
  },
  {
    id: "intersect-face-touching-boxes",
    family: "spatial-relationship",
    outcome: "empty",
    tolerance: "contact-sensitive",
    notes: "Two boxes flush against a shared FULL face, INTERSECTED. MEASURED: zero solids — a shared boundary face has zero volume, so the declared outcome is EMPTY even though the two boxes share an entire face's worth of coincident geometry.",
    build: (b) => {
      const left = call(b, "box", 10, 10, 10);
      const right = call(b, "translate", call(b, "box", 10, 10, 10), [10, 0, 0]);
      return { operands: [{ role: "operand-a-step", shape: left }, { role: "operand-b-step", shape: right }], result: call(b, "intersect", left, right) };
    },
  },
  {
    id: "cut-edge-touching-boxes",
    family: "spatial-relationship",
    outcome: "no-op",
    tolerance: "contact-sensitive",
    notes: "Edge contact only (diagonal placement), CUT. MEASURED: byte-identical to the untouched base — volume 1000, 6 faces / 12 edges / 8 vertices. Complements `robustness`'s `fuse-edge-touching-boxes` (disjoint) with the cut side of the same arrangement.",
    build: (b) => {
      const a = call(b, "box", 10, 10, 10);
      const diagonal = call(b, "translate", call(b, "box", 10, 10, 10), [10, 10, 0]);
      return { operands: [{ role: "operand-a-step", shape: a }, { role: "operand-b-step", shape: diagonal }], result: call(b, "cut", a, diagonal) };
    },
  },
  {
    id: "intersect-edge-touching-boxes",
    family: "spatial-relationship",
    outcome: "empty",
    tolerance: "contact-sensitive",
    notes: "Edge contact only (diagonal placement), INTERSECTED. MEASURED: zero solids — a shared edge has zero volume.",
    build: (b) => {
      const a = call(b, "box", 10, 10, 10);
      const diagonal = call(b, "translate", call(b, "box", 10, 10, 10), [10, 10, 0]);
      return { operands: [{ role: "operand-a-step", shape: a }, { role: "operand-b-step", shape: diagonal }], result: call(b, "intersect", a, diagonal) };
    },
  },
  {
    id: "cut-vertex-touching-boxes",
    family: "spatial-relationship",
    outcome: "no-op",
    tolerance: "contact-sensitive",
    notes: "Two boxes sharing only a single corner point, CUT. MEASURED: byte-identical to the untouched base — volume 1000, 6 faces / 12 edges / 8 vertices. A single shared point is even less than a shared edge, and still imprints nothing.",
    build: (b) => {
      const a = call(b, "box", 10, 10, 10);
      const corner = call(b, "translate", call(b, "box", 10, 10, 10), [10, 10, 10]);
      return { operands: [{ role: "operand-a-step", shape: a }, { role: "operand-b-step", shape: corner }], result: call(b, "cut", a, corner) };
    },
  },
  {
    id: "fuse-vertex-touching-boxes",
    family: "spatial-relationship",
    outcome: "disjoint",
    tolerance: "contact-sensitive",
    notes: "Two boxes sharing only a single corner point, FUSED. MEASURED (re-imported): TWO solids survive (volume 2000 = 1000+1000, 12 faces / 24 edges / 16 vertices — exactly 2×6/2×12/2×8, i.e. no sharing at all), so the declared class is DISJOINT — same convention as `robustness`'s edge-touching case, one contact dimension lower. The IN-MEMORY kernel result tells a different, more interesting story before export: it WELDS the two coincident corner points into a single shared vertex (15, not 16) even while leaving the two bodies unmerged — but that weld does not survive the STEP round-trip: re-imported, the two solids each keep their own vertex at the same coincident point, exactly the same in-memory-vs-reimport gap the generator's own `reimport` doc records for `fuse-edge-touching-boxes`. The committed metrics are the reimported 16, per the pipeline's own rule to measure what was written.",
    build: (b) => {
      const a = call(b, "box", 10, 10, 10);
      const corner = call(b, "translate", call(b, "box", 10, 10, 10), [10, 10, 10]);
      return { operands: [{ role: "operand-a-step", shape: a }, { role: "operand-b-step", shape: corner }], result: call(b, "fuse", a, corner) };
    },
  },
  {
    id: "intersect-vertex-touching-boxes",
    family: "spatial-relationship",
    outcome: "empty",
    tolerance: "contact-sensitive",
    notes: "Two boxes sharing only a single corner point, INTERSECTED. MEASURED: zero solids — a single shared point has zero volume and the kernel reports true emptiness rather than a degenerate point-shape.",
    build: (b) => {
      const a = call(b, "box", 10, 10, 10);
      const corner = call(b, "translate", call(b, "box", 10, 10, 10), [10, 10, 10]);
      return { operands: [{ role: "operand-a-step", shape: a }, { role: "operand-b-step", shape: corner }], result: call(b, "intersect", a, corner) };
    },
  },
  {
    id: "cut-tangential-sphere-contact",
    family: "spatial-relationship",
    outcome: "no-op",
    tolerance: "contact-sensitive",
    notes: "A sphere tangent to a box's top face at a single POINT (as opposed to `robustness`'s cylinder tangent along a LINE), cut from the box. SURPRISE, and the inverse of `fuse-vertex-touching-boxes`'s finding: the RAW in-memory kernel result does gain an extra vertex at the tangent point (9 instead of the plain box's 8, with faces/edges unchanged at 6/12) — but that lone imprinted vertex does NOT survive the STEP export/reimport round trip. The committed, re-imported `expected.step` measures 6 faces / 12 edges / 8 vertices and volume 8000 — byte-identical to the untouched box, exactly like `cut-vertex-touching-boxes`. Declared NO-OP per the pipeline's own rule (measure what was WRITTEN, not the in-memory shape); the in-memory-only imprint is recorded here because `cut-tangent-cylinder-exact`'s LINE tangency imprint (6→7 faces) does survive reimport, and the difference between a surviving line imprint and a vanishing point imprint is itself the finding.",
    build: (b) => {
      const box = call(b, "box", 20, 20, 20);
      const ball = call(b, "translate", call(b, "sphere", 5), [10, 10, 25]);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: ball }], result: call(b, "cut", box, ball) };
    },
  },
  {
    id: "fuse-tangential-sphere-contact",
    family: "spatial-relationship",
    outcome: "disjoint",
    tolerance: "contact-sensitive",
    notes: "The same point-tangent sphere, FUSED to the box. MEASURED: two solids (volume 8000 + 523.6 sphere ≈ 8523.6), so — like edge- and vertex-touching — a single-point tangency does not merge the two bodies into one.",
    build: (b) => {
      const box = call(b, "box", 20, 20, 20);
      const ball = call(b, "translate", call(b, "sphere", 5), [10, 10, 25]);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: ball }], result: call(b, "fuse", box, ball) };
    },
  },
  {
    id: "intersect-tangential-sphere-contact",
    family: "spatial-relationship",
    outcome: "empty",
    tolerance: "contact-sensitive",
    notes: "The same point-tangent sphere, INTERSECTED with the box. MEASURED: zero solids — a point of tangency has zero volume.",
    build: (b) => {
      const box = call(b, "box", 20, 20, 20);
      const ball = call(b, "translate", call(b, "sphere", 5), [10, 10, 25]);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: ball }], result: call(b, "intersect", box, ball) };
    },
  },
  {
    id: "cut-coincident-faces",
    family: "spatial-relationship",
    outcome: "applied",
    tolerance: "contact-sensitive",
    notes: "Two boxes with the SAME x/y footprint stacked so all four side faces are pairwise coincident planes, overlapping halfway in z (A: z∈[0,10], B: z∈[5,15]) — distinct from `face-touching` (zero-overlap flush adjacency): here the coincident planes carry a genuine shared interior, not just a shared boundary. CUT (A−B). MEASURED: a clean 6-face / 12-edge / 8-vertex box of volume 500 (the untouched lower half, z∈[0,5]) — the coincident side planes cause no extra faces here.",
    build: (b) => {
      const a = call(b, "box", 10, 10, 10);
      const stacked = call(b, "translate", call(b, "box", 10, 10, 10), [0, 0, 5]);
      return { operands: [{ role: "operand-a-step", shape: a }, { role: "operand-b-step", shape: stacked }], result: call(b, "cut", a, stacked) };
    },
  },
  {
    id: "fuse-coincident-faces",
    family: "spatial-relationship",
    outcome: "applied",
    tolerance: "contact-sensitive",
    notes: "The same coincident-side-plane boxes, FUSED. Geometrically the union is a plain 10×10×15 box with no re-entrant feature at all. MEASURED: volume 1500 is exactly right, but the shape carries 14 faces / 28 edges / 16 vertices, NOT the 6 / 12 / 8 a simplified box would have — `fuse` does not auto-merge the coincident-but-only-partially-overlapping side faces into single planar faces (there is a separate `simplify()` in the API for that, and this recipe deliberately does not call it). This is the sharpest surprise in the family: a boolean answer that is analytically a plain box is topologically nothing like one.",
    build: (b) => {
      const a = call(b, "box", 10, 10, 10);
      const stacked = call(b, "translate", call(b, "box", 10, 10, 10), [0, 0, 5]);
      return { operands: [{ role: "operand-a-step", shape: a }, { role: "operand-b-step", shape: stacked }], result: call(b, "fuse", a, stacked) };
    },
  },
  {
    id: "intersect-coincident-faces",
    family: "spatial-relationship",
    outcome: "applied",
    tolerance: "contact-sensitive",
    notes: "The same coincident-side-plane boxes, INTERSECTED. MEASURED: a clean 6-face / 12-edge / 8-vertex box of volume 500 (the shared middle slab, z∈[5,10]) — unlike `fuse-coincident-faces`, intersect DOES land on the minimal topology here, so the two operations disagree on how much to simplify the identical coincident planes.",
    build: (b) => {
      const a = call(b, "box", 10, 10, 10);
      const stacked = call(b, "translate", call(b, "box", 10, 10, 10), [0, 0, 5]);
      return { operands: [{ role: "operand-a-step", shape: a }, { role: "operand-b-step", shape: stacked }], result: call(b, "intersect", a, stacked) };
    },
  },
  {
    id: "fuse-coplanar-cutter-boxes",
    family: "spatial-relationship",
    outcome: "applied",
    tolerance: "contact-sensitive",
    notes: "Coplanar cutters, fuse side: a wider, shorter slab whose BOTTOM face lies exactly on the tall box's bottom face plane (`robustness`'s `cut-coplanar-face-cutter` supplies the cut side of this same arrangement). MEASURED: single solid, volume 13000 (8000+9000−4000, the analytic union).",
    build: (b) => {
      const box = call(b, "box", 20, 20, 20);
      const slab = call(b, "translate", call(b, "box", 30, 30, 10), [-5, -5, 0]);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: slab }], result: call(b, "fuse", box, slab) };
    },
  },
  {
    id: "intersect-coplanar-cutter-boxes",
    family: "spatial-relationship",
    outcome: "applied",
    tolerance: "contact-sensitive",
    notes: "The same coplanar-bottom-face arrangement, INTERSECTED. MEASURED: a clean 6-face / 12-edge / 8-vertex box of volume 4000 — the slab fully covers the tall box's footprint for z∈[0,10], so the intersection is exactly that lower half.",
    build: (b) => {
      const box = call(b, "box", 20, 20, 20);
      const slab = call(b, "translate", call(b, "box", 30, 30, 10), [-5, -5, 0]);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: slab }], result: call(b, "intersect", box, slab) };
    },
  },
  {
    id: "fuse-identical-operands",
    family: "spatial-relationship",
    outcome: "applied",
    tolerance: "contact-sensitive",
    notes: "Two independently-constructed but geometrically identical boxes, FUSED — the component count is the whole assertion, since a duplicate-detection bug could just as easily double an operand into two exactly-coincident solids or corrupt the shared-face topology. MEASURED: a single solid, volume 1728 (12³), a clean 6 faces / 12 edges / 8 vertices — indistinguishable from either operand alone.",
    build: (b) => {
      const a = call(b, "box", 12, 12, 12);
      const same = call(b, "box", 12, 12, 12);
      return { operands: [{ role: "operand-a-step", shape: a }, { role: "operand-b-step", shape: same }], result: call(b, "fuse", a, same) };
    },
  },
  {
    id: "intersect-identical-operands",
    family: "spatial-relationship",
    outcome: "applied",
    tolerance: "contact-sensitive",
    notes: "Two independently-constructed but geometrically identical boxes, INTERSECTED (`failure`'s `cut-identical-operands` already covers the cut side, which is empty by construction — A−A). MEASURED: a single solid, volume 1728, 6 faces / 12 edges / 8 vertices. The component count of exactly 1 is the assertion the task calls out explicitly: volume alone cannot tell 'one coincident solid' from 'two exactly-overlapping solids nobody merged.'",
    build: (b) => {
      const a = call(b, "box", 12, 12, 12);
      const same = call(b, "box", 12, 12, 12);
      return { operands: [{ role: "operand-a-step", shape: a }, { role: "operand-b-step", shape: same }], result: call(b, "intersect", a, same) };
    },
  },
  {
    id: "cut-nearly-identical-operands",
    family: "spatial-relationship",
    outcome: "applied",
    tolerance: "epsilon-degenerate",
    notes: "Two corner-aligned boxes differing by 0.001 mm in one dimension (12 vs 11.999) — a NEAR duplicate rather than an exact one. CUT leaves a paper-thin sliver. MEASURED: volume 0.144 (0.001×12×12), the sliver a naive tolerance sized in millimetres would swallow whole.",
    build: (b) => {
      const a = call(b, "box", 12, 12, 12);
      const nearly = call(b, "box", 11.999, 12, 12);
      return { operands: [{ role: "operand-a-step", shape: a }, { role: "operand-b-step", shape: nearly }], result: call(b, "cut", a, nearly) };
    },
  },
  {
    id: "fuse-nearly-identical-operands",
    family: "spatial-relationship",
    outcome: "applied",
    tolerance: "epsilon-degenerate",
    notes: "The same near-duplicate pair, FUSED. The smaller box is a corner-aligned subset of the larger one, so the union's VOLUME matches the larger operand exactly (1728 = 12³) — but unlike `fuse-contained-operand`'s clean 6/12/8, this one measures 10 faces / 20 edges / 12 vertices. The smaller box's own top face sits only 0.001 mm below the larger box's top face, close enough that the kernel leaves it behind as a redundant near-coplanar sliver face instead of absorbing it cleanly — a volume-only comparison would call this a no-op; the face count is what catches the artifact. Declared APPLIED on the same reasoning as `cut-tangent-cylinder-exact`.",
    build: (b) => {
      const a = call(b, "box", 12, 12, 12);
      const nearly = call(b, "box", 11.999, 12, 12);
      return { operands: [{ role: "operand-a-step", shape: a }, { role: "operand-b-step", shape: nearly }], result: call(b, "fuse", a, nearly) };
    },
  },
  {
    id: "intersect-nearly-identical-operands",
    family: "spatial-relationship",
    outcome: "applied",
    tolerance: "epsilon-degenerate",
    notes: "The same near-duplicate pair, INTERSECTED. MEASURED: volume 1727.856 (11.999×12×12) — exactly the smaller operand, the mirror image of `cut-nearly-identical-operands`'s 0.144 sliver (1728 = 1727.856 + 0.144, so the two fixtures cross-check each other).",
    build: (b) => {
      const a = call(b, "box", 12, 12, 12);
      const nearly = call(b, "box", 11.999, 12, 12);
      return { operands: [{ role: "operand-a-step", shape: a }, { role: "operand-b-step", shape: nearly }], result: call(b, "intersect", a, nearly) };
    },
  },
  {
    id: "intersect-splits-into-several-bodies",
    family: "spatial-relationship",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A result that splits into SEVERAL bodies via INTERSECT rather than cut (`cut-disconnected-result` already covers the cut side): a slab intersected with a three-tooth comb tool (three disjoint boxes bundled as one compound operand) leaves three disjoint pieces. MEASURED: 3 solids, total volume 4500 (3×1500) — the component count is the assertion, since the total volume alone reads identically whether the pieces are 3 separate bodies or one kernel mistakenly welded together.",
    build: (b) => {
      const slab = call(b, "box", 30, 30, 10);
      const tooth1 = call(b, "translate", call(b, "box", 5, 30, 20), [0, 0, -5]);
      const tooth2 = call(b, "translate", call(b, "box", 5, 30, 20), [12, 0, -5]);
      const tooth3 = call(b, "translate", call(b, "box", 5, 30, 20), [24, 0, -5]);
      const comb = call(b, "compound", [tooth1, tooth2, tooth3]);
      return { operands: [{ role: "operand-a-step", shape: slab }, { role: "operand-b-step", shape: comb }], result: call(b, "intersect", slab, comb) };
    },
  },
];
//#endregion 🧪️Recipes
