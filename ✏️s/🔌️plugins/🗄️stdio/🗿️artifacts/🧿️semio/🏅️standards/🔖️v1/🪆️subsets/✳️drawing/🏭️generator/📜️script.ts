#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party SVG fixture generator for `s.stdio.semio@v1/✳️drawing`.
//
// SVG is the one carrier that natively represents BOTH things this subset's vocabulary edits: an
// element `transform` (rotate/scale/translate) and `<g>` nesting (group/ungroup/flatten/unflatten).
// Every fixture is a `(before.svg, after.svg)` PAIR for exactly one mutation kind — the pair IS the
// expectation, `quick-xml-drawing-svg-reader` parses both halves, and the difference it reports is
// what the mutation must produce. Nothing here applies one of our mutations: the `after` scene is
// authored directly, which is what keeps this a READER oracle and not a predicting one.
//
// Generation and execution are SEPARATE — a normal test run must never be able to rewrite the
// expectation it is measured against.
//
//   bun 📜️script.ts generate                      # build the engine and write 🧫️fixtures/<kind>/{before,after}.svg
//   bun 📜️script.ts manifests                     # (re)write 🧫️fixtures/🔣️.json
//
// @see ../../../../../../../🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts
//      — the carrier generator this file mirrors in CLI shape and manifest fields.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { createHash } from "node:crypto";
import { existsSync, readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { dirname, join } from "node:path";
//#endregion 🔌️Adapters

//#region 🧭️Paths
const here = dirname(Bun.fileURLToPath(import.meta.url));
const subset = join(here, "..");
const fixtures = join(subset, "🧫️fixtures");
const engine = join(here, "🦀️svg-engine");
const KINDS = ["change-stroke-color", "change-stroke-width", "create-layer", "create-node", "delete-layer", "delete-node", "drag-nodes", "flatten-node", "group-nodes", "move-node", "reorder-nodes", "replace-fill", "replace-path", "rotate-node", "scale-node", "unflatten-node", "ungroup-node"] as const;
//#endregion 🧭️Paths

//#region 🏭️Generate
const generate = (): void => {
  const built = spawnSync("cargo", ["build", "--release", "--offline"], { cwd: engine, stdio: "inherit" });
  if (built.status !== 0) throw new Error("engine build failed");
  const run = spawnSync(join(engine, "target", "release", "generate"), [fixtures], { stdio: "inherit" });
  if (run.status !== 0) throw new Error("fixture generation failed");
};
//#endregion 🏭️Generate

//#region 🧾️Manifests
const digest = (path: string): string => `sha256:${createHash("sha256").update(readFileSync(path)).digest("hex")}`;

const manifests = (): void => {
  const entries = KINDS.flatMap((kind) => {
    const dir = join(fixtures, kind);
    if (!existsSync(dir)) return [];
    return [{
      schema: "semio.repository-test.fixture/v2",
      id: `drawing-svg-${kind}`,
      class: "third-party-generated",
      target: { artifact: "s.stdio.semio", standard: "v1", subset: "drawing" },
      mutation: kind,
      outcome: "applied",
      // 📐️SVG user units, and SVG's own axis convention: +y points DOWN, which is why `up` is not `y`.
      units: { length: "svg-user-unit", angle: "degree", handedness: "left", up: "-y" },
      files: ["before.svg", "after.svg"].map((name) => ({
        role: name === "before.svg" ? "input-svg" : "expected-svg",
        path: `../🧫️fixtures/${kind}/${name}`,
        mediaType: "image/svg+xml",
        sha256: digest(join(dir, name)),
        bytes: statSync(join(dir, name)).size,
      })),
      generator: {
        oracle: "quick-xml-drawing-svg-reader",
        packageVersion: "0.37.5",
        engineFamily: "quick-xml",
        engineVersion: "0.37.5",
        command: `bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🏭️generator/📜️script.ts generate`,
        platform: `${process.platform}-${process.arch}`,
      },
      provenance: {
        source: "generated",
        license: "MIT (quick-xml)",
        attribution: "Serialized with quick-xml (MIT); no semio code participates in the expectation",
        security: "scanned-clean",
        privacy: "no-personal-data",
      },
      comparisonProfile: "xml-element-tree",
      toleranceProfile: "exact",
      reproducible: true,
      family: "drawing-svg-carrier",
      notes: `Authored (before, after) SVG pair for \`${kind}\`. The pair is the expectation; quick-xml reads both halves.`,
    }];
  });
  writeFileSync(join(fixtures, "🔣️.json"), `${JSON.stringify(entries, null, 2)}\n`);
  // 🧾️The REGISTRY reads `fixtureManifests` off the contribution file itself (`loadOracleRegistry`
  // parses it there, not from `🧫️fixtures/🔣️.json`), so the generated block is merged into
  // the catalog too — the standalone file stays as the generator's reviewable output.
  const catalogPath = join(subset, "🧪️oracle", "🔣️.json");
  const catalog = JSON.parse(readFileSync(catalogPath, "utf8"));
  const keep = (catalog.fixtureManifests ?? []).filter((entry: { family?: string }) => entry.family !== "drawing-svg-carrier");
  catalog.fixtureManifests = [...keep, ...entries];
  writeFileSync(catalogPath, `${JSON.stringify(catalog, null, 2)}\n`);
  console.log(`${entries.length} fixture manifest(s) written and registered in 🔣️oracle.json`);
};
//#endregion 🧾️Manifests

//#region 🚀️Main
const command = process.argv[2];
if (command === "generate") generate();
else if (command === "manifests") manifests();
else { console.error("usage: 📜️script.ts <generate|manifests>"); process.exit(2); }
//#endregion 🚀️Main
