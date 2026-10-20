#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External carrier probe for `s.draw.drawing@1/✳️any`.
//
// Everything here MARSHALS and INVOKES; nothing here reads a carrier or computes a number. Every
// measurement comes out of `quick-xml` in the sibling `🦀️oracle-probe` crate. This script only
// stamps the ProbeReport envelope around what it reported.
//
// WHY A RUST CHILD PROCESS RATHER THAN AN INLINE JS PARSER. The approved `test-oracle` reader for
// this carrier is a Rust crate (`🔒️dependencies.json`: `quick-xml 0.42`, already registered for
// `semio@v1/drawing`, `svg@1.1/any`, `bcf`, `docx`, `xlsx` and `pptx`). The XML parsers vendored in
// `node_modules` (`fast-xml-parser`, `sax`, `saxes`, `@xmldom/xmldom`) are NOT approved oracle
// packages here. The child crate declares its own `[workspace]` and links no repository crate at all,
// so it builds and runs independently of the plugin (which does not currently compile).
//
// THE CARRIER DECIDES WHAT IS CHECKABLE. `drawing`'s own dwg/dxf/pdf/png export dialects are confirmed
// `serde_json::to_value`/`from_value` structural-coercion stubs — read in full, see this subset's own
// `🔣️oracle.json` rationale — so svg is the ONLY real carrier this probe is registered against. A
// probe handed anything but a `.svg` returns `unsupported`, never an empty `ok`.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts svg-structure --input <a.svg>
//   bun 📜️script.ts svg-compare   --input <expected.svg> --input <actual.svg>
//   bun 📜️script.ts gate-inputs   --out <dir>
//   bun 📜️script.ts fixtures      --out <dir>
//
// @see 📖️reader/🦀️.rs — the reader itself
// @see 🧰️framework/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
// @see ../../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🔬️probes/📜️script.ts
//      — the spawn/offline/agent-scoped-target pattern this file mirrors

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
