#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.svg@1.1/🧱️base`.
//
// Every recipe's BEFORE and AFTER `.svg` bytes are built DIRECTLY by the sibling standalone
// `🦀️quick-xml-svg-codec` binary, which depends on nothing but `quick-xml` 0.42 — never by
// "applying" this repository's own `SvgMutation`/`SvgDiff` dispatch, and never by this subset's
// own `🦀️oracle.rs` (that module COMPUTES what a mutation should produce; it is a
// SEPARATE, untouched `cross-semio-implementation` oracle). This file only shells out per recipe
// and turns the bytes the codec wrote into a fixture bundle + manifest entry; it computes no SVG
// semantics of its own.
//
// Generation and execution are SEPARATE operations, same shape as the sibling `avi`/`bcf`
// generators this file's CLI is mirrored from: a normal test run must never be able to rewrite
// the expectation it is measured against.
//
//   bun 📜️script.ts generate  [--only <fixture-id>]     # writes the semantic fixture directory's ⬅️before/➡️after pair
//   bun 📜️script.ts manifests [--only <fixture-id>]     # prints the fixtureManifests block (JSON)
//
// @see ../../../../../📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🏭️generator/📜️script.ts — the
//      sibling generator this file's CLI/recipe shape is mirrored from.
// @see ./🦀️quick-xml-svg-codec/src/main.rs — the actual codec; `build <recipe-id> <out-dir> <physical-dir> <before-file> <after-file>`,
//      `project <path>` and `list-recipes` are its only three commands.
// @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/

//#endregion 🧲️Header

//#region 🔌️Adapters
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
//#endregion 🔌️Adapters

//#region 🧬️Contract
const CODEC_MANIFEST = join(import.meta.dir, "🦀️quick-xml-svg-codec", "Cargo.toml");
const ORACLE_ID = "quick-xml-svg-1-1-mutate-reader";
const ENGINE_FAMILY = "quick-xml";
const ENGINE_VERSION = "0.42.0";

type Recipe = Readonly<{ id: string; mutation: string; notes: string }>;

/** 🍳️ Mirrors `RECIPE_IDS`/`recipe()` in `🦀️quick-xml-svg-codec/src/main.rs` verbatim — one
 *  `-applied` entry per declared `SvgMutation` kind. Every one of the 9 kinds is registered
 *  `outcomes: ["applied"]` only in `../🔮️oracle/🔣️.json`: every `SvgMutation` leaf's own `diff()`
 *  under `../🧬️schema/🧬️mutations/` unconditionally returns `MutationOutcome::new(..)`,
 *  never `empty`/`error`/`fatal` — so there is no `-rejected-*` recipe to build here. */
const RECIPES: readonly Recipe[] = [
  { id: "set-declaration-applied", mutation: "set-declaration", notes: "The XML declaration gains a standalone=\"no\"; doctype/root untouched." },
  { id: "set-doctype-applied", mutation: "set-doctype", notes: "The DOCTYPE is replaced with a simplified SYSTEM form; declaration/root untouched." },
  { id: "insert-element-applied", mutation: "insert-element", notes: "A new <circle> is appended as the 3rd child of <g>." },
  { id: "remove-element-applied", mutation: "remove-element", notes: "The <text> child of <g> (index 1) is removed entirely." },
  { id: "set-element-name-applied", mutation: "set-element-name", notes: "<rect> is renamed to <ellipse> in place; attributes/children untouched." },
  { id: "set-attribute-applied", mutation: "set-attribute", notes: "A new fill=\"red\" attribute is appended to <rect>." },
  { id: "set-text-applied", mutation: "set-text", notes: "The text node inside <text> changes from \"Hello\" to \"World\"." },
  { id: "set-view-box-applied", mutation: "set-view-box", notes: "Root <svg>'s own viewBox changes from \"0 0 100 100\" to \"0 0 200 200\" (raw string, not decomposed — see this subset's own reader-oracle rationale)." },
  { id: "set-transform-applied", mutation: "set-transform", notes: "<g>'s own transform changes from \"translate(10,20)\" to \"translate(30,40) scale(2)\" (raw string, not decomposed)." },
];
const FIXTURE_DIRECTORY_NAMES: Readonly<Record<string, string>> = {
  "insert-element-applied": "➕️insert-element-applied",
  "remove-element-applied": "➖️remove-element-applied",
  "set-attribute-applied": "🏷️set-attribute-applied",
  "set-declaration-applied": "📣️set-declaration-applied",
  "set-doctype-applied": "📜️set-doctype-applied",
  "set-element-name-applied": "🔤️set-element-name-applied",
  "set-text-applied": "✍️set-text-applied",
  "set-transform-applied": "🔄️set-transform-applied",
  "set-view-box-applied": "🖼️set-view-box-applied",
};
const BEFORE_FILE = "⬅️before.svg";
const AFTER_FILE = "➡️after.svg";
//#endregion 🧬️Contract

//#region 🏭️Generate
const FIXTURE_PATH_PREFIX = "../🧫️fixtures/";

function contentDigest(bytes: Buffer): string {
  return `sha256:${createHash("sha256").update(bytes).digest("hex")}`;
}

function platformId(): string {
  const os = process.platform === "win32" ? "win32" : process.platform === "darwin" ? "darwin" : "linux";
  const arch = process.arch === "arm64" ? "arm64" : "x64";
  return `${os}-${arch}`;
}

/** 🦀️ Shells out to the standalone `quick-xml-svg-codec` binary — the ONLY place this file
 *  touches it. */
function codecBuild(id: string, outDir: string, directory: string): void {
  const result = spawnSync("cargo", ["run", "--quiet", "--manifest-path", CODEC_MANIFEST, "--", "build", id, outDir, directory, BEFORE_FILE, AFTER_FILE], { encoding: "utf8" });
  if (result.status !== 0) {
    throw new Error(`quick-xml-svg-codec build ${id} failed (exit ${result.status}): ${result.stderr}`);
  }
}

function fileEntry(role: string, dir: string, filename: string, directory: string): { role: string; path: string; mediaType: string; sha256: string; bytes: number } {
  const abs = join(dir, filename);
  const bytes = readFileSync(abs);
  return { role, path: `${FIXTURE_PATH_PREFIX}${directory}/${filename}`, mediaType: "image/svg+xml", sha256: contentDigest(bytes), bytes: bytes.length };
}

function generateOne(recipe: Recipe, outDir: string): Record<string, unknown> {
  const directory = FIXTURE_DIRECTORY_NAMES[recipe.id]!;
  const dir = join(outDir, directory);
  codecBuild(recipe.id, outDir, directory);
  if (!existsSync(join(dir, BEFORE_FILE)) || !existsSync(join(dir, AFTER_FILE))) {
    throw new Error(`recipe ${recipe.id} did not produce both ${BEFORE_FILE} and ${AFTER_FILE}`);
  }
  const files = [fileEntry("expected-before-svg", dir, BEFORE_FILE, directory), fileEntry("expected-after-svg", dir, AFTER_FILE, directory)];

  return {
    schema: "semio.repository-test.fixture/v2",
    id: recipe.id,
    class: "third-party-generated",
    target: { artifact: "s.stdio.svg", standard: "1.1", subset: "base" },
    mutation: recipe.mutation,
    outcome: "applied",
    units: { length: "unitless", angle: "degree" },
    files,
    generator: {
      oracle: ORACLE_ID,
      packageVersion: ENGINE_VERSION,
      engineFamily: ENGINE_FAMILY,
      engineVersion: ENGINE_VERSION,
      command: `bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/📜️script.ts generate --only ${recipe.id}`,
      platform: platformId(),
    },
    provenance: { source: "generated", license: "MIT OR Apache-2.0 (quick-xml)", attribution: "Generated with quick-xml (MIT OR Apache-2.0) via the standalone quick-xml-svg-codec binary in this same directory", security: "scanned-clean", privacy: "no-personal-data" },
    comparisonProfile: "svg-1-1-quick-xml-reader-v1",
    reproducible: true,
    family: "structural",
    notes: recipe.notes,
  };
}

async function main(argv: readonly string[]): Promise<number> {
  const [command = "generate", ...rest] = argv;
  const value = (flag: string): string | null => {
    const index = rest.indexOf(flag);
    return index === -1 ? null : (rest[index + 1] ?? null);
  };
  const only = value("--only");
  const recipes = only === null ? RECIPES : RECIPES.filter((recipe) => recipe.id === only);
  if (recipes.length === 0) {
    console.error(`[svg generator] no recipe matches ${JSON.stringify(only)} — known: ${RECIPES.map((recipe) => recipe.id).join(", ")}`);
    return 1;
  }
  const outDir = process.env.SEMIO_FIXTURE_OUT ?? value("--out") ?? join(import.meta.dir, "..", "🧫️fixtures");
  mkdirSync(outDir, { recursive: true });

  if (command !== "generate" && command !== "manifests") {
    console.error(`[svg generator] unknown command ${JSON.stringify(command)} — expected generate | manifests`);
    return 1;
  }

  const manifests: Record<string, unknown>[] = [];
  let failed = 0;
  for (const recipe of recipes) {
    try {
      manifests.push(generateOne(recipe, outDir));
      console.error(`[svg generator] ${recipe.id} (${recipe.mutation}/applied)`);
    } catch (error) {
      // 🧭️A recipe the codec refuses is REPORTED, never dropped — see the avi/mesh/brep generators'
      // own identical rationale.
      failed += 1;
      console.error(`[svg generator] ${recipe.id} FAILED — ${(error as Error).message}`);
    }
  }

  if (command === "manifests") {
    process.stdout.write(`${JSON.stringify(manifests, null, 2)}\n`);
  }
  console.error(`[svg generator] ${manifests.length}/${recipes.length} bundle(s) generated into ${outDir}${failed > 0 ? `, ${failed} failed` : ""}`);
  return failed > 0 ? 1 : 0;
}

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
//#endregion 🏭️Generate
