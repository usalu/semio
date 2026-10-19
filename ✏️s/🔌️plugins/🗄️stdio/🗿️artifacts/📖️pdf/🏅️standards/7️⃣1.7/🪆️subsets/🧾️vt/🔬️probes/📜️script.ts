#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔐️ The ENCRYPTION reader for every `pdf@1.7` conformance subset. Shared deliberately: the question
// — does this document carry a standard security handler — is identical across `vt`, `a`, `e` and `x`,
// and the framework lets a probe's `command` point at any script (brep's manifold probe already points
// at step's).
//
// Why a second reader at all: `lopdf` 0.44, which judges every other kind in these subsets, DECRYPTS
// transparently on load with the empty user password and then reports `is_encrypted() == false` on a
// genuinely encrypted document — measured. It also cannot WRITE an encryption dictionary: its writer
// demands the encryption state a real decryption would have recorded, so a synthetic `/Encrypt` can be
// neither written nor read back. Both halves of the encryption kinds were therefore invisible to it.
//
// `pypdf` 6.14 both encrypts (`PdfWriter.encrypt`) and reports it (`PdfReader.is_encrypted`), and its
// output is byte-deterministic across runs — checked three times before this was relied on.
//
//   bun 📜️script.ts pdf-encryption-project --input <a.pdf>
//   bun 📜️script.ts pdf-encryption-compare --input <a.pdf> --input <b.pdf>

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
