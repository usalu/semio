import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";

//#endregion 🔌️Adapters

//#region 🧬️Contract
const CODEC_MANIFEST = join(import.meta.dir, "🔁️codec", "📦️packages", "🦀️rust", "Cargo.toml");
const ORACLE_ID = "quick-xml-1-0-mutate-reader";
const ENGINE_FAMILY = "quick-xml";
const ENGINE_VERSION = "0.42.0";

type Recipe = Readonly<{ id: string; mutation: string; notes: string }>;

/** 🍳️ Mirrors `RECIPE_IDS`/`recipe()` in `🔁️codec/🦀️.rs` verbatim — one entry
 *  per declared `XmlMutation` kind in `xml-1-0-base`'s own `kinds` list. Every kind in this catalog
 *  declares only the `applied` outcome (see `../🔮️oracles/🔣️.json`'s own `mutationManifests`), so
 *  there is no `-rejected-*` recipe in this corpus. */
const RECIPES: readonly Recipe[] = [
  { id: "set-declaration-applied", mutation: "set-declaration", notes: "Whole-value replace of the declaration only (version/encoding/standalone); doctype, prolog and root untouched." },
  { id: "set-doctype-applied", mutation: "set-doctype", notes: "Whole-value replace of the doctype only (SYSTEM -> PUBLIC external id, one entity added); declaration, prolog and root untouched." },
  { id: "insert-element-applied", mutation: "insert-element", notes: "A third `item` element is inserted into the root's children before the CDATA section; every other node keeps its position." },
  { id: "remove-element-applied", mutation: "remove-element", notes: "The root's CDATA child is removed; the two items, comment and PI keep their relative order." },
  { id: "set-attribute-applied", mutation: "set-attribute", notes: "The first `item`'s `qty` attribute changes value; its text, the second item, and every other attribute are untouched." },
  { id: "set-text-applied", mutation: "set-text", notes: "The first `item`'s text run changes to a DIFFERENT pre-escaped named+numeric entity pair (Euro sign), proving reassembly survives a mutation." },
];
const FIXTURE_DIRECTORY_NAMES: Readonly<Record<string, string>> = {
  "set-declaration-applied": "📣️set-declaration-applied",
  "set-doctype-applied": "📜️set-doctype-applied",
  "insert-element-applied": "➕️insert-element-applied",
  "remove-element-applied": "➖️remove-element-applied",
  "set-attribute-applied": "🪪️set-attribute-applied",
  "set-text-applied": "✍️set-text-applied",
};
const BEFORE_FILE = "⬅️before.xml";
const AFTER_FILE = "➡️after.xml";
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

/** 🦀️ Shells out to the standalone `quick-xml-oracle-codec` binary — the ONLY place this file
 *  touches it. */
function codecBuild(id: string, outDir: string, directory: string): void {
  const result = spawnSync("cargo", ["run", "--quiet", "--offline", "--manifest-path", CODEC_MANIFEST, "--", "build", id, outDir, directory, BEFORE_FILE, AFTER_FILE], { encoding: "utf8" });
  if (result.status !== 0) {
    throw new Error(`quick-xml-oracle-codec build ${id} failed (exit ${result.status}): ${result.stderr}`);
  }
}

function fileEntry(role: string, dir: string, filename: string, directory: string): { role: string; path: string; mediaType: string; sha256: string; bytes: number } {
  const abs = join(dir, filename);
  const bytes = readFileSync(abs);
  return { role, path: `${FIXTURE_PATH_PREFIX}${directory}/${filename}`, mediaType: "application/xml", sha256: contentDigest(bytes), bytes: bytes.length };
}

function generateOne(recipe: Recipe, outDir: string): Record<string, unknown> {
  const directory = FIXTURE_DIRECTORY_NAMES[recipe.id]!;
  const dir = join(outDir, directory);
  codecBuild(recipe.id, outDir, directory);
  if (!existsSync(join(dir, AFTER_FILE))) throw new Error(`recipe ${recipe.id} is declared applied but the codec produced no ${AFTER_FILE}`);
  const files = [fileEntry("expected-before-xml", dir, BEFORE_FILE, directory), fileEntry("expected-after-xml", dir, AFTER_FILE, directory)];

  return {
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
      command: `bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/📜️script.ts generate --only ${recipe.id}`,
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
  const outDir = process.env.SEMIO_FIXTURE_OUT ?? value("--out") ?? join(import.meta.dir, "..", "🧪️tests/🧫️fixtures");
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
/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */
export async function runScriptProgram(): Promise<void> {
  
  if (true) process.exitCode = await main(process.argv.slice(2));
}
