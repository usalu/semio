import{join}from"node:path";
import{existsSync}from"node:fs";
import{receiveScriptProcessInvocation,createIssuedResourceControlV1,withRequestedIssuedResourceControlV1}from"@semio-tech/process/routing";
import{withCmakeInputsV1,cmakeInputFactsV1,type CmakeInputsV1}from"../../📥️inputs/🟦️.ts";
import{runCmakeOwnedStageV1}from"../../🎬️stage/🟦️.ts";

/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */
export async function runScriptProgram(): Promise<void> {
  const root=process.argv[2],kind=process.argv[3];if(!root||!['error','undefined','cancel'].includes(kind??''))throw new Error("Closed CMake ordinary failure required");const primary=kind==="undefined"?undefined:Error("Exact ordinary CMake primary"),cancel=new AbortController();let escaped:CmakeInputsV1|undefined,late=false;
  try{await receiveScriptProcessInvocation(process.env,async original=>{const control=createIssuedResourceControlV1(original,"cmake:failure",10000);await withRequestedIssuedResourceControlV1(control,0,phase=>withCmakeInputsV1(new Map([["source/input.txt",join(root,"input.txt")]]),phase,async(inputs,actual)=>{escaped=inputs;const ready=join(root,"ready"),command=kind==="cancel"?"require('node:fs').writeFileSync(process.argv[1],'ready');setInterval(()=>{},1000)":"console.log('settled')",work=runCmakeOwnedStageV1({kind:"build",directory:root,commands:[{program:"node",arguments:["-e",command,ready],cwd:root} ]},inputs,actual,process.env,async(_rows,consumer)=>{void withRequestedIssuedResourceControlV1(consumer,0,async child=>{await child.yieldContinuation();cmakeInputFactsV1(inputs);late=true;});throw primary;});if(kind==="cancel")void withRequestedIssuedResourceControlV1(actual,0,async child=>{while(!existsSync(ready))await child.yieldContinuation();cmakeInputFactsV1(inputs);late=true;cancel.abort(primary);});await work;}),cancel.signal);});throw new Error("Failed CMake Original unexpectedly succeeded");}catch(error){let retired=false;try{if(escaped)cmakeInputFactsV1(escaped);}catch{retired=true;}console.log(JSON.stringify({kind,exact:Object.is(error,primary),late,retired}));process.exitCode=1;}
}
