import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {resolve,join} from "node:path";
import {sha256} from "@noble/hashes/sha2.js";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import schema from "../../🔁️churn/🧬️schema/🔣️.json";
import {CurrentPhysicalOwnerV1} from "../../📁️current/🟦️.ts";
import {ConcurrentEditErrorV1,retryOnConcurrentEditV1,churnProtocolV1,churnReasonTextV1} from "../../🔁️churn/🟦️.ts";
const output=process.env.SEMIO_TEST_ARTIFACT_DIR;
if(!output)throw Error("Owned test output required");
mkdirSync(output,{recursive:true});
const directory=mkdtempSync(join(output,"comparison-"));
afterAll(()=>rmSync(directory,{recursive:true,force:true}));
const control=()=>({maxBytes:65536,maxWork:1000,chunkBytes:1024,cancelled:()=>false,remainingMs:()=>60000,onProgress:()=>{}});
const ajv=new Ajv({strict:true});ajv.addSchema(schema);
const validate=(value:unknown)=>ajv.validate({$ref:schema.$id+"#/definitions/PhysicalFileComparison"},value);
for(const row of fixture.cases)test("physical comparison: "+row.id,async()=>{
 const path=resolve(directory,row.id),owner=new CurrentPhysicalOwnerV1(directory,control());
 writeFileSync(path,row.before);await owner.digest(path);
 for(const key of ["before","after"] as const){const text=row[key],expected=row[key==="before"?"beforeOperand":"afterOperand"];if(text===null)continue;const bytes=new TextEncoder().encode(text),hash=Buffer.from(sha256(bytes)).toString("hex");expect(hash).toBe(Buffer.from(await crypto.subtle.digest("SHA-256",bytes)).toString("hex"));expect({byteLength:bytes.length,sha256:hash}).toEqual(expected);}
 if(row.after===null)rmSync(path);else writeFileSync(path,row.after);
 if(row.reason===null){await owner.recheck();expect(owner.custody.observedReads().files.get(path)?.reads).toBe(2);return;}
 let failure:any;try{await owner.recheck();}catch(error){failure=error;}
 expect(failure).toBeInstanceOf(ConcurrentEditErrorV1);expect(failure.reason).toBe(row.reason);expect(failure.path).toBe(path);expect(failure.rank).toBe(churnProtocolV1().profiles.snapshot.rank-1);
 const expected={kind:"physical-file-comparison",expected:row.beforeOperand,observed:row.afterOperand};expect(failure.comparison).toEqual(expected);expect(validate(failure.comparison)).toBe(true);
 expect(validate({...expected,source:row.before})).toBe(false);expect(owner.custody.observedReads().files.get(path)?.reads).toBe(row.after===null?1:2);
 for(const language of ["en","de"]){
 const events:any[]=[],sleeps:number[]=[];let calls=0;
 await expect(retryOnConcurrentEditV1("snapshot",async()=>{calls++;throw failure;},{environment:{LANG:language+"_XX.UTF-8"},modifiedAgeMs:()=>undefined,sleep:async ms=>{sleeps.push(ms);},onRetry:event=>events.push(event)})).rejects.toMatchObject({reason:row.reason,comparison:expected});
 expect(calls).toBe(churnProtocolV1().profiles.snapshot.maximumAttempts);expect(sleeps).toEqual(churnProtocolV1().profiles.snapshot.backoffMs);expect(events.every(event=>event.text.includes(JSON.stringify(expected))&&event.text.includes(churnReasonTextV1(language,failure.reason)))).toBe(true);
 }
 expect(owner.custody.observedReads().files.get(path)?.reads).toBe(row.after===null?1:2);
});
