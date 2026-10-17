import {acquireCompilerCapacityLeaseV1,scriptCompilerAdmissionControlV1} from "../../../🟦️.ts";
import {receiveScriptProcessInvocation} from "../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import fixture from "../🧫️fixtures/🔣️.json" with {type:"json"};

await receiveScriptProcessInvocation(process.env,async original=>{
 const control=scriptCompilerAdmissionControlV1(original,"test:unpaid-capacity-child",fixture.maximumAdmissionMilliseconds);
 let entered=false,parallelism:number|null=null,outcome="refused";
 try{const lease=await acquireCompilerCapacityLeaseV1({directory:process.argv[2]!,control,environment:process.env,request:fixture.request as import("../../../🟦️.ts").CompilerCapacityRequestV1});try{entered=true;parallelism=lease.parallelism;outcome="granted";}finally{lease.release();}}
 catch{}
 process.stdout.write(JSON.stringify({version:1,entered,outcome,parallelism}));
});
