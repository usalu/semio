#!/usr/bin/env bun
import { buildModeEnvironment, readBuildMode } from "../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { buildViteArtifact } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts";
import { prefetchPlayMapTiles } from "./🗺️map-tiles/🟦️.ts";
import { PLAY_HOST, playRuntimeComponentIds } from "../🧩️runtime/🟦️.ts";
import { bindPlayRuntimeAssetsV1, playModuleCatalogFindingsV1, publishPlayPages } from "./📄pages/🟦️.ts";

/** 🛑️ Runs one cancellable step: SIGINT and SIGTERM abort its signal and set the conventional exit code. */
async function cancellable(run: (signal: AbortSignal) => Promise<void>): Promise<void> {
  const controller = new AbortController(), cancel = (): void => { controller.abort(Error("Play release step cancelled")); process.exitCode = 130; };
  process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
  try { await run(controller.signal); } finally { process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
}

/** 🚀️ `build`: the whole play release as ONE build driver — the components of every pane's closure in one Cargo pass per workspace, each streamed through
 * describe, transpile, `wasm-opt` and its module probe as it links; the descriptor emitter, fonts, browser support, map tiles and the wasm-bindgen engines as
 * jobs under the same jobserver (the engines in the `wasm-release` lane, before the component passes); the Vite bundle as soon as its engines exist; and the
 * pages assembly once every module is published. The pipeline is declared in this project's `build` target metadata.
 * @see ../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts */
class BuildScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("The play build accepts no arguments; SEMIO_RELEASE_GENERATION=fresh selects a cold generation");
    const root = resolve(this.root, "../.."), pipeline = JSON.parse(readFileSync(join(root, "📋️project.json"), "utf8")).targets.build.metadata.semio.release;
    const [{ releaseGenerationKindV1, runReleaseDriverV1 }, { cargoDirectories, releasePipelinePlanV1 }, { repoCacheDirectory }, { componentDriverStagesV1 }] = await Promise.all([
      import("../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts"),
      import("../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts"),
      import("../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts"),
      import("../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts"),
    ]);
    const { plan, jobEnvironment } = releasePipelinePlanV1(this.repoRoot, pipeline, new Set(playRuntimeComponentIds()));
    await cancellable(async (signal) => {
      await runReleaseDriverV1({ plan, cwd: this.repoRoot, cacheRoot: repoCacheDirectory(this.repoRoot), shared: cargoDirectories(this.repoRoot), generation: releaseGenerationKindV1(process.env), leaseDirectory: repoCacheDirectory(this.repoRoot, "agents", "resource-leases"), environment: process.env, signal: AbortSignal.any([signal, this.invocation.control.signal]), stages: componentDriverStagesV1(this.repoRoot), jobEnvironment });
    });
  }
}

/** 🗺️ `map-tiles`: prefetches the map tiles the bundle embeds (network-bound). */
class MapTilesScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("map-tiles accepts no arguments");
    await cancellable((signal) => prefetchPlayMapTiles(this.repoRoot, signal));
  }
}

/** 🌐️ `bundle`: the Vite site bundle into `dist/site`. It reads the generated catalog and the wasm-bindgen engines but no compiled component, so it runs
 * while the components still compile; the runtime modules are bound at pages assembly. */
class BundleSiteScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("bundle accepts no arguments");
    const root = resolve(this.root, "../..");
    await cancellable((signal) => buildViteArtifact({ root, workspace: this.repoRoot, config: join(root, "🏗️builder/🌐️vite/🟦️.ts"), output: join(root, "dist", readBuildMode(process.env.SEMIO_BUILD_MODE), "site"), owner: "play:site", signal, environment: { SEMIO_BUILD_MODE: readBuildMode(process.env.SEMIO_BUILD_MODE), SEMIO_RENDERER: "react", GIS_MAP_TILE_SERVE_MODE: "bundle" }, ...(process.env.SEMIO_TICKET_DIR ? { temporaryRoot: join(process.env.SEMIO_TICKET_DIR, "🗑️generated") } : {}) }));
  }
}

/** 📄️ `pages`: binds the runtime modules, browser support and fonts into the bundled site with the module catalog, then splits it into the CDN pages. */
class PagesScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("pages accepts no arguments");
    const root = resolve(this.root, "../..");
    await cancellable(async (signal) => {
      const catalog = await bindPlayRuntimeAssetsV1(join(root, "dist", readBuildMode(process.env.SEMIO_BUILD_MODE), "site"), this.repoRoot, signal);
      console.log(`Play module catalog: ${catalog.modules.length} module(s) bound to their receipts`);
      publishPlayPages(join(root, "dist", readBuildMode(process.env.SEMIO_BUILD_MODE), "site"), join(root, "dist", readBuildMode(process.env.SEMIO_BUILD_MODE), "pages"), PLAY_HOST);
    });
  }
}

/** ⚖️ `verify`: the release gate over the published pages — the generated catalog equals a refusing render of the committed descriptors, and every module
 * page file is exactly the one the module catalog binds. */
class VerifyScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("verify accepts no arguments");
    const root = resolve(this.root, "../.."), registry = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated");
    const { renderCatalogFiles } = await import("../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts");
    const findings = [...Object.entries(renderCatalogFiles(this.repoRoot, undefined, "refuse").files).filter(([name, content]) => !existsSync(join(registry, name)) || readFileSync(join(registry, name), "utf8") !== content).map(([name]) => `Generated catalog ${name} differs from the committed descriptors; run bun nx run @semio-tech/plugin-registry:generate`), ...playModuleCatalogFindingsV1(join(root, "dist", readBuildMode(process.env.SEMIO_BUILD_MODE), "pages"))];
    for (const finding of findings) console.error(finding);
    if (findings.length) throw new Error(`Play release verification failed: ${findings.length} finding(s)`);
    console.log("Play release verified: generated catalog and module catalog bind the published pages");
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("map-tiles", MapTilesScript).register("bundle", BundleSiteScript).register("pages", PagesScript).register("verify", VerifyScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (router).run(process.argv.slice(2), original));
