import {withJavascriptDependencyStorageV1,prepareJavascriptDependencies} from "../💾️storage/🟦️.ts";
import {Script,checkScriptInvocation,createIssuedResourceControlV1,findWorkspaceRoot} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {getWorkspaceRoot} from "../../../../🗂️workspaces/🟦️.ts";

/** 📦️ Synchronizes the locked JavaScript environment without loading application modules. */
export class SyncScript extends Script {
  protected readonly mode: "sync" | "lock" = "sync";
  async run(args: string[]): Promise<void> {
    if (args.length && !(args.length===1 && args[0]==="--all") && !(args.length===2 && args[0]==="--scope")) throw new Error("Dependency synchronization accepts --all or --scope <owner>");
    checkScriptInvocation(this.invocation);
    const selection=args[0]==="--all"||!args.length?{kind:"all" as const}:{kind:"scope" as const,owner:args[0]==="--scope"?args[1]!:"."};
    const phaseControl=createIssuedResourceControlV1(this.invocation,"bun:dependency-storage",0);
    await withJavascriptDependencyStorageV1(findWorkspaceRoot(getWorkspaceRoot()),this.invocation,phaseControl,(store,control)=>prepareJavascriptDependencies(this.mode,findWorkspaceRoot(getWorkspaceRoot()),store,control,selection));
  }
}

/** 🔒️ Refreshes only the declared workspace lock through the selected Bun runtime. */
export class RefreshLockScript extends SyncScript {
  protected readonly mode = "lock" as const;
}
