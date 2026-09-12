#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔀 BRep fixture recipes — the `booleans` family: the 13-verb primitives above are what a real
// dispatcher applies ONE AT A TIME, but `create-solid`/`delete-solid` are what a `fuse`/`cut`/
// `intersect` BATCH ultimately produces or consumes, and this family exercises that batching directly
// at the COMPLICATED end the goal explicitly calls out — multi-step chains, tangent contacts, a fully
// coincident shared face, nested voids, disjoint results and a non-manifold single-point touch — each
// MEASURED against what OpenCASCADE actually returns rather than what a Boolean is assumed to do.
//
// Every contact case below was PROBED, not guessed: two spheres tangent at a single point fuse into
// TWO solids, not one — OCCT's Boolean does not bridge a zero-area point contact. Two boxes sharing an
// entire coincident face DO fuse into one. A cutter that fully engulfs its target measures 0 solids
// (`outcome: "empty"`); a cutter with no overlap at all leaves the target's own volume unchanged
// (`outcome: "no-op"`).
//
// @see ../📜️script.ts — the generator that runs these

//#endregion 🧲️Header

//#region 🔌️Adapters
import { type Kernel, type Recipe, call } from "../📜️script.ts";
//#endregion 🔌️Adapters

//#region 🧪️Recipes
/** 🔀 The `booleans` recipes: chains, tangency, coincidence, nested voids, disjoint/empty/no-op results. */
export const RECIPES: readonly Recipe[] = [
  //#region ⛓️multi-step chains
  {
    id: "booleans-cut-fuse-chain-small",
    family: "booleans",
    kind: "create-solid",
    outcome: "applied",
    tolerance: "boolean-standard",
    notes: "A two-step chain: a box bored through by a cylinder (`cut`), then a cylindrical boss fused onto the resulting face (`fuse`). `operand-a`/`operand-b` are the chain's two RAW inputs — the plain box and the bore cylinder — and `expected.step` is the state after BOTH steps, not just the first.",
    build: (b) => {
      const box = call(b, "box", 30, 30, 20);
      const bore = call(b, "translate", call(b, "cylinder", 5, 30), [15, 15, -5]);
      const bored = call(b, "cut", box, bore);
      const boss = call(b, "translate", call(b, "cylinder", 8, 10), [15, 15, 20]);
      const result = call(b, "fuse", bored, boss);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: bore }], result };
    },
  },
  {
    id: "booleans-cut-fuse-chain-large",
    family: "booleans",
    kind: "create-solid",
    outcome: "applied",
    tolerance: "boolean-standard",
    notes: "The bore-then-boss chain at 1e2 scale.",
    build: (b) => {
      const box = call(b, "box", 3000, 3000, 2000);
      const bore = call(b, "translate", call(b, "cylinder", 500, 3000), [1500, 1500, -500]);
      const bored = call(b, "cut", box, bore);
      const boss = call(b, "translate", call(b, "cylinder", 800, 1000), [1500, 1500, 2000]);
      const result = call(b, "fuse", bored, boss);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: bore }], result };
    },
  },
  {
    id: "booleans-fuse-cut-intersect-three-step",
    family: "booleans",
    kind: "create-solid",
    outcome: "applied",
    tolerance: "boolean-standard",
    notes: "A THREE-operation chain touching all three Boolean verbs in sequence: `fuse(box, cylinder)`, then `cut` by a second cylinder, then `intersect` with a bounding box that trims the result back down. `operand-a`/`operand-b` are the first `fuse`'s two inputs; the cutter and the trimming box are recorded only in `notes`, same convention `s.stdio.step@ap214/✳️cc6`'s own chain fixtures use for a chain that folds in more tools than two operand files could represent.",
    build: (b) => {
      const box = call(b, "box", 30, 30, 15);
      const post = call(b, "translate", call(b, "cylinder", 8, 25), [15, 15, 15]);
      const fused = call(b, "fuse", box, post);
      const cutter = call(b, "translate", call(b, "cylinder", 3, 40), [15, 15, -5]);
      const cutResult = call(b, "cut", fused, cutter);
      const trimBox = call(b, "box", 30, 30, 30);
      const result = call(b, "intersect", cutResult, trimBox);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: post }], result };
    },
  },
  {
    id: "booleans-multistep-complex-scaled-1e4",
    family: "booleans",
    kind: "create-solid",
    outcome: "applied",
    tolerance: "boolean-standard",
    notes: "A FOUR-step chain (`fuse`, `cut`, `cut`, `fuse`) at 1e4 scale — the largest linear dimension in this family, chosen to exercise `tessellationToleranceFor`'s relative term against the small chains' absolute floor in the same comparison run.",
    build: (b) => {
      const box = call(b, "box", 10000, 6000, 4000);
      const rib = call(b, "translate", call(b, "box", 200, 6000, 4000), [4900, 0, 0]);
      let solid = call(b, "fuse", box, rib);
      const bore1 = call(b, "translate", call(b, "cylinder", 400, 4000), [2000, 3000, 0]);
      solid = call(b, "cut", solid, bore1);
      const bore2 = call(b, "translate", call(b, "cylinder", 400, 4000), [8000, 3000, 0]);
      solid = call(b, "cut", solid, bore2);
      const boss = call(b, "translate", call(b, "cylinder", 600, 800), [5000, 3000, 4000]);
      const result = call(b, "fuse", solid, boss);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: rib }], result };
    },
  },
  //#endregion ⛓️multi-step chains

  //#region 👆tangent contact (disjoint)
  {
    id: "booleans-tangent-spheres-point-contact",
    family: "booleans",
    kind: "create-solid",
    outcome: "disjoint",
    tolerance: "boolean-standard",
    notes: "Two spheres of equal radius placed exactly `2r` apart touch at a single POINT. MEASURED: `fuse` on this pair returns TWO solids, not one — OpenCASCADE's Boolean does not bridge a zero-area point contact into a single manifold body, whatever the geometric intuition of 'touching' suggests. `outcome: disjoint` records the kernel's actual classification, not an assumption about what tangency should do.",
    build: (b) => {
      const sphereA = call(b, "sphere", 5);
      const sphereB = call(b, "translate", call(b, "sphere", 5), [10, 0, 0]);
      const result = call(b, "fuse", sphereA, sphereB);
      return { operands: [{ role: "operand-a-step", shape: sphereA }, { role: "operand-b-step", shape: sphereB }], result };
    },
  },
  {
    id: "booleans-tangent-spheres-point-contact-large",
    family: "booleans",
    kind: "create-solid",
    outcome: "disjoint",
    tolerance: "boolean-standard",
    notes: "The point-tangent sphere pair at 1e2 scale.",
    build: (b) => {
      const sphereA = call(b, "sphere", 500);
      const sphereB = call(b, "translate", call(b, "sphere", 500), [1000, 0, 0]);
      const result = call(b, "fuse", sphereA, sphereB);
      return { operands: [{ role: "operand-a-step", shape: sphereA }, { role: "operand-b-step", shape: sphereB }], result };
    },
  },
  {
    id: "booleans-tangent-cylinders-line-contact",
    family: "booleans",
    kind: "create-solid",
    outcome: "disjoint",
    tolerance: "boolean-standard",
    notes: "Two parallel cylinders of equal radius placed exactly `2r` apart touch along a LINE (their full height) rather than a single point. MEASURED: `fuse` on this pair ALSO returns two solids — the same non-bridging behaviour as the point-tangent spheres, now confirmed for a higher-dimensional (1D, not 0D) tangency locus.",
    build: (b) => {
      const cylA = call(b, "cylinder", 5, 20);
      const cylB = call(b, "translate", call(b, "cylinder", 5, 20), [10, 0, 0]);
      const result = call(b, "fuse", cylA, cylB);
      return { operands: [{ role: "operand-a-step", shape: cylA }, { role: "operand-b-step", shape: cylB }], result };
    },
  },
  {
    id: "booleans-non-manifold-corner-touch-boxes",
    family: "booleans",
    kind: "create-solid",
    outcome: "disjoint",
    tolerance: "boolean-standard",
    notes: "Two boxes placed so they share exactly ONE corner VERTEX and nothing else — the non-manifold single-point touch the goal calls out by name, distinct from the sphere/cylinder tangency above in that the shared locus is a single shared VERTEX of two otherwise axis-aligned solids, not a smooth tangent point on curved surfaces. MEASURED: `fuse` returns two solids; the compound of the two UNFUSED operands already shows the shared corner as a coincident vertex position across 16 total vertices, not a merged 15.",
    build: (b) => {
      const boxA = call(b, "box", 10, 10, 10);
      const boxB = call(b, "translate", call(b, "box", 10, 10, 10), [10, 10, 10]);
      const result = call(b, "fuse", boxA, boxB);
      return { operands: [{ role: "operand-a-step", shape: boxA }, { role: "operand-b-step", shape: boxB }], result };
    },
  },
  //#endregion 👆tangent contact (disjoint)

  //#region 🟰coincident faces (merge)
  {
    id: "booleans-coincident-face-stack-fuse-small",
    family: "booleans",
    kind: "create-solid",
    outcome: "applied",
    tolerance: "boolean-standard",
    notes: "Two boxes stacked so an ENTIRE face is exactly coincident (not merely tangent at a point or a line). MEASURED: `fuse` merges these into ONE solid of the combined volume (2000 = 10×10×20) — the coincident face is fully absorbed, unlike the point/line tangency cases above. The contrast between this fixture and the tangent ones is the point: whether a Boolean merges two solids depends on the DIMENSION of their shared locus, not merely on 'do they touch'.",
    build: (b) => {
      const boxA = call(b, "box", 10, 10, 10);
      const boxB = call(b, "translate", call(b, "box", 10, 10, 10), [0, 0, 10]);
      const result = call(b, "fuse", boxA, boxB);
      return { operands: [{ role: "operand-a-step", shape: boxA }, { role: "operand-b-step", shape: boxB }], result };
    },
  },
  {
    id: "booleans-coincident-face-stack-fuse-large",
    family: "booleans",
    kind: "create-solid",
    outcome: "applied",
    tolerance: "boolean-standard",
    notes: "The full-face-coincident stack at 1e2 scale.",
    build: (b) => {
      const boxA = call(b, "box", 1000, 1000, 1000);
      const boxB = call(b, "translate", call(b, "box", 1000, 1000, 1000), [0, 0, 1000]);
      const result = call(b, "fuse", boxA, boxB);
      return { operands: [{ role: "operand-a-step", shape: boxA }, { role: "operand-b-step", shape: boxB }], result };
    },
  },
  //#endregion 🟰coincident faces (merge)

  //#region 🕳️nested voids
  {
    id: "booleans-nested-void-single-small",
    family: "booleans",
    kind: "create-solid",
    outcome: "applied",
    tolerance: "boolean-standard",
    notes: "A single fully-internal cavity: the cutter never reaches any outer face, so the outer surface is completely untouched and only an internal shell records the change — exactly the case where a surface-only check would miss a lost cavity. MEASURED: volume 7875 = 20³ − 5³.",
    build: (b) => {
      const outer = call(b, "box", 20, 20, 20);
      const cavity = call(b, "translate", call(b, "box", 5, 5, 5), [7, 7, 7]);
      const result = call(b, "cut", outer, cavity);
      return { operands: [{ role: "operand-a-step", shape: outer }, { role: "operand-b-step", shape: cavity }], result };
    },
  },
  {
    id: "booleans-nested-void-single-large",
    family: "booleans",
    kind: "create-solid",
    outcome: "applied",
    tolerance: "boolean-standard",
    notes: "The single fully-internal cavity at 1e2 scale.",
    build: (b) => {
      const outer = call(b, "box", 2000, 2000, 2000);
      const cavity = call(b, "translate", call(b, "box", 500, 500, 500), [700, 700, 700]);
      const result = call(b, "cut", outer, cavity);
      return { operands: [{ role: "operand-a-step", shape: outer }, { role: "operand-b-step", shape: cavity }], result };
    },
  },
  {
    id: "booleans-nested-void-double-small",
    family: "booleans",
    kind: "create-solid",
    outcome: "applied",
    tolerance: "boolean-standard",
    notes: "TWO separate, non-touching internal cavities cut into the same outer box. MEASURED: still 1 solid overall, but 3 shells (1 outer + 2 independent inner cavity shells) and volume 26750 = 30³ − 2×5³ — the shell COUNT is where this fixture differs from the single-cavity ones, not the solid count.",
    build: (b) => {
      const outer = call(b, "box", 30, 30, 30);
      const cavity1 = call(b, "translate", call(b, "box", 5, 5, 5), [5, 5, 5]);
      const cavity2 = call(b, "translate", call(b, "box", 5, 5, 5), [20, 20, 20]);
      let result = call(b, "cut", outer, cavity1);
      result = call(b, "cut", result, cavity2);
      return { operands: [{ role: "operand-a-step", shape: outer }, { role: "operand-b-step", shape: cavity1 }], result };
    },
  },
  {
    id: "booleans-nested-void-double-large",
    family: "booleans",
    kind: "create-solid",
    outcome: "applied",
    tolerance: "boolean-standard",
    notes: "The double-internal-cavity cut at 1e2 scale.",
    build: (b) => {
      const outer = call(b, "box", 3000, 3000, 3000);
      const cavity1 = call(b, "translate", call(b, "box", 500, 500, 500), [500, 500, 500]);
      const cavity2 = call(b, "translate", call(b, "box", 500, 500, 500), [2000, 2000, 2000]);
      let result = call(b, "cut", outer, cavity1);
      result = call(b, "cut", result, cavity2);
      return { operands: [{ role: "operand-a-step", shape: outer }, { role: "operand-b-step", shape: cavity1 }], result };
    },
  },
  //#endregion 🕳️nested voids

  //#region 🚫empty / no-op / disjoint results
  {
    id: "booleans-cut-fully-engulfed-empty",
    family: "booleans",
    kind: "delete-solid",
    outcome: "empty",
    tolerance: "boolean-standard",
    notes: "A cutter that fully ENGULFS its target. MEASURED: `cut` returns 0 solids — genuinely nothing, not a degenerate near-zero sliver — which this generator records as `hasExtent: false` rather than fabricating a placeholder shape or a mesh for a volume that does not exist.",
    build: (b) => {
      const target = call(b, "box", 5, 5, 5);
      const bigCutter = call(b, "translate", call(b, "box", 100, 100, 100), [-50, -50, -50]);
      const result = call(b, "cut", target, bigCutter);
      return { operands: [{ role: "operand-a-step", shape: target }, { role: "operand-b-step", shape: bigCutter }], result };
    },
  },
  {
    id: "booleans-cut-no-overlap-noop",
    family: "booleans",
    kind: "delete-solid",
    outcome: "no-op",
    tolerance: "boolean-standard",
    notes: "A cutter with NO overlap with its target at all. MEASURED: `cut` returns 1 solid with the target's own UNCHANGED volume (125 = 5³) — the Boolean is a legal no-op, distinguishable from the fully-engulfed `empty` fixture only by actually measuring the result rather than assuming 'a cut with no interference is either always a no-op or always a failure'.",
    build: (b) => {
      const target = call(b, "box", 5, 5, 5);
      const farCutter = call(b, "translate", call(b, "box", 5, 5, 5), [100, 100, 100]);
      const result = call(b, "cut", target, farCutter);
      return { operands: [{ role: "operand-a-step", shape: target }, { role: "operand-b-step", shape: farCutter }], result };
    },
  },
  {
    id: "booleans-intersect-non-overlap-empty",
    family: "booleans",
    kind: "create-solid",
    outcome: "empty",
    tolerance: "boolean-standard",
    notes: "`intersect` on two solids with no shared volume at all. MEASURED: 0 solids — the same `empty` classification as the fully-engulfed `cut`, now from the opposite direction (no overlap rather than total overlap), confirming `empty` is about the RESULT'S measured extent, not about which Boolean verb produced it.",
    build: (b) => {
      const a = call(b, "box", 5, 5, 5);
      const c = call(b, "translate", call(b, "box", 5, 5, 5), [100, 100, 100]);
      const result = call(b, "intersect", a, c);
      return { operands: [{ role: "operand-a-step", shape: a }, { role: "operand-b-step", shape: c }], result };
    },
  },
  {
    id: "booleans-disjoint-compound-mixed-scale",
    family: "booleans",
    kind: "create-solid",
    outcome: "disjoint",
    tolerance: "boolean-standard",
    notes: "Two solids at DELIBERATELY MISMATCHED scales (a 5mm cube and a 500mm box, three orders of magnitude apart in linear size) placed far enough apart that neither Boolean operand overlaps or touches the other at all — the plainest possible `disjoint` classification, and a check that `tessellationToleranceFor`'s relative term is computed from the RESULT's own bounding box (which spans both scales at once) rather than from either operand's scale alone.",
    build: (b) => {
      const small = call(b, "box", 5, 5, 5);
      const large = call(b, "translate", call(b, "box", 500, 500, 500), [2000, 2000, 2000]);
      const result = call(b, "compound", [small, large]);
      return { operands: [{ role: "operand-a-step", shape: small }, { role: "operand-b-step", shape: large }], result };
    },
  },
  //#endregion 🚫empty / no-op / disjoint results
];
//#endregion 🧪️Recipes
