#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.jpg@jfif-1.01/✳️document`'s reader oracle.
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
//   bun 📜️script.ts generate  [--only <fixture-id>]     # writes <outDir>/<id>/{before,after}.jpg
//   bun 📜️script.ts manifests [--only <fixture-id>]     # prints the fixtureManifests block (JSON)
//
// @see ../../../../📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🏭️generator/📜️script.ts — the sibling
//      generator this file's CLI/recipe shape is mirrored from.
// @see ./🦀️jpeg-jfif-codec/src/main.rs — the actual codec; `build <recipe-id> <out-dir>` and
//      `project <path>` are its only two commands. Its own module docstring records exactly which
//      of `image` 0.25.10's public API surface each recipe below relies on.
// @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/

//#endregion 🧲️Header

//#region 🔌️Adapters
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
//#endregion 🔌️Adapters

//#region 🧬️Contract
const CODEC_MANIFEST = join(import.meta.dir, "🦀️jpeg-jfif-codec", "Cargo.toml");
const ORACLE_ID = "image-jpeg-jfif-1-01-mutate-reader";
const ENGINE_FAMILY = "image-rs";
const ENGINE_VERSION = "0.25.10";

type Recipe = Readonly<{ id: string; mutation: string; witnessable: boolean; notes: string }>;

/** 🍳️ Mirrors `RECIPE_IDS`/`recipe()` in `🦀️jpeg-jfif-codec/src/main.rs` verbatim — one `-applied`
 *  entry per declared `JpgMutation` kind in `../🔣️oracle.json`'s `jpg-jfif-1-01-document`
 *  catalog. `witnessable` records whether THIS reader (checked against the real `image` 0.25.10 /
 *  zune-jpeg 0.5.15 source, not assumed) can see the recipe's own effect — it drives which
 *  `oracleRequirements` entry each kind gets in the oracle JSON, never the recipe bytes themselves. */
const RECIPES: readonly Recipe[] = [
  { id: "change-jfif-header-applied", mutation: "change-jfif-header", witnessable: false, notes: "APP0 density changes from the encoder's default to 300x300 DPI — a real byte difference `image`/zune-jpeg 0.5.15 write but have no decode-side getter for (verified: `ImageInfo.x_density`/`y_density` exist but their setters are never called anywhere in zune-jpeg's source)." },
  { id: "replace-quant-table-applied", mutation: "replace-quant-table", witnessable: false, notes: "Byte-identical before/after: this subset's own production encoder regenerates DQT fresh from re_encode_quality on every write, so no encoder in this repository can carry a replaced quant table into the bytes." },
  { id: "remove-quant-table-applied", mutation: "remove-quant-table", witnessable: false, notes: "Byte-identical before/after, same reason as replace-quant-table." },
  { id: "replace-huffman-table-applied", mutation: "replace-huffman-table", witnessable: false, notes: "Byte-identical before/after: production regenerates DHT fresh on every write." },
  { id: "remove-huffman-table-applied", mutation: "remove-huffman-table", witnessable: false, notes: "Byte-identical before/after, same reason as replace-huffman-table." },
  { id: "change-restart-interval-applied", mutation: "change-restart-interval", witnessable: false, notes: "Byte-identical before/after: production never emits a DRI/restart marker at all." },
  { id: "insert-other-segment-applied", mutation: "insert-other-segment", witnessable: true, notes: "An APP1 XMP segment is spliced in after APP0 — the one generic-segment payload shape `image`'s public `xmp_metadata()` accessor actually surfaces." },
  { id: "remove-other-segment-applied", mutation: "remove-other-segment", witnessable: true, notes: "The inverse of insert-other-segment: before carries the spliced APP1 XMP segment, after does not." },
  { id: "replace-pixels-applied", mutation: "replace-pixels", witnessable: true, notes: "The gradient/checkerboard base raster is replaced with a uniform mid-grey fill — the decoded raster digest changes." },
  { id: "change-re-encode-quality-applied", mutation: "change-re-encode-quality", witnessable: true, notes: "The same textured base raster re-encoded at quality 90 then quality 20 — quantization noise moves the decoded raster digest." },
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
function codecBuild(id: string, outDir: string): void {
  const result = spawnSync("cargo", ["run", "--quiet", "--offline", "--manifest-path", CODEC_MANIFEST, "--", "build", id, outDir], { encoding: "utf8" });
  if (result.status !== 0) {
    throw new Error(`jpeg-jfif-codec build ${id} failed (exit ${result.status}): ${result.stderr}`);
  }
}

function fileEntry(role: string, dir: string, filename: string, id: string): { role: string; path: string; mediaType: string; sha256: string; bytes: number } {
  const abs = join(dir, filename);
  const bytes = readFileSync(abs);
  return { role, path: `${FIXTURE_PATH_PREFIX}${id}/${filename}`, mediaType: "image/jpeg", sha256: contentDigest(bytes), bytes: bytes.length };
}

function generateOne(recipe: Recipe, outDir: string): Record<string, unknown> {
  const dir = join(outDir, recipe.id);
  codecBuild(recipe.id, outDir);
  if (!existsSync(join(dir, "after.jpg"))) throw new Error(`recipe ${recipe.id} is declared applied but the codec produced no after.jpg`);
  const files = [fileEntry("expected-before-jpg", dir, "before.jpg", recipe.id), fileEntry("expected-after-jpg", dir, "after.jpg", recipe.id)];

  return {
    schema: "semio.repository-test.fixture/v2",
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
      command: `bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🏭️generator/📜️script.ts generate --only ${recipe.id}`,
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
  const value = (flag: string): string | null => {
    const index = rest.indexOf(flag);
    return index === -1 ? null : (rest[index + 1] ?? null);
  };
  const only = value("--only");
  const recipes = only === null ? RECIPES : RECIPES.filter((recipe) => recipe.id === only);
  if (recipes.length === 0) {
    console.error(`[jpg generator] no recipe matches ${JSON.stringify(only)} — known: ${RECIPES.map((recipe) => recipe.id).join(", ")}`);
    return 1;
  }
  const outDir = process.env.SEMIO_FIXTURE_OUT ?? value("--out") ?? join(import.meta.dir, "..", "🧫️fixtures");
  mkdirSync(outDir, { recursive: true });

  if (command !== "generate" && command !== "manifests") {
    // 🔧️LIBJPEG MODE — the two kinds no LIBRARY reader reaches, closed by a third-party CLI.
    //
    // Protocol v2 lists `third-party-cli` as a qualifying oracle kind, and this is where it earns its
    // place: `image`-rs decodes to pixels, Pillow's DRI handler is literally `Skip` and its Huffman
    // accessors return empty and are deprecated for removal in Pillow 12, and `zune-jpeg` parses the DRI
    // marker but keeps `restart_interval` `pub(crate)`. `djpeg -v -v` prints every marker it walks,
    // Huffman table code-length counts and `Define Restart Interval N` included.
    //
    // The WRITER is `jpegtran`, from the same toolchain, and it is a LOSSLESS transcoder — `-restart`
    // and `-optimize` re-emit the identical image, so each pair differs in its marker structure and in
    // nothing else. The generator asserts the Start-of-Frame line is unchanged rather than trusting that.
    if (command === "libjpeg" || command === "libjpeg-manifests") {
      const KINDS: Record<string, string[]> = {
        "change-restart-interval": ["-restart", "2"],
        "replace-huffman-table": ["-optimize"],
      };
      // 🪓️`remove-huffman-table` cannot be produced by jpegtran: it is a LOSSLESS transcoder and never
      // drops a table. cjpeg's `-scans` wizard switch can, because a JPEG defines a Huffman table
      // BECAUSE a scan references it — so a scan script with one fewer AC scan yields one fewer table.
      //
      // That entailment is also the honest limit of this pair, and it is stated in the manifest rather
      // than papered over: no conforming writer can emit a file whose table list differs by exactly one
      // entry while the scan list is identical, so the pair witnesses the table-list effect and does not
      // isolate it from the scan that entailed the table.
      const SCAN_KIND = "remove-huffman-table";
      const SCANS = { before: "📄️four-tables.scan", after: "📄️three-tables.scan" };
      const dhtSet = (file: string): string[] => {
        const dump = spawnSync("djpeg", ["-verbose", "-outfile", "/dev/null", file], { encoding: "utf8" });
        return `${dump.stdout ?? ""}${dump.stderr ?? ""}`.split("\n").filter((line) => line.startsWith("Define Huffman Table")).map((line) => line.trim().split(/\s+/).pop()!);
      };
      const probes = join(import.meta.dir, "..", "🔬️probes", "📜️script.ts");
      if (command === "libjpeg") {
        const failures: string[] = [];
        for (const [kind, args] of Object.entries(KINDS)) {
          const dir = join(outDir, kind);
          mkdirSync(dir, { recursive: true });
          const seed = spawnSync("python3", ["-c", "import sys\nfrom PIL import Image\nim=Image.new('RGB',(32,32))\n[im.putpixel((x,y),((x*8)%256,(y*8)%256,((x+y)*4)%256)) for x in range(32) for y in range(32)]\nim.save(sys.argv[1], quality=90)", join(dir, "before.jpg")], { stdio: "inherit" });
          if (seed.status !== 0) { failures.push(`${kind}: seed writer failed`); continue; }
          const transcoded = spawnSync("jpegtran", [...args, "-outfile", join(dir, "after.jpg"), join(dir, "before.jpg")], { stdio: "inherit" });
          if (transcoded.status !== 0) { failures.push(`${kind}: jpegtran failed`); continue; }
          const cmp = spawnSync("bun", [probes, "jpg-libjpeg-compare", "--input", join(dir, "before.jpg"), "--input", join(dir, "after.jpg")], { encoding: "utf8" });
          if (cmp.status !== 0) { failures.push(`${kind}: reader refused the pair`); continue; }
          const m = JSON.parse(cmp.stdout).measurements;
          if (m.equal === true) { failures.push(`${kind}: not observable in the marker projection`); continue; }
          // 🔍️jpegtran is lossless; the frame must be untouched, or the pair proves less than it claims.
          const sof = (dump: string[]) => dump.find((line: string) => line.startsWith("Start Of Frame"));
          if (sof(m.expected.markerDump) !== sof(m.actual.markerDump)) failures.push(`${kind}: the pair also changed the frame`);
        }
        // 🪓️the scan-script pair, written and verified by the same toolchain.
        {
          const dir = join(outDir, SCAN_KIND);
          mkdirSync(dir, { recursive: true });
          const ppm = join(dir, "seed.ppm");
          const seed = spawnSync("python3", ["-c", "import sys\nfrom PIL import Image\nim=Image.new('RGB',(64,64))\n[im.putpixel((x,y),((x*4)%256,(y*4)%256,((x+y)*2)%256)) for x in range(64) for y in range(64)]\nim.save(sys.argv[1])", ppm], { stdio: "inherit" });
          if (seed.status !== 0) failures.push(`${SCAN_KIND}: seed writer failed`);
          else {
            for (const [half, script] of [["before.jpg", SCANS.before], ["after.jpg", SCANS.after]] as const) {
              const made = spawnSync("cjpeg", ["-scans", join(import.meta.dir, "🧾️scans", script), "-outfile", join(dir, half), ppm], { stdio: "inherit" });
              if (made.status !== 0) failures.push(`${SCAN_KIND}: cjpeg refused ${script}`);
            }
            rmSync(ppm, { force: true });
            const before = dhtSet(join(dir, "before.jpg"));
            const after = dhtSet(join(dir, "after.jpg"));
            const removed = before.filter((table) => !after.includes(table));
            // 🔍️the pair must remove EXACTLY ONE table, and nothing may appear that was not there before.
            if (removed.length !== 1) failures.push(`${SCAN_KIND}: expected exactly one table removed, saw [${removed.join(", ")}]`);
            if (after.some((table) => !before.includes(table))) failures.push(`${SCAN_KIND}: the after half defines a table the before half did not`);
            // 🔍️both halves must remain decodable, or the fixture is not a JPEG the oracle can read.
            for (const half of ["before.jpg", "after.jpg"]) {
              const decoded = spawnSync("djpeg", ["-outfile", "/dev/null", join(dir, half)], { encoding: "utf8" });
              if (decoded.status !== 0) failures.push(`${SCAN_KIND}: ${half} does not decode`);
            }
          }
        }
        for (const failure of failures) console.error(`[jpg generator] ${failure}`);
        return failures.length > 0 ? 1 : 0;
      }
      const entries = [];
      for (const kind of [...Object.keys(KINDS), SCAN_KIND]) {
        const files = [];
        for (const [role, name] of [["expected-before-jpg", "before.jpg"], ["expected-after-jpg", "after.jpg"]] as const) {
          const bytes = readFileSync(join(outDir, kind, name));
          files.push({ role, path: `${FIXTURE_PATH_PREFIX}${kind}/${name}`, mediaType: "image/jpeg", sha256: contentDigest(bytes), bytes: bytes.length });
        }
        entries.push({
          schema: "semio.repository-test.fixture/v2",
          id: `libjpeg-${kind}`,
          class: "third-party-generated",
          target: { artifact: "s.stdio.jpg", standard: "jfif-1.01", subset: "document" },
          mutation: kind,
          outcome: "applied",
          units: { length: "unitless", angle: "degree" },
          files,
          provenance: { source: "generated", license: "public-domain (synthetic, no third-party content embedded)" },
          generator: { oracle: "libjpeg-jpg-jfif-1-01-marker-cli", packageVersion: "3.2.0", engineFamily: "libjpeg-turbo", engineVersion: "3.2.0", command: "bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🏭️generator/📜️script.ts libjpeg", platform: process.platform },
          comparisonProfile: "semantic-jpg-libjpeg-marker-v1",
          reproducible: true,
          family: "mechanical",
          invariants: kind === SCAN_KIND ? { local: ["huffman-table-list-agrees-with-djpeg"], enclosing: ["both-halves-remain-decodable-by-djpeg"] } : undefined,
          notes: kind === SCAN_KIND
            ? `A Pillow-written seed encoded twice by cjpeg under ${SCANS.before} and ${SCANS.after}. The scan scripts differ by component 1's AC scan, so the table list differs by exactly one entry (AC chroma 0x11) — the generator asserts exactly one table is removed, that no new table appears, and that both halves still decode. HONEST LIMIT: the pair does NOT isolate the table from the scan. A JPEG defines a Huffman table BECAUSE a scan references it, so no conforming writer can emit a file whose table list differs by one entry while the scan list is identical; dropping the scan is the only way to drop the table. djpeg -verbose enumerates every DHT, which is what makes the table-list effect witnessable at all.`
            : `A Pillow-written seed transcoded by jpegtran ${(KINDS[kind] ?? []).join(" ")}. jpegtran is LOSSLESS, so the pair differs in its marker structure and not in the image — the generator asserts the Start-of-Frame line is unchanged. No library reader reaches these markers: image-rs decodes to pixels, Pillow's DRI handler is Skip and its Huffman accessors are empty and deprecated, and zune-jpeg keeps restart_interval pub(crate). djpeg -v -v prints them.`,
        });
      }
      process.stdout.write(`${JSON.stringify(entries, null, 2)}\n`);
      return 0;
    }

    // 🏷️MARKER MODE — the two kinds `image`-rs cannot see.
    //
    // `image` decodes a JPEG to PIXELS: the quantisation tables and the JFIF APP0 segment are consumed
    // and discarded on the way, which is why those kinds were `-uncarried` against it. Pillow keeps
    // both (`im.quantization`, `im.info['jfif_*']`) and can also WRITE the differences — a different
    // `quality` produces different quantisation tables, and `dpi=` rewrites the JFIF density and unit.
    //
    // Measured, and the measurement set the scope: `replace-quant-table` and `change-jfif-header` move
    // the reader's projection. `change-restart-interval` does not (Pillow does not read the DRI segment
    // back), and the Huffman accessors return empty and are deprecated for removal in Pillow 12. Those
    // four stay `-uncarried` rather than being claimed.
    if (command === "markers" || command === "markers-manifests") {
      const WRITER = String.raw`
import sys
from PIL import Image
out, kind = sys.argv[1], sys.argv[2]
im = Image.new('RGB', (32, 32))
for x in range(32):
    for y in range(32):
        im.putpixel((x, y), ((x * 8) % 256, (y * 8) % 256, ((x + y) * 4) % 256))
import os

# 📐️The Annex-K luminance table, passed explicitly so the table COUNT is what varies and nothing else.
STD = [16,11,10,16,24,40,51,61, 12,12,14,19,26,58,60,55, 14,13,16,24,40,57,69,56,
       14,17,22,29,51,87,80,62, 18,22,37,56,68,109,103,77, 24,35,55,64,81,104,113,92,
       49,64,78,87,103,121,120,101, 72,92,95,98,112,100,103,99]

d = os.path.join(out, kind); os.makedirs(d, exist_ok=True)
if kind == 'replace-quant-table':
    im.save(os.path.join(d, 'before.jpg'), quality=90)
    im.save(os.path.join(d, 'after.jpg'), quality=40)
elif kind == 'change-jfif-header':
    im.save(os.path.join(d, 'before.jpg'), quality=90)
    im.save(os.path.join(d, 'after.jpg'), quality=90, dpi=(300, 300))
elif kind == 'remove-quant-table':
    # 🔢️Two explicit tables -> one shared by every component. Same mode, same size, same pixels
    # source: only the table COUNT moves, which is what this kind names.
    im.save(os.path.join(d, 'before.jpg'), qtables=[STD, STD])
    im.save(os.path.join(d, 'after.jpg'), qtables=[STD])
else:
    raise SystemExit('unknown kind ' + kind)
print(kind + ': written')
`;
      const KINDS = ["replace-quant-table", "change-jfif-header", "remove-quant-table"];
      const probes = join(import.meta.dir, "..", "🔬️probes", "📜️script.ts");
      const root = outDir;
      if (command === "markers") {
        const failures: string[] = [];
        for (const kind of KINDS) {
          const written = spawnSync("python3", ["-c", WRITER, root, kind], { stdio: "inherit" });
          if (written.status !== 0) { failures.push(`${kind}: writer failed`); continue; }
          const cmp = spawnSync("bun", [probes, "jpg-marker-compare", "--input", join(root, kind, "before.jpg"), "--input", join(root, kind, "after.jpg")], { encoding: "utf8" });
          if (cmp.status !== 0) { failures.push(`${kind}: reader refused the pair`); continue; }
          if (JSON.parse(cmp.stdout).measurements.equal === true) failures.push(`${kind}: not observable in the marker projection`);
        }
        for (const failure of failures) console.error(`[jpg generator] ${failure}`);
        return failures.length > 0 ? 1 : 0;
      }
      const entries = [];
      for (const kind of KINDS) {
        const files = [];
        for (const [role, name] of [["expected-before-jpg", "before.jpg"], ["expected-after-jpg", "after.jpg"]] as const) {
          const path = join(outDir, kind, name);
          const bytes = readFileSync(join(outDir, kind, name));
          files.push({ role, path: `${FIXTURE_PATH_PREFIX}${kind}/${name}`, mediaType: "image/jpeg", sha256: contentDigest(bytes), bytes: bytes.length });
        }
        entries.push({
          schema: "semio.repository-test.fixture/v2",
          id: `marker-${kind}`,
          class: "third-party-generated",
          target: { artifact: "s.stdio.jpg", standard: "jfif-1.01", subset: "document" },
          mutation: kind,
          outcome: "applied",
          units: { length: "unitless", angle: "degree" },
          files,
          provenance: { source: "generated", license: "public-domain (synthetic, no third-party content embedded)" },
          generator: { oracle: "pillow-jpg-jfif-1-01-marker-reader", packageVersion: "11.3.0", engineFamily: "pillow", engineVersion: "11.3.0", command: "bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🏭️generator/📜️script.ts markers", platform: process.platform },
          comparisonProfile: "semantic-jpg-marker-v1",
          reproducible: true,
          family: "mechanical",
          notes: `A Pillow-written JPEG pair differing only in the ${kind} marker facts — a different quality for the quantisation tables, a different dpi for the JFIF density and unit. image-rs, this subset's other reader, decodes to pixels and discards both. Observability was checked through the Pillow marker reader before the pair was written.`,
        });
      }
      process.stdout.write(`${JSON.stringify(entries, null, 2)}\n`);
      return 0;
    }
    console.error(`[jpg generator] unknown command ${JSON.stringify(command)} — expected generate | manifests | markers | markers-manifests | libjpeg | libjpeg-manifests`);
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
