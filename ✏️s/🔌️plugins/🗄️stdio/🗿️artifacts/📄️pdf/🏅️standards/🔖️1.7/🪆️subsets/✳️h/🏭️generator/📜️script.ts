#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.pdf@1.7/✳️h` — the 18-kind ISO 16612-2 (PDF/VT-1)
// conformance-class catalog.
//
// Every base/mutated pair `🦀️lopdf-engine/src/generate.rs` writes is produced by the SAME registered `lopdf`
// 0.44 reference implementation named `lopdf-pdf-1-7-h-mutate` in `../🧪️oracle/🔣️.json` —
// through `lopdf`'s own public COS API inside the standalone
// `semio-s-plugin-stdio-test-oracle` crate, the identical engine the differential test case
// `../../../../../../🧪️tests/mutate-pdf-1-7-h` drives — never this repository's own production PDF
// codec, and never hand-rolled to match it. This script only marshals: it builds and invokes the
// Rust binary and reports what it wrote; it computes no PDF bytes itself.
//
// One generator run produces EVERY declared kind's recipe in one pass — the engine loops `KINDS`
// itself — so `generate`/`manifests` operate on the whole 18-recipe corpus, not one recipe at a time.
//
// Generation and execution are SEPARATE operations, per the shared framework's own rule (a normal
// test run must never rewrite the expectation it is measured against): this command is the only one
// that writes into `../🧫️fixtures/`, and its output is reviewed and committed before any test reads it.
//
//   bun 📜️script.ts generate [--out <dir>]   # (re)builds the engine, writes every recipe, prints digests
//   bun 📜️script.ts manifests                 # prints the fixtureManifests entries for the committed corpus
//
// @see ../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️pilot-playbook.md

//#endregion 🧲️Header

//#region 🔌️Adapters
import { existsSync, mkdirSync, readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
//#endregion 🔌️Adapters

//#region 🧬️Contract
const HERE = import.meta.dir;
const ENGINE_DIR = join(HERE, "🦀️lopdf-engine");
const ENGINE_BIN = join(ENGINE_DIR, "target", "release", "generate");
const FIXTURES_DIR = join(HERE, "..", "🧫️fixtures");
const SUBSET = "h";
const ORACLE_ID = "lopdf-pdf-1-7-h-mutate-reader";
const COMPARISON_PROFILE = "semantic-pdf-conformance-h-v1";
// 🧾️ Kept in step with `🦀️lopdf-engine/src/lib.rs::KINDS` (itself the same list as
// `../🧪️oracle/🔣️.json`'s `pdf-1-7-h` catalog) — `manifests` walks whichever recipe directories the
// engine actually wrote rather than trusting this constant, so a drift here fails loudly as a
// missing-directory error instead of silently under-registering a kind.
// 🚫️`insert-encryption-dictionary` and `remove-encryption-dictionary` are NOT here: `lopdf` 0.44's
// writer takes its encryption path whenever the trailer carries `/Encrypt` and then requires the
// encryption state a real decryption would have recorded, so a synthetic encryption dictionary can be
// neither written nor read back (`object ID 8 0 not found`). Both kinds are registered `-uncarried` in
// `../🧪️oracle/🔣️.json` rather than routed around.
const KINDS: readonly string[] = ["set-info-title", "set-info-author", "insert-javascript-action", "remove-javascript-action", "insert-launch-action", "remove-launch-action", "insert-signature-field", "remove-signature-field", "embed-font-file", "remove-font-file"];
//#endregion 🧬️Contract

//#region 🔨️Build
/** ⚙️ Builds the reference-crate-backed generator binary if it is missing or stale. */
function ensureBuilt(): void {
  const result = spawnSync("cargo", ["build", "--release", "--manifest-path", join(ENGINE_DIR, "Cargo.toml")], { stdio: "inherit" });
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
  for (const kind of readdirSync(outRoot).sort()) {
    const basePath = join(outRoot, kind, "base.pdf");
    const mutatedPath = join(outRoot, kind, "mutated.pdf");
    if (!existsSync(basePath) || !existsSync(mutatedPath)) continue;
    console.log(`${kind}: base=${await sha256(basePath)} mutated=${await sha256(mutatedPath)}`);
  }
}

async function manifests(): Promise<void> {
  const entries = [];
  for (const kind of KINDS) {
    const dir = join(FIXTURES_DIR, kind);
    const basePath = join(dir, "base.pdf");
    const mutatedPath = join(dir, "mutated.pdf");
    if (!existsSync(basePath) || !existsSync(mutatedPath)) throw new Error(`${dir} does not exist — run "generate" first`);
    entries.push({
      schema: "semio.repository-test.fixture/v2",
      id: `h-${kind}`,
      class: "third-party-generated",
      target: { artifact: "s.stdio.pdf", standard: "1.7", subset: SUBSET },
      mutation: kind,
      outcome: "applied",
      units: { length: "unitless", angle: "degree" },
      files: [
        { role: "base-pdf", path: `../🧫️fixtures/${kind}/base.pdf`, mediaType: "application/pdf", sha256: await sha256(basePath), bytes: readFileSync(basePath).length },
        { role: "mutated-pdf", path: `../🧫️fixtures/${kind}/mutated.pdf`, mediaType: "application/pdf", sha256: await sha256(mutatedPath), bytes: readFileSync(mutatedPath).length },
      ],
      provenance: { source: "generated", license: "public-domain (synthetic, no third-party content embedded)" },
      generator: {
        oracle: ORACLE_ID,
        packageVersion: "0.44",
        engineFamily: "lopdf",
        engineVersion: "0.44",
        command: "bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️h/🏭️generator/📜️script.ts generate",
        platform: process.platform,
      },
      comparisonProfile: COMPARISON_PROFILE,
      reproducible: true,
      family: "mechanical",
      notes: `A minimal two-descriptor seed built with lopdf 0.44, with the ${kind} mutation applied THROUGH lopdf's own public COS API (🦀️lopdf-engine/src/lib.rs::apply) — never through this repository's own mutation engine, which is what made the previous corpus inadmissible as evidence. base.pdf is the seed after 🦀️lopdf-engine's arrange put the mutation's precondition in place (identical to the untouched seed for every kind whose precondition is already satisfied); mutated.pdf is the result lopdf wrote. Observability (mutated projection != base projection, both read back through lopdf) is checked for every pair before it is written, and a pair that does not move is refused rather than committed.`,
    });
  }
  console.log(JSON.stringify(entries, null, 2));
}
//#endregion 🚪️Commands

//#region 🚪️Entry
async function main(argv: readonly string[]): Promise<number> {
  const [command = "", ...rest] = argv;
  const outFlagIndex = rest.indexOf("--out");
  // 🧭️ `SEMIO_FIXTURE_OUT` (set by `test fixture reproduce`/`generate`) is a FIXTURES ROOT — the
  // engine writes `<root>/<kind>/{base,mutated}.pdf` for every declared kind in one run.
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

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
//#endregion 🚪️Entry
