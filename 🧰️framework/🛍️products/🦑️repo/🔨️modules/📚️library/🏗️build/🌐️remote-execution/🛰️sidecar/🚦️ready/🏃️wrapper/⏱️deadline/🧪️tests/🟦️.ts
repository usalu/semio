import {expect,test} from "bun:test";
import {armWrapperDeadlineV1,wrapperDeadlineOfV1} from "../🟦️.ts";

/** ⏱️ Contract: the wrapper ends its command at the absolute deadline with terminate, then kill after the grace, without any sidecar; a disarmed deadline fires nothing. */
const clock=()=>{let at=0;const timers:{at:number;run:()=>void;off:boolean}[]=[];return{now:()=>at,setTimer:(run:()=>void,ms:number)=>{const row={at:at+ms,run,off:false};timers.push(row);return row;},clearTimer:(row:unknown)=>{(row as {off:boolean}).off=true;},advance:(ms:number)=>{at+=ms;for(const row of timers.filter(row=>!row.off&&row.at<=at)){row.off=true;row.run();}}};};

test("terminate at the deadline and kill after the grace",()=>{
 const time=clock(),events:string[]=[];
 armWrapperDeadlineV1({deadlineMs:1000,graceMs:200,now:time.now,setTimer:time.setTimer,clearTimer:time.clearTimer,terminate:()=>events.push("terminate"),kill:()=>events.push("kill")});
 time.advance(999);expect(events).toEqual([]);
 time.advance(1);expect(events).toEqual(["terminate"]);
 time.advance(200);expect(events).toEqual(["terminate","kill"]);
});

test("a disarmed deadline and a deadline in the past",()=>{
 const time=clock(),events:string[]=[];
 const off=armWrapperDeadlineV1({deadlineMs:1000,graceMs:200,now:time.now,setTimer:time.setTimer,clearTimer:time.clearTimer,terminate:()=>events.push("terminate"),kill:()=>events.push("kill")});
 off();time.advance(5000);expect(events).toEqual([]);
 time.advance(0);
 armWrapperDeadlineV1({deadlineMs:1,graceMs:0,now:time.now,setTimer:time.setTimer,clearTimer:time.clearTimer,terminate:()=>events.push("t"),kill:()=>events.push("k")});
 time.advance(0);expect(events).toEqual(["t"]);
});

test("the argument is parsed strictly",()=>{
 expect(wrapperDeadlineOfV1("--deadline=1795218790432")).toBe(1795218790432);
 for(const bad of [undefined,"--deadline=","--deadline=-1","--deadline=x","--deadline=1.5","exit.json"])expect(wrapperDeadlineOfV1(bad)).toBeUndefined();
});
