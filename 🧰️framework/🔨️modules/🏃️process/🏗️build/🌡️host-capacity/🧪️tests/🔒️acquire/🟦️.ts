import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {defer,firstValueFrom,of,retry} from "rxjs";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {releaseHostLockV1,takeHostLockV1} from "../../🎫️gate/🟦️.ts";
import {readHostCapacityPolicyV1} from "../../🟦️.ts";
import decision from "../🧫️fixtures/🔣️.json";
import fixture from "./🔣️.json";
import schema from "./🧬️.json";

const roots:string[]=[],policy=readHostCapacityPolicyV1(decision.policy),pauses=[10,40,160,640,2560];
const fresh=()=>{const p=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_ROOT??tmpdir(),"hcla-"));roots.push(p);return p;};
afterAll(()=>{for(const p of roots)rmSync(p,{recursive:true,force:true});});
test("Portable acquisition cases are independently schema valid",()=>expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true));
for(const row of fixture.cases)test(row.id+": bounded lock acquisition agrees with independent RxJS",async()=>{
 const directory=fresh(),lock=join(directory,"lock"),peer=process.pid+":"+(Date.now()+100000);let attempts=0,acquired=false,error:string|null=null;const sleeps:number[]=[];
 const create=(path:string)=>{const code=row.errors[attempts++];if(code){if(row.replace&&code==="EPERM"){mkdirSync(path);writeFileSync(join(path,"owner"),peer);}throw Object.assign(Error(code),{code,path,syscall:"mkdir"});}mkdirSync(path,{mode:0o700});};
 try{acquired=(takeHostLockV1 as unknown as (...args:any[])=>boolean)(directory,Date.now(),policy,process.pid,undefined,{create,retry:{onRetry:(e:{delayMs:number})=>sleeps.push(e.delayMs),sleepSync:()=>{}}});}catch(e){const x=e as {reason?:string;code?:string};error=x.reason??x.code??"unknown";}
 const actual={attempts,sleeps,acquired,error};expect(actual).toEqual(row.expect);if(row.replace)expect(readFileSync(join(lock,"owner"),"utf8")).toBe(peer);if(acquired)releaseHostLockV1(directory,process.pid);
 let count=0,held=false,fault:string|null=null;const waits:number[]=[];
 try{held=await firstValueFrom(defer(()=>{const code=row.errors[count++];if(code==="EEXIST")return of(false);if(code)throw Object.assign(Error(code),{code});return of(true);}).pipe(retry({count:5,delay:(e,index)=>{if(!["EPERM","EBUSY"].includes(e.code))throw e;waits.push(pauses[index-1]!);return of(0);}})));}catch(e){const code=(e as {code:string}).code;fault=["EPERM","EBUSY"].includes(code)?"busy":code;}
 expect(actual).toEqual({attempts:count,sleeps:waits,acquired:held,error:fault});
});
test("An owner written by a peer between directory creation and publication is never overwritten",()=>{
 const directory=fresh(),peer="1:9999999999999";const acquired=takeHostLockV1(directory,Date.now(),policy,process.pid,()=>writeFileSync(join(directory,"lock","owner"),peer));
 expect(acquired).toBe(false);expect(readFileSync(join(directory,"lock","owner"),"utf8")).toBe(peer);
});
