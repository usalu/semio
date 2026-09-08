#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.pdf@1.7/🧱️base`.
//
// Drives TWO engines, both depending on `lopdf` 0.44 and nothing else, and neither of them this
// repository's own `encode_pdf`:
//
//   * `🦀️engine`       — writes `📊️report-strip.pdf`, the shared seed ASSET other subsets reuse.
//   * `⚖️lopdf-engine` — writes the sixteen before/after MUTATION pairs, applying each
//                        mutation through lopdf's own public COS API and reading the structural
//                        projection back through it, so nothing here predicts what the reader judges.
//
// This script only marshals: it builds and invokes the binaries and reports what they wrote.
//
// Generation and execution are SEPARATE operations, per the shared framework's own rule (a normal
// test run must never rewrite the expectation it is measured against): this command is the only one
// that writes into `../🧫️fixtures/`, and its output is reviewed and committed before any test reads it.
//
//   bun 📜️script.ts generate [--out <dir>]   # (re)builds both engines and writes every fixture
//   bun 📜️script.ts manifests                 # prints the fixtureManifests entries for all 17 committed fixtures
//
// @see ../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️gif-las-pdf17-findings.md

//#endregion 🧲️Header

//#region 🔌️Adapters
import { existsSync, mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
//#endregion 🔌️Adapters

//#region 🧬️Contract
const HERE = import.meta.dir;
const SUBSET = "base";
const ASSET_ENGINE = join(HERE, "🦀️engine");
const MUTATION_ENGINE = join(HERE, "⚖️lopdf-engine");
const FIXTURES_DIR = join(HERE, "..", "🧫️fixtures");
const ORACLE_ID = "lopdf-pdf-1-7-base-mutate-reader";
const COMPARISON_PROFILE = "semantic-pdf-structural-base-v1";
const ASSET_RECIPE = "report-strip";
const ASSET_DIRECTORY = "📊️report-strip";
const ASSET_FILE = "📊️report-strip.pdf";
/** 🧾️ Kept in step with `⚖️lopdf-engine/src/lib.rs::KINDS`. */
const KINDS: readonly string[] = ["insert-page", "remove-page", "move-page", "set-page-media-box", "set-page-crop-box", "set-page-rotation", "set-page-content", "append-page-content", "set-info", "insert-object", "remove-object", "set-object-value", "set-dict-entry", "remove-dict-entry", "set-trailer-entry", "remove-trailer-entry"];
const FIXTURE_DIRECTORY_BY_KIND: Readonly<Record<string, string>> = {
  "insert-page": "📥️insert-page",
  "remove-page": "🗑️remove-page",
  "move-page": "🔀️move-page",
  "set-page-media-box": "📐️set-page-media-box",
  "set-page-crop-box": "✂️set-page-crop-box",
  "set-page-rotation": "🔄️set-page-rotation",
  "set-page-content": "✏️set-page-content",
  "append-page-content": "➕️append-page-content",
  "set-info": "ℹ️set-info",
  "insert-object": "📦️insert-object",
  "remove-object": "🧹️remove-object",
  "set-object-value": "🔧️set-object-value",
  "set-dict-entry": "🔑️set-dict-entry",
  "remove-dict-entry": "🚫️remove-dict-entry",
  "set-trailer-entry": "🧳️set-trailer-entry",
  "remove-trailer-entry": "🧽️remove-trailer-entry",
};
const BEFORE_FILENAME = "⬅️before.pdf";
const AFTER_FILENAME = "➡️after.pdf";
//#endregion 🧬️Contract

//#region 🔨️Build
function build(dir: string): void {
  const result = spawnSync("cargo", ["build", "--release", "--offline", "--manifest-path", join(dir, "Cargo.toml")], { stdio: "inherit" });
  if (result.status !== 0) throw new Error(`cargo build failed for ${dir} with status ${result.status}`);
}

async function sha256(path: string): Promise<string> {
  const digest = await crypto.subtle.digest("SHA-256", readFileSync(path));
  return `sha256:${[...new Uint8Array(digest)].map((byte) => byte.toString(16).padStart(2, "0")).join("")}`;
}
//#endregion 🔨️Build

//#region 🚪️Commands
async function generate(outRoot: string): Promise<void> {
  build(ASSET_ENGINE);
  build(MUTATION_ENGINE);
  mkdirSync(join(outRoot, ASSET_DIRECTORY), { recursive: true });
  const asset = spawnSync(join(ASSET_ENGINE, "target", "release", "generate"), [join(outRoot, ASSET_DIRECTORY, ASSET_FILE)], { stdio: "inherit" });
  if (asset.status !== 0) throw new Error(`asset engine failed with status ${asset.status}`);
  const mutations = spawnSync(join(MUTATION_ENGINE, "target", "release", "generate"), [outRoot], { stdio: "inherit" });
  if (mutations.status !== 0) throw new Error(`mutation engine failed with status ${mutations.status}`);
}

async function manifests(): Promise<void> {
  const entries: unknown[] = [];
  entries.push({
    schema: "semio.repository-test.fixture/v2",
    id: ASSET_RECIPE,
    class: "third-party-generated",
    target: { artifact: "s.stdio.pdf", standard: "1.7", subset: SUBSET },
    units: { length: "unitless", angle: "degree" },
    files: [{ role: "seed-pdf", path: `../🧫️fixtures/${ASSET_DIRECTORY}/${ASSET_FILE}`, mediaType: "application/pdf", sha256: await sha256(join(FIXTURES_DIR, ASSET_DIRECTORY, ASSET_FILE)), bytes: readFileSync(join(FIXTURES_DIR, ASSET_DIRECTORY, ASSET_FILE)).length }],
    provenance: { source: "generated", license: "public-domain (synthetic, no third-party content embedded)" },
    generator: { oracle: ORACLE_ID, packageVersion: "0.44", engineFamily: "lopdf", engineVersion: "0.44", command: `bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/📜️script.ts generate`, platform: process.platform },
    // 📎️Carried for schema completeness only: this asset declares no mutation and no pipeline reads it,
    // but `fixtureProvenance` requires the field on every entry.
    comparisonProfile: COMPARISON_PROFILE,
    reproducible: true,
    family: "mechanical",
    notes: "The shared seed ASSET other pdf subsets reuse, written entirely by lopdf 0.44. Not a mutation fixture — it declares no mutation and is judged by nothing.",
  });
  for (const kind of KINDS) {
    const directory = FIXTURE_DIRECTORY_BY_KIND[kind]!;
    const dir = join(FIXTURES_DIR, directory);
    if (!existsSync(dir)) throw new Error(`missing fixture directory for ${kind} — run generate first`);
    const files = [];
    for (const [role, name] of [["base-pdf", BEFORE_FILENAME], ["mutated-pdf", AFTER_FILENAME]] as const) {
      const path = join(dir, name);
      files.push({ role, path: `../🧫️fixtures/${directory}/${name}`, mediaType: "application/pdf", sha256: await sha256(path), bytes: readFileSync(path).length });
    }
    entries.push({
      schema: "semio.repository-test.fixture/v2",
      id: `${SUBSET}-${kind}`,
      class: "third-party-generated",
      target: { artifact: "s.stdio.pdf", standard: "1.7", subset: SUBSET },
      mutation: kind,
      outcome: "applied",
      units: { length: "unitless", angle: "degree" },
      files,
      provenance: { source: "generated", license: "public-domain (synthetic, no third-party content embedded)" },
      generator: { oracle: ORACLE_ID, packageVersion: "0.44", engineFamily: "lopdf", engineVersion: "0.44", command: `bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/📜️script.ts generate`, platform: process.platform },
      comparisonPipeline: "pdf-1-7-base-lopdf-compare-v1",
      comparisonProfile: COMPARISON_PROFILE,
      reproducible: true,
      family: "mechanical",
      notes: `A two-page lopdf-built seed with the ${kind} mutation applied through lopdf's own public COS API (⚖️lopdf-engine/src/lib.rs::apply). ${BEFORE_FILENAME} is the arranged seed and ${AFTER_FILENAME} is the result lopdf wrote. Observability is checked before a pair is written.`,
    });
  }
  console.log(JSON.stringify(entries, null, 2));
}
//#endregion 🚪️Commands

//#region 🚀️Entry
const [command, ...rest] = process.argv.slice(2);
const outFlagIndex = rest.indexOf("--out");
const outRoot = outFlagIndex >= 0 ? rest[outFlagIndex + 1]! : (process.env.SEMIO_FIXTURE_OUT ?? FIXTURES_DIR);
if (command === "generate") await generate(outRoot);
else if (command === "manifests") await manifests();
else {
  console.error("usage: bun 📜️script.ts <generate [--out <dir>]|manifests>");
  process.exit(2);
}
//#endregion 🚀️Entry
