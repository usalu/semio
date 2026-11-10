/** 🗂️ Contract: the stat index reads a file only when its size, mtime or inode changed or its record was racy, and answers digests equal to a naive model written in Python; it persists per directory and survives a restart, a damaged record is ignored, and the derived graph's no-journal verification of unchanged files reads nothing. */
import{expect,test}from"bun:test";
import{mkdtempSync,rmSync,readdirSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{createDerivedGraphV1}from"../../🟨️.mjs";
import{KIND_RULES_V1}from"../../🧩️kinds/🟨️.mjs";
import{createStatIndexV1}from"../../🗂️stat-index/🟨️.mjs";
import fixture from"./🔣️.json";

type File={size:number;mtime:number;ino:number;content:string};
const oracle=join(dirname(fileURLToPath(import.meta.url)),"../🐍️oracle/🗂️.py");
const model=()=>{const run=Bun.spawnSync([testPythonV1()!,"-I",oracle,"run"],{stdin:Buffer.from(JSON.stringify(fixture))});expect(run.exitCode).toBe(0);return JSON.parse(run.stdout.toString()).steps as {digests:Record<string,string>;reads:string[]}[];};
const worldOf=(files:Record<string,File>,reads:string[])=>({stat:async(path:string)=>{const f=files[path];return f?{size:f.size,mtimeMs:f.mtime,ino:f.ino}:undefined;},read:async(path:string)=>{reads.push(path);const f=files[path];return f?Buffer.from(f.content):undefined;}});

test("the stat index decisions and digests equal the independent model, in one process and across a restart",async()=>{
 const root=mkdtempSync(join(tmpdir(),"derived-stat-")),want=model();
 try{
  for(const restartEvery of[0,1]){
   let clock=0,index=createStatIndexV1({root:join(root,String(restartEvery)),now:()=>clock});
   for(const[at,step]of fixture.steps.entries()){
    clock=step.now;
    if(restartEvery&&at>0){await index.flush();index=createStatIndexV1({root:join(root,String(restartEvery)),now:()=>clock});}
    const reads:string[]=[],world=worldOf(step.files as Record<string,File>,reads),digests:Record<string,string>={};
    for(const path of step.query)digests[path]=(await index.digest(path,world)).digest;
    expect(digests,"step "+at).toEqual(want[at]!.digests);
    expect([...reads].sort(),"step "+at+(restartEvery?" restarted":"")).toEqual(want[at]!.reads);
   }
  }
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("records are grouped per directory, a damaged record is ignored and the disk is pruned by count",async()=>{
 const root=mkdtempSync(join(tmpdir(),"derived-stat-")),file=(n:number):File=>({size:1,mtime:1,ino:n,content:"c"+n});
 try{
  const index=createStatIndexV1({root,now:()=>10_000_000}),files:Record<string,File>={"a/1":file(1),"a/2":file(2),"b/3":file(3)},reads:string[]=[];
  for(const path of Object.keys(files))await index.digest(path,worldOf(files,reads));
  await index.flush();
  const names=readdirSync(join(root,"d"));
  expect(names.length).toBe(2);
  for(const name of names)writeFileSync(join(root,"d",name),"damaged");
  const again=createStatIndexV1({root,now:()=>10_000_000}),second:string[]=[];
  await again.digest("a/1",worldOf(files,second));
  expect(second).toEqual(["a/1"]);
  expect((await createStatIndexV1({root,limits:{...(await import("../../🟨️.mjs")).DERIVED_NODES_POLICY_V1.statIndex,maximumRecords:1}}).prune()).kept).toBe(1);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the graph verifies unchanged files without reading them when it is given a stat index",async()=>{
 const files:Record<string,File>={"w/a.ts":{size:1,mtime:1,ino:1,content:"a"},"w/b.ts":{size:1,mtime:1,ino:2,content:"b"}},reads:string[]=[],root=mkdtempSync(join(tmpdir(),"derived-stat-"));
 try{
  const rules={walk:async(ctx:any)=>ctx.glob("",["w/**/*.ts"])},world={...worldOf(files,reads),files:async()=>Object.keys(files)};
  const graph=createDerivedGraphV1({rules:rules as any,world,statIndex:createStatIndexV1({root,now:()=>10_000_000}),trace:true}),key=graph.key("walk","x");
  const first=(await graph.get(key)).value;await graph.flush();
  expect(reads.length).toBe(2);
  const fresh=createDerivedGraphV1({rules:rules as any,world,statIndex:createStatIndexV1({root,now:()=>10_000_000})}),before=reads.length;
  expect((await fresh.get(fresh.key("walk","x"))).value).toEqual(first);
  expect(reads.length).toBe(before);expect(fresh.stats().statReused).toBe(2);
  files["w/b.ts"]={size:1,mtime:5,ino:2,content:"B"};
  const changed=createDerivedGraphV1({rules:rules as any,world,statIndex:createStatIndexV1({root,now:()=>10_000_000})});
  expect((await changed.get(changed.key("walk","x"))).value.digest).not.toBe(first.digest);
  expect(reads.length).toBe(before+1);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the capacity port follows the admitted grant of the host capacity module and falls back to the host",async()=>{
 const{capacityOfGrantV1}=await import("../../🟨️.mjs"),{grantJobLimitV1}=await import("../../../../🧱️job-limits/🎫️admission/🟦️.ts");
 const limit=grantJobLimitV1({reservedMegabytes:4096}),handle={cpu:6,jobs:4,reservedMegabytes:4096};
 const port=capacityOfGrantV1(()=>({handle,limit}),{rss:()=>1024*1048576,host:()=>({cpus:64,availableBytes:1})});
 expect(port()).toEqual({cpus:6,availableBytes:(limit.memoryMegabytes-1024)*1048576,reserveBytes:Math.floor(limit.memoryMegabytes*1048576/4)});
 expect(capacityOfGrantV1(()=>undefined,{host:()=>({cpus:3,availableBytes:7})})()).toEqual({cpus:3,availableBytes:7});
 let current={handle:{cpu:2,jobs:1,reservedMegabytes:1024}};
 const live=capacityOfGrantV1(()=>current,{rss:()=>0}),small=live();
 current={handle:{cpu:8,jobs:4,reservedMegabytes:8192}};
 expect(live().availableBytes).toBeGreaterThan(small.availableBytes);
 const widths:number[]=[];
 for(const grant of[{cpu:2,reservedMegabytes:1024},{cpu:16,reservedMegabytes:16384}]){
  let live=0,peak=0;
  const world={read:async()=>undefined,files:async(root:string)=>{live++;peak=Math.max(peak,live);await new Promise(done=>setTimeout(done,15));live--;return[root+"/a"];}};
  const graph=createDerivedGraphV1({rules:{x:async(ctx:any)=>ctx.glob("",[ctx.self.subject+"/**"])},world,capacity:capacityOfGrantV1(()=>({handle:{...grant,jobs:1}}),{rss:()=>0})});
  await graph.getMany(Array.from({length:12},(_,index)=>graph.key("x","d"+index)));widths.push(peak);
 }
 expect(widths[0]).toBe(1);expect(widths[1]).toBeGreaterThan(widths[0]!);
 void KIND_RULES_V1;
});

test("the disk world lists the files below a root like an independent walk and remembers unchanged directories",async()=>{
 const{createDiskWorldV1}=await import("../../🌍️world/🟨️.mjs"),{mkdirSync,utimesSync}=await import("node:fs");
 const root=mkdtempSync(join(tmpdir(),"derived-world-"));
 try{
  for(const path of["a/b/c.ts","a/d.ts","e/😀.ts","node_modules/x/y.js","a/target/z.rs","top.txt"]){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),path);}
  const old=new Date(Date.now()-3_600_000);
  for(const directory of["","a","a/b","e"])utimesSync(join(root,directory),old,old);
  const walk=Bun.spawnSync([testPythonV1()!,"-I","-c","import os,sys\nroot=sys.argv[1]\nout=[]\nfor d,ds,fs in os.walk(root):\n    ds[:]=[x for x in ds if x not in {'.git','node_modules','target','__pycache__','.venv','.cache'}]\n    out+=[os.path.relpath(os.path.join(d,f),root) for f in fs]\nprint('\\n'.join(sorted(out)))",root]);
  const expected=walk.stdout.toString().trim().split("\n");
  const plain=createDiskWorldV1(root),remembering=createDiskWorldV1(root,{remember:true});
  expect([...await plain.files("")].sort()).toEqual(expected);
  expect([...await remembering.files("")].sort()).toEqual(expected);
  writeFileSync(join(root,"a/new.ts"),"n");
  expect([...await remembering.files("")].sort()).toEqual([...expected,"a/new.ts"].sort());
  expect([...await remembering.files("a/b")]).toEqual(["a/b/c.ts"]);
  expect(await plain.stat("top.txt")).toMatchObject({size:7});expect(await plain.stat("a")).toBeUndefined();
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("an unchanged directory stamp lets the no-journal pass skip the roster scan and a changed one rescans",async()=>{
 const{createDiskWorldV1}=await import("../../🌍️world/🟨️.mjs"),{mkdirSync,utimesSync}=await import("node:fs");
 const root=mkdtempSync(join(tmpdir(),"derived-stamp-")),state=join(root,"state");
 try{
  for(const path of["ws/a/1.ts","ws/a/2.ts","ws/b/3.ts"]){mkdirSync(dirname(join(root,"ws-src",path)),{recursive:true});writeFileSync(join(root,"ws-src",path),path);}
  const base=join(root,"ws-src"),old=(hours:number)=>new Date(Date.now()-hours*3_600_000);
  const settle=(hours:number)=>{for(const directory of["","ws","ws/a","ws/b"])utimesSync(join(base,directory),old(hours),old(hours));};
  settle(5);
  const world=createDiskWorldV1(base,{remember:true}),rules={walk:async(ctx:any)=>ctx.glob("",["ws/**/*.ts"])};
  const graph=createDerivedGraphV1({rules:rules as any,world,statIndex:createStatIndexV1({root:state})}),key=graph.key("walk","x");
  const first=(await graph.get(key)).value;
  const one=await graph.invalidate({paths:[],overflow:true});
  expect(one.computed).toBe(0);expect(graph.stats().stampHeld).toBe(1);
  writeFileSync(join(base,"ws/a/3.ts"),"new");settle(7);
  const two=await graph.invalidate({paths:[],overflow:true});
  expect(two.computed).toBe(1);expect((await graph.get(key)).value.files).toBe(first.files+1);expect((await graph.get(key)).value.digest).not.toBe(first.digest);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a shard round-trips through the compact binary form and a damaged byte is refused",async()=>{
 const{encodeShardV1,decodeShardV1}=await import("../../🗂️stat-index/🟨️.mjs");
 const digest="ab".repeat(32),shard={files:new Map([["",new Map([["top.txt",[3,1.5,2 ** 40,digest,1_700_000_000_000]]])],["a/😀",new Map([["ü.ts",[0,1700000000000.25,7,digest,5]]])]]),dirs:new Map([["a/😀",{mtimeMs:12.5,at:99,entries:[["ü.ts",0],["sub",1]]}],["",{mtimeMs:1,at:2,entries:[]}]])};
 const bytes=encodeShardV1(shard as any),back=decodeShardV1(bytes,1<<20)!;
 expect([...back.files].map(([dir,map])=>[dir,[...map]])).toEqual([...shard.files].map(([dir,map])=>[dir,[...map]]).sort((a,b)=>a[0]!<b[0]!?-1:1));
 expect([...back.dirs].sort()).toEqual([...shard.dirs].sort());
 const damaged=Buffer.from(bytes);damaged[Math.floor(damaged.length/2)]^=0xff;
 expect(decodeShardV1(damaged,1<<20)).toBeUndefined();
 expect(bytes.length).toBeLessThan(400);
});

test("the directory table is durable: a new process verifies a quiet tree by stat alone with an equal Merkle stamp",async()=>{
 const{createDiskWorldV1}=await import("../../🌍️world/🟨️.mjs"),{mkdirSync,utimesSync}=await import("node:fs");
 const root=mkdtempSync(join(tmpdir(),"derived-merkle-")),base=join(root,"src"),state=join(root,"state");
 try{
  for(const path of["p/a/1.ts","p/a/2.ts","p/b/c/3.ts","q/4.ts"]){mkdirSync(dirname(join(base,path)),{recursive:true});writeFileSync(join(base,path),path);}
  const old=(hours:number)=>new Date(Date.now()-hours*3_600_000);
  const settle=(hours:number)=>{for(const directory of["","p","p/a","p/b","p/b/c","q"])utimesSync(join(base,directory),old(hours),old(hours));};
  settle(5);
  const first=createStatIndexV1({root:state}),one=createDiskWorldV1(base,{index:first});
  await one.files("");const stamp=one.stamp("p");expect(stamp).toBeDefined();expect(one.stamp("q")).not.toBe(stamp);
  await first.flush();
  const second=createStatIndexV1({root:state}),two=createDiskWorldV1(base,{index:second});
  expect(await two.verify("")).toBe(one.stamp(""));expect(two.stamp("p")).toBe(stamp);
  writeFileSync(join(base,"p/b/c/new.ts"),"x");for(const directory of["p/b/c","p/b","p",""])utimesSync(join(base,directory),old(7),old(7));
  await two.verify("");expect(two.stamp("p")).not.toBe(stamp);expect(two.stamp("q")).toBe(one.stamp("q"));
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the workspace factory wires world, stat index, node store and the admitted grant, and a second process restarts from the durable parts",async()=>{
 const{createWorkspaceGraphV1}=await import("../../🏭️workspace/🟨️.mjs"),{mkdirSync,utimesSync}=await import("node:fs");
 const root=mkdtempSync(join(tmpdir(),"derived-workspace-")),work=join(root,"ws"),cache=join(root,"cache");
 try{
  for(const path of["p/project.json","p/Cargo.toml","p/src/a.rs"]){mkdirSync(dirname(join(work,path)),{recursive:true});}
  writeFileSync(join(work,"p/project.json"),JSON.stringify({name:"p",namedInputs:{src:["src/**/*.rs"]},targets:{build:{command:"c",inputs:["src"]}}}));writeFileSync(join(work,"p/Cargo.toml"),"[package]\nname = \"p\"\n");writeFileSync(join(work,"p/src/a.rs"),"fn a(){}");
  const old=new Date(Date.now()-5*3_600_000);for(const path of["p/project.json","p/Cargo.toml","p/src/a.rs","","p","p/src"])utimesSync(join(work,path),old,old);
  const project={root:"p",manifests:["p/project.json","p/Cargo.toml"]},granted=()=>({handle:{cpu:4,jobs:2,reservedMegabytes:4096}});
  const one=createWorkspaceGraphV1({workspaceRoot:work,cacheRoot:cache,grant:granted}),key=one.graph.key("action-key","p:build",{root:"p",target:"build",project,env:{}});
  const first=(await one.graph.get(key)).value;await one.graph.flush();
  const two=createWorkspaceGraphV1({workspaceRoot:work,cacheRoot:cache,grant:granted}),again=await two.graph.get(two.graph.key("action-key","p:build",{root:"p",target:"build",project,env:{}}));
  expect(again.value).toEqual(first);expect(two.graph.stats().computed).toBe(0);expect(two.graph.stats().statReused).toBeGreaterThan(0);
  expect(readdirSync(join(cache,"stat-index","d")).length).toBeGreaterThan(0);expect(readdirSync(join(cache,"derived-nodes","n")).length).toBeGreaterThan(0);
 }finally{rmSync(root,{recursive:true,force:true});}
});
