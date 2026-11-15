/** 🛡️ Contract: an owner that comes back from a worker is admitted against the headroom again, with the cost recorded for its own key, and a learned peak of a kind never shrinks to the share an owner took while others ran. */
import{expect,test}from"bun:test";
import{GRAPH_MEMORY_POLICY_V1,createMemoryGovernorV1}from"../../🟨️.mjs";

const MEGABYTE=1048576,fast={...GRAPH_MEMORY_POLICY_V1,admitMinimumMilliseconds:0,pollMilliseconds:1},sleep=()=>new Promise<void>(resolve=>setTimeout(resolve,1));

test("owners returning from workers honour the headroom with the recorded cost of their own key",async()=>{
 let inProcess=0,peak=0;
 const governor=createMemoryGovernorV1({workers:8,headroom:()=>1000*MEGABYTE-inProcess*400*MEGABYTE,estimate:()=>400*MEGABYTE,policy:fast,sleep});
 await Promise.all(Array.from({length:8},(_,index)=>governor.run(async handle=>{
  await handle.away(()=>Bun.sleep(10));
  inProcess++;peak=Math.max(peak,inProcess);
  await Bun.sleep(15);
  inProcess--;
 },{kind:"language:cargo",key:"owner-"+index})));
 governor.dispose();
 expect(peak,"never more owners in process than the headroom covers").toBeLessThanOrEqual(2);
});

test("the last owner always returns, whatever the headroom",async()=>{
 const governor=createMemoryGovernorV1({workers:4,headroom:()=>0,estimate:()=>400*MEGABYTE,policy:fast,sleep});
 let inProcess=0,peak=0;
 await Promise.all(Array.from({length:4},(_,index)=>governor.run(async handle=>{
  await handle.away(()=>Bun.sleep(5));
  inProcess++;peak=Math.max(peak,inProcess);await Bun.sleep(5);inProcess--;
 },{kind:"k",key:"o"+index})));
 governor.dispose();
 expect(peak).toBe(1);
});

test("a share taken while owners ran together never lowers the learned peak of the kind",async()=>{
 let used=0;const written:Record<string,number[]>[]=[];
 const governor=createMemoryGovernorV1({workers:3,headroom:()=>1e15-used,estimate:()=>1,policy:{...fast,peakSamples:1},peaks:{read:()=>({k:[500*MEGABYTE]}),write:peaks=>{written.push(peaks);}},sleep});
 await Promise.all(Array.from({length:3},(_,index)=>governor.run(async()=>{used+=10*MEGABYTE;await Bun.sleep(20);used-=10*MEGABYTE;},{kind:"k",key:"o"+index})));
 expect(governor.stats().kinds.k).toBe(500);
 governor.dispose();
});
