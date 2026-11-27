/** 🧱️ Contract: every worker tells the fleet what its `.cache/b` partitions hold (files, logical and allocated bytes, biggest holders) in its store record, within a budget, so a full partition of a machine nobody can reach is read from the store. The independent oracle is the Python standard library walking the same tree.
 * @see ../🟦️.ts */
import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{PARTITION_CENSUS_POLICY_V1 as policy,createPartitionCensusMirrorV1,partitionCensusV1}from"../🟦️.ts";

const build=():string=>{
 const root=mkdtempSync(join(tmpdir(),"partition-census-")),put=(path:string,size:number)=>{mkdirSync(join(root,path,".."),{recursive:true});writeFileSync(join(root,path),Buffer.alloc(size,1));};
 put(".cache/b/l/m/data/a.sqlite",8192);put(".cache/b/l/m/data/b.sqlite",100);put(".cache/b/l/m/claims/c",1);put(".cache/b/l/s/x/y",5000);put(".cache/b/r/big/one/f",70000);put(".cache/b/r/big/two/g",4096);put(".cache/b/r/small/h",0);
 return root;
};
const oracle=(root:string):Record<string,{files:number;bytes:number;allocated:number}>=>{
 const run=spawnSync("python3",["-I","-c","import json,os,sys\nbase=os.path.join(sys.argv[1],'.cache','b');out={}\nfor name in sorted(os.listdir(base)):\n f=b=a=0\n for d,ds,fs in os.walk(os.path.join(base,name)):\n  for x in fs:\n   s=os.lstat(os.path.join(d,x)).st_size;f+=1;b+=s;a+=-(-s//4096)*4096\n out[name]={'files':f,'bytes':b,'allocated':a}\nprint(json.dumps(out))",root],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 return JSON.parse(run.stdout);
};

test("the census agrees with an independent walk on files, bytes and allocated bytes, and names the biggest holders first",async()=>{
 const root=build();
 try{
  const record=await partitionCensusV1(root,{origin:"g1-test",platform:"win32"}),expected=oracle(root);
  expect(record.partitions.map(row=>row.name)).toEqual(Object.keys(expected));
  for(const row of record.partitions){expect([row.files,row.bytes,row.allocatedBytes]).toEqual([expected[row.name]!.files,expected[row.name]!.bytes,expected[row.name]!.allocated]);expect(row.partial).toBe(false);}
  const rust=record.partitions.find(row=>row.name==="r")!;
  expect(rust.top[0]!.path).toBe("big/one");expect(rust.top[0]!.allocatedBytes).toBe(73728);
  expect(record.partitions.find(row=>row.name==="l")!.top.map(row=>row.path)).toContain("m/data");
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("an entry budget cuts a partition short and says so instead of walking on",async()=>{
 const root=build();
 try{
  const record=await partitionCensusV1(root,{origin:"g1-test",entryBudget:3});
  expect(record.partitions.some(row=>row.partial)).toBe(true);
  for(const row of record.partitions)expect(row.files+row.directories).toBeLessThanOrEqual(3);
  const stopped=await partitionCensusV1(root,{origin:"g1-test",signal:AbortSignal.abort()});
  expect(stopped.partitions.every(row=>row.partial)).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a huge partition never starves the next ones, and the folders outside the partitions are counted with the volume facts",async()=>{
 const root=build(),put=(path:string,size:number)=>{mkdirSync(join(root,path,".."),{recursive:true});writeFileSync(join(root,path),Buffer.alloc(size,1));};
 put(".cache/b/a/f1",1);put(".cache/b/a/f2",1);put(".cache/b/a/f3",1);put(".cache/b/a/f4",1);put(".cache/other/x/y",100);put("target/debug/big",9000);put("node_modules/p/i.js",10);
 try{
  const record=await partitionCensusV1(root,{origin:"g1-test",entryBudget:2,volume:()=>({freeBytes:1,totalBytes:2,freeInodes:3,totalInodes:4})});
  const names=record.partitions.map(row=>row.name);
  expect(names).toEqual(["a","l","r","cache/other","root/node_modules","root/target"]);
  expect(record.partitions.find(row=>row.name==="a")!.partial).toBe(true);
  expect(record.partitions.find(row=>row.name==="r")!.directories).toBeGreaterThan(0);
  const target=record.partitions.find(row=>row.name==="root/target")!;
  expect(target.files).toBe(1);expect(target.bytes).toBe(9000);expect(target.partial).toBe(false);
  expect(record.volume).toEqual({freeBytes:1,totalBytes:2,freeInodes:3,totalInodes:4});
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the mirror publishes a valid record after the first delay, not again before the interval, and never without a store",async()=>{
 const root=build(),store=mkdtempSync(join(tmpdir(),"partition-store-"));
 try{
  let at=1_000_000;const mirror=createPartitionCensusMirrorV1({root,store,origin:"g1-test",now:()=>at}),file=join(store,"v1","origins","g1-test",policy.fileName);
  expect(await mirror.tick()).toBe(false);expect(existsSync(file)).toBe(false);
  at+=policy.firstDelayMilliseconds;
  expect(await mirror.tick()).toBe(true);
  const written=JSON.parse(readFileSync(file,"utf8"));expect(written.origin).toBe("g1-test");expect(written.partitions.length).toBe(2);
  at+=policy.intervalMilliseconds-1;expect(await mirror.tick()).toBe(false);
  at+=1;expect(await mirror.tick()).toBe(true);
  expect(await createPartitionCensusMirrorV1({root,store:undefined,origin:"x",now:()=>at+10**9}).tick()).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});rmSync(store,{recursive:true,force:true});}
});
