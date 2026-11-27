import{expect,test}from"bun:test";
import{existsSync,mkdtempSync,readdirSync,readFileSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createHeapProfileStateV1,profileHeapV1,summariseHeapSnapshotV1}from"../🟨️.mjs";

const holder=new Map<string,{name:string;rows:string[]}>();

test("the summary names the property that holds a large collection of a real snapshot",()=>{
 for(let at=0;at<20000;at++)holder.set("key-"+at+"-"+Math.random(),{name:"owner-"+at+Math.random(),rows:["row-a-"+at+Math.random(),"row-b-"+at+Math.random()]});
 (globalThis as unknown as{hold:unknown}).hold=holder;
 const summary=summariseHeapSnapshotV1(globalThis.Bun.generateHeapSnapshot());
 const top=summary.retainers.find(row=>row.path.includes("hold:Map"));
 expect(top).toBeDefined();
 expect(top!.bytes).toBeGreaterThan(1_000_000);
});

test("the holder groups name WHO holds the numerous strings and arrays of a real snapshot",()=>{
 const wave:{carryRows:string[];carryLists:number[][]}={carryRows:[],carryLists:[]};
 for(let at=0;at<30000;at++){wave.carryRows.push("carry-row-"+at+"-"+Math.random());wave.carryLists.push([at,at+1,at+2]);}
 (globalThis as unknown as{leakyWaves:unknown}).leakyWaves={wave1:wave};
 const summary=summariseHeapSnapshotV1(globalThis.Bun.generateHeapSnapshot());
 const row=summary.holders.string.find(group=>group.holder.includes("carryRows")&&group.holder.includes("leakyWaves"));
 expect(row).toBeDefined();
 expect(row!.count).toBeGreaterThanOrEqual(30000);
 expect(row!.holder).toContain("wave1");
 expect(row!.path).toContain("leakyWaves");
 const lists=summary.holders.Array.find(group=>group.holder.includes("carryLists"));
 expect(lists).toBeDefined();
 expect(lists!.count).toBeGreaterThanOrEqual(30000);
});

test("an aborted signal stops the summary",()=>{
 const controller=new AbortController();controller.abort();
 expect(()=>summariseHeapSnapshotV1(globalThis.Bun.generateHeapSnapshot(),{signal:controller.signal})).toThrow();
});

test("a snapshot per further step, bounded in number, with the process counters, written atomically",()=>{
 const directory=mkdtempSync(join(tmpdir(),"heap-profile-")),tiny={nodes:[0,0,0,0],edges:[],nodeClassNames:["<root>"],edgeNames:[],edgeTypes:[]},state=createHeapProfileStateV1(),common={directory,generate:()=>tiny,minimum:100,step:100,limit:3,state,memory:()=>({heapUsed:1048576*300,external:0,arrayBuffers:0,rss:1048576*400})};
 try{
  expect(profileHeapV1({...common,managedMegabytes:10})).toBeUndefined();
  expect(profileHeapV1({...common,directory:undefined,managedMegabytes:500})).toBeUndefined();
  expect(profileHeapV1({...common,managedMegabytes:150})?.nodes).toBe(1);
  expect(profileHeapV1({...common,managedMegabytes:200})).toBeUndefined();
  expect(profileHeapV1({...common,managedMegabytes:260})?.nodes).toBe(1);
  expect(profileHeapV1({...common,managedMegabytes:400})?.nodes).toBe(1);
  expect(profileHeapV1({...common,managedMegabytes:900})).toBeUndefined();
  const files=readdirSync(directory).sort();
  expect(files.length).toBe(3);
  expect(files.some(file=>file.endsWith(".tmp"))).toBe(false);
  const first=JSON.parse(readFileSync(join(directory,files[0]),"utf8"));
  expect(first.managedMegabytes).toBe(150);
  expect(first.process.heapUsedMegabytes).toBe(300);
  expect(first.process.unreachedMegabytes).toBe(150);
  expect(first.cost.generateMilliseconds).toBeGreaterThanOrEqual(0);
  expect(existsSync(join(directory,files[0]+".tmp"))).toBe(false);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("the budget of snapshot time stops further snapshots",()=>{
 const directory=mkdtempSync(join(tmpdir(),"heap-profile-")),tiny={nodes:[0,0,0,0],edges:[],nodeClassNames:["<root>"],edgeNames:[],edgeTypes:[]},state=createHeapProfileStateV1();let clock=0;
 try{
  const common={directory,generate:()=>tiny,minimum:0,step:1,limit:9,budget:1000,state,now:()=>(clock+=250)};
  expect(profileHeapV1({...common,managedMegabytes:1})).toBeDefined();
  expect(profileHeapV1({...common,managedMegabytes:5})).toBeDefined();
  expect(profileHeapV1({...common,managedMegabytes:9})).toBeUndefined();
 }finally{rmSync(directory,{recursive:true,force:true});}
});
