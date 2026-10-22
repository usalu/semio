import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";

import { currentPlatform } from "@semio-tech/framework/test/adapter";
import { runToolSyncV1 } from "../../../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import { runtimeCommandNameV1 } from "../../../../../../../../../../🧰️framework/🔨️modules/⚙️runtime/🏷️identity/🟦️.ts";

//#endregion 🔌️Adapters

//#region 🧬️Contract
const CODEC_MANIFEST = join(import.meta.dir, "🔁️codec", "📦️packages", "🦀️rust", "Cargo.toml");
const ORACLE_ID = "png-png-1-2-mutate-reader";
const ENGINE_FAMILY = "png";
const ENGINE_VERSION = "0.18.1";

type Recipe = Readonly<{ id: string; directory: string; mutation?: string; notes: string }>;

/** 🍳️ Mirrors `RECIPE_IDS`/`recipe()` in `🔁️codec/🦀️.rs` verbatim — one reference pair per chunk-level edit.
 *  Only a recipe naming a `mutation` witnesses a live `png-1-2-any` kind (`change-gamma`); the rest are reference
 *  documents for the native admission boundary and owned image vocabulary. */
const RECIPES: readonly Recipe[] = [
  { id: "change-header-applied", directory: "📐️change-header-applied", notes: "Whole-value IHDR replace: width/height change (4x2 -> 6x2), colour type/bit depth/interlace held fixed." },
  { id: "replace-palette-applied", directory: "🎨️replace-palette-applied", notes: "Whole-value PLTE replace over an Indexed base; index bytes (pixels) untouched." },
  { id: "change-transparency-applied", directory: "👁️change-transparency-applied", notes: "tRNS color-key add over an RGB (non-alpha) base." },
  { id: "change-gamma-applied", directory: "🌗️change-gamma-applied", mutation: "change-gamma", notes: "gAMA replace (1/2.2 -> 1.0, scaled x100000)." },
  { id: "change-chromaticities-applied", directory: "🌈️change-chromaticities-applied", notes: "cHRM replace (sRGB primaries -> an arbitrary other primary/white-point set)." },
  { id: "change-srgb-intent-applied", directory: "🖌️change-srgb-intent-applied", notes: "sRGB rendering intent replace (Perceptual -> RelativeColorimetric)." },
  { id: "change-physical-dims-applied", directory: "📏️change-physical-dims-applied", notes: "pHYs replace (2835x2835 px/m -> 1000x4000, unit Unspecified)." },
  { id: "change-timestamp-applied", directory: "🕰️change-timestamp-applied", notes: "tIME replace with a FIXED, hand-chosen 7-byte payload — never wall-clock. UNCARRIED: png::Info 0.18.1 has no tIME field." },
  { id: "change-background-applied", directory: "🖼️change-background-applied", notes: "bKGD replace, written through png::Writer::write_chunk's raw escape hatch (the encoder has no bKGD setter at all)." },
  { id: "insert-text-chunk-applied", directory: "📥️insert-text-chunk-applied", notes: "No tEXt chunk -> one (keyword \"Comment\")." },
  { id: "remove-text-chunk-applied", directory: "🗑️remove-text-chunk-applied", notes: "One tEXt chunk -> none." },
  { id: "replace-text-chunk-applied", directory: "✏️replace-text-chunk-applied", notes: "The one tEXt chunk's text is replaced, keyword held fixed." },
  { id: "replace-pixels-applied", directory: "🔲️replace-pixels-applied", notes: "Same header, disjoint pixel sample bytes (byte-inverted)." },
  { id: "insert-unknown-chunk-applied", directory: "📦️insert-unknown-chunk-applied", notes: "No unrecognised chunk -> one private ancillary chunk (fourcc prVt). UNCARRIED: the decoder skips unrecognised ancillary chunks entirely." },
  { id: "remove-unknown-chunk-applied", directory: "📤️remove-unknown-chunk-applied", notes: "One unrecognised chunk -> none. UNCARRIED for the same reason." },
];
//#endregion 🧬️Contract

//#region 🏭️Generate
const FIXTURE_PATH_PREFIX = "../🧪️tests/🧫️fixtures";

function contentDigest(bytes: Buffer): string {
  return `sha256:${createHash("sha256").update(bytes).digest("hex")}`;
}

function platformId(): string {
  const os = process.platform === "win32" ? "win32" : process.platform === "darwin" ? "darwin" : "linux";
  const arch = process.arch === "arm64" ? "arm64" : "x64";
  return `${os}-${arch}`;
}

/** 🦀️ Shells out to the standalone `png-codec` binary — the ONLY place this file touches it. */
function codecBuild(id: string, outDir: string): void {
  const result = runToolSyncV1("cargo", ["run", "--quiet", "--offline", "--manifest-path", CODEC_MANIFEST, "--", "build", id, outDir], { encoding: "utf8" });
  if (result.status !== 0) {
    throw new Error(`png-codec build ${id} failed (exit ${result.status}): ${result.stderr}`);
  }
}

function fileEntry(role: string, dir: string, filename: string, id: string): { role: string; path: string; mediaType: string; sha256: string; bytes: number } {
  const abs = join(dir, filename);
  const bytes = readFileSync(abs);
  return { role, path: `${FIXTURE_PATH_PREFIX}${id}/${filename}`, mediaType: "image/png", sha256: contentDigest(bytes), bytes: bytes.length };
}

function generateOne(recipe: Recipe, outDir: string): Record<string, unknown> {
  const dir = join(outDir, recipe.directory);
  codecBuild(recipe.id, dir);
  if (!existsSync(join(dir, "⬅️before.png")) || !existsSync(join(dir, "➡️after.png"))) {
    throw new Error(`recipe ${recipe.id} did not produce both ⬅️before.png and ➡️after.png`);
  }
  const files = [fileEntry("expected-before-png", dir, "⬅️before.png", recipe.directory), fileEntry("expected-after-png", dir, "➡️after.png", recipe.directory)];

  return {
    id: recipe.id,
    class: "third-party-generated",
    target: { artifact: "s.stdio.png", standard: "1.2", subset: "any" },
    ...(recipe.mutation ? { mutation: recipe.mutation, outcome: "applied" } : {}),
    units: { length: "unitless", angle: "degree" },
    files,
    generator: {
      oracle: ORACLE_ID,
      packageVersion: ENGINE_VERSION,
      engineFamily: ENGINE_FAMILY,
      engineVersion: ENGINE_VERSION,
      command: `repo ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/📜️script.ts generate --only ${recipe.id}`,
      platform: platformId(),
    },
    provenance: { source: "generated", license: "MIT OR Apache-2.0 (png)", attribution: "Generated with png (MIT OR Apache-2.0) via the standalone png-codec binary in this same directory", security: "scanned-clean", privacy: "no-personal-data" },
    comparisonProfile: "semantic-png-1-2-v1",
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
    console.error(`[png generator] no recipe matches ${JSON.stringify(only)} — known: ${RECIPES.map((recipe) => recipe.id).join(", ")}`);
    return 1;
  }
  const outDir = process.env.SEMIO_FIXTURE_OUT ?? value("--out") ?? join(import.meta.dir, "..", "🧪️tests/🧫️fixtures");
  mkdirSync(outDir, { recursive: true });

  if (command !== "generate" && command !== "manifests") {
    if (command === "chunks" || command === "chunks-manifests") {
      const WRITER = String.raw`
import os, sys
from PIL import Image, PngImagePlugin

TIME_BEFORE = bytes([7, 233, 1, 15, 10, 30, 0])
TIME_AFTER = bytes([7, 234, 2, 20, 11, 45, 30])
UNKNOWN = b'semio-private-payload'

def image():
    im = Image.new('RGB', (8, 8))
    for x in range(8):
        for y in range(8):
            im.putpixel((x, y), ((x * 32) % 256, (y * 32) % 256, ((x + y) * 16) % 256))
    return im

def save(path, **chunks):
    info = PngImagePlugin.PngInfo()
    for key, value in chunks.items():
        info.add(key.encode('ascii'), value)
    image().save(path, format='PNG', pnginfo=info)

d, kind = sys.argv[1], sys.argv[2]
os.makedirs(d, exist_ok=True)
if kind == 'change-timestamp':
    save(os.path.join(d, '⬅️before.png'), tIME=TIME_BEFORE)
    save(os.path.join(d, '➡️after.png'), tIME=TIME_AFTER)
elif kind == 'insert-unknown-chunk':
    save(os.path.join(d, '⬅️before.png'), tIME=TIME_BEFORE)
    save(os.path.join(d, '➡️after.png'), tIME=TIME_BEFORE, prVt=UNKNOWN)
elif kind == 'remove-unknown-chunk':
    save(os.path.join(d, '⬅️before.png'), tIME=TIME_BEFORE, prVt=UNKNOWN)
    save(os.path.join(d, '➡️after.png'), tIME=TIME_BEFORE)
else:
    raise SystemExit('unknown kind ' + kind)
print(kind + ': written')
`;
      const KINDS = [
        { kind: "change-timestamp", directory: "📅️change-timestamp" },
        { kind: "insert-unknown-chunk", directory: "➕️insert-unknown-chunk" },
        { kind: "remove-unknown-chunk", directory: "➖️remove-unknown-chunk" },
      ];
      const probes = join(import.meta.dir, "..", "🔬️probes", "📜️script.ts");
      if (command === "chunks") {
        const failures: string[] = [];
        for (const { kind, directory } of KINDS) {
          const written = runToolSyncV1("python", ["-c", WRITER, join(outDir, directory), kind], { stdio: "inherit" });
          if (written.status !== 0) { failures.push(`${kind}: writer failed`); continue; }
          const cmp = runToolSyncV1(runtimeCommandNameV1(), [probes, "png-chunk-compare", "--input", join(outDir, directory, "⬅️before.png"), "--input", join(outDir, directory, "➡️after.png")], { encoding: "utf8" });
          if (cmp.status !== 0) { failures.push(`${kind}: reader refused the pair`); continue; }
          if (JSON.parse(cmp.stdout).measurements.equal === true) failures.push(`${kind}: not observable in the chunk projection`);
        }
        for (const failure of failures) console.error(`[png generator] ${failure}`);
        return failures.length > 0 ? 1 : 0;
      }
      const entries = [];
      for (const { kind, directory } of KINDS) {
        const files = [];
        for (const [role, name] of [["expected-before-png", "⬅️before.png"], ["expected-after-png", "➡️after.png"]] as const) {
          const bytes = readFileSync(join(outDir, directory, name));
          files.push({ role, path: `${FIXTURE_PATH_PREFIX}${directory}/${name}`, mediaType: "image/png", sha256: contentDigest(bytes), bytes: bytes.length });
        }
        entries.push({
          id: `chunk-${kind}`,
          class: "third-party-generated",
          target: { artifact: "s.stdio.png", standard: "1.2", subset: "any" },
          units: { length: "unitless", angle: "degree" },
          files,
          provenance: { source: "generated", license: "public-domain (synthetic, no third-party content embedded)" },
          generator: { oracle: "pillow-png-1-2-chunk-reader", packageVersion: "11.3.0", engineFamily: "pillow", engineVersion: "11.3.0", command: "repo ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/📜️script.ts chunks", platform: currentPlatform() },
          comparisonProfile: "semantic-png-chunk-v1",
          reproducible: true,
          family: "mechanical",
          notes: `A Pillow-written PNG pair differing only in the chunk this kind touches. The png crate, this subset's other reader, has no tIME field and skips unrecognised ancillary chunks, so it cannot witness either. Observability was checked through the Pillow ChunkStream reader before the pair was written.`,
        });
      }
      process.stdout.write(`${JSON.stringify(entries, null, 2)}\n`);
      return 0;
    }
    console.error(`[png generator] unknown command ${JSON.stringify(command)} — expected generate | manifests | chunks | chunks-manifests`);
    return 1;
  }

  const manifests: Record<string, unknown>[] = [];
  let failed = 0;
  for (const recipe of recipes) {
    try {
      manifests.push(generateOne(recipe, outDir));
      console.error(`[png generator] ${recipe.id}`);
    } catch (error) {
      failed += 1;
      console.error(`[png generator] ${recipe.id} FAILED — ${(error as Error).message}`);
    }
  }

  if (command === "manifests") {
    process.stdout.write(`${JSON.stringify(manifests, null, 2)}\n`);
  }
  console.error(`[png generator] ${manifests.length}/${recipes.length} bundle(s) generated into ${outDir}${failed > 0 ? `, ${failed} failed` : ""}`);
  return failed > 0 ? 1 : 0;
}
/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */
export async function runScriptProgram(): Promise<void> {
  
  if (true) process.exit(await main(process.argv.slice(2)));
}
