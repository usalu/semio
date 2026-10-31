import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {defer,firstValueFrom,of,retry} from "rxjs";
import {existsSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {releaseHostLockV1,takeHostLockV1} from "../../🎫️gate/🟦️.ts";
import {readHostCapacityPolicyV1} from "../../🟦️.ts";
import decision from "../🧫️fixtures/🔣️.json";
import fixture from "./🔣️.json";
import schema from "./🧬️.json";

const roots:string[]=[],policy=readHostCapacityPolicyV1(decision.policy),pauses=[10,40,160,640,2560];
afterAll(()=>{for(const p of roots)rmSync(p,{recursive:true,force:true});});
test("Portable release cases agree with independent JSON Schema validation",()=>expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true));
for(const row of fixture.cases)test(row.id+": bounded sharing retries preserve owner fencing and match RxJS",async()=>{
 const directory=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_ROOT??tmpdir(),"hclr-"));roots.push(directory);expect(takeHostLockV1(directory,Date.now(),policy,process.pid)).toBe(true);const lock=join(directory,"lock"),owner=join(lock,"owner");let attempts=0,failed=0,error:string|null=null;const sleeps:number[]=[];
 const remove=(path:string):void=>{attempts++;const code=row.errors[failed++];if(code){if(row.replace)writeFileSync(owner,"1:9999999999999");throw Object.assign(Error(code),{code,path,syscall:"rmdir"});}rmSync(path,{recursive:true,force:true});};
 try{(releaseHostLockV1 as unknown as (directory:string,pid:number,ports:unknown)=>void)(directory,process.pid,{remove,retry:{onRetry:(event:{delayMs:number})=>sleeps.push(event.delayMs),sleepSync:()=>{}}});}catch(e){const x=e as {reason?:string;code?:string};error=x.reason==="busy"?"EBUSY":x.code??"unknown";}
 const actual={attempts,sleeps,exists:existsSync(lock),error};expect(actual).toEqual(row.expect);if(row.replace)expect(readFileSync(owner,"utf8")).toBe("1:9999999999999");
 let count=0,current=true,present=true,oracleError:string|null=null;const waited:number[]=[];
 try{await firstValueFrom(defer(()=>{if(!current)return of(0);const code=row.errors[count++];if(code){if(row.replace)current=false;throw Object.assign(Error(code),{code});}present=false;return of(0);}).pipe(retry({count:5,delay:(e,index)=>{if(e.code!=="EBUSY")throw e;waited.push(pauses[index-1]!);return of(0);}})));}catch(e){oracleError=(e as {code:string}).code;}
 expect(actual).toEqual({attempts:count,sleeps:waited,exists:present,error:oracleError});
});
