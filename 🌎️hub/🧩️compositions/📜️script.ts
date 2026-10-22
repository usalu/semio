#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { runRepositoryCargoTests } from "../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runRepositoryCommand } from "../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { join } from "node:path";
import { assertConcreteCompositionOwnership } from "./🧪️tests/📇️ownership/🟦️.ts";

class OwnershipScript extends BundleScript {
  async run(): Promise<void> {
    assertConcreteCompositionOwnership(this.root);
  }
}

/** 🧪️ Runs the TypeScript twins of the hub composition laws that share fixtures with their native tests. */
class TypescriptScript extends BundleScript {
  async run(): Promise<void> {
    const tests = ["📏️layout/🖊️drawing/🧪️tests/🪶️sqlite/🟦️.ts", "📏️layout/🖊️drawing/🧪️tests/🪪️document/🟦️.ts"].map((file) => join(this.root, file));
    await runRepositoryCommand(this.invocation, process.execPath, ["test", ...tests], this.repoRoot, "hub-compositions", { version: 1, maximumElapsedMilliseconds: 0, output: null });
  }
}

class NativeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const names = assertConcreteCompositionOwnership(this.root);
    const { rest } = resolveTestLevel(segments, "quick");
    await runRepositoryCargoTests(names, this.repoRoot, this.invocation, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("ownership", OwnershipScript).register("native", NativeScript).register("typescript", TypescriptScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "ownership" }) }));
