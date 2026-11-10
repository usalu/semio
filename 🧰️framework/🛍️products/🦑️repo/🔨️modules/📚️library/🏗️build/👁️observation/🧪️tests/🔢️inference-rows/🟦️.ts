/** 🔢️ Contract: the rows a running project inference has observed are countable by its record path while it runs and forgotten when it ends, so the owner stage can show them next to every running owner. */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{fileURLToPath}from"node:url";
import{forgetObservedInferenceRowsV1,observedInferenceRowsV1}from"../../🟨️.mjs";

test("an unknown record path has no rows and forgetting it is harmless",()=>{
 expect(observedInferenceRowsV1("/nowhere/np/none.json")).toBe(0);
 expect(()=>forgetObservedInferenceRowsV1("/nowhere/np/none.json")).not.toThrow();
});

test("the recording of an inference registers the size of its observation map under its record path",()=>{
 const text=readFileSync(fileURLToPath(new URL("../../🟨️.mjs",import.meta.url)),"utf8");
 expect(text).toContain("inferenceRowCountsV1.set(cachePath,()=>observations.size);");
 expect(text.indexOf("const observations = new Map(), started = Date.now();")).toBeLessThan(text.indexOf("inferenceRowCountsV1.set(cachePath,()=>observations.size);"));
});
