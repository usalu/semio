/** ⏱️ Contract: every lookup of a stored project record is counted and its time is split into the steps of the lookup, so a slow take shows which step it waits in. @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";

const observation=await import("../../🟨️.mjs") as any;

test("a found record is counted and its read and row probes are timed",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"lookup-clock-")),file=join(root,"input.txt"),cachePath=join(root,"np","record.json");
 try{
  writeFileSync(file,"one");await new Promise(done=>setTimeout(done,2200));
  const compute=async()=>({text:observation.observedFs.readFileSync(file,"utf8")}),before=observation.observationLookupClockV1();
  await observation.observeInference({cachePath,key:["clock"]},compute);
  const middle=observation.observationLookupClockV1();
  await observation.observeInference({cachePath,key:["clock"]},compute);
  const after=observation.observationLookupClockV1();
  expect(middle.count-before.count).toBe(1);expect(after.count-middle.count).toBe(1);
  for(const key of["read","settle","hold","reusable","offer","gate"])expect(after[key]).toBeGreaterThanOrEqual(middle[key]);
  expect(after.read+after.hold).toBeGreaterThan(before.read+before.hold-1);
 }finally{rmSync(root,{recursive:true,force:true});}
});
