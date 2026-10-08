#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.tiff@6.0/🧾️document`.
//
// Every recipe's BEFORE and (where the library can actually produce it) AFTER `.tiff` bytes are
// built DIRECTLY by the sibling standalone `🦀️tiff-ifd-codec` binary, which depends on nothing but
// `tiff` 0.11 — never by "applying" this repository's own `TiffMutation` dispatch. This file only
// shells out per recipe and turns the bytes the codec wrote into a fixture bundle + manifest
// entry; it computes no TIFF semantics of its own.
//
//   bun 📜️script.ts generate  [--only <fixture-id>]     # writes <outDir>/<id>/{before,after}.tiff
//   bun 📜️script.ts manifests [--only <fixture-id>]     # prints the testEvidence block (JSON)
//
// @see ../../../../../../📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🏭️generator/📜️script.ts — the
//      sibling generator this file's CLI/recipe shape is mirrored from.
// @see ./🔁️codec/🦀️.rs — the actual codec; `build <recipe-id> <out-dir> <directory-name>` and
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
const CODEC_MANIFEST = join(import.meta.dir, "🔁️codec", "📦️packages", "🦀️rust", "Cargo.toml");
const ORACLE_ID = "image-tiff-6-0-mutate-reader";
const ENGINE_FAMILY = "tiff";
const ENGINE_VERSION = "0.11.3";

type Recipe = Readonly<{ id: string; directoryName: string; mutation: string; notes: string }>;
/** 🍳️ Six authored mutation recipes, generated through the independent TIFF encoder. */
const RECIPES: readonly Recipe[] = [
 {id:"insert-ifd-applied",directoryName:"📥️insert-ifd-applied",mutation:"insert-ifd",notes:"A second smaller image page is appended."},
 {id:"remove-ifd-applied",directoryName:"📤️remove-ifd-applied",mutation:"remove-ifd",notes:"The second image page is removed."},
 {id:"replace-tag-applied",directoryName:"🏷️replace-tag-applied",mutation:"replace-tag",notes:"ImageDescription changes while raster samples stay identical."},
 {id:"remove-tag-applied",directoryName:"🗑️remove-tag-applied",mutation:"remove-tag",notes:"ImageDescription is omitted while raster samples stay identical."},
 {id:"paint-region-applied",directoryName:"🎨️paint-region-applied",mutation:"paint-region",notes:"Only pixel 1,1 changes to RGB 9,8,7."},
 {id:"replace-samples-applied",directoryName:"🧮️replace-samples-applied",mutation:"replace-samples",notes:"Only the first three owned sample words change to 24,96,192."},
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

/** 🦀️ Shells out to the standalone `tiff-ifd-codec` binary — the ONLY place this file touches it. */
function codecBuild(recipe: Recipe, outDir: string): void {
  const result = spawnSync("cargo", ["run", "--offline", "--quiet", "--manifest-path", CODEC_MANIFEST, "--", "build", recipe.id, outDir, recipe.directoryName], { encoding: "utf8" });
  if (result.status !== 0) {
    throw new Error(`tiff-ifd-codec build ${recipe.id} failed (exit ${result.status}): ${result.stderr}`);
  }
}

function fileEntry(role: string, dir: string, filename: string, id: string): { role: string; path: string; mediaType: string; sha256: string; bytes: number } {
  const abs = join(dir, filename);
  const bytes = readFileSync(abs);
  return { role, path: `${FIXTURE_PATH_PREFIX}${id}/${filename}`, mediaType: "image/tiff", sha256: contentDigest(bytes), bytes: bytes.length };
}

function generateOne(recipe: Recipe, outDir: string): Record<string, unknown> {
  const dir = join(outDir, recipe.directoryName);
  codecBuild(recipe, outDir);
  if (!existsSync(join(dir, "➡️after.tiff"))) throw new Error(`recipe ${recipe.id} is declared applied but the codec produced no after.tiff`);
  const files = [fileEntry("expected-before-tiff", dir, "⬅️before.tiff", recipe.directoryName), fileEntry("expected-after-tiff", dir, "➡️after.tiff", recipe.directoryName)];

  return {
    id: recipe.id,
    class: "third-party-generated",
    target: { artifact: "s.stdio.tiff", standard: "6.0", subset: "document" },
    ...(recipe.mutation ? { mutation: recipe.mutation, outcome: "applied" } : {}),
    units: { length: "unitless", angle: "degree" },
    files,
    generator: {
      oracle: ORACLE_ID,
      packageVersion: ENGINE_VERSION,
      engineFamily: ENGINE_FAMILY,
      engineVersion: ENGINE_VERSION,
      command: `bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/📜️script.ts generate --only ${recipe.id}`,
      platform: platformId(),
    },
    provenance: { source: "generated", license: "MIT OR Apache-2.0 (tiff)", attribution: "Generated with tiff (MIT OR Apache-2.0) via the standalone tiff-ifd-codec binary in this same directory", privacy: "no-personal-data" },
    comparisonProfile: "semantic-raster-v1",
    reproducible: true,
    family: "structural",
    notes: recipe.notes,
  };
}

async function main(argv: readonly string[]): Promise<number> {
  const [command = "generate", ...rest] = argv;
  if (command === "test-oracle" || command === "test-conformance-oracle") return spawnSync("cargo", ["test", "--offline", "--manifest-path", join(import.meta.dir,"../../../../../🔮️oracles/📦️packages/🦀️rust/Cargo.toml"), "--features", "oracles", "--lib", command === "test-oracle" ? "oracle_owned" : "baseline::component::tests", "--", "--nocapture"], { stdio: "inherit" }).status ?? 1;
  if (command === "test") return spawnSync("cargo", ["test", "--offline", "--manifest-path", CODEC_MANIFEST], { stdio: "inherit" }).status ?? 1;
  const value = (flag: string): string | null => {
    const index = rest.indexOf(flag);
    return index === -1 ? null : (rest[index + 1] ?? null);
  };
  const only = value("--only");
  const recipes = only === null ? RECIPES : RECIPES.filter((recipe) => recipe.id === only);
  if (recipes.length === 0) {
    console.error(`[tiff generator] no recipe matches ${JSON.stringify(only)} — known: ${RECIPES.map((recipe) => recipe.id).join(", ")}`);
    return 1;
  }
  const outDir = process.env.SEMIO_FIXTURE_OUT ?? value("--out") ?? join(import.meta.dir, "..", "🧫️fixtures");
  mkdirSync(outDir, { recursive: true });

  if (command !== "generate" && command !== "manifests") {
    console.error(`[tiff generator] unknown command ${JSON.stringify(command)} — expected generate | manifests | test | test-oracle | test-conformance-oracle`);
    return 1;
  }

  const manifests: Record<string, unknown>[] = [];
  let failed = 0;
  for (const recipe of recipes) {
    try {
      manifests.push(generateOne(recipe, outDir));
      console.error(`[tiff generator] ${recipe.id}${recipe.mutation ? ` (${recipe.mutation}/applied)` : ""}`);
    } catch (error) {
      failed += 1;
      console.error(`[tiff generator] ${recipe.id} FAILED — ${(error as Error).message}`);
    }
  }

  if (command === "manifests") {
    process.stdout.write(`${JSON.stringify(manifests, null, 2)}\n`);
  }
  console.error(`[tiff generator] ${manifests.length}/${recipes.length} bundle(s) generated into ${outDir}${failed > 0 ? `, ${failed} failed` : ""}`);
  return failed > 0 ? 1 : 0;
}

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
//#endregion 🏭️Generate
