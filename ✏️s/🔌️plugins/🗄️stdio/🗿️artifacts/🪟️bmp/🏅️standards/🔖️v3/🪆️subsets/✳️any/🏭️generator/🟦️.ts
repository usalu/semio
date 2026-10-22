import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";

import { currentPlatform } from "@semio-tech/framework/test/adapter";
import { runToolSyncV1 } from "../../../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import { runtimeCommandNameV1 } from "../../../../../../../../../../🧰️framework/🔨️modules/⚙️runtime/🏷️identity/🟦️.ts";

//#endregion 🔌️Adapters

//#region 🧬️Contract
const CODEC_MANIFEST = join(import.meta.dir, "🔁️codec", "📦️packages", "🦀️rust", "Cargo.toml");
const ORACLE_ID = "image-bmp-3-mutate-reader";
const ENGINE_FAMILY = "image-rs";
const ENGINE_VERSION = "0.25.10";

type Recipe = Readonly<{ id: string; directoryName: string; mutation: string; notes: string }>;

/** 🍳️ Mirrors `RECIPE_IDS`/`recipe()` in `🔁️codec/🦀️.rs` verbatim — one entry per
 *  witnessable `BmpMutation` kind (see this subset's own report for why `change-header-fields` has
 *  no entry here). Every recipe's outcome is `applied` — this catalog declares no `rejected`
 *  mutations, per this ticket's own scoping. */
const RECIPES: readonly Recipe[] = [
  { id: "insert-palette-entry-applied", directoryName: "📥️insert-palette-entry-applied", mutation: "insert-palette-entry", notes: "A new colour is inserted into the palette table at index 2; the 16-entry index buffer is left byte-identical, matching this subset's own semantics that a palette edit changes the table and leaves the picture alone." },
  { id: "remove-palette-entry-applied", directoryName: "📤️remove-palette-entry-applied", mutation: "remove-palette-entry", notes: "The unreferenced spare palette entry (index 5, colour [0,0,0]) is removed; the index buffer is left byte-identical." },
  { id: "replace-palette-entry-applied", directoryName: "🎨️replace-palette-entry-applied", mutation: "replace-palette-entry", notes: "The unreferenced spare palette entry (index 5) is replaced with a new colour; the index buffer is left byte-identical." },
  { id: "replace-pixel-data-applied", directoryName: "🔲️replace-pixel-data-applied", mutation: "replace-pixel-data", notes: "A direct-colour 4x4 BMP's entire pixel array is replaced with a different solid RGB fill." },
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

/** 🦀️ Shells out to the standalone `image-bmp-codec` binary — the ONLY place this file touches it. */
function codecBuild(id: string, outDir: string): void {
  const result = runToolSyncV1("cargo", ["run", "--quiet", "--manifest-path", CODEC_MANIFEST, "--", "build", id, outDir], { encoding: "utf8" });
  if (result.status !== 0) {
    throw new Error(`image-bmp-codec build ${id} failed (exit ${result.status}): ${result.stderr}`);
  }
}

function fileEntry(role: string, dir: string, filename: string, directoryName: string): { role: string; path: string; mediaType: string; sha256: string; bytes: number } {
  const abs = join(dir, filename);
  const bytes = readFileSync(abs);
  return { role, path: `${FIXTURE_PATH_PREFIX}${directoryName}/${filename}`, mediaType: "image/bmp", sha256: contentDigest(bytes), bytes: bytes.length };
}

function generateOne(recipe: Recipe, outDir: string): Record<string, unknown> {
  const dir = join(outDir, recipe.directoryName);
  codecBuild(recipe.id, outDir);
  if (!existsSync(join(dir, "➡️after.bmp"))) throw new Error(`recipe ${recipe.id} is declared applied but the codec produced no ➡️after.bmp`);
  const files = [fileEntry("expected-before-bmp", dir, "⬅️before.bmp", recipe.directoryName), fileEntry("expected-after-bmp", dir, "➡️after.bmp", recipe.directoryName)];

  return {
    id: recipe.id,
    class: "third-party-generated",
    target: { artifact: "s.stdio.bmp", standard: "v3", subset: "any" },
    mutation: recipe.mutation,
    outcome: "applied",
    units: { length: "unitless", angle: "degree" },
    files,
    generator: {
      oracle: ORACLE_ID,
      packageVersion: ENGINE_VERSION,
      engineFamily: ENGINE_FAMILY,
      engineVersion: ENGINE_VERSION,
      command: `repo ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/📜️script.ts generate --only ${recipe.id}`,
      platform: platformId(),
    },
    provenance: { source: "generated", license: "MIT OR Apache-2.0 (image)", attribution: "Generated with image (MIT OR Apache-2.0) via the standalone image-bmp-codec binary in this same directory", security: "scanned-clean", privacy: "no-personal-data" },
    comparisonProfile: "semantic-bmp-reader-v1",
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
    console.error(`[bmp generator] no recipe matches ${JSON.stringify(only)} — known: ${RECIPES.map((recipe) => recipe.id).join(", ")}`);
    return 1;
  }
  const outDir = process.env.SEMIO_FIXTURE_OUT ?? value("--out") ?? join(import.meta.dir, "..", "🧪️tests/🧫️fixtures");
  mkdirSync(outDir, { recursive: true });

  if (command !== "generate" && command !== "manifests") {
    if (command === "header" || command === "header-manifests") {
      const WRITER = String.raw`
import os, sys
from PIL import Image

out, kind, directory_name = sys.argv[1], sys.argv[2], sys.argv[3]

def save(path, **kw):
    im = Image.new('P', (8, 8))
    im.putpalette([255, 0, 0, 0, 255, 0, 0, 0, 255] + [0] * (256 * 3 - 9))
    im.putdata([(x + y) % 3 for y in range(8) for x in range(8)])
    im.save(path, format='BMP', **kw)

d = os.path.join(out, directory_name); os.makedirs(d, exist_ok=True)
save(os.path.join(d, '⬅️before.bmp'))
save(os.path.join(d, '➡️after.bmp'), dpi=(300, 300))
print(kind + ': written')
`;
      const KINDS = [{ id: "change-header-fields", directoryName: "📐️change-header-fields" }];
      const probes = join(import.meta.dir, "..", "🔬️probes", "📜️script.ts");
      if (command === "header") {
        const failures: string[] = [];
        for (const kind of KINDS) {
          const written = runToolSyncV1("python", ["-c", WRITER, outDir, kind.id, kind.directoryName], { stdio: "inherit" });
          if (written.status !== 0) { failures.push(`${kind.id}: writer failed`); continue; }
          const cmp = runToolSyncV1(runtimeCommandNameV1(), [probes, "bmp-header-compare", "--input", join(outDir, kind.directoryName, "⬅️before.bmp"), "--input", join(outDir, kind.directoryName, "➡️after.bmp")], { encoding: "utf8" });
          if (cmp.status !== 0) { failures.push(`${kind.id}: reader refused the pair`); continue; }
          if (JSON.parse(cmp.stdout).measurements.equal === true) failures.push(`${kind.id}: not observable in the header projection`);
        }
        for (const failure of failures) console.error(`[bmp generator] ${failure}`);
        return failures.length > 0 ? 1 : 0;
      }
      const entries = [];
      for (const kind of KINDS) {
        const files = [];
        for (const [role, name] of [["expected-before-bmp", "⬅️before.bmp"], ["expected-after-bmp", "➡️after.bmp"]] as const) {
          const bytes = readFileSync(join(outDir, kind.directoryName, name));
          files.push({ role, path: `${FIXTURE_PATH_PREFIX}${kind.directoryName}/${name}`, mediaType: "image/bmp", sha256: contentDigest(bytes), bytes: bytes.length });
        }
        entries.push({
          id: `header-${kind.id}`,
          class: "third-party-generated",
          target: { artifact: "s.stdio.bmp", standard: "v3", subset: "any" },
          mutation: kind.id,
          outcome: "applied",
          units: { length: "unitless", angle: "degree" },
          files,
          provenance: { source: "generated", license: "public-domain (synthetic, no third-party content embedded)" },
          generator: { oracle: "pillow-bmp-v3-header-reader", packageVersion: "11.3.0", engineFamily: "pillow", engineVersion: "11.3.0", command: "repo ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/📜️script.ts header", platform: currentPlatform() },
          comparisonProfile: "semantic-bmp-header-v1",
          reproducible: true,
          family: "mechanical",
          notes: `A Pillow-written BMP pair differing only in the BITMAPINFOHEADER resolution fields (biXPelsPerMeter / biYPelsPerMeter, 3780 against 11811). image-rs, this subset's other reader, decodes to pixels and never surfaces them. Observability was checked through the Pillow header reader before the pair was written.`,
        });
      }
      process.stdout.write(`${JSON.stringify(entries, null, 2)}\n`);
      return 0;
    }
    console.error(`[bmp generator] unknown command ${JSON.stringify(command)} — expected generate | manifests`);
    return 1;
  }

  const manifests: Record<string, unknown>[] = [];
  let failed = 0;
  for (const recipe of recipes) {
    try {
      manifests.push(generateOne(recipe, outDir));
      console.error(`[bmp generator] ${recipe.id} (${recipe.mutation}/applied)`);
    } catch (error) {
      failed += 1;
      console.error(`[bmp generator] ${recipe.id} FAILED — ${(error as Error).message}`);
    }
  }

  if (command === "manifests") {
    process.stdout.write(`${JSON.stringify(manifests, null, 2)}\n`);
  }
  console.error(`[bmp generator] ${manifests.length}/${recipes.length} bundle(s) generated into ${outDir}${failed > 0 ? `, ${failed} failed` : ""}`);
  return failed > 0 ? 1 : 0;
}
/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */
export async function runScriptProgram(): Promise<void> {
  
  if (true) process.exit(await main(process.argv.slice(2)));
}
