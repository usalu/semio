#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.obj@3.0/📐️geometry`.
//
// Two independent recipe FAMILIES, each mirroring the sibling `📼️avi/🏅️standards/🔖️1.0` generator's
// shape (`generate [--only <id>]`, `SEMIO_FIXTURE_OUT` read as a fixtures ROOT — every generator in
// the repository writes `<root>/<recipe>/<file>`):
//   * `pattern-shell` — the ORIGINAL single-fixture recipe (unchanged since it was first committed),
//     built by `../🦀️engine` and admitted through `tobj` 4. Its own `fixtureManifests` entry's
//     `generator.command` still reads plain `generate` with no `--only`, so that exact invocation
//     must keep producing exactly this one file — this script preserves that path byte-for-byte.
//   * the 20 reader-oracle corpus recipes (`no-mutation-no-op`, `set-snapshot-applied`, …) — built by
//     `../📖️tobj-obj-reader`'s `build <recipe-id> <out-dir>`, one dedicated before/after (or
//     before-only, for `-rejected-`) pair per WITNESSABLE mutation kind. See that crate's module
//     doc for exactly which 12 of the 22 declared kinds are witnessable by a pure `tobj` mesh
//     reader and why the other 10 are not (`obj-3-0-mutate-uncarried` instead).
//
// Generation and execution are SEPARATE operations, per the shared framework's own rule (a normal
// test run must never rewrite the expectation it is measured against): this command is the only one
// that writes into `../🧫️fixtures/`, and its output is reviewed and committed before any test reads it.
//
//   bun 📜️script.ts generate [--only <recipe-id>] [--out <dir>]   # (re)builds + writes fixture(s)
//   bun 📜️script.ts manifests [--only <recipe-id>]                 # prints the fixtureManifests entry/entries
//   bun 📜️script.ts list-recipes                                   # every known recipe id
//
// @see ../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️obj-3-0-any-reader-oracle-retrofit.md

//#endregion 🧲️Header

//#region 🔌️Adapters
import { existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
//#endregion 🔌️Adapters

//#region 🧬️Contract
const HERE = import.meta.dir;
const FIXTURES_DIR = join(HERE, "..", "🧫️fixtures");

const LEGACY_ENGINE_DIR = join(HERE, "🦀️engine");
const LEGACY_ENGINE_BIN = join(LEGACY_ENGINE_DIR, "target", "release", process.platform === "win32" ? "generate.exe" : "generate");
const LEGACY_RECIPE = "pattern-shell";
const LEGACY_DIRECTORY = "🐚️pattern-shell";
const LEGACY_FIXTURE_FILE = "🐚️pattern-shell.obj";

const READER_MANIFEST = join(HERE, "📖️tobj-obj-reader", "Cargo.toml");
const READER_BIN = join(HERE, "📖️tobj-obj-reader", "target", "release", process.platform === "win32" ? "tobj-obj-reader.exe" : "tobj-obj-reader");

/** 🍳️ One entry per reader-oracle corpus recipe — mirrors `📖️tobj-obj-reader/src/main.rs`'s own
 *  `RECIPES` table (id + whether it has an `after.obj`), kept in sync by hand since this file never
 *  parses Rust. */
type ReaderRecipe = Readonly<{ id: string; directoryName: string; hasAfter: boolean }>;
const READER_RECIPES: readonly ReaderRecipe[] = [
  { id: "no-mutation-no-op", directoryName: "⏸️no-mutation-no-op", hasAfter: true },
  { id: "set-snapshot-applied", directoryName: "📸️set-snapshot-applied", hasAfter: true },
  { id: "set-vertex-applied", directoryName: "📍️set-vertex-applied", hasAfter: true },
  { id: "set-vertex-rejected-out-of-bounds", directoryName: "⛔️set-vertex-rejected-out-of-bounds", hasAfter: false },
  { id: "set-texcoord-applied", directoryName: "🧭️set-texcoord-applied", hasAfter: true },
  { id: "set-texcoord-rejected-out-of-bounds", directoryName: "🗺️set-texcoord-rejected-out-of-bounds", hasAfter: false },
  { id: "set-normal-applied", directoryName: "🧲️set-normal-applied", hasAfter: true },
  { id: "set-normal-rejected-out-of-bounds", directoryName: "🚧️set-normal-rejected-out-of-bounds", hasAfter: false },
  { id: "insert-face-applied", directoryName: "🔷️insert-face-applied", hasAfter: true },
  { id: "insert-face-rejected-out-of-bounds", directoryName: "🚷️insert-face-rejected-out-of-bounds", hasAfter: false },
  { id: "remove-face-applied", directoryName: "🗑️remove-face-applied", hasAfter: true },
  { id: "remove-face-rejected-missing", directoryName: "🕵️remove-face-rejected-missing", hasAfter: false },
  { id: "set-face-applied", directoryName: "🔶️set-face-applied", hasAfter: true },
  { id: "set-face-rejected-out-of-bounds", directoryName: "🛑️set-face-rejected-out-of-bounds", hasAfter: false },
  { id: "set-group-applied", directoryName: "🏷️set-group-applied", hasAfter: true },
  { id: "remove-group-applied", directoryName: "🪓️remove-group-applied", hasAfter: true },
  { id: "remove-group-rejected-missing", directoryName: "📭️remove-group-rejected-missing", hasAfter: false },
  { id: "set-object-applied", directoryName: "📦️set-object-applied", hasAfter: true },
  { id: "remove-object-applied", directoryName: "🗃️remove-object-applied", hasAfter: true },
  { id: "remove-object-rejected-missing", directoryName: "👻️remove-object-rejected-missing", hasAfter: false },
];
const DOCUMENT_COORDINATES: Readonly<Record<string, readonly [string, string]>> = {
  "set-mtllib": ["🎨️material", "🎨️set-mtllib"],
  "set-usemtl": ["🎨️material", "🖌️set-usemtl"],
  "set-smoothing-groups": ["📐️geometry", "🧵️set-smoothing-groups"],
  "insert-vertex": ["📐️geometry", "➕️insert-vertex"],
  "remove-vertex": ["📐️geometry", "➖️remove-vertex"],
  "insert-texcoord": ["📐️geometry", "🧷️insert-texcoord"],
  "remove-texcoord": ["📐️geometry", "🚮️remove-texcoord"],
  "insert-normal": ["📐️geometry", "📐️insert-normal"],
  "remove-normal": ["📐️geometry", "🚫️remove-normal"],
  "set-unknown-statements": ["📐️geometry", "🕳️set-unknown-statements"],
};
//#endregion 🧬️Contract

//#region 🔨️Build
function ensureLegacyBuilt(): void {
  const result = spawnSync("cargo", ["build", "--release", "--manifest-path", join(LEGACY_ENGINE_DIR, "Cargo.toml")], { stdio: "inherit" });
  if (result.status !== 0) throw new Error(`cargo build (legacy engine) failed with status ${result.status}`);
}

function ensureReaderBuilt(): void {
  const result = spawnSync("cargo", ["build", "--release", "--manifest-path", READER_MANIFEST], { stdio: "inherit" });
  if (result.status !== 0) throw new Error(`cargo build (tobj-obj-reader) failed with status ${result.status}`);
}

async function sha256(path: string): Promise<string> {
  const bytes = readFileSync(path);
  const digest = await crypto.subtle.digest("SHA-256", bytes);
  return `sha256:${[...new Uint8Array(digest)].map((byte) => byte.toString(16).padStart(2, "0")).join("")}`;
}
//#endregion 🔨️Build

//#region 🚪️Generate
async function generateLegacy(outDir: string): Promise<void> {
  ensureLegacyBuilt();
  mkdirSync(outDir, { recursive: true });
  const outPath = join(outDir, LEGACY_FIXTURE_FILE);
  const result = spawnSync(LEGACY_ENGINE_BIN, [outPath], { stdio: "inherit" });
  if (result.status !== 0) throw new Error(`legacy generator binary failed with status ${result.status}`);
  const digest = await sha256(outPath);
  console.log(`${outPath}\n${digest}`);
}

function generateReaderRecipe(recipe: ReaderRecipe, fixturesRoot: string): void {
  ensureReaderBuilt();
  mkdirSync(fixturesRoot, { recursive: true });
  const result = spawnSync(READER_BIN, ["build", recipe.id, join(fixturesRoot, recipe.directoryName)], { stdio: "inherit" });
  if (result.status !== 0) throw new Error(`tobj-obj-reader build ${recipe.id} failed with status ${result.status}`);
}

/** 🚪️ `--only <id>` dispatches to whichever family owns that id; NO `--only` rebuilds ONLY the
 *  legacy `pattern-shell` recipe — the exact behavior its own committed `fixtureManifests` entry's
 *  `generator.command` (`generate`, no flags) has always invoked, unchanged. The 20 reader-oracle
 *  corpus recipes are only ever built when named explicitly via `--only` (each carries its own
 *  `fixtureManifests` entry whose `generator.command` always names one), so a bare `generate` never
 *  grows to rebuild the whole corpus underneath the one recorded pattern-shell invocation. */
async function generate(outFlagDir: string | null, only: string | null): Promise<void> {
  const fixtureOutRoot = process.env.SEMIO_FIXTURE_OUT;
  const root = outFlagDir ?? (fixtureOutRoot !== undefined && fixtureOutRoot.length > 0 ? fixtureOutRoot : FIXTURES_DIR);
  if (only === null || only === LEGACY_RECIPE) {
    await generateLegacy(join(root, LEGACY_DIRECTORY));
    return;
  }
  const recipe = READER_RECIPES.find((entry) => entry.id === only);
  if (recipe === undefined) {
    throw new Error(`unknown recipe ${JSON.stringify(only)} — known: ${[LEGACY_RECIPE, ...READER_RECIPES.map((r) => r.id)].join(", ")}`);
  }
  generateReaderRecipe(recipe, root);
}
//#endregion 🚪️Generate

//#region 🚪️Manifests
async function fileEntry(role: string, dir: string, filename: string, recipeId: string): Promise<{ role: string; path: string; mediaType: string; sha256: string; bytes: number }> {
  const abs = join(dir, filename);
  return { role, path: `../🧫️fixtures/${recipeId}/${filename}`, mediaType: "model/obj", sha256: await sha256(abs), bytes: statSync(abs).size };
}

async function manifestForLegacy(): Promise<Record<string, unknown>> {
  const dir = join(FIXTURES_DIR, LEGACY_DIRECTORY);
  const outPath = join(dir, LEGACY_FIXTURE_FILE);
  if (!existsSync(outPath)) throw new Error(`${outPath} does not exist — run "generate" first`);
  return {
    schema: "semio.repository-test.fixture/v2",
    id: LEGACY_RECIPE,
    class: "third-party-generated",
    target: { artifact: "s.stdio.obj", standard: "3.0", subset: "geometry" },
    units: { length: "unitless", angle: "degree" },
    files: [await fileEntry("primary-obj", dir, LEGACY_FIXTURE_FILE, LEGACY_DIRECTORY)],
    generator: { oracle: "tobj-obj-3-0-mutate", packageVersion: "4", engineFamily: "tobj", engineVersion: "4", command: `bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📜️script.ts generate`, platform: `${process.platform}-${process.arch}` },
    provenance: { source: "generated", license: "public-domain (synthetic, no third-party content embedded)" },
    comparisonProfile: "semantic-obj-3-0-v1",
    reproducible: true,
    family: "mechanical",
  };
}

async function manifestForReaderRecipe(recipe: ReaderRecipe): Promise<Record<string, unknown>> {
  const dir = join(FIXTURES_DIR, recipe.directoryName);
  const beforePath = join(dir, "⬅️before.obj");
  if (!existsSync(beforePath)) throw new Error(`${beforePath} does not exist — run "generate --only ${recipe.id}" first`);
  const files = [await fileEntry("expected-before-obj", dir, "⬅️before.obj", recipe.directoryName)];
  if (recipe.hasAfter) files.push(await fileEntry("expected-after-obj", dir, "➡️after.obj", recipe.directoryName));
  return {
    schema: "semio.repository-test.fixture/v2",
    id: recipe.id,
    class: "third-party-generated",
    target: { artifact: "s.stdio.obj", standard: "3.0", subset: "geometry" },
    units: { length: "unitless", angle: "degree" },
    files,
    generator: { oracle: "tobj-obj-3-0-mutate-reader", packageVersion: "4", engineFamily: "tobj", engineVersion: "4", command: `bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📜️script.ts generate --only ${recipe.id}`, platform: `${process.platform}-${process.arch}` },
    provenance: { source: "generated", license: "public-domain (synthetic, no third-party content embedded)" },
    comparisonProfile: "semantic-obj-3-0-v1",
    reproducible: true,
    family: "structural",
  };
}

async function manifests(only: string | null): Promise<void> {
  const entries: Record<string, unknown>[] = [];
  if (only === null || only === LEGACY_RECIPE) entries.push(await manifestForLegacy());
  const readerTargets = only === null ? READER_RECIPES : READER_RECIPES.filter((recipe) => recipe.id === only);
  for (const recipe of readerTargets) entries.push(await manifestForReaderRecipe(recipe));
  console.log(JSON.stringify(only === null ? entries : entries[0], null, 2));
}
//#endregion 🚪️Manifests

//#region 🚪️Entry
async function main(argv: readonly string[]): Promise<number> {
  const [command = "", ...rest] = argv;
  const outFlagIndex = rest.indexOf("--out");
  const outFlagDir = outFlagIndex >= 0 ? (rest[outFlagIndex + 1] ?? null) : null;
  const onlyFlagIndex = rest.indexOf("--only");
  const only = onlyFlagIndex >= 0 ? (rest[onlyFlagIndex + 1] ?? null) : null;
  if (command === "generate") {
    await generate(outFlagDir, only);
    return 0;
  }
  if (command === "manifests") {
    await manifests(only);
    return 0;
  }
  if (command === "list-recipes") {
    console.log([LEGACY_RECIPE, ...READER_RECIPES.map((r) => r.id)].join("\n"));
    return 0;
  }
  // 🧾️DOCUMENT MODE — the three statement kinds `tobj` cannot see.
  //
  // `tobj` is a MESH reader: it resolves faces into buffers and discards `mtllib`, `usemtl` and
  // smoothing-group statements entirely, which is why those kinds were `-uncarried` against it. They
  // are plain OBJ statements, so the fixtures are handcrafted OBJ text — admissible under this
  // repository's own precedent (svg and xml were built the same way) — and the JUDGE is a different
  // third-party implementation, `three`'s OBJLoader, which parses and keeps them.
  //
  // Only the three kinds MEASURED to move that reader's projection are written here. The
  // vertex/texcoord/normal insert-and-remove kinds and `set-unknown-statements` were measured NOT to
  // move it (an unreferenced element is dropped by this loader too; an unknown statement is skipped
  // with a warning) and stay `-uncarried` rather than being claimed.
  if (command === "document" || command === "document-manifests") {
    const BASE = ["mtllib base.mtl", "v 0 0 0", "v 1 0 0", "v 0 1 0", "vt 0 0", "vt 1 0", "vt 0 1", "vn 0 0 1", "vn 1 0 0", "usemtl red", "s 1", "f 1/1/1 2/2/1 3/3/1", ""].join("\n");
    // 🔢️The v/vt/vn kinds insert or remove at the FRONT of their list. OBJ face indices are ABSOLUTE,
    // so a front edit changes what every subsequent index resolves to and the mutation becomes visible
    // to a mesh reader. An edit past the last referenced element would not be — that is a real limit of
    // reading OBJ through a mesh loader, and it is stated in the oracle's rationale rather than hidden
    // by a fixture that happens to avoid it.
    const PAIRS: Record<string, string> = {
      "set-mtllib": BASE.replace("mtllib base.mtl", "mtllib other.mtl"),
      "set-usemtl": BASE.replace("usemtl red", "usemtl blue"),
      "set-smoothing-groups": BASE.replace("s 1", "s off"),
      "insert-vertex": BASE.replace("v 0 0 0", "v 9 9 9\nv 0 0 0"),
      "remove-vertex": BASE.replace("v 0 0 0\n", "").replace("f 1/1/1 2/2/1 3/3/1", "f 1/1/1 2/2/1 2/3/1"),
      "insert-texcoord": BASE.replace("vt 0 0", "vt 0.7 0.7\nvt 0 0"),
      "remove-texcoord": BASE.replace("vt 0 0\n", "").replace("f 1/1/1 2/2/1 3/3/1", "f 1/1/1 2/2/1 3/2/1"),
      "insert-normal": BASE.replace("vn 0 0 1", "vn 0 1 0\nvn 0 0 1"),
      "remove-normal": BASE.replace("vn 0 0 1\n", ""),
      // 🧾️`set-unknown-statements` is instantiated with an `l` POLYLINE. This subset's codec models
      // v/vt/vn/f/g/o/mtllib/usemtl/s and nothing else, so `l` lands in `unknown_statements` — while
      // three's OBJLoader parses it into a Line and the projection moves. Measured alongside the
      // instances that do NOT work: a `#` comment and a truly unrecognised directive (`zz custom 42`)
      // are both invisible to this reader, and the oracle's rationale says so rather than implying the
      // kind is covered for every statement.
      "set-unknown-statements": BASE + "l 1 2 3\n",
    };
    const probes = join(HERE, "..", "🔬️probes", "📜️script.ts");
    const projectionOf = (path: string): string => {
      const result = spawnSync("bun", [probes, "obj-document-project", "--input", path], { encoding: "utf8" });
      if (result.status !== 0) throw new Error(`document reader refused ${path}: ${result.stdout}${result.stderr}`);
      return JSON.stringify(JSON.parse(result.stdout).measurements);
    };
    if (command === "document") {
      const failures: string[] = [];
      for (const [kind, after] of Object.entries(PAIRS)) {
        const [subset, name] = DOCUMENT_COORDINATES[kind]!;
        const dir = join(outFlagDir ?? process.env.SEMIO_FIXTURE_OUT ?? join(HERE, "..", "..", subset, "🧫️fixtures"), name);
        mkdirSync(dir, { recursive: true });
        writeFileSync(join(dir, "⬅️before.obj"), BASE);
        writeFileSync(join(dir, "➡️after.obj"), after);
        // 🔍️Observability through the READER, before the pair is allowed to stand.
        if (projectionOf(join(dir, "⬅️before.obj")) === projectionOf(join(dir, "➡️after.obj"))) failures.push(`${kind}: not observable in the document projection`);
        else console.error(`[generator] ${kind}: observable`);
      }
      for (const failure of failures) console.error(`[generator] ${failure}`);
      return failures.length > 0 ? 1 : 0;
    }
    const entries = [];
    for (const kind of Object.keys(PAIRS)) {
      const [subset, directoryName] = DOCUMENT_COORDINATES[kind]!;
      const dir = join(HERE, "..", "..", subset, "🧫️fixtures", directoryName);
      if (!existsSync(dir)) throw new Error(`missing fixture directory for ${kind} — run document first`);
      const files = [];
      for (const [role, name] of [["expected-before-obj", "⬅️before.obj"], ["expected-after-obj", "➡️after.obj"]] as const) {
        const path = join(dir, name);
        files.push({ role, path: `../🧫️fixtures/${directoryName}/${name}`, mediaType: "model/obj", sha256: await sha256(path), bytes: statSync(path).size });
      }
      entries.push({
        schema: "semio.repository-test.fixture/v2",
        id: `document-${kind}`,
        class: "handcrafted",
        target: { artifact: "s.stdio.obj", standard: "3.0", subset: subset === "🎨️material" ? "material" : "geometry" },
        mutation: kind,
        outcome: "applied",
        units: { length: "unitless", angle: "radian" },
        files,
        provenance: { source: "handcrafted", license: "public-domain (synthetic, no third-party content embedded)" },
        generator: { oracle: "three-obj-3-0-document-reader", packageVersion: "0.182.0", engineFamily: "threejs", engineVersion: "0.182.0", command: "bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📜️script.ts document", platform: process.platform },
        comparisonProfile: "semantic-obj-document-v1",
        reproducible: true,
        family: "mechanical",
        notes: `A minimal handcrafted OBJ document with the ${kind} statement changed. tobj, this subset's other reader, is a MESH reader and discards this statement entirely; three's OBJLoader parses and keeps it. Observability was checked through that reader before the pair was written.`,
      });
    }
    process.stdout.write(`${JSON.stringify(entries, null, 2)}\n`);
    return 0;
  }
  console.error(`usage: 📜️script.ts <generate|manifests|list-recipes|document|document-manifests> [--only <recipe-id>] [--out <dir>]`);
  return 2;
}

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
//#endregion 🚪️Entry
