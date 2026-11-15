/** 🌊️ Contract: the wave row of a graph read is progress for the watch of the server (it moves the stall clock and the newest generation, however the stderr is cut) and is handed to the server once per row. */
import{expect,test}from"bun:test";
import{createGraphWaveMeterV1,renderGraphWaveRowV1}from"../../../../../🕸️graph/📈️wave-row/🟨️.mjs";
import{createGraphReadWatchV1,wantedServerPolicyV1}from"../../../🛠️server/🟦️.ts";

const row=(generation:number)=>renderGraphWaveRowV1(createGraphWaveMeterV1({now:()=>0}).take({generation,parentMegabytes:900,done:48,total:96}));

test("a wave row alone is progress: the newest generation moves and the stall clock restarts",()=>{
 let clock=0;const seen:number[]=[],watch=createGraphReadWatchV1({...wantedServerPolicyV1,inferStallMilliseconds:1000},()=>clock,wave=>seen.push(wave.generation));
 clock=900;watch.feed(row(5)+"\n");
 expect(watch.generation()).toBe(5);expect(watch.silentMilliseconds()).toBe(0);expect(seen).toEqual([5]);
 clock=1800;expect(watch.verdict()).toBe("go");clock=1900;expect(watch.verdict()).toBe("stalled");
});

test("rows cut at any byte are read once complete and an older generation does not move the newest",()=>{
 const text=row(9)+"\nnoise\n"+row(8)+"\n",seen:number[]=[],watch=createGraphReadWatchV1(wantedServerPolicyV1,()=>0,wave=>seen.push(wave.generation));
 for(const piece of text.match(/[\s\S]{1,7}/gu)!)watch.feed(piece);
 expect(seen).toEqual([9,8]);expect(watch.generation()).toBe(9);
});

test("a callback that throws never breaks the watch, a broken row is ignored",()=>{
 const watch=createGraphReadWatchV1(wantedServerPolicyV1,()=>0,()=>{throw new Error("log full");});
 watch.feed(row(3)+"\n[graph-wave] {\n");expect(watch.generation()).toBe(3);
});
