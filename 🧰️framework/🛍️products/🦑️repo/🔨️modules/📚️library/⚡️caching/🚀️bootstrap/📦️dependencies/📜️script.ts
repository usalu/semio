#!/usr/bin/env bun
import {withJavascriptDependencyStorageV1} from "./💾️storage/🟦️.ts";
import {prepareJavascriptDependencies,runTool} from "./🟦️.ts";
import {mkdirSync} from "node:fs";
import {dirname,join} from "node:path";
import {Script,ScriptRouter,checkScriptInvocation,receiveScriptProcessInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {publishBunWorkspaceOwnership} from "../../../🗂️workspaces/🟦️bun/🟦️.ts";
import {repositoryCargoPreparationStorageV1} from "../../📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
import {getWorkspaceRoot} from "../../../🗂️workspaces/🟦️.ts";

/** 📦️ Synchronizes the locked JavaScript environment without loading application modules. */
export class SyncScript extends Script {
  protected readonly mode: "sync" | "lock" = "sync";
  async run(args: string[]): Promise<void> {
    if (args.length && !(args.length===1 && args[0]==="--all") && !(args.length===2 && args[0]==="--scope")) throw new Error("Dependency synchronization accepts --all or --scope <owner>");
    checkScriptInvocation(this.invocation);
    if(args[0]==="--scope"){await withJavascriptDependencyStorageV1(this.repoRoot,this.invocation,store=>prepareJavascriptDependencies(this.mode,join(this.root,args[1]!),store));return;}
    const report = await publishBunWorkspaceOwnership(this.root, {
      signal: this.invocation.control.signal,
      advance: async progress => {
        checkScriptInvocation(this.invocation);
        if (progress.completedOperations % 128 === 0) {
          await this.invocation.control.publish({ owner: this.invocation.policy.owner, command: "Bun ownership " + progress.operation + " " + progress.completedOperations, stage: "running" });
          await this.invocation.control.yieldContinuation();
        }
        checkScriptInvocation(this.invocation);
      },
    });
    const scopes = report.scopes.map(scope => ({ ...scope, directory: dirname(scope.manifest).replaceAll("\\", "/") })), requested = args[0] === "--all" || !args.length && this.mode === "lock" ? scopes : scopes.filter(scope => scope.directory === (args[0] === "--scope" ? args[1] : "."));
    if (!requested.length) throw new Error("Unknown present Bun installation scope");
    for (const scope of requested) {
      checkScriptInvocation(this.invocation);
      console.log("[bun-workspace] " + scope.manifest + " " + this.mode);
      await withJavascriptDependencyStorageV1(this.repoRoot,this.invocation,store=>prepareJavascriptDependencies(this.mode,join(this.root,scope.directory),store));
      checkScriptInvocation(this.invocation);
    }
  }
}

/** 🔒️ Refreshes only the declared workspace lock through the selected Bun runtime. */
export class RefreshLockScript extends SyncScript {
  protected readonly mode = "lock" as const;
}

/** ⚖️ Executes portable dependency command and independent Node oracle laws. */
export class ContractCheckScript extends Script {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Dependency contract check accepts no arguments");
    checkScriptInvocation(this.invocation);
    const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR ?? join(this.root, ".🧬semio/🦑️repo/⚡️cache/tests/bootstrap-dependencies");
    mkdirSync(artifacts, { recursive: true });
    await runTool(process.execPath,["test", join(import.meta.dir, "🧪️tests/🟦️.ts")],this.root,this.invocation.control.signal,false,{ ...process.env, SEMIO_TEST_ARTIFACT_DIR: artifacts },repositoryCargoPreparationStorageV1(process.cwd()));
    checkScriptInvocation(this.invocation);
  }
}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(getWorkspaceRoot()).register("sync", SyncScript).register("lock", RefreshLockScript).register("contract-check", ContractCheckScript)).run(process.argv.slice(2), original));
