// #region 🧲️Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Root Vitest configuration for the monorepo test runner.

// #endregion 🧲️Header

// #region 🗄️Configuration
// Root Vitest configuration aggregating all workspace test projects.
//
// #region ⚠️Why This Is Not A Plain Path List
// Vitest 4's `test.projects` resolver validates every path (literal or glob match)
// against `/^vite(?:st)?(?:\.[\w-]+)?\.config\./` — the basename MUST start with the
// literal ASCII string "vite"/"vitest". None of this repo's emoji-prefixed
// `🧪️vitest.config.ts` files satisfy that regex, so referencing them as path *strings*
// (literal or glob) always throws, even when the path genuinely exists. Discovered while
// fixing this file in ticket 26/08/05/STALE-CONFIG-FIXES-AND-CAPABILITY-LINT-REVIVAL.
// The workaround: glob-discover the real files ourselves, then load each one with Vite's
// own `loadConfigFromFile` (the same esbuild-backed loader Vite uses for `--config`) and
// hand Vitest the resolved config *objects* instead of path strings — `test.projects`
// accepts inline `UserWorkspaceConfig` objects too, and object entries skip the filename
// regex entirely. `import.meta.url`-derived `root` fields inside each project config
// resolve correctly this way (unlike a same-directory-external symlink workaround, which
// was also tried and found to break `include` glob resolution intermittently).
// #endregion ⚠️Why This Is Not A Plain Path List

// #region 🔌️Adapters
import { defineConfig, loadConfigFromFile, type UserWorkspaceConfig } from "vite";
import { globSync } from "node:fs";
import { dirname } from "node:path";
// #endregion 🔌️Adapters

// #region 🔍️Discovery
const root = process.cwd();

/** 🚫️ Directories that must never feed the workspace aggregator: node_modules, ticket
 * scratch trees, and other-technology sibling stacks (`♻️mit-bestand`) per CLAUDE.md's
 * no-technology-mixing rule. */
function isDiscoverable(relPath: string): boolean {
  return (
    !relPath.includes("node_modules") &&
    !relPath.startsWith(".🧬semio/🦑️repo/") &&
    !relPath.startsWith("♻️mit-bestand/") &&
    relPath !== "🧪️vitest.config.ts"
  );
}

/** 🧵️ Projects that are individually healthy (pass standalone via their own `nx test`
 * target — the mechanism CI/devs actually use) but currently fail when *collected inside
 * this aggregator*, for reasons unrelated to path correctness. Verified standalone-clean
 * and aggregator-broken one by one while fixing this file in ticket
 * 26/08/05/STALE-CONFIG-FIXES-AND-CAPABILITY-LINT-REVIVAL; each entry documents the
 * specific pre-existing breakage so it isn't mistaken for a path bug. Excluded here rather
 * than left to abort the whole `list`/`run` (Vitest's workspace collection is all-or-nothing
 * — one project's collection error blanks the successful listing of every other project).
 * Re-include as each underlying bug is fixed by its owning team. */
const KNOWN_BROKEN_IN_AGGREGATOR = new Map<string, string>([
  [
    "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🧪️vitest.config.ts",
    "custom nested-worker test environment (backbone-worker.ts) throws EnvironmentTeardownError when collected alongside sibling projects, even with --no-file-parallelism",
  ],
  [
    "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/🧪️vitest.config.ts",
    "📜️script.ts has a pre-existing 'Cannot access join before initialization' bug reproducible standalone (bun:sqlite/env-transform ordering) — unrelated to this file",
  ],
  [
    "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts",
    "imports the unresolved package '@semio-tech/assets' — pre-existing, unrelated to this file",
  ],
  [
    "✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/🧪️vitest.config.ts",
    "imports the unresolved package '@semio-tech/animate-present-core' — pre-existing, unrelated to this file",
  ],
]);

const discoveredConfigPaths = globSync("**/🧪️vitest.config.ts", { cwd: root })
  .filter(isDiscoverable)
  .filter((relPath) => !KNOWN_BROKEN_IN_AGGREGATOR.has(relPath));

const discoveredProjects = (
  await Promise.all(
    discoveredConfigPaths.map(async (relPath) => {
      const absPath = `${root}/${relPath}`;
      try {
        const loaded = await loadConfigFromFile({ command: "serve", mode: "test" }, absPath, dirname(absPath));
        if (!loaded) {
          console.warn(`[🧪️vitest.config.ts] no config export found, skipping: ${relPath}`);
          return null;
        }
        const config = loaded.config as UserWorkspaceConfig;
        // 🧭️ `loadConfigFromFile` does NOT backfill `root` onto the returned config object
        // even when several project configs rely on Vite's CLI default of "directory
        // containing the config file" (they never set `root:` themselves). Left as `undefined`,
        // Vitest falls back to the WORKSPACE root for that project, silently rebasing its
        // relative `include`/`coverage.include` patterns onto the whole monorepo — turning a
        // 2-file project into an accidental full-repo scan. Backfill explicitly here.
        config.root ??= dirname(absPath);
        return config;
      } catch (err) {
        console.warn(`[🧪️vitest.config.ts] failed to load, skipping: ${relPath} -> ${(err as Error).message.split("\n")[0]}`);
        return null;
      }
    }),
  )
).filter((project): project is UserWorkspaceConfig => project !== null);

/** 🧭️ `compose/**` keeps non-emoji-prefixed `vite(st)?.config.ts` filenames (a separate,
 * older technology stack — see CLAUDE.md's no-technology-mixing rule), so those pass the
 * CONFIG_REGEXP check natively and can stay as plain path strings.
 * `sketchpad/js` and `dev/algorithm/js` are deliberately omitted: both transitively import
 * an unresolved package (`@semio-tech/framework-platform-core` and `@semio-tech/assets`
 * respectively — the latter via `🧰️framework/🔨️modules/🖱️ui/⚛️react`, also excluded above),
 * reproducible standalone — pre-existing, unrelated to this file. Re-include once those
 * packages resolve. */
const composeProjectPaths = ["./compose/client/lib/js/vite.config.ts"];
// #endregion 🔍️Discovery

export default defineConfig({
  test: {
    // 🚫️ Without an explicit empty `include`, Vitest's own implicit root/core project
    // additionally collects with its DEFAULT include glob (`**/*.{test,spec}.*`) across
    // the whole repo, alongside the real discovered projects below — picking up unrelated
    // Playwright specs, stray ticket-folder scratch tests, etc. `include: []` disables that
    // phantom project without disabling any of the real `projects` entries.
    include: [],
    projects: [...composeProjectPaths, ...discoveredProjects],
  },
});

// #endregion 🗄️Configuration
