#!/usr/bin/env bun
import { join, relative } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { buildRepositoryCargoArtifacts } from "../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";

class BuildScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Font tool build takes no arguments");
    const workspace = this.repoRoot;
    await buildRepositoryCargoArtifacts(relative(workspace, join(import.meta.dir, "Cargo.toml")), ["--bin", "dump-guestslim-typst-fonts"], workspace);
  }
}

if (import.meta.main) await new ScriptRouter(import.meta.dir).register("build", BuildScript).run(process.argv.slice(2));
