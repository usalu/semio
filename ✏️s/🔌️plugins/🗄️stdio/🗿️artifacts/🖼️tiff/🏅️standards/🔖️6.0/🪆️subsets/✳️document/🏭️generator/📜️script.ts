#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.tiff@6.0/✳️document`.
//
// Every recipe's BEFORE and (where the library can actually produce it) AFTER `.tiff` bytes are
// built DIRECTLY by the sibling standalone `🦀️tiff-ifd-codec` binary, which depends on nothing but
// `tiff` 0.11 — never by "applying" this repository's own `TiffMutation` dispatch. This file only
// shells out per recipe and turns the bytes the codec wrote into a fixture bundle + manifest
// entry; it computes no TIFF semantics of its own.
//
// `change-byte-order-applied` is deliberately absent from `RECIPES`: the codec's own `build`
// subcommand refuses it (tiff 0.11.3's encoder can only write native-target-endian bytes, and
// every platform this oracle targets is little-endian — see the codec's own header comment and
// this ticket's own report for the full reasoning), so there is no library-authored "after" to
// generate. That mutation kind stays in the catalog with an `-uncarried` oracle requirement
// instead of a fabricated fixture.
//
//   bun 📜️script.ts generate  [--only <fixture-id>]     # writes <outDir>/<id>/{before,after}.tiff
//   bun 📜️script.ts manifests [--only <fixture-id>]     # prints the fixtureManifests block (JSON)
//
// @see ../../../../../../📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🏭️generator/📜️script.ts — the
//      sibling generator this file's CLI/recipe shape is mirrored from.
// @see ./🦀️tiff-ifd-codec/src/main.rs — the actual codec; `build <recipe-id> <out-dir>` and
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
const CODEC_MANIFEST = join(import.meta.dir, "🦀️tiff-ifd-codec", "Cargo.toml");
const ORACLE_ID = "image-tiff-6-0-mutate-reader";
const ENGINE_FAMILY = "tiff";
const ENGINE_VERSION = "0.11.3";

type Recipe = Readonly<{ id: string; mutation: string; notes: string }>;

/** 🍳️ Mirrors `RECIPE_IDS`/`recipe()` in `🦀️tiff-ifd-codec/src/main.rs` verbatim, minus
 *  `change-byte-order-applied` (the codec refuses to build it — see this file's own header). One
 *  entry per witnessable declared `TiffMutation` kind; every outcome is `applied` per this
 *  subset's own catalog (`../🧪️oracle/🔣️.json` — all 6 kinds declare `outcomes: ["applied"]` only). */
const RECIPES: readonly Recipe[] = [
  { id: "insert-ifd-applied", mutation: "insert-ifd", notes: "A second, smaller IFD is appended after the first — ifdCount 1 -> 2." },
  { id: "remove-ifd-applied", mutation: "remove-ifd", notes: "The second IFD of a two-IFD document is dropped — ifdCount 2 -> 1." },
  { id: "replace-tag-applied", mutation: "replace-tag", notes: "IFD 0's ImageDescription tag value changes; every other tag and the raster are untouched." },
  { id: "remove-tag-applied", mutation: "remove-tag", notes: "IFD 0's ImageDescription tag is omitted entirely on encode." },
  { id: "replace-pixels-applied", mutation: "replace-pixels", notes: "IFD 0 keeps its dimensions/tags; the raster bytes are wholly different." },
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
function codecBuild(id: string, outDir: string): void {
  const result = spawnSync("cargo", ["run", "--offline", "--quiet", "--manifest-path", CODEC_MANIFEST, "--", "build", id, outDir], { encoding: "utf8" });
  if (result.status !== 0) {
    throw new Error(`tiff-ifd-codec build ${id} failed (exit ${result.status}): ${result.stderr}`);
  }
}

function fileEntry(role: string, dir: string, filename: string, id: string): { role: string; path: string; mediaType: string; sha256: string; bytes: number } {
  const abs = join(dir, filename);
  const bytes = readFileSync(abs);
  return { role, path: `${FIXTURE_PATH_PREFIX}${id}/${filename}`, mediaType: "image/tiff", sha256: contentDigest(bytes), bytes: bytes.length };
}

function generateOne(recipe: Recipe, outDir: string): Record<string, unknown> {
  const dir = join(outDir, recipe.id);
  codecBuild(recipe.id, outDir);
  if (!existsSync(join(dir, "after.tiff"))) throw new Error(`recipe ${recipe.id} is declared applied but the codec produced no after.tiff`);
  const files = [fileEntry("expected-before-tiff", dir, "before.tiff", recipe.id), fileEntry("expected-after-tiff", dir, "after.tiff", recipe.id)];

  return {
    schema: "semio.repository-test.fixture/v2",
    id: recipe.id,
    class: "third-party-generated",
    target: { artifact: "s.stdio.tiff", standard: "6.0", subset: "document" },
    mutation: recipe.mutation,
    outcome: "applied",
    units: { length: "unitless", angle: "degree" },
    files,
    generator: {
      oracle: ORACLE_ID,
      packageVersion: ENGINE_VERSION,
      engineFamily: ENGINE_FAMILY,
      engineVersion: ENGINE_VERSION,
      command: `bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🏭️generator/📜️script.ts generate --only ${recipe.id}`,
      platform: platformId(),
    },
    provenance: { source: "generated", license: "MIT OR Apache-2.0 (tiff)", attribution: "Generated with tiff (MIT OR Apache-2.0) via the standalone tiff-ifd-codec binary in this same directory", security: "scanned-clean", privacy: "no-personal-data" },
    comparisonProfile: "semantic-raster-v1",
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
    console.error(`[tiff generator] no recipe matches ${JSON.stringify(only)} — known: ${RECIPES.map((recipe) => recipe.id).join(", ")}`);
    return 1;
  }
  const outDir = process.env.SEMIO_FIXTURE_OUT ?? value("--out") ?? join(import.meta.dir, "..", "🧫️fixtures");
  mkdirSync(outDir, { recursive: true });

  if (command !== "generate" && command !== "manifests") {
    // 🔡️BYTE-ORDER MODE — the one kind a pixel-level projection cannot see.
    //
    // tiff 0.11's own docs say why: "the image decoding methods will correct to the host byte order
    // automatically". The order survives only on the Decoder, via the public `byte_order()`.
    //
    // Its ENCODER cannot write a non-native order (`UsageError::ByteOrderMismatch`) and Pillow's
    // `im.save` always emits `II` regardless of any prefix hint — both measured. What does work is
    // Pillow's own IFD serialiser: `ImageFileDirectory_v2(ifh=...)` encodes every tag in the requested
    // endianness, so the library does the field layout and this script only lays out the 8-byte header
    // and concatenates the strip. Nothing here hand-encodes a TIFF field.
    if (command === "byte-order" || command === "byte-order-manifests") {
      const WRITER = String.raw`
import os, sys
from PIL import TiffImagePlugin

W = H = 8
PIXELS = bytes(((x * 8 + y * 3) % 256) for y in range(H) for x in range(W))

def build(endian):
    ifh = b'MM\x00\x2a\x00\x00\x00\x08' if endian == 'MM' else b'II\x2a\x00\x08\x00\x00\x00'
    ifd = TiffImagePlugin.ImageFileDirectory_v2(ifh=ifh)
    ifd[256] = W; ifd[257] = H; ifd[258] = 8; ifd[259] = 1; ifd[262] = 1
    ifd[277] = 1; ifd[278] = H; ifd[279] = len(PIXELS)
    # 📍️Left at 0: PIL's tobytes() relocates StripOffsets itself once it knows where the strip lands.
    ifd[273] = 0
    return ifh[:8] + ifd.tobytes(8) + PIXELS

out, kind = sys.argv[1], sys.argv[2]
d = os.path.join(out, kind); os.makedirs(d, exist_ok=True)
open(os.path.join(d, 'before.tif'), 'wb').write(build('II'))
open(os.path.join(d, 'after.tif'), 'wb').write(build('MM'))
print(kind + ': written')
`;
      const KINDS = ["change-byte-order"];
      const readerDir = join(import.meta.dir, "..", "🔬️probes", "🦀️byte-order-reader");
      const built = spawnSync("cargo", ["build", "--release", "--offline", "--manifest-path", join(readerDir, "Cargo.toml")], { stdio: "inherit" });
      if (built.status !== 0) throw new Error(`cargo build failed with status ${built.status}`);
      const readerBin = join(readerDir, "target", "release", "reader");
      if (command === "byte-order") {
        const failures: string[] = [];
        for (const kind of KINDS) {
          const written = spawnSync("python3", ["-c", WRITER, outDir, kind], { stdio: "inherit" });
          if (written.status !== 0) { failures.push(`${kind}: writer failed`); continue; }
          const cmp = spawnSync(readerBin, ["compare", join(outDir, kind, "before.tif"), join(outDir, kind, "after.tif")], { encoding: "utf8" });
          if (cmp.status !== 0) { failures.push(`${kind}: reader refused the pair`); continue; }
          const m = JSON.parse(cmp.stdout).measurements;
          if (m.equal === true) failures.push(`${kind}: not observable in the byte-order projection`);
          // 🔍️The pair must differ in the ORDER and in nothing else; a fixture that also moved the
          // image would pass a bare order comparison and prove less than it appears to.
          else if (m.expected.pixelChecksum !== m.actual.pixelChecksum) failures.push(`${kind}: the pair also changed the pixels (${m.expected.pixelChecksum} vs ${m.actual.pixelChecksum})`);
        }
        for (const failure of failures) console.error(`[tiff generator] ${failure}`);
        return failures.length > 0 ? 1 : 0;
      }
      const entries = [];
      for (const kind of KINDS) {
        const files = [];
        for (const [role, name] of [["expected-before-tiff", "before.tif"], ["expected-after-tiff", "after.tif"]] as const) {
          const bytes = readFileSync(join(outDir, kind, name));
          files.push({ role, path: `${FIXTURE_PATH_PREFIX}${kind}/${name}`, mediaType: "image/tiff", sha256: contentDigest(bytes), bytes: bytes.length });
        }
        entries.push({
          schema: "semio.repository-test.fixture/v2",
          id: `byte-order-${kind}`,
          class: "third-party-generated",
          target: { artifact: "s.stdio.tiff", standard: "6.0", subset: "document" },
          mutation: kind,
          outcome: "applied",
          units: { length: "unitless", angle: "degree" },
          files,
          provenance: { source: "generated", license: "public-domain (synthetic, no third-party content embedded)" },
          generator: { oracle: "tiff-6-0-byte-order-reader", packageVersion: "11.3.0", engineFamily: "pillow", engineVersion: "11.3.0", command: "bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🏭️generator/📜️script.ts byte-order", platform: process.platform },
          comparisonProfile: "semantic-tiff-byte-order-v1",
          reproducible: true,
          family: "mechanical",
          notes: `The same 8x8 image written twice through Pillow's own IFD serialiser, once little-endian and once big-endian. Both decode to an identical pixel checksum, so the pair differs in the declared byte order and in nothing else — checked at generation time. tiff 0.11 corrects to host order during image decoding, so only Decoder::byte_order() witnesses this.`,
        });
      }
      process.stdout.write(`${JSON.stringify(entries, null, 2)}\n`);
      return 0;
    }
    console.error(`[tiff generator] unknown command ${JSON.stringify(command)} — expected generate | manifests`);
    return 1;
  }

  const manifests: Record<string, unknown>[] = [];
  let failed = 0;
  for (const recipe of recipes) {
    try {
      manifests.push(generateOne(recipe, outDir));
      console.error(`[tiff generator] ${recipe.id} (${recipe.mutation}/applied)`);
    } catch (error) {
      // 🧭️A recipe the codec refuses is REPORTED, never dropped — see the sibling avi generator's
      // own identical rationale. (change-byte-order-applied is never even attempted here — see
      // this file's own header — but a genuine future failure still surfaces this way.)
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
