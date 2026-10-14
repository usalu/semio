#!/usr/bin/env bun
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain, scriptInvocationBudget } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { GuestFrameworkCheckScript, RebuildAllScript, printRebuildPlanV1 } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild/🟦️.ts";
import { readSRebuildPlanV1, readSGuestRebuildPlanV1 } from "./🟦️.ts";

/** 🏁️ Runs the owning S deployment's explicit composed rebuild. */
class SRebuildScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    await new RebuildAllScript(this.root, this.repoRoot, this.invocation, readSRebuildPlanV1()).run(args);
  }
}

/** 🧊️ Checks only the S owner's guest contribution. */
class SGuestCheckScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    await new GuestFrameworkCheckScript(this.root, this.repoRoot, this.invocation, readSGuestRebuildPlanV1()).run(args);
  }
}

/** 📋️ Observes the complete validated deployment plan without building components. */
class SPlanCheckScript extends BundleScript {
  run(args: string[]): void {
    if (args.length) throw Error("plan-check accepts no arguments");
    printRebuildPlanV1(readSRebuildPlanV1());
  }
}

/** 🧪️ Verifies concrete owner composition against independent Ajv admission. */
class SRebuildTestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw Error("test accepts no arguments");
    await runOwnedCommand(process.execPath, ["test", "./🧪️tests/🟦️.ts"], this.root, "s-rebuild-contract", scriptInvocationBudget(this.invocation, 120000), { signal: this.invocation.control.signal });
  }
}
const router = new ScriptRouter(import.meta.dir).register("rebuild-all", SRebuildScript).register("guest-framework-check", SGuestCheckScript).register("plan-check", SPlanCheckScript).register("test", SRebuildTestScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, invocation => runScriptMain(router, { invocation }));
