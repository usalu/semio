#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.las@1.0/✳️header`.
//
// The bytes this file produces are written entirely by the real `las` 0.11 crate's byte-exact
// `las::raw::{Header, Vlr, Point}` types (`🦀️engine/src/main.rs`) — the SAME crate registered as
// `las-1-0-any-mutate` in `../🔣️oracle.json` — never by this repository's own `encode_las`. This
// script only marshals: it builds and invokes the Rust binary and reports what it wrote; it
// computes no LAS bytes itself.
//
// Generation and execution are SEPARATE operations, per the shared framework's own rule (a normal
// test run must never rewrite the expectation it is measured against): this command is the only one
// that writes into `../🧫️fixtures/`, and its output is reviewed and committed before any test reads it.
//
//   bun 📜️script.ts generate [--out <dir>]   # (re)builds the engine, writes the fixture, prints its sha256
//   bun 📜️script.ts manifests                 # prints the fixtureManifests entry for the committed fixture
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
const ENGINE_DIR = join(HERE, "🦀️engine");
const ENGINE_BIN = join(ENGINE_DIR, "target", "release", "generate");
const FIXTURES_DIR = join(HERE, "..", "🧫️fixtures");
const RECIPE = "survey-strip";
const FIXTURE_FILE = "survey-strip.las";
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
async function generate(outDir: string): Promise<void> {
  ensureBuilt();
  mkdirSync(outDir, { recursive: true });
  const outPath = join(outDir, FIXTURE_FILE);
  const result = spawnSync(ENGINE_BIN, [outPath], { stdio: "inherit" });
  if (result.status !== 0) throw new Error(`generator binary failed with status ${result.status}`);
  const digest = await sha256(outPath);
  console.log(`${outPath}\n${digest}`);
}

async function manifests(): Promise<void> {
  const outPath = join(FIXTURES_DIR, RECIPE, FIXTURE_FILE);
  if (!existsSync(outPath)) throw new Error(`${outPath} does not exist — run "generate" first`);
  const digest = await sha256(outPath);
  const entry = {
    schema: "semio.repository-test.fixture/v2",
    id: RECIPE,
    class: "third-party-generated",
    family: "mechanical",
    files: [{ path: `../🧫️fixtures/${RECIPE}/${FIXTURE_FILE}`, mediaType: "application/vnd.las", sha256: digest }],
    provenance: { license: "public-domain (synthetic, no third-party content embedded)" },
    generator: {
      oracle: "las-1-0-any-mutate",
      packageVersion: "0.11",
      engineFamily: "las-rs",
      engineVersion: "0.11",
      command: "bun 🏭️generator/📜️script.ts generate",
      platform: process.platform,
    },
    reproducible: true,
  };
  console.log(JSON.stringify(entry, null, 2));
}
//#endregion 🚪️Commands

//#region 🚪️Entry
async function main(argv: readonly string[]): Promise<number> {
  const [command = "", ...rest] = argv;
  const outFlagIndex = rest.indexOf("--out");
  // 🧭️ `SEMIO_FIXTURE_OUT` (set by `test fixture reproduce`/`generate`) is a FIXTURES ROOT, not a
  // per-recipe directory — every generator in the repository writes `<root>/<recipe>/<file>`.
  const fixtureOutRoot = process.env.SEMIO_FIXTURE_OUT;
  const outDir = outFlagIndex >= 0 ? rest[outFlagIndex + 1]! : fixtureOutRoot !== undefined ? join(fixtureOutRoot, RECIPE) : join(FIXTURES_DIR, RECIPE);
  if (command === "generate") {
    await generate(outDir);
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
