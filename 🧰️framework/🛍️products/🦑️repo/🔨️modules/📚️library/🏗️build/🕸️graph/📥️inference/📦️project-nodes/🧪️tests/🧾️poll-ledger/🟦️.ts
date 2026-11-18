/** 🧾️ Contract: the ledger line of the owner stage ("N of M done: present, fetched from the fleet, inferred here, wanted from the fleet") is told again and again while the poll for the owners the fleet has not stored yet runs, and stops with the poll; the reporter of the first pass ended with that pass and the line stood at its first numbers for hours (R194: `0 fetched from the fleet` of an 8-hour read that was taking records).
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import source from"../../🟨️.mjs"with{type:"text"};
import{reportWhilePollingV1}from"../../🟨️.mjs";

test("the line is told at the interval while the poll runs and no more after the stop or the abort",async()=>{
 const told:number[]=[],polling=new AbortController(),stop=reportWhilePollingV1({tell:()=>told.push(Date.now()),milliseconds:20,polling:polling.signal});
 await new Promise(resolve=>setTimeout(resolve,130));
 expect(told.length).toBeGreaterThanOrEqual(3);expect(told.length).toBeLessThanOrEqual(7);
 polling.abort();const aborted=told.length;await new Promise(resolve=>setTimeout(resolve,80));expect(told.length).toBe(aborted);
 stop();
 const again:number[]=[],other=new AbortController(),stopAgain=reportWhilePollingV1({tell:()=>again.push(1),milliseconds:15,polling:other.signal});
 await new Promise(resolve=>setTimeout(resolve,50));stopAgain();const had=again.length;await new Promise(resolve=>setTimeout(resolve,60));
 expect(had).toBeGreaterThanOrEqual(2);expect(again.length).toBe(had);
});

test("the owner stage uses it for the poll of the missing owners, with the progress interval of the take policy, and ends it with the poll",()=>{
 expect(source).toContain("reportWhilePollingV1({tell:()=>takeSay(language=>ledger.line(language)),milliseconds:TAKE_POLICY_V1.progressMilliseconds,polling:polling.signal})");
 expect(source).toContain(".catch(()=>undefined).finally(stopTelling);");
});
