#!/usr/bin/env repo
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.avi@1.0/🎛️hdrl`.
//
// Every recipe's BEFORE and (where the outcome is legal) AFTER `.avi` bytes are built DIRECTLY by
// the sibling standalone `🦀️riff-avi-codec` binary, which depends on nothing but `riff` 2.0
// (crates.io, MIT, 11.9M downloads) for the generic RIFF chunk/LIST framing — never by "applying"
// this repository's own `AviMutation` dispatch. This file only shells out per recipe and turns the
// bytes the codec wrote into a fixture bundle + manifest entry; it computes no AVI semantics of
// its own.
//
// Generation and execution are SEPARATE operations, same shape as the sibling `bcf`/`mesh`/`brep`
// generators this file's CLI is mirrored from: a normal test run must never be able to rewrite the
// expectation it is measured against.
//
//   bun 📜️script.ts generate  [--only <fixture-id>]     # writes each declared emoji directory and leaf
//   bun 📜️script.ts manifests [--only <fixture-id>]     # prints the testEvidence block (JSON)
//
// @see ../../../../💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🏭️generator/📜️script.ts — the sibling
//      generator this file's CLI/recipe shape is mirrored from (both hand-author before/after
//      states directly rather than executing mutation dispatch).
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
