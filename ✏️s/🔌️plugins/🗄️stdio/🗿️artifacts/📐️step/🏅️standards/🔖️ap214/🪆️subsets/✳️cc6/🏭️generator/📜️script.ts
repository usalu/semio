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
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
//#endregion 🔌️Adapters

//#region 🧬️Contract
/** 👪️ The fixture families the corpus is sharded and reported by — never at artifact level. */
export type Family = "spatial-relationship" | "shape-complexity" | "robustness" | "mechanical" | "failure";

/**
 * 🧪️ One corpus entry. `outcome` is the DECLARED semantic class: a fixture that accepts any
 * non-crash result measures nothing, and different kernels legitimately classify exact contact
 * differently, so every entry says which answer it expects rather than discovering it.
 */
export type Recipe = Readonly<{
  id: string;
  family: Family;
  outcome: "applied" | "no-op" | "empty" | "disjoint" | "rejected";
  tolerance: string;
  notes: string;
  build: (b: Kernel) => { operands: { role: string; shape: unknown }[]; result: unknown };
}>;

export type Kernel = Record<string, (...args: never[]) => unknown>;

const ENGINE_FAMILY = "opencascade";
const ENGINE_VERSION = "0.15.6";
const ORACLE = "brepjs-occt";
const PACKAGE_VERSION = "18.119.8";
const SEED = 4815162342;
/**
 * 🔺️ Tessellation tolerance, resolved SCALE-RELATIVE — `max(absolute, relative × bounding-box diagonal)`,
 * the same rule every dimensional tolerance in this protocol uses.
 *
 * A fixed absolute 1e-3 was the original, and it is exactly the mistake the protocol exists to prevent.
 * On a part translated to 1e6 units that is a RELATIVE tolerance of 5e-11, and the meshing stage did
 * not merely produce a large mesh — it ran for over twelve minutes and climbed past 2.4 GB before being
 * killed, while the underlying exact Boolean had completed in under a second. The measuring tool was
 * consumed by the boundary it existed to measure.
 *
 * The relative term is what makes one setting serve a 0.02 mm bore and a 10 m building; the absolute
 * floor is what stops a tiny part being tessellated more finely than the kernel's own 1e-7 tolerance
 * can even represent.
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

/** 📐️ `box(dx, dy, dz)` sits CORNER-at-origin; `cylinder(r, h)` sits AXIS-at-origin; `rotate(shape,
 * angleDegrees, {at, axis})` takes ONE options object. Every one of these was measured, not assumed. */
export const call = (b: Kernel, name: string, ...args: unknown[]): unknown => unwrap((b[name] as unknown as (...a: unknown[]) => unknown)(...args), name);
//#endregion 🧰️Kernel

//#region 🧪️Corpus
/**
 * 🧪️ The corpus, assembled from one module per FAMILY. Splitting it this way is not cosmetic: the
 * families are the sharding key CI uses and the axis the exhaustive Boolean matrix is organised by, so
 * a family is the unit somebody extends, reviews or runs in isolation. It also means two people can
 * grow two families at once without touching the same file.
 */
const RECIPES: readonly Recipe[] = [
  ...(await import("./🧪️spatial-relationship/📜️script.ts")).RECIPES,
  ...(await import("./🧪️shape-complexity/📜️script.ts")).RECIPES,
  ...(await import("./🧪️robustness/📜️script.ts")).RECIPES,
  ...(await import("./🧪️mechanical/📜️script.ts")).RECIPES,
  ...(await import("./🧪️failure/📜️script.ts")).RECIPES,
];
//#endregion 🧪️Corpus

//#endregion 🧪️Corpus

//#region 🏭️Generate
async function blobText(value: unknown): Promise<string> {
  return typeof value === "string" ? value : await (value as Blob).text();
}

/**
 * 🕰️ Strips the two fields OCCT's STEP writer draws from PROCESS STATE rather than from the shape:
 * the `FILE_NAME` wall-clock timestamp, and the incrementing translator counter it stamps into the
 * root `PRODUCT`'s name and description (`'Open CASCADE STEP translator 8.0 <n>'`, `<n>` counting
 * every `exportSTEP` call this kernel instance has ever made — so it depends on how many fixtures
 * were exported before this one in the same process, not on this shape at all).
 *
 * MEASURED, not assumed: two independent full-corpus regenerations of all 121 fixtures in this
 * subset produced byte-identical output on every entity line once exactly these two fields were
 * normalised — see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/.
 * Entity ordering, ids, coordinates and topology are untouched by this function; it never fires on
 * `handcrafted-tetrahedron`, whose own `FILE_NAME` timestamp is already a fixed constant.
 */
function canonicalizeStep(text: string): string {
  // 🧼️OCCT stamps three PROCESS-GLOBAL values into every export: a wall-clock timestamp, a translator
  // counter on PRODUCT, and an occurrence counter on NEXT_ASSEMBLY_USAGE_OCCURRENCE. All three depend on
  // how many exports ran earlier in the same process, never on the shape, so two byte-identical solids
  // exported at different points in a batch differ in them and nowhere else.
  //
  // The third one is why generating the whole corpus twice is NOT a sufficient reproducibility test:
  // both runs export in the same order, so the counters agree and the corpus looks stable. Regenerating
  // ONE fixture on its own — which is what `test fixture reproduce` does — starts the counter from a
  // different place and exposes it. That check found 23 of 119 fixtures still differing after the first
  // two were normalized, every difference confined to these counters and none to geometry.
  return text
    .replace(/(FILE_NAME\('[^']*',')[^']*(',)/, "$11970-01-01T00:00:00$2")
    .replace(/(Open CASCADE STEP translator [0-9.]+) \d+/g, "$1")
    .replace(/(NEXT_ASSEMBLY_USAGE_OCCURRENCE\(')\d+(')/g, "$10$2");
}

/**
 * 📥️ Reads back what was actually WRITTEN. Every measurement a fixture records must describe the
 * committed `expected.step`, not the in-memory shape it was exported from, because the committed file
 * is the only thing a consumer can re-measure.
 *
 * The difference is real and was found by re-measuring: `fuse-edge-touching-boxes` holds two solids
 * that SHARE the contact edge in memory (23 edges, 14 vertices), and STEP export separates them into
 * two independent solids (24 edges, 16 vertices). Recording the in-memory numbers put a topology count
 * in the manifest that nobody could reproduce from the file it claims to describe.
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

/** 🏭️ Generates one recipe's complete bundle: operand STEPs, expected STEP, mesh and measurements. */
async function generateOne(b: Kernel, recipe: Recipe, outDir: string): Promise<Record<string, unknown>> {
  const dir = join(outDir, recipe.id);
  const files: { role: string; path: string; mediaType: string; sha256: string; bytes: number }[] = [];

  const emitStep = async (role: string, shape: unknown, filename: string): Promise<void> => {
    const text = canonicalizeStep(await blobText(call(b, "exportSTEP", shape)));
    write(join(dir, filename), text);
    files.push({ role, path: `${recipe.id}/${filename}`, mediaType: "model/step", sha256: await contentDigest(text), bytes: Buffer.byteLength(text) });
  };

  const { operands, result } = recipe.build(b);
  for (const operand of operands) await emitStep(operand.role, operand.shape, `${operand.role.replace(/-step$/, "")}.step`);

  const solids = (b.getSolids as unknown as (s: unknown) => unknown[])(result) ?? [];
  const empty = solids.length === 0;
  // 📥️Export FIRST, then measure what was written — see `reimport` above.
  const exportedText = empty ? "" : canonicalizeStep(await blobText(call(b, "exportSTEP", result)));
  const measured = empty ? null : await reimport(b, exportedText);

  // 🫙️An EMPTY result has no STEP body to export and no volume to measure. Writing a placeholder shape
  // would turn "correctly nothing" into "something", so the bundle records emptiness as the fact it is.
  const measurements: Record<string, unknown> = { declaredOutcome: recipe.outcome, solids: solids.length, empty };
  if (!empty && measured !== null) {
    write(join(dir, "expected.step"), exportedText);
    files.push({ role: "expected-step", path: `${recipe.id}/expected.step`, mediaType: "model/step", sha256: await contentDigest(exportedText), bytes: Buffer.byteLength(exportedText) });
    const bounds = call(b, "getBounds", measured) as Record<string, number>;
    measurements.measuredFrom = "expected.step, re-imported";
    measurements.solids = ((b.getSolids as unknown as (s: unknown) => unknown[])(measured) ?? []).length;
    measurements.volume = call(b, "measureVolume", measured);
    measurements.area = call(b, "measureArea", measured);
    measurements.boundingBox = bounds;
    measurements.boundingBoxDiagonal = Math.hypot(bounds.xMax! - bounds.xMin!, bounds.yMax! - bounds.yMin!, bounds.zMax! - bounds.zMin!);
    measurements.faces = ((b.getFaces as unknown as (s: unknown) => unknown[])(measured) ?? []).length;
    measurements.edges = ((b.getEdges as unknown as (s: unknown) => unknown[])(measured) ?? []).length;
    measurements.vertices = ((b.getVertices as unknown as (s: unknown) => unknown[])(measured) ?? []).length;
    measurements.validSolid = (b.isValidSolid as unknown as (s: unknown) => boolean)(measured);

    const tessellationTolerance = tessellationToleranceFor(measurements.boundingBoxDiagonal as number);
    const meshed = call(b, "mesh", measured, { tolerance: tessellationTolerance, angularTolerance: ANGULAR_TOLERANCE }) as Record<string, ArrayLike<number>>;
    const vertices = Array.from(meshed.vertices ?? meshed.positions!);
    const triangles = Array.from(meshed.triangles ?? meshed.indices!);
    const meshBody = `${JSON.stringify({ vertices, triangles, tolerance: tessellationTolerance, angularTolerance: ANGULAR_TOLERANCE })}\n`;
    write(join(dir, "expected.mesh.json"), meshBody);
    files.push({ role: "expected-mesh", path: `${recipe.id}/expected.mesh.json`, mediaType: "application/json", sha256: await contentDigest(meshBody), bytes: Buffer.byteLength(meshBody) });
    measurements.tessellationTolerance = tessellationTolerance;
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
    // 🏭️`reproducible: true` is a MEASURED fact, not a shrug: the qualification spike found exactly two
    // fields OCCT's STEP writer draws from process state rather than from the shape — the `FILE_NAME`
    // wall-clock timestamp and an incrementing translator counter stamped into the root `PRODUCT` — and
    // `canonicalizeStep` above strips both before this bundle is written or hashed. Two independent
    // full-corpus regenerations produced byte-identical output on every fixture once that ran; nothing
    // here reorders entities or renumbers ids.
    reproducible: true,
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
    else {
      // 🧬️A NARROWED run MERGES into the manifest index; it does not replace it. Writing only what this
      // invocation produced meant `generate --only <one>` silently reduced the index to that single
      // entry, so a sequence of narrowed runs — the natural way to develop a recipe — destroyed every
      // other fixture's record while leaving its files on disk. The bug was invisible from the command
      // itself: it reported success for exactly the fixture asked for.
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
    }
    console.error(`[generator] ${manifests.length}/${recipes.length} bundle(s) generated into ${outDir}${failed > 0 ? `, ${failed} failed` : ""}`);
    return failed > 0 ? 1 : 0;
  }
  console.error(`[generator] unknown command ${JSON.stringify(command)} — expected generate | manifests`);
  return 1;
}

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
//#endregion 🚪️Entry
