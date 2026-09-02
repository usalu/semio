// #region 🧲️Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

// #region 🗄️Configuration
// 🧪️ Root Vitest configuration — deliberately NOT a repository-wide aggregator.
//
// Until ticket 26/08/23/END-TO-END-TESTING-REFACTOR this file glob-discovered every emoji-named
// `🧪️vitest.config.ts` in the tree, loaded each one through Vite's own config loader, carried a
// hand-maintained `KNOWN_BROKEN_IN_AGGREGATOR` allowlist and explicitly pulled in a `compose`
// project. That made one runner responsible for discovering, ordering and excusing every test in
// the repository — a defensive aggregator rather than a discovery system, and it turned any single
// project's collection error into a blank listing for all of them.
//
// Test discovery is now owned by the testing domain
// (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`): cases are found from
// `**/🧪️tests/*/component.feature`, one cacheable Nx project is generated per case, and each
// project runs through its own native host. `compose/**` is excluded in that discovery library, not
// here and not by a CI path filter. No allowlist may turn a failure into a skip.
//
// What remains is the host-local config for the root itself: nothing to collect. Each package keeps
// its own `🧪️tests/🟦️.ts` vitest config and runs it through its own `📜️script.ts test` target. Note there is
// deliberately no `projects` key at all — Vitest rejects an explicitly EMPTY `projects: []` with
// "No projects were found", whereas omitting it leaves just this one root project collecting nothing.
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    // 🚫️ Without an explicit empty `include`, Vitest's implicit root project collects with its
    // DEFAULT glob (`**/*.{test,spec}.*`) across the whole repository, picking up Playwright specs
    // and ticket-folder scratch files. The root owns no tests of its own.
    include: [],
  },
});

// #endregion 🗄️Configuration
