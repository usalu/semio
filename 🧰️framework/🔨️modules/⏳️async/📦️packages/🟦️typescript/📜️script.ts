#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { runVitestV1, readVitestPolicyV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🧪️vitest/🟦️.ts";
/** 🧭️ Runs the async scheduler, fixed-slot, publication, and public API contracts. */

import { BundleScript, ScriptRouter } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 📏️ Executes the actual closed public API compiler and runtime laws. */
class ApiContractScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runApiContractScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
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
    await runPublicationContractScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
import { runApiContractScript, runPublicationContractScript, runSinglePollCheckScript } from "../../🧪️tests/🔬️typescript-package-commands/🟦️.ts";

/** 🔂️ Verifies immediate one-poll semantics in the owned language-neutral corpus. */
class SinglePollCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runSinglePollCheckScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("info", InfoScript).register("test", TestScript).register("twin", TwinScript).register("test-publication-contract", PublicationContractScript).register("single-poll-check", SinglePollCheckScript).register("test-api-contract", ApiContractScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "info" }) }));
