/** 💾️ Contract: a recorded observation row costs at most the declared bytes of live JavaScript memory on the real record path (every distinct fact of a 4,000 project workspace is one row, 530,000 of them in the graph), the rows come out in the order they were recorded and a repeated fact is one row. */
import{expect,test}from"bun:test";
import{mkdtempSync,realpathSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import Ajv from"ajv";
import fixture from"./🔣️.json";
import fixtureSchema from"./🧬️schema/🔣️.json";
import{captureObservedSourceInputsV1,observeInferenceSnapshotV1,observedFs,releaseInferenceMemoryV1}from"../../🟨️.mjs";

const bun=typeof Bun!=="undefined"?await import("bun:jsc"):undefined;
const live=()=>{Bun.gc(true);Bun.gc(true);const stats=bun!.heapStats();return stats.heapSize+stats.extraMemorySize;};

test("the fixture is closed under its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(fixtureSchema,fixture),JSON.stringify(ajv.errors)).toBe(true);
});

test("a recorded row stays within the declared bytes and repeated facts are one row in recording order",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"row-cost-"))),paths=Array.from({length:fixture.rows},(_,at)=>join(root,"d".repeat(fixture.pathCharacters-String(at).length-root.length-1),String(at)));
 try{
  releaseInferenceMemoryV1();
  const before=bun?live():0;
  const rows=await observeInferenceSnapshotV1(async()=>(await captureObservedSourceInputsV1(async()=>{for(const path of paths)observedFs.existsSync(path);for(const path of paths.slice(0,1000))observedFs.existsSync(path);},undefined,false)).observations);
  expect(rows.length).toBe(fixture.rows);
  expect(rows.map(row=>row.path)).toEqual(paths);
  expect(rows.every(row=>row.kind==="exists"&&row.fact===false)).toBe(true);
  if(bun){const bytes=(live()-before)/rows.length;expect(bytes,`bytes per row ${Math.round(bytes)} (before the row table: ${fixture.reportedBeforeBytesPerRow})`).toBeLessThan(fixture.budgetBytesPerRow);}
 }finally{releaseInferenceMemoryV1();rmSync(root,{recursive:true,force:true});}
},120000);
