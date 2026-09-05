/** 🧪️ Vitest config for `@semio-tech/framework-os-dev`. `includeSource` enables in-source
 * `import.meta.vitest` blocks inside `📜️script.ts` itself (the task-router entry, not a `js/index.ts`
 * — this bundle root has no separate library entry point) for pure helper logic (marker parsing,
 * built-module scanning) that doesn't need a live cargo/vite process. */
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { stripExecutableShebang } from "./🧹️executable-source/🟦️.ts";

/** 🎚️`resolveTestLevel` (`🦑️repo/📚️library/📦️packages/🟦️typescript/🟦️.ts`) exports `testLevelAtLeast` and
 * publishes the active level in `SEMIO_TEST_LEVEL` before spawning Vitest. This config reads the env
 * variable rather than importing that module: Vite esbuild-bundles and executes the config's whole import
 * graph on every run, and pulling the repo tooling library in for one predicate cost more startup than the
 * `quick` level's entire wall-clock budget allows. */
const testLevelAtLeast = (level: "long" | "exhaustive"): boolean => (level === "long" ? ["long", "exhaustive"] : ["exhaustive"]).includes(process.env.SEMIO_TEST_LEVEL ?? "");

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");

/** 🎚️In-source WIT mapping suites whose every case spawns a full strict-TypeScript program check (~60 s each,
 * measured 2026-09-05) — far past the `quick` wall-clock budget, so they join the run from `long` upwards.
 * Their owning modules live under `🔌️plugin/`, not in this bundle, so the gate belongs here rather than in
 * a per-case `atTestLevel` inside runtime source. */
const WIT_MAPPING_IN_SOURCE = ["../../../🔌️plugin/📤️return/🟦️.ts", "../../../🔌️plugin/📥️poll/🏘️composition/🟦️.ts"];
const inSource = ["📜️script.ts", ...(testLevelAtLeast("long") ? WIT_MAPPING_IN_SOURCE : [])];

export default defineConfig({
  root,
  plugins: [
    {
      name: "semio-strip-executable-shebang",
      enforce: "pre",
      transform(source, id) {
        if (!/(?:^|[\\/])📜️script\.ts(?:[?#].*)?$/u.test(id) || !source.startsWith("#!")) return null;
        return { code: stripExecutableShebang(source), map: null };
      },
    },
  ],
  test: {
    name: "@semio-tech/framework-os-dev",
    /** 🎚️Only the level-gated cases touch a DOM (Canvas PNG pixel parity); the `quick` subset is pure
     * Node helper logic, and paying jsdom's ~7 s environment setup there costs a quarter of the level's
     * whole wall-clock budget. */
    environment: testLevelAtLeast("long") ? "jsdom" : "node",
    // 🩹️ In-source files belong only in `includeSource`; listing them in BOTH keys made Vitest
    // collect them twice. Dedicated regression files remain ordinary `include` entries.
    include: ["🧹️config.test.ts"],
    includeSource: inSource,
    coverage: { include: ["📜️script.ts", ...WIT_MAPPING_IN_SOURCE] },
  },
});
