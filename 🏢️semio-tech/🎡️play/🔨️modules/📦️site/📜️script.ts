#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { join, resolve } from "node:path";
import { availableParallelism, totalmem } from "node:os";
import { mkdir, mkdtemp, writeFile } from "node:fs/promises";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { buildViteArtifact } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts";
import { repoCacheDirectory } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";
import { componentPrebuildPlanV1 } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { runOwnedNxInvocationV1 } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts";
import { prefetchPlayMapTiles } from "./🗺️map-tiles/🟦️.ts";
import { PLAY_HOST } from "../🧩️runtime/🟦️.ts";
import { publishPlayPages } from "./📄pages/🟦️.ts";
import { PLAY_COMPONENT_WORKSPACES, PLAY_FRESH_GENERATION_PREFIX, playFreshBuildArgs, playFreshBuildEnvironment } from "./🆕️fresh-build/🟦️.ts";

/** 🆕️ Executes the complete release graph in a new compiler and Nx cache generation. Every plugin and extension component of the hub and artifact workspaces is compiled in one Cargo pass per profile (`SEMIO_CARGO_PREBUILD_PLAN`), and the nested Nx run streams its complete output to the generation's `o.log` (`SEMIO_NX_OUTPUT_LOG`) so this task's own output stays bounded. */
class FreshBuildScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("The fresh play build accepts no arguments");
    const directory = repoCacheDirectory(this.repoRoot);
    await mkdir(directory, { recursive: true });
    const generation = await mkdtemp(join(directory, PLAY_FRESH_GENERATION_PREFIX)), environment = playFreshBuildEnvironment(this.repoRoot, generation, process.env);
    for (const key of Object.keys(process.env)) if (!(key in environment)) delete process.env[key];
    Object.assign(process.env, environment);
    await writeFile(environment.SEMIO_CARGO_PREBUILD_PLAN!, JSON.stringify(componentPrebuildPlanV1(this.repoRoot, PLAY_COMPONENT_WORKSPACES)));
    console.log(`Fresh play release cache: ${generation}`);
    await runOwnedNxInvocationV1(this.repoRoot, playFreshBuildArgs({ cpus: availableParallelism(), memoryBytes: totalmem() }), this.invocation);
  }
}

/** 📇️ Publishes the current descriptor catalog after every release component prerequisite. */
class CatalogScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("The play release catalog accepts no arguments");
    const { GenerateScript, renderCatalogFiles } = await import("../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts");
    renderCatalogFiles(this.repoRoot, undefined, "refuse");
    new GenerateScript(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry"), this.repoRoot, this.invocation).run([]);
  }
}

/** 🎡️ Publishes play as one static site using prerequisites selected by the outer Nx graph. */
class BuildScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("The play build accepts no compiler arguments");
    const root = resolve(this.root, "../.."), controller = new AbortController();
    const cancel = (): void => { controller.abort(); process.exitCode = 130; };
    process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
    try {
      await prefetchPlayMapTiles(this.repoRoot, controller.signal);
      await buildViteArtifact({ root, workspace: this.repoRoot, config: join(root, "🏗️builder/🌐️vite/🟦️.ts"), output: join(root, "dist/site"), owner: "play:site", signal: controller.signal, environment: { SEMIO_BUILD_MODE: "ship", SEMIO_RENDERER: "react", GIS_MAP_TILE_SERVE_MODE: "bundle" }, ...(process.env.SEMIO_TICKET_DIR ? { temporaryRoot: join(process.env.SEMIO_TICKET_DIR, "🗑️generated") } : {}) });
      publishPlayPages(join(root, "dist/site"), join(root, "dist/pages"), PLAY_HOST);
    } finally { process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("build-fresh", FreshBuildScript).register("catalog", CatalogScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (router).run(process.argv.slice(2), original));
