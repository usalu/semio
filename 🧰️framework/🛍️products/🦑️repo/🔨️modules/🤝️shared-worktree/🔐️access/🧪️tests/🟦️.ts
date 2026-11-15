import {afterAll,expect,test} from "bun:test";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {compileJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {testPythonV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {awaitWorktreeAccessV1,clearAccessV1,listAccessV1,probeWorktreeAccessV1,readAccessV1,writeAccessV1,type AccessRecordV1,type AccessStateV1} from "../🟦️.ts";

const scratch:string[]=[],python=testPythonV1(),oracle=join(import.meta.dirname,"🐍️.py");
const temp=(label:string):string=>{const path=mkdtempSync(join(tmpdir(),`access-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch){try{chmodSync(path,0o755);}catch{}rmSync(path,{recursive:true,force:true});}});
const refusal=(code:string)=>()=>Promise.reject(Object.assign(Error(code),{code}));
const record=(state:AccessStateV1="denied"):AccessRecordV1=>({version:1,originId:"g1-m01",state,wallMs:1_700_000_000_000,worktree:"/Users/a b/Documents/semio",platform:"darwin",error:"EPERM"});
const store=():string=>{const root=temp("store");mkdirSync(join(root,"v1","origins","g1-m01"),{recursive:true});mkdirSync(join(root,"v1","tmp"),{recursive:true});return root;};

test("a refused listing is denied, a listing that does not return is waiting, a missing directory and a readable one are fine",async()=>{
 const folder=temp("folder");
 expect(await probeWorktreeAccessV1(folder,500)).toBe("ok");
 expect(await probeWorktreeAccessV1(join(folder,"missing"),500)).toBe("ok");
 expect(await probeWorktreeAccessV1(folder,500,refusal("EPERM"))).toBe("denied");
 expect(await probeWorktreeAccessV1(folder,500,refusal("EACCES"))).toBe("denied");
 expect(await probeWorktreeAccessV1(folder,500,refusal("EIO"))).toBe("ok");
 const started=Date.now();
 expect(await probeWorktreeAccessV1(folder,60,()=>new Promise(()=>{}))).toBe("waiting");
 expect(Date.now()-started).toBeLessThan(2000);
});

test.skipIf(process.platform==="win32"||process.getuid?.()===0)("a worktree without permission is denied by the operating system itself",async()=>{
 const folder=temp("locked");
 chmodSync(folder,0o000);
 try{expect(await probeWorktreeAccessV1(folder,500)).toBe("denied");}finally{chmodSync(folder,0o755);}
});

test("the record round-trips through the store, validates against the schema and the Python oracle, and is listed and cleared",async()=>{
 const root=store(),compiled=compileJsonSchemaSubset({$ref:"#/definitions/Record"},schema);
 expect(await readAccessV1(root,"g1-m01")).toBeUndefined();
 for(const state of ["denied","waiting-for-consent"] as const){
  await writeAccessV1(root,record(state));
  expect(await readAccessV1(root,"g1-m01")).toEqual(record(state));
  expect(compiled.validate(record(state))).toEqual([]);
  if(python){
   const check=Bun.spawnSync([python,"-I",oracle,"record",join(root,"v1","origins","g1-m01","access.json")],{stdout:"pipe",stderr:"pipe"});
   expect(check.stdout.toString()+check.stderr.toString()).toBe("");
  }
 }
 expect(await listAccessV1(root)).toEqual([record("waiting-for-consent")]);
 expect(compiled.validate({...record(),state:"gone"})).not.toEqual([]);
 expect(compiled.validate({...record(),extra:1})).not.toEqual([]);
 writeFileSync(join(root,"v1","origins","g1-m01","access.json"),"{broken");
 expect(await readAccessV1(root,"g1-m01")).toBeUndefined();
 await clearAccessV1(root,"g1-m01");
 expect(existsSync(join(root,"v1","origins","g1-m01","access.json"))).toBe(false);
 if(python){
  const file=join(root,"bad.json");writeFileSync(file,JSON.stringify({...record(),state:"gone"}));
  expect(Bun.spawnSync([python,"-I",oracle,"record",file],{stdout:"pipe",stderr:"pipe"}).exitCode).toBe(1);
 }
});

test("the daemon waits while the worktree is refused, reports each state once, writes the record and continues, removing it, when the worktree opens",async()=>{
 const root=store(),folder=temp("tree"),reports:string[]=[];
 let state:"denied"|"waiting"|"ok"="denied";
 const list=():Promise<unknown>=>state==="denied"?refusal("EPERM")():state==="waiting"?new Promise(()=>{}):Promise.resolve([]);
 const finished=awaitWorktreeAccessV1({worktree:folder,storeRoot:root,originId:"g1-m01",platform:"darwin",probeMilliseconds:40,pollMilliseconds:20,list},new AbortController().signal,(reached,error)=>{reports.push(`${reached} ${error}`);});
 await Bun.sleep(120);
 expect((await readAccessV1(root,"g1-m01"))?.state).toBe("denied");
 state="waiting";await Bun.sleep(200);
 expect((await readAccessV1(root,"g1-m01"))?.state).toBe("waiting-for-consent");
 state="ok";await finished;
 expect(await readAccessV1(root,"g1-m01")).toBeUndefined();
 expect(reports).toEqual(["denied EPERM","waiting-for-consent no answer"]);
});

test("a worktree that is readable from the start never writes a record, and an abort ends the wait",async()=>{
 const root=store(),folder=temp("tree");
 await awaitWorktreeAccessV1({worktree:folder,storeRoot:root,originId:"g1-m01"},new AbortController().signal,()=>{throw Error("reported");});
 expect(await readAccessV1(root,"g1-m01")).toBeUndefined();
 const controller=new AbortController(),waiting=awaitWorktreeAccessV1({worktree:folder,storeRoot:root,originId:"g1-m01",probeMilliseconds:20,pollMilliseconds:20,list:refusal("EPERM")},controller.signal,()=>{});
 await Bun.sleep(60);controller.abort();
 await expect(waiting).rejects.toThrow();
});
