#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External measurement probes for `s.stdio.avi@1.0/🎛️hdrl`.
//
// Everything here MARSHALS and READS; nothing here applies a mutation or predicts what one should
// produce. The actual RIFF/AVI decode is performed by the sibling standalone `riff-avi-codec`
// binary (`../🏭️generator/🔁️codec/📦️packages/🦀️rust`, depends on nothing but `riff` 2.0) via its `project`
// subcommand — this file only shells out to it, hashes each opaque chunk payload it returns into a
// size+digest pair (per `semantic-avi-v1`'s own "opaque binary payload" treatment, mirroring the
// BCF probe's identical `snapshotDigest` treatment of a PNG viewpoint snapshot), and performs the
// GATING structural comparison itself — no AVI semantics computed here, only projection + compare.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts avi-import  --input <a.avi>
//   bun 📜️script.ts avi-project --input <a.avi>
//   bun 📜️script.ts avi-compare --input <expected.avi> --input <actual.avi>
//
// @see 🧰️framework/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
// @see ../../../../💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🔬️probes/📜️script.ts — the sibling
//      probe suite this file's CLI/dispatch/compare shape is mirrored from (both hand the
//      structural equality itself to this file, never to a computed prediction)
// @see ../🏭️generator/🔁️codec/🦀️.rs — the `project` subcommand this file calls

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
