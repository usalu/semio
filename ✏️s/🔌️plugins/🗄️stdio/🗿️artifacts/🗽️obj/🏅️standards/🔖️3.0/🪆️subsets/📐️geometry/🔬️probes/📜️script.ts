#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External measurement probes for `s.stdio.obj@3.0/📐️geometry`.
//
// Everything here MARSHALS and READS; nothing here applies a mutation or predicts what one should
// produce. The actual OBJ decode is performed by the sibling standalone `tobj-obj-reader` binary
// (`../🏭️generator/📖️reader/📦️packages/🦀️rust`, depends on nothing but the real `tobj` 4 — the SAME crate
// registered as `tobj-obj-3-0-mutate-reader`) via its `project` subcommand — this file only shells
// out to it and performs the GATING structural comparison itself. No OBJ semantics computed here,
// only projection + compare, mirroring the sibling `avi`/`bcf` probe suites' identical division of
// labor.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts obj-import  --input <a.obj>
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
