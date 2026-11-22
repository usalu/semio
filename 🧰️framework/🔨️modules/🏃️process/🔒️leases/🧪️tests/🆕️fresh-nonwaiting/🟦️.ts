import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {acquireFreshResourceLeaseV1,ResourceLeaseTimeoutErrorV1} from "../../🟦️.ts";

/** 🆕️ Contract: a fresh claim never waits for a holder. With the resource held exclusively it ends within a bound with the typed `ResourceLeaseTimeoutErrorV1` naming the resource, so the caller reports or retries instead of hanging (the `native-storage:<identity>` claim of the staged-input verifier relies on it). */
const root=mkdtempSync(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"fresh-"));
afterAll(()=>rmSync(root,{recursive:true,force:true}));

test("a fresh exclusive claim on a held resource fails fast with the typed refusal",async()=>{
 const signal=new AbortController().signal,options={directory:root,resource:"native-storage:identity",mode:"exclusive" as const,signal,control:null};
 const held=await acquireFreshResourceLeaseV1(options as never),started=Date.now();
 const error=await acquireFreshResourceLeaseV1(options as never).catch(failure=>failure);
 held.release();
 expect(error).toBeInstanceOf(ResourceLeaseTimeoutErrorV1);
 expect(String((error as Error).message)).toContain("native-storage:identity");
 expect(Date.now()-started).toBeLessThan(5000);
 (await acquireFreshResourceLeaseV1(options as never)).release();
});
