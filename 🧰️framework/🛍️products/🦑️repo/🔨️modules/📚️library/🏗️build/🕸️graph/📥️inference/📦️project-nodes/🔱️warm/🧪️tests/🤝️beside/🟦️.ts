/** 🤝️ Contract: the warm pass runs beside the parent's take, never before it, and every lane works its cheap owners first so the summed counters move from the first seconds.
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{planWarmLanesV1,runWarmBesideTakeV1,runWarmLanesV1,warmDigestV1,warmLineV1,warmThroughV1,warmWaveV1}from"../../🟨️.mjs";

import{tmpdir}from"node:os";
import{join}from"node:path";
const sleep=(milliseconds:number)=>new Promise(resolve=>setTimeout(resolve,milliseconds));

test("every lane lists its owners in ascending cost, the lane totals and the cover stay the same",()=>{
 const owners=Array.from({length:60},(_,index)=>({name:"o"+index,key:warmDigestV1("o"+index),cost:index%9===0?900:index%4===0?60:1})),cost=new Map(owners.map(owner=>[owner.name,owner.cost]));
 const lanes=planWarmLanesV1(owners,4);
 for(const lane of lanes)expect(lane.map(name=>cost.get(name)!)).toEqual([...lane.map(name=>cost.get(name)!)].sort((a,b)=>a-b));
 expect(lanes.flat().sort()).toEqual(owners.map(owner=>owner.name).sort());
 expect(planWarmLanesV1([...owners].reverse(),4)).toEqual(lanes);
});

test("the first owner of a lane is settled within the first milliseconds although the heavy owners of the lane need long",async()=>{
 const owners=["heavy-a","light-a","heavy-b","light-b","light-c"].map(name=>({name,key:name,cost:name.startsWith("heavy")?900:1})),lanes=planWarmLanesV1(owners,1),first:number[]=[],started=performance.now();
 await runWarmLanesV1({lanes,inspect:()=>({key:""}),warm:async name=>{await sleep(name.startsWith("heavy")?150:5);first.push(performance.now()-started);return{peakMegabytes:1};}});
 expect(first.slice(0,3).every(at=>at<100)).toBe(true);
});

test("the take starts at once and finishes while the warm pass still runs, and the pass is awaited at the end",async()=>{
 const events:string[]=[];let warmDone=false;
 await runWarmBesideTakeV1(async()=>{events.push("warm-start");await sleep(120);warmDone=true;events.push("warm-end");},async()=>{events.push("take-start");await sleep(20);events.push("take-end");expect(warmDone).toBe(false);});
 expect(events).toEqual(["warm-start","take-start","take-end","warm-end"]);
});

test("a failing take still waits for the warm pass and rethrows, a failing pass is the pass's own business",async()=>{
 let warmDone=false;
 await expect(runWarmBesideTakeV1(async()=>{await sleep(40);warmDone=true;},async()=>{throw new Error("take failed");})).rejects.toThrow("take failed");
 expect(warmDone).toBe(true);
});

test("taken and warmed both advance in the first moments of a beside run",async()=>{
 let warmed=0,taken=0;const seen:{warmed:number;taken:number}[]=[];
 const owners=Array.from({length:30},(_,index)=>({name:"o"+index,key:"k"+index,cost:1})),lanes=planWarmLanesV1(owners,2);
 const run=runWarmBesideTakeV1(async()=>{await runWarmLanesV1({lanes,inspect:()=>({key:""}),warm:async()=>{await sleep(10);warmed++;return{peakMegabytes:1};}});},async()=>{for(let index=0;index<30;index++){await sleep(5);taken++;}});
 await sleep(60);seen.push({warmed,taken});await run;
 expect(seen[0].warmed).toBeGreaterThan(0);expect(seen[0].taken).toBeGreaterThan(0);expect(warmed).toBe(30);expect(taken).toBe(30);
});

test("a lane that works on one long owner still tells its progress at every beat, so the stage never reads as stalled",async()=>{
 const told:number[]=[],lanes=[["long"]];
 await runWarmLanesV1({lanes,inspect:()=>({key:""}),warm:async()=>{await sleep(260);return{peakMegabytes:1};},tell:fields=>told.push(fields.done),progressMilliseconds:40});
 expect(told.length).toBeGreaterThanOrEqual(4);expect(told.at(-1)).toBe(1);expect(told.slice(0,-1).every(done=>done===0)).toBe(true);
});

test("the reasons a farm gives for the owners it could not warm reach the counters and the progress line",async()=>{
 const reasons:Record<string,number>={},told:string[]=[];
 const farm={engage(){},whenReady:async()=>true,width:()=>1,workerMegabytes:()=>100,warmReasons:()=>({...reasons}),warm:async()=>{reasons["no-usable-child"]=(reasons["no-usable-child"]??0)+1;return undefined;}};
 const names=Array.from({length:70},(_,index)=>"o"+index),counters=await warmWaveV1({farm,names,describe:(name:string)=>({key:warmDigestV1(name),stamp:undefined,cost:1,job:{name}}),directory:join(tmpdir(),"semio-warm-why-"+process.pid),cores:4,limitMegabytes:Infinity,tell:(fields:any)=>told.push(warmLineV1("en",fields))});
 expect(counters!.failed).toBe(70);expect(counters!.why).toBe("no-usable-child 70");expect(told.at(-1)).toContain("[no-usable-child 70]");
});

test("an owner waits for a child that is starting, runs again when its child was lost, and ends with a reason of its own",async()=>{
 let clock=0,children=0;const now=()=>clock,wait=async(milliseconds:number)=>{clock+=milliseconds;if(clock>=2000)children=1;};
 const first=await warmThroughV1({pick:()=>children?"child":undefined,run:async()=>({reply:{peakMegabytes:5}}),hopeless:()=>undefined,wait,now});
 expect(first.reply).toEqual({peakMegabytes:5});expect(clock).toBe(2000);
 let tries=0;const second=await warmThroughV1({pick:()=>"child",run:async()=>++tries<3?{lost:"exited"}:{reply:{}},hopeless:()=>undefined,wait,now});
 expect(second.reply).toEqual({});expect(tries).toBe(3);
 const third=await warmThroughV1({pick:()=>"child",run:async()=>({lost:"exited"}),hopeless:()=>undefined,wait,now});
 expect(third.why).toBe("lost: exited");
 const fourth=await warmThroughV1({pick:()=>"child",run:async()=>({failure:new Error("boom")}),hopeless:()=>undefined,wait,now});
 expect(fourth.why).toBe("failure: boom");
 const fifth=await warmThroughV1({pick:()=>undefined,run:async()=>({}),hopeless:()=>"no-usable-child",wait,now});
 expect(fifth.why).toBe("no-usable-child");
 const sixth=await warmThroughV1({pick:()=>undefined,run:async()=>({}),hopeless:()=>undefined,wait,now,deadlineMilliseconds:3000});
 expect(sixth.why).toStartWith("no child within 3 s");
});
