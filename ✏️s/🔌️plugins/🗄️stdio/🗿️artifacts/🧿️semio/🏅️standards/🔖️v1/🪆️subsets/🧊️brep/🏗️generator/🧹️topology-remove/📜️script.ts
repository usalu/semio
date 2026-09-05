#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🗑️ BRep fixture recipes — the `topology-remove` family: the `delete-*` counterpart of every
// `topology-build` pair, PLUS the transitions that a real dispatcher must REJECT. A rejection here is
// PROVEN, not asserted: each rejected recipe attempts the transition the deletion would require and
// records what OpenCASCADE actually did with it — a wire that refuses to close, a `solid()` call that
// throws on an empty face list, a `solid()` call that SUCCEEDS but returns `isValidSolid() === false`.
// Two distinct rejection MECHANISMS appear below on purpose: a thrown kernel error is a different
// failure mode from a silently-invalid accepted result, and a comparison harness has to handle both.
//
// @see ../📜️script.ts — the generator that runs these
// @see ../📜️topology-buildscript.ts — the `create-*` counterpart these recipes invert

//#endregion 🧲️Header

//#region 🔌️Adapters
import { type Kernel, type Recipe, call } from "../📜️script.ts";
//#endregion 🔌️Adapters

//#region 🧪️Recipes
/** 🗑️ The `topology-remove` recipes: `delete-*` counterparts, each with at least one REJECTED variant. */
export const RECIPES: readonly Recipe[] = [
  //#region 🗑️delete-vertex
  {
    id: "topology-remove-delete-vertex-loose-apex-small",
    family: "topology-remove",
    kind: "delete-vertex",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "Inverts `topology-build-create-vertex-apex-above-box-small`: BEFORE is `compound(box, apex)`, AFTER removes the loose apex vertex and leaves the box alone. The plainest legal `delete-vertex`: the removed entity had no incident edges to cascade into.",
    build: (b) => {
      const box = call(b, "box", 10, 10, 10);
      const apex = call(b, "vertex", [5, 5, 15]);
      return { operands: [{ role: "operand-a-step", shape: call(b, "compound", [box, apex]) }], result: box };
    },
  },
  {
    id: "topology-remove-delete-vertex-loose-apex-large",
    family: "topology-remove",
    kind: "delete-vertex",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "The loose-vertex removal at 1e3 scale.",
    build: (b) => {
      const box = call(b, "box", 1000, 1000, 1000);
      const apex = call(b, "vertex", [500, 500, 1500]);
      return { operands: [{ role: "operand-a-step", shape: call(b, "compound", [box, apex]) }], result: box };
    },
  },
  {
    id: "topology-remove-delete-vertex-corner-cascade-rejected",
    family: "topology-remove",
    kind: "delete-vertex",
    outcome: "rejected",
    tolerance: "topology-exact",
    notes: "Mirrors this subset's own oracle scenario `delete-vertex/removes-a-corner-vertex-and-cascades-into-its-two-incident-edges`, and proves WHY it must be rejected here: removing a square's corner vertex forces its two incident edges out too, leaving only the two OPPOSITE, mutually disconnected edges. `wireLoop` on those two — MEASURED — throws `WIRE_BUILD_FAILED` (the kernel cannot even attempt to close two edges that share no endpoint at all, a step short of the `WIRE_NOT_CLOSED` the sibling `delete-edge` fixture below gets from a wire that DOES connect but not into a loop). The deletion is topologically incoherent without a replacement edge no `delete-vertex` call supplies, so a real dispatcher has nothing legal to build.",
    build: (b) => {
      const p1: [number, number, number] = [0, 0, 0];
      const p2: [number, number, number] = [10, 0, 0];
      const p3: [number, number, number] = [10, 10, 0];
      const p4: [number, number, number] = [0, 10, 0];
      const square = call(b, "polygon", [p1, p2, p3, p4]);
      const remaining = [call(b, "line", p2, p3), call(b, "line", p4, p1)];
      let rejectionReason = "unexpected: kernel accepted the disconnected remainder";
      let result: unknown = null;
      try {
        call(b, "wireLoop", remaining);
      } catch (error) {
        rejectionReason = `wireLoop(remaining-two-edges) — ${(error as Error).message}`;
      }
      return { operands: [{ role: "operand-a-step", shape: square }], result, rejectionReason };
    },
  },
  //#endregion 🗑️delete-vertex

  //#region ✂️delete-edge
  {
    id: "topology-remove-delete-edge-loose-diagonal-small",
    family: "topology-remove",
    kind: "delete-edge",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "Inverts `topology-build-create-edge-diagonal-across-square-small`: BEFORE is `compound(face, diagonal)`, AFTER removes the loose diagonal and leaves the face alone — the diagonal was never part of the face's own boundary wire.",
    build: (b) => {
      const square = call(b, "polygon", [[0, 0, 0], [10, 0, 0], [10, 10, 0], [0, 10, 0]]);
      const diagonal = call(b, "line", [0, 0, 0], [10, 10, 0]);
      return { operands: [{ role: "operand-a-step", shape: call(b, "compound", [square, diagonal]) }], result: square };
    },
  },
  {
    id: "topology-remove-delete-edge-loose-diagonal-large",
    family: "topology-remove",
    kind: "delete-edge",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "The loose-diagonal removal at 5e2 scale.",
    build: (b) => {
      const square = call(b, "polygon", [[0, 0, 0], [500, 0, 0], [500, 500, 0], [0, 500, 0]]);
      const diagonal = call(b, "line", [0, 0, 0], [500, 500, 0]);
      return { operands: [{ role: "operand-a-step", shape: call(b, "compound", [square, diagonal]) }], result: square };
    },
  },
  {
    id: "topology-remove-delete-edge-boundary-edge-rejected",
    family: "topology-remove",
    kind: "delete-edge",
    outcome: "rejected",
    tolerance: "topology-exact",
    notes: "Deleting an edge that is part of a face's own boundary wire, not a loose addition. Removing one of the square's four boundary edges and reassembling the other three — MEASURED — throws `WIRE_NOT_CLOSED`: three edges of a four-edge rectangle span only 3/4 of the perimeter and cannot close. A face's boundary can only lose an edge if something else fills the gap it leaves, which `delete-edge` alone does not provide.",
    build: (b) => {
      const p1: [number, number, number] = [0, 0, 0];
      const p2: [number, number, number] = [10, 0, 0];
      const p3: [number, number, number] = [10, 10, 0];
      const p4: [number, number, number] = [0, 10, 0];
      const square = call(b, "polygon", [p1, p2, p3, p4]);
      const remaining = [call(b, "line", p1, p2), call(b, "line", p2, p3), call(b, "line", p3, p4)];
      let rejectionReason = "unexpected: kernel accepted the open 3-edge remainder as closed";
      let result: unknown = null;
      try {
        call(b, "wireLoop", remaining);
      } catch (error) {
        rejectionReason = `wireLoop(remaining-three-edges) — ${(error as Error).message}`;
      }
      return { operands: [{ role: "operand-a-step", shape: square }], result, rejectionReason };
    },
  },
  //#endregion ✂️delete-edge

  //#region 🚮delete-face
  {
    id: "topology-remove-delete-face-from-redundant-shell-small",
    family: "topology-remove",
    kind: "delete-face",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "A `delete-face` that LEAVES A VALID SOLID: BEFORE is `compound(box, redundantFlippedShell)` — the box plus a second, non-load-bearing shell built from its own faces with flipped orientation (see `topology-build-create-shell-second-shell-flipped-sense`). AFTER removes one face from the REDUNDANT shell only; the box itself, still a complete 6-face solid, is untouched and remains valid. Proves a face deletion is safe exactly when nothing else depends on the face's closure.",
    build: (b) => {
      const box = call(b, "box", 10, 10, 10);
      const faces = call(b, "getFaces", box) as unknown[];
      const flipped = faces.map((f) => call(b, "flipFaceOrientation", f));
      const redundantShellFull = call(b, "sewShells", flipped, true);
      const redundantShellMissingOne = call(b, "sewShells", flipped.slice(1), true);
      return { operands: [{ role: "operand-a-step", shape: call(b, "compound", [box, redundantShellFull]) }], result: call(b, "compound", [box, redundantShellMissingOne]) };
    },
  },
  {
    id: "topology-remove-delete-face-from-redundant-shell-large",
    family: "topology-remove",
    kind: "delete-face",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "The redundant-shell face deletion at 5e2 scale.",
    build: (b) => {
      const box = call(b, "box", 500, 500, 500);
      const faces = call(b, "getFaces", box) as unknown[];
      const flipped = faces.map((f) => call(b, "flipFaceOrientation", f));
      const redundantShellFull = call(b, "sewShells", flipped, true);
      const redundantShellMissingOne = call(b, "sewShells", flipped.slice(1), true);
      return { operands: [{ role: "operand-a-step", shape: call(b, "compound", [box, redundantShellFull]) }], result: call(b, "compound", [box, redundantShellMissingOne]) };
    },
  },
  {
    id: "topology-remove-delete-face-still-bounding-closed-shell-rejected",
    family: "topology-remove",
    kind: "delete-face",
    outcome: "rejected",
    tolerance: "topology-exact",
    notes: "The exact case this subset's own goal calls out: deleting a face still bounding a closed shell. BEFORE is a complete 6-face box solid. Attempting the deletion by rebuilding `solid()` from the remaining 5 faces — MEASURED — does NOT throw: OpenCASCADE returns a shape immediately (`isValidSolid()` on THAT in-memory object measures `false`), but the honest proof only shows up downstream: exported to STEP and re-imported — the only form a real consumer ever sees — `getSolids()` on it measures `0`, even though it still carries all 5 faces / 12 edges / 8 vertices and `isValidSolid()` on the re-imported shape now (misleadingly) measures `true`. `isValidSolid()` is NOT a reliable signal across a STEP round-trip; `getSolids().length === 0` on the re-imported artifact is. Both numbers are recorded in `expected.metrics.json` rather than only the one that reads cleanly.",
    build: (b) => {
      const box = call(b, "box", 10, 10, 10);
      const faces = call(b, "getFaces", box) as unknown[];
      const attempt = call(b, "solid", faces.slice(1));
      const preExportValid = call(b, "isValidSolid", attempt) as boolean;
      return {
        operands: [{ role: "operand-a-step", shape: box }],
        result: attempt,
        rejectionReason: `solid(5 of 6 faces) did not throw; isValidSolid() on the in-memory attempt measured ${preExportValid} — the metrics below re-measure it from the re-imported expected.step, where getSolids().length===0 is the reliable proof of rejection regardless of what isValidSolid() reports there`,
      };
    },
  },
  //#endregion 🚮delete-face

  //#region 💥delete-shell
  {
    id: "topology-remove-delete-shell-redundant-shell-small",
    family: "topology-remove",
    kind: "delete-shell",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "Inverts `topology-build-create-shell-second-shell-flipped-sense`: BEFORE is `compound(box, redundantShell)`, AFTER removes the redundant shell entirely and leaves the box's own single shell untouched.",
    build: (b) => {
      const box = call(b, "box", 10, 10, 10);
      const faces = call(b, "getFaces", box) as unknown[];
      const flipped = faces.map((f) => call(b, "flipFaceOrientation", f));
      const redundantShell = call(b, "sewShells", flipped, true);
      return { operands: [{ role: "operand-a-step", shape: call(b, "compound", [box, redundantShell]) }], result: box };
    },
  },
  {
    id: "topology-remove-delete-shell-redundant-shell-large",
    family: "topology-remove",
    kind: "delete-shell",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "The redundant-shell removal at 5e2 scale.",
    build: (b) => {
      const box = call(b, "box", 500, 500, 500);
      const faces = call(b, "getFaces", box) as unknown[];
      const flipped = faces.map((f) => call(b, "flipFaceOrientation", f));
      const redundantShell = call(b, "sewShells", flipped, true);
      return { operands: [{ role: "operand-a-step", shape: call(b, "compound", [box, redundantShell]) }], result: box };
    },
  },
  {
    id: "topology-remove-delete-shell-only-shell-of-solid-rejected",
    family: "topology-remove",
    kind: "delete-shell",
    outcome: "rejected",
    tolerance: "topology-exact",
    notes: "Deleting the ONLY shell that defines a solid — nothing would be left to bound it. Rebuilding via `solid([])` (an empty face/shell list) — MEASURED — throws `Cannot determine shape type: shape is null` directly out of the kernel: there is no shape at all to wrap, let alone a valid one, which is the cleanest possible proof that a solid's last shell cannot simply vanish.",
    build: (b) => {
      const box = call(b, "box", 10, 10, 10);
      let rejectionReason = "unexpected: solid([]) returned a shape";
      let result: unknown = null;
      try {
        call(b, "solid", []);
      } catch (error) {
        rejectionReason = `solid([]) — ${(error as Error).message}`;
      }
      return { operands: [{ role: "operand-a-step", shape: box }], result, rejectionReason };
    },
  },
  //#endregion 💥delete-shell

  //#region 🕳️delete-solid
  {
    id: "topology-remove-delete-solid-second-disjoint-solid-small",
    family: "topology-remove",
    kind: "delete-solid",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "Inverts `topology-build-create-solid-second-disjoint-solid-small`: BEFORE is `compound(boxA, boxB)`, AFTER removes `boxB` and leaves `boxA` alone.",
    build: (b) => {
      const boxA = call(b, "box", 10, 10, 10);
      const boxB = call(b, "translate", call(b, "box", 6, 6, 6), [30, 0, 0]);
      return { operands: [{ role: "operand-a-step", shape: call(b, "compound", [boxA, boxB]) }], result: boxA };
    },
  },
  {
    id: "topology-remove-delete-solid-second-disjoint-solid-large",
    family: "topology-remove",
    kind: "delete-solid",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "The second-disjoint-solid removal at 1e3 scale.",
    build: (b) => {
      const boxA = call(b, "box", 1000, 1000, 1000);
      const boxB = call(b, "translate", call(b, "box", 600, 600, 600), [3000, 0, 0]);
      return { operands: [{ role: "operand-a-step", shape: call(b, "compound", [boxA, boxB]) }], result: boxA };
    },
  },
  {
    id: "topology-remove-delete-solid-one-of-three",
    family: "topology-remove",
    kind: "delete-solid",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "THREE solids in the compound, ONE deleted — exercises `delete-solid` where the entity count doesn't collapse to a single leftover, distinguishing it from the two-solid fixtures above.",
    build: (b) => {
      const boxA = call(b, "box", 8, 8, 8);
      const boxB = call(b, "translate", call(b, "box", 8, 8, 8), [20, 0, 0]);
      const boxC = call(b, "translate", call(b, "box", 8, 8, 8), [40, 0, 0]);
      return { operands: [{ role: "operand-a-step", shape: call(b, "compound", [boxA, boxB, boxC]) }], result: call(b, "compound", [boxA, boxC]) };
    },
  },
  //#endregion 🕳️delete-solid
];
//#endregion 🧪️Recipes
