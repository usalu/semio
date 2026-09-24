#!/usr/bin/env bun
import { join, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { buildViteArtifact } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts";
import { prefetchPlayMapTiles } from "./🗺️map-tiles/🟦️.ts";
import { PLAY_HOST } from "../🧩️runtime/🟦️.ts";
import { publishPlayPages } from "./📄pages/🟦️.ts";

/** @emoji 🎡️ Publishes play as one static site using prerequisites selected by the outer Nx graph. */
class BuildScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("The play build accepts no compiler arguments");
    const root = resolve(this.root, "../.."), controller = new AbortController();
    const cancel = (): void => { controller.abort(); process.exitCode = 130; };
    process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
    try {
      await prefetchPlayMapTiles(this.repoRoot);
      await buildViteArtifact({ root, workspace: this.repoRoot, config: join(root, "🏗️builder/🌐️vite/🟦️.ts"), output: join(root, "dist/site"), owner: "play:site", signal: controller.signal, environment: { SEMIO_BUILD_MODE: "ship", SEMIO_RENDERER: "react", GIS_MAP_TILE_SERVE_MODE: "bundle" }, ...(process.env.SEMIO_TICKET_DIR ? { temporaryRoot: join(process.env.SEMIO_TICKET_DIR, "🗑️generated") } : {}) });
      publishPlayPages(join(root, "dist/site"), join(root, "dist/pages"), PLAY_HOST);
    } finally { process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript);
if (import.meta.main) await router.run(process.argv.slice(2));
