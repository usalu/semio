#!/usr/bin/env repo
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External carrier probes for `s.stdio.semio@v1/🖊️drawing`.
//
// Everything here MARSHALS and INVOKES; nothing here reads a carrier or computes a number. Every
// measurement comes out of a third-party library in the sibling `🦀️oracle-probe` crate — `quick-xml`
// parses the SVG, the IxMilia `dxf` crate parses the DXF, `lopdf` parses the PDF. This script only
// stamps the ProbeReport envelope around what they reported.
//
// WHY A RUST CHILD PROCESS RATHER THAN AN INLINE JS PARSER. The approved `test-oracle` readers for
// these three carriers are Rust crates (`🔒️dependencies.json`: `quick-xml 0.42`, `dxf 0.6`,
// `lopdf 0.44`). The XML and PDF parsers vendored in `node_modules` — `fast-xml-parser`, `sax`,
// `saxes`, `@xmldom/xmldom`, `pdfjs-dist` — are NOT approved oracle packages, and `pdfjs-dist` is
// production-reachable through `react-pdf`, so reading a carrier with it would put production code on
// the measurement path. The child crate declares its own `[workspace]` and links no repository crate
// at all, so it builds and runs independently of the plugin.
//
// THE CARRIER DECIDES WHAT IS CHECKABLE. A probe handed a carrier that cannot encode the property
// asked about returns `unsupported`, never an empty `ok`: DXF has no paint field for a path and PDF
// carries text alone, so an empty result read as ok would let a recolour pass against a file that
// never carried a colour.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts svg-structure  --input <a.svg>
//   bun 📜️script.ts dxf-entities   --input <a.dxf>
//   bun 📜️script.ts pdf-text       --input <a.pdf>
//   bun 📜️script.ts svg-compare    --input <expected.svg> --input <actual.svg>
//   bun 📜️script.ts style-compare  --input <expected.svg> --input <actual.svg>
//   bun 📜️script.ts dxf-compare    --input <expected.dxf> --input <actual.dxf>
//   bun 📜️script.ts pdf-compare    --input <expected.pdf> --input <actual.pdf>
//   bun 📜️script.ts gate-inputs    --out <dir>
//
// @see 📖️reader/🦀️.rs — the readers themselves
// @see 🧰️framework/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
// @see ../../../../../../📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/🏭️bridge/📜️script.ts — the
//      spawn/offline/agent-scoped-target pattern this file mirrors

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
