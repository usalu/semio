#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.bmp@v3/✳️any`.
//
// Every recipe's BEFORE and AFTER `.bmp` bytes are built DIRECTLY by the sibling standalone
// `🦀️image-bmp-codec` binary, which depends on nothing but `image` 0.25 (crates.io, MIT OR
// Apache-2.0, `bmp` feature only) — never by "applying" this repository's own `BmpMutation`
// dispatch. This file only shells out per recipe and turns the bytes the codec wrote into a
// fixture bundle + manifest entry; it computes no BMP semantics of its own.
//
// Only 4 of this subset's 5 declared mutation kinds have a recipe here — `change-header-fields`
// has none, because `image`'s public BMP decoder exposes neither the row-order flag, x/y
// pixels-per-metre, compression, colorsUsed as a value distinct from palette length, nor
// colorsImportant (see the codec's own header comment and this subset's own
// `📓️bmp-v3-any-reader-oracle-retrofit.md` for the source-level evidence); that mutation is
// registered `bmp-3-mutate-uncarried` in `../🔣️oracle.json` instead of against this oracle.
//
// Generation and execution are SEPARATE operations, same shape as the sibling `avi`/`bcf`/`mesh`/
// `brep` generators this file's CLI is mirrored from: a normal test run must never be able to
// rewrite the expectation it is measured against.
//
//   bun 📜️script.ts generate  [--only <fixture-id>]     # writes each handpicked fixture directory
//   bun 📜️script.ts manifests [--only <fixture-id>]     # prints the testEvidence block (JSON)
//
// @see ../../../../../../../../📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🏭️generator/📜️script.ts —
//      the sibling generator this file's CLI/recipe shape is mirrored from.
// @see ./🔁️codec/🦀️.rs — the actual codec; `build <recipe-id> <out-dir>` and
//      `project <path>` are its only two commands.
// @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/

//#endregion 🧲️Header

//#region 🔌️Adapters
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptProgram } from "./🟦️.ts";

/** 🚪️ `run` — runs the program of this script. */
class RunScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
const router = new ScriptRouter(import.meta.dir).register("run", RunScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, argv: ["run"] }));
