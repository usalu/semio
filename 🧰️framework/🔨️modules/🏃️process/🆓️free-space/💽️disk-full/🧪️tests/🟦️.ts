/** 🧪️ Tests for the full-disk classification: the language-agnostic cases in `🔣️.json` run here and in the independent Python oracle. */
import{spawnSync}from"node:child_process";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{afterAll,expect,test}from"bun:test";
import cases from"./🔣️.json"with{type:"json"};
import{diskFullOfV1,isDiskFullV1,volumeMountOfV1}from"../🟦️.ts";

const directories:string[]=[];
afterAll(()=>{for(const path of directories)rmSync(path,{recursive:true,force:true});});
const build=(row:{code?:string;message?:string;cause?:unknown}):unknown=>{const error=Object.assign(new Error(row.message??"failure"),row.code?{code:row.code}:{});return row.cause?Object.assign(error,{cause:build(row.cause as{code?:string;message?:string})}):error;};

test("authored failures are classified as a full disk by TypeScript and by the independent Python oracle",()=>{
 const run=spawnSync("python3",["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url))],{input:JSON.stringify(cases.cases),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const python=JSON.parse(run.stdout) as boolean[];
 cases.cases.forEach((row,index)=>{expect(isDiskFullV1(build(row)),row.name).toBe(row.full);expect(python[index],row.name).toBe(row.full);});
});

test("a full-disk failure names the partition of its path and the free bytes, anything else names nothing",()=>{
 const found=diskFullOfV1(Object.assign(new Error("ENOSPC: no space left on device, write"),{code:"ENOSPC",path:"/data/cache/x"}),{free:path=>({availableBytes:path==="/data/cache/x"?4096:0,totalBytes:1<<30}),mount:()=>"/data"});
 expect(found).toEqual({partition:"/data",freeBytes:4096,totalBytes:1<<30});
 expect(diskFullOfV1(new Error("database or disk is full"),{probes:["/probe"],free:()=>undefined,mount:path=>path})).toEqual({partition:"/probe",freeBytes:undefined,totalBytes:undefined});
 expect(diskFullOfV1(new Error("EACCES: permission denied"))).toBeUndefined();
});

test("the mount of a path is the highest ancestor on its device, also for a path that does not exist yet",()=>{
 const root=mkdtempSync(join(tmpdir(),"disk-full-"));directories.push(root);
 const mount=volumeMountOfV1(root);
 expect(root.startsWith(mount)).toBe(true);
 expect(volumeMountOfV1(join(root,"missing","deeper"))).toBe(mount);
});
