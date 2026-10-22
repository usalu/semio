#!/usr/bin/env repo
import { receiveScriptProcessInvocation, BundleScript, ScriptRouter, runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { GoTestScript } from "../../🧪️execution/🐹️go/🟦️.ts";
import { WorkspacePublicationScript } from "../../🗂️workspaces/🏃️execution/🟦️.ts";
import { TypecheckScript } from "../../🎛️commands/🪁️typecheck/🟦️.ts";
import { LintScript } from "../../🎛️commands/🕸️dependency-lint/🟦️.ts";
import { SchemaReadInventoryScript } from "../../🎛️commands/🗂️schema-read-inventory/🟦️.ts";
import { RepositoryUiScript } from "../../🎛️commands/🖼️repository-ui/🟦️.ts";
import { FixtureOwnershipTestScript } from "../../🎛️commands/🧱️fixture-ownership/🟦️.ts";
import { GeneralExecutionOwnershipScript } from "../../🎛️commands/🗺️general-execution-ownership/🟦️.ts";
import { TestScript } from "../../🧪️tests/🔬️typescript-package-commands/🟦️.ts";


/** 💤️ Loads the tool-run policy source owner only for its explicit test command. */
class ToolRunPolicySourceTestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const { ToolRunPolicySourceScript } = await import("../../🔍️discovery/📋️policy/⏯️tool-run/🧪️tests/🔬️script-commands/🟦️.ts");
    await new ToolRunPolicySourceScript(this.root, this.repoRoot, this.invocation).run(args);
  }
}

/** 🧭️ `@semio-tech/repo-lib` router: `repo ./📜️script.ts <typecheck|test [level]|workspaces <--write|--check>>`. */
const router = new ScriptRouter(import.meta.dirname)
  .register("test-tool-run-policy-source",ToolRunPolicySourceTestScript)
  .register("test-fixture-ownership",FixtureOwnershipTestScript).register("test-general-execution-ownership",GeneralExecutionOwnershipScript)
  .register("typecheck", TypecheckScript)
  .register("lint", LintScript)
  .register("schema-read-inventory", SchemaReadInventoryScript)
  .register("test", TestScript)
  .register("go-test", GoTestScript)
  .register("workspaces", WorkspacePublicationScript)
  .register("ui", RepositoryUiScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));