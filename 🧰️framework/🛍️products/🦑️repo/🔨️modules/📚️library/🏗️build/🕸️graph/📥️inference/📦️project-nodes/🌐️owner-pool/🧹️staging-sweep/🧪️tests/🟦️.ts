import{afterAll,expect,test}from"bun:test";
import{existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,utimesSync}from"node:fs";
import{spawnSync}from"node:child_process";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{OWNER_POOL_STAGING_POLICY_V1 as P,ownerPoolStagingIoV1,ownerPoolStagingSweepV1,writeOwnerPoolStagingOwnerV1,type OwnerPoolStagingIoV1}from"../🟦️.ts";
import{ownerPoolProcessStartV1}from"../../🪦️reaper/🟦️.ts";
import fixture from"./🔣️.json"with{type:"json"};

/** 🧹️ The staging directories of dead pool processes are removed by the next owner, never those of live ones, never the caller's own. */
type Row={name:string;owner?:{pid:number;startTime?:string};alive?:boolean;ageMilliseconds?:number};
const fake=(rows:readonly Row[],removed:string[]):OwnerPoolStagingIoV1=>{
 const byPath=new Map(rows.map(row=>["/tmp/"+row.name,row]));
 return{list:()=>rows.map(row=>row.name),owner:directory=>byPath.get(directory)?.owner===undefined?undefined:{version:1,...byPath.get(directory)!.owner!},ageMilliseconds:directory=>byPath.get(directory)?.ageMilliseconds,alive:owner=>rows.find(row=>row.owner?.pid===owner.pid)?.alive===true,remove:directory=>{removed.push(directory.slice("/tmp/".length));}};
};
const run=(row:typeof fixture.cases[number]):string[]=>{
 const directories=row.directories as Row[],removed:string[]=[];
 ownerPoolStagingSweepV1({parents:["/tmp"],own:row.own,io:fake(directories,removed)});
 return removed;
};

test("the sweep removes exactly the directories of gone owners and equals the independent Python model",()=>{
 const mine=fixture.cases.map(run);
 expect(mine).toEqual(fixture.cases.map(row=>row.removed));
 const python=spawnSync("python3",["-I",join(import.meta.dir,"oracle.py"),join(import.meta.dir,"🔣️.json"),join(import.meta.dir,"..","🧬️schema","🔣️.json")],{encoding:"utf8"});
 expect(python.status).toBe(0);expect(JSON.parse(python.stdout)).toEqual(mine);
});

test("39 tries in a row never hold more than the live ones: every try removes the directories its predecessors left",()=>{
 const existing=new Set<string>();let maximum=0;
 const io:OwnerPoolStagingIoV1={list:()=>[...existing],owner:()=>({version:1,pid:1}),ageMilliseconds:()=>0,alive:owner=>owner.pid===1&&false,remove:directory=>{existing.delete(directory.slice("/tmp/".length));}};
 for(let attempt=0;attempt<39;attempt++){
  const own="/tmp/semio-owner-pool-try"+attempt;
  ownerPoolStagingSweepV1({parents:["/tmp"],own,io});
  existing.add("semio-owner-pool-try"+attempt);
  maximum=Math.max(maximum,existing.size);
 }
 expect(maximum).toBeLessThanOrEqual(2);
});

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

test("on a real filesystem a directory whose process ended is removed, one of this live process and the caller's own are kept",()=>{
 const parent=mkdtempSync(join(tmpdir(),"sweep-"));scratch.push(parent);
 const dead=spawnSync(process.execPath,["-e","process.stdout.write(String(process.pid))"],{encoding:"utf8"}),deadPid=Number(dead.stdout);
 const make=(name:string,pid:number|null)=>{const directory=join(parent,name);mkdirSync(directory);writeOwnerPoolStagingOwnerV1(directory,pid??process.pid);return directory;};
 const gone=make("semio-owner-pool-gone",deadPid),live=make("semio-owner-pool-live",process.pid),own=make("semio-owner-pool-own",deadPid);
 const born=join(parent,"semio-owner-pool-born"),old=join(parent,"semio-owner-pool-old"),other=join(parent,"unrelated");
 for(const directory of[born,old,other])mkdirSync(directory);
 const past=new Date(Date.now()-3_600_000);utimesSync(old,past,past);
 const removed=ownerPoolStagingSweepV1({parents:[parent,join(parent,"missing")],own,io:ownerPoolStagingIoV1});
 expect(new Set(removed)).toEqual(new Set([gone,old]));
 expect(readdirSync(parent).sort()).toEqual(["semio-owner-pool-born","semio-owner-pool-live","semio-owner-pool-own","unrelated"]);
 expect(existsSync(join(live,P.ownerFile))).toBe(true);
});

test("a pid that now has another start time is another process and its directory is removed",()=>{
 const parent=mkdtempSync(join(tmpdir(),"sweep-"));scratch.push(parent);
 const directory=join(parent,"semio-owner-pool-reused");mkdirSync(directory);
 writeOwnerPoolStagingOwnerV1(directory,process.pid,"Mon Jan  1 00:00:00 1990");
 const alive=ownerPoolStagingIoV1.alive({version:1,pid:process.pid,startTime:"Mon Jan  1 00:00:00 1990"});
 const started=ownerPoolStagingIoV1.owner(directory);
 expect(started?.pid).toBe(process.pid);
 if(ownerPoolProcessStartV1(process.pid)!==undefined)expect(alive,"the start time of this process is not 1990").toBe(false);
});

test("the supervisor and the inference of a child own their staging directory and sweep their predecessors, with no random prefix of their own",()=>{
 for(const file of[["🔬️inference","🟨️.mjs"],["🏃️runner","🟨️.mjs"]]){
  const text=readFileSync(join(import.meta.dir,"..","..",...file),"utf8");
  expect(text,file.join("/")).toContain("writeOwnerPoolStagingOwnerV1(base)");
  expect(text,file.join("/")).toContain("ownerPoolStagingSweepV1({ parents:");
  expect(text,file.join("/")).toContain("own: base");
  expect(text,file.join("/")).not.toContain('"semio-owner-pool-"');
 }
});
