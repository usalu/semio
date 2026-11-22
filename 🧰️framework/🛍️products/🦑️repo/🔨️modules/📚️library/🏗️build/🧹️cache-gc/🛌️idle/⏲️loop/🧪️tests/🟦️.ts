/** 🧪️ Tests for the maintenance loop of compaction on idle with a fake clock. */
import {expect,test} from "bun:test";
import {startIdleCompactionLoopV1,startSidecarIdleCompactionV1} from "../🟦️.ts";
import {CACHE_GC_LIMITS_V1} from "../../../🧬️schema/🟦️.ts";

const clock=()=>{let now=0;const timers:{at:number;callback:()=>void;live:boolean}[]=[];
 return{now:()=>now,setTimer:(callback:()=>void,milliseconds:number)=>{const row={at:now+milliseconds,callback,live:true};timers.push(row);return{cancel:()=>{row.live=false;}};},
  advance:async(milliseconds:number)=>{const end=now+milliseconds;for(;;){const next=timers.filter(row=>row.live&&row.at<=end).sort((left,right)=>left.at-right.at)[0];if(!next)break;next.live=false;now=next.at;next.callback();await Promise.resolve();await new Promise(done=>setImmediate(done));}now=end;}};};
const HOUR=3600000;

test("the first pass waits for its delay, then one pass follows each pass after the interval, never two at once",async()=>{
 const fake=clock(),log:string[]=[];let release:()=>void=()=>undefined;
 const loop=startIdleCompactionLoopV1({firstDelayMilliseconds:600000,intervalMilliseconds:HOUR,ports:{setTimer:fake.setTimer,run:async()=>{log.push("start "+fake.now());await new Promise<void>(done=>{release=done;});log.push("end "+fake.now());}}});
 await fake.advance(599999);expect(log).toEqual([]);
 await fake.advance(1);expect(log).toEqual(["start 600000"]);
 await fake.advance(5*HOUR);expect(log).toEqual(["start 600000"]);
 release();await loop.settled();await new Promise(done=>setImmediate(done));
 await fake.advance(HOUR-1);expect(log).toEqual(["start 600000","end "+(600000+5*HOUR)]);
 await fake.advance(1);expect(log.length).toBe(3);expect(log[2]).toBe("start "+(600000+6*HOUR));
 release();loop.stop();
});

test("a failing pass is reported and the loop goes on; stop cancels the timer and the running pass",async()=>{
 const fake=clock(),failures:string[]=[];let passes=0,cancelled=false;
 const loop=startIdleCompactionLoopV1({firstDelayMilliseconds:1,intervalMilliseconds:10,ports:{setTimer:fake.setTimer,failed:error=>failures.push(String((error as Error).message)),run:async signal=>{passes++;if(passes<3)throw Error("pass "+passes);await new Promise<void>(done=>signal.addEventListener("abort",()=>{cancelled=true;done();},{once:true}));}}});
 await fake.advance(1);await fake.advance(10);await fake.advance(10);
 expect(failures).toEqual(["pass 1","pass 2"]);expect(passes).toBe(3);
 loop.stop();await loop.settled();expect(cancelled).toBe(true);
 await fake.advance(1000);expect(passes).toBe(3);
});

test("a signal that is already aborted never schedules a pass",async()=>{
 const fake=clock();let passes=0;const controller=new AbortController();controller.abort();
 startIdleCompactionLoopV1({firstDelayMilliseconds:1,intervalMilliseconds:1,signal:controller.signal,ports:{setTimer:fake.setTimer,run:async()=>{passes++;}}});
 await fake.advance(100);expect(passes).toBe(0);
});

test("the sidecar of an agent and of the cache runs the pass at the declared delay and interval with its root and environment, the other roles run nothing",async()=>{
 for(const role of ["agent","cache"]){
  const fake=clock(),calls:string[]=[],controller=new AbortController();
  const loop=startSidecarIdleCompactionV1({role,root:"/work",environment:{A:"1"},signal:controller.signal,ports:{setTimer:fake.setTimer,run:async()=>{calls.push(String(fake.now()));}}});
  expect(loop).toBeDefined();
  await fake.advance(CACHE_GC_LIMITS_V1.idleCompactionFirstDelayMilliseconds-1);expect(calls).toEqual([]);
  await fake.advance(1);await fake.advance(CACHE_GC_LIMITS_V1.idleCompactionIntervalMilliseconds);
  expect(calls).toEqual([String(CACHE_GC_LIMITS_V1.idleCompactionFirstDelayMilliseconds),String(CACHE_GC_LIMITS_V1.idleCompactionFirstDelayMilliseconds+CACHE_GC_LIMITS_V1.idleCompactionIntervalMilliseconds)]);
  controller.abort();await fake.advance(10*HOUR);expect(calls.length).toBe(2);
 }
 for(const role of ["ready","pool","scheduler","graph"])expect(startSidecarIdleCompactionV1({role,root:"/work",environment:{},signal:new AbortController().signal})).toBeUndefined();
});
