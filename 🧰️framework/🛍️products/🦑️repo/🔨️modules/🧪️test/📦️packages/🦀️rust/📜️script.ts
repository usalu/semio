#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { resolve } from "node:path";
import { runRepositoryCargoTests } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runRepositoryCommand } from "../../../📚️library/🏃️process/🎛️owned-execution/🟦️.ts";

/** 🧪️ Checks the neutral host without compiling any contributed adapter. */
class CheckScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("The neutral test host check accepts no arguments");
    await runRepositoryCommand("cargo", ["check", "--locked", "--manifest-path", resolve(this.root, "Cargo.toml")], this.repoRoot, "repo-test-host:check");
  }
}

/** ⚖️ Runs the host's own protocol and runner laws through the shared native test contract. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const { rest } = resolveTestLevel(args);
    await runRepositoryCargoTests(["semio-repo-test-host"], this.repoRoot, this.invocation.control, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("check", CheckScript).register("test", TestScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "check" }) }));
