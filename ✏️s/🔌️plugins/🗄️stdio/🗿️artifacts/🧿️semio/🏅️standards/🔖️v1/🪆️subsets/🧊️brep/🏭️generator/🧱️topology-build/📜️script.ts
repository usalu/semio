#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🧱️ BRep fixture recipes — the `topology-build` family: `create-vertex`/`create-edge`/`create-face`/
// `create-shell`/`create-solid`, each as a BEFORE/AFTER pair where the AFTER adds exactly one new
// entity of the named kind. `../📜️script.ts` runs each `build`, exports BEFORE and (where legal) AFTER
// as STEP, re-imports what it wrote and measures THAT.
//
// brepjs has no low-level Euler operator ("insert this vertex into this solid") — this subset's own
// mutation vocabulary is native to semio's document IR, not to any third-party kernel's API. Every
// recipe below therefore represents the EFFECT of the edit as two independently-built, kernel-valid
// B-Reps: a topology missing the new entity, and the same topology WITH it, exactly as OCCT's own
// builders (`vertex`, `wireLoop`+`face`, `sewShells`, `solid`, `compound`) would construct either.
//
// @see ../📜️script.ts — the generator that runs these
// @see ../../🧬️schema/🧬️mutations/🦀️.rs — the 13-verb vocabulary this corpus exercises

//#endregion 🧲️Header

//#region 🔌️Adapters
import { type Kernel, type Recipe, call } from "../📜️script.ts";
//#endregion 🔌️Adapters

//#region 🧪️Recipes
/** 🧱️ The `topology-build` recipes: one BEFORE/AFTER pair per `create-*` kind, at more than one scale. */
export const RECIPES: readonly Recipe[] = [
  //#region 🏗️create-vertex
  {
    id: "topology-build-create-vertex-apex-above-box-small",
    family: "topology-build",
    kind: "create-vertex",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "Adds a single loose apex vertex above a box's top face, as a `compound(solid, vertex)` — the smallest possible `create-vertex` edit: a new 0D entity, no incident edges, no effect on the solid it accompanies.",
    build: (b) => {
      const box = call(b, "box", 10, 10, 10);
      const apex = call(b, "vertex", [5, 5, 15]);
      return { operands: [{ role: "operand-a-step", shape: box }], result: call(b, "compound", [box, apex]) };
    },
  },
  {
    id: "topology-build-create-vertex-apex-above-box-large",
    family: "topology-build",
    kind: "create-vertex",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "The same apex-vertex edit at 1e3 scale — the tessellation tolerance for this fixture is resolved from ITS OWN bounding-box diagonal, so a fixed absolute tolerance would either over- or under-mesh it relative to the small variant.",
    build: (b) => {
      const box = call(b, "box", 1000, 1000, 1000);
      const apex = call(b, "vertex", [500, 500, 1500]);
      return { operands: [{ role: "operand-a-step", shape: box }], result: call(b, "compound", [box, apex]) };
    },
  },
  {
    id: "topology-build-create-vertex-three-loose-points",
    family: "topology-build",
    kind: "create-vertex",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "THREE `create-vertex` edits folded into one AFTER state — exercises a compound whose vertex count grows by more than one per edit, which is the case a fixture that only ever adds a single vertex would never touch.",
    build: (b) => {
      const box = call(b, "box", 20, 20, 20);
      const v1 = call(b, "vertex", [-5, 10, 10]);
      const v2 = call(b, "vertex", [25, 10, 10]);
      const v3 = call(b, "vertex", [10, 10, 25]);
      return { operands: [{ role: "operand-a-step", shape: box }], result: call(b, "compound", [box, v1, v2, v3]) };
    },
  },
  //#endregion 🏗️create-vertex

  //#region 🖇️create-edge
  {
    id: "topology-build-create-edge-diagonal-across-square-small",
    family: "topology-build",
    kind: "create-edge",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "Adds a diagonal edge across a square face, as a `compound(face, edge)` — the diagonal is NOT part of the face's boundary wire, so the face itself is untouched; only the compound's edge count grows. Mirrors this subset's own oracle scenario `create-edge/adds-a-diagonal-edge-across-the-square`.",
    build: (b) => {
      const square = call(b, "polygon", [[0, 0, 0], [10, 0, 0], [10, 10, 0], [0, 10, 0]]);
      const diagonal = call(b, "line", [0, 0, 0], [10, 10, 0]);
      return { operands: [{ role: "operand-a-step", shape: square }], result: call(b, "compound", [square, diagonal]) };
    },
  },
  {
    id: "topology-build-create-edge-diagonal-across-square-large",
    family: "topology-build",
    kind: "create-edge",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "The diagonal-edge edit at 5e2 scale.",
    build: (b) => {
      const square = call(b, "polygon", [[0, 0, 0], [500, 0, 0], [500, 500, 0], [0, 500, 0]]);
      const diagonal = call(b, "line", [0, 0, 0], [500, 500, 0]);
      return { operands: [{ role: "operand-a-step", shape: square }], result: call(b, "compound", [square, diagonal]) };
    },
  },
  {
    id: "topology-build-create-edge-chord-across-disk",
    family: "topology-build",
    kind: "create-edge",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "The same edit against a CURVED boundary: a chord edge added across a circular disk face. Where the square case adds a straight edge to a face whose own boundary is also straight, this pairs a straight new edge with a face whose boundary is a single closed circular edge — a different combination of the curve types the added entity and its host can carry.",
    build: (b) => {
      const circleEdge = call(b, "circle", 8);
      const circleWire = call(b, "wire", [circleEdge]);
      const disk = call(b, "face", circleWire);
      const chord = call(b, "line", [-8, 0, 0], [4, Math.sqrt(48), 0]);
      return { operands: [{ role: "operand-a-step", shape: disk }], result: call(b, "compound", [disk, chord]) };
    },
  },
  //#endregion 🖇️create-edge

  //#region 🔷create-face
  {
    id: "topology-build-create-face-closes-open-box-small",
    family: "topology-build",
    kind: "create-face",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "BEFORE is an OPEN shell — a box missing its +z cap face, sewn from the other 5. AFTER adds the 6th face and welds all six into a closed, valid solid via `solid()`. This is `create-face` at the point it matters most: the moment a shell BECOMES a solid.",
    build: (b) => {
      const box = call(b, "box", 10, 10, 10) as unknown[];
      const faces = call(b, "getFaces", box) as unknown[];
      const openShell = call(b, "sewShells", faces.slice(1));
      const closedSolid = call(b, "solid", faces);
      return { operands: [{ role: "operand-a-step", shape: openShell }], result: closedSolid };
    },
  },
  {
    id: "topology-build-create-face-closes-open-box-large",
    family: "topology-build",
    kind: "create-face",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "The open-shell-to-solid edit at 2e3 scale.",
    build: (b) => {
      const box = call(b, "box", 2000, 2000, 2000) as unknown[];
      const faces = call(b, "getFaces", box) as unknown[];
      const openShell = call(b, "sewShells", faces.slice(1));
      const closedSolid = call(b, "solid", faces);
      return { operands: [{ role: "operand-a-step", shape: openShell }], result: closedSolid };
    },
  },
  {
    id: "topology-build-create-face-caps-open-cylinder",
    family: "topology-build",
    kind: "create-face",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "The same edit against a CURVED shell: a cylinder missing its top disc (lateral surface + bottom disc only, an open shell), capped by adding the third face. Exercises `create-face` where the new face closes a shell that already contains an analytic non-planar face, unlike the all-planar box case.",
    build: (b) => {
      const cyl = call(b, "cylinder", 6, 15) as unknown[];
      const faces = call(b, "getFaces", cyl) as unknown[];
      const openShell = call(b, "sewShells", faces.slice(0, 2));
      const closedSolid = call(b, "solid", faces);
      return { operands: [{ role: "operand-a-step", shape: openShell }], result: closedSolid };
    },
  },
  //#endregion 🔷create-face

  //#region 🐚create-shell
  {
    id: "topology-build-create-shell-second-shell-flipped-sense",
    family: "topology-build",
    kind: "create-shell",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "AFTER adds a SECOND shell built from the same box's own faces with every face's orientation flipped, alongside the original solid in a compound — mirrors this subset's own oracle scenario `create-shell/adds-a-second-shell-that-reuses-the-face-with-flipped-sense`. The two shells occupy the same geometric locus but are DISTINCT topological entities: `getShells` on the compound counts both.",
    build: (b) => {
      const box = call(b, "box", 10, 10, 10);
      const faces = call(b, "getFaces", box) as unknown[];
      const flipped = faces.map((f) => call(b, "flipFaceOrientation", f));
      const secondShell = call(b, "sewShells", flipped, true);
      return { operands: [{ role: "operand-a-step", shape: box }], result: call(b, "compound", [box, secondShell]) };
    },
  },
  {
    id: "topology-build-create-shell-second-shell-flipped-sense-large",
    family: "topology-build",
    kind: "create-shell",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "The flipped-sense second-shell edit at 5e2 scale.",
    build: (b) => {
      const box = call(b, "box", 500, 500, 500);
      const faces = call(b, "getFaces", box) as unknown[];
      const flipped = faces.map((f) => call(b, "flipFaceOrientation", f));
      const secondShell = call(b, "sewShells", flipped, true);
      return { operands: [{ role: "operand-a-step", shape: box }], result: call(b, "compound", [box, secondShell]) };
    },
  },
  {
    id: "topology-build-create-shell-from-cylinder-faces",
    family: "topology-build",
    kind: "create-shell",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "The same second-shell edit against a cylinder rather than a box, so the added shell carries a non-planar face — a different shape family from the two box-based `create-shell` fixtures above.",
    build: (b) => {
      const cyl = call(b, "cylinder", 6, 15);
      const faces = call(b, "getFaces", cyl) as unknown[];
      const flipped = faces.map((f) => call(b, "flipFaceOrientation", f));
      const secondShell = call(b, "sewShells", flipped, true);
      return { operands: [{ role: "operand-a-step", shape: cyl }], result: call(b, "compound", [cyl, secondShell]) };
    },
  },
  //#endregion 🐚create-shell

  //#region 🧊create-solid
  {
    id: "topology-build-create-solid-second-disjoint-solid-small",
    family: "topology-build",
    kind: "create-solid",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "AFTER adds a second, spatially disjoint solid alongside the first in a compound — the plainest `create-solid` edit: a wholly new entity with no interaction with the existing one.",
    build: (b) => {
      const boxA = call(b, "box", 10, 10, 10);
      const boxB = call(b, "translate", call(b, "box", 6, 6, 6), [30, 0, 0]);
      return { operands: [{ role: "operand-a-step", shape: boxA }], result: call(b, "compound", [boxA, boxB]) };
    },
  },
  {
    id: "topology-build-create-solid-second-disjoint-solid-large",
    family: "topology-build",
    kind: "create-solid",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "The second-disjoint-solid edit at 1e3 scale.",
    build: (b) => {
      const boxA = call(b, "box", 1000, 1000, 1000);
      const boxB = call(b, "translate", call(b, "box", 600, 600, 600), [3000, 0, 0]);
      return { operands: [{ role: "operand-a-step", shape: boxA }], result: call(b, "compound", [boxA, boxB]) };
    },
  },
  {
    id: "topology-build-create-solid-void-boundary-as-solid",
    family: "topology-build",
    kind: "create-solid",
    outcome: "applied",
    tolerance: "topology-exact",
    notes: "AFTER treats an already-cut internal void's own boundary as a SECOND solid alongside the hollowed body — mirrors this subset's own oracle scenario `create-solid/adds-a-second-solid-that-treats-the-shell-as-a-void`: the cavity that `cut` bored out of the outer box is independently rebuilt as its own solid and added to the compound, rather than staying merely a hole.",
    build: (b) => {
      const outer = call(b, "box", 30, 30, 30);
      const cavity = call(b, "translate", call(b, "box", 8, 8, 8), [11, 11, 11]);
      const hollowed = call(b, "cut", outer, cavity);
      const cavitySolid = call(b, "translate", call(b, "box", 8, 8, 8), [11, 11, 11]);
      return { operands: [{ role: "operand-a-step", shape: hollowed }], result: call(b, "compound", [hollowed, cavitySolid]) };
    },
  },
  //#endregion 🧊create-solid
];
//#endregion 🧪️Recipes
