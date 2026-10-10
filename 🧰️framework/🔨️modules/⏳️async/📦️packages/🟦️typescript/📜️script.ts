#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { runVitestV1, readVitestPolicyV1 } from "../../../🏃️process/🧪️testing/🧪️vitest/🟦️.ts";
/** 🧭️ Runs the async scheduler, fixed-slot, publication, and public API contracts. */

import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { resolve } from "node:path";
import { runOwnedCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";

/** 📏️ Executes the actual closed public API compiler and runtime laws. */
class ApiContractScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw Error("test-api-contract accepts no arguments");
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../📏️api/🧪️tests/🟦️.ts")], this.repoRoot, "async:public-api", 30_000);
  }
}

class InfoScript extends BundleScript {
  run(): void {
    console.log(
      "@semio-tech/framework-async: owned async wire types and host continuation scheduling. " +
        "test runs scheduler and fixed-slot laws; twin checks the host event loop; " +
        "test-api-contract verifies the closed public surface.",
    );
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitestV1(readVitestPolicyV1(process.env,this.root), rest, "../../🧪️tests/🎚️config/🟦️.ts", process.env);
  }
}

/** 🪞️ Node twin: the continuation fixture suite with NO test framework, on the platform's own
 * `MessageChannel`/`setTimeout` — the independent oracle for the virtual-clock suite. */
class TwinScript extends BundleScript {
  async run(): Promise<void> {
    await import("../../🪃️continuation/🧪️tests/🔬️node-twin/🟦️.ts");
  }
}

/** 🔐️ Checks the closed language-neutral publication admission contract with independent oracles. */
class PublicationContractScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw Error("Expected test-publication-contract");
    const tests = [resolve(this.root, "../../🔐️publication/🧪️tests/🟦️.ts"), resolve(this.root, "../../🔐️publication/🛂️checkpoint/🧪️tests/🟦️.ts")];
    await runOwnedCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", this.root), "--noEmit", "--strict", "--noUncheckedIndexedAccess", "--skipLibCheck", "--allowImportingTsExtensions", "--target", "ES2022", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", ...tests], this.root, "async:publication:strict", 30000);
    await runOwnedCommand(process.execPath, ["test", ...tests], this.root, "async:publication:contract", 30000);
  }
}

import { runBudgetedTestCommand } from "../../../🏃️process/🧪️testing/🎛️execution/🟦️.ts";

/** 🔂️ Verifies immediate one-poll semantics in the owned language-neutral corpus. */
class SinglePollCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw Error("single-poll-check accepts no arguments");
    const test = resolve(this.root, "../../🔂️poll/🧪️tests/🟦️.ts");
    await runBudgetedTestCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", this.root), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", test], {cwd: this.repoRoot, budgetMs: 30000, throwOnFailure: true});
    await runBudgetedTestCommand(process.execPath, ["test", test], {cwd: this.repoRoot, budgetMs: 15000, throwOnFailure: true});
  }
}

const router = new ScriptRouter(import.meta.dir).register("info", InfoScript).register("test", TestScript).register("twin", TwinScript).register("test-publication-contract", PublicationContractScript).register("single-poll-check", SinglePollCheckScript).register("test-api-contract", ApiContractScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "info" }) }));
