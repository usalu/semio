import type { OwnedBuildPlugin, OwnedBuildConfig } from "../../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🛠️build-tooling/🟦️.ts";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const PRESENTATION_RENDERER_VITEST_START = "//#region 🧪️Tests";

/** ✂️ Drops vitest regions from animate present renderer in browser dev. */
export function animatePresentRendererVitestStripPlugin(animatePresentIndexPath: string): OwnedBuildPlugin {
  return {
    name: "animate-present-renderer-vitest-strip",
    enforce: "pre",
    load(id) {
      if (process.env.VITEST) return;
      const filePath = id.split("?")[0];
      if (filePath !== animatePresentIndexPath) return;
      const source = readFileSync(animatePresentIndexPath, "utf8");
      const testsStart = source.indexOf(PRESENTATION_RENDERER_VITEST_START);
      if (testsStart < 0) return source;
      return source.slice(0, testsStart);
    },
  };
}

/** 🎬️ R3F packages that must resolve once with {@link sceneHostPort} and drei controls. */
export const PLAYGROUND_SCENE_HOST_DEDUPE = ["@react-three/fiber", "@react-three/drei"] as const;

/** 🎬️ Vite aliases that pin R3F to a single node_modules entry (avoids duplicate Canvas stores). */
export function playgroundSceneHostResolveAliases(repoRoot: string): ReadonlyArray<{ readonly find: string | RegExp; readonly replacement: string }> {
  return [
    { find: /^@react-three\/fiber$/, replacement: resolve(repoRoot, "node_modules/@react-three/fiber/dist/react-three-fiber.esm.js") },
    { find: /^@react-three\/drei$/, replacement: resolve(repoRoot, "node_modules/@react-three/drei/index.js") },
  ];
}

/** 🎬️ CommonJS leaves an EXCLUDED R3F package still reaches — fiber → `scheduler`; drei → `stats.js`;
 * drei → `tunnel-rat` → (nested) `zustand` → `use-sync-external-store/shim{,/with-selector}.js`, and top-level
 * `zustand/esm/react.mjs` → the same shims. Vite serves an excluded package's files raw and rewrites their bare
 * imports to a prebundled copy only when that dependency IS in the optimizer — so without these entries a fresh
 * serve boots black on `The requested module '…/scheduler/index.js' (or …/with-selector.js) does not provide an
 * export named 'default'` (lanes started before the R3F exclusion landed kept working only because they
 * predated it — measured on :6086, 2026-09-16). Named per file where the package has no ESM entry at all,
 * exactly as Vite's own "exclude an ESM dependency, include its CJS descendants" rule asks; every other
 * package in the two graphs ships an ESM entry (`its-fine`, `suspend-react`, `tunnel-rat`, `zustand`,
 * `react-use-measure`, `stats-gl`, `three-stdlib`, `maath`, …) and is served raw on purpose. */
export const PLAYGROUND_SCENE_HOST_CJS_INCLUDE = ["scheduler", "stats.js", "use-sync-external-store/shim/index.js", "use-sync-external-store/shim/with-selector.js"] as const;

/** 📦️ ESM packages the EXCLUDED R3F graph reaches through a barrel and that the optimizer must bundle anyway. Every
 * drei control imports `three-stdlib`'s index, and an import from an excluded package is never scanned, so Vite served the
 * whole barrel as separate raw modules on every cold boot: 282 files, 15.7 MB — a third of the shell's boot requests after
 * drei's own barrel was cut (measured on :6580, ticket 26/09/23 F2). Prebundled, it is one module; `three` stays one instance
 * because the optimizer bundles `three` too and shares it between both. */
export const PLAYGROUND_SCENE_HOST_ESM_INCLUDE = ["three-stdlib"] as const;

/** 🎬️ `optimizeDeps` preset for configs that use {@link playgroundSceneHostResolveAliases}: never prebundle R3F — a `.vite/deps` fiber copy and the aliased ESM entry are two Canvas stores, and drei's `PerspectiveCamera` then throws outside Canvas — but DO prebundle the CJS shims R3F's excluded graph imports ({@link PLAYGROUND_SCENE_HOST_CJS_INCLUDE}). */
export function playgroundSceneHostOptimizeDeps(extra?: Pick<NonNullable<OwnedBuildConfig["optimizeDeps"]>, "include" | "exclude">): NonNullable<OwnedBuildConfig["optimizeDeps"]> {
  const include = ["three", ...PLAYGROUND_SCENE_HOST_CJS_INCLUDE, ...PLAYGROUND_SCENE_HOST_ESM_INCLUDE, ...(extra?.include ?? [])].filter((id) => !PLAYGROUND_SCENE_HOST_DEDUPE.includes(id as (typeof PLAYGROUND_SCENE_HOST_DEDUPE)[number]));
  const exclude = [...PLAYGROUND_SCENE_HOST_DEDUPE, ...(extra?.exclude ?? [])];
  return { include: [...new Set(include)], exclude: [...new Set(exclude)] };
}
