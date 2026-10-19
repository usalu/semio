#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { resolve } from "node:path";
import { runRepositoryCargoTests } from "../../../📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runRepositoryCommand } from "../../../📚️library/📦️packages/🟦️typescript/🏃️process/🟦️.ts";

/** 🧪️ Checks the neutral host without compiling any contributed adapter. */
class CheckScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("The neutral test host check accepts no arguments");
    await runRepositoryCommand(this.invocation,"cargo",["check", "--locked", "--manifest-path", resolve(this.root, "Cargo.toml")],this.repoRoot,"repo-test-host:check",{version:1,maximumElapsedMilliseconds:0,output:null});
  }
}

/** ⚖️ Runs the host's own protocol and runner laws through the shared native test contract. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const { rest } = resolveTestLevel(args);
    await runRepositoryCargoTests(["semio-repo-test-host"], this.repoRoot, this.invocation, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("check", CheckScript).register("test", TestScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "check" }) }));
