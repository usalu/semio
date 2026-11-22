import {expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {testPythonV1} from "../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {leaseOrderPolicyV1} from "../../🟦️.ts";
import {leaseRegistryFolderV1,pruneLeaseRegistryIfDueV1,pruneLeaseRegistryV1} from "../../🗂️registry/🟦️.ts";

/** 🧹️ Registry pruning removes only rows of gone processes and folders of gone lease directories, at most a bounded batch per pass, resuming where the last pass stopped; the survivors equal the independent Python reading of the same rule. */
const rules={maximumFolders:2,budgetMilliseconds:1000,idleMilliseconds:0};
const dead=2147483646;
const row=(pid:number,directory:string,now:number)=>JSON.stringify({version:1,pid,startedAtMs:7,updatedAtMs:now,command:"x",directory,standing:{version:1},holds:[],waits:[]});
function build(){
 const root=mkdtempSync(join(tmpdir(),"prune-")),base=join(root,"world"),now=Date.now()+1000;
 const folders:{name:string;directory:string;exists:boolean;live:boolean;dead:number}[]=[];
 const add=(label:string,exists:boolean,live:boolean,deadRows:number,parent=true)=>{
  const directory=parent?join(base,label):join(root,"gone-"+label,"leases");
  if(exists)mkdirSync(directory,{recursive:true});else if(parent)mkdirSync(base,{recursive:true});
  const folder=leaseRegistryFolderV1(join(root,"registry"),directory);mkdirSync(folder,{recursive:true});
  writeFileSync(join(folder,"directory.json"),JSON.stringify({version:1,directory}));
  for(let index=0;index<deadRows;index++)writeFileSync(join(folder,`${dead-index}-7.json`),row(dead-index,directory,now));
  if(live)writeFileSync(join(folder,`${process.pid}-8.json`),row(process.pid,directory,now));
  folders.push({name:folder,directory,exists,live,dead:deadRows});
 };
 add("a",true,false,2);add("b",false,false,1,false);add("c",false,true,0);add("d",false,false,0,false);add("e",true,true,1);add("f",false,false,3);
 return {root,registry:join(root,"registry"),now,folders};
}
const survivors=(rows:ReturnType<typeof build>["folders"])=>rows.filter(row=>row.exists||row.live).map(row=>row.name).sort();

test("a bounded prune pass removes gone folders and dead rows, never a folder with a live row, and resumes after its cursor",()=>{
 const world=build();
 try{
  let cursor="",passes=0,more=true;
  while(more){const result=pruneLeaseRegistryV1(world.registry,rules,world.now,cursor);expect(result.examined).toBeLessThanOrEqual(rules.maximumFolders);cursor=result.cursor;more=result.more;if(++passes>10)throw Error("prune did not converge");}
  expect(passes).toBeGreaterThan(1);
  const remaining=world.folders.filter(row=>existsSync(row.name)).map(row=>row.name).sort();
  expect(remaining).toEqual(survivors(world.folders));
  for(const row of world.folders.filter(row=>existsSync(row.name)))for(const index of [0,1,2])expect(existsSync(join(row.name,`${dead-index}-7.json`))).toBe(false);
  for(const row of world.folders.filter(row=>row.live))expect(existsSync(join(row.name,`${process.pid}-8.json`))).toBe(true);
 }finally{rmSync(world.root,{recursive:true,force:true});}
});

test("the surviving folders equal the independent Python reading of the same rule",()=>{
 const python=testPythonV1();if(python===null)throw Error("python is required for the prune oracle");
 const world=build();
 try{
  const facts=world.folders.map(row=>({name:row.name,directory:row.directory,live:row.live}));
  const reading=Bun.spawnSync([python,"-c","import json,os,sys\nrows=json.loads(sys.argv[1])\nprint(json.dumps(sorted(r['name'] for r in rows if r['live'] or os.path.exists(r['directory']))))",JSON.stringify(facts)],{stdout:"pipe",stderr:"pipe"});
  expect(reading.exitCode,reading.stderr.toString()).toBe(0);
  const all={maximumFolders:100,budgetMilliseconds:1000,idleMilliseconds:0},result=pruneLeaseRegistryV1(world.registry,all,world.now);
  expect(result.more).toBe(false);
  expect(world.folders.filter(row=>existsSync(row.name)).map(row=>row.name).sort()).toEqual((JSON.parse(reading.stdout.toString()) as string[]).sort());
 }finally{rmSync(world.root,{recursive:true,force:true});}
});

test("a folder whose directory is missing but whose parent exists is kept until it has been idle for the declared time",()=>{
 const world=build();
 try{
  const idle={maximumFolders:100,budgetMilliseconds:1000,idleMilliseconds:3600000};
  pruneLeaseRegistryV1(world.registry,idle,world.now);
  const young=world.folders.filter(row=>!row.exists&&!row.live&&row.directory.includes("world"));
  expect(young.length).toBeGreaterThan(0);
  for(const row of young)expect(existsSync(row.name),row.directory).toBe(true);
  pruneLeaseRegistryV1(world.registry,idle,world.now+3600001);
  for(const row of young)expect(existsSync(row.name),row.directory).toBe(false);
 }finally{rmSync(world.root,{recursive:true,force:true});}
});

test("the due pass retires the whole backlog in one pass with bounded slices and is claimed through a stamp: concurrent or early callers skip it, an aborted pass comes due sooner than a finished registry",async()=>{
 const world=build(),policy=leaseOrderPolicyV1({}),tight={...policy,registry:{...policy.registry,prune:{...policy.registry.prune,maximumFolders:1,budgetMilliseconds:1000,idleMilliseconds:0,intervalMilliseconds:100000,backlogMilliseconds:1000}}};
 try{
  const first=await pruneLeaseRegistryIfDueV1(world.registry,tight,world.now);
  expect(first?.more).toBe(false);
  expect(first?.examined).toBe(world.folders.length);
  expect(world.folders.filter(row=>existsSync(row.name)).map(row=>row.name).sort()).toEqual(survivors(world.folders));
  expect(JSON.parse(readFileSync(join(world.registry,"prune.json"),"utf8")).more).toBe(false);
  expect(await pruneLeaseRegistryIfDueV1(world.registry,tight,world.now+10)).toBeUndefined();
  expect(await pruneLeaseRegistryIfDueV1(world.registry,tight,world.now+1001)).toBeUndefined();
  expect(await pruneLeaseRegistryIfDueV1(world.registry,tight,world.now+100001)).toBeDefined();
 }finally{rmSync(world.root,{recursive:true,force:true});}
});

test("an aborted pass leaves the stamp saying more so the next pass resumes soon, and slices yield to the event loop",async()=>{
 const world=build(),policy=leaseOrderPolicyV1({}),tight={...policy,registry:{...policy.registry,prune:{...policy.registry.prune,maximumFolders:1,budgetMilliseconds:1000,idleMilliseconds:0,intervalMilliseconds:100000,backlogMilliseconds:1000}}};
 try{
  const controller=new AbortController();let ticks=0;
  const timer=setInterval(()=>{ticks++;},0);
  const pending=pruneLeaseRegistryIfDueV1(world.registry,tight,world.now,controller.signal);
  controller.abort();
  const cut=await pending;clearInterval(timer);
  expect(cut?.more).toBe(true);
  expect(JSON.parse(readFileSync(join(world.registry,"prune.json"),"utf8")).more).toBe(true);
  const rest=await pruneLeaseRegistryIfDueV1(world.registry,tight,world.now+1001);
  expect(rest?.more).toBe(false);
  expect(world.folders.filter(row=>existsSync(row.name)).map(row=>row.name).sort()).toEqual(survivors(world.folders));
 }finally{rmSync(world.root,{recursive:true,force:true});}
});

test("pruning never throws on an unreadable or missing registry and a stray file in the root",()=>{
 const root=mkdtempSync(join(tmpdir(),"prune-")),registry=join(root,"registry");
 try{
  expect(pruneLeaseRegistryV1(join(root,"missing"),rules)).toMatchObject({examined:0,more:false});
  mkdirSync(registry,{recursive:true});writeFileSync(join(registry,"stray.txt"),"x");writeFileSync(join(registry,"0123456789abcdef01234567"),"not a folder");
  expect(pruneLeaseRegistryV1(registry,rules)).toMatchObject({removedFolders:0});
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the declared policy retires the folder of a vanished temporary lease directory within a quarter of an hour, not a day",()=>{
 const policy=leaseOrderPolicyV1({}),minute=60000,world=build();
 try{
  expect(policy.registry.prune.idleMilliseconds).toBeLessThanOrEqual(15*minute);
  const young=world.folders.filter(row=>!row.exists&&!row.live&&row.directory.includes("world"));
  expect(young.length).toBeGreaterThan(0);
  pruneLeaseRegistryV1(world.registry,{...policy.registry.prune,maximumFolders:100,budgetMilliseconds:1000},world.now+14*minute);
  for(const row of young)expect(existsSync(row.name),row.directory).toBe(true);
  pruneLeaseRegistryV1(world.registry,{...policy.registry.prune,maximumFolders:100,budgetMilliseconds:1000},world.now+16*minute);
  for(const row of young)expect(existsSync(row.name),row.directory).toBe(false);
 }finally{rmSync(world.root,{recursive:true,force:true});}
});
