#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.xml@1.0/✳️base`.
//
// Every recipe's BEFORE and AFTER `.xml` bytes are built DIRECTLY by the sibling standalone
// `🦀️quick-xml-oracle-codec` binary, which depends on nothing but `quick-xml` 0.42 — never by
// "applying" this repository's own `XmlMutation` dispatch or this subset's own
// `🧪️oracle/🦀️component.rs` (which stays untouched, registered `cross-semio-implementation`). This
// file only shells out per recipe and turns the bytes the codec wrote into a fixture bundle +
// manifest entry; it computes no XML semantics of its own.
//
// Generation and execution are SEPARATE operations, same shape as the sibling AVI/BCF generators
// this file's CLI is mirrored from: a normal test run must never be able to rewrite the expectation
// it is measured against.
//
//   bun 📜️script.ts generate  [--only <fixture-id>]     # writes <outDir>/<id>/{before,after}.xml
//   bun 📜️script.ts manifests [--only <fixture-id>]     # prints the fixtureManifests block (JSON)
//
// @see ../../../../../📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🏭️generator/📜️script.ts — the sibling
//      generator this file's CLI/recipe shape is mirrored from.
// @see ./🦀️quick-xml-oracle-codec/src/main.rs — the actual codec; `build <recipe-id> <out-dir>` and
//      `project <path>` are its only two commands.
// @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/

//#endregion 🧲️Header

//#region 🔌️Adapters
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
//#endregion 🔌️Adapters

//#region 🧬️Contract
const CODEC_MANIFEST = join(import.meta.dir, "🦀️quick-xml-oracle-codec", "Cargo.toml");
const ORACLE_ID = "quick-xml-1-0-mutate-reader";
const ENGINE_FAMILY = "quick-xml";
const ENGINE_VERSION = "0.42.0";

type Recipe = Readonly<{ id: string; mutation: string; notes: string }>;

/** 🍳️ Mirrors `RECIPE_IDS`/`recipe()` in `🦀️quick-xml-oracle-codec/src/main.rs` verbatim — one entry
 *  per declared `XmlMutation` kind in `xml-1-0-base`'s own `kinds` list. Every kind in this catalog
 *  declares only the `applied` outcome (see `../🧪️oracle/🔣️.json`'s own `mutationManifests`), so
 *  there is no `-rejected-*` recipe in this corpus. */
const RECIPES: readonly Recipe[] = [
  { id: "set-declaration-applied", mutation: "set-declaration", notes: "Whole-value replace of the declaration only (version/encoding/standalone); doctype, prolog and root untouched." },
  { id: "set-doctype-applied", mutation: "set-doctype", notes: "Whole-value replace of the doctype only (SYSTEM -> PUBLIC external id, one entity added); declaration, prolog and root untouched." },
  { id: "insert-element-applied", mutation: "insert-element", notes: "A third `item` element is inserted into the root's children before the CDATA section; every other node keeps its position." },
  { id: "remove-element-applied", mutation: "remove-element", notes: "The root's CDATA child is removed; the two items, comment and PI keep their relative order." },
  { id: "set-attribute-applied", mutation: "set-attribute", notes: "The first `item`'s `qty` attribute changes value; its text, the second item, and every other attribute are untouched." },
  { id: "set-text-applied", mutation: "set-text", notes: "The first `item`'s text run changes to a DIFFERENT pre-escaped named+numeric entity pair (Euro sign), proving reassembly survives a mutation." },
];
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

/** 🦀️ Shells out to the standalone `quick-xml-oracle-codec` binary — the ONLY place this file
 *  touches it. */
function codecBuild(id: string, outDir: string): void {
  const result = spawnSync("cargo", ["run", "--quiet", "--offline", "--manifest-path", CODEC_MANIFEST, "--", "build", id, outDir], { encoding: "utf8" });
  if (result.status !== 0) {
    throw new Error(`quick-xml-oracle-codec build ${id} failed (exit ${result.status}): ${result.stderr}`);
  }
}

function fileEntry(role: string, dir: string, filename: string, id: string): { role: string; path: string; mediaType: string; sha256: string; bytes: number } {
  const abs = join(dir, filename);
  const bytes = readFileSync(abs);
  return { role, path: `${FIXTURE_PATH_PREFIX}${id}/${filename}`, mediaType: "application/xml", sha256: contentDigest(bytes), bytes: bytes.length };
}

function generateOne(recipe: Recipe, outDir: string): Record<string, unknown> {
  const dir = join(outDir, recipe.id);
  codecBuild(recipe.id, outDir);
  if (!existsSync(join(dir, "after.xml"))) throw new Error(`recipe ${recipe.id} is declared applied but the codec produced no after.xml`);
  const files = [fileEntry("expected-before-xml", dir, "before.xml", recipe.id), fileEntry("expected-after-xml", dir, "after.xml", recipe.id)];

  return {
    schema: "semio.repository-test.fixture/v2",
    id: recipe.id,
    class: "third-party-generated",
    target: { artifact: "s.stdio.xml", standard: "1.0", subset: "base" },
    mutation: recipe.mutation,
    outcome: "applied",
    units: { length: "unitless", angle: "degree" },
    files,
    generator: {
      oracle: ORACLE_ID,
      packageVersion: ENGINE_VERSION,
      engineFamily: ENGINE_FAMILY,
      engineVersion: ENGINE_VERSION,
      command: `bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🏭️generator/📜️script.ts generate --only ${recipe.id}`,
      platform: platformId(),
    },
    provenance: { source: "generated", license: "MIT OR Apache-2.0 (quick-xml)", attribution: "Generated with quick-xml (MIT OR Apache-2.0) via the standalone quick-xml-oracle-codec binary in this same directory", security: "scanned-clean", privacy: "no-personal-data" },
    comparisonProfile: "semantic-xml-v1",
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
    console.error(`[xml generator] no recipe matches ${JSON.stringify(only)} — known: ${RECIPES.map((recipe) => recipe.id).join(", ")}`);
    return 1;
  }
  const outDir = process.env.SEMIO_FIXTURE_OUT ?? value("--out") ?? join(import.meta.dir, "..", "🧫️fixtures");
  mkdirSync(outDir, { recursive: true });

  if (command !== "generate" && command !== "manifests") {
    console.error(`[xml generator] unknown command ${JSON.stringify(command)} — expected generate | manifests`);
    return 1;
  }

  const manifests: Record<string, unknown>[] = [];
  let failed = 0;
  for (const recipe of recipes) {
    try {
      manifests.push(generateOne(recipe, outDir));
      console.error(`[xml generator] ${recipe.id} (${recipe.mutation}/applied)`);
    } catch (error) {
      // 🧭️A recipe the codec refuses is REPORTED, never dropped — see the sibling AVI/BCF
      // generators' own identical rationale.
      failed += 1;
      console.error(`[xml generator] ${recipe.id} FAILED — ${(error as Error).message}`);
    }
  }

  if (command === "manifests") {
    process.stdout.write(`${JSON.stringify(manifests, null, 2)}\n`);
  }
  console.error(`[xml generator] ${manifests.length}/${recipes.length} bundle(s) generated into ${outDir}${failed > 0 ? `, ${failed} failed` : ""}`);
  return failed > 0 ? 1 : 0;
}

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
//#endregion 🏭️Generate
