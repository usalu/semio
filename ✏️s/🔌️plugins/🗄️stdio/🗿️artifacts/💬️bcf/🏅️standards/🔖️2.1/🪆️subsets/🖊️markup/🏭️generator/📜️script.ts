#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.bcf@2.1/🖊️markup`.
//
// Every recipe below is a BEFORE and (where the outcome is legal) an AFTER bcfzip, each one built
// DIRECTLY by `jszip`+`fast-xml-parser`'s `XMLBuilder` — never by "applying" a mutation in code. Both
// states of every recipe are independently authored here; nothing in this file re-derives one from
// the other by executing mutation semantics, which is the whole reason this counts as independent
// evidence rather than a reflection of this repository's own dispatch.
//
// Generation and execution are SEPARATE operations, same shape as the `mesh`/`brep`/`step@ap214/cc6`
// generators this file's CLI is mirrored from: a normal test run must never be able to rewrite the
// expectation it is measured against.
//
//   bun 📜️script.ts generate --only <fixture-id>
//
// @see ../../../../🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏭️generator/📜️script.ts — the sibling
//      generator this file's CLI shape is mirrored from
// @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/

//#endregion 🧲️Header

//#region 🔌️Adapters
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptProgram } from "./🟦️.ts";

/** 🚪️ `run` — runs the program of this script. */
class RunScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
const router = new ScriptRouter(import.meta.dir).register("run", RunScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "run" }));
