import {withJavascriptDependencyStorageV1,prepareJavascriptDependencies} from "../💾️storage/🟦️.ts";
import {Script,checkScriptInvocation,createIssuedResourceControlV1,findWorkspaceRoot} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {getWorkspaceRoot} from "../../../../🗂️workspaces/🟦️.ts";

/** 📦️ Synchronizes the locked JavaScript environment without loading application modules. */
export class SyncScript extends Script {
  protected readonly mode: "sync" | "lock" = "sync";
  async run(args: string[]): Promise<void> {
    const owners=args.length>=2&&args.length%2===0&&args.every((value,index)=>index%2===0?value==="--scope":value.length>0)?args.filter((_,index)=>index%2===1):[];
    if (args.length && !(args.length===1 && args[0]==="--all") && !owners.length) throw new Error("Dependency synchronization accepts --all or one or more --scope <owner>");
    if (owners.length>1 && this.mode==="lock") throw new Error("Dependency lock accepts --all or one --scope <owner>");
    checkScriptInvocation(this.invocation);
    const selection=args[0]==="--all"||!args.length?{kind:"all" as const}:owners.length>1?{kind:"scopes" as const,owners:[...new Set(owners)]}:{kind:"scope" as const,owner:owners[0]!};
    (await import("../../../../🏗️build/🚀️bootstrap/🟦️.ts")).publishBuildBootstrapSourcesV1(findWorkspaceRoot(getWorkspaceRoot()));
    const phaseControl=createIssuedResourceControlV1(this.invocation,"bun:dependency-storage",0);
    await withJavascriptDependencyStorageV1(findWorkspaceRoot(getWorkspaceRoot()),this.invocation,phaseControl,(store,control)=>prepareJavascriptDependencies(this.mode,findWorkspaceRoot(getWorkspaceRoot()),store,control,selection));
  }
}

/** 🔒️ Refreshes only the declared workspace lock through the selected Bun runtime. */
export class RefreshLockScript extends SyncScript {
  protected readonly mode = "lock" as const;
}
