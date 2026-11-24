/** 🚫️ Law: a shipped closure entry that is refused at the intake leaves the first own row that did not hold, so a take that later finds no entry names that row (`closure-entry-absent … refused at the intake, moved <kind> <path>`) instead of the unnamed "no moved row found"; an entry that was never shipped says so; the memory of refusals is bounded and an entry that is stored later is forgotten.
 * @see ../../🟨️.mjs `rememberImportRefusalV1` `importRefusalsV1` `closureFailureWhyV1` */
import{expect,test}from"bun:test";
import{mkdtempSync,realpathSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";

const{rememberImportRefusalV1,importRefusalsV1,closureFailureWhyV1}=await import("../../🟨️.mjs");
const observation=await import("../../../../../👁️observation/🟨️.mjs");
const root=mkdtempSync(join(realpathSync(tmpdir()),"import-refused-")),files=["a","b"].map(name=>join(root,name));
files.forEach((file,at)=>writeFileSync(file,"content "+at));
await new Promise(done=>setTimeout(done,2200));
const rowsOf=async(paths:string[])=>(await observation.observeDetachedV1(async()=>{for(const path of paths)observation.observedFs.readFileSync(path,"utf8");})).rows;

test("the first own row that does not hold is remembered, a bundle whose rows all hold leaves null, and the miss text names the row or says the entry never came",async()=>{
 const rows=await rowsOf(files);
 rememberImportRefusalV1("held",{rows});
 expect(importRefusalsV1.get("held")).toBeNull();
 writeFileSync(files[1]!,"moved own file, longer still");
 rememberImportRefusalV1("moved",{rows});
 expect(importRefusalsV1.get("moved")?.path).toBe(files[1]);
 const miss=(movedRow:unknown)=>Object.assign(Error("miss"),{code:"BUILD_TAKE_MISS",absent:true,movedRow});
 expect(closureFailureWhyV1(miss(importRefusalsV1.get("moved")),"abcdef0123456789")).toBe("closure-entry-absent (recipe abcdef012345, refused at the intake, moved "+importRefusalsV1.get("moved")!.kind+" "+files[1]+")");
 expect(closureFailureWhyV1(miss(undefined),"abcdef0123456789")).toBe("closure-entry-absent (recipe abcdef012345, never shipped or taken here)");
 expect(closureFailureWhyV1(Object.assign(Error("miss"),{code:"BUILD_TAKE_MISS",absent:false}),"abcdef0123456789")).toBe("closure-entry-stale (recipe abcdef012345, no moved row found)");
});

test("the memory of refusals is bounded and the oldest refusal goes first",()=>{
 importRefusalsV1.clear();
 for(let at=0;at<4200;at++)rememberImportRefusalV1("id"+at,{rows:[]});
 expect(importRefusalsV1.stats().entries).toBe(4096);
 expect(importRefusalsV1.has("id0")).toBe(false);
 expect(importRefusalsV1.has("id4199")).toBe(true);
});
