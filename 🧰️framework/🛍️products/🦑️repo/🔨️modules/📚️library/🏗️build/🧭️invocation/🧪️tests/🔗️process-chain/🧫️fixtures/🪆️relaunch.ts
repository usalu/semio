/** 🔗️ Fixture for build invocation process-chain. */
import{writeFileSync}from"node:fs";
import{adoptCapturedBuildCommandV1,runAdoptedCapturedBuildSourceV1}from"../../../../📥️source/🔏️closure/🚪️command/🟦️.ts";
import{receiveScriptProcessInvocation,createIssuedResourceControlV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 🪆️ Fixture entry of the process chain law: the first loader starts the second loader from its own snapshot; the second one only reports who it is. */
export async function runCapturedBuildSourceV1(args:readonly string[],proof:Parameters<typeof adoptCapturedBuildCommandV1>[0]):Promise<void>{
 proof.recheck();adoptCapturedBuildCommandV1(proof);
 const report={pid:process.pid,parent:process.ppid,sha256:proof.sha256,args};
 if(process.env.CHAIN_LAW_REPORT_FILE){writeFileSync(process.env.CHAIN_LAW_REPORT_FILE,JSON.stringify(report));if(process.env.CHAIN_LAW_SLEEP)await new Promise<void>(accept=>setTimeout(accept,120000));return;}
 await receiveScriptProcessInvocation(process.env,async original=>{
  const control=createIssuedResourceControlV1(original,"repository:command",0),file=process.env.CHAIN_LAW_SECOND_FILE!;
  let error:string|null=null;
  try{await runAdoptedCapturedBuildSourceV1([...args,"second"],original,control,{label:"build:graph-plan",cwd:process.cwd(),budgetMilliseconds:Number(process.env.CHAIN_LAW_BUDGET??60000),environment:{...process.env,CHAIN_LAW_REPORT_FILE:file},stdout:"stderr",onProgress:()=>undefined});}catch(failure){error=String((failure as Error).message);}
  console.log("CHAIN "+JSON.stringify({first:report,secondFile:file,error}));
 }).catch(()=>undefined);
}
