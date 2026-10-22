#!/usr/bin/env repo
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Language-neutral production mutation bridge for `s.stdio.semio@v1/✉️base`.
//
// The test platform must be able to ask ANY owner what production dispatch offers, without knowing
// which language that owner is written in. So the bridge contract is a PROCESS: one executable at
// `<owner>/🏭️bridge/📜️script.ts` that answers `list-mutations <artifact> <standard> <subset>` with a
// RuntimeMutationInventory on stdout. This subset's dispatch is Rust, so this script builds and runs
// the sibling binary. The envelope's dispatch enum wraps all eighteen arms, so that binary links the
// artifact crate itself — never `semio-s-plugin-stdio`, whose other artifacts would block this
// inventory for no reason.
//
//   bun 📜️script.ts list-mutations s.stdio.semio v1 base
//
// @see 🦀️.rs — the binary that reads the dispatch enum
// @see 🧰️framework/🔨️modules/🧪️test/🧬️schema/🔣️.json — RuntimeMutationInventory

//#endregion 🧲️Header

//#region 🔌️Adapters
import { runRepositoryMutationBridgeScriptV1 } from "../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🟦️.ts";
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🚪️Entry
/** 🚪️ `list-mutations` — delivers the production mutation inventory of this owner through its bridge binary. */
class ListMutationsScript extends BundleScript {
  async run(): Promise<void> {
    await runRepositoryMutationBridgeScriptV1(this.root, "semio-semio-v1-base-bridge");
  }
}
const router = new ScriptRouter(import.meta.dir).register("list-mutations", ListMutationsScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
//#endregion 🚪️Entry
