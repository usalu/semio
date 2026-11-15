import {expect,setDefaultTimeout,test} from "bun:test";
import {createStoreGuardV1,type StoreStateV1} from "../🟦️.ts";

setDefaultTimeout(60_000);
const hanging=(deadlineMs:number)=>{
 const states:StoreStateV1[]=[],aborts:unknown[]=[];
 const guard=createStoreGuardV1({deadlineMs,now:Date.now,changed:state=>states.push(state),probe:()=>new Promise<boolean>(()=>{})});
 return {guard,states,aborts};
};

test("an upload that keeps beating is not abandoned although the probe on the same saturated mount stays unanswered",async()=>{
 const {guard,states}=hanging(120);
 let aborted=false;
 const turn=await guard.turn(async signal=>{
  signal.addEventListener("abort",()=>{aborted=true;});
  for(let step=0;step<14;step++){await Bun.sleep(40);guard.beat();}
  return "uploaded";
 });
 expect(turn).toEqual({status:"done",value:"uploaded"});
 expect(aborted).toBe(false);expect(states).toEqual([]);expect(guard.state()).toBe("ok");
});

test("an upload that stops beating is abandoned once the probe and the silence both exceed the deadline, and it can be started again after the store answers",async()=>{
 const {guard,states}=hanging(120);
 let aborted=false;
 const started=Date.now();
 const turn=await guard.turn(async signal=>{
  signal.addEventListener("abort",()=>{aborted=true;});
  for(let step=0;step<3;step++){await Bun.sleep(40);guard.beat();}
  await new Promise<void>(done=>signal.addEventListener("abort",()=>done()));
 });
 expect(turn).toEqual({status:"stalled"});
 expect(aborted).toBe(true);expect(states).toEqual(["stalled"]);
 expect(Date.now()-started).toBeGreaterThanOrEqual(220);expect(Date.now()-started).toBeLessThan(3000);
});

test("without any beat the guard abandons a turn at the deadline as before",async()=>{
 const {guard}=hanging(100);
 const started=Date.now();
 const turn=await guard.turn(async signal=>{await new Promise<void>(done=>signal.addEventListener("abort",()=>done()));});
 expect(turn).toEqual({status:"stalled"});
 expect(Date.now()-started).toBeLessThan(2000);
});
