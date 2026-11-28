/** 📨️ Contract: the run record of `ready-timings.json` names who ended the run (`sender`, the kind the supervisor decided on) and, when this supervisor stopped the stage itself, the `stopBy` row it wrote into the stage record before the signal (supervisor pid, reason, time); the store file validates with both, so a census of stops needs no log parsing.
 * @see ../../🟦️.ts `readyTimingSenderV1` */
import{expect,test}from"bun:test";
import{readyTimingSenderV1}from"../../🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"../../🧬️schema/🔣️.json"with{type:"json"};

const run={stage:"workspace",startedMs:1,endedMs:2000,seconds:2,exit:null,outcome:"interrupted",peakMegabytes:null,attempts:1,digest:"a579a1413f46"};
const document=(row:object)=>({version:1,origin:"g1-m09",runs:[row],updatedMs:3000});

test("a run record carries the sender kind and the stop row, and nothing when the supervisor knows neither",()=>{
 expect(readyTimingSenderV1({})).toEqual({});
 expect(readyTimingSenderV1({sender:"outside"})).toEqual({sender:"outside"});
 const stopBy={supervisor:94133,reason:"cancel" as const,atMs:1_795_776_904_000};
 expect(readyTimingSenderV1({sender:"supervisor",stopBy})).toEqual({sender:"supervisor",stopBy});
});

test("the timings file of the store validates with the sender and the stop row, and refuses a stop row without its reason",()=>{
 const stopBy={supervisor:94133,reason:"cancel",atMs:1_795_776_904_000};
 expect(validateJsonSchemaSubset(schema.definitions.Timings,document({...run,signal:"SIGTERM",sender:"supervisor",stopBy}),schema)).toEqual([]);
 expect(validateJsonSchemaSubset(schema.definitions.Timings,document({...run,sender:"outside"}),schema)).toEqual([]);
 expect(validateJsonSchemaSubset(schema.definitions.Timings,document({...run,stopBy:{supervisor:1,atMs:5}}),schema).length).toBeGreaterThan(0);
});
