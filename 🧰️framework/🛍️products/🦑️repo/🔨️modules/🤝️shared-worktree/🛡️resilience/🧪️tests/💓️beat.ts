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

test("a store call that never returns does not wedge the guard: after the store answers a probe, later calls and turns run again", async () => {
 const states: StoreStateV1[] = [];
 let up = true;
 const guard = createStoreGuardV1({deadlineMs: 60, now: Date.now, changed: state => states.push(state), probe: async () => up});
 await expect(guard.call(() => new Promise<never>(() => {}))).rejects.toThrow("Store call refused");
 expect(guard.state()).toBe("stalled");
 expect(guard.outstanding()).toBe(0);
 await Bun.sleep(5 * 60 + 40);
 expect(await guard.reachable()).toBe(true);
 expect(guard.state()).toBe("ok");
 expect(await guard.call(async () => 5)).toBe(5);
 expect(await guard.turn(async () => 6)).toEqual({status: "done", value: 6});
 expect(states).toEqual(["stalled", "ok"]);
});

test("a turn that never returns is abandoned and the next turn is not reported busy once the store answers", async () => {
 const states: StoreStateV1[] = [];
 let hang = true;
 const guard = createStoreGuardV1({deadlineMs: 60, now: Date.now, changed: state => states.push(state), probe: () => hang ? new Promise<boolean>(() => {}) : Promise.resolve(true)});
 expect(await guard.turn(() => new Promise<never>(() => {}))).toEqual({status: "stalled"});
 hang = false;
 await Bun.sleep(5 * 60 + 40);
 for (let round = 0; round < 40 && guard.state() !== "ok"; round++) {await guard.reachable();await Bun.sleep(20);}
 expect(guard.state()).toBe("ok");
 expect(await guard.turn(async () => 7)).toEqual({status: "done", value: 7});
});

test("a probe that never answers is orphaned after three deadlines and a new probe recovers the guard; four leaked calls keep it stalled", async () => {
 let hang = true;
 const guard = createStoreGuardV1({deadlineMs: 50, now: Date.now, changed: () => {}, probe: () => hang ? new Promise<boolean>(() => {}) : Promise.resolve(true)});
 expect(await guard.reachable()).toBe(false);
 hang = false;
 await Bun.sleep(5 * 50 + 50);
 for (let round = 0; round < 40 && guard.state() !== "ok"; round++) {await guard.reachable();await Bun.sleep(20);}
 expect(guard.state()).toBe("ok");
 const capped = createStoreGuardV1({deadlineMs: 30, now: Date.now, changed: () => {}, probe: async () => true});
 for (let leak = 0; leak < 4; leak++) {
  await Bun.sleep(5 * 30 + 20);
  await capped.reachable();
  await expect(capped.call(() => new Promise<never>(() => {}))).rejects.toThrow("Store call refused");
 }
 await Bun.sleep(5 * 30 + 20);
 await capped.reachable();
 await expect(capped.call(async () => 1)).rejects.toThrow("Store call refused");
});
