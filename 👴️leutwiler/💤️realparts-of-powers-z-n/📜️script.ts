#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
/** 💤️ `@leutwiler/realparts-of-powers-z-n` task router: `bun ./📜️script.ts <deps|build|test> [lean|📯️notes|🏆️proof]`. */
import { BundleScript, ScriptRouter } from "../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { build, prepareDependencies, verifyAxioms } from "./🟦️.ts";

class DepsScript extends BundleScript {
  async run(): Promise<void> {
    await prepareDependencies(this.root, this.repoRoot);
  }
}

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await build(this.root, this.repoRoot, segments[0]);
  }
}

class TestScript extends BundleScript {
  async run(): Promise<void> {
    await verifyAxioms(this.root, this.repoRoot);
  }
}

const router = new ScriptRouter(import.meta.dir).register("deps", DepsScript).register("build", BuildScript).register("test", TestScript);

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => router.run(process.argv.slice(2), original));
