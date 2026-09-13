#!/usr/bin/env bun
import { join, relative } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { buildCargoArtifacts } from "../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";

class BuildScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Font tool build takes no arguments");
    const workspace = this.repoRoot;
    await buildCargoArtifacts(relative(workspace, join(import.meta.dir, "Cargo.toml")), ["--bin", "dump-guestslim-typst-fonts"], workspace);
  }
}

if (import.meta.main) await new ScriptRouter(import.meta.dir).register("build", BuildScript).run(process.argv.slice(2));
