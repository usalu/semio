#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 📐️ The `step-line-circle` family — five recipes, class `handcrafted`, NOT third-party-generated,
// and the distinction is deliberate rather than cautious.
//
// `ruststep` 0.4.0 cannot write Part-21. Measured two ways: no AST type implements `Display`
// (`Record`, `DataSection` and `Exchange` all fail to compile in a `println!`), and `ast::ser::to_record`
// stops at an in-memory `Record` whose nesting disagrees with what its own parser produces for the
// same entity. Its lib.rs claim to do "reading and writing" does not hold for 0.4.0's write half.
// No other STEP writer is available here — `brepjs` was measured against a cad-shaped file and
// returns zero shapes, because OCCT transfers through product/shape-representation structure and
// this subset's export emits bare LINE/CIRCLE primitives.
//
// So the Part-21 text is OURS, and the class says `handcrafted`. What ruststep does contribute is
// VERIFICATION: every file is read back by ruststep before it is hashed, and the generator refuses
// to emit one whose LINE/CIRCLE geometry it does not recover exactly. Calling these
// `third-party-generated` would be a false provenance claim, so they are not.
//
// The recipes stay inside Line and Circle on purpose: `SemioCadToStep` drops the other seven entity
// shapes and never reads layers or blocks at all, so a STEP recipe touching those would be
// unwitnessable by construction. Those kinds are registered against the dxf oracle alone.
//
//   bun 📜️script.ts generate [--only <recipe>]
//
// @see ../📜️script.ts — the family router that actually invokes the builder

//#endregion 🧲️Header

//#region 🚪️Entry
import { spawnSync } from "node:child_process";
import { join } from "node:path";

const FAMILY = "step-line-circle";
const ROUTER = join(import.meta.dir, "..", "📜️script.ts");

const RECIPES = ["step-no-mutation-identity", "step-set-snapshot-replaces-entities", "step-add-entity-circle", "step-remove-entity-line", "step-set-entity-geometry-circle-radius"] as const;

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
