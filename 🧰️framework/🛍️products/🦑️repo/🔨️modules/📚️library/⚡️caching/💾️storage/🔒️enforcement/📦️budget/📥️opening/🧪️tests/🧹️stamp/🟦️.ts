import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtempSync,readFileSync,rmSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import fixture from "./🔣️.json" with {type:"json"};
import {leaseRetirementFreshV1,touchLeaseRetirementV1} from "../../🟦️.ts";

const python=["python3","python"].find(name=>spawnSync(name,["-c","pass"]).status===0);
const oracleSource="import os,sys\nrow=os.lstat(sys.argv[1]);age=float(sys.argv[2])*1000-row.st_mtime*1000\nprint('1' if (not os.path.islink(sys.argv[1]) and 0<=age<=float(sys.argv[3])+1000 and age<=float(sys.argv[3])) else '0')";

for(const row of fixture.cases)test("lease retirement stamp: "+row.name,()=>{
 const directory=mkdtempSync(join(tmpdir(),"lease-stamp-")),stamp=join(directory,".retired"),now=Date.now();
 try{
  if(row.kind==="absent"){expect(leaseRetirementFreshV1(stamp,now)).toBe(false);return;}
  if(row.kind==="link"){writeFileSync(join(directory,"target"),"");symlinkSync(join(directory,"target"),stamp);expect(leaseRetirementFreshV1(stamp,now)).toBe(false);return;}
  writeFileSync(stamp,"");const mtime=new Date(now-row.ageMilliseconds!);utimesSync(stamp,mtime,mtime);
  expect(leaseRetirementFreshV1(stamp,now)).toBe(row.fresh!);
  if(python&&row.kind==="file"){const verdict=spawnSync(python,["-I","-c",oracleSource,stamp,String(now/1000),String(fixture.intervalMilliseconds)],{encoding:"utf8"}).stdout.trim();expect(verdict).toBe(row.fresh?"1":"0");}
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("lease retirement stamp: touching makes it fresh",()=>{
 const directory=mkdtempSync(join(tmpdir(),"lease-stamp-")),stamp=join(directory,".retired");
 try{touchLeaseRetirementV1(stamp);expect(leaseRetirementFreshV1(stamp,Date.now())).toBe(true);}finally{rmSync(directory,{recursive:true,force:true});}
});

test("lease retirement stamp: the opener sweeps under the ordinary protocol, shortens the interval under pressure and never fails because of the sweep",()=>{
 const source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),block=source.slice(source.indexOf("async function openRepositoryLeasePartitionV1"),source.indexOf("export async function openRepositoryCachePartitionV1"));
 expect(block).toContain("sweepResourceLeaseStoreV1(");expect(block).toContain("resourceLeaseRetirementIdlePolicyV1.pressureSweepIntervalMilliseconds");expect(block).toContain("formatResourceLeaseSweepFailureV1");
 expect(block).not.toContain("withExclusiveMaintenance");expect(block.indexOf("touchLeaseRetirementV1(stamp)")).toBeLessThan(block.indexOf("sweepResourceLeaseStoreV1("));
 const directory=mkdtempSync(join(tmpdir(),"lease-stamp-")),stamp=join(directory,".retired");
 try{writeFileSync(stamp,"");const old=new Date(Date.now()-60000);utimesSync(stamp,old,old);expect(leaseRetirementFreshV1(stamp,Date.now())).toBe(true);expect(leaseRetirementFreshV1(stamp,Date.now(),30000)).toBe(false);}finally{rmSync(directory,{recursive:true,force:true});}
});
