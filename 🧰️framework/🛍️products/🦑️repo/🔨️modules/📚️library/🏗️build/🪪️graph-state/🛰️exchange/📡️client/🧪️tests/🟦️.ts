import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {createGraphFleetClientV1} from "../🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

const hash=(value:string)=>createHash("sha256").update(value).digest("hex"),encoder=new TextEncoder();
const client=(request:typeof fetch)=>createGraphFleetClientV1({endpoint:"http://graph.invalid",instance:"test",token:undefined,upload:false,stateDirectory:"/nonexistent"},{fetch:request});

for(const row of fixture.cases)test(row.name,async()=>{
 let requests=0,chunksRead=0,cancelled=false,fault:string|null=null;
 const graph=client((async()=>{
  requests++;
  return new Response(new ReadableStream<Uint8Array>({
   pull(stream){if(chunksRead===row.chunks.length)stream.close();else stream.enqueue(encoder.encode(row.chunks[chunksRead++]!));},
   cancel(){cancelled=true;}
  },{highWaterMark:0}),{headers:row.headers as Record<string,string>});
 }) as typeof fetch);
 const result=await graph.getBlob(hash(row.expectedBody),row.size,undefined,reason=>{fault=reason;});
 expect({bytes:result?.byteLength??null,fault,requests,chunksRead,cancelled}).toEqual(row.expected);
 if(result!==undefined)expect(new TextDecoder().decode(result)).toBe(row.expectedBody);
});

test("the caller aborts a read that is waiting on a body chunk",async()=>{
 const controller=new AbortController(),reason=new Error("cancel graph read");
 let began!:()=>void,cancelled=false;
 const reading=new Promise<void>(resolve=>{began=resolve;});
 const graph=client((async(_url,init)=>{
  expect(init?.signal?.aborted).toBe(false);
  return new Response(new ReadableStream<Uint8Array>({pull(){began();return new Promise<void>(()=>{});},cancel(){cancelled=true;}},{highWaterMark:0}));
 }) as typeof fetch);
 const result=graph.getBlob(hash("a"),1,controller.signal);
 await reading;
 controller.abort(reason);
 await expect(result).rejects.toBe(reason);
 expect(cancelled).toBe(true);
});

test("the request deadline cancels a stalled response body",async()=>{
 let cancelled=false,fault:string|undefined;
 const graph=client((async()=>new Response(new ReadableStream<Uint8Array>({
  pull(){return new Promise<void>(()=>{});},
  cancel(){cancelled=true;}
 },{highWaterMark:0}))) as typeof fetch);
 expect(await graph.getBlob(hash("a"),1,undefined,reason=>{fault=reason;})).toBeUndefined();
 expect(fault).toBe("transport");
 expect(cancelled).toBe(true);
},30_000);

test("a failed stream maps to transport",async()=>{
 const graph=client((async()=>new Response(new ReadableStream({pull(stream){stream.error(new Error("read failure"));}},{highWaterMark:0}))) as typeof fetch);
 let fault:string|undefined;
 expect(await graph.getBlob(hash("a"),1,undefined,reason=>{fault=reason;})).toBeUndefined();
 expect(fault).toBe("transport");
});
