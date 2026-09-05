#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏗️ Fixture generator for `s.stdio.semio@v1/📐️cad`. Two families, and their PROVENANCE DIFFERS —
// which is why they are separate families rather than one corpus:
//
//   🖊️dxf-entities     class `third-party-generated`. Built by the `dxf` crate's OWN `Drawing`
//                      builder and written by its OWN `save_file`. Nothing of ours writes DXF here.
//
//   ⭕️step-line-circle class `handcrafted`. `ruststep` 0.4.0 has NO Part-21 writer — no AST type
//                      implements `Display`, and `ast::ser::to_record` stops at an in-memory
//                      `Record` whose nesting disagrees with its own parser — so no third party in
//                      this repository can write STEP. The text is ours; ruststep then READS every
//                      file back and the generator REFUSES to emit one whose geometry it does not
//                      recover exactly. That is verification by a third party, not generation by
//                      one, and the class says so rather than overclaiming.
//
// Generation and execution are separate operations on purpose: a normal test run must never be able
// to rewrite the expectation it is being measured against.
//
//   bun 📜️script.ts generate                 — the whole corpus
//   bun 📜️script.ts generate --only <recipe> — one recipe, MERGED into the corpus, never replacing it
//
// `SEMIO_FIXTURE_OUT` (set by `test fixture generate|reproduce`) is a FIXTURES ROOT; every recipe
// writes `<root>/<recipe>/<file>`. Absent it, the committed 🧫️fixtures directory is the root.
//
// @see ../🔬️probes/🦀️oracle-probe/src/main.rs — the builders and the ruststep verification
// @see ../🔣️oracle.json — the fixture manifests these files are hashed into

//#endregion 🧲️Header

//#region 🔌️Adapters
import { spawnSync } from "node:child_process";
import { join } from "node:path";
//#endregion 🔌️Adapters

//#region 🗂️Recipes
/** 🖊️ Built and written BY the `dxf` crate — genuinely third-party-generated. */
const DXF_RECIPES = [
  "no-mutation-identity",
  "set-snapshot-replaces-drawing",
  "add-layer-hidden-services",
  "remove-layer-scratch",
  "set-layer-walls-color",
  "add-block-door",
  "remove-block-window",
  "set-block-base-point-door",
  "add-entity-arc-fillet",
  "remove-entity-middle-polyline",
  "set-entity-layer-text-to-annotations",
  "set-entity-geometry-circle-radius",
  "add-block-entity-door-swing",
  "remove-block-entity-window-mullion",
  "set-block-entity-layer-door-leaf",
  "set-block-entity-geometry-window-pane",
] as const;

/** 📐️ Written by us, VERIFIED by ruststep — handcrafted, and labelled handcrafted. */
const STEP_RECIPES = ["step-no-mutation-identity", "step-set-snapshot-replaces-entities", "step-add-entity-circle", "step-remove-entity-line", "step-set-entity-geometry-circle-radius"] as const;

const CRATE_DIR = join(import.meta.dir, "..", "🔬️probes", "🦀️oracle-probe");
const COMMITTED_FIXTURES = join(import.meta.dir, "..", "🧫️fixtures");
//#endregion 🗂️Recipes

//#region 🚪️Entry
function main(argv: readonly string[]): number {
  const [command = "generate"] = argv;
  if (command !== "generate") {
    console.error(`[generator] unknown command ${JSON.stringify(command)} — expected generate [--only <recipe>]`);
    return 2;
  }
  const onlyIndex = argv.indexOf("--only");
  const only = onlyIndex === -1 ? [] : argv.slice(onlyIndex + 1).filter((entry) => !entry.startsWith("--"));
  const known = [...DXF_RECIPES, ...STEP_RECIPES] as readonly string[];
  const unknown = only.filter((recipe) => !known.includes(recipe));
  if (unknown.length > 0) {
    console.error(`[generator] unknown recipe(s) ${unknown.join(", ")} — known: ${known.join(", ")}`);
    return 2;
  }
  const out = process.env.SEMIO_FIXTURE_OUT ?? COMMITTED_FIXTURES;
  const target = process.env.CARGO_TARGET_DIR ?? join(process.env.SEMIO_AGENT_CACHE ?? join(CRATE_DIR, "target"), "probe");
  const args = ["run", "--quiet", "--offline", "--bin", "semio-cad-oracle-probe", "--", "generate", "--out", out, ...only.flatMap((recipe) => ["--only", recipe])];
  const run = spawnSync("cargo", args, { cwd: CRATE_DIR, encoding: "utf8", env: { ...process.env, CARGO_TARGET_DIR: target }, stdio: ["ignore", "inherit", "pipe"] });
  if (run.status !== 0) {
    // 🚫️A generator that cannot run must SAY SO and exit non-zero. Leaving the previous bytes in
    // place and reporting success would make `fixture reproduce` compare a stale corpus with itself.
    console.error(`[generator] cargo exited ${run.status}: ${(run.stderr ?? "").trim().split("\n").slice(-8).join("\n")}`);
    return 1;
  }
  return 0;
}

if (import.meta.main) process.exit(main(process.argv.slice(2)));
export { DXF_RECIPES, STEP_RECIPES };
//#endregion 🚪️Entry
