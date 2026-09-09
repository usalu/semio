#!/usr/bin/env bun
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { assertGeneratedSources, readSourceInputContract } from "../../../../../../../🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/🟨️.mjs";
import { renderBrowserBoot } from "./🟦️.ts";

/** 🚀️ Materializes the browser boot bundle after Nx prepares its generated data inputs. */
class GenerateBrowserBootScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Browser boot generation accepts no arguments");
    assertGeneratedSources(readSourceInputContract(join(import.meta.dir, "🔣️.json")), this.repoRoot);
    const artifact = await renderBrowserBoot(this.root);
    mkdirSync(dirname(artifact.path), { recursive: true });
    writeFileSync(artifact.path, artifact.content, "utf8");
    console.log("framework-renderer-wgpu: generated 🚀️boot.js");
  }
}

const router = new ScriptRouter(resolve(import.meta.dir, "../📦️packages/🦀️rust")).register("generate-browser-boot", GenerateBrowserBootScript);
if (import.meta.main) await router.run(process.argv.slice(2));
