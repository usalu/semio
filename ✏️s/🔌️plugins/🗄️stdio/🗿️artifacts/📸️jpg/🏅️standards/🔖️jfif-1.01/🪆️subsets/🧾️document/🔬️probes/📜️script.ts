#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External measurement probes for `s.stdio.jpg@jfif-1.01/🧾️document`'s reader oracle.
//
// Everything here MARSHALS and READS; nothing here applies a mutation or predicts what one should
// produce. The actual JPEG decode is performed by the sibling standalone `jpeg-jfif-codec` binary
// (`../🏭️generator/🔁️codec/📦️packages/🦀️rust`, depends on nothing but `image` 0.25) via its `project`
// subcommand — this file only shells out to it and performs the GATING structural comparison
// itself. No JPEG semantics are computed here, only projection + compare.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts jpg-import  --input <a.jpg>
//   bun 📜️script.ts jpg-project --input <a.jpg>
//   bun 📜️script.ts jpg-compare --input <expected.jpg> --input <actual.jpg>
//
// @see 🧰️framework/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
// @see ../../../../📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🔬️probes/📜️script.ts — the sibling probe
//      suite this file's CLI/dispatch/compare shape is mirrored from.
// @see ../🏭️generator/🔁️codec/🦀️.rs — the `project` subcommand this file calls,
//      and its own module docstring recording exactly what `image` 0.25.10 can and cannot see.

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
