import {resetInactiveNxCacheNamespaceV1} from "./🟦️.ts";
import {BundleScript,ScriptRouter,runScriptMain,receiveScriptProcessInvocation,requireIssuedResourceControlInvocationV1,withRequestedIssuedResourceControlV1,createIssuedResourceControlV1,type ScriptInvocation} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {readNativeCallerAuthorityV1} from "../../../../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
/** ♻️ Runs only the fixed recomputable inactive Nx namespace operation. */
class InactiveNxResetScript extends BundleScript {async run(args:string[]):Promise<void>{if(args.length)throw Error("Inactive Nx reset accepts no arguments");await runInactiveNxCacheResetV1({root:this.root,repoRoot:this.repoRoot,invocation:this.invocation,control:createIssuedResourceControlV1(this.invocation,"cache:reset-inactive-nx",schema.$defs.Protocol.const.maximumMilliseconds)});}}
/** 🛡️ Receives the same issued caller phase without entering the generic build graph or cache. */
export function runInactiveNxCacheResetV1(context:Readonly<{root:string;repoRoot:string;invocation:ScriptInvocation;control:ResourceLeaseControl}>):Promise<void>{requireIssuedResourceControlInvocationV1(context.control,context.invocation);checkResourceLeaseControl(context.control);if(readNativeCallerAuthorityV1(context.invocation.capabilities).repositoryRoot!==context.repoRoot)throw Error("Exact repository Nx reset owner required");return withRequestedIssuedResourceControlV1(context.control,schema.$defs.Protocol.const.maximumMilliseconds,async phase=>{const result=await resetInactiveNxCacheNamespaceV1(context.invocation,phase);console.log(JSON.stringify(result));});}
const router=new ScriptRouter(import.meta.dir).register("reset",InactiveNxResetScript);
if(import.meta.main)await receiveScriptProcessInvocation(process.env,invocation=>runScriptMain(router,{invocation}));
