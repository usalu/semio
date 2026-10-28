import {test,expect} from "bun:test";
import {spawnSync} from "node:child_process";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {Database} from "bun:sqlite";
import {leaseDatabaseUnopenableBusyV1,leaseFileBusyV1,leaseFilePortsV1,leaseRemovalOptionsV1,leaseUnopenablePausesV1,settleLeaseFileV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import {testPythonV1} from "../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {receiveScriptProcessInvocation,createIssuedResourceControlV1} from "../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";

/** 🚧️ The lease store on a Windows host: sharing violations of a removal, a rename or a database open are retried on the declared pauses and then kept for the next sweep, never thrown, while the same codes on POSIX stay real refusals. The Windows rules run on any host through the platform argument and the file ports; the retirement runs over REAL lease databases, and Python's own sqlite3 reads the kept database back. */
const policy=schema.definitions.Policy.const,python=testPythonV1()??Bun.which("python3")??"python3",reader=join(import.meta.dir,"🐍️oracle","🐍️.py");
const failure=(code:string):Error=>Object.assign(new Error(code+": sharing violation"),{code});
const sleeps=()=>{const rows:number[]=[];return{rows,sleep:async(milliseconds:number)=>{rows.push(milliseconds);}};};

test("busy classification: the sharing violations of a Windows host are busy, the same codes on POSIX are real refusals (table equals the churn race table)",async()=>{
 const codes=["EBUSY","EPERM","EACCES","ENOENT","ENOTEMPTY","EEXIST","ENOSPC","EIO"];
 for(const platform of ["win32","darwin","linux"])for(const code of codes)expect(leaseFileBusyV1(failure(code),platform),platform+" "+code).toBe(platform==="win32"&&["EBUSY","EPERM","EACCES"].includes(code));
 expect(leaseFileBusyV1(new Error("no code"),"win32")).toBe(false);expect(leaseFileBusyV1(null,"win32")).toBe(false);expect(leaseFileBusyV1("EPERM","win32")).toBe(false);
 const churn=await import("../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts") as Record<string,unknown>;
 if(typeof churn.filesystemBusyErrorV1==="function"){const busy=churn.filesystemBusyErrorV1 as (error:unknown,platform:string)=>boolean;for(const platform of ["win32","darwin","linux"])for(const code of codes)expect(leaseFileBusyV1(failure(code),platform),"churn "+platform+" "+code).toBe(busy(failure(code),platform));}
});

test("unopenable classification: a REAL SQLite open failure is transient only under Windows rules, and the removal options carry Node's bounded retry only there",()=>{
 let real:unknown;try{new Database(join(tmpdir(),"semio-no-such-directory-"+process.pid,"x.sqlite"),{readwrite:true,create:false});}catch(error){real=error;}
 expect((real as {code?:string}).code).toBe("SQLITE_CANTOPEN");
 expect(leaseDatabaseUnopenableBusyV1(real,"win32")).toBe(true);expect(leaseDatabaseUnopenableBusyV1(real,"darwin")).toBe(false);expect(leaseDatabaseUnopenableBusyV1(real,"linux")).toBe(false);
 expect(leaseDatabaseUnopenableBusyV1({errcode:14|(2<<8)},"win32")).toBe(true);expect(leaseDatabaseUnopenableBusyV1({code:"SQLITE_BUSY",errcode:5},"win32")).toBe(false);expect(leaseDatabaseUnopenableBusyV1(null,"win32")).toBe(false);
 expect(leaseRemovalOptionsV1("darwin")).toEqual({force:true});expect(leaseRemovalOptionsV1("linux")).toEqual({force:true});expect(leaseRemovalOptionsV1("win32")).toEqual({force:true,maxRetries:3,retryDelay:50});
 expect(leaseUnopenablePausesV1).toEqual(policy.pausesMilliseconds);
});

test("settleLeaseFileV1 retries a sharing violation on the declared pauses, returns the last one when it persists, and never retries a real refusal",async()=>{
 for(const failures of [0,1,2,4]){const clock=sleeps();let calls=0;const verdict=await settleLeaseFileV1(()=>{if(calls++<failures)throw failure("EPERM");return"done";},{platform:"win32",sleep:clock.sleep});
  if(failures<=policy.pausesMilliseconds.length){expect(verdict).toEqual({settled:true,value:"done"});expect(clock.rows).toEqual(policy.pausesMilliseconds.slice(0,failures));}
  else{expect(verdict.settled).toBe(false);}}
 const clock=sleeps();let calls=0;const verdict=await settleLeaseFileV1(()=>{calls++;throw failure("EBUSY");},{platform:"win32",sleep:clock.sleep});
 expect(verdict.settled).toBe(false);expect((verdict as {error:{code:string}}).error.code).toBe("EBUSY");expect(calls).toBe(policy.pausesMilliseconds.length+1);expect(clock.rows).toEqual(policy.pausesMilliseconds);
 for(const [platform,code] of [["win32","ENOENT"],["win32","ENOSPC"],["darwin","EPERM"],["linux","EBUSY"]] as const){const quiet=sleeps();let attempts=0;await expect(settleLeaseFileV1(()=>{attempts++;throw failure(code);},{platform,sleep:quiet.sleep})).rejects.toMatchObject({code});expect(attempts).toBe(1);expect(quiet.rows).toEqual([]);}
 const controller=new AbortController();controller.abort(new Error("cancelled"));await expect(settleLeaseFileV1(()=>{throw failure("EPERM");},{platform:"win32",sleep:leaseFilePortsV1.sleep},controller.signal)).rejects.toThrow("cancelled");
});

test.skipIf(process.platform==="win32")("a REAL permission refusal (read-only directory) is final under POSIX rules and retried under Windows rules",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"semio-lease-win-")),file=join(directory,"x");writeFileSync(file,"x");chmodSync(directory,0o555);
 try{
  if(process.getuid?.()===0)return;
  const posix=sleeps(),windows=sleeps();let attempts=0;
  await expect(settleLeaseFileV1(()=>leaseFilePortsV1.unlink(file),{platform:process.platform,sleep:posix.sleep})).rejects.toMatchObject({code:"EACCES"});expect(posix.rows).toEqual([]);
  const verdict=await settleLeaseFileV1(()=>{attempts++;return leaseFilePortsV1.unlink(file);},{platform:"win32",sleep:windows.sleep});
  expect(verdict.settled).toBe(false);expect(attempts).toBe(policy.pausesMilliseconds.length+1);expect(windows.rows).toEqual(policy.pausesMilliseconds);expect(existsSync(file)).toBe(true);
 }finally{chmodSync(directory,0o755);rmSync(directory,{recursive:true,force:true});}
});

test("retirement of REAL lease databases: a busy unlink is retried, a persistent one keeps the lease for the next sweep, POSIX rules still refuse",async()=>receiveScriptProcessInvocation(process.env,async original=>{
 const api=await import("../../🟦️.ts"),control=createIssuedResourceControlV1(original,"test:lease-windows",60000),root=mkdtempSync(join(tmpdir(),"semio-lease-win-"));
 const create=async(name:string)=>{const directory=join(root,name);mkdirSync(directory);(await api.acquireResourceLease({directory,resource:"windows:"+name,mode:"exclusive",signal:control.signal,control})).release();return{directory,path:api.resourceLeasePathV1(directory,"windows:"+name)};};
 const observe=(path:string)=>JSON.parse(spawnSync(python,["-I",reader,path],{encoding:"utf8"}).stdout) as {applicationId:number;tables:string[]};
 try{
  const eventual=await create("eventual"),clock=sleeps();let unlinks=0;
  const first=await api.retireIdleResourceLeasesV1(eventual.directory,control,20000,{files:{platform:"win32",sleep:clock.sleep,unlink:path=>{if(++unlinks<=2)throw failure("EPERM");leaseFilePortsV1.unlink(path);}}});
  expect(first.retired).toBe(1);expect(unlinks).toBe(3);expect(clock.rows).toEqual(policy.pausesMilliseconds.slice(0,2));expect(existsSync(eventual.path)).toBe(false);

  const stuck=await create("stuck"),later=sleeps();let attempts=0;
  const kept=await api.retireIdleResourceLeasesV1(stuck.directory,control,20000,{files:{platform:"win32",sleep:later.sleep,unlink:()=>{attempts++;throw failure("EBUSY");}}});
  expect(kept.retired).toBe(0);expect(kept.kept).toBeGreaterThanOrEqual(1);expect(attempts).toBe(policy.pausesMilliseconds.length+1);expect(later.rows).toEqual(policy.pausesMilliseconds);
  expect(existsSync(stuck.path)).toBe(true);expect(observe(stuck.path)).toEqual({applicationId:0x534d4c53,tables:["semio_resource_lease"]});
  const again=await api.retireIdleResourceLeasesV1(stuck.directory,control,20000,{});expect(again.retired).toBe(1);expect(existsSync(stuck.path)).toBe(false);

  const posixRules=await create("posix"),quiet=sleeps();let tries=0;
  await expect(api.retireIdleResourceLeasesV1(posixRules.directory,control,20000,{files:{platform:"linux",sleep:quiet.sleep,unlink:()=>{tries++;throw failure("EPERM");}}})).rejects.toMatchObject({code:"EPERM"});
  expect(tries).toBe(1);expect(quiet.rows).toEqual([]);expect(existsSync(posixRules.path)).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
}),60000);
