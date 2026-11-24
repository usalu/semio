import {expect,test}from"bun:test";
import {nativeBuildPassOffloadableV1,runPassesInJobSlotsV1,type PassJobSlotV1}from"../🟦️.ts";
import fixture from"./🧫️fixtures/🔣️.json";
type Row={id:string;target:string};
const flush=()=>new Promise<void>(resolve=>setImmediate(resolve));
async function simulate(passes:readonly Row[],jobs:number){
 const starts:string[]=[],pending:{id:string;finish:()=>void}[]=[];let running=0,maximumRunning=0;
 const done=runPassesInJobSlotsV1(passes,{jobs,check:()=>undefined,run:(pass:Row,slot:PassJobSlotV1)=>new Promise<void>(resolve=>{starts.push(pass.id+":"+slot);running++;maximumRunning=Math.max(maximumRunning,running);pending.push({id:pass.id,finish:()=>{running--;resolve();}});})});
 let settled=false;void done.then(()=>{settled=true;},()=>{settled=true;});
 for(let step=0;step<1000&&!settled;step++){await flush();const next=pending.shift();if(next)next.finish();}
 await done;
 return {starts,maximumRunning};
}
test("the job slot schedule of every vector is the one written down",async()=>{
 for(const row of fixture.cases)expect(await simulate(row.passes,row.jobs),row.name).toEqual(row.expect);
});
test("only wasm32 passes are offloadable",()=>{
 expect(nativeBuildPassOffloadableV1({target:"wasm32-wasip2"})).toBe(true);
 expect(nativeBuildPassOffloadableV1({target:"wasm32-unknown-unknown"})).toBe(true);
 expect(nativeBuildPassOffloadableV1({target:"aarch64-apple-darwin"})).toBe(false);
});
test("a stub job that blocks shows concurrent offloadable jobs, and the local slot is held by at most one",async()=>{
 const passes=Array.from({length:6},(_,index)=>({id:"p"+index,target:"wasm32-wasip2"}));
 const gates:(()=>void)[]=[],held:PassJobSlotV1[]=[];let local=0,maximumLocal=0,running=0,maximum=0;
 const done=runPassesInJobSlotsV1(passes,{jobs:4,check:()=>undefined,run:(_pass,slot)=>new Promise<void>(resolve=>{held.push(slot);running++;maximum=Math.max(maximum,running);if(slot==="local"){local++;maximumLocal=Math.max(maximumLocal,local);}gates.push(()=>{running--;if(slot==="local")local--;resolve();});})});
 await flush();expect(running).toBe(4);expect(held).toEqual(["local","fleet","fleet","fleet"]);
 gates.shift()!();await flush();expect(running).toBe(4);
 while(gates.length){gates.shift()!();await flush();}
 await done;expect(maximum).toBe(4);expect(maximumLocal).toBe(1);expect(held.length).toBe(6);
});
test("the first failure is thrown after every running pass settled, stops launching and reaches the driver at once",async()=>{
 const passes=Array.from({length:5},(_,index)=>({id:"p"+index,target:"wasm32-wasip2"})),started:string[]=[],failed:unknown[]=[];let finished=0,release=()=>{};
 const gate=new Promise<void>(resolve=>{release=resolve;});
 const done=runPassesInJobSlotsV1(passes,{jobs:3,check:()=>undefined,fail:error=>{failed.push(error);release();},run:async pass=>{started.push(pass.id);if(pass.id==="p1")throw Error("boom");await gate;finished++;}});
 await expect(done).rejects.toThrow("boom");
 expect(failed.length).toBe(1);expect(started).toEqual(["p0","p1","p2"]);expect(finished).toBe(2);
});
test("a failing check ends the launching, hands the error to the driver and still waits for the running passes",async()=>{
 const passes=Array.from({length:4},(_,index)=>({id:"p"+index,target:"wasm32-wasip2"}));let checks=0,finished=0,release=()=>{};const gate=new Promise<void>(resolve=>{release=resolve;}),failed:unknown[]=[];
 const done=runPassesInJobSlotsV1(passes,{jobs:4,check:()=>{if(++checks===3)throw Error("cancelled");},fail:error=>{failed.push(error);release();},run:async()=>{await gate;finished++;}});
 await expect(done).rejects.toThrow("cancelled");expect(failed.length).toBe(1);expect(finished).toBe(2);
});
