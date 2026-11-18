/** 🧭️ Contract: when a farm child cannot give the verdict (its job failed three times) the reader does not verify the record itself: with `verdictFinal` the refusal reaches the owner (which is set aside); without it a lost verdict still lets the reader verify, as before.
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";

const observation=await import("../../🟨️.mjs") as any;
const calls={settle:0};
observation.registerClosureProviderV1({answer:()=>"true",settle:async()=>{calls.settle++;},verify:async()=>true});
const root=mkdtempSync(join(realpathSync(tmpdir()),"verdict-failure-")),file=join(root,"input.txt");
writeFileSync(file,"one");
await new Promise(done=>setTimeout(done,2200));
const cachePath=join(root,"np","o.json");
await observation.observeInference({cachePath,key:["o"]},async()=>{observation.recordClosureFactV1(JSON.stringify([root,"o",["d"]]),"true");return{text:observation.observedFs.readFileSync(file,"utf8")};});

test("a failing verdict refuses the owner under verdictFinal and never makes the reader settle or verify",async()=>{
 calls.settle=0;
 await expect(observation.observeInference({cachePath,key:["o"],verdictFinal:true,verdict:async()=>{throw Object.assign(new Error("farm children could not infer"),{code:"graph.farm-job-failed"});}},async()=>({text:"computed"}))).rejects.toMatchObject({code:"graph.farm-job-failed"});
 expect(calls.settle).toBe(0);
});

test("without verdictFinal a failing verdict still lets the reader verify",async()=>{
 const value=await observation.observeInference({cachePath,key:["o"],verdict:async()=>{throw new Error("child lost");}},async()=>({text:"computed"}));
 expect(value.text).toBe("one");expect(calls.settle).toBeGreaterThan(0);
 rmSync(root,{recursive:true,force:true});
});
