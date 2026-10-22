import {resolve} from "node:path";
import {Script,checkScriptInvocation,createIssuedResourceControlV1,findWorkspaceRoot} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {withRepositoryTestArtifactsV1,repositoryTestArtifactEnvironmentV1} from "../../../../../../🧪️test/📦️packages/🟦️typescript/💾️storage/🟦️.ts";
import {runRepositoryCommand} from "../../../../../🏃️process/🎛️owned-execution/🟦️.ts";
import {getWorkspaceRoot} from "../../../../../🗂️workspaces/🟦️.ts";

/** ⚖️ Executes portable dependency command and independent Node oracle laws. */
export class ContractCheckScript extends Script {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Dependency contract check accepts no arguments");
    checkScriptInvocation(this.invocation);
    await withRepositoryTestArtifactsV1(findWorkspaceRoot(getWorkspaceRoot()),this.invocation,createIssuedResourceControlV1(this.invocation,"dependencies:tests",0),()=>runRepositoryCommand(this.invocation,process.execPath,["test",resolve(import.meta.dir,"../🟦️.ts")],findWorkspaceRoot(getWorkspaceRoot()),"dependencies:contract-check",{version:1,maximumElapsedMilliseconds:120000,output:{maximumBytes:134217728,maximumLines:65536}},{env:repositoryTestArtifactEnvironmentV1(findWorkspaceRoot(getWorkspaceRoot()))}));
    checkScriptInvocation(this.invocation);
  }
}
