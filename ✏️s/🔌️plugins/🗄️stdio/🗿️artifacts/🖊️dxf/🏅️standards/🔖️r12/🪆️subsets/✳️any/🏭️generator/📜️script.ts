#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.dxf@r12/✳️any`.
//
// Every recipe's bytes are written entirely by the real `dxf` 0.6 crate's own typed `Drawing`
// model (`🦀️engine/src/main.rs`) — the SAME crate registered as `dxf-crate-r12-mutate`
// (`cross-semio-implementation`, untouched) and now also as `dxf-crate-r12-mutate-reader`
// (`third-party-library`) in `../🧪️oracle/🔣️.json` — never by this repository's own DXF codec.
// This script only marshals: it builds and invokes the Rust binary and reports what it wrote; it
// computes no DXF bytes itself.
//
// Two recipe SHAPES share one engine binary:
//   * `drafting-plate` — the pre-existing single-document fixture for the `cross-semio-implementation`
//     oracle's own testing shape. Untouched in content (verified byte-identical to the previously
//     committed fixture — see the ticket-root report). `RECIPES` below carries it with `kind:
//     "single"` so `manifests` reproduces its EXACT pre-existing entry shape (`files: [{role:
//     "primary-dxf", ...}]`, no `mutation`/`outcome`/`comparisonProfile`).
//   * one `<kind>-applied` / `<kind>-no-op` / `<kind>-rejected-<reason>` recipe per WITNESSABLE
//     mutation kind — `kind: "pair"`, `files: [expected-before-dxf, expected-after-dxf?]`, the new
//     corpus this retrofit adds for the `dxf-crate-r12-mutate-reader` oracle.
//
// Generation and execution are SEPARATE operations (a normal test run must never rewrite the
// expectation it is measured against): this command is the only one that writes into
// `../🧫️fixtures/`, and its output is reviewed and committed before any test reads it.
//
//   bun 📜️script.ts generate  [--only <recipe-id>]     # (re)builds the engine, writes fixture(s)
//   bun 📜️script.ts manifests [--only <recipe-id>]     # prints the fixtureManifests block (JSON array)
//
// @see ./🦀️engine/src/main.rs — the actual codec; `build <recipe-id> <out-dir>` and
//      `project <path>` are its only two data-producing commands (plus `list-recipes`).
// @see ../../../../💬️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🏭️generator/📜️script.ts — the sibling
//      generator this file's `--only`/RECIPES/generate/manifests shape is mirrored from.
// @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️dxf-r12-any-reader-oracle-retrofit.md

//#endregion 🧲️Header

//#region 🔌️Adapters
import { existsSync, mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
//#endregion 🔌️Adapters

//#region 🧬️Contract
const HERE = import.meta.dir;
const ENGINE_DIR = join(HERE, "🦀️engine");
const ENGINE_BIN = join(ENGINE_DIR, "target", "release", "generate");
const FIXTURE_PATH_PREFIX = "../🧫️fixtures/";
const READER_ORACLE_ID = "dxf-crate-r12-mutate-reader";
const CROSS_SEMIO_ORACLE_ID = "dxf-crate-r12-mutate";
const ENGINE_VERSION = "0.6.1";

type Outcome = "applied" | "no-op" | "rejected";
type Recipe = Readonly<{ id: string; kind: "single" | "pair"; mutation: string | null; outcome: Outcome | null; notes: string }>;

/** 🍳️ Mirrors `RECIPE_IDS`/`recipe()` in `🦀️engine/src/main.rs` verbatim — `drafting-plate` (the
 *  pre-existing single-document fixture, untouched) plus one entry per witnessable `(mutation,
 *  outcome)` coordinate this retrofit adds. `set-header-var` carries `-applied` only — see that
 *  file's own comment for the reachability proof (no fixture-less claim: verified from the real
 *  validation code, not asserted). */
const RECIPES: readonly Recipe[] = [
  { id: "drafting-plate", kind: "single", mutation: null, outcome: null, notes: "Pre-existing base document for the cross-semio-implementation oracle's own testing shape. Untouched in content by this retrofit." },
  { id: "no-mutation-no-op", kind: "pair", mutation: "no-mutation", outcome: "no-op", notes: "Identity — before and after are the same document, encoded independently." },
  { id: "set-snapshot-applied", kind: "pair", mutation: "set-snapshot", outcome: "applied", notes: "Whole-document replace: $INSBASE, a new LAYER, and the circle entity's radius (the subset's own declared 'widens-the-circle-entity-radius' scenario) all change together." },
  { id: "set-snapshot-no-op", kind: "pair", mutation: "set-snapshot", outcome: "no-op", notes: "Replacement snapshot equals the base — DxfDiff::between is empty." },
  { id: "set-snapshot-rejected-duplicate-layer", kind: "pair", mutation: "set-snapshot", outcome: "rejected", notes: "Payload snapshot declares a layer name that collides with an existing base layer — invalid-add-target (🔺️diff/🦀️component.rs:1571)." },
  { id: "set-header-var-applied", kind: "pair", mutation: "set-header-var", outcome: "applied", notes: "$INSBASE changes value — the one generic $VAR dxf's Header persists unconditionally on an R12 save." },
  { id: "remove-header-var-applied", kind: "pair", mutation: "remove-header-var", outcome: "applied", notes: "$INSBASE resets to the origin." },
  { id: "remove-header-var-rejected-missing", kind: "pair", mutation: "remove-header-var", outcome: "rejected", notes: "Target name is genuinely absent from header_vars — invalid-remove-target, carrier-independent." },
  { id: "insert-layer-applied", kind: "pair", mutation: "insert-layer", outcome: "applied", notes: "A new named LAYER row is inserted at a valid index." },
  { id: "insert-layer-rejected-duplicate", kind: "pair", mutation: "insert-layer", outcome: "rejected", notes: "Target name already exists in the base layer table — invalid-add-target." },
  { id: "remove-layer-applied", kind: "pair", mutation: "remove-layer", outcome: "applied", notes: "The named LAYER row is removed." },
  { id: "remove-layer-rejected-missing", kind: "pair", mutation: "remove-layer", outcome: "rejected", notes: "Target name is absent — invalid-remove-target." },
  { id: "set-layer-applied", kind: "pair", mutation: "set-layer", outcome: "applied", notes: "The named LAYER row's colour and linetype are whole-value replaced." },
  { id: "set-layer-rejected-missing", kind: "pair", mutation: "set-layer", outcome: "rejected", notes: "Target name is absent — invalid-modify-target (diff_set_layer always emits a modified entry, unlike set-header-var)." },
  { id: "insert-style-applied", kind: "pair", mutation: "insert-style", outcome: "applied", notes: "A new named STYLE row is inserted at a valid index." },
  { id: "insert-style-rejected-duplicate", kind: "pair", mutation: "insert-style", outcome: "rejected", notes: "Target name already exists — invalid-add-target." },
  { id: "remove-style-applied", kind: "pair", mutation: "remove-style", outcome: "applied", notes: "The named STYLE row is removed." },
  { id: "remove-style-rejected-missing", kind: "pair", mutation: "remove-style", outcome: "rejected", notes: "Target name is absent — invalid-remove-target." },
  { id: "set-style-applied", kind: "pair", mutation: "set-style", outcome: "applied", notes: "The named STYLE row's font is whole-value replaced." },
  { id: "set-style-rejected-missing", kind: "pair", mutation: "set-style", outcome: "rejected", notes: "Target name is absent — invalid-modify-target." },
  { id: "insert-linetype-applied", kind: "pair", mutation: "insert-linetype", outcome: "applied", notes: "A new named LTYPE row is inserted at a valid index." },
  { id: "insert-linetype-rejected-duplicate", kind: "pair", mutation: "insert-linetype", outcome: "rejected", notes: "Target name already exists — invalid-add-target." },
  { id: "remove-linetype-applied", kind: "pair", mutation: "remove-linetype", outcome: "applied", notes: "The named LTYPE row is removed." },
  { id: "remove-linetype-rejected-missing", kind: "pair", mutation: "remove-linetype", outcome: "rejected", notes: "Target name is absent — invalid-remove-target." },
  { id: "set-linetype-applied", kind: "pair", mutation: "set-linetype", outcome: "applied", notes: "The named LTYPE row's description is whole-value replaced." },
  { id: "set-linetype-rejected-missing", kind: "pair", mutation: "set-linetype", outcome: "rejected", notes: "Target name is absent — invalid-modify-target." },
  { id: "insert-entity-applied", kind: "pair", mutation: "insert-entity", outcome: "applied", notes: "A new entity is inserted at a valid middle index." },
  { id: "insert-entity-rejected-out-of-bounds", kind: "pair", mutation: "insert-entity", outcome: "rejected", notes: "Index exceeds the evolving sequence length — invalid-add-index." },
  { id: "remove-entity-applied", kind: "pair", mutation: "remove-entity", outcome: "applied", notes: "The entity at a valid middle index is removed." },
  { id: "remove-entity-rejected-missing", kind: "pair", mutation: "remove-entity", outcome: "rejected", notes: "Index does not exist — invalid-remove-index." },
  { id: "set-entity-applied", kind: "pair", mutation: "set-entity", outcome: "applied", notes: "The entity at a present index is whole-value replaced (the genuine modify branch, not the insert fallback)." },
  { id: "set-entity-rejected-out-of-bounds", kind: "pair", mutation: "set-entity", outcome: "rejected", notes: "Index is absent AND beyond the evolving length, so even the insert fallback rejects it — invalid-add-index." },
  { id: "insert-block-applied", kind: "pair", mutation: "insert-block", outcome: "applied", notes: "A new BLOCK is inserted at a valid index." },
  { id: "insert-block-rejected-out-of-bounds", kind: "pair", mutation: "insert-block", outcome: "rejected", notes: "Index exceeds the evolving sequence length — invalid-add-index." },
  { id: "remove-block-applied", kind: "pair", mutation: "remove-block", outcome: "applied", notes: "The BLOCK at a valid index is removed." },
  { id: "remove-block-rejected-missing", kind: "pair", mutation: "remove-block", outcome: "rejected", notes: "Index does not exist — invalid-remove-index." },
  { id: "set-block-applied", kind: "pair", mutation: "set-block", outcome: "applied", notes: "The BLOCK at a present index has its base point and nested entities whole-value replaced." },
  { id: "set-block-rejected-out-of-bounds", kind: "pair", mutation: "set-block", outcome: "rejected", notes: "Index is absent AND beyond the evolving length — invalid-add-index via the insert fallback." },
];
//#endregion 🧬️Contract

//#region 🔨️Build
function ensureBuilt(): void {
  const result = spawnSync("cargo", ["build", "--release", "--manifest-path", join(ENGINE_DIR, "Cargo.toml")], { stdio: "inherit" });
  if (result.status !== 0) throw new Error(`cargo build failed with status ${result.status}`);
}

async function sha256(bytes: Buffer): Promise<string> {
  const digest = await crypto.subtle.digest("SHA-256", bytes);
  return `sha256:${[...new Uint8Array(digest)].map((byte) => byte.toString(16).padStart(2, "0")).join("")}`;
}

function platformId(): string {
  const os = process.platform === "win32" ? "win32" : process.platform === "darwin" ? "darwin" : "linux";
  const arch = process.arch === "arm64" ? "arm64" : "x64";
  return `${os}-${arch}`;
}

/** 🦀️ Shells out to the standalone engine binary — the ONLY place this file touches it. */
function engineBuild(id: string, outDir: string): void {
  const result = spawnSync(ENGINE_BIN, ["build", id, outDir], { encoding: "utf8" });
  if (result.status !== 0) {
    throw new Error(`engine build ${id} failed (exit ${result.status}): ${result.stderr}`);
  }
}
//#endregion 🔨️Build

//#region 🏭️Generate
async function fileEntry(role: string, dir: string, filename: string, id: string): Promise<{ role: string; path: string; mediaType: string; sha256: string; bytes: number }> {
  const abs = join(dir, filename);
  const bytes = readFileSync(abs);
  return { role, path: `${FIXTURE_PATH_PREFIX}${id}/${filename}`, mediaType: "image/vnd.dxf", sha256: await sha256(bytes), bytes: bytes.length };
}

async function manifestForSingle(recipe: Recipe, outDir: string): Promise<Record<string, unknown>> {
  const dir = join(outDir, recipe.id);
  const filename = `${recipe.id}.dxf`;
  engineBuild(recipe.id, outDir);
  const bytes = readFileSync(join(dir, filename));
  return {
    schema: "semio.repository-test.fixture/v2",
    id: recipe.id,
    class: "third-party-generated",
    family: "mechanical",
    files: [{ role: "primary-dxf", path: `${FIXTURE_PATH_PREFIX}${recipe.id}/${filename}`, mediaType: "image/vnd.dxf", sha256: await sha256(bytes), bytes: bytes.length }],
    provenance: { source: "generated", license: "public-domain (synthetic, no third-party content embedded)" },
    generator: { oracle: CROSS_SEMIO_ORACLE_ID, packageVersion: "0.6", engineFamily: "dxf-rs", engineVersion: "0.6", command: `bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🏭️generator/📜️script.ts generate --only ${recipe.id}`, platform: process.platform },
    reproducible: true,
  };
}

async function manifestForPair(recipe: Recipe, outDir: string): Promise<Record<string, unknown>> {
  const dir = join(outDir, recipe.id);
  engineBuild(recipe.id, outDir);
  const files: Awaited<ReturnType<typeof fileEntry>>[] = [await fileEntry("expected-before-dxf", dir, "before.dxf", recipe.id)];
  const hasAfter = existsSync(join(dir, "after.dxf"));
  if (recipe.outcome === "rejected") {
    if (hasAfter) throw new Error(`recipe ${recipe.id} is declared rejected but the engine produced an after.dxf anyway`);
  } else {
    if (!hasAfter) throw new Error(`recipe ${recipe.id} is declared ${recipe.outcome} but the engine produced no after.dxf`);
    files.push(await fileEntry("expected-after-dxf", dir, "after.dxf", recipe.id));
  }

  return {
    schema: "semio.repository-test.fixture/v2",
    id: recipe.id,
    class: "third-party-generated",
    target: { artifact: "s.stdio.dxf", standard: "r12", subset: "any" },
    mutation: recipe.mutation,
    outcome: recipe.outcome,
    units: { length: "unitless", angle: "degree" },
    files,
    generator: {
      oracle: READER_ORACLE_ID,
      packageVersion: "0.6",
      engineFamily: "dxf-rs",
      engineVersion: ENGINE_VERSION,
      command: `bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🏭️generator/📜️script.ts generate --only ${recipe.id}`,
      platform: platformId(),
    },
    provenance: { source: "generated", license: "MIT (dxf)", attribution: "Generated with dxf (MIT) via the standalone engine binary in this same directory", security: "scanned-clean", privacy: "no-personal-data" },
    comparisonProfile: "semantic-dxf-r12-v1",
    reproducible: true,
    family: "structural",
    notes: recipe.notes,
  };
}
//#endregion 🏭️Generate

//#region 🚪️Entry
async function main(argv: readonly string[]): Promise<number> {
  const [command = "", ...rest] = argv;
  const outFlagIndex = rest.indexOf("--out");
  const onlyIndex = rest.indexOf("--only");
  const only = onlyIndex === -1 ? null : (rest[onlyIndex + 1] ?? null);
  // 🧭️ `SEMIO_FIXTURE_OUT` (set by `test fixture reproduce`/`generate`) is a FIXTURES ROOT, not a
  // per-recipe directory — every generator in the repository writes `<root>/<recipe>/<file>`.
  const fixtureOutRoot = process.env.SEMIO_FIXTURE_OUT;
  const outDir = outFlagIndex >= 0 ? rest[outFlagIndex + 1]! : fixtureOutRoot !== undefined && fixtureOutRoot.length > 0 ? fixtureOutRoot : join(HERE, "..", "🧫️fixtures");

  if (command !== "generate" && command !== "manifests") {
    console.error(`usage: 📜️script.ts <generate|manifests> [--only <recipe-id>] [--out <dir>]`);
    return 2;
  }

  const recipes = only === null ? RECIPES : RECIPES.filter((recipe) => recipe.id === only);
  if (recipes.length === 0) {
    console.error(`[dxf generator] no recipe matches ${JSON.stringify(only)} — known: ${RECIPES.map((recipe) => recipe.id).join(", ")}`);
    return 1;
  }

  mkdirSync(outDir, { recursive: true });
  ensureBuilt();

  const manifests: Record<string, unknown>[] = [];
  let failed = 0;
  for (const recipe of recipes) {
    try {
      const manifest = recipe.kind === "single" ? await manifestForSingle(recipe, outDir) : await manifestForPair(recipe, outDir);
      manifests.push(manifest);
      console.error(`[dxf generator] ${recipe.id}${recipe.mutation ? ` (${recipe.mutation}/${recipe.outcome})` : ""}`);
    } catch (error) {
      // 🧭️A recipe the engine refuses is REPORTED, never dropped.
      failed += 1;
      console.error(`[dxf generator] ${recipe.id} FAILED — ${(error as Error).message}`);
    }
  }

  if (command === "manifests") {
    process.stdout.write(`${JSON.stringify(manifests, null, 2)}\n`);
  }
  console.error(`[dxf generator] ${manifests.length}/${recipes.length} bundle(s) generated into ${outDir}${failed > 0 ? `, ${failed} failed` : ""}`);
  return failed > 0 ? 1 : 0;
}

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
//#endregion 🚪️Entry
