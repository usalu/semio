import{test,expect}from"bun:test";
import{mkdtempSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";

const observation=await import("../../../👁️observation/🟨️.mjs");

const file=()=>{const directory=mkdtempSync(join(tmpdir(),"retain-rows-")),path=join(directory,"source.txt");writeFileSync(path,"one");return path;};

test("rows an earlier observation consumed are retained by the capture around the caller",async()=>{
 const path=file(),first=await observation.captureObservedSourceInputsV1(async()=>observation.observedFs.readFileSync(path,"utf8"),undefined,false);
 const rows=first.observations.filter((row:{path:string})=>row.path===path);expect(rows.length).toBeGreaterThan(0);
 const second=await observation.captureObservedSourceInputsV1(async()=>{observation.retainObservedRowsV1(rows);return"reused";},undefined,false);
 expect(second.value).toBe("reused");expect(second.observations.map((row:{path:string})=>row.path)).toContain(path);
});

test("a nested capture hands the retained rows to its parent",async()=>{
 const path=file(),first=await observation.captureObservedSourceInputsV1(async()=>observation.observedFs.readFileSync(path,"utf8"),undefined,false),rows=first.observations.filter((row:{path:string})=>row.path===path);
 const outer=await observation.captureObservedSourceInputsV1(async()=>{await observation.captureObservedSourceInputsV1(async()=>{observation.retainObservedRowsV1(rows);},undefined,false);},undefined,false);
 expect(outer.observations.map((row:{path:string})=>row.path)).toContain(path);
});

test("retaining outside any capture changes nothing",()=>{
 expect(()=>observation.retainObservedRowsV1([])).not.toThrow();
});
