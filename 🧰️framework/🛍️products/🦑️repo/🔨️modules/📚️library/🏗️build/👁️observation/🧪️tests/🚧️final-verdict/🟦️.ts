/** 🚧️ Contract: while a farm child is the only verifier, a verdict that is not `holds` is final. A `stale`, `other` or `missing` verdict of the child ends the look at the stored record without settling or verifying a single row in the reader (the parent of a read), and `prepareAttempts` asks the farm to prepare the record again before the reader would compute it itself. Without a verdict (no farm any more) the reader verifies as before.
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";

const observation=await import("../../🟨️.mjs") as any;
const calls={settle:0,verify:0};
observation.registerClosureProviderV1({answer:()=>"true",settle:async()=>{calls.settle++;},verify:async()=>{calls.verify++;return true;}});
const root=mkdtempSync(join(realpathSync(tmpdir()),"final-verdict-")),file=join(root,"input.txt");
writeFileSync(file,"one");
await new Promise(done=>setTimeout(done,2200));
const written=async(name:string)=>{
 const cachePath=join(root,"np",name+".json");
 await observation.observeInference({cachePath,key:[name]},async()=>{observation.recordClosureFactV1(JSON.stringify([root,name,["digest"]]),"true");return{text:observation.observedFs.readFileSync(file,"utf8")};});
 return cachePath;
};
const computes=()=>{let count=0;return{count:()=>count,compute:async()=>{count++;return{text:"computed"};}};};

test("a stale, other or missing verdict of the child is final: the reader settles and verifies nothing and computes the owner",async()=>{
 const cachePath=await written("final");
 for(const verdict of["stale","other","missing"]){
  calls.settle=0;calls.verify=0;
  const run=computes();
  const value=await observation.observeInference({cachePath,key:["final"],verdictFinal:true,verdict:async({headerDigest}:{headerDigest:string})=>({verdict,headerDigest})},run.compute);
  expect(value.text,verdict).toBe("computed");expect(run.count()).toBe(1);
  expect(calls,verdict).toEqual({settle:0,verify:0});
 }
});

test("without verdictFinal a stale verdict is verified by the reader as before, and a lost verdict too",async()=>{
 const cachePath=await written("open");
 for(const verdict of[async({headerDigest}:{headerDigest:string})=>({verdict:"stale",headerDigest}),async()=>undefined]){
  calls.settle=0;calls.verify=0;
  const value=await observation.observeInference({cachePath,key:["open"],verdict},async()=>({text:"computed"}));
  expect(value.text).toBe("one");expect(calls.settle).toBeGreaterThan(0);expect(calls.verify).toBeGreaterThan(0);
 }
 calls.settle=0;
 const value=await observation.observeInference({cachePath,key:["open"],verdictFinal:true,verdict:async()=>undefined},async()=>({text:"computed"}));
 expect(value.text,"no verdict at all (no farm): the reader verifies").toBe("one");expect(calls.settle).toBeGreaterThan(0);
});

test("the farm is asked to prepare the record as many times as prepareAttempts says before the reader computes it, and never more than once by default",async()=>{
 for(const [attempts,expected] of[[undefined,1],[3,3]] as const){
  const cachePath=join(root,"np","never-stored-"+expected+".json");let prepared=0;const run=computes();
  const value=await observation.observeInference({cachePath,key:["never"],...attempts===undefined?{}:{prepareAttempts:attempts},prepare:async()=>{prepared++;return true;}},run.compute);
  expect(prepared).toBe(expected);expect(run.count()).toBe(1);expect(value.text).toBe("computed");
 }
 rmSync(root,{recursive:true,force:true});
});
