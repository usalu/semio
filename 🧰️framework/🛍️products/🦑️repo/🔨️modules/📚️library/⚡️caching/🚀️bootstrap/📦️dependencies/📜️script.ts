#!/usr/bin/env bun
import {withJavascriptDependencyStorageV1} from "./💾️storage/🟦️.ts";
import {prepareJavascriptDependencies,runTool} from "./🟦️.ts";
import {mkdirSync} from "node:fs";
import {dirname,join} from "node:path";
import {Script,ScriptRouter,checkScriptInvocation,receiveScriptProcessInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {discoverCurrentBunWorkspaceScopesV1} from "../../../🗂️workspaces/🟦️bun/🟦️.ts";
import {repositoryCargoPreparationStorageV1} from "../../📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
import {getWorkspaceRoot} from "../../../🗂️workspaces/🟦️.ts";

/** 📦️ Synchronizes the locked JavaScript environment without loading application modules. */
export class SyncScript extends Script {
  protected readonly mode: "sync" | "lock" = "sync";
  async run(args: string[]): Promise<void> {
    if (args.length && !(args.length===1 && args[0]==="--all") && !(args.length===2 && args[0]==="--scope")) throw new Error("Dependency synchronization accepts --all or --scope <owner>");
    checkScriptInvocation(this.invocation);
    const selection=args[0]==="--all"||!args.length&&this.mode==="lock"?{kind:"all" as const}:{kind:"scope" as const,owner:args[0]==="--scope"?args[1]!:"."};
    const control = {
      signal: this.invocation.control.signal,
      advance: async (progress:import("../../../🗂️workspaces/🟦️bun/🟦️.ts").BunWorkspaceProgress) => {
        checkScriptInvocation(this.invocation);
        if (progress.completedOperations % 128 === 0) {
          await this.invocation.control.publish({ owner: this.invocation.policy.owner, command: "Bun ownership " + progress.operation + " " + progress.completedOperations, stage: "running" });
          await this.invocation.control.yieldContinuation();
        }
        checkScriptInvocation(this.invocation);
      },
    };
    const scopes = await discoverCurrentBunWorkspaceScopesV1(this.root, selection, control);
    if (!scopes.length) throw new Error("Unknown present Bun installation scope");
    for (const scope of scopes) {
      checkScriptInvocation(this.invocation);
      console.log("[bun-workspace] " + scope.manifest + " " + this.mode);
      await withJavascriptDependencyStorageV1(this.repoRoot,this.invocation,store=>prepareJavascriptDependencies(this.mode,join(this.root,dirname(scope.manifest)),store));
      checkScriptInvocation(this.invocation);
    }
    if(selection.kind==="all"){const current=await discoverCurrentBunWorkspaceScopesV1(this.root,selection,control);if(JSON.stringify(current.map(scope=>scope.manifest))!==JSON.stringify(scopes.map(scope=>scope.manifest)))throw Error("Original Bun installation owner membership changed");}
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
