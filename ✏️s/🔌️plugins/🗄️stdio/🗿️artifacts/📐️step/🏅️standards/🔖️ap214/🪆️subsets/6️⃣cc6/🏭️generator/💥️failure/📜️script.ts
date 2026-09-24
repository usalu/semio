#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🚫️ BRep fixture recipes — the `failure` family.
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
/** 🚫️ The `failure` recipes. */
export const RECIPES: readonly Recipe[] = [
  {
    id: "intersect-disjoint-operands",
    family: "failure",
    outcome: "empty",
    tolerance: "analytic-strict",
    notes: "Empty intersection. The declared outcome is EMPTY, distinguishing 'correctly nothing' from 'the operation failed and produced nothing'.",
    build: (b) => {
      const left = call(b, "box", 10, 10, 10);
      const away = call(b, "translate", call(b, "box", 10, 10, 10), [50, 0, 0]);
      return { operands: [{ role: "operand-a-step", shape: left }, { role: "operand-b-step", shape: away }], result: call(b, "intersect", left, away) };
    },
  },
  {
    id: "cut-identical-operands",
    family: "failure",
    outcome: "empty",
    tolerance: "analytic-strict",
    notes: "Identical operands. A − A is empty; a kernel that returned a zero-volume shell instead of nothing would fail the declared outcome.",
    build: (b) => {
      const box = call(b, "box", 12, 12, 12);
      const same = call(b, "box", 12, 12, 12);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: same }], result: call(b, "cut", box, same) };
    },
  },

  // 🧭️ The declared-outcome vocabulary from here down. Every "rejected" claim below was tested against
  // the ACTUAL kernel via a scratch harness before being written — see 📓️corpus-spatial-and-failure.md
  // for the full list of what was attempted and what the kernel actually did, including the cases where
  // it accepted input this file expected it to reject.

  {
    id: "cut-valid-no-op",
    family: "failure",
    outcome: "no-op",
    tolerance: "analytic-strict",
    notes: "A canonical VALID no-op: a cylinder tool positioned entirely outside the base box, cut. MEASURED: the result is byte-identical to the untouched base (same volume, same 6 faces / 12 edges / 8 vertices) — the vocabulary's cleanest 'legitimately does nothing' case, distinct in construction from `spatial-relationship`'s `cut-disjoint-operands` so the two families don't share a fixture.",
    build: (b) => {
      const box = call(b, "box", 15, 15, 15);
      const tool = call(b, "translate", call(b, "cylinder", 3, 20), [200, 200, -10]);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: tool }], result: call(b, "cut", box, tool) };
    },
  },
  {
    id: "fuse-disjoint-result",
    family: "failure",
    outcome: "disjoint",
    tolerance: "analytic-strict",
    notes: "The 'disjoint / no-intersection result' vocabulary item, canonical form: two boxes that share no boundary at all, fused. MEASURED: two solids survive, total volume 2000 (1000+1000) — fusing shapes with nothing in common does not force them into a single body.",
    build: (b) => {
      const a = call(b, "box", 10, 10, 10);
      const far = call(b, "translate", call(b, "box", 10, 10, 10), [-200, 0, 0]);
      return { operands: [{ role: "operand-a-step", shape: a }, { role: "operand-b-step", shape: far }], result: call(b, "fuse", a, far) };
    },
  },
  {
    id: "intersect-empty-valid-result",
    family: "failure",
    outcome: "empty",
    tolerance: "analytic-strict",
    notes: "The 'empty result where that is valid' vocabulary item: a cylinder and a box that do not overlap, intersected — distinct geometry from `intersect-disjoint-operands` above so the two zero-solid fixtures in this file exercise different shape pairs. MEASURED: zero solids, correctly nothing rather than a degenerate zero-volume shell.",
    build: (b) => {
      const box = call(b, "box", 10, 10, 10);
      const cyl = call(b, "translate", call(b, "cylinder", 3, 10), [50, 50, 0]);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: cyl }], result: call(b, "intersect", box, cyl) };
    },
  },
  {
    id: "reject-negative-box-width",
    family: "failure",
    outcome: "no-op",
    tolerance: "epsilon-degenerate",
    notes: "'Rejected invalid input', attempted: `box(-5, 10, 10)` — a negative extent. MEASURED SURPRISE: the kernel does NOT reject this. It silently accepts the negative width and mirrors the box into the NEGATIVE-x half-space — measured bounds x∈[-5,0], y∈[0,10], z∈[0,10] — producing an ordinary 5×10×10 box (volume 500, 6 faces / 12 edges / 8 vertices, `isValidSolid` true) with no error, warning or flag of any kind. The declared outcome is NO-OP because the cut tool below is placed far away specifically so the boolean itself changes nothing observable; the finding this fixture pins is entirely in the base operand's own construction, not in the boolean applied to it.",
    build: (b) => {
      const mirrored = call(b, "box", -5, 10, 10);
      const tool = call(b, "translate", call(b, "box", 5, 5, 5), [500, 500, 500]);
      return { operands: [{ role: "operand-a-step", shape: mirrored }, { role: "operand-b-step", shape: tool }], result: call(b, "cut", mirrored, tool) };
    },
  },
  {
    id: "reject-zero-height-cylinder",
    family: "failure",
    outcome: "applied",
    tolerance: "epsilon-degenerate",
    notes: "'Rejected invalid input', second attempt: `cylinder(5, 0)` — zero height. MEASURED SURPRISE, in two parts. First, construction is ACCEPTED (no throw), producing a degenerate point-like shape (1 solid, volume 0, 1 face / 2 edges / 1 vertex) that the kernel's OWN `isValidSolid` then reports FALSE for. Second, and worse: cutting this already-invalid degenerate tool from an ordinary box does not raise either — it silently returns a result (7 faces / 15 edges / 9 vertices, volume unchanged at 8000 since the tool had none to remove) whose OWN `isValidSolid` is ALSO false. The invalidity of an operand propagates silently into the invalidity of a boolean's output, with no exception at either step — a caller checking only for thrown errors would see two green lights around one invalid result.",
    build: (b) => {
      const box = call(b, "box", 20, 20, 20);
      const degenerate = call(b, "translate", call(b, "cylinder", 5, 0), [10, 10, 5]);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: degenerate }], result: call(b, "cut", box, degenerate) };
    },
  },
  {
    id: "reject-ellipse-structured-error",
    family: "failure",
    outcome: "rejected",
    tolerance: "analytic-strict",
    notes: "'A deterministic structured error': `ellipse(5, 10)` — a minor radius LARGER than the major one. Unlike the raw primitives above, this constructor genuinely rejects, and does so with a typed, reproducible payload rather than an opaque exception: caught VERBATIM as `{\"kind\":\"VALIDATION\",\"code\":\"ELLIPSE_RADII\",\"message\":\"The minor radius must be smaller than the major one\"}`. Because the malformed ellipse never becomes a usable shape, `result`/`operands` fall back to an ordinary disjoint cut (recorded for provenance only) — the finding this fixture pins is the exact error payload above, not the fallback geometry's numbers.",
    build: (b) => {
      let caught = "";
      try {
        call(b, "ellipse", 5, 10);
      } catch (error) {
        caught = (error as Error).message;
      }
      if (!caught.includes("ELLIPSE_RADII")) throw new Error(`expected ellipse(5,10) to reject with ELLIPSE_RADII, got: ${caught}`);
      const box = call(b, "box", 10, 10, 10);
      const away = call(b, "translate", call(b, "box", 10, 10, 10), [500, 0, 0]);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: away }], result: call(b, "cut", box, away) };
    },
  },
  {
    id: "reject-open-shell-fuse",
    family: "failure",
    outcome: "rejected",
    tolerance: "contact-sensitive",
    notes: "'Rejected open shell where a solid is required': a box built from only 5 of its 6 faces via `sewShells` (no cap on the 6th) is an open Shell, not a Solid — yet `isValid` and even `isValidSolid` both report TRUE for it standing alone. Fed as the base into `fuse`, the kernel DOES reject, but not with a validation message: it logs 'fuse history path produced null result; retrying without evolution tracking', retries, and then throws the caught, VERBATIM message `Cannot determine shape type: shape is null`. The same open shell fed into `cut` or `intersect` does NOT throw at all — see `cut-open-shell-accepted-as-empty` below for that half of the finding. Because the kernel genuinely raises here, `result` falls back to the untouched second operand (recorded for provenance only; the finding is the caught message, not this fallback shape's numbers).",
    build: (b) => {
      const box = call(b, "box", 10, 10, 10);
      const faces = call(b, "getFaces", box) as unknown[];
      const openShell = call(b, "sewShells", faces.slice(0, 5));
      const other = call(b, "translate", call(b, "box", 10, 10, 10), [5, 5, 5]);
      let caught = "";
      try {
        call(b, "fuse", openShell, other);
      } catch (error) {
        caught = (error as Error).message;
      }
      if (!caught.includes("Cannot determine shape type")) throw new Error(`expected fuse(openShell, other) to reject with "Cannot determine shape type", got: ${JSON.stringify(caught)}`);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: other }], result: other };
    },
  },
  {
    id: "cut-open-shell-accepted-as-empty",
    family: "failure",
    outcome: "empty",
    tolerance: "contact-sensitive",
    notes: "The other half of `reject-open-shell-fuse`'s finding: the SAME open, uncapped 5-face shell fed into `cut` (instead of `fuse`) does NOT raise at all. It silently returns a shape with ZERO solids (declared EMPTY here, matching `getSolids().length === 0`) while direct kernel introspection during authoring measured that same in-memory result still reporting `isValidSolid() === true` — a validity flag that is not actually asserting there is a solid. `intersect` on the same open shell behaves the same way (0 solids, `isValidSolid` true). Only `fuse` raises; `cut` and `intersect` fail silently instead of rejecting.",
    build: (b) => {
      const box = call(b, "box", 10, 10, 10);
      const faces = call(b, "getFaces", box) as unknown[];
      const openShell = call(b, "sewShells", faces.slice(0, 5));
      const other = call(b, "translate", call(b, "box", 10, 10, 10), [5, 5, 5]);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: other }], result: call(b, "cut", openShell, other) };
    },
  },
  {
    id: "cut-self-intersecting-tool",
    family: "failure",
    outcome: "applied",
    tolerance: "epsilon-degenerate",
    notes: "'Rejected self-intersecting input', attempted: a bowtie (self-crossing) quadrilateral wire, faced and `thicken`-ed into a solid whose own faces cross themselves. Both `isValid` and `isValidSolid` correctly report FALSE for this shape standing alone (measured volume rounds to ~0, ±4e-14). MEASURED SURPRISE: cutting it from an ordinary box does NOT raise — `cut` silently accepts the known-invalid operand and returns a result with `isValidSolid` now TRUE, unchanged volume (nothing to remove, since the bowtie's own volume is ~0) but an imprinted extra face and edges from the self-crossing surface. The kernel performs no validity check on a boolean operand before running the operation.",
    build: (b) => {
      const p1: readonly [number, number, number] = [0, 0, 0];
      const p2: readonly [number, number, number] = [10, 10, 0];
      const p3: readonly [number, number, number] = [10, 0, 0];
      const p4: readonly [number, number, number] = [0, 10, 0];
      const e1 = call(b, "line", p1, p2);
      const e2 = call(b, "line", p2, p3);
      const e3 = call(b, "line", p3, p4);
      const e4 = call(b, "line", p4, p1);
      const bowtieWire = call(b, "wireLoop", [e1, e2, e3, e4]);
      const bowtieFace = call(b, "face", bowtieWire);
      const bowtieSolid = call(b, "thicken", bowtieFace, 5);
      const box = call(b, "box", 20, 20, 20);
      return { operands: [{ role: "operand-a-step", shape: box }, { role: "operand-b-step", shape: bowtieSolid }], result: call(b, "cut", box, bowtieSolid) };
    },
  },
  {
    id: "fuse-nonmanifold-vertex-compound",
    family: "failure",
    outcome: "disjoint",
    tolerance: "contact-sensitive",
    notes: "'Rejected non-manifold input', attempted: two boxes touching at a single vertex, bundled with `compound()` into ONE operand (rather than passed as two separate operands, as `spatial-relationship`'s `fuse-vertex-touching-boxes` does) — a compound whose local topology at that shared point is not a 2-manifold surface. `isValid` reports TRUE for the bare compound. MEASURED SURPRISE: fusing a third, disjoint box against this non-manifold compound does not raise at all — the kernel accepts it outright and returns THREE disjoint solids (volume 2000+125=2125), simply treating the compound as 'two solids' and unioning in the third. No non-manifold check is performed on a boolean operand.",
    build: (b) => {
      const a = call(b, "box", 10, 10, 10);
      const cornerTouching = call(b, "translate", call(b, "box", 10, 10, 10), [10, 10, 10]);
      const nonManifold = call(b, "compound", [a, cornerTouching]);
      const third = call(b, "translate", call(b, "box", 5, 5, 5), [100, 100, 100]);
      return { operands: [{ role: "operand-a-step", shape: nonManifold }, { role: "operand-b-step", shape: third }], result: call(b, "fuse", nonManifold, third) };
    },
  },
];
//#endregion 🧪️Recipes
