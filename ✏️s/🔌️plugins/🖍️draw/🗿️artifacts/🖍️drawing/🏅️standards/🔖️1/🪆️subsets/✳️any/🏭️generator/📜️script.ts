#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for the three LAYER-METADATA kinds of `s.draw.drawing@1/✳️any`.
//
// This subset already carries `quick-xml-drawing-1-mutate`, which judges the SVG export. SVG has no
// representation for a layer's `locked` flag, its `blendMode`, or its authoring `name` — editor
// metadata that never reaches the rendered document — so those three kinds were `-uncarried` against
// it. They ride this subset's JSON carrier instead, where `DrawingSnapshot::layers` is an INLINE
// `Vec<DrawingLayerNode>` and every one of the three is a carrier-level fact.
//
// `🧩️json` depends on `json` (json-rust) and nothing else — never `serde_json`, which the draw plugin
// itself links: it applies each mutation as an edit to the carrier and reads it back through the same
// third-party library. It refuses to write a pair whose
// projection does not move, so a no-op cannot be committed as a fixture that would pass forever.
//
//   bun 📜️script.ts generate [--out <dir>]   # builds the engine and writes the reviewed fixture pairs
//   bun 📜️script.ts manifests                 # refreshes digests and provenance of this oracle's fixture manifests
//
// @see ../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️fem-carrier-reader-retrofit.md

//#endregion 🧲️Header

//#region 🔌️Adapters
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptProgram } from "./🟦️.ts";

/** 🚪️ `generate` — runs the program of this script. */
class GenerateScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🧭️ `manifests` — runs the program of this script. */
class ManifestsScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
const router = new ScriptRouter(import.meta.dir).register("generate", GenerateScript).register("manifests", ManifestsScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
