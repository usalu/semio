import { existsSync, mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
import { cargoTargetDirectory } from "@semio-tech/framework/process/execution";
import { findWorkspaceRoot } from "@semio-tech/framework/process/routing";
import { currentPlatform } from "@semio-tech/framework/test/adapter";
import { runToolSyncV1 } from "../../../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";

//#endregion 🔌️Adapters

//#region 🧬️Contract
const HERE = import.meta.dir;
const ENGINE_DIR = join(HERE, "🔁️codec", "📦️packages", "🦀️rust");
const ENGINE_BIN = join(cargoTargetDirectory(findWorkspaceRoot()), "release", "generate");
const FIXTURES_DIR = join(HERE, "..", "🧪️tests/🧫️fixtures");
const SUBSET = "x";
const ORACLE_ID = "lopdf-pdf-1-4-x-mutate-reader";
const COMPARISON_PROFILE = "semantic-pdf-1-4-conformance-x-v1";
// 🧾️ Kept in step with `🔁️codec/🦀️.rs::KINDS` (itself the same list as
// `../🔣️oracle.json`'s `pdf-1-4-x` catalog) — `manifests` walks whichever recipe directories the
// engine actually wrote rather than trusting this constant, so a drift here fails loudly as a
// missing-directory error instead of silently under-registering a kind.
// 🚫️`insert-encryption-dictionary` and `remove-encryption-dictionary` are NOT here: `lopdf` 0.44's
// writer takes its encryption path whenever the trailer carries `/Encrypt` and then requires the
// encryption state a real decryption would have recorded, so a synthetic encryption dictionary can be
// neither written nor read back (`object ID 8 0 not found`). Both kinds are registered `-uncarried` in
// `../🔣️oracle.json` rather than routed around.
const KINDS: readonly string[] = ["set-page-size", "collapse-page-size"];
const FIXTURE_DIRECTORY_BY_KIND: Readonly<Record<string, string>> = {
  "set-page-size": "📐️set-page-size",
  "collapse-page-size": "📉️collapse-page-size",
};
const BEFORE_FILENAME = "⬅️before.pdf";
const AFTER_FILENAME = "➡️after.pdf";
//#endregion 🧬️Contract

//#region 🔨️Build
/** ⚙️ Builds the reference-crate-backed generator binary if it is missing or stale. */
function ensureBuilt(): void {
  const result = runToolSyncV1("cargo", ["build", "--release", "--manifest-path", join(ENGINE_DIR, "Cargo.toml")], { stdio: "inherit" });
  if (result.status !== 0) throw new Error(`cargo build failed with status ${result.status}`);
}

async function sha256(path: string): Promise<string> {
  const bytes = readFileSync(path);
  const digest = await crypto.subtle.digest("SHA-256", bytes);
  return `sha256:${[...new Uint8Array(digest)].map((byte) => byte.toString(16).padStart(2, "0")).join("")}`;
}
//#endregion 🔨️Build

//#region 🚪️Commands
async function generate(outRoot: string): Promise<void> {
  ensureBuilt();
  mkdirSync(outRoot, { recursive: true });
  const result = spawnSync(ENGINE_BIN, [outRoot], { stdio: "inherit" });
  if (result.status !== 0) throw new Error(`generator binary failed with status ${result.status}`);
  for (const kind of KINDS) {
    const directory = FIXTURE_DIRECTORY_BY_KIND[kind]!;
    const beforePath = join(outRoot, directory, BEFORE_FILENAME);
    const afterPath = join(outRoot, directory, AFTER_FILENAME);
    if (!existsSync(beforePath) || !existsSync(afterPath)) continue;
    console.log(`${kind}: before=${await sha256(beforePath)} after=${await sha256(afterPath)}`);
  }
}

async function manifests(): Promise<void> {
  const entries = [];
  for (const kind of KINDS) {
    const directory = FIXTURE_DIRECTORY_BY_KIND[kind]!;
    const dir = join(FIXTURES_DIR, directory);
    const beforePath = join(dir, BEFORE_FILENAME);
    const afterPath = join(dir, AFTER_FILENAME);
    if (!existsSync(beforePath) || !existsSync(afterPath)) throw new Error(`${dir} does not contain the generated pair — run "generate" first`);
    entries.push({
      id: `${kind}`,
      class: "third-party-generated",
      target: { artifact: "s.stdio.pdf", standard: "1.4", subset: SUBSET },
      mutation: kind,
      outcome: "applied",
      units: { length: "unitless", angle: "degree" },
      files: [
        { role: "base-pdf", path: `../🧫️fixtures/${directory}/${BEFORE_FILENAME}`, mediaType: "application/pdf", sha256: await sha256(beforePath), bytes: readFileSync(beforePath).length },
        { role: "mutated-pdf", path: `../🧫️fixtures/${directory}/${AFTER_FILENAME}`, mediaType: "application/pdf", sha256: await sha256(afterPath), bytes: readFileSync(afterPath).length },
      ],
      provenance: { source: "generated", license: "public-domain (synthetic, no third-party content embedded)" },
      generator: {
        oracle: ORACLE_ID,
        packageVersion: "0.44",
        engineFamily: "lopdf",
        engineVersion: "0.44",
        command: "repo ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/📜️script.ts generate",
        platform: currentPlatform(),
      },
      comparisonPipeline: "pdf-1-4-x-lopdf-compare-v1",
      comparisonProfile: COMPARISON_PROFILE,
      reproducible: true,
      family: "mechanical",
      notes: `A minimal two-descriptor seed built with lopdf 0.44, with the ${kind} mutation applied through lopdf's own public COS API (🔁️codec/🦀️.rs::apply). ${BEFORE_FILENAME} is the arranged seed and ${AFTER_FILENAME} is the result lopdf wrote. Observability is checked before the pair is written.`,
    });
  }
  console.log(JSON.stringify(entries, null, 2));
}
//#endregion 🚪️Commands

//#region 🚪️Entry
async function main(argv: readonly string[]): Promise<number> {
  const [command = "", ...rest] = argv;
  const outFlagIndex = rest.indexOf("--out");
  const fixtureOutRoot = process.env.SEMIO_FIXTURE_OUT;
  const outRoot = outFlagIndex >= 0 ? rest[outFlagIndex + 1]! : (fixtureOutRoot ?? FIXTURES_DIR);
  if (command === "generate") {
    await generate(outRoot);
    return 0;
  }
  if (command === "manifests") {
    await manifests();
    return 0;
  }
  console.error(`usage: 📜️script.ts <generate|manifests> [--out <dir>]`);
  return 2;
}
/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */
export async function runScriptProgram(): Promise<void> {
  
  if (true) process.exitCode = await main(process.argv.slice(2));
}
