#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🖊️ The `dxf-entities` family — sixteen recipes, one per mutation kind, class
// `third-party-generated`. Every drawing is assembled with the `dxf` crate's own `Drawing`/`Entity`
// builders and written by its own `save_file`; no byte of DXF here is produced by this repository.
//
// The drawings are R12 (`$ACADVER AC1009`), matching the subset's dialect, and they carry SEVEN of
// the nine `CadEntity` shapes. `Ellipse` and `Dimension` are absent because they are R13+ entities
// that the `dxf` crate will not write into an R12 document — a measured constraint of the dialect,
// recorded in the manifest as uncarried rather than worked around by silently promoting the version.
//
//   bun 📜️script.ts generate [--only <recipe>]
//
// @see ../📜️script.ts — the family router that actually invokes the builder

//#endregion 🧲️Header

//#region 🚪️Entry
import { spawnSync } from "node:child_process";
import { join } from "node:path";

const FAMILY = "dxf-entities";
const ROUTER = join(import.meta.dir, "..", "📜️script.ts");

const RECIPES = [
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

function main(argv: readonly string[]): number {
  const onlyIndex = argv.indexOf("--only");
  const requested = onlyIndex === -1 ? [...RECIPES] : argv.slice(onlyIndex + 1).filter((entry) => !entry.startsWith("--"));
  const mine = requested.filter((recipe) => (RECIPES as readonly string[]).includes(recipe));
  if (mine.length === 0) {
    console.log(`[${FAMILY}] nothing to do`);
    return 0;
  }
  const run = spawnSync("bun", [ROUTER, "generate", ...mine.flatMap((recipe) => ["--only", recipe])], { encoding: "utf8", stdio: ["ignore", "inherit", "inherit"] });
  return run.status ?? 1;
}

if (import.meta.main) process.exit(main(process.argv.slice(2)));
export { FAMILY, RECIPES };
//#endregion 🚪️Entry
