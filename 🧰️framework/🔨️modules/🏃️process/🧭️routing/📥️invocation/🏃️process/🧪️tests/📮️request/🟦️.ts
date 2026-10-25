import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import{AsyncLocalStorage}from"node:async_hooks";
import*as api from"../../🟦️.ts";

const envelope=(owner:string,detail:string)=>api.createScriptProcessEnvelope({version:1,owner,maximumElapsedMilliseconds:0},{fixture:detail},Date.now());
const environment=(value:api.ScriptProcessEnvelope)=>({[api.SCRIPT_PROCESS_INVOCATION_ENV]:JSON.stringify(value)});

/** 📮️ Pins the request law of a long-lived owner: a forwarded request envelope is a root of its own unless it is the owner's exact envelope. */
test("a forwarded request envelope is received beside, not beneath, a long-lived owner with another envelope",async()=>{
 const service=envelope("fixture:service","service"),request=envelope("fixture:request","request"),oracle=new Database(":memory:");
 try{
  await api.withScriptProcessEnvelope(service,async owner=>{
   await expect(api.receiveScriptProcessInvocation(environment(request),async()=>undefined)).rejects.toThrow("Nested Original process envelope must preserve its actual parent");
   const ambient=new AsyncLocalStorage<string>();
   const seen=await ambient.run("owner",()=>api.receiveScriptProcessRequestInvocationV1(environment(request),async received=>{expect(ambient.getStore()).toBeUndefined();
    const control=api.createIssuedResourceControlV1(received,"fixture:request",0);expect(received).not.toBe(owner);api.requireIssuedResourceControlInvocationV1(control,received);
    return{owner:received.policy.owner,detached:received!==owner};
   }));
   expect(ambient.run("owner",()=>ambient.getStore())).toBe("owner");
   const row=oracle.query("select json_extract(?1,'$.policy.owner') owner").get(JSON.stringify(request))as{owner:string};
   expect(seen).toEqual({owner:row.owner,detached:true});
   const same=await api.receiveScriptProcessRequestInvocationV1(environment(service),async received=>received===owner);
   expect(same).toBe(true);
   await api.withIssuedResourceControlV1(owner,"fixture:after",0,async control=>{api.requireIssuedResourceControlInvocationV1(control,owner);});
   await expect(api.receiveScriptProcessRequestInvocationV1({},async()=>undefined)).rejects.toThrow("process.routing.forwarded-request-invocation-required");
   await expect(api.receiveScriptProcessRequestInvocationV1({[api.SCRIPT_PROCESS_INVOCATION_ENV]:"{\"version\":1}"},async()=>undefined)).rejects.toThrow();
  });
 }finally{oracle.close();}
});

/** 📌️ Contract: work run under the received implementation pin is committed to the Sources it loaded through the shared pin context every module copy reads; the pin ends with the work. The reading side lives with the repository library (`🏗️build/📥️inputs/🔏️revision/🧪️tests/📌️received-pin`). */
test("a request run under the received implementation pin opens the shared pin context for the work and closes it afterwards",async()=>{
 const pinned=()=>(globalThis as unknown as Record<symbol,AsyncLocalStorage<{active:boolean}>|undefined>)[Symbol.for("semio.implementationPin.context.v1")]?.getStore()?.active===true;
 expect(pinned()).toBe(false);
 const inside=await api.withReceivedImplementationPinV1(async()=>{await Promise.resolve();return pinned();});
 expect(inside).toBe(true);expect(pinned()).toBe(false);
 await expect(api.withReceivedImplementationPinV1(async()=>{throw Error("failed body");})).rejects.toThrow("failed body");expect(pinned()).toBe(false);
});
