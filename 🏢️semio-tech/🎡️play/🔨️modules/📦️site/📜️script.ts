#!/usr/bin/env bun
import {frameworkOsEngineDeliveriesV1} from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🕸️engines/🟦️.ts";
import {flowBrowserPackageRequestV1} from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/📦️publication/📋️contract/🟦️.ts";
import {repositoryCachePartitionRootDataV1} from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📂️paths/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { buildModeEnvironment, readBuildMode } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { buildViteArtifact } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts";
import { prefetchPlayMapTiles } from "./🗺️map-tiles/🟦️.ts";
import { PLAY_HOST, playRuntimeComponentIds } from "../🧩️runtime/🟦️.ts";
import {playModuleCatalogFindingsV1} from "./📄pages/🟦️.ts";
import {publishPlayPageCohortV1,withPublishedPlayPagesV1} from "./📄pages/📦️publication/🟦️.ts";

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
 * @see ../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts
 * @see ../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts */
class BuildScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("The play build accepts no arguments; SEMIO_NATIVE_BUILD_GENERATION=fresh selects a cold generation");
    const root = resolve(this.root, "../.."), pipeline = JSON.parse(readFileSync(join(root, "📋️project.json"), "utf8")).targets.build.metadata.semio.release;
    const [{ buildReleasePipelineV1 }, { repositoryCargoNetworkV1 }] = await Promise.all([
      import("../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts"),
      import("../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts"),
    ]);
    await cancellable(async () => {
      await buildReleasePipelineV1(this.repoRoot, pipeline, { facet:"browser",invocation:this.invocation,components: new Set(playRuntimeComponentIds()), network: repositoryCargoNetworkV1(this.invocation, this.repoRoot), control: scriptCompilerAdmissionControlV1(this.invocation, "native:driver", 0) });
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
    await cancellable(async(signal) => {await buildViteArtifact({control:scriptCompilerAdmissionControlV1(this.invocation,"vite:entry",0), invocation: this.invocation, mode: readBuildMode(process.env.SEMIO_BUILD_MODE), artifactInputs: [...frameworkOsEngineDeliveriesV1(repositoryCachePartitionRootDataV1(this.repoRoot,"products"),{version:1,mode:readBuildMode(process.env.SEMIO_BUILD_MODE),facet:"play"}).map(row=>row.request),flowBrowserPackageRequestV1(repositoryCachePartitionRootDataV1(this.repoRoot,"products"),{version:1,mode:readBuildMode(process.env.SEMIO_BUILD_MODE)})], root, workspace: this.repoRoot, config: join(root, "🏗️builder/🌐️vite/🟦️.ts"), owner: "play:site", signal: this.invocation.control.signal, environment: { SEMIO_BUILD_MODE: readBuildMode(process.env.SEMIO_BUILD_MODE), SEMIO_RENDERER: "react", GIS_MAP_TILE_SERVE_MODE: "bundle" } });});
  }
}

/** 📄️ `pages`: binds the runtime modules, browser support and fonts into the bundled site with the module catalog, then splits it into the CDN pages. */
class PagesScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("pages accepts no arguments");
    await publishPlayPageCohortV1(this.repoRoot,this.invocation,PLAY_HOST);
  }
}

/** ⚖️ `verify`: the release gate over the published pages — the generated catalog equals a refusing render of the committed descriptors, and every module
 * page file is exactly the one the module catalog binds. */
class VerifyScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("verify accepts no arguments");
    const registry = join(this.repoRoot, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated");
    await withPublishedPlayPagesV1(this.repoRoot,this.invocation,async pages=>{
      const {renderCatalogFiles}=await import("../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts");
      const findings=[...Object.entries(renderCatalogFiles(this.repoRoot,undefined,"refuse").files).filter(([name,content])=>!existsSync(join(registry,name))||readFileSync(join(registry,name),"utf8")!==content).map(([name])=>`Generated catalog ${name} differs from committed descriptors`),...playModuleCatalogFindingsV1(pages.root)];
      for(const finding of findings)console.error(finding);
      if(findings.length)throw Error(`Play release verification failed: ${findings.length} finding(s)`);
      console.log("Play release verified: exact cohort and module catalog bind published pages");
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("map-tiles", MapTilesScript).register("bundle", BundleSiteScript).register("pages", PagesScript).register("verify", VerifyScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (router).run(process.argv.slice(2), original));
