#!/usr/bin/env repo
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.las@1.0/🎩️header`.
//
// The bytes this file produces are written entirely by the real `las` 0.11 crate's byte-exact
// `las::raw::{Header, Vlr, Point}` types (`🧫️fixtures/🦀️.rs`) — the SAME crate registered as
// `las-1-0-any-mutate` in `../🔣️oracle.json` — never by this repository's own `encode_las`. This
// script only marshals: it builds and invokes the Rust binary and reports what it wrote; it
// computes no LAS bytes itself.
//
// Generation and execution are SEPARATE operations, per the shared framework's own rule (a normal
// test run must never rewrite the expectation it is measured against): this command is the only one
// that writes into `../🧫️fixtures/`, and its output is reviewed and committed before any test reads it.
//
//   bun 📜️script.ts generate [--out <dir>]   # (re)builds the engine, writes the fixture, prints its sha256
//   bun 📜️script.ts manifests                 # prints the testEvidence entry for the committed fixture
//
// @see ../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️gif-las-pdf17-findings.md

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
