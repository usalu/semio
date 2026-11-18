import{afterAll,expect,test}from"bun:test";
import{existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{spawn,spawnSync}from"node:child_process";
import{validateJsonSchemaSubset}from"../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import{SLICE_REPORT_POLICY_V1 as P,measureSliceProcessesV1,parseProcessTreeV1,sharedBytesV1,buildSliceReportV1,registerSliceReportSourceV1,startSliceProgressV1,writeSliceReportV1}from"../🟨️.mjs";
import{createReadyPortsV1}from"../../../🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts";

import{collectSliceCollectionsV1,describeCollectionV1,estimateCollectionBytesV1,registerSliceCollectionsV1}from"../🟨️.mjs";
/** 📋️ The slice memory report: a machine nobody has a shell on tells what its slice worker holds, bounded, in the store next to ready.json. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`slice-report-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

test("a report is closed by its schema, names the phase, memory, heap types and every source, and a source that throws is named and does not stop it",()=>{
 registerSliceReportSourceV1("owners",()=>({jobs:703,ledger:{present:3,fetched:0,here:10}}));
 registerSliceReportSourceV1("broken",()=>{throw new Error("nope");});
 const report=buildSliceReportV1({reason:"interval",heap:()=>({heapSize:104857600,extraMemorySize:52428800,objectCount:12345,objectTypeCounts:{string:100,Object:50,Array:20}})});
 expect(validateJsonSchemaSubset({$ref:"#/definitions/Report"},report,schema)).toEqual([]);
 expect((report.sources as Record<string,unknown>).owners).toEqual({jobs:703,ledger:{present:3,fetched:0,here:10}});
 expect(((report.sources as Record<string,{error:string}>).broken).error).toBe("nope");
 expect((report.heap as {types:{type:string}[]}).types[0]!.type).toBe("string");
});

test("a report never exceeds fileBytes: the largest sources are dropped first and named",()=>{
 registerSliceReportSourceV1("huge",()=>({names:Array.from({length:20000},(_all,index)=>"owner-"+index)}));
 const root=temp("bound");writeSliceReportV1(root,{reason:"plan-stop"});
 const text=readFileSync(join(root,P.directory,P.name),"utf8"),report=JSON.parse(text);
 expect(text.length).toBeLessThanOrEqual(P.fileBytes);
 expect(report.dropped).toContain("huge");
 expect(report.sources.owners).toBeDefined();
});

test("real process: a worker that reads writes the report while it works, the moment its plan stops and at its end",()=>{
 const root=temp("real"),script=join(root,"worker.mjs");
 writeFileSync(script,`import{registerSliceReportSourceV1,startSliceReportV1,noteSliceReportPhaseV1,noteSliceReportStopV1}from${JSON.stringify(join(import.meta.dir,"..","🟨️.mjs"))};
registerSliceReportSourceV1("owners",()=>({jobs:5}));const r=startSliceReportV1({root:process.argv[2],footprint:{self:async()=>200*1048576,tree:async()=>300*1048576},intervalMilliseconds:50});
noteSliceReportPhaseV1({phase:"inference",done:3,total:5});await new Promise(done=>setTimeout(done,200));
noteSliceReportStopV1({stage:"inference",operation:"inference",usedMegabytes:9933,budgetMegabytes:6553});r.now("plan-stop");
import{readFileSync}from"node:fs";console.log(JSON.parse(readFileSync(process.argv[2]+"/${P.directory}/${P.name}","utf8")).reason);r.stop();`);
 const run=spawnSync(process.execPath,[script,root],{encoding:"utf8"});
 expect(run.stdout.trim()).toBe("plan-stop");
 const last=JSON.parse(readFileSync(join(root,P.directory,P.name),"utf8"));
 expect(last.reason).toBe("end");expect(last.phase).toEqual({name:"inference",done:3,total:5});expect(last.stop.usedMegabytes).toBe(9933);
 expect(validateJsonSchemaSubset({$ref:"#/definitions/Report"},last,schema)).toEqual([]);
 expect(last.footprintMegabytes).toBe(200);expect(last.guard.measure).toBe("heapUsed+external");
 expect(Math.abs(last.guard.megabytes-(last.memory.heapUsed+last.memory.external))).toBeLessThan(2);
 if(process.platform!=="win32"){expect(last.processes.self.rssMegabytes).toBeGreaterThan(0);expect(last.processes.count).toBe(0);expect(last.treeFootprintMegabytes).toBeUndefined();}
});

test("the ready child copies the report next to ready.json when it mirrors the log, and a refusal line is not cut",()=>{
 const root=temp("root"),store=temp("store");mkdirSync(join(root,P.directory),{recursive:true});
 writeFileSync(join(root,P.directory,P.name),JSON.stringify({version:1,marker:"copied"}));
 const ports=createReadyPortsV1({root,environment:{SEMIO_FLEET_STORE_ROOT:store},origin:"g1-test",group:undefined,say:()=>undefined});
 const long="The project graph could not be read on this machine yet: "+"x".repeat(900)+" END";
 ports.mirror!("workspace-failed",[long]);
 expect(JSON.parse(readFileSync(join(store,"v1","origins","g1-test",P.name),"utf8")).marker).toBe("copied");
 expect(existsSync(join(store,"v1","origins","g1-test","ready-log-workspace-failed.txt"))).toBe(true);
});

test("a structure shared by many entries is charged once, not once per referrer",()=>{
 const members=Array.from({length:150},(_all,at)=>Array.from({length:500},(_item,item)=>"m"+at+"-"+item));
 const entries=Array.from({length:120},()=>({sidecar:{facts:members.map((_member,at)=>({at})),members:members.slice()}}));
 const seen=new Set();let total=0;for(const entry of entries)total+=8+sharedBytesV1(entry.sidecar,seen);
 const inner=150*(16+500*8),perEntry=16+(16+(16+150*(8+32)))+(16+(16+150*8));
 expect(total).toBe(120*(8+perEntry)+inner);
 expect(total).toBeLessThan(2*1048576);
 expect(estimateCollectionBytesV1(120,entries.slice(0,16).map(entry=>entry.sidecar))).toBeGreaterThan(total*20);
 expect(sharedBytesV1(new Set([1,2,3]))).toBe(16+3*8);expect(sharedBytesV1(new Map([[1,{a:1}]]))).toBe(16+16+(16+16));
 const cycle:{self?:unknown}={};cycle.self=cycle;expect(sharedBytesV1(cycle)).toBe(16+16);
});

test("a process table names the descendants of a process by role with their resident and virtual sizes and leaves everything else out",()=>{
 const table=[" 100     1   2048   4096 /usr/bin/bun /x/👷️worker.ts --flag","\n 101   100  10240  20480 /usr/bin/bun /x/🏭️walk-farm.mjs 1","\n 102   101   5120   8192 /usr/bin/node child.js","\n 103     1  99999  99999 /usr/bin/other","\n 104   100    900   1000 ps -A -o pid=,ppid=,rss=,vsz=,command=","\n garbage line"].join("");
 const tree=parseProcessTreeV1(table,100);
 expect(tree).toEqual({self:{rssMegabytes:2,vszMegabytes:4},count:2,rssMegabytes:15,children:[{pid:101,role:"🏭️walk-farm.mjs",rssMegabytes:10,vszMegabytes:20},{pid:102,role:"child.js",rssMegabytes:5,vszMegabytes:8}]});
 expect(parseProcessTreeV1(table,100,1).children.length).toBe(1);expect(parseProcessTreeV1(table,555)).toEqual({count:0,rssMegabytes:0,children:[]});
});

test("real processes: the children of this process are found with their roles and the resident size that ps reports for each of them",async()=>{
 if(process.platform==="win32")return;
 const root=temp("census"),script=join(root,"probe-kid.mjs");writeFileSync(script,"const keep=new Uint8Array(24*1048576).fill(1);setTimeout(()=>keep.length,30000);");
 const kids=[spawn(process.execPath,[script],{stdio:"ignore"}),spawn(process.execPath,[script],{stdio:"ignore"})];
 try{
  await new Promise(resolve=>setTimeout(resolve,1500));
  const row=await measureSliceProcessesV1({footprint:{self:async()=>7*1048576}});
  expect(row.footprintMegabytes).toBe(7);expect(row.processes!.count).toBeGreaterThanOrEqual(2);
  const found=row.processes!.children.filter(child=>child.role==="probe-kid.mjs");
  expect(found.map(child=>child.pid).sort()).toEqual(kids.map(kid=>kid.pid!).sort());
  for(const child of found){
   const oracle=Number(spawnSync("ps",["-o","rss=","-p",String(child.pid)],{encoding:"utf8"}).stdout.trim())/1024;
   expect(child.rssMegabytes).toBeGreaterThan(20);expect(Math.abs(child.rssMegabytes-oracle)).toBeLessThan(Math.max(8,oracle*0.25));
  }
  expect(row.treeFootprintMegabytes).toBeUndefined();
  expect((await measureSliceProcessesV1({footprint:{tree:async()=>11*1048576}})).treeFootprintMegabytes).toBe(11);
 }finally{for(const kid of kids)kid.kill();}
},60000);

/** 🧱️ A heap snapshot of a 500 MB heap held 4 GB of resident pages for the rest of the process (measured: 613 MB to 4 780 MB, not returned by a collection) and its summary made the plan guard (heapUsed plus external) see 10 GB. The report must never walk the heap, even where the caller still passes a snapshot function. */
test("real process: a report that runs for 24 seconds beside a 560 MB heap keeps the guard number under 2 GB and the resident set under 3 GB",()=>{
 const root=temp("noheap"),script=join(root,"worker.mjs");
 writeFileSync(script,`import{startSliceReportV1}from${JSON.stringify(join(import.meta.dir,"..","🟨️.mjs"))};
const MB=1048576,keep=[];while(process.memoryUsage().heapUsed/MB<560){for(let at=0;at<200;at++)keep.push({id:at,name:"p"+Math.random(),rows:["a"+at,"b"+at,{x:at}]});}
const report=startSliceReportV1({root:process.argv[2],snapshot:()=>globalThis.Bun.generateHeapSnapshot(),intervalMilliseconds:250});
let guard=0,rss=0;const probe=setInterval(()=>{const usage=process.memoryUsage();guard=Math.max(guard,(usage.heapUsed+usage.external)/MB);rss=Math.max(rss,usage.rss/MB);},100);
await new Promise(done=>setTimeout(done,24000));clearInterval(probe);report.stop();
const usage=process.memoryUsage();console.log(JSON.stringify({guard:Math.max(guard,(usage.heapUsed+usage.external)/MB),rss:Math.max(rss,usage.rss/MB),kept:keep.length}));`);
 const run=spawnSync(process.execPath,[script,root],{encoding:"utf8",timeout:150000}),row=JSON.parse(run.stdout.trim().split("\n").pop()!);
 expect(row.kept).toBeGreaterThan(1000);expect(row.guard).toBeLessThan(2000);expect(row.rss).toBeLessThan(3000);
},170000);

test("collections are estimated by entries times the mean size of 16 sampled entries, sorted by bytes, at most 20 and bounded to 32 KiB, and a broken source does not stop the report",()=>{
 expect(estimateCollectionBytesV1(1000,["abcd","abcd"])).toBe(8000);
 expect(estimateCollectionBytesV1(10,[{a:1}])).toBe(70);
 expect(estimateCollectionBytesV1(5,[])).toBe(0);
 const big=new Map<number,string>(),small=new Map<number,string>();for(let at=0;at<3000;at++)big.set(at,"x".repeat(500));for(let at=0;at<10;at++)small.set(at,"y");
 registerSliceCollectionsV1("test-many",()=>[describeCollectionV1("big",big),describeCollectionV1("small",small),{name:"declared",entries:1,bytes:5}]);
 registerSliceCollectionsV1("test-broken",()=>{throw new Error("nope");});
 registerSliceCollectionsV1("test-flood",()=>Array.from({length:400},(_all,at)=>({name:"flood-"+at+"-".repeat(70),entries:at,bytes:at})));
 const rows=collectSliceCollectionsV1();
 expect(rows.length).toBeLessThanOrEqual(P.collectionsTop);
 expect(JSON.stringify(rows).length).toBeLessThanOrEqual(P.collectionsBytes);
 expect(rows[0]!.name).toBe("big");expect(rows[0]!.entries).toBe(3000);expect(Math.abs(rows[0]!.estimatedBytes/(3000*JSON.stringify([0,"x".repeat(500)]).length)-1)).toBeLessThan(0.01);
 for(let at=1;at<rows.length;at++)expect(rows[at-1]!.estimatedBytes).toBeGreaterThanOrEqual(rows[at]!.estimatedBytes);
 const report=buildSliceReportV1({reason:"interval"});
 expect(validateJsonSchemaSubset({$ref:"#/definitions/Report"},report,schema)).toEqual([]);
 expect((report.collections as {name:string}[])[0]!.name).toBe("big");
});

/** 💓️ A worker that takes owners tells the count that rose, once per count, and nothing while the source is absent or the count stands. */
test("the take counters are told as progress while they rise and not while they stand",async()=>{
 let ledger:{done:number;total:number}|undefined,throwing=false;const told:{phase:string;done:number;total:number}[]=[];
 const stop=startSliceProgressV1({emit:event=>told.push(event),read:()=>{if(throwing)throw new Error("nope");return ledger===undefined?undefined:{ledger};},milliseconds:10});
 try{
  await new Promise(resolve=>setTimeout(resolve,60));expect(told).toEqual([]);
  ledger={done:3,total:478};await new Promise(resolve=>setTimeout(resolve,60));
  ledger={done:9,total:478};await new Promise(resolve=>setTimeout(resolve,60));
  expect(told).toEqual([{phase:"taken",done:3,total:478},{phase:"taken",done:9,total:478}]);
  ledger={done:9,total:0};await new Promise(resolve=>setTimeout(resolve,40));expect(told.length).toBe(2);
 }finally{stop();}
 const after=told.length;ledger={done:11,total:478};await new Promise(resolve=>setTimeout(resolve,40));expect(told.length,"stopped").toBe(after);
 expect(P.progressMilliseconds).toBeGreaterThan(0);expect(P.progressMilliseconds).toBeLessThanOrEqual(10000);
});
