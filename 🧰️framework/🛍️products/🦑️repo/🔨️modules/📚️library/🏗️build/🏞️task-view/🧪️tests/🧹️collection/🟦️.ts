import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdirSync,readdirSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {TASK_VIEW_POLICY_V1,collectTaskViewsV1,digestTaskViewInputsV1,openTaskViewV1,selectTaskViewsToCollectV1,type TaskViewRowV1} from "../../🟦️.ts";
import {growTreeV1,hasOracleV1,oracleV1,removeV1,scratchV1} from "../🧰️world/🟦️.ts";

const scratch:string[]=[];
const world=(tag:string)=>{const root=scratchV1(tag);scratch.push(root);return root;};
afterAll(()=>{for(const root of scratch)removeV1(root);});
const bounds=TASK_VIEW_POLICY_V1.collection,HOUR=3600000;
const seeded=(seed:number)=>()=>{seed=(seed+0x6d2b79f5)|0;let value=Math.imul(seed^(seed>>>15),1|seed);value^=value+Math.imul(value^(value>>>7),61|value);return((value^(value>>>14))>>>0)/4294967296;};

test.skipIf(!hasOracleV1)("collection law: the selection equals an independent Python model on 400 seeded cases (leases, idle edges, count and byte bounds)",()=>{
 const next=seeded(20261118),cases:{rows:TaskViewRowV1[];now:number;bounds:typeof bounds}[]=[];
 for(let round=0;round<400;round++){
  const now=10*HOUR*1000,count=Math.floor(next()*40),rows:TaskViewRowV1[]=[];
  for(let index=0;index<count;index++)rows.push({hash:"h"+String(index).padStart(6,"0"),lastUsedMs:now-Math.floor(next()*(round%4===0?30:8)*HOUR)+(round%7===0?0:Math.floor(next()*3)*1000),bytes:Math.floor(next()*5e9),leased:next()<0.2});
  cases.push({rows,now,bounds:{maximumViews:1+Math.floor(next()*12),maximumLogicalBytes:Math.floor(next()*3e10),maximumIdleMilliseconds:(1+Math.floor(next()*10))*HOUR,minimumIdleMilliseconds:Math.floor(next()*2)*600000,leaseMilliseconds:bounds.leaseMilliseconds}});
 }
 const model=oracleV1({op:"select",cases}) as string[][];
 cases.forEach((row,index)=>expect(selectTaskViewsToCollectV1(row.rows,row.now,row.bounds),"case "+index).toEqual(model[index]!));
 expect(model.some(row=>row.length>0)&&model.some(row=>row.length===0)).toBe(true);
});

test("collection law: views are bounded in count, a leased or young view is never collected, a dead process's lease is dropped",async()=>{
 const live=world("tv-live"),directory=world("tv-views"),paths=growTreeV1(live,6),inputs=await digestTaskViewInputsV1(live,paths);
 const now=Date.now();
 for(let index=0;index<8;index++){
  const view=await openTaskViewV1({source:live,directory,hash:"gc00000"+index,inputs,pid:index===0?process.pid:999999});
  if(view.kind!=="ready")throw Error("open");
  const old=new Date(now-(8-index)*HOUR);utimesSync(join(directory,"gc00000"+index,"manifest.json"),old,old);
  if(index!==0&&index!==7)await view.view.release();
 }
 const policy={...TASK_VIEW_POLICY_V1,collection:{...bounds,maximumViews:3}};
 const dry=await collectTaskViewsV1({directory,policy,now:()=>now,dryRun:true,alive:pid=>pid===process.pid});
 expect(dry.removed.length).toBeGreaterThan(0);expect(readdirSync(directory).filter(name=>name.startsWith("gc0")).length).toBe(8);
 const done=await collectTaskViewsV1({directory,policy,now:()=>now,alive:pid=>pid===process.pid});
 const left=readdirSync(directory).filter(name=>name.startsWith("gc0")).sort();
 expect(left.length,"count bound").toBeLessThanOrEqual(3+1);
 expect(left,"the view held by a live process stays").toContain("gc000000");
 expect(left,"the newest stays").toContain("gc000007");
 expect(done.freedBytes).toBeGreaterThan(0);
 expect(readdirSync(join(directory,".use")).filter(name=>name.startsWith("gc000007")),"a lease of a dead process is dropped").toEqual([]);
 expect(readdirSync(join(directory,".use")).filter(name=>name.startsWith("gc000000")).length).toBe(1);
});

test("collection law: only direct children named like a task hash are touched, never through a link, and old staging directories go",async()=>{
 const directory=world("tv-views"),outside=world("tv-outside"),now=Date.now();
 writeFileSync(join(outside,"precious"),"keep");
 symlinkSync(outside,join(directory,"deadbeef"));
 mkdirSync(join(directory,"not a hash"));mkdirSync(join(directory,".tmp-old"));mkdirSync(join(directory,".tmp-new"));
 const old=new Date(now-3*HOUR);utimesSync(join(directory,".tmp-old"),old,old);
 const done=await collectTaskViewsV1({directory,now:()=>now});
 expect(existsSync(join(outside,"precious"))).toBe(true);
 expect(existsSync(join(directory,"deadbeef"))).toBe(true);
 expect(existsSync(join(directory,"not a hash"))).toBe(true);
 expect(existsSync(join(directory,".tmp-old"))).toBe(false);expect(existsSync(join(directory,".tmp-new"))).toBe(true);
 expect(done.temporaries).toBe(1);
 expect((await collectTaskViewsV1({directory:join(directory,"missing")})).removed).toEqual([]);
});

test("collection law: a task that takes a view between the choice and the removal keeps it",async()=>{
 const live=world("tv-live"),directory=world("tv-views"),paths=growTreeV1(live,4),inputs=await digestTaskViewInputsV1(live,paths),now=Date.now();
 const first=await openTaskViewV1({source:live,directory,hash:"race0001",inputs,pid:999999});
 if(first.kind!=="ready")throw Error("open");await first.view.release();
 const old=new Date(now-20*HOUR);utimesSync(join(directory,"race0001","manifest.json"),old,old);
 const done=await collectTaskViewsV1({directory,now:()=>now,alive:()=>true,afterSelect:async()=>{writeFileSync(join(directory,".use","race0001.1.late"),JSON.stringify({pid:1,startedAt:now}));}});
 expect(done.removed).toEqual([]);
 expect(existsSync(join(directory,"race0001","r"))).toBe(true);
});
