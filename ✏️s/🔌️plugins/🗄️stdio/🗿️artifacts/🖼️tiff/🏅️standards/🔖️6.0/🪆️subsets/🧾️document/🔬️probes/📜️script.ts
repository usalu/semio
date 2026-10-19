#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External measurement probes for `s.stdio.tiff@6.0/🧾️document`.
//
// Everything here MARSHALS and READS; nothing here applies a mutation or predicts what one should
// produce. The actual TIFF IFD/tag decode is performed by the sibling standalone `tiff-ifd-codec`
// binary (`../🏭️generator/🔁️codec/📦️packages/🦀️rust`, depends on nothing but `tiff` 0.11) via its `project`
// subcommand — this file only shells out to it and performs the GATING structural comparison
// itself (already-hashed `samplesDigest` for the raster, per the codec's own opaque-payload
// treatment of pixel data — never raw bytes cross this boundary either) — no TIFF semantics
// computed here, only marshalling + compare.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts tiff-import  --input <a.tiff>
//   bun 📜️script.ts tiff-project --input <a.tiff>
//   bun 📜️script.ts tiff-compare --input <expected.tiff> --input <actual.tiff>
//
// @see 🧰️framework/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
// @see ../../../../../../📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🔬️probes/📜️script.ts — the sibling
//      probe suite this file's CLI/dispatch/compare shape is mirrored from (both hand the
//      structural equality itself to this file, never to a computed prediction)
// @see ../🏭️generator/🔁️codec/🦀️.rs — the `project` subcommand this file calls

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
