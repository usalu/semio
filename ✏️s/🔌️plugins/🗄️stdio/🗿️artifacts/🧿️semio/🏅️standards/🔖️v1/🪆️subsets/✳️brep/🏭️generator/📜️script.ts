#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.semio@v1/✳️brep` — the 13-verb kernel-edit vocabulary
// (`create-vertex`/`delete-vertex`, `create-edge`/`delete-edge`, `create-face`/`delete-face`,
// `create-shell`/`delete-shell`, `create-solid`/`delete-solid`, `replace-curve`, `replace-surface`,
// `move-vertex`) plus the Boolean matrix these primitives are batched into.
//
// This is TOPOLOGICAL/GEOMETRIC coverage, not a file-format corpus: every recipe below describes a
// BEFORE B-Rep and, where the transition is legal, an AFTER B-Rep — both built and measured by
// `brepjs`'s OpenCASCADE kernel. Nothing here reimplements a topology edit, a Boolean, a tessellation
// or a measurement — an expectation produced by a second Semio implementation only proves the two
// agree, not that either is right.
//
// Generation and execution are SEPARATE operations, same as the sibling `s.stdio.step@ap214/✳️cc6`
// corpus this file's shape is deliberately modelled on: a normal test run must never be able to
// rewrite the expectation it is measured against.
//
//   bun 📜️script.ts generate [--out <dir>] [--only <fixture-id>]
//   bun 📜️script.ts manifests                      # emit the fixtureManifests block for 🧪️oracle
//
// @see ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/🏭️generator/📜️script.ts
//      — the sibling this generator's shape, CLI and manifest format are mirrored from
// @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/

//#endregion 🧲️Header

//#region 🔌️Adapters
import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
//#endregion 🔌️Adapters

//#region 🧬️Contract
/** 👪️ The fixture families the corpus is sharded and reported by. */
export type Family = "topology-build" | "topology-remove" | "geometry-replace" | "move-vertex" | "booleans";

/** 🧬️ The 13-verb vocabulary this subset's `🧬️schema/🧬️mutations/🦀️.rs` dispatches — one kind per recipe. */
export type MutationKind =
  | "create-vertex"
  | "delete-vertex"
  | "create-edge"
  | "delete-edge"
  | "create-face"
  | "delete-face"
  | "create-shell"
  | "delete-shell"
  | "create-solid"
  | "delete-solid"
  | "replace-curve"
  | "replace-surface"
  | "move-vertex";

/**
 * 🧪️ One corpus entry. `outcome` is the DECLARED semantic class: a fixture that accepts any non-crash
 * result measures nothing, and a kernel that merely fails to throw is not the same as a kernel that
 * accepted the edit, so every entry says which answer it expects rather than discovering it.
 *
 * `build` returns the BEFORE operand(s) and, where the edit is legal, the AFTER shape. A `result` of
 * `null` is itself a DECLARED, MEASURED fact for `outcome: "rejected"` — see `📓️` note on
 * `generateOne` for how a rejection is proven rather than asserted.
 */
export type Recipe = Readonly<{
  id: string;
  family: Family;
  kind: MutationKind;
  outcome: "applied" | "rejected" | "disjoint" | "no-op" | "empty";
  tolerance: string;
  notes: string;
  build: (b: Kernel) => { operands: { role: string; shape: unknown }[]; result: unknown | null; rejectionReason?: string };
}>;

export type Kernel = Record<string, (...args: never[]) => unknown>;

const ENGINE_FAMILY = "opencascade";
const ENGINE_VERSION = "8.0";
const ORACLE = "brepjs-occt";
const PACKAGE_VERSION = "18.119.8";
const SEED = 4815162342;

/**
 * 🔺️ Tessellation tolerance, resolved SCALE-RELATIVE — `max(absolute, relative × bounding-box
 * diagonal)`. A fixed absolute tolerance is exactly the mistake this rule exists to prevent: on a part
 * translated to a different scale a fixed absolute number becomes an unintentionally tight relative
 * one, and meshing can run for minutes on a shape an exact kernel resolved in under a second.
 */
const TESSELLATION_RELATIVE = 3e-5;
const TESSELLATION_ABSOLUTE_FLOOR = 1e-6;
const ANGULAR_TOLERANCE = 0.1;

/** 🔺️ The tessellation tolerance for one shape, from its own measured size. */
function tessellationToleranceFor(diagonal: number): number {
  return Math.max(TESSELLATION_ABSOLUTE_FLOOR, TESSELLATION_RELATIVE * Math.abs(diagonal));
}
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

/** 📐️ `box(dx,dy,dz)` sits CORNER-at-origin; `cylinder(r,h)` sits AXIS-at-origin extending +z;
 * `sphere(r)`/`cone` centre-or-base-at `at`; `rotate(shape, angleDegrees, {at, axis})` takes ONE
 * options object. Every one of these was MEASURED against the installed `brepjs@18.119.8`, not
 * assumed from documentation. */
export const call = (b: Kernel, name: string, ...args: unknown[]): unknown => unwrap((b[name] as unknown as (...a: unknown[]) => unknown)(...args), name);

/** 🧯️Calls `fn`, swallowing a kernel throw or a `Result` error into `null` — used for measurements
 * that are legitimately inapplicable to a given shape kind (e.g. `measureVolume` on an open shell)
 * rather than a fixture-breaking failure. */
function tryMeasure<T>(fn: () => T): T | null {
  try {
    return fn();
  } catch {
    return null;
  }
}
//#endregion 🧰️Kernel

//#region 🧪️Corpus
/**
 * 🧪️ The corpus, assembled from one module per FAMILY — the sharding key this subset's CI runs and
 * reports by, and the unit two people can grow independently without touching the same file.
 */
const RECIPES: readonly Recipe[] = [
  ...(await import("./🧪️topology-build/📜️script.ts")).RECIPES,
  ...(await import("./🧪️topology-remove/📜️script.ts")).RECIPES,
  ...(await import("./🧪️geometry-replace/📜️script.ts")).RECIPES,
  ...(await import("./🧪️move-vertex/📜️script.ts")).RECIPES,
  ...(await import("./🧪️booleans/📜️script.ts")).RECIPES,
];
//#endregion 🧪️Corpus

//#region 🏭️Generate
async function blobText(value: unknown): Promise<string> {
  return typeof value === "string" ? value : await (value as Blob).text();
}

/**
 * 📥️ Reads back what was actually WRITTEN. Every measurement a fixture records must describe the
 * committed `expected.step`, not the in-memory shape it was exported from — the committed file is the
 * only thing a consumer can re-measure. `s.stdio.step@ap214/✳️cc6`'s `fuse-edge-touching-boxes` found
 * this the hard way: 23 edges/14 vertices in memory, 24/16 once STEP export separated the shared edge.
 */
async function reimport(b: Kernel, stepText: string): Promise<unknown> {
  const imported = unwrap(await (b.importSTEP as unknown as (blob: Blob) => unknown)(new Blob([stepText])), "importSTEP");
  const resolved = imported instanceof Promise ? unwrap(await imported, "importSTEP await") : imported;
  if (Array.isArray(resolved)) return resolved[0];
  const record = resolved as { shape?: unknown };
  return record.shape ?? resolved;
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

/**
 * 🔬️ Every topology count and measurement this generator can pull off a re-imported shape, guarded
 * individually: this subset's fixtures range over vertices, edges, faces, shells, solids and compounds
 * of these, and a measurement that is inapplicable to one shape kind (e.g. `measureVolume` on a bare
 * face) must not abort measurement of everything else the shape DOES support.
 */
function measureShape(b: Kernel, shape: unknown): Record<string, unknown> {
  const solids = (tryMeasure(() => (b.getSolids as unknown as (s: unknown) => unknown[])(shape)) ?? []) as unknown[];
  const shells = (tryMeasure(() => (b.getShells as unknown as (s: unknown) => unknown[])(shape)) ?? []) as unknown[];
  const faces = (tryMeasure(() => (b.getFaces as unknown as (s: unknown) => unknown[])(shape)) ?? []) as unknown[];
  const edges = (tryMeasure(() => (b.getEdges as unknown as (s: unknown) => unknown[])(shape)) ?? []) as unknown[];
  const vertices = (tryMeasure(() => (b.getVertices as unknown as (s: unknown) => unknown[])(shape)) ?? []) as unknown[];
  const bounds = tryMeasure(() => call(b, "getBounds", shape)) as Record<string, number> | null;
  const volume = tryMeasure(() => call(b, "measureVolume", shape)) as number | null;
  const area = tryMeasure(() => call(b, "measureArea", shape)) as number | null;
  const validSolid = tryMeasure(() => (b.isValidSolid as unknown as (s: unknown) => boolean)(shape)) as boolean | null;
  const isCompound = tryMeasure(() => (b.isCompound as unknown as (s: unknown) => boolean)(shape)) as boolean | null;
  const eulerCharacteristic = vertices.length - edges.length + faces.length;
  // 🔺️Genus is only well-defined for a SHAPE THAT IS ITSELF ONE closed valid solid of one shell —
  // Euler's `V-E+F=2-2g` does not resolve to a meaningful integer for an open shell, a bare wire, or a
  // COMPOUND that merely CONTAINS one solid alongside other loose entities (`isValidSolid` measured
  // `true` even on `compound(box, looseVertex)`, so that guard alone is not enough — the extra vertex's
  // count still pollutes `eulerCharacteristic` and must be excluded by requiring the shape not be a
  // compound at all, not merely by counting how many solids it holds).
  const genus = !isCompound && solids.length === 1 && shells.length === 1 && validSolid === true ? (2 - eulerCharacteristic) / 2 : null;
  const boundingBoxDiagonal = bounds !== null ? Math.hypot((bounds.xMax ?? 0) - (bounds.xMin ?? 0), (bounds.yMax ?? 0) - (bounds.yMin ?? 0), (bounds.zMax ?? 0) - (bounds.zMin ?? 0)) : null;
  return { solids: solids.length, shells: shells.length, faces: faces.length, edges: edges.length, vertices: vertices.length, boundingBox: bounds, boundingBoxDiagonal, volume, area, validSolid, eulerCharacteristic, genus };
}

/**
 * 🧼️ OCCT stamps three PROCESS-GLOBAL values into every STEP export: a wall-clock `FILE_NAME` timestamp,
 * a translator counter on `PRODUCT`, and an occurrence counter on `NEXT_ASSEMBLY_USAGE_OCCURRENCE`. All
 * three depend on how many exports ran earlier in the same process, never on the shape — so two
 * byte-identical solids exported at different points in one batch differ in exactly these and nowhere
 * else. Normalizing them is what makes the corpus reproducible; geometry is untouched.
 *
 * The occurrence counter is the one that hides. Generating the WHOLE corpus twice is not a sufficient
 * test, because both passes export in the same order and the counters agree. Only regenerating a single
 * fixture on its own — what `test fixture reproduce` does — starts the counters elsewhere and exposes it.
 * On the sibling `s.stdio.step@ap214/✳️cc6` corpus that check found 23 of 119 fixtures still differing
 * after the first two values had been normalized, every difference confined to this counter.
 */
function canonicalizeStep(text: string): string {
  return text
    .replace(/(FILE_NAME\('[^']*',')[^']*(',)/, "$11970-01-01T00:00:00$2")
    .replace(/(Open CASCADE STEP translator [0-9.]+) \d+/g, "$1")
    .replace(/(NEXT_ASSEMBLY_USAGE_OCCURRENCE\(')\d+(')/g, "$10$2");
}

/**
 * 🏭️ Generates one recipe's complete bundle: operand STEP(s), and — for a legal edit — the AFTER
 * STEP, its tessellated mesh, and every measurement above, all read back from the FILES this function
 * just wrote (see `reimport`).
 *
 * A `result` of `null` is `outcome: "rejected"`'s PROOF, not a shrug: this subset has no low-level
 * Euler operator that "deletes a face", so a rejection is demonstrated by attempting the transition
 * the mutation would require and recording what the kernel actually did with it — an open shell that
 * `solid()` refuses to close, a non-planar wire `face()` throws `FACE_NOT_PLANAR` on, a self-crossing
 * polygon whose signed area collapses to numerically zero. Where the kernel still returns SOME shape
 * for that attempt, `result` carries it and its own `validSolid`/`area` measurements are the evidence;
 * where the kernel refuses to produce anything at all, `result` is `null` and only the BEFORE operand
 * is measured.
 */
async function generateOne(b: Kernel, recipe: Recipe, outDir: string): Promise<Record<string, unknown>> {
  const dir = join(outDir, recipe.id);
  const files: { role: string; path: string; mediaType: string; sha256: string; bytes: number }[] = [];

  const emitStep = async (role: string, shape: unknown, filename: string): Promise<void> => {
    const text = canonicalizeStep(await blobText(call(b, "exportSTEP", shape)));
    write(join(dir, filename), text);
    files.push({ role, path: `../🧫️fixtures/${recipe.id}/${filename}`, mediaType: "model/step", sha256: await contentDigest(text), bytes: Buffer.byteLength(text) });
  };

  const { operands, result, rejectionReason } = recipe.build(b);
  for (const operand of operands) await emitStep(operand.role, operand.shape, `${operand.role.replace(/-step$/, "")}.step`);

  const measurements: Record<string, unknown> = { declaredOutcome: recipe.outcome, kind: recipe.kind, hasExpected: result !== null };
  if (result !== null) {
    const preMeasure = measureShape(b, result);
    // 🫙️A Boolean that measures NOTHING at all — a `cut` fully engulfed, an `intersect` with no shared
    // volume — is `outcome: "empty"`'s proof, but OCCT's own STEP writer THROWS `STEP_EXPORT_UNSERIALIZABLE`
    // on a shape with zero sub-entities rather than emitting a valid empty document. MEASURED directly:
    // attempting the export anyway turns a correct `empty` fixture into a reported generator FAILURE.
    const trulyEmpty = preMeasure.solids === 0 && preMeasure.shells === 0 && preMeasure.faces === 0 && preMeasure.edges === 0 && preMeasure.vertices === 0;
    if (trulyEmpty) {
      Object.assign(measurements, preMeasure);
      measurements.hasExpected = false;
      measurements.measuredFrom = "in-memory result — the kernel's own shape has no sub-entities at all; STEP export refuses to serialize empty geometry";
    } else {
      try {
        // 📥️Export FIRST, then measure what was written — never the in-memory shape. See `reimport` above.
        const exportedText = canonicalizeStep(await blobText(call(b, "exportSTEP", result)));
        write(join(dir, "expected.step"), exportedText);
        files.push({ role: "expected-step", path: `../🧫️fixtures/${recipe.id}/expected.step`, mediaType: "model/step", sha256: await contentDigest(exportedText), bytes: Buffer.byteLength(exportedText) });
        const measured = await reimport(b, exportedText);
        Object.assign(measurements, measureShape(b, measured));
        measurements.measuredFrom = "expected.step, re-imported";

        // 🫙️A shape with no measurable extent (no volume AND no area — a bare vertex or a degenerate
        // self-intersecting face) has nothing to tessellate; forcing a mesh through it would either fail
        // in the kernel or record a fabricated zero-triangle mesh as if it meant something.
        const hasExtent = ((measurements.volume as number | null) ?? 0) !== 0 || ((measurements.area as number | null) ?? 0) !== 0;
        if (hasExtent && measurements.boundingBoxDiagonal !== null) {
          const tessellationTolerance = tessellationToleranceFor(measurements.boundingBoxDiagonal as number);
          const meshed = tryMeasure(() => call(b, "mesh", measured, { tolerance: tessellationTolerance, angularTolerance: ANGULAR_TOLERANCE })) as Record<string, ArrayLike<number>> | null;
          if (meshed !== null) {
            const vertices = Array.from(meshed.vertices ?? meshed.positions!);
            const triangles = Array.from(meshed.triangles ?? meshed.indices!);
            const meshBody = `${JSON.stringify({ vertices, triangles, tolerance: tessellationTolerance, angularTolerance: ANGULAR_TOLERANCE })}\n`;
            write(join(dir, "expected.mesh.json"), meshBody);
            files.push({ role: "expected-mesh", path: `../🧫️fixtures/${recipe.id}/expected.mesh.json`, mediaType: "application/json", sha256: await contentDigest(meshBody), bytes: Buffer.byteLength(meshBody) });
            measurements.tessellationTolerance = tessellationTolerance;
            measurements.meshVertexCount = vertices.length / 3;
            measurements.meshTriangleCount = triangles.length / 3;
          } else {
            measurements.meshSkippedReason = "kernel mesh() refused this shape";
          }
        } else {
          measurements.meshSkippedReason = "no measurable volume or area";
        }
      } catch (error) {
        // 🧯️Not every empty-ish shape is caught by the zero-sub-entity guard above (e.g. a shape that
        // still reports sub-entities but that the STEP writer nonetheless finds degenerate) — this is
        // the second, narrower net, and it still records a REAL measurement (of the in-memory shape)
        // rather than silently downgrading the fixture to a bare failure.
        Object.assign(measurements, preMeasure);
        measurements.hasExpected = false;
        measurements.measuredFrom = "in-memory result only — STEP export threw and was not attempted again";
        measurements.exportFailedReason = (error as Error).message;
      }
    }
  } else {
    // 🚫️`outcome: "rejected"` with no exportable AFTER at all — the BEFORE operand(s) above are the
    // whole bundle, and `notes` says what the kernel did when the transition was attempted.
    Object.assign(measurements, measureShape(b, operands[0]!.shape));
    measurements.measuredFrom = "before operand only — kernel produced no AFTER shape";
  }
  // 📌️Recorded regardless of which branch ran above: a `rejected` recipe whose attempted AFTER still
  // exported SOMETHING (e.g. `delete-face-still-bounding-closed-shell`, where `solid()` on 5 of 6 faces
  // does not throw) needs its rejection evidence in the metrics just as much as one where it's `null`.
  if (rejectionReason !== undefined) measurements.rejectionReason = rejectionReason;

  const metricsBody = `${JSON.stringify(measurements, null, 2)}\n`;
  write(join(dir, "expected.metrics.json"), metricsBody);
  files.push({ role: "expected-measurements", path: `../🧫️fixtures/${recipe.id}/expected.metrics.json`, mediaType: "application/json", sha256: await contentDigest(metricsBody), bytes: Buffer.byteLength(metricsBody) });

  return {
    schema: "semio.repository-test.fixture/v2",
    id: recipe.id,
    class: "third-party-generated",
    target: { artifact: "s.stdio.semio", standard: "v1", subset: "brep" },
    mutation: recipe.kind,
    outcome: recipe.outcome,
    units: { length: "millimetre", angle: "radian", handedness: "right", up: "z" },
    files,
    generator: {
      oracle: ORACLE,
      packageVersion: PACKAGE_VERSION,
      engineFamily: ENGINE_FAMILY,
      engineVersion: ENGINE_VERSION,
      command: `bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🏭️generator/📜️script.ts generate --only ${recipe.id}`,
      seed: SEED,
      platform: `${process.platform === "win32" ? "win32" : process.platform === "darwin" ? "darwin" : "linux"}-${process.arch === "arm64" ? "arm64" : "x64"}`,
    },
    provenance: { source: "generated", license: "Apache-2.0", attribution: "Generated with brepjs (Apache-2.0) over brepjs-opencascade (LGPL-2.1-only, OpenCASCADE 8.0 WASM)", security: "scanned-clean", privacy: "no-personal-data" },
    comparisonProfile: "semantic-brep-kernel-edit-v1",
    toleranceProfile: recipe.tolerance,
    // 🏭️Filled in by `verifyReproducibility` below — a MEASURED fact from an actual second generation
    // pass and byte-comparison, never a default or an assumption. See that function's docstring.
    reproducible: false,
    family: recipe.family,
    notes: recipe.notes,
  };
}

/**
 * 🔁️Proves reproducibility rather than asserting it: regenerates the SAME recipe into a throwaway
 * directory and byte-compares every file the first pass wrote. `s.stdio.step@ap214/✳️cc6`'s own
 * qualification found OCCT's STEP writer embeds a wall-clock `FILE_NAME` timestamp AND an incrementing
 * per-process translator-instance counter in `PRODUCT` — this repo's own spike (see the ticket doc)
 * additionally confirmed the SECOND finding by direct diff. Both make `expected.step`/`operand-*.step`
 * near-certain to differ between two independent process runs; `expected.mesh.json` and
 * `expected.metrics.json` carry no such header and are the files where reproducibility is actually
 * observed, when it is.
 */
async function verifyReproducibility(b: Kernel, recipe: Recipe, primaryDir: string, scratchDir: string): Promise<{ reproducible: boolean; diffs: string[] }> {
  await generateOne(b, recipe, scratchDir);
  const primary = join(primaryDir, recipe.id);
  const scratch = join(scratchDir, recipe.id);
  if (!existsSync(primary) || !existsSync(scratch)) return { reproducible: false, diffs: ["missing-output"] };
  const names = new Set([...readdirSync(primary), ...readdirSync(scratch)]);
  const diffs: string[] = [];
  for (const name of names) {
    const a = existsSync(join(primary, name)) ? readFileSync(join(primary, name)) : null;
    const c = existsSync(join(scratch, name)) ? readFileSync(join(scratch, name)) : null;
    if (a === null || c === null || !a.equals(c)) diffs.push(name);
  }
  rmSync(scratch, { recursive: true, force: true });
  return { reproducible: diffs.length === 0, diffs };
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
  const ids = new Set(RECIPES.map((recipe) => recipe.id));
  if (ids.size !== RECIPES.length) {
    console.error(`[generator] duplicate recipe id(s) detected — every id must be unique across all families`);
    return 1;
  }
  const outDir = process.env.SEMIO_FIXTURE_OUT ?? value("--out") ?? join(import.meta.dir, "..", "🧫️fixtures");

  if (command === "manifests" || command === "generate") {
    const b = await brep();
    const manifests: Record<string, unknown>[] = [];
    let failed = 0;
    const failures: { id: string; error: string }[] = [];
    for (const recipe of recipes) {
      try {
        const manifest = await generateOne(b, recipe, outDir);
        if (command === "generate") {
          const scratchDir = join(outDir, ".reproduce-tmp");
          const repro = await verifyReproducibility(b, recipe, outDir, scratchDir);
          // 🧭️Reproducibility belongs on the MANIFEST and nowhere else. Writing it back into
          // `expected.metrics.json` — as this did — rewrote the file AFTER its digest had been recorded,
          // so every one of the 72 recorded `expected-measurements` hashes described content that no
          // longer existed on disk. It also made the file self-referential: a metrics file whose content
          // states whether that same file reproduces. The metrics describe the geometry; whether the
          // bundle regenerates byte-identically is a fact about the bundle.
          manifest.reproducible = repro.reproducible;
          (manifest as Record<string, unknown>).reproducibilityDiffs = repro.diffs;
        }
        manifests.push(manifest);
        console.error(`[generator] ${recipe.id} (${recipe.family}, ${recipe.kind}, ${recipe.outcome})${command === "generate" ? ` reproducible=${manifest.reproducible}` : ""}`);
      } catch (error) {
        // 🧭️A recipe the kernel refuses is REPORTED, never dropped: a corpus that quietly shrank to
        // whatever happened to build would read as complete coverage of a smaller matrix.
        failed += 1;
        failures.push({ id: recipe.id, error: (error as Error).message });
        console.error(`[generator] ${recipe.id} FAILED — ${(error as Error).message}`);
      }
    }
    if (existsSync(join(outDir, ".reproduce-tmp"))) rmSync(join(outDir, ".reproduce-tmp"), { recursive: true, force: true });
    if (command === "manifests") process.stdout.write(`${JSON.stringify(manifests, null, 2)}\n`);
    else {
      // 🧬️A NARROWED run MERGES into the manifest index; it does not replace it — see the sibling
      // generator's docstring for the incident this guards against.
      const indexPath = join(outDir, "🔣️.json");
      const previous = (() => {
        if (only === null || !existsSync(indexPath)) return [];
        try {
          return JSON.parse(readFileSync(indexPath, "utf8")) as Record<string, unknown>[];
        } catch {
          return [];
        }
      })();
      const produced = new Set(manifests.map((entry) => entry.id as string));
      const merged = [...previous.filter((entry) => !produced.has(entry.id as string)), ...manifests].sort((a, b) => String(a.id).localeCompare(String(b.id)));
      write(indexPath, `${JSON.stringify(merged, null, 2)}\n`);
      if (only !== null) console.error(`[generator] merged ${manifests.length} regenerated entr(ies) into ${merged.length} total`);
      const reproducibleCount = merged.filter((entry) => entry.reproducible === true).length;
      console.error(`[generator] reproducibility: ${reproducibleCount}/${merged.length} fixture(s) byte-identical across two generation passes`);
    }
    console.error(`[generator] ${manifests.length}/${recipes.length} bundle(s) generated into ${outDir}${failed > 0 ? `, ${failed} failed` : ""}`);
    if (failed > 0) console.error(`[generator] failures: ${JSON.stringify(failures, null, 2)}`);
    return failed > 0 ? 1 : 0;
  }
  console.error(`[generator] unknown command ${JSON.stringify(command)} — expected generate | manifests`);
  return 1;
}

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
//#endregion 🚪️Entry
