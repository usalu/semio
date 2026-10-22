#!/usr/bin/env repo
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External reading probes for `s.stdio.semio@v1/📑️document`.
//
// Everything here MARSHALS and INVOKES; nothing here computes a document and nothing here PREDICTS what
// a mutation ought to produce. Every value comes out of a third-party library reading committed bytes:
// `fflate` unzips the OPC container and `fast-xml-parser` decodes the WordprocessingML, `markdown-it`
// parses the CommonMark. The pipeline compares the emitted `measurements`; this file states facts about
// files and nothing else.
//
// The readers are deliberately DIFFERENT LIBRARIES from the writers in `../🏭️generator/📜️script.ts`
// (`jszip` + `@xmldom/xmldom` write docx, `mdast-util-to-markdown` writes md). A reading produced by the
// library that did the writing would confirm its own serializer instead of checking the bytes.
//
// The carrier decides what is checkable, which is why `document-property` can answer `unsupported`.
// docx flattens lists, so it cannot witness `list-ordered`; CommonMark drops named styles, so it cannot
// witness `paragraph-style` or `style-table`; and NEITHER serializes the id-keyed embedded image store,
// so `image-store` is unsupported everywhere. An empty reading returned as `ok` would let such a
// mutation pass against evidence that was never there.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts document-read      --input <a.docx|a.md>
//   bun 📜️script.ts document-property  --property <p> --input <before.ext> --input <after.ext>
//   bun 📜️script.ts document-compare   --input <before.ext> --input <after.ext>
//   bun 📜️script.ts carrier-agreement  --input <a.docx> --input <a.md>
//
// @see 🧰️framework/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
// @see ../../🔺️mesh/🔬️probes/📜️script.ts — the pilot this file mirrors in report contract and CLI shape
// @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️document-subset-oracle.md

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
