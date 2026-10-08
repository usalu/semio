#!/usr/bin/env bun
import { join } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../🏃️process/🧭️routing/🟦️.ts";
import { buildCargoArtifacts, readCargoArtifactBuildPolicyV1 } from "../../../../🏃️process/📦️artifacts/🏗️native-build/🟦️.ts";

class BuildScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Font tool build takes no arguments");
    await buildCargoArtifacts(join(this.root, "Cargo.toml"), ["--bin", "pack-typst-font-assets"], readCargoArtifactBuildPolicyV1(process.env, this.root));
  }
}

if (import.meta.main) await new ScriptRouter(import.meta.dir).register("build", BuildScript).run(process.argv.slice(2));
