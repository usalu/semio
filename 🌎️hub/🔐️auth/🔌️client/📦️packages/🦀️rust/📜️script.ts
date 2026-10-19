#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { resolve } from "node:path";
import { runRepositoryCargoTests } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runRepositoryCommand } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";

/** 🛂️ Checks the actual Hub credential provider. */
class CheckScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Credential provider check accepts no arguments");
    await runRepositoryCommand(this.invocation,"cargo",["check", "--locked", "--manifest-path", resolve(this.root, "Cargo.toml")],this.repoRoot,"hub-auth-client:check",{version:1,maximumElapsedMilliseconds:0,output:null});
  }
}

/** 🧪️ Runs the actual native owner credential laws. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const { rest } = resolveTestLevel(args);
    await runRepositoryCargoTests(["semio-hub-auth-client"], this.repoRoot, this.invocation, rest);
  }
}

/** 🧬️ Proves portable protocol selection and independent schema admission. */
class SourceScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Credential source law accepts no arguments");
    const { testCredentialProtocolSourceV1 } = await import("../../🧪️tests/🧬️source/🟦️.ts");
    await testCredentialProtocolSourceV1(this.repoRoot);
  }
}

const router = new ScriptRouter(import.meta.dir).register("check", CheckScript).register("test", TestScript).register("test-source", SourceScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "check" }) }));
