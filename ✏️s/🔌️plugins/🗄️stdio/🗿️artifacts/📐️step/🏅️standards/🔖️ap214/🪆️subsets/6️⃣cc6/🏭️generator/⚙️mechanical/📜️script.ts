#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// ⚙️ BRep fixture recipes — the `mechanical` family.
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

//#region 🧰️Finders
/** 🔎️ Edges whose local direction is `dir` — used to fillet/chamfer a SELECTED rim rather than every edge a subtractive chain produced. */
const edgesInDirection = (b: Kernel, dir: readonly number[]): unknown => (call(b, "edgeFinder") as { inDirection: (d: readonly number[]) => unknown }).inDirection(dir);
/** 🔎️ Faces whose normal is `dir` — the selector `shell` needs to say which face to remove. */
const facesInDirection = (b: Kernel, dir: readonly number[]): unknown => (call(b, "faceFinder") as { inDirection: (d: readonly number[]) => unknown }).inDirection(dir);
//#endregion 🧰️Finders

//#region 🧪️Recipes
/** ⚙️ The `mechanical` recipes. */
export const RECIPES: readonly Recipe[] = [
  {
    id: "mechanical-fixture-plate",
    family: "mechanical",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A fixture plate: four patterned through-holes and a central pocket, produced by a subtractive sequence rather than a single operation. Repeated Boolean chains are where intermediate-state defects accumulate.",
    build: (b) => {
      let plate = call(b, "box", 60, 40, 8);
      for (const [x, y] of [[8, 8], [52, 8], [8, 32], [52, 32]] as const) plate = call(b, "cut", plate, call(b, "translate", call(b, "cylinder", 2.5, 20), [x, y, -6]));
      plate = call(b, "cut", plate, call(b, "translate", call(b, "box", 24, 16, 4), [18, 12, 4]));
      return { operands: [{ role: "operand-a-step", shape: call(b, "box", 60, 40, 8) }], result: plate };
    },
  },
  {
    id: "mechanical-pipe-manifold",
    family: "mechanical",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A manifold: a main bore with a branch bore meeting it at right angles inside the body. Intersecting internal passages are the case where a lost cavity leaves the outer surface — and therefore the Hausdorff distance — untouched.",
    build: (b) => {
      const body = call(b, "box", 60, 30, 30);
      let manifold = call(b, "cut", body, call(b, "translate", call(b, "rotate", call(b, "cylinder", 8, 80), 90, { at: [0, 0, 0], axis: [0, 1, 0] }), [-10, 15, 15]));
      manifold = call(b, "cut", manifold, call(b, "translate", call(b, "cylinder", 5, 60), [30, 15, -10]));
      return { operands: [{ role: "operand-a-step", shape: body }], result: manifold };
    },
  },
  {
    id: "mechanical-ribbed-enclosure",
    family: "mechanical",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A shelled enclosure with internal ribs and vent slots — a nested cavity crossed by thin partitions, and the closest this corpus comes to a real housing.",
    build: (b) => {
      const outer = call(b, "box", 50, 30, 20);
      let enclosure = call(b, "cut", outer, call(b, "translate", call(b, "box", 46, 26, 18), [2, 2, 2]));
      for (const x of [12, 24, 36]) enclosure = call(b, "fuse", enclosure, call(b, "translate", call(b, "box", 2, 26, 14), [x, 2, 2]));
      for (const y of [8, 16, 24]) enclosure = call(b, "cut", enclosure, call(b, "translate", call(b, "box", 60, 2, 3), [-5, y, 17]));
      return { operands: [{ role: "operand-a-step", shape: outer }], result: enclosure };
    },
  },
  {
    id: "mechanical-filleted-bracket",
    family: "mechanical",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "An angled bracket built by a CHAIN: fuse the upright and the foot, then two angled cuts. Like the other mechanical fixtures it exports only its BASE — the fused blank — because a chain is not decomposable into two operand files, and declaring `operand-a`/`operand-b` here would promise a reproduction path that does not exist. Filleting is deliberately left out where the kernel refuses it, because a fixture that quietly skipped its own defining feature would be worse than none.",
    build: (b) => {
      const upright = call(b, "box", 8, 40, 50);
      const foot = call(b, "box", 40, 40, 8);
      let bracket = call(b, "fuse", upright, foot);
      bracket = call(b, "cut", bracket, call(b, "translate", call(b, "rotate", call(b, "cylinder", 4, 60), 25, { at: [0, 0, 0], axis: [0, 1, 0] }), [20, 20, -10]));
      bracket = call(b, "cut", bracket, call(b, "translate", call(b, "cylinder", 3, 30), [4, 30, 20]));
      // 🧩️ONLY the base is exported, like every other `mechanical` chain fixture. Exporting `upright`
      // and `foot` as operand-a/operand-b made this look like a decomposable two-operand Boolean while
      // silently folding in two further cutters that no fixture file carries — so nobody could
      // re-derive `expected.step` from the declared operands. A chain declares its base and says so.
      return { operands: [{ role: "operand-a-step", shape: call(b, "fuse", call(b, "box", 8, 40, 50), call(b, "box", 40, 40, 8)) }], result: bracket };
    },
  },
  {
    id: "mechanical-enclosure-boss-vented",
    family: "mechanical",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A CHAIN beyond `mechanical-ribbed-enclosure`: hollow box, 3 ribs, 3 vent slots through the front wall, 4 bosses on the floor with pilot holes, 4 through mounting holes in the solid corner posts, then `fillet` on the SELECTED vertical corner edges (`edgeFinder().inDirection([0,0,1])`), not every edge — 17 operations total. Exercises real edge-selected filleting after a long subtractive/additive chain, which is where a kernel's edge-blend solver meets the most topology at once. MEASURED: volume 27840.1 mm3, area 22999.6 mm2, 145 faces / 504 edges / 356 vertices, 1 solid, mesh genus 12, self step-mesh-compare normalizedSymmetricDifferenceVolume 0.",
    build: (b) => {
      const outer = call(b, "box", 60, 40, 25);
      let enclosure = call(b, "cut", outer, call(b, "translate", call(b, "box", 55, 35, 20), [2.5, 2.5, 2.5]));
      for (const x of [15, 30, 45]) enclosure = call(b, "fuse", enclosure, call(b, "translate", call(b, "box", 2.5, 35, 20), [x, 2.5, 2.5]));
      for (const z of [6, 12, 18]) enclosure = call(b, "cut", enclosure, call(b, "translate", call(b, "box", 40, 5, 2), [10, -1, z]));
      for (const [x, y] of [[8, 8], [52, 8], [8, 32], [52, 32]] as const) enclosure = call(b, "fuse", enclosure, call(b, "translate", call(b, "cylinder", 4, 12), [x, y, 2.5]));
      for (const [x, y] of [[8, 8], [52, 8], [8, 32], [52, 32]] as const) enclosure = call(b, "cut", enclosure, call(b, "translate", call(b, "cylinder", 1.5, 14), [x, y, 1.5]));
      for (const [x, y] of [[1.25, 1.25], [58.75, 1.25], [1.25, 38.75], [58.75, 38.75]] as const) enclosure = call(b, "cut", enclosure, call(b, "translate", call(b, "cylinder", 1, 27), [x, y, -1]));
      enclosure = call(b, "fillet", enclosure, edgesInDirection(b, [0, 0, 1]), 1);
      return { operands: [{ role: "operand-a-step", shape: call(b, "box", 60, 40, 25) }], result: enclosure };
    },
  },
  {
    id: "mechanical-pipe-manifold-reducer-branch",
    family: "mechanical",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "Beyond `mechanical-pipe-manifold`: a STEPPED main bore (Ø16 cylinder, a reducing cone, Ø8 cylinder, built as one `fuseAll` tool then cut in one pass), a vertical branch, a 30° angled branch, and a bolt-hole ring at each flange face via `circularPattern`. Intersecting internal passages leave the outer surface untouched, which is exactly the case where a lost cavity would go undetected by a surface-only check. MEASURED: volume 67910.6 mm3, area 16541.7 mm2, 20 faces / 48 edges / 30 vertices, 1 solid, mesh genus 3, self step-mesh-compare normalizedSymmetricDifferenceVolume 0.",
    build: (b) => {
      const body = call(b, "box", 90, 30, 30);
      const seg1 = call(b, "translate", call(b, "rotate", call(b, "cylinder", 8, 50), 90, { at: [0, 0, 0], axis: [0, 1, 0] }), [-5, 15, 15]);
      const reducer = call(b, "translate", call(b, "rotate", call(b, "cone", 8, 4, 10), 90, { at: [0, 0, 0], axis: [0, 1, 0] }), [45, 15, 15]);
      const seg2 = call(b, "translate", call(b, "rotate", call(b, "cylinder", 4, 40), 90, { at: [0, 0, 0], axis: [0, 1, 0] }), [55, 15, 15]);
      const mainTool = call(b, "fuseAll", [seg1, reducer, seg2]);
      let manifold = call(b, "cut", body, mainTool);
      manifold = call(b, "cut", manifold, call(b, "translate", call(b, "cylinder", 4, 20), [20, 15, 10]));
      manifold = call(b, "cut", manifold, call(b, "translate", call(b, "rotate", call(b, "cylinder", 3, 25), 30, { at: [0, 0, 0], axis: [1, 0, 0] }), [70, 15, -5]));
      const boltLeft = call(b, "circularPattern", call(b, "translate", call(b, "cylinder", 2, 10), [-2, 15, 26]), [1, 0, 0], 4, 360, [0, 15, 15]);
      const boltRight = call(b, "circularPattern", call(b, "translate", call(b, "cylinder", 2, 10), [88, 15, 26]), [1, 0, 0], 4, 360, [90, 15, 15]);
      manifold = call(b, "cutAll", manifold, [boltLeft, boltRight]);
      return { operands: [{ role: "operand-a-step", shape: body }], result: manifold };
    },
  },
  {
    id: "mechanical-fixture-plate-slotted",
    family: "mechanical",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "Beyond `mechanical-fixture-plate`: 4 counterbored holes (two concentric cylinders `cutAll`'d together per hole), a central pocket, two stadium-shaped slots (`fuseAll` of two cylinders and a connecting box, then cut as one tool), and a 3×2 fastener grid produced by `rectangularPattern` rather than a manual loop. 21 operations total, none of the six feature groups overlapping in plan so the case isolates each feature's contribution. MEASURED: volume 38071.2 mm3, area 13720.9 mm2, 41 faces / 107 edges / 71 vertices, 1 solid, mesh genus 12, self step-mesh-compare normalizedSymmetricDifferenceVolume 0.",
    build: (b) => {
      let plate = call(b, "box", 90, 50, 10);
      for (const [x, y] of [[10, 10], [80, 10], [10, 40], [80, 40]] as const) {
        const through = call(b, "translate", call(b, "cylinder", 2.5, 14), [x, y, -2]);
        const counterbore = call(b, "translate", call(b, "cylinder", 5, 4), [x, y, 7]);
        plate = call(b, "cutAll", plate, [through, counterbore]);
      }
      plate = call(b, "cut", plate, call(b, "translate", call(b, "box", 30, 20, 4), [30, 15, 6]));
      for (const y of [22, 32]) {
        const end1 = call(b, "translate", call(b, "cylinder", 3, 14), [55, y, -2]);
        const end2 = call(b, "translate", call(b, "cylinder", 3, 14), [75, y, -2]);
        const mid = call(b, "translate", call(b, "box", 20, 6, 14), [55, y - 3, -2]);
        plate = call(b, "cut", plate, call(b, "fuseAll", [end1, end2, mid]));
      }
      const fastener = call(b, "translate", call(b, "cylinder", 1.5, 14), [40, 5, -2]);
      const fastenerGrid = call(b, "rectangularPattern", fastener, { xDir: [1, 0, 0], xCount: 3, xSpacing: 12, yDir: [0, 1, 0], yCount: 2, ySpacing: 5 });
      plate = call(b, "cut", plate, fastenerGrid);
      return { operands: [{ role: "operand-a-step", shape: call(b, "box", 90, 50, 10) }], result: plate };
    },
  },
  {
    id: "mechanical-skewed-bracket-gusseted",
    family: "mechanical",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A CHAIN: fuse an upright and a foot, build ONE gusset as a `polygon` + `extrude` triangular plate, `mirror` it to the opposite side and fuse both, then three cutters at COMPOUND angles (two sequential `rotate` calls each, around different axes) plus two vertical mounting holes. 10 operations. The compound-angle cutters take every analytic shortcut off the table the way a single-axis skew cannot. MEASURED: volume 27550.4 mm3, area 10073.8 mm2, 31 faces / 81 edges / 51 vertices, 1 solid, mesh genus 2, self step-mesh-compare normalizedSymmetricDifferenceVolume 0.",
    build: (b) => {
      const upright = call(b, "box", 8, 40, 50);
      const foot = call(b, "box", 40, 40, 8);
      let bracket = call(b, "fuse", upright, foot);
      const gusset1 = call(b, "extrude", call(b, "polygon", [[8, 5, 8], [8, 5, 30], [30, 5, 8]]), [0, 5, 0]);
      const gusset2 = call(b, "mirror", gusset1, { normal: [0, 1, 0], at: [0, 20, 0] });
      bracket = call(b, "fuse", bracket, gusset1);
      bracket = call(b, "fuse", bracket, gusset2);
      bracket = call(b, "cut", bracket, call(b, "translate", call(b, "rotate", call(b, "rotate", call(b, "cylinder", 3, 30), 20, { axis: [0, 1, 0] }), 15, { axis: [1, 0, 0] }), [4, 20, 35]));
      bracket = call(b, "cut", bracket, call(b, "translate", call(b, "rotate", call(b, "cylinder", 2.5, 20), 35, { axis: [1, 0, 0] }), [4, 10, 15]));
      bracket = call(b, "cut", bracket, call(b, "translate", call(b, "rotate", call(b, "cylinder", 2, 25), -25, { axis: [0, 1, 0] }), [4, 35, 10]));
      bracket = call(b, "cutAll", bracket, [call(b, "translate", call(b, "cylinder", 2.5, 10), [10, 10, -1]), call(b, "translate", call(b, "cylinder", 2.5, 10), [30, 30, -1])]);
      return { operands: [{ role: "operand-a-step", shape: call(b, "fuse", call(b, "box", 8, 40, 50), call(b, "box", 40, 40, 8)) }], result: bracket };
    },
  },
  {
    id: "mechanical-valve-body",
    family: "mechanical",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A spool-shaped body built by `revolve`-ing an 8-point profile around Z (not a primitive), then a through main bore, a perpendicular branch port meeting it inside the body, and a STEPPED counterbore at the top flange (two concentric cylinders) standing in for a valve seat's concentric sealing faces. 6 operations. The revolved body means every downstream cut lands on a genuinely doubly-curved surface, not an axis-aligned box. MEASURED: volume 19445.3 mm3, area 7387.3 mm2, 14 faces / 28 edges / 18 vertices, 1 solid, mesh genus 3, self step-mesh-compare normalizedSymmetricDifferenceVolume 0.",
    build: (b) => {
      const profile = call(b, "polygon", [[2, 0, 0], [18, 0, 0], [18, 0, 6], [10, 0, 6], [10, 0, 34], [18, 0, 34], [18, 0, 40], [2, 0, 40]]);
      let valve = call(b, "revolve", profile);
      valve = call(b, "cut", valve, call(b, "translate", call(b, "cylinder", 1.5, 42), [0, 0, -1]));
      valve = call(b, "cut", valve, call(b, "translate", call(b, "rotate", call(b, "cylinder", 3, 40), 90, { at: [0, 0, 0], axis: [0, 1, 0] }), [-20, 0, 20]));
      valve = call(b, "cutAll", valve, [call(b, "translate", call(b, "cylinder", 8, 2), [0, 0, 38]), call(b, "translate", call(b, "cylinder", 5, 5), [0, 0, 35])]);
      return { operands: [{ role: "operand-a-step", shape: call(b, "revolve", call(b, "polygon", [[2, 0, 0], [18, 0, 0], [18, 0, 6], [10, 0, 6], [10, 0, 34], [18, 0, 34], [18, 0, 40], [2, 0, 40]])) }], result: valve };
    },
  },
  {
    id: "mechanical-nested-shell-channels",
    family: "mechanical",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A NESTED shell: an outer box hollowed by the real `shell()` operation (not a manual cut), a second smaller box independently `shell()`-ed and seated inside the first cavity, two ribs bridging the gap between the two shells, then two horizontal channels piercing BOTH walls into the inner cavity. 6 shape operations — 1 solid, 34 faces, 89 edges, MEASURED. ABANDONED building the union with `fuseAll([outerShell, innerShell, rib1, rib2])`: MEASURED to leave 4 separate `getSolids()` entries in the result despite correct total volume — `fuseAll` over 3+ shapes does not merge touching/overlapping pieces into one manifold solid the way sequential pairwise `fuse` does. Fixed by folding the four shapes in with 3 sequential `fuse` calls instead, which measured back to a single solid. This is the direct test of thin-partition and double-wall handling the goal calls out — a lost inner cavity here leaves the outer shell completely untouched. MEASURED: volume 38875.6 mm3, area 25935.7 mm2, 34 faces / 89 edges / 56 vertices, 1 solid, mesh genus 7, self step-mesh-compare normalizedSymmetricDifferenceVolume 0.",
    build: (b) => {
      const outer = call(b, "box", 70, 50, 40);
      const outerShell = call(b, "shell", outer, facesInDirection(b, [0, 0, 1]), 3);
      const innerBox = call(b, "translate", call(b, "box", 40, 25, 22), [15, 12, 3]);
      const innerShell = call(b, "shell", innerBox, facesInDirection(b, [0, 0, 1]), 2);
      const rib1 = call(b, "translate", call(b, "box", 12, 25, 10), [3, 12, 5]);
      const rib2 = call(b, "translate", call(b, "box", 40, 9, 10), [15, 3, 5]);
      let nested = call(b, "fuse", outerShell, innerShell);
      nested = call(b, "fuse", nested, rib1);
      nested = call(b, "fuse", nested, rib2);
      const channel1 = call(b, "translate", call(b, "rotate", call(b, "cylinder", 3, 80), 90, { at: [0, 0, 0], axis: [0, 1, 0] }), [-5, 25, 15]);
      const channel2 = call(b, "translate", call(b, "rotate", call(b, "cylinder", 2.5, 60), 90, { at: [0, 0, 0], axis: [1, 0, 0] }), [35, -5, 15]);
      nested = call(b, "cutAll", nested, [channel1, channel2]);
      return { operands: [{ role: "operand-a-step", shape: call(b, "shell", call(b, "box", 70, 50, 40), facesInDirection(b, [0, 0, 1]), 3) }], result: nested };
    },
  },
  {
    id: "mechanical-block-fifteen-cuts",
    family: "mechanical",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A machined block produced by a LONG subtractive sequence: 16 through-holes cut ONE AT A TIME (not batched with `cutAll`) in a 4×4 grid, then a shallow pocket. 17 sequential operations — the case this goal names explicitly, where intermediate-state defects accumulate one boolean at a time. ABANDONED a trailing `chamfer(block, 0.5)` over all edges: MEASURED kernel failure `CHAMFER_FAILED: [object WebAssembly.Exception]` at 88 edges — the isolated 16-hole case from the qualification spike chamfered fine, so the failure is specific to this combination of hole-grid and pocket edges, not to chamfering-after-many-cuts in general. Recorded as a kernel finding rather than silently dropped. MEASURED: volume 64938.2 mm3, area 16531.7 mm2, 29 faces / 88 edges / 58 vertices, 1 solid, mesh genus 16, self step-mesh-compare normalizedSymmetricDifferenceVolume 0.",
    build: (b) => {
      let block = call(b, "box", 60, 40, 30);
      for (const x of [8, 24, 40, 56]) for (const y of [6, 15, 24, 33]) block = call(b, "cut", block, call(b, "translate", call(b, "cylinder", 2, 34), [x, y, -2]));
      block = call(b, "cut", block, call(b, "translate", call(b, "box", 20, 12, 5), [20, 14, 25]));
      return { operands: [{ role: "operand-a-step", shape: call(b, "box", 60, 40, 30) }], result: block };
    },
  },
  {
    id: "mechanical-multi-union-trim-drilled",
    family: "mechanical",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "Four DISTINCT primitives (a base plate, two cylinders and a cone boss) fused into one blank, a single planar TRIM cut that flattens whichever components overhang it, then a final round of drilling. 6 shape operations, exercising the multi-component-union-then-trim-then-drill pattern the goal calls out as distinct from a plain subtractive chain. ABANDONED `fuseAll([base, post1, post2, boss])` for the union: MEASURED to leave 7 separate `getSolids()` entries even though the total volume matched a correctly-merged single solid exactly — the same `fuseAll`-does-not-merge defect found in `mechanical-nested-shell-channels`, here with all 4 primitives genuinely volume-overlapping (not just touching). Fixed with 3 sequential `fuse` calls, which measured back to 1 solid. MEASURED: volume 22067.7 mm3, area 6890.0 mm2, 18 faces / 30 edges / 20 vertices, 1 solid, mesh genus 0, self step-mesh-compare normalizedSymmetricDifferenceVolume 0.",
    build: (b) => {
      const base = call(b, "box", 40, 40, 10);
      const post1 = call(b, "translate", call(b, "cylinder", 8, 20), [10, 10, 8]);
      const post2 = call(b, "translate", call(b, "cylinder", 6, 15), [30, 30, 8]);
      const boss = call(b, "translate", call(b, "cone", 10, 4, 18), [10, 30, 8]);
      let union = call(b, "fuse", base, post1);
      union = call(b, "fuse", union, post2);
      union = call(b, "fuse", union, boss);
      union = call(b, "cut", union, call(b, "translate", call(b, "box", 100, 100, 100), [-30, -30, 24]));
      union = call(b, "cutAll", union, [call(b, "translate", call(b, "cylinder", 2, 14), [10, 10, -2]), call(b, "translate", call(b, "cylinder", 2, 14), [30, 30, -2]), call(b, "translate", call(b, "cylinder", 2, 14), [10, 30, -2])]);
      return { operands: [{ role: "operand-a-step", shape: call(b, "fuse", call(b, "fuse", call(b, "fuse", call(b, "box", 40, 40, 10), call(b, "translate", call(b, "cylinder", 8, 20), [10, 10, 8])), call(b, "translate", call(b, "cylinder", 6, 15), [30, 30, 8])), call(b, "translate", call(b, "cone", 10, 4, 18), [10, 30, 8])) }], result: union };
    },
  },
  {
    id: "mechanical-housing-threaded-boss",
    family: "mechanical",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A housing with a boss carrying a REAL external thread: `thread({radius:6, pitch:2.5, height:15})` builds the helical ridge (a loft over rotated tooth sections, not a primitive), fused onto the boss core, fused onto the housing, then a blind pilot bore down through the boss. 4 operations. `thread` is the one place in this corpus that exercises a helical sweep. MEASURED: volume 31318.7 mm3, area 7669.4 mm2, 269 faces / 655 edges / 390 vertices, 1 solid, mesh genus 0, self step-mesh-compare normalizedSymmetricDifferenceVolume 0.",
    build: (b) => {
      const housing = call(b, "box", 50, 30, 20);
      const bossCore = call(b, "translate", call(b, "cylinder", 6.15, 15), [25, 15, 20]);
      const ridge = call(b, "translate", call(b, "thread", { radius: 6, pitch: 2.5, height: 15 }), [25, 15, 20]);
      let threadedBoss = call(b, "fuse", bossCore, ridge);
      threadedBoss = call(b, "fuse", housing, threadedBoss);
      threadedBoss = call(b, "cut", threadedBoss, call(b, "translate", call(b, "cylinder", 3, 25), [25, 15, 10]));
      return { operands: [{ role: "operand-a-step", shape: housing }], result: threadedBoss };
    },
  },
  {
    id: "mechanical-heatsink-fins",
    family: "mechanical",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A base plate carrying 15 fins, EACH 1mm thick and spaced 3.6mm apart, produced by ONE `rectangularPattern` call rather than 15 manual translates — the case that stresses thin-wall handling at SCALE rather than in isolation. Two mounting holes complete the plate. 3 operations, but 15 thin walls in the result. MEASURED: volume 36551.2 mm3, area 46729.0 mm2, 85 faces / 210 edges / 140 vertices, 1 solid, mesh genus 2, self step-mesh-compare normalizedSymmetricDifferenceVolume 0.",
    build: (b) => {
      const base = call(b, "box", 60, 60, 5);
      const finTemplate = call(b, "translate", call(b, "box", 1, 50, 25), [5, 5, 5]);
      const fins = call(b, "rectangularPattern", finTemplate, { xDir: [1, 0, 0], xCount: 15, xSpacing: 3.6, yDir: [0, 1, 0], yCount: 1, ySpacing: 1 });
      let heatsink = call(b, "fuse", base, fins);
      heatsink = call(b, "cutAll", heatsink, [call(b, "translate", call(b, "cylinder", 2.5, 7), [4, 4, -1]), call(b, "translate", call(b, "cylinder", 2.5, 7), [56, 56, -1])]);
      return { operands: [{ role: "operand-a-step", shape: base }], result: heatsink };
    },
  },
  {
    id: "mechanical-gearbox-cover",
    family: "mechanical",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A gearbox-style cover disc: a central through shaft bore, a shallow bearing pocket, a concentric O-RING GROOVE (built as the difference of two thin cylinders, then cut as one tool), a 6-hole bolt circle via `circularPattern`, and finally `chamfer` on every edge the chain produced. 6 operations, and the groove tool is itself a nested Boolean — a cut used to BUILD a cutter rather than to finish a part. MEASURED: volume 26197.5 mm3, area 11903.5 mm2, 38 faces / 77 edges / 44 vertices, 1 solid, mesh genus 7, self step-mesh-compare normalizedSymmetricDifferenceVolume 0.",
    build: (b) => {
      const disc = call(b, "cylinder", 40, 6);
      let cover = call(b, "cut", disc, call(b, "translate", call(b, "cylinder", 9, 10), [0, 0, -2]));
      cover = call(b, "cut", cover, call(b, "translate", call(b, "cylinder", 15, 3), [0, 0, 3]));
      const groove = call(b, "cut", call(b, "translate", call(b, "cylinder", 22, 2), [0, 0, 4]), call(b, "translate", call(b, "cylinder", 20, 2), [0, 0, 4]));
      cover = call(b, "cut", cover, groove);
      const boltCircle = call(b, "circularPattern", call(b, "translate", call(b, "cylinder", 2, 8), [32, 0, -1]), [0, 0, 1], 6, 360, [0, 0, 0]);
      cover = call(b, "cut", cover, boltCircle);
      cover = call(b, "chamfer", cover, 0.5);
      return { operands: [{ role: "operand-a-step", shape: disc }], result: cover };
    },
  },
  {
    id: "mechanical-lightening-bracket-grid",
    family: "mechanical",
    outcome: "applied",
    tolerance: "mechanical-standard",
    notes: "A filleted plate then a 5×3 grid of 15 through-holes cut as ONE tool built by `gridPattern`, rather than the manual `[x,y]` loops the older `mechanical-fixture-plate` uses — the lightening-pattern case the goal names, and a plate with genus 15 if the kernel tracks it correctly. ABANDONED `fillet(plate, 4)` over ALL 12 edges: MEASURED as a SILENT kernel defect, not a thrown error — brepjs returned a shape with `isValidSolid() === false` and `measureVolume() === 0` while reporting SUCCESS, because a 4mm radius exceeds what a 6mm-thick slab's top/bottom rim edges can support. The fix, verified: fillet only the 4 SELECTED vertical edges (`edgeFinder().inDirection([0,0,1])`) at radius 2 — a bound the thickness supports. MEASURED: volume 21434.7 mm3, area 10500.8 mm2, 25 faces / 69 edges / 46 vertices, 1 solid, mesh genus 15, self step-mesh-compare normalizedSymmetricDifferenceVolume 0.",
    build: (b) => {
      const plate = call(b, "box", 100, 40, 6);
      const filleted = call(b, "fillet", plate, edgesInDirection(b, [0, 0, 1]), 2);
      const hole = call(b, "translate", call(b, "cylinder", 3, 10), [10, 10, -2]);
      const holes = call(b, "gridPattern", hole, [1, 0, 0], [0, 1, 0], 5, 3, 18, 10);
      const bracket = call(b, "cut", filleted, holes);
      return { operands: [{ role: "operand-a-step", shape: call(b, "box", 100, 40, 6) }], result: bracket };
    },
  },
];
//#endregion 🧪️Recipes
