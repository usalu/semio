/** 🔎️ Contract: a refusal of the shared content store names the reason the store gave, bounded, so a failing upload can be diagnosed from the log line alone. */
import {expect,test} from "bun:test";
import {RemoteExecutionContentErrorV1,httpRemoteExecutionContentV1} from "../../🗄️content/🟦️.ts";

const HASH="a".repeat(64);
const storeOf=(status:number,body:string)=>httpRemoteExecutionContentV1({endpoint:"http://store.invalid",retries:0,fetch:(async()=>new Response(body,{status}))as unknown as typeof fetch});
const refusal=async(run:()=>Promise<unknown>)=>{try{await run();}catch(error){return error as RemoteExecutionContentErrorV1;}throw new Error("a refusal was expected");};

test("a refused action result write names status, address and the reason of the store",async()=>{
 const error=await refusal(()=>storeOf(400,"Invalid action result: missing blob 7f").writeActionResult("",HASH,new Uint8Array([1])));
 expect(error).toBeInstanceOf(RemoteExecutionContentErrorV1);
 expect(error.code).toBe(3);
 expect(error.message).toBe(`Content store answered 400 for ac/${HASH}: Invalid action result: missing blob 7f`);
});

test("a refused blob write names the reason, long and multi-line reasons are collapsed and bounded",async()=>{
 const digest={hash:HASH,size:1};
 const error=await refusal(()=>storeOf(400,"Digest mismatch\n\n"+"x".repeat(1000)).write("",digest,new Uint8Array([1])));
 expect(error.message.startsWith(`Content store answered 400 for cas/${HASH}: Digest mismatch xxx`)).toBe(true);
 expect(error.message.length).toBeLessThanOrEqual(`Content store answered 400 for cas/${HASH}: `.length+200);
 const streamed=await refusal(()=>storeOf(507,"quota").write("",digest,(async function*(){yield new Uint8Array([1]);})()));
 expect(streamed.code).toBe(8);expect(streamed.message.endsWith(": quota")).toBe(true);
});

test("an accepted write and an empty refusal body keep the plain messages",async()=>{
 await storeOf(200,"").writeActionResult("",HASH,new Uint8Array([1]));
 const error=await refusal(()=>storeOf(403,"").writeActionResult("",HASH,new Uint8Array([1])));
 expect(error.message).toBe(`Content store answered 403 for ac/${HASH}`);
});
