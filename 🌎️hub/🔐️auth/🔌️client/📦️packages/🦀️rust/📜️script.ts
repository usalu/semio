#!/usr/bin/env bun
import { resolveTestLevel } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { resolve } from "node:path";
import { runRepositoryCargoTests } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runRepositoryCommand } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";

/** 🛂️ Checks the actual Hub credential provider. */
class CheckScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Credential provider check accepts no arguments");
    await runRepositoryCommand("cargo", ["check", "--locked", "--manifest-path", resolve(this.root, "Cargo.toml")], this.repoRoot, "hub-auth-client:check");
  }
}

/** 🧪️ Runs the actual native owner credential laws. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const { rest } = resolveTestLevel(args);
    await runRepositoryCargoTests(["semio-hub-auth-client"], this.repoRoot, rest);
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
if (import.meta.main) await runScriptMain(router, { defaultCommand: "check" });
