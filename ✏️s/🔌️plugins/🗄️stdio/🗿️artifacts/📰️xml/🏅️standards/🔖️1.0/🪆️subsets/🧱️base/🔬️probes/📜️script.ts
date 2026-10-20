#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External measurement probes for `s.stdio.xml@1.0/✳️base`'s READER oracle (`quick-xml-1-0-mutate-reader`).
//
// Everything here MARSHALS and READS; nothing here applies a mutation or predicts what one should
// produce. The actual XML decode is performed by the sibling standalone `quick-xml-oracle-codec`
// binary (`../🏭️generator/🔁️codec/📦️packages/🦀️rust`, depends on nothing but `quick-xml` 0.42) via
// its `project` subcommand — this file only shells out to it and performs the GATING structural
// comparison itself, over the SAME JSON shape this subset's own `semantic-xml-v1` comparison profile
// describes and this subset's own `../🔮️oracles/🦀️.rs::project_xml_1_0` independently produces
// (declaration, doctype, prolog, and the full element tree with attributes as an unordered
// name/value map) — no XML semantics computed here, only projection + compare.
//
// This subset ALSO carries `../🔮️oracles/🦀️.rs` registered `cross-semio-implementation` —
// that module COMPUTES what a mutation should produce. This probe suite is a DIFFERENT mechanism:
// the expected state is never computed, it is COMMITTED as the `after` half of a byte-reproducible
// fixture, and `quick-xml` reads BOTH sides independently. See `../🔮️oracles/🔣️.json`'s own
// `quick-xml-1-0-mutate-reader` oracle rationale.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts xml-import  --input <a.xml>
//   bun 📜️script.ts xml-project --input <a.xml>
//   bun 📜️script.ts xml-compare --input <expected.xml> --input <actual.xml>
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
