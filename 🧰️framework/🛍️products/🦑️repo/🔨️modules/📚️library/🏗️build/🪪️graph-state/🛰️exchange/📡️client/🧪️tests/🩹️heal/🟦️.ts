/** 🩹️ Contract: one slow answer does not close the fleet tier for the whole machine while other requests succeed, and a burst of reads never starts more than the declared requests at once (a request that waits for a slot has no deadline yet: its timeout starts when it is sent). A request that was sent after the marker was written and was answered lifts the marker; one that was sent before it does not.
 * @see ../../📡️client/🟦️.ts */
import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdtempSync,rmSync,statSync,utimesSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createGraphFleetClientV1} from "../../🟦️.ts";
import {GRAPH_STATE_POLICY_V1 as policy} from "../../../../🟦️.ts";

const scratch:string[]=[];afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const directoryOf=()=>{const path=mkdtempSync(join(tmpdir(),"heal-"));scratch.push(path);return path;};
const settingsOf=(stateDirectory:string)=>({endpoint:"http://graph.invalid",instance:"test",token:undefined,upload:false,stateDirectory});
const silent=(signal:AbortSignal):Promise<Response>=>new Promise((_,reject)=>{signal.addEventListener("abort",()=>reject(signal.reason),{once:true});});

test("a request answered after the marker was written lifts the marker at once, and a failure marks again",async()=>{
 const directory=directoryOf(),queue:("timeout"|"miss")[]=["timeout","miss","miss","timeout"];
 const client=createGraphFleetClientV1(settingsOf(directory),{fetch:((_url:string,init:{signal:AbortSignal})=>queue.shift()==="timeout"?silent(init.signal):Promise.resolve(new Response(null,{status:404}))) as unknown as typeof fetch});
 expect(await client.getResult("0".repeat(64))).toBe("down");
 expect(existsSync(join(directory,"down"))).toBe(true);expect(client.usable()).toBe(false);
 await Bun.sleep(20);
 expect(await client.getResult("1".repeat(64))).toBe("miss");
 expect(existsSync(join(directory,"down")),"the answer of a request sent after the marker lifts it").toBe(false);expect(client.usable()).toBe(true);
 expect(await client.getResult("2".repeat(64))).toBe("miss");
 expect(await client.getResult("3".repeat(64))).toBe("down");expect(client.usable()).toBe(false);
},30000);

test("an answer to a request that was sent before the marker does not lift it",async()=>{
 const directory=directoryOf();let release!:()=>void;const gate=new Promise<void>(done=>{release=done;});
 const client=createGraphFleetClientV1(settingsOf(directory),{fetch:((_url:string,init:{signal:AbortSignal})=>init.signal.aborted?Promise.reject(init.signal.reason):gate.then(()=>new Response(null,{status:404}))) as unknown as typeof fetch});
 const early=client.getResult("4".repeat(64));
 await Bun.sleep(30);
 const stamp=join(directory,"down"),{writeFileSync}=await import("node:fs");writeFileSync(stamp,"x");const later=new Date(Date.now()+5);utimesSync(stamp,later,later);
 release();expect(await early).toBe("miss");
 expect(existsSync(stamp),"the marker is newer than the request that was answered").toBe(true);expect(statSync(stamp).size).toBe(1);
},30000);

test("a burst of 120 reads starts at most readsInFlightMaximum requests at once, and none of them times out while it waits for a slot",async()=>{
 const directory=directoryOf();let flying=0,most=0,sent=0;
 const slow=policy.timeouts.lookupMilliseconds/4;
 const client=createGraphFleetClientV1(settingsOf(directory),{fetch:(async(_url:string,init:{signal:AbortSignal})=>{sent++;flying++;most=Math.max(most,flying);try{await Bun.sleep(slow);init.signal.throwIfAborted();return new Response(null,{status:404});}finally{flying--;}}) as unknown as typeof fetch});
 const answers=await Promise.all(Array.from({length:120},(_all,at)=>client.getResult(at.toString(16).padStart(64,"0"))));
 expect(answers.every(answer=>answer==="miss"),"every read was answered although the burst needs 5 rounds of the 4 times the slack").toBe(true);
 expect(sent).toBe(120);expect(most).toBe(policy.requests.readsInFlightMaximum);expect(existsSync(join(directory,"down"))).toBe(false);
},60000);

test("a cancelled read that waits for a slot leaves the queue and sends nothing",async()=>{
 const directory=directoryOf();let sent=0;
 const client=createGraphFleetClientV1(settingsOf(directory),{fetch:((_url:string,init:{signal:AbortSignal})=>{sent++;return silent(init.signal);}) as unknown as typeof fetch});
 const holders=Array.from({length:policy.requests.readsInFlightMaximum},(_all,at)=>client.getResult(at.toString(16).padStart(64,"0")));
 await Bun.sleep(20);
 const controller=new AbortController(),waiting=client.getResult("f".repeat(64),controller.signal).then(()=>"answered",error=>String(error?.name??error));
 await Bun.sleep(20);controller.abort(new DOMException("stop","AbortError"));
 expect(await waiting).toBe("AbortError");expect(sent).toBe(policy.requests.readsInFlightMaximum);
 await Promise.all(holders);
},30000);
