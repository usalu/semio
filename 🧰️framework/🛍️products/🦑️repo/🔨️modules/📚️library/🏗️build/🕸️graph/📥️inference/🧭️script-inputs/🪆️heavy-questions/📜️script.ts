import{writeFileSync}from"node:fs";
import{HEAVY_QUESTION_POLICY_V1}from"./🟨️.mjs";
import{openPlannerDeriverV1}from"../../../../🧭️invocation/🕸️graph/🧠️memo/🧮️derive/🟦️.ts";

/** 🪆️ The child of one heavy derived question: opens the planner's deriver, answers the question read from stdin, writes the answer to the file named by the parent and stops at its resident memory cap. */
const[root,output]=process.argv.slice(2),limit=Number(process.env[HEAVY_QUESTION_POLICY_V1.environment])*1048576;
setInterval(()=>{if(process.memoryUsage().rss>limit){process.stderr.write("resident memory cap of "+process.env[HEAVY_QUESTION_POLICY_V1.environment]+" MB exceeded\n");process.exit(137);}},HEAVY_QUESTION_POLICY_V1.pollMilliseconds).unref();
const request=JSON.parse(await Bun.stdin.text()),derive=await openPlannerDeriverV1(root!);
if(request.prime){
 const scriptInputs=await import("../🟨️.mjs"),{observeInferenceSnapshotV1}=await import("../../../../👁️observation/🟨️.mjs");
 scriptInputs.bindClosureStoreV1(root!,request.prime.storeDirectory,true);
 writeFileSync(output!,JSON.stringify(await observeInferenceSnapshotV1(()=>scriptInputs.primeSharedClosuresV1(root!,(derive as any).scripts))));
}else{
 const answer=await derive(request.row);
 writeFileSync(output!,JSON.stringify({fact:answer.fact,rows:answer.rows}));
}
process.exit(0);
