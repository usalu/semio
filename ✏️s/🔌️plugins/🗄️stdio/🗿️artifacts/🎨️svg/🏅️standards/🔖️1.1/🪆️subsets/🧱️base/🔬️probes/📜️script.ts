#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External measurement probes for `s.stdio.svg@1.1/🧱️base`'s READER oracle
// (`quick-xml-svg-1-1-mutate-reader`).
//
// Everything here MARSHALS and READS; nothing here applies a mutation or predicts what one
// should produce. The actual XML decode is performed by the sibling standalone
// `quick-xml-svg-codec` binary (`../🏭️generator/🔁️codec/📦️packages/🦀️rust`, depends on nothing but
// `quick-xml` 0.42) via its `project` subcommand — this file only shells out to it, sorts each
// element's attributes by name (per this oracle's own `svg-1-1-quick-xml-reader-v1`
// comparisonProfile — SVG attribute order is real writer freedom, never source-order-significant)
// and performs the GATING structural comparison itself. No SVG semantics computed here: `viewBox`
// and `transform` are compared as OPAQUE ATTRIBUTE STRINGS, exactly like every other attribute —
// `quick-xml` itself has no notion of SVG geometry grammar, so this reader never decomposes them
// into numbers the way this subset's own `🦀️oracle.rs` (a computed, NOT a read,
// cross-semio-implementation oracle) does. A `viewBox`/`transform` value that changed is still a
// string that differs, which IS a witnessed, honest reader comparison — just not a semantic one.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts svg-import  --input <a.svg>
//   bun 📜️script.ts svg-project --input <a.svg>
//   bun 📜️script.ts svg-compare --input <expected.svg> --input <actual.svg>
//
// @see 🧰️framework/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
// @see ../../../../../📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🔬️probes/📜️script.ts — the sibling
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
