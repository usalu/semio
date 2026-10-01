#!/usr/bin/env bun
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { assertGeneratedSources, readSourceInputContract } from "../../../../../../../🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/🟨️.mjs";
import { renderBrowserBoot } from "./🟦️.ts";

/** 🚀️ Materializes the browser boot bundle after Nx prepares its generated data inputs. */
class GenerateBrowserBootScript extends BundleScript {
  protected readonly entry: "browser-boot" | "renderer-boot" = "browser-boot";
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Browser boot generation accepts no arguments");
    assertGeneratedSources(readSourceInputContract(join(import.meta.dir, "🔣️.json")), this.repoRoot);
    const artifact = await renderBrowserBoot(this.root, this.repoRoot, this.entry);
    mkdirSync(dirname(artifact.path), { recursive: true });
    writeFileSync(artifact.path, artifact.content, "utf8");
    console.log("framework-renderer-wgpu: generated " + this.entry);
  }
}

/** 🪆️ Materializes the public embedded browser library through its exclusive producer. */
class GenerateRendererBootScript extends GenerateBrowserBootScript {
  protected readonly entry = "renderer-boot" as const;
}

const router = new ScriptRouter(resolve(import.meta.dir, "../📦️packages/🦀️rust"))
  .register("generate-browser-boot", GenerateBrowserBootScript)
  .register("generate-renderer-boot", GenerateRendererBootScript);
if (import.meta.main) await router.run(process.argv.slice(2));
