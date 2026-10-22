#!/usr/bin/env repo
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External measurement probes for `s.stdio.dxf@r12/📰️header`.
//
// Everything here MARSHALS and READS; nothing here applies a mutation or predicts what one should
// produce. The actual DXF decode is performed by the sibling standalone engine binary
// (`../🏭️generator/🧫️fixtures/📦️packages/🦀️rust`, depends on nothing but `dxf` 0.6) via its `project` subcommand — this
// file only shells out to it and performs the GATING structural comparison itself against
// `semantic-dxf-r12-v1`'s own rules (LAYER/STYLE/LTYPE table rows are NAME-keyed; BLOCKS and
// ENTITIES — including a block's own nested entity list — are ORDER-significant, matching the real
// production dispatch's own index-addressed `DxfBlocksDiff`/`DxfEntitiesDiff`, read directly from
// `../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️dxf-r12-any-reader-oracle-retrofit.md`'s
// own citations of `🧬️schema/🔺️diff/🦀️.rs`) — no DXF semantics computed here beyond that
// comparison, only projection + compare. This subset's document has no large opaque binary
// payloads (unlike AVI's movi chunks), so unlike that sibling probe suite there is nothing here to
// hash: the engine's own typed projection is already the full comparable shape.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts dxf-import  --input <a.dxf>
//   bun 📜️script.ts dxf-project --input <a.dxf>
//   bun 📜️script.ts dxf-compare --input <expected.dxf> --input <actual.dxf>
//
// @see 🧰️framework/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
// @see ../../../../💬️avi/🏅️standards/🔖️1.0/🪆️subsets/📰️header/🔬️probes/📜️script.ts — the sibling
//      probe suite this file's CLI/dispatch/compare shape is mirrored from
// @see ../🏭️generator/🧫️fixtures/🦀️.rs — the `project` subcommand this file calls

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
