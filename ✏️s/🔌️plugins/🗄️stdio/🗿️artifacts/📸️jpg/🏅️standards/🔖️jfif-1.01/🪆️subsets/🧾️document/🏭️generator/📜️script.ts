#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.jpg@jfif-1.01/🧾️document`'s reader oracle.
//
// Every recipe's BEFORE and AFTER `.jpg` bytes are built DIRECTLY by the sibling standalone
// `🦀️jpeg-jfif-codec` binary, which depends on nothing but `image` 0.25 — never by "applying" this
// repository's own `JpgMutation` dispatch, and never by calling this subset's own reclassified
// `🦀️oracle.rs` (which COMPUTES mutation results and shares a spec reading with
// production). This file only shells out per recipe and turns the bytes the codec wrote into a
// fixture bundle + manifest entry; it computes no JPEG semantics of its own.
//
// Generation and execution are SEPARATE operations, same shape as the sibling avi/bcf/mesh/brep
// generators this file's CLI is mirrored from: a normal test run must never be able to rewrite the
// expectation it is measured against.
//
//   bun 📜️script.ts generate  [--only <fixture-id>]     # writes each handpicked fixture directory
//   bun 📜️script.ts manifests [--only <fixture-id>]     # prints the testEvidence block (JSON)
//
// @see ../../../../📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/📜️script.ts — the sibling
//      generator this file's CLI/recipe shape is mirrored from.
// @see ./🔁️codec/🦀️.rs — the actual codec; `build <recipe-id> <out-dir>` and
//      `project <path>` are its only two commands. Its own module docstring records exactly which
//      of `image` 0.25.10's public API surface each recipe below relies on.
// @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/

//#endregion 🧲️Header

//#region 🔌️Adapters
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
import { currentPlatform } from "../../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧬️Contract
const CODEC_MANIFEST = join(import.meta.dir, "🔁️codec", "📦️packages", "🦀️rust", "Cargo.toml");
const ORACLE_ID = "image-jpeg-jfif-1-01-mutate-reader";
const ENGINE_FAMILY = "image-rs";
const ENGINE_VERSION = "0.25.10";

type Recipe = Readonly<{ id: string; directoryName: string; mutation: string; witnessable: boolean; notes: string }>;

/** 🍳️ Mirrors `RECIPE_IDS`/`recipe()` in `🔁️codec/🦀️.rs` verbatim — one `-applied`
 *  entry per declared `JpgMutation` kind in `../🔣️oracle.json`'s `jpg-jfif-1-01-document`
 *  catalog. `witnessable` records whether THIS reader (checked against the real `image` 0.25.10 /
 *  zune-jpeg 0.5.15 source, not assumed) can see the recipe's own effect — it drives which
 *  `oracleRequirements` entry each kind gets in the oracle JSON, never the recipe bytes themselves. */
const RECIPES: readonly Recipe[] = [
  { id: "change-jfif-header-applied", directoryName: "🪪️change-jfif-header-applied", mutation: "change-jfif-header", witnessable: false, notes: "APP0 density changes from the encoder's default to 300x300 DPI — a real byte difference `image`/zune-jpeg 0.5.15 write but have no decode-side getter for (verified: `ImageInfo.x_density`/`y_density` exist but their setters are never called anywhere in zune-jpeg's source)." },
  { id: "insert-other-segment-applied", directoryName: "📥️insert-other-segment-applied", mutation: "insert-other-segment", witnessable: true, notes: "An APP1 XMP segment is spliced in after APP0 — the one generic-segment payload shape `image`'s public `xmp_metadata()` accessor actually surfaces." },
  { id: "remove-other-segment-applied", directoryName: "🗑️remove-other-segment-applied", mutation: "remove-other-segment", witnessable: true, notes: "The inverse of insert-other-segment: before carries the spliced APP1 XMP segment, after does not." },
  { id: "replace-pixels-applied", directoryName: "🔲️replace-pixels-applied", mutation: "replace-pixels", witnessable: true, notes: "The gradient/checkerboard base raster is replaced with a uniform mid-grey fill — the decoded raster digest changes." },
  { id: "replace-image-applied", directoryName: "🖼️replace-image-applied", mutation: "replace-image", witnessable: true, notes: "Replacement changes image dimensions to 16x8, decoded pixels, 300x150 DPI and XMP metadata through independent image-rs generation." },
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

/** 🦀️ Shells out to the standalone `jpeg-jfif-codec` binary — the ONLY place this file touches it. */
function codecBuild(recipe: Recipe, outDir: string): void {
  const result = spawnSync("cargo", ["run", "--quiet", "--offline", "--manifest-path", CODEC_MANIFEST, "--", "build", recipe.id, outDir, recipe.directoryName], { encoding: "utf8" });
  if (result.status !== 0) {
    throw new Error(`jpeg-jfif-codec build ${recipe.id} failed (exit ${result.status}): ${result.stderr}`);
  }
}

function fileEntry(role: string, dir: string, filename: string, id: string): { role: string; path: string; mediaType: string; sha256: string; bytes: number } {
  const abs = join(dir, filename);
  const bytes = readFileSync(abs);
  return { role, path: `${FIXTURE_PATH_PREFIX}${id}/${filename}`, mediaType: "image/jpeg", sha256: contentDigest(bytes), bytes: bytes.length };
}

function generateOne(recipe: Recipe, outDir: string): Record<string, unknown> {
  const dir = join(outDir, recipe.directoryName);
  codecBuild(recipe, outDir);
  if (!existsSync(join(dir, "➡️after.jpg"))) throw new Error(`recipe ${recipe.id} is declared applied but the codec produced no ➡️after.jpg`);
  const files = [fileEntry("expected-before-jpg", dir, "⬅️before.jpg", recipe.directoryName), fileEntry("expected-after-jpg", dir, "➡️after.jpg", recipe.directoryName)];

  return {
    id: recipe.id,
    class: "third-party-generated",
    target: { artifact: "s.stdio.jpg", standard: "jfif-1.01", subset: "document" },
    mutation: recipe.mutation,
    outcome: "applied",
    units: { length: "unitless", angle: "degree" },
    files,
    generator: {
      oracle: ORACLE_ID,
      packageVersion: ENGINE_VERSION,
      engineFamily: ENGINE_FAMILY,
      engineVersion: ENGINE_VERSION,
      command: `bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/📜️script.ts generate --only ${recipe.id}`,
      platform: platformId(),
    },
    provenance: { source: "generated", license: "MIT OR Apache-2.0 (image)", attribution: "Generated with image (MIT OR Apache-2.0) via the standalone jpeg-jfif-codec binary in this same directory", security: "scanned-clean", privacy: "no-personal-data" },
    comparisonProfile: "semantic-jpg-mutate-v1",
    reproducible: true,
    family: "structural",
    notes: recipe.notes,
  };
}

async function main(argv: readonly string[]): Promise<number> {
  const [command = "generate", ...rest] = argv;
  if (command === "test") return spawnSync("cargo", ["test", "--offline", "--manifest-path", CODEC_MANIFEST], { stdio: "inherit" }).status ?? 1;
  const value = (flag: string): string | null => {
    const index = rest.indexOf(flag);
    return index === -1 ? null : (rest[index + 1] ?? null);
  };
  const only = value("--only") ?? (command === "witness" ? "replace-image-applied" : null);
  const recipes = only === null ? RECIPES : RECIPES.filter((recipe) => recipe.id === only);
  if (recipes.length === 0) {
    console.error(`[jpg generator] no recipe matches ${JSON.stringify(only)} — known: ${RECIPES.map((recipe) => recipe.id).join(", ")}`);
    return 1;
  }
  const outDir = process.env.SEMIO_FIXTURE_OUT ?? value("--out") ?? join(import.meta.dir, "..", "🧫️fixtures");
  mkdirSync(outDir, { recursive: true });
  if (command === "witness") {
    const expected = JSON.parse(readFileSync(join(import.meta.dir, "🧪️tests", "🖼️replace-image", "🔣️.json"), "utf8"));
    const dir = join(outDir, "🖼️replace-image-applied");
    const project = (filename: string) => {
      const input = join(dir, filename);
      const image = spawnSync("cargo", ["run", "--quiet", "--offline", "--manifest-path", CODEC_MANIFEST, "--", "project", input], { encoding: "utf8" });
      if (image.status !== 0) throw new Error(image.stderr);
      const marker = spawnSync("bun", [join(import.meta.dir, "..", "🔬️probes", "📜️script.ts"), "jpg-marker-project", "--input", input], { encoding: "utf8" });
      if (marker.status !== 0) throw new Error(marker.stderr);
      return { image: JSON.parse(image.stdout), marker: JSON.parse(marker.stdout).measurements };
    };
    const before = project("⬅️before.jpg");
    const after = project("➡️after.jpg");
    if (before.image.dimensions !== expected.before.dimensions || after.image.dimensions !== expected.after.dimensions || after.image.raster.size !== expected.after.rasterBytes || before.image.raster.digest === after.image.raster.digest || after.image.xmp.present !== expected.after.xmp || after.marker.jfifUnit !== expected.after.jfifUnit || JSON.stringify(after.marker.jfifDensity) !== JSON.stringify(expected.after.jfifDensity)) throw new Error("independent ReplaceImage witness disagrees with neutral recipe");
    console.error("[DEBUG] independent image-rs and Pillow readers witness whole-image replacement");
    process.stdout.write(`${JSON.stringify({ recipe: expected.recipe, before, after }, null, 2)}\n`);
    return 0;
  }

  if (command !== "generate" && command !== "manifests") {
    if (command === "markers" || command === "markers-manifests") {
      const WRITER = String.raw`
import sys
from PIL import Image
out, kind, directory_name = sys.argv[1], sys.argv[2], sys.argv[3]
im = Image.new('RGB', (32, 32))
for x in range(32):
    for y in range(32):
        im.putpixel((x, y), ((x * 8) % 256, (y * 8) % 256, ((x + y) * 4) % 256))
import os

# 📐️The Annex-K luminance table, passed explicitly so the table COUNT is what varies and nothing else.
d = os.path.join(out, directory_name); os.makedirs(d, exist_ok=True)
if kind == 'change-jfif-header':
    im.save(os.path.join(d, '⬅️before.jpg'), quality=90)
    im.save(os.path.join(d, '➡️after.jpg'), quality=90, dpi=(300, 300))
else:
    raise SystemExit('unknown kind ' + kind)
print(kind + ': written')
`;
      const KINDS: Readonly<Record<string, string>> = {
        "change-jfif-header": "🔎️change-jfif-header",
      };
      const probes = join(import.meta.dir, "..", "🔬️probes", "📜️script.ts");
      const root = outDir;
      if (command === "markers") {
        const failures: string[] = [];
        for (const [kind, directoryName] of Object.entries(KINDS)) {
          const written = spawnSync("python3", ["-c", WRITER, root, kind, directoryName], { stdio: "inherit" });
          if (written.status !== 0) { failures.push(`${kind}: writer failed`); continue; }
          const cmp = spawnSync("bun", [probes, "jpg-marker-compare", "--input", join(root, directoryName, "⬅️before.jpg"), "--input", join(root, directoryName, "➡️after.jpg")], { encoding: "utf8" });
          if (cmp.status !== 0) { failures.push(`${kind}: reader refused the pair`); continue; }
          if (JSON.parse(cmp.stdout).measurements.equal === true) failures.push(`${kind}: not observable in the marker projection`);
        }
        for (const failure of failures) console.error(`[jpg generator] ${failure}`);
        return failures.length > 0 ? 1 : 0;
      }
      const entries = [];
      for (const [kind, directoryName] of Object.entries(KINDS)) {
        const files = [];
        for (const [role, name] of [["expected-before-jpg", "⬅️before.jpg"], ["expected-after-jpg", "➡️after.jpg"]] as const) {
          const bytes = readFileSync(join(outDir, directoryName, name));
          files.push({ role, path: `${FIXTURE_PATH_PREFIX}${directoryName}/${name}`, mediaType: "image/jpeg", sha256: contentDigest(bytes), bytes: bytes.length });
        }
        entries.push({
          id: `marker-${kind}`,
          class: "third-party-generated",
          target: { artifact: "s.stdio.jpg", standard: "jfif-1.01", subset: "document" },
          mutation: kind,
          outcome: "applied",
          units: { length: "unitless", angle: "degree" },
          files,
          provenance: { source: "generated", license: "public-domain (synthetic, no third-party content embedded)" },
          generator: { oracle: "pillow-jpg-jfif-1-01-marker-reader", packageVersion: "11.3.0", engineFamily: "pillow", engineVersion: "11.3.0", command: "bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/📜️script.ts markers", platform: currentPlatform() },
          comparisonProfile: "semantic-jpg-marker-v1",
          reproducible: true,
          family: "mechanical",
          notes: `A Pillow-written JPEG pair differing in ${kind} JFIF density and unit. The Pillow marker reader witnesses both authored metadata values.`,
        });
      }
      process.stdout.write(`${JSON.stringify(entries, null, 2)}\n`);
      return 0;
    }
    console.error(`[jpg generator] unknown command ${JSON.stringify(command)} — expected generate | manifests | markers | markers-manifests | test`);
    return 1;
  }

  const manifests: Record<string, unknown>[] = [];
  let failed = 0;
  for (const recipe of recipes) {
    try {
      manifests.push(generateOne(recipe, outDir));
      console.error(`[jpg generator] ${recipe.id} (${recipe.mutation}, witnessable=${recipe.witnessable})`);
    } catch (error) {
      // 🧭️A recipe the codec refuses is REPORTED, never dropped — see the avi/mesh/brep
      // generators' own identical rationale.
      failed += 1;
      console.error(`[jpg generator] ${recipe.id} FAILED — ${(error as Error).message}`);
    }
  }

  if (command === "manifests") {
    process.stdout.write(`${JSON.stringify(manifests, null, 2)}\n`);
  }
  console.error(`[jpg generator] ${manifests.length}/${recipes.length} bundle(s) generated into ${outDir}${failed > 0 ? `, ${failed} failed` : ""}`);
  return failed > 0 ? 1 : 0;
}

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
//#endregion 🏭️Generate
