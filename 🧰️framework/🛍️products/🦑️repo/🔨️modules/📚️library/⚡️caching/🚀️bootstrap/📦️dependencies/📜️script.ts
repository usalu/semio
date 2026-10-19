#!/usr/bin/env bun
import {withJavascriptDependencyStorageV1,prepareJavascriptDependencies} from "./💾️storage/🟦️.ts";
import {join} from "node:path";
import {Script,ScriptRouter,checkScriptInvocation,receiveScriptProcessInvocation,createIssuedResourceControlV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{withRepositoryTestArtifactsV1,repositoryTestArtifactEnvironmentV1}from"../../../../🧪️test/📦️packages/🟦️typescript/💾️storage/🟦️.ts";
import{runRepositoryCommand}from"../../../🏃️process/🎛️owned-execution/🟦️.ts";
import {getWorkspaceRoot} from "../../../🗂️workspaces/🟦️.ts";

/** 📦️ Synchronizes the locked JavaScript environment without loading application modules. */
export class SyncScript extends Script {
  protected readonly mode: "sync" | "lock" = "sync";
  async run(args: string[]): Promise<void> {
    if (args.length && !(args.length===1 && args[0]==="--all") && !(args.length===2 && args[0]==="--scope")) throw new Error("Dependency synchronization accepts --all or --scope <owner>");
    checkScriptInvocation(this.invocation);
    const selection=args[0]==="--all"||!args.length?{kind:"all" as const}:{kind:"scope" as const,owner:args[0]==="--scope"?args[1]!:"."};
    const phaseControl=createIssuedResourceControlV1(this.invocation,"bun:dependency-storage",0);
    await withJavascriptDependencyStorageV1(this.repoRoot,this.invocation,phaseControl,(store,control)=>prepareJavascriptDependencies(this.mode,this.repoRoot,store,control,selection));
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
    await withRepositoryTestArtifactsV1(this.repoRoot,this.invocation,createIssuedResourceControlV1(this.invocation,"dependencies:tests",0),()=>runRepositoryCommand(this.invocation,process.execPath,["test",join(import.meta.dir,"🧪️tests/🟦️.ts")],this.repoRoot,"dependencies:contract-check",{version:1,maximumElapsedMilliseconds:120000,output:{maximumBytes:134217728,maximumLines:65536}},{env:repositoryTestArtifactEnvironmentV1(this.repoRoot)}));
    checkScriptInvocation(this.invocation);
  }
}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(getWorkspaceRoot()).register("sync", SyncScript).register("lock", RefreshLockScript).register("contract-check", ContractCheckScript)).run(process.argv.slice(2), original));
