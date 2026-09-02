#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Language-neutral production mutation bridge for `s.stdio.semio@v1/✳️brep`.
//
// The test platform must be able to ask ANY owner what production dispatch offers, without knowing
// which language that owner is written in. So the bridge contract is a PROCESS: one executable at
// `<owner>/🏭️bridge/📜️script.ts` that answers `list-mutations <artifact> <standard> <subset>` with a
// RuntimeMutationInventory on stdout. This subset's dispatch is Rust, so this script builds and runs
// the sibling binary. That binary compiles the PRODUCTION source files by `#[path]` — the same bytes
// the plugin's own glue.rs includes — rather than linking `semio-s-plugin-stdio`, so one unrelated
// artifact mid-refactor cannot block this subset's inventory; a TypeScript-owned subset would answer inline and the platform could not tell.
//
//   bun 📜️script.ts list-mutations s.stdio.semio v1 brep
//
// @see 🦀️.rs — the binary that reads the dispatch enum
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — RuntimeMutationInventory

//#endregion 🧲️Header

//#region 🔌️Adapters
import { spawnSync } from "node:child_process";
import { join } from "node:path";
//#endregion 🔌️Adapters

//#region 🚪️Entry
const BRIDGE_VERSION = 1;

function main(argv: readonly string[]): number {
  const [command = "", artifact = "s.stdio.semio", standard = "v1", subset = "brep"] = argv;
  if (command !== "list-mutations") {
    console.error(`[bridge] unknown command ${JSON.stringify(command)} — expected list-mutations <artifact> <standard> <subset>`);
    return 2;
  }
  // 🏭️`--offline` and an agent-scoped target directory: the bridge runs inside a test sweep alongside
  // peer sessions, and a shared target directory is the single biggest source of lock contention here.
  const target = process.env.CARGO_TARGET_DIR ?? join(process.env.SEMIO_AGENT_CACHE ?? join(import.meta.dir, "target"), "bridge");
  const built = spawnSync("cargo", ["run", "--quiet", "--offline", "--bin", "semio-semio-v1-brep-bridge", "--", command, artifact, standard, subset], {
    cwd: import.meta.dir,
    encoding: "utf8",
    env: { ...process.env, CARGO_TARGET_DIR: target },
  });
  if (built.status !== 0) {
    // 🚫️A bridge that cannot run must SAY SO and exit non-zero. Emitting a plausible-looking inventory
    // from the manifest would make the equality gate compare the manifest with itself, which is the
    // one thing Protocol v2's runtime half exists to prevent.
    console.error(`[bridge] cargo exited ${built.status}: ${(built.stderr ?? "").trim().split("\n").slice(-6).join("\n")}`);
    return 1;
  }
  process.stdout.write(built.stdout);
  return 0;
}

if (import.meta.main) process.exit(main(process.argv.slice(2)));
export { BRIDGE_VERSION };
//#endregion 🚪️Entry
