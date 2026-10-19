import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";

//#endregion 🔌️Adapters

//#region 🧬️Contract
const CODEC_MANIFEST = join(import.meta.dir, "🔁️codec", "📦️packages", "🦀️rust", "Cargo.toml");
const ORACLE_ID = "riff-avi-1-0-mutate";
const ENGINE_FAMILY = "riff";
const ENGINE_VERSION = "2.0.0";

type Outcome = "applied" | "rejected";
type Recipe = Readonly<{ id: string; directoryName: string; subset: "hdrl" | "idx1" | "movi"; mutation: string; outcome: Outcome; notes: string }>;

/** 🍳️ Mirrors `RECIPE_IDS`/`recipe()` in `🔁️codec/🦀️.rs` verbatim — one entry per
 *  declared `AviMutation` kind, `-applied` always, `-rejected-<reason>` wherever the real
 *  `validate_indexed`/`AviDiff::apply` genuinely refuses the input (see that file's own recipe
 *  comments for exactly which real error code each rejection corresponds to). */
const RECIPES: readonly Recipe[] = [
  { id: "no-mutation-applied", directoryName: "⏸️no-mutation-applied", subset: "hdrl", mutation: "no-mutation", outcome: "applied", notes: "Identity — before and after bytes are the same document." },
  { id: "set-main-header-applied", directoryName: "🧾️set-main-header-applied", subset: "hdrl", mutation: "set-main-header", outcome: "applied", notes: "Only avih fields change; streams/idx1/unknown chunks untouched." },
  { id: "set-idx1-present-applied", directoryName: "📇️set-idx1-present-applied", subset: "idx1", mutation: "set-idx1-present", outcome: "applied", notes: "idx1Present flips false; the idx1 chunk is omitted entirely on encode." },
  { id: "insert-stream-applied", directoryName: "📥️insert-stream-applied", subset: "hdrl", mutation: "insert-stream", outcome: "applied", notes: "A third stream is appended; mainHeader.streams is left stale at 2, matching real dispatch." },
  { id: "insert-stream-rejected-out-of-bounds", directoryName: "⛔️insert-stream-rejected-out-of-bounds", subset: "hdrl", mutation: "insert-stream", outcome: "rejected", notes: "Attempted insertion index exceeds the final collection length — mutation.apply.invalid-index." },
  { id: "remove-stream-applied", directoryName: "📤️remove-stream-applied", subset: "hdrl", mutation: "remove-stream", outcome: "applied", notes: "The audio stream (index 1) is removed; mainHeader.streams stays stale at 2." },
  { id: "remove-stream-rejected-missing", directoryName: "❓️remove-stream-rejected-missing", subset: "hdrl", mutation: "remove-stream", outcome: "rejected", notes: "Attempted removal index does not exist — mutation.apply.missing-target." },
  { id: "set-stream-header-applied", directoryName: "🎞️set-stream-header-applied", subset: "hdrl", mutation: "set-stream-header", outcome: "applied", notes: "Stream 0's strh is whole-value replaced (rate changes)." },
  { id: "set-stream-header-rejected-missing-stream", directoryName: "⚠️set-stream-header-rejected-missing-stream", subset: "hdrl", mutation: "set-stream-header", outcome: "rejected", notes: "Attempted stream_index does not exist — missing-target on the streams collection." },
  { id: "set-stream-format-applied", directoryName: "🎨️set-stream-format-applied", subset: "hdrl", mutation: "set-stream-format", outcome: "applied", notes: "Stream 1's strf is whole-value replaced (samplesPerSec changes)." },
  { id: "set-stream-format-rejected-missing-stream", directoryName: "🚫️set-stream-format-rejected-missing-stream", subset: "hdrl", mutation: "set-stream-format", outcome: "rejected", notes: "Attempted stream_index does not exist — missing-target on the streams collection." },
  { id: "insert-chunk-applied", directoryName: "🧩️insert-chunk-applied", subset: "movi", mutation: "insert-chunk", outcome: "applied", notes: "A 4th movi chunk is appended to stream 0; strh.length is left stale at 3." },
  { id: "insert-chunk-rejected-missing-stream", directoryName: "🚫️insert-chunk-rejected-missing-stream", subset: "movi", mutation: "insert-chunk", outcome: "rejected", notes: "Attempted stream_index does not exist — missing-target at the STREAMS level." },
  { id: "remove-chunk-applied", directoryName: "🗑️remove-chunk-applied", subset: "movi", mutation: "remove-chunk", outcome: "applied", notes: "The middle movi chunk of stream 0 is removed." },
  { id: "remove-chunk-rejected-missing-chunk", directoryName: "❓️remove-chunk-rejected-missing-chunk", subset: "movi", mutation: "remove-chunk", outcome: "rejected", notes: "Valid stream, attempted chunk index does not exist — missing-target at the CHUNKS level." },
  { id: "set-chunk-keyframe-applied", directoryName: "🔑️set-chunk-keyframe-applied", subset: "movi", mutation: "set-chunk-keyframe", outcome: "applied", notes: "The last movi chunk of stream 0 flips its keyframe flag; chunk data is untouched." },
  { id: "set-chunk-keyframe-rejected-missing-chunk", directoryName: "⚠️set-chunk-keyframe-rejected-missing-chunk", subset: "movi", mutation: "set-chunk-keyframe", outcome: "rejected", notes: "Valid stream, attempted chunk index does not exist — missing-target at the CHUNKS level." },
  { id: "add-unknown-chunk-applied", directoryName: "🧱️add-unknown-chunk-applied", subset: "movi", mutation: "add-unknown-chunk", outcome: "applied", notes: "A second top-level unknown chunk is appended after the existing JUNK chunk." },
  { id: "add-unknown-chunk-rejected-out-of-bounds", directoryName: "⛔️add-unknown-chunk-rejected-out-of-bounds", subset: "movi", mutation: "add-unknown-chunk", outcome: "rejected", notes: "Attempted insertion index exceeds the final collection length — invalid-index." },
  { id: "remove-unknown-chunk-applied", directoryName: "🧹️remove-unknown-chunk-applied", subset: "movi", mutation: "remove-unknown-chunk", outcome: "applied", notes: "The one top-level unknown (JUNK) chunk is removed, leaving none." },
  { id: "remove-unknown-chunk-rejected-missing", directoryName: "🔍️remove-unknown-chunk-rejected-missing", subset: "movi", mutation: "remove-unknown-chunk", outcome: "rejected", notes: "Attempted removal index does not exist — missing-target." },
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

function fixtureRoot(recipe: Recipe, override: string | null): string {
  if (override !== null) return override;
  if (recipe.subset === "hdrl") return join(import.meta.dir, "..", "🧪️tests/🧫️fixtures");
  if (recipe.subset === "idx1") return join(import.meta.dir, "..", "..", "📇️idx1", "🧪️tests/🧫️fixtures");
  return join(import.meta.dir, "..", "..", "🎞️movi", "🧪️tests/🧫️fixtures");
}

/** 🦀️ Shells out to the standalone `riff-avi-codec` binary — the ONLY place this file touches it. */
function codecBuild(id: string, fixtureDir: string): void {
  const result = spawnSync("cargo", ["run", "--quiet", "--manifest-path", CODEC_MANIFEST, "--", "build", id, fixtureDir], { encoding: "utf8" });
  if (result.status !== 0) {
    throw new Error(`riff-avi-codec build ${id} failed (exit ${result.status}): ${result.stderr}`);
  }
}

function fileEntry(role: string, dir: string, filename: string, directoryName: string): { role: string; path: string; mediaType: string; sha256: string; bytes: number } {
  const abs = join(dir, filename);
  const bytes = readFileSync(abs);
  return { role, path: `${FIXTURE_PATH_PREFIX}${directoryName}/${filename}`, mediaType: "video/x-msvideo", sha256: contentDigest(bytes), bytes: bytes.length };
}

function generateOne(recipe: Recipe, outDir: string): Record<string, unknown> {
  const dir = join(outDir, recipe.directoryName);
  codecBuild(recipe.id, dir);
  const files: ReturnType<typeof fileEntry>[] = [fileEntry("expected-before-avi", dir, "⬅️before.avi", recipe.directoryName)];
  if (recipe.outcome === "applied") {
    if (!existsSync(join(dir, "➡️after.avi"))) throw new Error(`recipe ${recipe.id} is declared applied but the codec produced no ➡️after.avi`);
    files.push(fileEntry("expected-after-avi", dir, "➡️after.avi", recipe.directoryName));
  } else if (existsSync(join(dir, "➡️after.avi"))) {
    throw new Error(`recipe ${recipe.id} is declared rejected but the codec produced an ➡️after.avi anyway`);
  }

  return {
    id: recipe.id,
    class: "third-party-generated",
    target: { artifact: "s.stdio.avi", standard: "1.0", subset: recipe.subset },
    mutation: recipe.mutation,
    outcome: recipe.outcome,
    units: { length: "unitless", angle: "degree" },
    files,
    generator: {
      oracle: ORACLE_ID,
      packageVersion: ENGINE_VERSION,
      engineFamily: ENGINE_FAMILY,
      engineVersion: ENGINE_VERSION,
      command: `bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/📜️script.ts generate --only ${recipe.id}`,
      platform: platformId(),
    },
    provenance: { source: "generated", license: "MIT (riff)", attribution: "Generated with riff (MIT) via the standalone riff-avi-codec binary in this same directory", security: "scanned-clean", privacy: "no-personal-data" },
    comparisonProfile: "semantic-avi-v1",
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
    console.error(`[avi generator] no recipe matches ${JSON.stringify(only)} — known: ${RECIPES.map((recipe) => recipe.id).join(", ")}`);
    return 1;
  }
  const outputOverride = process.env.SEMIO_FIXTURE_OUT ?? value("--out");

  if (command !== "generate" && command !== "manifests") {
    console.error(`[avi generator] unknown command ${JSON.stringify(command)} — expected generate | manifests`);
    return 1;
  }

  const manifests: Record<string, unknown>[] = [];
  let failed = 0;
  for (const recipe of recipes) {
    try {
      const outputRoot = fixtureRoot(recipe, outputOverride);
      mkdirSync(outputRoot, { recursive: true });
      manifests.push(generateOne(recipe, outputRoot));
      console.error(`[avi generator] ${recipe.id} (${recipe.mutation}/${recipe.outcome})`);
    } catch (error) {
      failed += 1;
      console.error(`[avi generator] ${recipe.id} FAILED — ${(error as Error).message}`);
    }
  }

  if (command === "manifests") {
    process.stdout.write(`${JSON.stringify(manifests, null, 2)}\n`);
  }
  console.error(`[avi generator] ${manifests.length}/${recipes.length} bundle(s) generated${failed > 0 ? `, ${failed} failed` : ""}`);
  return failed > 0 ? 1 : 0;
}
/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */
export async function runScriptProgram(): Promise<void> {
  
  if (true) process.exit(await main(process.argv.slice(2)));
}
