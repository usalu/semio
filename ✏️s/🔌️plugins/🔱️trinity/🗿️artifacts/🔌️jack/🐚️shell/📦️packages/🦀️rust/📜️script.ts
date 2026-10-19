#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runAdmittedCargoPackageTestsV1 } from "../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runRunScript, BINARY_NAME } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

class TestScript extends BundleScript {
  async run(_segments: string[]): Promise<void> {
    await runAdmittedCargoPackageTestsV1([BINARY_NAME], this.repoRoot, this.invocation);
  }
}

/** 🚀️ Execs the `build` target's staged binary (never `cargo run` — no wrapper-process tree to chase). */
class RunScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runRunScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("run", RunScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test" }));
