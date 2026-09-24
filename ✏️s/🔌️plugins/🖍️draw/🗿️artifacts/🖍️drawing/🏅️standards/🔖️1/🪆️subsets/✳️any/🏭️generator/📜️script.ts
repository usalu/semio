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
// `🧩️json` depends on `json` (json-rust) and nothing else — never `serde_json`, which the draw plugin
// itself links: it applies each mutation as an edit to the carrier and reads it back through the same
// third-party library. It refuses to write a pair whose
// projection does not move, so a no-op cannot be committed as a fixture that would pass forever.
//
//   bun 📜️script.ts generate [--out <dir>]   # builds the engine and writes the reviewed fixture pairs
//   bun 📜️script.ts manifests                 # refreshes digests and provenance of this oracle's fixture manifests
//
// @see ../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️fem-carrier-reader-retrofit.md

//#endregion 🧲️Header

//#region 🔌️Adapters
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { spawnSync } from "node:child_process";
import { cargoTargetDirectory } from "../../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { getWorkspaceRoot } from "../../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import { currentPlatform } from "../../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧬️Contract
const HERE = import.meta.dir;
const ENGINE = join(HERE, "🧩️json", "📦️packages", "🦀️rust");
const SUBSETS_DIR = join(HERE, "..", "..");
const CATALOGS = ["🏷️metadata", "🎨️style"].map((subset) => join(SUBSETS_DIR, subset, "🔮️oracles", "🔣️.json"));
const ORACLE_ID = "json-rust-drawing-carrier-reader";
const GENERATOR = { oracle: ORACLE_ID, packageVersion: "0.12", engineFamily: "json-rust", engineVersion: "0.12", command: "bun ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts generate" } as const;
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
  const result = spawnSync(join(cargoTargetDirectory(getWorkspaceRoot()), "release", "generate"), [outRoot], { stdio: "inherit" });
  return result.status ?? 1;
}

async function manifests(): Promise<void> {
  let refreshed = 0;
  for (const catalogPath of CATALOGS) {
    const catalog = JSON.parse(readFileSync(catalogPath, "utf8")) as { fixtureManifests?: { generator?: { oracle?: string }; files: { path: string; sha256: string; bytes: number }[] }[] };
    for (const entry of (catalog.fixtureManifests ?? []).filter((candidate) => candidate.generator?.oracle === ORACLE_ID)) {
      for (const file of entry.files) {
        const path = join(dirname(catalogPath), file.path);
        file.sha256 = await sha256(path);
        file.bytes = readFileSync(path).length;
      }
      entry.generator = { ...GENERATOR, platform: currentPlatform() };
      refreshed += 1;
    }
    writeFileSync(catalogPath, `${JSON.stringify(catalog, null, 2)}\n`);
  }
  if (refreshed === 0) throw new Error(`no fixture manifest names ${ORACLE_ID}`);
  console.log(`${refreshed} fixture manifest(s) refreshed`);
}
//#endregion 🚪️Commands

//#region 🚀️Entry
const [command, ...rest] = process.argv.slice(2);
const outFlagIndex = rest.indexOf("--out");
const outRoot = outFlagIndex >= 0 ? rest[outFlagIndex + 1]! : (process.env.SEMIO_FIXTURE_OUT ?? SUBSETS_DIR);
if (command === "generate") process.exit(generate(outRoot));
else if (command === "manifests") await manifests();
else {
  console.error("usage: bun 📜️script.ts <generate [--out <dir>]|manifests>");
  process.exit(2);
}
//#endregion 🚀️Entry
