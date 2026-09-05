#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for the three LAYER-METADATA kinds of `s.draw.drawing@1/✳️any`.
//
// This subset already carries `quick-xml-drawing-1-mutate`, which judges the SVG export. SVG has no
// representation for a layer's `locked` flag, its `blendMode`, or its authoring `name` — editor
// metadata that never reaches the rendered document — so those three kinds were `-uncarried` against
// it. They ride this subset's JSON carrier instead, where `DrawingSnapshot::layers` is an INLINE
// `Vec<DrawingLayerNode>` and every one of the three is a carrier-level fact.
//
// `🦀️json-engine` depends on `serde_json` and nothing else: it applies each mutation as an edit to the
// carrier and reads it back through the same third-party library. It refuses to write a pair whose
// projection does not move, so a no-op cannot be committed as a fixture that would pass forever.
//
//   bun 📜️script.ts generate [--out <dir>]   # builds the engine and writes the fixture pairs
//   bun 📜️script.ts manifests                 # prints the fixtureManifests entries
//
// @see ../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️fem-carrier-reader-retrofit.md

//#endregion 🧲️Header

//#region 🔌️Adapters
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
//#endregion 🔌️Adapters

//#region 🧬️Contract
const HERE = import.meta.dir;
const ENGINE = join(HERE, "🦀️json-engine");
const FIXTURES_DIR = join(HERE, "..", "🧫️fixtures");
const ORACLE_ID = "serde-json-drawing-carrier-reader";
const COMPARISON_PROFILE = "semantic-drawing-carrier-v1";
/** 🧾️ Kept in step with `🦀️json-engine/src/📚️lib.rs::KINDS`. */
const KINDS: readonly string[] = ["set-layer-locked", "set-layer-blend-mode", "rename-layer"];
//#endregion 🧬️Contract

//#region 🔨️Build
function build(): void {
  const result = spawnSync("cargo", ["build", "--release", "--offline", "--manifest-path", join(ENGINE, "Cargo.toml")], { stdio: "inherit" });
  if (result.status !== 0) throw new Error(`cargo build failed with status ${result.status}`);
}

async function sha256(path: string): Promise<string> {
  const digest = await crypto.subtle.digest("SHA-256", readFileSync(path));
  return `sha256:${[...new Uint8Array(digest)].map((byte) => byte.toString(16).padStart(2, "0")).join("")}`;
}
//#endregion 🔨️Build

//#region 🚪️Commands
function generate(outRoot: string): number {
  build();
  const result = spawnSync(join(ENGINE, "target", "release", "generate"), [outRoot], { stdio: "inherit" });
  return result.status ?? 1;
}

async function manifests(): Promise<void> {
  const entries = [];
  for (const kind of KINDS) {
    const dir = join(FIXTURES_DIR, kind);
    if (!existsSync(dir)) throw new Error(`missing fixture directory for ${kind} — run generate first`);
    const files = [];
    for (const [role, name] of [["expected-before-json", "before.json"], ["expected-after-json", "after.json"]] as const) {
      const path = join(dir, name);
      files.push({ role, path: `../🧫️fixtures/${kind}/${name}`, mediaType: "application/json", sha256: await sha256(path), bytes: readFileSync(path).length });
    }
    entries.push({
      schema: "semio.repository-test.fixture/v2",
      id: `carrier-${kind}`,
      class: "third-party-generated",
      target: { artifact: "s.draw.drawing", standard: "1", subset: "any" },
      mutation: kind,
      outcome: "applied",
      units: { length: "unitless", angle: "radian" },
      files,
      provenance: { source: "generated", license: "public-domain (synthetic, no third-party content embedded)" },
      generator: { oracle: ORACLE_ID, packageVersion: "1", engineFamily: "serde-json", engineVersion: "1", command: "bun ✏️s/🔌️plugins/🖍️drawing/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts generate", platform: process.platform },
      comparisonProfile: COMPARISON_PROFILE,
      reproducible: true,
      family: "mechanical",
      notes: `A deterministic two-layer drawing document with the ${kind} mutation applied as an edit to the JSON CARRIER and read back through serde_json — never through this repository's own mutation engine. SVG, which this subset's other reader judges, has no representation for this field. Observability is checked before a pair is written, and a pair that does not move is refused rather than committed.`,
    });
  }
  console.log(JSON.stringify(entries, null, 2));
}
//#endregion 🚪️Commands

//#region 🚀️Entry
const [command, ...rest] = process.argv.slice(2);
const outFlagIndex = rest.indexOf("--out");
const outRoot = outFlagIndex >= 0 ? rest[outFlagIndex + 1]! : (process.env.SEMIO_FIXTURE_OUT ?? FIXTURES_DIR);
if (command === "generate") process.exit(generate(outRoot));
else if (command === "manifests") await manifests();
else {
  console.error("usage: bun 📜️script.ts <generate [--out <dir>]|manifests>");
  process.exit(2);
}
//#endregion 🚀️Entry
