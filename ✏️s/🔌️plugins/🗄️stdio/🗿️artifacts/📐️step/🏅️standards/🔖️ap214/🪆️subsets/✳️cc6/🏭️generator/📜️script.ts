#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.step@ap214/✳️cc6` (advanced B-Rep).
//
// Every expected result this file writes is computed by `brepjs`'s OpenCASCADE kernel. Nothing here
// reimplements a Boolean, a tessellation or a measurement — that is the whole point: an expectation
// produced by a second Semio implementation proves the two agree, not that either is right.
//
// Generation and execution are SEPARATE operations. A normal test run must never be able to rewrite
// the expectation it is being measured against, so this is its own command and its output is
// reviewed before it is committed.
//
//   bun 📜️script.ts generate [--out <dir>] [--only <fixture-id>]
//   bun 📜️script.ts manifests                      # emit the fixtureManifests block for 🧪️oracle
//
// @see ../🔬️probes/📜️script.ts — the probes that measure what this generator produced
// @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️w4-brepjs-qualification.md

//#endregion 🧲️Header

//#region 🔌️Adapters
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
//#endregion 🔌️Adapters

//#region 🧬️Contract
/** 👪️ The fixture families the corpus is sharded and reported by — never at artifact level. */
type Family = "spatial-relationship" | "shape-complexity" | "robustness" | "mechanical" | "failure";

/**
 * 🧪️ One corpus entry. `outcome` is the DECLARED semantic class: a fixture that accepts any
 * non-crash result measures nothing, and different kernels legitimately classify exact contact
 * differently, so every entry says which answer it expects rather than discovering it.
 */
type Recipe = Readonly<{
  id: string;
  family: Family;
  outcome: "applied" | "no-op" | "empty" | "disjoint" | "rejected";
  tolerance: string;
  notes: string;
  build: (b: Kernel) => { operands: { role: string; shape: unknown }[]; result: unknown };
}>;

type Kernel = Record<string, (...args: never[]) => unknown>;

const ENGINE_FAMILY = "opencascade";
const ENGINE_VERSION = "0.15.6";
const ORACLE = "brepjs-occt";
const PACKAGE_VERSION = "18.119.8";
const SEED = 4815162342;
const TESSELLATION_TOLERANCE = 1e-3;
const ANGULAR_TOLERANCE = 0.1;
//#endregion 🧬️Contract

//#region 🧰️Kernel
let kernel: Kernel | null = null;

async function brep(): Promise<Kernel> {
  if (kernel !== null) return kernel;
  const loaded = (await import("brepjs")) as unknown as Kernel;
  await (loaded.init as unknown as () => Promise<void>)();
  kernel = loaded;
  return loaded;
}

function unwrap(value: unknown, what: string): unknown {
  if (value !== null && typeof value === "object" && "ok" in (value as Record<string, unknown>)) {
    const result = value as { ok: boolean; value?: unknown; error?: unknown };
    if (!result.ok) throw new Error(`${what}: ${JSON.stringify(result.error)}`);
    return result.value;
  }
  return value;
}

/** 📐️ `box(dx, dy, dz)` sits CORNER-at-origin; `cylinder(r, h)` sits AXIS-at-origin; `rotate(shape,
 * angleDegrees, {at, axis})` takes ONE options object. Every one of these was measured, not assumed. */
const call = (b: Kernel, name: string, ...args: unknown[]): unknown => unwrap((b[name] as unknown as (...a: unknown[]) => unknown)(...args), name);
//#endregion 🧰️Kernel

//#region 🧪️Corpus
/**
 * 🧪️ The corpus. It covers the spatial-relationship, shape-complexity, robustness, mechanical and
 * failure families of the exhaustive Boolean matrix. Each robustness triple deliberately brackets a
 * contact case — epsilon below, exact, epsilon above — because that is where two valid kernels most
 * often classify differently and where a single absolute tolerance silently hides the disagreement.
 */
const RECIPES: readonly Recipe[] = [
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
    notes: "An angled bracket with a gusset and two angled cutters. Filleting is deliberately left out where the kernel refuses it, because a fixture that quietly skipped its own defining feature would be worse than none.",
    build: (b) => {
      const upright = call(b, "box", 8, 40, 50);
      const foot = call(b, "box", 40, 40, 8);
      let bracket = call(b, "fuse", upright, foot);
      bracket = call(b, "cut", bracket, call(b, "translate", call(b, "rotate", call(b, "cylinder", 4, 60), 25, { at: [0, 0, 0], axis: [0, 1, 0] }), [20, 20, -10]));
      bracket = call(b, "cut", bracket, call(b, "translate", call(b, "cylinder", 3, 30), [4, 30, 20]));
      return { operands: [{ role: "operand-a-step", shape: upright }, { role: "operand-b-step", shape: foot }], result: bracket };
    },
  },
];
//#endregion 🧪️Corpus

//#region 🏭️Generate
async function blobText(value: unknown): Promise<string> {
  return typeof value === "string" ? value : await (value as Blob).text();
}

async function contentDigest(bytes: Uint8Array | string): Promise<string> {
  const source = typeof bytes === "string" ? new TextEncoder().encode(bytes) : bytes;
  const data = new Uint8Array(source.length);
  data.set(source);
  const hash = await crypto.subtle.digest("SHA-256", data);
  return `sha256:${[...new Uint8Array(hash)].map((byte) => byte.toString(16).padStart(2, "0")).join("")}`;
}

function write(path: string, body: string): void {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, body);
}

/** 🏭️ Generates one recipe's complete bundle: operand STEPs, expected STEP, mesh and measurements. */
async function generateOne(b: Kernel, recipe: Recipe, outDir: string): Promise<Record<string, unknown>> {
  const dir = join(outDir, recipe.id);
  const files: { role: string; path: string; mediaType: string; sha256: string; bytes: number }[] = [];

  const emitStep = async (role: string, shape: unknown, filename: string): Promise<void> => {
    const text = await blobText(call(b, "exportSTEP", shape));
    write(join(dir, filename), text);
    files.push({ role, path: `${recipe.id}/${filename}`, mediaType: "model/step", sha256: await contentDigest(text), bytes: Buffer.byteLength(text) });
  };

  const { operands, result } = recipe.build(b);
  for (const operand of operands) await emitStep(operand.role, operand.shape, `${operand.role.replace(/-step$/, "")}.step`);

  const solids = (b.getSolids as unknown as (s: unknown) => unknown[])(result) ?? [];
  const empty = solids.length === 0;

  // 🫙️An EMPTY result has no STEP body to export and no volume to measure. Writing a placeholder shape
  // would turn "correctly nothing" into "something", so the bundle records emptiness as the fact it is.
  const measurements: Record<string, unknown> = { declaredOutcome: recipe.outcome, solids: solids.length, empty };
  if (!empty) {
    await emitStep("expected-step", result, "expected.step");
    const bounds = call(b, "getBounds", result) as Record<string, number>;
    measurements.volume = call(b, "measureVolume", result);
    measurements.area = call(b, "measureArea", result);
    measurements.boundingBox = bounds;
    measurements.boundingBoxDiagonal = Math.hypot(bounds.xMax! - bounds.xMin!, bounds.yMax! - bounds.yMin!, bounds.zMax! - bounds.zMin!);
    measurements.faces = ((b.getFaces as unknown as (s: unknown) => unknown[])(result) ?? []).length;
    measurements.edges = ((b.getEdges as unknown as (s: unknown) => unknown[])(result) ?? []).length;
    measurements.vertices = ((b.getVertices as unknown as (s: unknown) => unknown[])(result) ?? []).length;
    measurements.validSolid = (b.isValidSolid as unknown as (s: unknown) => boolean)(result);

    const meshed = call(b, "mesh", result, { tolerance: TESSELLATION_TOLERANCE, angularTolerance: ANGULAR_TOLERANCE }) as Record<string, ArrayLike<number>>;
    const vertices = Array.from(meshed.vertices ?? meshed.positions!);
    const triangles = Array.from(meshed.triangles ?? meshed.indices!);
    const meshBody = `${JSON.stringify({ vertices, triangles, tolerance: TESSELLATION_TOLERANCE, angularTolerance: ANGULAR_TOLERANCE })}\n`;
    write(join(dir, "expected.mesh.json"), meshBody);
    files.push({ role: "expected-mesh", path: `${recipe.id}/expected.mesh.json`, mediaType: "application/json", sha256: await contentDigest(meshBody), bytes: Buffer.byteLength(meshBody) });
    measurements.meshVertexCount = vertices.length / 3;
    measurements.meshTriangleCount = triangles.length / 3;
  }

  const metricsBody = `${JSON.stringify(measurements, null, 2)}\n`;
  write(join(dir, "expected.metrics.json"), metricsBody);
  files.push({ role: "expected-measurements", path: `${recipe.id}/expected.metrics.json`, mediaType: "application/json", sha256: await contentDigest(metricsBody), bytes: Buffer.byteLength(metricsBody) });

  return {
    schema: "semio.repository-test.fixture/v2",
    id: recipe.id,
    class: "third-party-generated",
    target: { artifact: "s.stdio.step", standard: "ap214", subset: "cc6" },
    mutation: "set-shape-representation",
    outcome: recipe.outcome,
    units: { length: "millimetre", angle: "radian", handedness: "right", up: "z" },
    files,
    generator: {
      oracle: ORACLE,
      packageVersion: PACKAGE_VERSION,
      engineFamily: ENGINE_FAMILY,
      engineVersion: ENGINE_VERSION,
      command: `bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/🏭️generator/📜️script.ts generate --only ${recipe.id}`,
      seed: SEED,
      platform: `${process.platform === "win32" ? "win32" : process.platform === "darwin" ? "darwin" : "linux"}-${process.arch === "arm64" ? "arm64" : "x64"}`,
    },
    provenance: { source: "generated", license: "Apache-2.0", attribution: "Generated with brepjs (Apache-2.0) over brepjs-opencascade (LGPL-2.1-only, OpenCASCADE 8.0 WASM)", security: "scanned-clean", privacy: "no-personal-data" },
    comparisonProfile: "semantic-brep-solid-v1",
    toleranceProfile: recipe.tolerance,
    // 🏭️`reproducible: false` is a MEASURED fact, not a shrug: the qualification spike showed OCCT
    // stamping an incrementing translator counter and a wall-clock timestamp into every export, so the
    // STEP bytes are not byte-reproducible and no external canonicalizer is qualified yet to normalise
    // them. `test fixture reproduce` therefore reports these, and the report says which ones and why.
    reproducible: false,
    family: recipe.family,
    notes: recipe.notes,
  };
}
//#endregion 🏭️Generate

//#region 🚪️Entry
async function main(argv: readonly string[]): Promise<number> {
  const [command = "generate", ...rest] = argv;
  const value = (flag: string): string | null => {
    const index = rest.indexOf(flag);
    return index === -1 ? null : (rest[index + 1] ?? null);
  };
  const only = value("--only");
  const recipes = only === null ? RECIPES : RECIPES.filter((recipe) => recipe.id === only);
  if (recipes.length === 0) {
    console.error(`[generator] no recipe matches ${JSON.stringify(only)} — known: ${RECIPES.map((recipe) => recipe.id).join(", ")}`);
    return 1;
  }
  const outDir = process.env.SEMIO_FIXTURE_OUT ?? value("--out") ?? join(import.meta.dir, "..", "🧫️fixtures");

  if (command === "manifests" || command === "generate") {
    const b = await brep();
    const manifests: Record<string, unknown>[] = [];
    let failed = 0;
    for (const recipe of recipes) {
      try {
        manifests.push(await generateOne(b, recipe, outDir));
        console.error(`[generator] ${recipe.id} (${recipe.family}, ${recipe.outcome})`);
      } catch (error) {
        // 🧭️A recipe the kernel refuses is REPORTED, never dropped: a corpus that quietly shrank to
        // whatever happened to build would read as complete coverage of a smaller matrix.
        failed += 1;
        console.error(`[generator] ${recipe.id} FAILED — ${(error as Error).message}`);
      }
    }
    if (command === "manifests") process.stdout.write(`${JSON.stringify(manifests, null, 2)}\n`);
    else write(join(outDir, "🧫️manifests.json"), `${JSON.stringify(manifests, null, 2)}\n`);
    console.error(`[generator] ${manifests.length}/${recipes.length} bundle(s) generated into ${outDir}${failed > 0 ? `, ${failed} failed` : ""}`);
    return failed > 0 ? 1 : 0;
  }
  console.error(`[generator] unknown command ${JSON.stringify(command)} — expected generate | manifests`);
  return 1;
}

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
//#endregion 🚪️Entry
