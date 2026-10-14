#!/usr/bin/env bun
import {repositoryCargoNetworkV1} from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
import {buildRepositoryCargoArtifacts} from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
import { buildBudgetMs } from '../../../../🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts';
import { scriptCompilerAdmissionControlV1 } from '../../../../🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts';
import { receiveScriptProcessInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { join } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

class BuildScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Font tool build takes no arguments");
    await buildRepositoryCargoArtifacts(join(this.root, "Cargo.toml"), ["--bin", "pack-typst-font-assets"], this.repoRoot,{network:repositoryCargoNetworkV1(this.invocation,this.repoRoot),control:scriptCompilerAdmissionControlV1(this.invocation,"cargo:build",buildBudgetMs())});
  }
}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(import.meta.dir).register("build", BuildScript)).run(process.argv.slice(2), original));
