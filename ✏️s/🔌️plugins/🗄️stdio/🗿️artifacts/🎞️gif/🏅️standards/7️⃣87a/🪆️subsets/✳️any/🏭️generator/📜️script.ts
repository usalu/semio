#!/usr/bin/env repo
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.gif@87a/✳️any`.
//
// 🚫️ NOT `gif` 0.13, and that is the whole point. That crate cannot write a GIF87a at all: its encoder
// hardcodes `b"GIF89a"` (`src/encoder.rs:340`) and emits a Graphic Control Extension — an 89a-only
// block — for EVERY frame (`src/encoder.rs:178`, unconditional, despite its own doc comment saying
// "if necessary"). The generator this replaces patched the signature onto that output, which produced a
// file declaring `GIF87a` while carrying a GCE: a document that contradicts itself.
//
// Pillow writes conformant single-image GIF87a natively — correct signature, no GCE, background index
// settable — verified before this was built. Multi-image 87a is then ASSEMBLED from Pillow's own image
// blocks: the container header from one file, the image blocks from several, the trailer. Nothing is
// hand-encoded; the LZW data is Pillow's. The assembly asserts the source headers are byte-identical
// first, because differing global palettes would silently re-attribute every frame's colours.
//
// The JUDGE is `gif` 0.13, which reads 87a completely — see `../🔬️probes/📖️reader/📦️packages/🦀️rust`.
//
//   bun 📜️script.ts generate [--out <dir>]   # writes every fixture pair
//   bun 📜️script.ts manifests                 # prints the testEvidence entries
//
// @see ../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️gif-87a-conformance-and-writer-limits.md

//#endregion 🧲️Header

//#region 🔌️Adapters
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptProgram } from "./🟦️.ts";

/** 🚪️ `aspect` — runs the program of this script. */
class AspectScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🧭️ `aspect-manifests` — runs the program of this script. */
class AspectManifestsScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🏭️ `generate` — runs the program of this script. */
class GenerateScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🧪️ `manifests` — runs the program of this script. */
class ManifestsScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
const router = new ScriptRouter(import.meta.dir).register("aspect", AspectScript).register("aspect-manifests", AspectManifestsScript).register("generate", GenerateScript).register("manifests", ManifestsScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, argv: ["aspect"] }));
