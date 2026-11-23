/** 🌀️ Contract: the yield of a script process gives the event loop a turn at least every `turnMilliseconds` of the schema, so a synchronous stretch of inference is neither deaf to the timers and messages of its process (the parent's request to settle memory, cancellation) nor blind to its own progress. The oracle is the event loop delay histogram of `node:perf_hooks`, independent of the implementation.
 * @see ../../🟦️.ts */
import {monitorEventLoopDelay} from "node:perf_hooks";
import {expect,test} from "bun:test";
import {createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../🟦️.ts";
import turn from "../../🧬️schema/🌀️turn.json";

const burn=(milliseconds:number)=>{const until=performance.now()+milliseconds;while(performance.now()<until);};

test("a loop of inference yields lets timers run and keeps the event loop delay near the declared turn",async()=>{
 const histogram=monitorEventLoopDelay({resolution:5});histogram.enable();
 let ticks=0;const interval=setInterval(()=>{ticks++;},10);
 const yields=await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"event-loop-turn-fixture",maximumElapsedMilliseconds:0},{fixture:"turn"},Date.now()),async invocation=>{
  let count=0;for(const end=performance.now()+1200;performance.now()<end;count++){burn(4);await invocation.control.yieldContinuation();}return count;
 });
 clearInterval(interval);histogram.disable();
 expect(yields).toBeGreaterThan(50);
 expect(ticks).toBeGreaterThanOrEqual(5);
 expect(histogram.max/1e6).toBeLessThan(turn.const.turnMilliseconds*8);
});

test("a yield that finds the loop turned a moment ago is a microtask, so the hot loops of the inference do not pay for the turn",async()=>{
 const spent=await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"event-loop-turn-cost",maximumElapsedMilliseconds:0},{fixture:"turn"},Date.now()),async invocation=>{
  const began=performance.now();for(let count=0;count<20000;count++)await invocation.control.yieldContinuation();return performance.now()-began;
 });
 expect(spent).toBeLessThan(2000);
});

test("the declared turn is a positive number of milliseconds below one second",()=>{
 expect(turn.const.version).toBe(1);expect(turn.const.turnMilliseconds).toBeGreaterThan(0);expect(turn.const.turnMilliseconds).toBeLessThan(1000);
});
