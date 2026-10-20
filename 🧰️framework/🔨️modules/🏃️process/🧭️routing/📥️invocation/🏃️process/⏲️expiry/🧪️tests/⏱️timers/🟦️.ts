import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{createRequire}from"node:module";
import{Database}from"bun:sqlite";
import schema from"../../../🧬️schema/🔣️.json";
import processBudgetSchema from"../../../../../../⏱️budget/🧬️schema/🔣️.json";
import cleanupSchema from"../../../🧬️schema/🧹️cleanup.json";
import corpus from"./🔣️.json";
import{createScriptProcessEnvelope,withScriptProcessEnvelope,createIssuedResourceControlV1,withAcceptedPhysicalCleanupV1,readAcceptedPhysicalCleanupV1,beginAcceptedPhysicalCleanupV1}from"../../../🟦️.ts";

test("process birth and physical timers preserve fixed long deadlines and retire stale callbacks",()=>{
 const require=createRequire(import.meta.url),ts=require("typescript"),source=new URL("../../../🟦️.ts",import.meta.url),file=ts.createSourceFile(source.pathname,readFileSync(source,"utf8"),ts.ScriptTarget.Latest,true),declaration=file.statements.find((node:any)=>ts.isFunctionDeclaration(node)&&node.name?.text==="startProcessDeadlineV1"),validate=new(require("ajv").default)({strict:true}).compile(schema.definitions.DeadlineTimerCase),database=new Database(":memory:");
 expect(declaration).toBeDefined();const implementation=require("esbuild").transformSync(declaration.getText(file),{loader:"ts",format:"cjs"}).code;
 try{for(const row of corpus.cases){
  expect(validate(row),JSON.stringify(validate.errors)).toBe(true);let index=0,expirations=0,unrefs=0,next=0;const delays:number[]=[],pending=new Map<number,()=>void>(),create=new Function("processBudgetSchema","setTimeout","clearTimeout",implementation+";return startProcessDeadlineV1;")(processBudgetSchema,(callback:()=>void,delay:number)=>{delays.push(delay);const id=++next;pending.set(id,callback);return{id,unref:()=>unrefs++};},(handle:{id:number}|undefined)=>{if(handle)pending.delete(handle.id);}),stop=create(()=>Math.min(...row.remaining[index]!),()=>expirations++,row.unref);
  for(let check=0;check<row.remaining.length-1;check++){const entry=pending.entries().next().value as[number,()=>void]|undefined;if(entry)pending.delete(entry[0]);if("cancelBeforeCheck"in row&&row.cancelBeforeCheck===check)stop();index++;entry?.[1]();}
  stop();stop();expect(pending.size).toBe(0);expect(delays).toEqual(row.expectedDelays);expect(expirations).toBe(row.expectedExpirations);expect(unrefs).toBe(row.unref?delays.length:0);
  const cancelled=row.cancelBeforeCheck===undefined?null:row.cancelBeforeCheck+1,oracle=database.query("WITH readings AS (SELECT CAST(key AS INTEGER) ordinal,min(json_extract(value,'$[0]'),json_extract(value,'$[1]')) remaining FROM json_each(?1)) SELECT json_group_array(delay) delays FROM (SELECT min(?3,CAST(remaining AS INTEGER)+(remaining>CAST(remaining AS INTEGER))) delay FROM readings r WHERE remaining>0 AND ordinal<coalesce(?2,2147483647) AND NOT EXISTS(SELECT 1 FROM readings p WHERE p.ordinal<r.ordinal AND p.remaining<=0) ORDER BY ordinal)").get(JSON.stringify(row.remaining),cancelled,processBudgetSchema.$defs.TimerMaximumMilliseconds.const)as{delays:string},expiry=database.query("SELECT CASE WHEN EXISTS(SELECT 1 FROM json_each(?1) WHERE min(json_extract(value,'$[0]'),json_extract(value,'$[1]'))<=0 AND CAST(key AS INTEGER)<coalesce(?2,2147483647)) THEN 1 ELSE 0 END value").get(JSON.stringify(row.remaining),cancelled)as{value:number};
  expect(delays).toEqual(JSON.parse(oracle.delays));expect(expirations).toBe(expiry.value);
 }}finally{database.close();}
 expect(cleanupSchema.properties.reserveMilliseconds.maximum).toBeLessThanOrEqual(processBudgetSchema.$defs.TimerMaximumMilliseconds.const);
});

test("actual long finite Original and physical cleanup do not expire at timer overflow",async()=>{
 for(const maximumElapsedMilliseconds of[2147483647,2147483648,9007199254740991-Date.now()-10000]){
  await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"fixture:long-process-timers",maximumElapsedMilliseconds},{fixture:"long-process-timers"},Date.now()),async invocation=>{
   const body=createIssuedResourceControlV1(invocation,"fixture:long-physical",0);
   await withAcceptedPhysicalCleanupV1(body,{version:1,kind:"artifact-scratch",resource:"fixture:long-physical",reserveMilliseconds:20},async(token,control)=>{
    const facts=readAcceptedPhysicalCleanupV1(token);await new Promise<void>(done=>setTimeout(done,40));expect(invocation.control.signal.aborted).toBe(false);expect(control.signal.aborted).toBe(false);expect(facts.cleanupSignal.aborted).toBe(false);await beginAcceptedPhysicalCleanupV1(token);await new Promise<void>(done=>setTimeout(done,5));expect(facts.cleanupSignal.aborted).toBe(false);
   });
  });
 }
 expect(()=>createScriptProcessEnvelope({version:1,owner:"fixture:unrepresentable-timer",maximumElapsedMilliseconds:9007199254740991},{},Date.now())).toThrow();
 console.log("[DEBUG] process birth/physical timer laws: 8 neutral schedules and 3 actual long finite cleanups");
},5000);

test("actual body drain retires its control timer before terminal cleanup",async()=>{
 const realTimeout=globalThis.setTimeout,realClear=globalThis.clearTimeout,scheduled:{handle:ReturnType<typeof setTimeout>;callback:()=>void}[]=[],cleared=new Set<ReturnType<typeof setTimeout>>(),require=createRequire(import.meta.url),database=new Database(":memory:");
 expect(new(require("ajv").default)({strict:true}).validate(schema.definitions.PhysicalBodyTimerRetirement,corpus.bodyRetirement)).toBe(true);
 try{
  globalThis.setTimeout=((callback:()=>void,milliseconds:number)=>{const handle=realTimeout(callback,milliseconds);scheduled.push({handle,callback});return handle;})as typeof setTimeout;
  globalThis.clearTimeout=((handle:ReturnType<typeof setTimeout>)=>{cleared.add(handle);realClear(handle);})as typeof clearTimeout;
  await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"fixture:retired-body-timer",maximumElapsedMilliseconds:2147483648},{},Date.now()),invocation=>withAcceptedPhysicalCleanupV1(createIssuedResourceControlV1(invocation,"fixture:retired-body",0),{version:1,kind:"artifact-scratch",resource:"fixture:retired-body",reserveMilliseconds:20},async(token,body)=>{
   const count=scheduled.length;await beginAcceptedPhysicalCleanupV1(token);const retired=scheduled.filter(row=>cleared.has(row.handle));for(const row of retired)row.callback();
   const actual={scheduled:count,bodyRetired:retired.length,wholeRetained:count-retired.length,staleCallbackAborts:body.signal.aborted||invocation.control.signal.aborted||readAcceptedPhysicalCleanupV1(token).cleanupSignal.aborted,staleCallbackRearms:scheduled.length!==count};expect(actual).toEqual(corpus.bodyRetirement);
   const oracle=database.query("SELECT json_array_length(?1) scheduled,(SELECT count(*) FROM json_each(?1) WHERE json_extract(value,'$.retired')=1) retired").get(JSON.stringify(scheduled.map(row=>({retired:cleared.has(row.handle)}))))as{scheduled:number;retired:number};expect(oracle.scheduled).toBe(actual.scheduled);expect(oracle.retired).toBe(actual.bodyRetired);
  }));
  expect(scheduled.every(row=>cleared.has(row.handle))).toBe(true);
 }finally{globalThis.setTimeout=realTimeout;globalThis.clearTimeout=realClear;for(const row of scheduled)realClear(row.handle);database.close();}
});
