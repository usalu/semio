#!/usr/bin/env bun
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
//#endregion 🔌️Adapters

//#region 🚪️Entry
const BRIDGE_VERSION=1;
if(import.meta.main)await runRepositoryMutationBridgeScriptV1(import.meta.dir,"semio-semio-v1-base-bridge");
export {BRIDGE_VERSION};
//#endregion 🚪️Entry
