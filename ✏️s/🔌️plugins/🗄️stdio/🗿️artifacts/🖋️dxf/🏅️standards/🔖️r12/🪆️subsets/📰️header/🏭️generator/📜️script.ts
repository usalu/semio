#!/usr/bin/env repo
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.dxf@r12/📰️header`.
//
// Every recipe's bytes are written entirely by the real `dxf` 0.6 crate's own typed `Drawing`
// model (`🧫️fixtures/🦀️.rs`) — the SAME crate registered as `dxf-crate-r12-mutate`
// (`cross-semio-implementation`, untouched) and now also as `dxf-crate-r12-mutate-reader`
// (`third-party-library`) in `../🔮️oracles/🔣️.json` — never by this repository's own DXF codec.
// This script only marshals: it builds and invokes the Rust binary and reports what it wrote; it
// computes no DXF bytes itself.
//
// Two recipe SHAPES share one engine binary:
//   * `drafting-plate` — the pre-existing single-document fixture for the `cross-semio-implementation`
//     oracle's own testing shape. Untouched in content (verified byte-identical to the previously
//     committed fixture — see the ticket-root report). `RECIPES` below carries it with `kind:
//     "single"` so `manifests` reproduces its EXACT pre-existing entry shape (`files: [{role:
//     "primary-dxf", ...}]`, no `mutation`/`outcome`/`comparisonProfile`).
//   * one `<kind>-applied` / `<kind>-no-op` / `<kind>-rejected-<reason>` recipe per WITNESSABLE
//     mutation kind — `kind: "pair"`, `files: [expected-before-dxf, expected-after-dxf?]`, the new
//     corpus this retrofit adds for the `dxf-crate-r12-mutate-reader` oracle.
//
// Generation and execution are SEPARATE operations (a normal test run must never rewrite the
// expectation it is measured against): this command is the only one that writes into
// `../🧫️fixtures/`, and its output is reviewed and committed before any test reads it.
//
//   bun 📜️script.ts generate  [--only <recipe-id>]     # (re)builds the engine, writes fixture(s)
//   bun 📜️script.ts manifests [--only <recipe-id>]     # prints the testEvidence block (JSON array)
//
// @see ./🧫️fixtures/🦀️.rs — the actual codec; `build <recipe-id> <out-dir>` and
//      `project <path>` are its only two data-producing commands (plus `list-recipes`).
// @see ../../../../💬️avi/🏅️standards/🔖️1.0/🪆️subsets/📰️header/🏭️generator/📜️script.ts — the sibling
//      generator this file's `--only`/RECIPES/generate/manifests shape is mirrored from.
// @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️dxf-r12-any-reader-oracle-retrofit.md

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
