import { Script, scriptInvocationBudget } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";

/** 🚦️ The root `test` command: `test entrypoint` runs the command-startup law, every other selection goes to the test platform. */
export class RootTestScript extends Script {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "entrypoint") {
      if (segments.length !== 1) throw new Error("Expected test entrypoint without additional arguments");
      await runOwnedCommand(process.execPath, ["test", "./🧪️tests/🧭️entrypoint/🟦️.ts"], this.repoRoot, "workspace:command-startup", scriptInvocationBudget(this.invocation, 120_000), { signal: this.invocation.control.signal });
      return;
    }
    const { TestScript } = await import("../🟦️.ts");
    await new TestScript(this.root, this.repoRoot, this.invocation).run(segments);
  }
}
