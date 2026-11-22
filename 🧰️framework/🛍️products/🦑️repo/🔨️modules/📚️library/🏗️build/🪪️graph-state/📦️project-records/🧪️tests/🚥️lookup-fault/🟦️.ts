/** 🚥️ Sends the authored fault responses through the real graph exchange client and owner take scope. */
import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import{createHash}from"node:crypto";
import{mkdtempSync,rmSync,mkdirSync,writeFileSync,utimesSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import fixture from"./🔣️.json"with{type:"json"};
import{createProjectFleetPortV1,projectRecordStateV1,runTakeLookV1,type ProjectRecordInfoV1,type TakeLookV1}from"../../🟦️.ts";
import{graphFleetClientV1,packGraphDocumentV1,type GraphEntryNoteV1}from"../../../🛰️exchange/🟦️.ts";
import graphSchema from"../../../🧬️schema/🔣️.json"with{type:"json"};
import{REAPI_V1,encodeProtobufV1}from"../../../../🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
import{createAdoptCountersV1,recordTakeLookFailureV1,takePassV1}from"../../../../🕸️graph/📥️inference/📦️project-nodes/📥️take/🟨️.mjs";

test("real owner take preserves fallback while wire fault, miss and cancellation reach distinct counters",async()=>{
 const root=mkdtempSync(join(tmpdir(),"fo-")),db=new Database(":memory:"),counters=createAdoptCountersV1();
 let imports=0;
 const info:ProjectRecordInfoV1={cachePath:join(root,"p.json"),fleet:"owner",keyText:"fixture",exportRecord:()=>undefined,import:async()=>{imports++;return true;}};
 const state=projectRecordStateV1({root,fleet:info.fleet,keyText:info.keyText,platform:"fixture"});
 try{for(const [at,row] of fixture.cases.entries()){
  const directory=join(root,String(at)),controller=new AbortController(),cancel=Error("cancelled fixture");
  if(row.wire==="breaker"){mkdirSync(directory);writeFileSync(join(directory,"down"),"");}
  const packed=packGraphDocumentV1({name:row.wire==="no-record"?"aux":"record",bytes:Buffer.from("payload invalid JSON")})[0]!;
  const bytes=row.wire==="brotli"?Buffer.alloc(100,255):Buffer.from(packed.bytes),bad=Buffer.from(bytes);bad[0]^=1;
  const blob={name:packed.name,hash:createHash("sha256").update(bytes).digest("hex"),size:bytes.length};
  const note:GraphEntryNoteV1={version:1,kind:state.frame.kind,digest:row.wire==="wrong-digest"?"f".repeat(64):state.digest,platform:state.frame.platform,producer:"fixture",createdMs:1,documents:[blob]};
  const entry=encodeProtobufV1(REAPI_V1.actionResult,{stdout_raw:Buffer.from(JSON.stringify(note)),output_files:[{path:blob.name,digest:{hash:blob.hash,size_bytes:blob.size}}]});
  let requests=0;
  const server=Bun.serve({port:0,hostname:"127.0.0.1",fetch(request){
   requests++;
   if(new URL(request.url).pathname.includes("/ac/")){
    if(row.wire==="cancel"){controller.abort(cancel);return new Response(null,{status:503});}
    if(row.wire==="miss")return new Response(null,{status:404});
    if(row.wire==="denied")return new Response(null,{status:403});
    if(row.wire==="transport")return new Response(null,{status:503});
    return new Response(row.wire==="bad-note"?"not an entry":Buffer.from(entry));
   }
   if(row.wire==="missing-blob")return new Response(null,{status:404});
   return new Response(row.wire==="bad-blob"?bad:bytes);
  }});
  try{
   const client=graphFleetClientV1({endpoint:server.url.origin,instance:"main",token:undefined,upload:false,stateDirectory:directory});
   const port=createProjectFleetPortV1({root,environment:{},client,platform:"fixture"})!;
   let observed:TakeLookV1|undefined,reason:unknown;
   try{await runTakeLookV1(async look=>{observed=look;expect((await port.gate(info)).seeded,row.wire).toBe(false);},controller.signal);}
   catch(error){reason=error;}
   if(row.wire==="cancel")expect(reason).toBe(cancel);
   else expect(reason,row.wire).toBeUndefined();
   expect(requests,row.wire).toBe(row.requests);
   const fallback=recordTakeLookFailureV1(Object.assign(Error("fixture"),{code:"BUILD_TAKE_MISS",outcome:observed?.outcome,lookupFault:observed?.fault}),counters,controller.signal.aborted);
   expect(fallback,row.wire).toBe(row.outcome);
   expect(observed?.fault,row.wire).toBe(row.reason==="cancelled"?undefined:row.reason??undefined);
  }finally{server.stop(true);}
 }
 const expected=db.query<{reason:string;count:number},[string]>("select json_extract(value,'$.reason') reason,count(*) count from json_each(?1) where json_type(value,'$.reason')='text' group by reason").all(JSON.stringify(fixture.cases));
 expect(counters.lookups()).toEqual({misses:1,cancelled:expected.find(row=>row.reason==="cancelled")?.count,faults:{...Object.fromEntries(graphSchema.definitions.LookupFault.enum.map(key=>[key,0])),...Object.fromEntries(expected.filter(row=>row.reason!=="cancelled").map(row=>[row.reason,row.count]))}});
 const outcomes=db.query<{kind:string;count:number},[string]>("select json_extract(value,'$.reported') kind,count(*) count from json_each(?1) group by kind").all(JSON.stringify(fixture.cases));
 expect(counters.counts()).toMatchObject({...Object.fromEntries(outcomes.filter(row=>row.kind!=="cancelled").map(row=>[row.kind,row.count])),stale:0});
 expect(imports).toBe(0);
 }finally{db.close();rmSync(root,{recursive:true,force:true});}
},30000);

test("an open owner-cache breaker avoids repeated transport and recovers on the ordinary path",async()=>{
 const root=mkdtempSync(join(tmpdir(),"look-breaker-")),db=new Database(":memory:");
 const checked:typeof fixture.breakerCases=[];
 try{for(const row of fixture.breakerCases){
  const directory=join(root,row.name),marker=join(directory,"down"),counts=createAdoptCountersV1(),events:string[]=[];
  mkdirSync(directory);
  if(row.initial==="open")writeFileSync(marker,"");
  const client=graphFleetClientV1({endpoint:"http://127.0.0.1:1",instance:"main",token:"test-reader",upload:true,stateDirectory:directory},{fetch:(async(address,options)=>{
   expect(new Headers(options?.headers).get("authorization")).toBe("Bearer test-reader");
   const path=new URL(String(address)).pathname;
   if(path.includes("/claim/")){events.push("claim");return Response.json({state:"full"},{status:503});}
   events.push("request");
   if(row.initial==="transport"&&events.filter(event=>event==="request").length===1)return new Response(null,{status:503});
   if(row.initial==="malformed"&&events.filter(event=>event==="request").length===1)return new Response("invalid ActionResult");
   return new Response(null,{status:404});
  }) as typeof fetch});
  const sleep=async()=>{events.push("pause");},port=createProjectFleetPortV1({root,environment:{},client,platform:"fixture",sleep,random:()=>0})!;
  const info:ProjectRecordInfoV1={cachePath:join(directory,"p.json"),fleet:"owner",keyText:"fixture",exportRecord:()=>undefined,import:async()=>{throw Error("No bytes to adopt");}};
  if(row.lane==="take"){
   const outcomes=await takePassV1({names:["owner"],sleep,attempt:()=>runTakeLookV1(async looked=>{
    events.push("look");
    expect((await port.gate(info)).seeded).toBe(false);
    return recordTakeLookFailureV1(Object.assign(Error("fixture"),{code:"BUILD_TAKE_MISS",outcome:looked.outcome,lookupFault:looked.fault}),counts,false);
   })});
   expect(outcomes.get("owner")).toBe("miss");
  }else{
   expect((await port.gate(info)).seeded).toBe(false);
  }
  checked.push({...row,events:[...events]});
  if(row.reopens){
   expect(client.usable()).toBe(false);
   const old=new Date(0);
   utimesSync(marker,old,old);
   expect(client.usable()).toBe(true);
   const before=events.length;
   expect((await port.gate(info)).seeded).toBe(false);
   expect(events.slice(before)).toEqual(["request","claim"]);
  }
 }
 const tally=(input:typeof fixture.breakerCases)=>db.query("SELECT json_extract(entry.value,'$.name') AS name, sum(ifnull(event.value='request',0)) AS requests, sum(ifnull(event.value='pause',0)) AS pauses FROM json_each(?1) AS entry LEFT JOIN json_each(entry.value,'$.events') AS event ON 1 GROUP BY name ORDER BY name").all(JSON.stringify(input));
 expect(checked).toEqual(fixture.breakerCases);
 expect(tally(checked)).toEqual(tally(fixture.breakerCases));
 }finally{db.close();rmSync(root,{recursive:true,force:true});}
},30000);
