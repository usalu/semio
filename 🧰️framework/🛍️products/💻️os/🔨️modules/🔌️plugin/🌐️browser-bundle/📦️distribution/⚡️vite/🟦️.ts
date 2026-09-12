import { resolve } from "node:path";
import { copyBrowserArtifacts, type BrowserArtifactSource, type BrowserArtifactCopyOptions } from "../🟦️.ts";

/** 📦️ Adds completed runtime artifacts to Vite's private production output before publication. */
export function browserArtifactVitePlugin(sources: readonly BrowserArtifactSource[], options: BrowserArtifactCopyOptions = {}) {
  let output: string | undefined;
  return {
    name: "semio-browser-artifacts",
    apply: "build" as const,
    configResolved(config: { root: string; build: { outDir: string } }): void { output = resolve(config.root, config.build.outDir); },
    async writeBundle(): Promise<void> {
      if (!output) throw new Error("Browser artifact output was not resolved");
      const count = await copyBrowserArtifacts(output, sources, options);
      console.log(`Staged ${count} browser runtime files from ${sources.length} completed artifacts`);
    },
  };
}
