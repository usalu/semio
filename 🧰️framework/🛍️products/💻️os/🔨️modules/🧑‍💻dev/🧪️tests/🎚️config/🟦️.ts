/** 🧪️ Vitest config for `@semio-tech/framework-os-dev`. `includeSource` enables in-source
 * `import.meta.vitest` blocks inside `📜️script.ts` itself (the task-router entry, not a `js/index.ts`
 * — this bundle root has no separate library entry point) for pure helper logic (marker parsing,
 * built-module scanning) that doesn't need a live cargo/vite process. */
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { stripExecutableShebang } from "../../🧹️executable-source/🟦️.ts";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");

/** 🎚️`resolveTestLevel` (`🦑️repo/📚️library/📦️packages/🟦️typescript/🟦️.ts`) exports `testLevelAtLeast` and
 * publishes the active level in `SEMIO_TEST_LEVEL` before spawning Vitest. This config reads the env
 * variable rather than importing that module: Vite esbuild-bundles and executes the config's whole import
 * graph on every run, and pulling the repo tooling library in for one predicate cost more startup than the
 * `quick` level's entire wall-clock budget allows. */
const testLevelAtLeast = (level: "long" | "exhaustive"): boolean => (level === "long" ? ["long", "exhaustive"] : ["exhaustive"]).includes(process.env.SEMIO_TEST_LEVEL ?? "");

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");

/** 🎚️In-source WIT mapping suites whose every case spawns a full strict-TypeScript program check (~60 s each,
 * measured 2026-09-05) — far past the `quick` wall-clock budget, so they join the run from `long` upwards.
 * Their owning modules live under `🔌️plugin/`, not in this bundle, so the gate belongs here rather than in
 * a per-case `atTestLevel` inside runtime source. */
const WIT_MAPPING_IN_SOURCE = ["../../../🔌️plugin/📤️return/🟦️.ts", "../../../🔌️plugin/📥️poll/🏘️composition/🟦️.ts"];
const inSource = testLevelAtLeast("long") ? WIT_MAPPING_IN_SOURCE : [];

export default defineConfig({
  root: testRoot,
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
    root: testRoot,
    name: "@semio-tech/framework-os-dev",
    /** 🎚️Every suite this project collects is Node-side build tooling: the staging, staging-root and
     * config files reach `node:sqlite` / `bun:sqlite` lease stores, Bun's transpiler (`registryStaticImports`),
     * `node:vm` bridge harnesses and `fileURLToPath`, and NONE of them touches a DOM. The Canvas PNG pixel
     * parity module (`⚖️parity/🖼️pixels/🟦️.ts`) that once justified jsdom at `long` declares no
     * `describe`/`it` at all — it runs inside a real browser page — and is in neither `include` nor
     * `includeSource`, so the level-gated jsdom only ever told Vite to resolve this graph as a CLIENT one,
     * where a runtime builtin is refused outright ("Cannot bundle built-in module") and the runner's own
     * `Bun`/`URL` globals are replaced. That refusal failed the whole 88-law staging FILE and six of its laws. */
    environment: "node",
    // 🩹️ In-source files belong only in `includeSource`; listing them in BOTH keys made Vitest
    // collect them twice. Dedicated regression files remain ordinary `include` entries.
    include: ["../../🧪️tests/🧹️config/🟦️.ts", "../../🧪️tests/🔌️staging-root/🟦️.ts", "../../🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts", "../../🧪️tests/🚀️local-hub/🟦️.ts"],
    includeSource: inSource,
    coverage: { include: ["../../**/🟦️.ts", ...WIT_MAPPING_IN_SOURCE] },
  },
});
