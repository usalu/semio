#!/usr/bin/env repo
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.pdf@1.7/🧱️base`.
//
// Drives TWO engines, both depending on `lopdf` 0.44 and nothing else, and neither of them this
// repository's own `encode_pdf`:
//
//   * `🦀️engine`       — writes `📊️report-strip.pdf`, the shared seed ASSET other subsets reuse.
//   * `⚖️lopdf-engine` — writes the sixty before/after MUTATION pairs, applying each
//                        mutation through lopdf's own public COS API and reading the structural
//                        projection back through it, so nothing here predicts what the reader judges.
//
// This script only marshals: it builds and invokes the binaries and reports what they wrote.
//
// Generation and execution are SEPARATE operations, per the shared framework's own rule (a normal
// test run must never rewrite the expectation it is measured against): this command is the only one
// that writes into `../🧫️fixtures/`, and its output is reviewed and committed before any test reads it.
//
//   bun 📜️script.ts generate [--out <dir>]   # (re)builds both engines and writes every fixture
//   bun 📜️script.ts manifests                 # prints the testEvidence entries for all 61 committed fixtures
//
// @see ../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️gif-las-pdf17-findings.md

//#endregion 🧲️Header

//#region 🔌️Adapters
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptProgram } from "./🟦️.ts";

/** 🚪️ `generate` — runs the program of this script. */
class GenerateScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🧭️ `manifests` — runs the program of this script. */
class ManifestsScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
const router = new ScriptRouter(import.meta.dir).register("generate", GenerateScript).register("manifests", ManifestsScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, argv: ["generate"] }));
