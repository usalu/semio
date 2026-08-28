#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 📍 BRep fixture recipes — the `move-vertex` family: three DISTINCT consequences of moving one
// vertex's position while every other vertex, edge and face stays put — mirroring this subset's own
// oracle scenario `move-vertex/lifts-the-third-corner-off-the-base-plane` and then going past it into
// the two ways the same edit can go wrong.
//
//   1. a move that STAYS VALID — the rebuilt face or solid is exactly as legal as before.
//   2. a move that makes a face NON-PLANAR — `face()`/`polygon()` (both planar-only builders) THROW
//      `FACE_NOT_PLANAR`, MEASURED, the moment the four corners stop lying in one plane.
//   3. a move that SELF-INTERSECTS — reordering two corners crosses the boundary through itself, and
//      the kernel does NOT throw: it returns a face whose signed area — MEASURED — collapses to
//      numerically zero, a silently-degenerate result rather than a thrown error.
//
// @see ../📜️script.ts — the generator that runs these

//#endregion 🧲️Header

//#region 🔌️Adapters
import { type Kernel, type Recipe, call } from "../📜️script.ts";
//#endregion 🔌️Adapters

//#region 🧪️Recipes
/** 📍 The `move-vertex` recipes: valid in-plane moves, planarity-breaking moves, self-intersecting moves. */
export const RECIPES: readonly Recipe[] = [
  //#region ✅stays valid
  {
    id: "move-vertex-inplane-shift-quad-small",
    family: "move-vertex",
    kind: "move-vertex",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "The plainest legal `move-vertex`: BEFORE is a square face, AFTER moves one corner sideways WITHIN the same plane, staying a valid — if no longer square — planar quad. Area and boundary both change; validity does not.",
    build: (b) => {
      const before = call(b, "polygon", [[0, 0, 0], [10, 0, 0], [10, 10, 0], [0, 10, 0]]);
      const after = call(b, "polygon", [[0, 0, 0], [10, 0, 0], [14, 10, 0], [0, 10, 0]]);
      return { operands: [{ role: "operand-a-step", shape: before }], result: after };
    },
  },
  {
    id: "move-vertex-inplane-shift-quad-large",
    family: "move-vertex",
    kind: "move-vertex",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "The in-plane corner shift at 1e3 scale.",
    build: (b) => {
      const before = call(b, "polygon", [[0, 0, 0], [1000, 0, 0], [1000, 1000, 0], [0, 1000, 0]]);
      const after = call(b, "polygon", [[0, 0, 0], [1000, 0, 0], [1400, 1000, 0], [0, 1000, 0]]);
      return { operands: [{ role: "operand-a-step", shape: before }], result: after };
    },
  },
  {
    id: "move-vertex-inplane-shift-prism-solid-small",
    family: "move-vertex",
    kind: "move-vertex",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "The same in-plane corner shift carried through `extrude` into a SOLID rather than a flat face, so the moved vertex propagates into two faces (top and bottom) and two edges of a real 3D body, not just one face's own boundary.",
    build: (b) => {
      const beforeProfile = call(b, "polygon", [[0, 0, 0], [10, 0, 0], [10, 10, 0], [0, 10, 0]]);
      const before = call(b, "extrude", beforeProfile, [0, 0, 6]);
      const afterProfile = call(b, "polygon", [[0, 0, 0], [10, 0, 0], [14, 10, 0], [0, 10, 0]]);
      const after = call(b, "extrude", afterProfile, [0, 0, 6]);
      return { operands: [{ role: "operand-a-step", shape: before }], result: after };
    },
  },
  {
    id: "move-vertex-inplane-shift-prism-solid-large",
    family: "move-vertex",
    kind: "move-vertex",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "The in-plane corner shift on a solid at 2e2 scale.",
    build: (b) => {
      const beforeProfile = call(b, "polygon", [[0, 0, 0], [200, 0, 0], [200, 200, 0], [0, 200, 0]]);
      const before = call(b, "extrude", beforeProfile, [0, 0, 120]);
      const afterProfile = call(b, "polygon", [[0, 0, 0], [200, 0, 0], [280, 200, 0], [0, 200, 0]]);
      const after = call(b, "extrude", afterProfile, [0, 0, 120]);
      return { operands: [{ role: "operand-a-step", shape: before }], result: after };
    },
  },
  {
    id: "move-vertex-lifts-third-corner-off-base-plane",
    family: "move-vertex",
    kind: "move-vertex",
    outcome: "applied",
    tolerance: "geometry-tessellated",
    notes: "Mirrors this subset's own oracle scenario `move-vertex/lifts-the-third-corner-off-the-base-plane` exactly, using a TRIANGLE rather than a quad: a triangle's three vertices are always coplanar regardless of position, so lifting one corner into +z stays a perfectly valid — merely tilted — planar face. Isolates 'moves off the base plane' from 'moves that break planarity', which only applies once a face has four or more corners.",
    build: (b) => {
      const p1: [number, number, number] = [0, 0, 0];
      const p2: [number, number, number] = [12, 0, 0];
      const p3: [number, number, number] = [6, 10, 0];
      const before = call(b, "polygon", [p1, p2, p3]);
      const p3Lifted: [number, number, number] = [6, 10, 7];
      const after = call(b, "polygon", [p1, p2, p3Lifted]);
      return { operands: [{ role: "operand-a-step", shape: before }], result: after };
    },
  },
  {
    id: "move-vertex-lifts-third-corner-off-base-plane-large",
    family: "move-vertex",
    kind: "move-vertex",
    outcome: "applied",
    tolerance: "geometry-tessellated",
    notes: "The off-plane triangle-corner lift at 1e2 scale.",
    build: (b) => {
      const p1: [number, number, number] = [0, 0, 0];
      const p2: [number, number, number] = [120, 0, 0];
      const p3: [number, number, number] = [60, 100, 0];
      const before = call(b, "polygon", [p1, p2, p3]);
      const p3Lifted: [number, number, number] = [60, 100, 70];
      const after = call(b, "polygon", [p1, p2, p3Lifted]);
      return { operands: [{ role: "operand-a-step", shape: before }], result: after };
    },
  },
  //#endregion ✅stays valid

  //#region 🚫makes non-planar (rejected)
  {
    id: "move-vertex-makes-quad-nonplanar-rejected",
    family: "move-vertex",
    kind: "move-vertex",
    outcome: "rejected",
    tolerance: "topology-exact",
    notes: "Moving a QUAD's corner off the shared plane of the other three — unlike the triangle case above, four points generally stop being coplanar the moment one moves in z. Rebuilding the face via `polygon()` (planar-only) — MEASURED — throws `FACE_NOT_PLANAR`. `filledFace()` on the same non-planar wire DOES succeed (a general surface can span it), which is recorded in `notes` rather than substituted as the AFTER shape: this fixture is specifically about the planar builder's own refusal.",
    build: (b) => {
      const p1: [number, number, number] = [0, 0, 0];
      const p2: [number, number, number] = [10, 0, 0];
      const p3: [number, number, number] = [10, 10, 0];
      const p4: [number, number, number] = [0, 10, 0];
      const before = call(b, "polygon", [p1, p2, p3, p4]);
      const p3Moved: [number, number, number] = [10, 10, 5];
      let rejectionReason = "unexpected: polygon() accepted the non-planar quad";
      let result: unknown = null;
      try {
        call(b, "polygon", [p1, p2, p3Moved, p4]);
      } catch (error) {
        rejectionReason = `polygon(p1,p2,p3Moved,p4) — ${(error as Error).message} (filledFace() on the equivalent wireLoop DOES succeed as a non-planar BSPLINE_SURFACE — that is a different, legal edit, not this one)`;
      }
      return { operands: [{ role: "operand-a-step", shape: before }], result, rejectionReason };
    },
  },
  {
    id: "move-vertex-makes-pentagon-nonplanar-rejected",
    family: "move-vertex",
    kind: "move-vertex",
    outcome: "rejected",
    tolerance: "topology-exact",
    notes: "The non-planar-move rejection against a five-sided BEFORE face rather than a quad, checking the rejection isn't an artefact of exactly four corners.",
    build: (b) => {
      const pts: [number, number, number][] = [[0, 0, 0], [10, 0, 0], [13, 8, 0], [5, 13, 0], [-3, 8, 0]];
      const before = call(b, "polygon", pts);
      const moved: [number, number, number][] = [pts[0]!, pts[1]!, [13, 8, 6], pts[3]!, pts[4]!];
      let rejectionReason = "unexpected: polygon() accepted the non-planar pentagon";
      let result: unknown = null;
      try {
        call(b, "polygon", moved);
      } catch (error) {
        rejectionReason = `polygon(moved-pentagon) — ${(error as Error).message}`;
      }
      return { operands: [{ role: "operand-a-step", shape: before }], result, rejectionReason };
    },
  },
  //#endregion 🚫makes non-planar (rejected)

  //#region 🚫self-intersects (rejected)
  {
    id: "move-vertex-self-intersect-bowtie-rejected",
    family: "move-vertex",
    kind: "move-vertex",
    outcome: "rejected",
    tolerance: "topology-exact",
    notes: "Moving corner 2 to where corner 3 used to be (and vice versa, by construction) turns a simple square into a self-crossing bowtie. `polygon()` does NOT throw on the in-memory attempt — MEASURED there, its own `measureArea()` collapses to a value indistinguishable from zero, the two triangular lobes' signed contributions cancelling. But this is exactly lesson 1 in miniature: measured OFF `expected.step` — the form a real consumer sees — the picture is different again. OCCT's STEP writer silently splits the self-intersecting single face into TWO separate valid triangular faces meeting at the crossing point (2 faces, 2 shells, 5 vertices — one MORE than the 4 input points, the crossing point itself — 6 edges), whose combined AREA is 50, exactly HALF the original square's 100, not zero at all. Three measurements, three different pictures of the same illegal move: `false`-free in-memory success, a cancelled zero-ish signed area, and a round-tripped shape with real positive area the input geometry never specified. All three are recorded.",
    build: (b) => {
      const before = call(b, "polygon", [[0, 0, 0], [10, 0, 0], [10, 10, 0], [0, 10, 0]]);
      const bowtie = call(b, "polygon", [[0, 0, 0], [10, 10, 0], [10, 0, 0], [0, 10, 0]]);
      const preExportArea = call(b, "measureArea", bowtie) as number;
      return { operands: [{ role: "operand-a-step", shape: before }], result: bowtie, rejectionReason: `polygon() with corners 2 and 3 swapped did not throw; its PRE-EXPORT in-memory measureArea() is ${preExportArea} (numerically zero — a self-intersecting loop whose signed area cancels), but expected.metrics.json's area is measured from the RE-IMPORTED expected.step, where OCCT's STEP writer has already split the self-intersecting face into two separate triangles — see notes` };
    },
  },
  {
    id: "move-vertex-self-intersect-bowtie-rejected-large",
    family: "move-vertex",
    kind: "move-vertex",
    outcome: "rejected",
    tolerance: "topology-exact",
    notes: "The self-intersecting bowtie move at 5e2 scale — MEASURED to split the same way (2 faces, area = half the original square's, a new crossing-point vertex), confirming the STEP-writer split isn't an artefact of the small fixture's particular coordinates.",
    build: (b) => {
      const before = call(b, "polygon", [[0, 0, 0], [500, 0, 0], [500, 500, 0], [0, 500, 0]]);
      const bowtie = call(b, "polygon", [[0, 0, 0], [500, 500, 0], [500, 0, 0], [0, 500, 0]]);
      const preExportArea = call(b, "measureArea", bowtie) as number;
      return { operands: [{ role: "operand-a-step", shape: before }], result: bowtie, rejectionReason: `polygon() with corners 2 and 3 swapped did not throw; its PRE-EXPORT in-memory measureArea() is ${preExportArea} (numerically zero), but expected.metrics.json's area is measured from the RE-IMPORTED expected.step, where OCCT's STEP writer has already split the self-intersecting face into two separate triangles — see notes` };
    },
  },
  //#endregion 🚫self-intersects (rejected)
];
//#endregion 🧪️Recipes
