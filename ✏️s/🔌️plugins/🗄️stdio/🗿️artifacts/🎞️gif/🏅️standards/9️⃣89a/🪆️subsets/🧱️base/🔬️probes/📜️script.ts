#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ The LOGICAL SCREEN DESCRIPTOR reader, shared by both GIF versions.
//
// `set-pixel-aspect-ratio` was the last kind in this artifact, and it was recorded uncarried after
// every available LIBRARY was checked — correctly, as far as that went: `gif` 0.13.3 (`encoder.rs:345`)
// and 0.14.2 (`encoder.rs:401`) each write a hardcoded `0u8` for the aspect byte and NEITHER has any
// parse path for it, and Pillow surfaces only `background` and `version` after a round trip.
//
// What that survey never covered is that its INVENTORY was scoped to libraries — cargo, npm and PyPI —
// and never to installed command-line tools, even though Protocol v2 lists `third-party-cli` as a
// qualifying oracle kind. giflib's `giftext` prints the descriptor including `Aspect = N`, and its
// `gifbuild` writes a GIF from a text description carrying `pixel aspect byte N`. Reader and writer
// both third-party, both from giflib, and `gifbuild` is byte-deterministic across runs.
//
//   bun 📜️script.ts gif-screen-project --input <a.gif>
//   bun 📜️script.ts gif-screen-compare --input <a.gif> --input <b.gif>

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
