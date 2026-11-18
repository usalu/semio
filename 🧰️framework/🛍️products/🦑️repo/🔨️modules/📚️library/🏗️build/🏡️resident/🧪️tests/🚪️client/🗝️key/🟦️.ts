import {describe,expect,test} from "bun:test";
import {mkdtempSync,writeFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {HostUnavailableErrorV1,PLAN_KEY_WAIT_V1,planKeyRetryV1} from "../../../🚪️client/🛰️plan/🟨️.mjs";

const missing=Object.assign(new HostUnavailableErrorV1("key"),{missing:true});

describe("a plan request and the key of a service between two generations",()=>{
 test("a missing key file is waited for, at most the bound, every other failure is a refusal",()=>{
  expect(planKeyRetryV1(missing,0)).toBe(true);
  expect(planKeyRetryV1(missing,PLAN_KEY_WAIT_V1.maximum-1)).toBe(true);
  expect(planKeyRetryV1(missing,PLAN_KEY_WAIT_V1.maximum)).toBe(false);
  expect(planKeyRetryV1(new HostUnavailableErrorV1("key"),0)).toBe(false);
  expect(planKeyRetryV1(new HostUnavailableErrorV1("lost"),0)).toBe(false);
  expect(planKeyRetryV1(undefined,0)).toBe(false);
 });
 test("the exchange marks only ENOENT as missing (the real reader against a missing and a present key)",async()=>{
  const dir=mkdtempSync(join(tmpdir(),"r71-key-"));
  try{
   const {readFileSync}=await import("node:fs");
   const code=(path:string)=>{try{readFileSync(path);return undefined;}catch(error){return (error as NodeJS.ErrnoException).code;}};
   expect(code(join(dir,"absent.key"))).toBe("ENOENT");
   writeFileSync(join(dir,"present.key"),"k");
   expect(code(join(dir,"present.key"))).toBeUndefined();
   expect(code(dir)).toBe("EISDIR");
   expect(planKeyRetryV1(Object.assign(new HostUnavailableErrorV1("key"),{missing:code(dir)==="ENOENT"}),0)).toBe(false);
  }finally{rmSync(dir,{recursive:true,force:true});}
 });
 test("the plan client source marks ENOENT and retries through the helper (pin)",async()=>{
  const text=await Bun.file(new URL("../../../🚪️client/🛰️plan/🟨️.mjs",import.meta.url)).text();
  expect(text).toContain('missing:error?.code==="ENOENT"');
  expect(text).toContain("planKeyRetryV1(error,keyWaits)");
 });
});
