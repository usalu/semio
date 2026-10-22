#!/usr/bin/env repo
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.svg@1.1/🧱️base`.
//
// Every recipe's BEFORE and AFTER `.svg` bytes are built DIRECTLY by the sibling standalone
// `🦀️quick-xml-svg-codec` binary, which depends on nothing but `quick-xml` 0.42 — never by
// "applying" this repository's own `SvgMutation`/`SvgDiff` dispatch, and never by this subset's
// own `🦀️oracle.rs` (that module COMPUTES what a mutation should produce; it is a
// SEPARATE, untouched `cross-semio-implementation` oracle). This file only shells out per recipe
// and turns the bytes the codec wrote into a fixture bundle + manifest entry; it computes no SVG
// semantics of its own.
//
// Generation and execution are SEPARATE operations, same shape as the sibling `avi`/`bcf`
// generators this file's CLI is mirrored from: a normal test run must never be able to rewrite
// the expectation it is measured against.
//
//   bun 📜️script.ts generate  [--only <fixture-id>]     # writes the semantic fixture directory's ⬅️before/➡️after pair
//   bun 📜️script.ts manifests [--only <fixture-id>]     # prints the testEvidence block (JSON)
//
// @see ../../../../../📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🏭️generator/📜️script.ts — the
//      sibling generator this file's CLI/recipe shape is mirrored from.
// @see ./🔁️codec/🦀️.rs — the actual codec; `build <recipe-id> <out-dir> <physical-dir> <before-file> <after-file>`,
//      `project <path>` and `list-recipes` are its only three commands.
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
