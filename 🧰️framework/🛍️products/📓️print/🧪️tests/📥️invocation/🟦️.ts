import type{AdapterContext}from"@semio-tech/framework/test/adapter";
import{withPrintExecutionV1}from"../../📦️artifacts/🎛️operation/🟦️.ts";
import{compileVizProbe,type ProbeRecord}from"../../🔨️modules/🧪️viz-probe/🟦️.ts";
/** 🧪️ Carries the real host context into one retained compiler scope without inventing authority from paths. */
export function compileAdapterVizProbeV1(context:AdapterContext,texPath:string,selection:Readonly<{caseName?:string;scenario?:string;sources?:readonly string[]}>):Promise<ProbeRecord[]>{const{invocation,phaseControl,repoRoot:workspace}=context,path=texPath,captured=Object.freeze({...selection,sources:selection.sources?Object.freeze([...selection.sources]):undefined});return withPrintExecutionV1({workspace,invocation,phaseControl,kind:"probe"},execution=>compileVizProbe(path,{execution,...captured}));}
