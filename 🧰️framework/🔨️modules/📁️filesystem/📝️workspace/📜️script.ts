import {Script,ScriptRouter,receiveScriptProcessInvocation,runScriptMain,scriptInvocationBudget} from "../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runOwnedCommand} from "../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {join} from "node:path";

class TestScript extends Script {
 async run(args:string[]):Promise<void>{if(args.length)throw Error("Source workspace tests accept no arguments");await runOwnedCommand(process.execPath,["test",join(this.root,"🧪️tests/🟦️.ts")],this.repoRoot,"source-workspace:test",scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal});}
}
await receiveScriptProcessInvocation(process.env,original=>runScriptMain(new ScriptRouter(import.meta.dir).register("test",TestScript),{invocation:original}));
