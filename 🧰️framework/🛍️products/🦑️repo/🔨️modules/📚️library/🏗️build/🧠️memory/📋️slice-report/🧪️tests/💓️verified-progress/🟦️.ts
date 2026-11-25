/** 💓️ Contract: a take pass whose owner count stands while it verifies records (inventory names found present, records seeded from the fleet) still tells new progress lines, which the stall tracker of the stage sees as progress and whose `taken N of M` the graph counter still reads; a pass where nothing moves tells nothing new. The rendered lines are the ones the host prints.
 * @see ../../🟨️.mjs `startSliceProgressV1`
 * @see ../../../../🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🟦️.ts `readyStallTrackerV1`, `readyGraphCounterTrackerV1` */
import{expect,test}from"bun:test";
import{readyGraphCounterTrackerV1,readyStallTrackerV1}from"../../../../🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🟦️.ts";
import{registerSliceReportSourceV1,startSliceProgressV1}from"../../🟨️.mjs";

const pause=(milliseconds:number)=>new Promise(resolve=>setTimeout(resolve,milliseconds));
const line=(seconds:number,event:{phase:string;done:number;total:number})=>`Reading the project graph slice (${seconds} s): ${event.phase} ${event.done} of ${event.total}.`;

test("verified records move the progress line while taken stands, nothing moving tells nothing new",async()=>{
 let verified=0,seeded=0;
 const told:{phase:string;done:number;total:number}[]=[],remove=registerSliceReportSourceV1("owners",()=>({ledger:{done:52,total:608},presence:{verified:{remembered:verified}},fleet:{seeded}})),stop=startSliceProgressV1({emit:event=>told.push(event),milliseconds:10});
 try{
  await pause(40);
  expect(told.map(event=>event.phase)).toEqual(["taken"]);
  verified=297;await pause(40);
  seeded=3;await pause(40);
  const quiet=told.length;await pause(60);expect(told.length).toBe(quiet);
  expect(told.map(event=>event.phase)).toEqual(["taken","297 inventory records and fleet records verified / 297 Inventar- und Flotteneinträge bestätigt; taken","300 inventory records and fleet records verified / 300 Inventar- und Flotteneinträge bestätigt; taken"]);
  const stall=readyStallTrackerV1(),counter=readyGraphCounterTrackerV1();
  expect(told.map((event,at)=>stall.observe([line(100+at,event)]))).toEqual([true,true,true]);
  expect(stall.observe([line(999,told.at(-1)!)])).toBe(false);
  expect(counter.observe([line(1,told[2]!)])).toBe(true);expect(counter.snapshot()).toBe(52);
 }finally{stop();remove();}
});
