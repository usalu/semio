#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.pdf@1.7/♿️ua` — the 18-kind ISO 16612-2 (PDF/VT-1)
// conformance-class catalog.
//
// Every before/after pair `🔁️codec/🏗️generate/🦀️.rs` writes is produced by the SAME registered `lopdf`
// 0.44 reference implementation named `lopdf-pdf-1-7-ua-mutate` in `../🔣️oracle.json` —
// through `lopdf`'s own public COS API inside the standalone
// `semio-s-artifact-stdio-pdf-test-oracle` crate, the identical engine the differential test case
// `../../../../../../🧪️tests/♿️mutate-pdf-1-7-ua` drives — never this repository's own production PDF
// codec, and never hand-rolled to match it. This script only marshals: it builds and invokes the
// Rust binary and reports what it wrote; it computes no PDF bytes itself.
//
// One generator run produces EVERY declared kind's recipe in one pass — the engine loops `KINDS`
// itself — so `generate`/`manifests` operate on the whole 18-recipe corpus, not one recipe at a time.
//
// Generation and execution are SEPARATE operations, per the shared framework's own rule (a normal
// test run must never rewrite the expectation it is measured against): this command is the only one
// that writes into `../🧫️fixtures/`, and its output is reviewed and committed before any test reads it.
//
//   bun 📜️script.ts generate [--out <dir>]   # (re)builds the engine, writes every recipe, prints digests
//   bun 📜️script.ts manifests                 # prints the testEvidence entries for the committed corpus
//
// @see ../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️pilot-playbook.md

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
