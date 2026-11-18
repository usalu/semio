/** 🧹️ Contract: the parent of a graph read holds a declared amount of managed memory: the governor admits a project only while the budget less the memory in use covers it, and the collector runs when the managed bytes (never the resident set) passed its trigger. */
import{expect,test}from"bun:test";
import{execFileSync,spawnSync}from"node:child_process";
import{mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{GRAPH_MEMORY_POLICY_V1 as policy,createGraphParentMemoryGovernorV1,createParentCollectorV1,graphParentManagedBytesV1,hostMemoryMegabytesV1}from"../../🟨️.mjs";

const MB=1048576,ports=(state:{held:number;survives:number;collected:number})=>({sample:()=>state.held,collect:()=>{state.collected++;state.held=state.survives;}});

test("the collector ignores the first projects, collects once the managed bytes passed the trigger and waits for growth above what survived",()=>{
 const state={held:0,survives:700*MB,collected:0};let clock=0;
 const collector=createParentCollectorV1({ports:ports(state),now:()=>clock});
 state.held=policy.collectAboveMegabytes*MB-1;
 expect([collector.afterProject(),collector.afterProject()]).toEqual([false,false]);
 state.held=policy.collectAboveMegabytes*MB+1;
 expect(collector.afterProject()).toBe(false);
 expect(collector.afterProject()).toBe(true);
 expect(state.collected).toBe(1);expect(state.held).toBe(700*MB);
 clock+=policy.collectMinimumIntervalMilliseconds;state.held=(700+policy.collectGrowthMegabytes)*MB-1;
 expect([collector.afterProject(),collector.afterProject()]).toEqual([false,false]);
 state.held=(700+policy.collectGrowthMegabytes)*MB+1;
 expect([collector.afterProject(),collector.afterProject()]).toEqual([false,true]);
 expect(state.collected).toBe(2);
});

test("the collector never runs twice within the minimum interval and a heap that survives everything does not make it spin",()=>{
 const state={held:4096*MB,survives:4096*MB,collected:0};let clock=0;
 const collector=createParentCollectorV1({ports:ports(state),now:()=>clock});
 for(let step=0;step<40;step++){collector.afterProject();clock+=100;}
 expect(state.collected).toBeLessThanOrEqual(Math.ceil(4000/policy.collectMinimumIntervalMilliseconds)+1);
 expect(state.collected).toBeGreaterThan(0);
});

test("finish collects, answers the peak, the survivors and the collections of the wave, and the next wave starts from the survivors",()=>{
 const state={held:900*MB,survives:300*MB,collected:0};
 const collector=createParentCollectorV1({ports:ports(state),now:()=>0});
 state.held=1500*MB;collector.afterProject();collector.afterProject();state.held=1200*MB;
 expect(collector.finish()).toEqual({peakBytes:1500*MB,liveBytes:300*MB,collections:2,collectMilliseconds:0});
 expect(collector.finish()).toEqual({peakBytes:300*MB,liveBytes:300*MB,collections:1,collectMilliseconds:0});
});

test("the budget less the growth since the collector was made is the headroom the governor sees, however large the state the parent started with",()=>{
 const state={held:700*MB,survives:0,collected:0};
 const collector=createParentCollectorV1({ports:ports(state),budgetBytes:1000*MB});
 expect(collector.headroomBytes()).toBe(1000*MB);state.held=1100*MB;expect(collector.headroomBytes()).toBe(600*MB);state.held=1800*MB;expect(collector.headroomBytes()).toBe(-100*MB);
 expect(createParentCollectorV1({ports:ports(state),budgetBytes:1000*MB,baselineBytes:0}).headroomBytes()).toBe(-800*MB);
});

test("a 16 GB machine gets a growth budget of 1024 MB, a serving machine of 24000 MB or more a quarter of its memory",()=>{
 const small=graphParentManagedBytesV1({},{...policy,serveHostMegabytes:1e9,parentHostShare:1024/Math.max(1,hostMemoryMegabytesV1())});
 expect(small).toBe(policy.minimumBudgetMegabytes*MB);
 expect(policy.parentHostShare*16384).toBeLessThanOrEqual(policy.minimumBudgetMegabytes);
 expect(16384).toBeLessThan(policy.serveHostMegabytes);expect(32768).toBeGreaterThanOrEqual(policy.serveHostMegabytes);
 const serve=graphParentManagedBytesV1({},{...policy,serveHostMegabytes:1});
 expect(serve).toBeLessThanOrEqual(Math.floor(hostMemoryMegabytesV1()*policy.serveHostShare)*MB);expect(serve).toBeGreaterThanOrEqual(policy.parentManagedMegabytes*MB);
});

test("the managed budget is the declared one, smaller for a grant that leaves the parent less, never below the governor minimum",()=>{
 expect(graphParentManagedBytesV1({},{...policy,serveHostMegabytes:1e9})).toBe(Math.min(policy.parentManagedMegabytes,Math.floor(hostMemoryMegabytesV1()*policy.parentHostShare))*MB);
 expect(policy.parentManagedMegabytes).toBeLessThanOrEqual(policy.maximumBudgetMegabytes);
 expect(graphParentManagedBytesV1({},{...policy,serveHostMegabytes:1e9,parentManagedMegabytes:10})).toBe(policy.minimumBudgetMegabytes*MB);
});

test("with a budget the governor admits owners only while what they are expected to take fits the budget, with the host showing unlimited room",async()=>{
 const state={held:0,survives:0,collected:0},p=ports(state),budget=1000*MB,each=400*MB;
 const governor=createGraphParentMemoryGovernorV1({workers:8,ports:p,headroom:()=>1e15,attributable:p.sample,budgetBytes:budget,sleep:()=>new Promise(resolve=>setTimeout(resolve,1)),policy:{...policy,admitMinimumMilliseconds:0,pollMilliseconds:1}});
 let running=0,most=0,peak=0;
 const task=async()=>{running++;most=Math.max(most,running);state.held+=each;peak=Math.max(peak,state.held);await new Promise(resolve=>setTimeout(resolve,15));state.held-=each;running--;};
 await Promise.all(Array.from({length:24},()=>governor.run(task,{kind:"project"})));
 governor.dispose();
 expect(running).toBe(0);
 expect(most).toBeGreaterThanOrEqual(1);
 expect(most*each).toBeLessThanOrEqual(budget+each);
 expect(peak).toBeLessThanOrEqual(budget+each);
});

test("a parent that starts with a large state still runs as many owners as the growth budget allows",async()=>{
 const state={held:3000*MB,survives:0,collected:0},p=ports(state),budget=1000*MB,each=400*MB;
 const governor=createGraphParentMemoryGovernorV1({workers:8,ports:p,headroom:()=>1e15,attributable:p.sample,budgetBytes:budget,baselineBytes:3000*MB,sleep:()=>new Promise(resolve=>setTimeout(resolve,1)),policy:{...policy,admitMinimumMilliseconds:0,pollMilliseconds:1}});
 let running=0,most=0;
 const task=async()=>{running++;most=Math.max(most,running);state.held+=each;await new Promise(resolve=>setTimeout(resolve,15));state.held-=each;running--;};
 await Promise.all(Array.from({length:24},()=>governor.run(task,{kind:"project"})));
 governor.dispose();
 expect(most).toBeGreaterThanOrEqual(2);expect(most*each).toBeLessThanOrEqual(budget+each);
});

test("without a budget the same owners run as many at once as the workers allow",async()=>{
 const state={held:0,survives:0,collected:0},p=ports(state),each=400*MB;
 const governor=createGraphParentMemoryGovernorV1({workers:8,ports:p,headroom:()=>1e15,attributable:p.sample,sleep:()=>new Promise(resolve=>setTimeout(resolve,1)),policy:{...policy,admitMinimumMilliseconds:0,pollMilliseconds:1}});
 let running=0,most=0;
 const task=async()=>{running++;most=Math.max(most,running);state.held+=each;await new Promise(resolve=>setTimeout(resolve,15));state.held-=each;running--;};
 await Promise.all(Array.from({length:24},()=>governor.run(task,{kind:"project"})));
 governor.dispose();
 expect(most).toBeGreaterThanOrEqual(4);
});

const footprintMegabytes=():number|undefined=>{
 try{
  if(process.platform==="darwin"){const text=execFileSync("footprint",[String(process.pid)],{encoding:"utf8"}),found=/Footprint:\s+([\d.]+)\s+(KB|MB|GB)/u.exec(text);return found?Math.round(Number(found[1])*({KB:1/1024,MB:1,GB:1024} as Record<string,number>)[found[2]!]!):undefined;}
  if(process.platform==="linux"){const text=require("node:fs").readFileSync("/proc/self/smaps_rollup","utf8"),pss=/^Pss:\s+(\d+) kB/mu.exec(text),swap=/^SwapPss:\s+(\d+) kB/mu.exec(text);return pss?Math.round((Number(pss[1])+Number(swap?.[1]??0))/1024):undefined;}
 }catch{}
 return undefined;
};

const childSource=(modulePath:string,controlled:boolean)=>`
import{createGraphParentMemoryGovernorV1,createMemoryPortsV1,createParentCollectorV1}from${JSON.stringify(modulePath)};
import{execFileSync}from"node:child_process";
const MB=1048576,ports=createMemoryPortsV1(),collector=createParentCollectorV1({ports,budgetBytes:300*MB,policy:{...(await import(${JSON.stringify(modulePath)})).GRAPH_MEMORY_POLICY_V1,collectAboveMegabytes:100,collectGrowthMegabytes:50,collectMinimumIntervalMilliseconds:0,collectEveryProjects:1}});
const footprint=()=>{if(process.platform==="darwin"){const t=execFileSync("footprint",[String(process.pid)],{encoding:"utf8"}),m=/Footprint:\\s+([\\d.]+)\\s+(KB|MB|GB)/u.exec(t);return Math.round(Number(m[1])*{KB:1/1024,MB:1,GB:1024}[m[2]]);}if(process.platform==="linux"){const t=require("node:fs").readFileSync("/proc/self/smaps_rollup","utf8");return Math.round((Number(/^Pss:\\s+(\\d+) kB/mu.exec(t)[1])+Number(/^SwapPss:\\s+(\\d+) kB/mu.exec(t)?.[1]??0))/1024);}return -1;};
const governor=createGraphParentMemoryGovernorV1({workers:8,ports,headroom:()=>1e15,attributable:ports.sample,${controlled?"budgetBytes:300*MB,":""}policy:{...(await import(${JSON.stringify(modulePath)})).GRAPH_MEMORY_POLICY_V1,pollMilliseconds:5,admitMinimumMilliseconds:0}});
let peakManaged=0,peakFootprint=0,running=0,most=0;
const sampler=setInterval(()=>{peakManaged=Math.max(peakManaged,ports.sample());},5);
const project=async()=>{running++;most=Math.max(most,running);const live=[];for(let i=0;i<500000;i++)live.push({a:"s"+i+Math.random(),b:[i,i+1],c:{d:"y".repeat(20)+i}});await new Promise(r=>setTimeout(r,60));peakFootprint=Math.max(peakFootprint,footprint());live.length=0;running--;};
const loop=async()=>{for(let n=0;n<4;n++){await governor.run(project,{kind:"project"});${controlled?"collector.afterProject();":""}}};
await Promise.all(Array.from({length:8},loop));
clearInterval(sampler);governor.dispose();
console.log(JSON.stringify({most,peakManagedMegabytes:Math.round(peakManaged/MB),peakFootprintMegabytes:peakFootprint}));`;

test("a real process with eight loops holds less managed memory and a smaller physical footprint under the budget than without it",()=>{
 const directory=mkdtempSync(join(tmpdir(),"parent-budget-")),module=fileURLToPath(new URL("../../🟨️.mjs",import.meta.url));
 try{
  const run=(controlled:boolean)=>{const file=join(directory,(controlled?"controlled":"free")+".mjs");writeFileSync(file,childSource(module,controlled));const result=spawnSync(process.execPath,[file],{encoding:"utf8",env:{...process.env,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"}});expect(result.status,result.stderr).toBe(0);return JSON.parse(result.stdout.trim().split("\n").at(-1)!) as{most:number;peakManagedMegabytes:number;peakFootprintMegabytes:number};};
  const free=run(false),controlled=run(true);
  expect(free.most).toBeGreaterThanOrEqual(4);
  expect(controlled.most).toBeLessThan(free.most);
  expect(controlled.peakManagedMegabytes).toBeLessThan(free.peakManagedMegabytes);
  if(footprintMegabytes()!==undefined&&free.peakFootprintMegabytes>0)expect(controlled.peakFootprintMegabytes).toBeLessThan(free.peakFootprintMegabytes);
 }finally{rmSync(directory,{recursive:true,force:true});}
},120000);

test("R57 law: the forced collections of the parent take at most collectDutyPercent of the wall time however long a collection takes and however fast the garbage returns",()=>{
 let clock=0,held=4096*MB,collections=0,spent=0;const costMilliseconds=8000;
 const collector=createParentCollectorV1({ports:{sample:()=>held,collect:()=>{collections++;spent+=costMilliseconds;clock+=costMilliseconds;held=1000*MB;}},now:()=>clock});
 const began=clock;
 for(;clock-began<1_200_000;clock+=100){held+=3000*MB;collector.afterProject();collector.afterProject();}
 expect(collections).toBeGreaterThanOrEqual(2);
 expect(spent/(clock-began)).toBeLessThanOrEqual(policy.collectDutyPercent/100+0.02);
});

test("R57 law: a cheap collection keeps the old pace: not twice within the minimum interval, again once it passed",()=>{
 let clock=0,held=4096*MB,collections=0;
 const collector=createParentCollectorV1({ports:{sample:()=>held,collect:()=>{collections++;held=1000*MB;}},now:()=>clock});
 held=4096*MB;collector.afterProject();expect(collector.afterProject()).toBe(true);
 held=4096*MB;clock+=policy.collectMinimumIntervalMilliseconds-1;collector.afterProject();expect(collector.afterProject()).toBe(false);
 clock+=1;collector.afterProject();expect(collector.afterProject()).toBe(true);expect(collections).toBe(2);
});
