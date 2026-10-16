import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{receiveScriptProcessInvocation,createIssuedResourceControlV1,createRequestedIssuedResourceControlV1,withIssuedResourceControlV1,withIssuedResourceOperationV1,withRequestedIssuedResourceControlV1,withScriptInvocationSignalV1,withScriptProcessEnvelope,readIssuedScriptProcessEnvelopeV1,requireIssuedResourceControlV1,requireIssuedScriptInvocationV1}from"../../🟦️.ts";
import type{ScriptInvocation}from"../../../🟦️.ts";
import{captureOwnedCommandResultV1}from"../../../../../🎛️owned-execution/🟦️.ts";
import corpus from"./🔣️.json";
import schema from"../🧬️schema/🔣️.json";
const selected=process.env.SEMIO_FAILED_ADMISSION_CASE;
if(selected===undefined)test("neutral accepted failure closes births while genuine accepted siblings drain",async()=>receiveScriptProcessInvocation(process.env,async invocation=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);const db=new Database(":memory:"),control=createIssuedResourceControlV1(invocation,"test:failed-admission-children",30000);
 try{for(const[index,row]of corpus.cases.entries()){const result=await captureOwnedCommandResultV1(process.execPath,["test",import.meta.path],process.cwd(),{control,env:{...process.env,SEMIO_FAILED_ADMISSION_CASE:String(index)},output:{maximumBytes:32768,maximumLines:256}});expect(result.status,result.stdout+result.stderr).toBe(0);const records=result.stdout.split(/\r?\n/u).filter(line=>line.startsWith("actual-failed-admission:"));expect(records).toHaveLength(1);const actual=JSON.parse(records[0]!.slice("actual-failed-admission:".length))as{newEffects:number;siblingCompletions:number;refused:number;preserved:boolean;waited:boolean;retired:boolean};const oracle=db.query("SELECT CASE WHEN ? IN ('error','undefined') THEN 0 ELSE 1 END AS effects, 1 AS completed").get(row.failure)as{effects:number;completed:number};expect(actual.newEffects).toBe(oracle.effects);expect(actual.newEffects).toBe(row.expectedNewEffects);expect(actual.siblingCompletions).toBe(oracle.completed);expect(actual.siblingCompletions).toBe(row.expectedSiblingCompletions);expect(actual.refused).toBe(5);expect(actual.preserved).toBe(true);expect(actual.waited).toBe(true);expect(actual.retired).toBe(true);}
 console.log("[DEBUG] Actual failed Original children closed new births, retained accepted siblings and preserved Error or undefined against Ajv and SQLite");
 }finally{db.close();}
}),45000);
else test("actual failed Original stops admission before its callback returns",async()=>{
 const index=Number(selected),row=corpus.cases[index];if(!Number.isInteger(index)||!row)throw Error("Closed actual failed admission case required");const marker=row.failure==="undefined"?undefined:Error("actual accepted operation failure");let release!:()=>void,ready!:()=>void,birth!:ScriptInvocation,newEffects=0,siblingCompletions=0,refused=0,caught=false,failure:unknown,settled=false;
 const callbackReady=new Promise<void>(done=>{ready=done;}),run=async(invocation:ScriptInvocation)=>{birth=invocation;await withIssuedResourceControlV1(invocation,"test:failed-admission",20000,async control=>{
  void withIssuedResourceOperationV1(control,async()=>{await new Promise<void>(done=>{release=done;});requireIssuedResourceControlV1(control);siblingCompletions++;});
  try{await withIssuedResourceOperationV1(control,async()=>{throw marker;});}catch(error){expect(error).toBe(marker);}
  const attempts=[()=>withIssuedResourceOperationV1(control,async()=>{newEffects++;}),()=>withRequestedIssuedResourceControlV1(control,0,async()=>{newEffects++;}),()=>createRequestedIssuedResourceControlV1(control,0),()=>createIssuedResourceControlV1(invocation,"test:late-birth",0),()=>withScriptProcessEnvelope(readIssuedScriptProcessEnvelopeV1(invocation),async()=>{newEffects++;})];
  try{for(const attempt of attempts){try{await attempt();}catch{refused++;}}requireIssuedResourceControlV1(control);}finally{ready();}
 });};
 const parent=receiveScriptProcessInvocation(process.env,invocation=>row.scope==="signal"?withScriptInvocationSignalV1(invocation,new AbortController().signal,run):run(invocation));void parent.then(()=>{settled=true;},error=>{caught=true;failure=error;settled=true;});let waited=false;
 try{await Promise.race([callbackReady,parent]);await new Promise<void>(done=>setImmediate(done));waited=!settled;expect(waited).toBe(true);}finally{release?.();await parent.catch(()=>{});}
 let retired=false;try{requireIssuedScriptInvocationV1(birth);}catch{retired=true;}
 const actual={newEffects,siblingCompletions,refused,preserved:caught&&failure===marker,waited,retired};console.log("actual-failed-admission:"+JSON.stringify(actual));expect(actual).toEqual({newEffects:row.expectedNewEffects,siblingCompletions:row.expectedSiblingCompletions,refused:5,preserved:true,waited:true,retired:true});
},30000);
