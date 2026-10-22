import { existsSync, mkdirSync, readdirSync, readFileSync } from "node:fs";
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
const SUBSET = "a";
const ORACLE_ID = "lopdf-pdf-1-4-a-mutate-reader";
const COMPARISON_PROFILE = "semantic-pdf-1-4-conformance-a-v1";
// 🧾️ Kept in step with `🔁️codec/🦀️.rs::KINDS` (itself the same list as
// `../🔣️oracle.json`'s `pdf-1-4-a` catalog) — `manifests` walks whichever recipe directories the
// engine actually wrote rather than trusting this constant, so a drift here fails loudly as a
// missing-directory error instead of silently under-registering a kind.
// 🚫️`insert-encryption-dictionary` and `remove-encryption-dictionary` are NOT here: `lopdf` 0.44's
// writer takes its encryption path whenever the trailer carries `/Encrypt` and then requires the
// encryption state a real decryption would have recorded, so a synthetic encryption dictionary can be
// neither written nor read back (`object ID 8 0 not found`). Both kinds are registered `-uncarried` in
// `../🔣️oracle.json` rather than routed around.
const KINDS: readonly string[] = ["set-page-text", "clear-page-text"];
const FIXTURE_DIRECTORY_BY_KIND: Readonly<Record<string, string>> = {
  "clear-page-text": "🧹️clear-page-text",
  "set-page-text": "📝️set-page-text",
};
const BEFORE_FILE = "⬅️before.pdf";
const AFTER_FILE = "➡️after.pdf";
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
  for (const fixtureDirectory of readdirSync(outRoot).sort()) {
    const basePath = join(outRoot, fixtureDirectory, BEFORE_FILE);
    const mutatedPath = join(outRoot, fixtureDirectory, AFTER_FILE);
    if (!existsSync(basePath) || !existsSync(mutatedPath)) continue;
    console.log(`${fixtureDirectory}: before=${await sha256(basePath)} after=${await sha256(mutatedPath)}`);
  }
}

async function manifests(): Promise<void> {
  const entries = [];
  for (const kind of KINDS) {
    const fixtureDirectory = FIXTURE_DIRECTORY_BY_KIND[kind];
    if (fixtureDirectory === undefined) throw new Error(`unregistered fixture identity: ${kind}`);
    const dir = join(FIXTURES_DIR, fixtureDirectory);
    const basePath = join(dir, BEFORE_FILE);
    const mutatedPath = join(dir, AFTER_FILE);
    if (!existsSync(basePath) || !existsSync(mutatedPath)) throw new Error(`${dir} does not exist — run "generate" first`);
    entries.push({
      id: `${kind}`,
      class: "third-party-generated",
      target: { artifact: "s.stdio.pdf", standard: "1.4", subset: SUBSET },
      mutation: kind,
      outcome: "applied",
      units: { length: "unitless", angle: "degree" },
      files: [
        { role: "base-pdf", path: `../🧫️fixtures/${fixtureDirectory}/${BEFORE_FILE}`, mediaType: "application/pdf", sha256: await sha256(basePath), bytes: readFileSync(basePath).length },
        { role: "mutated-pdf", path: `../🧫️fixtures/${fixtureDirectory}/${AFTER_FILE}`, mediaType: "application/pdf", sha256: await sha256(mutatedPath), bytes: readFileSync(mutatedPath).length },
      ],
      provenance: { source: "generated", license: "public-domain (synthetic, no third-party content embedded)" },
      generator: {
        oracle: ORACLE_ID,
        packageVersion: "0.44",
        engineFamily: "lopdf",
        engineVersion: "0.44",
        command: "repo ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/📜️script.ts generate",
        platform: currentPlatform(),
      },
      comparisonProfile: COMPARISON_PROFILE,
      reproducible: true,
      family: "mechanical",
      notes: `A minimal two-descriptor seed built with lopdf 0.44, with the ${kind} mutation applied THROUGH lopdf's own public COS API (🔁️codec/🦀️.rs::apply) — never through this repository's own mutation engine, which is what made the previous corpus inadmissible as evidence. base.pdf is the seed after 🦀️lopdf-engine's arrange put the mutation's precondition in place (identical to the untouched seed for every kind whose precondition is already satisfied); mutated.pdf is the result lopdf wrote. Observability (mutated projection != base projection, both read back through lopdf) is checked for every pair before it is written, and a pair that does not move is refused rather than committed.`,
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
