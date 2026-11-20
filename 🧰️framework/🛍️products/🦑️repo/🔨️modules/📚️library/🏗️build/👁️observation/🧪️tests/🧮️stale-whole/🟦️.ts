/** 🧮️ Law: an aggregate refusal without an identified row reports the failed group and the original judgment.
 * @see ../../🟨️.mjs `staleWholeRowV1` `verifyStoredProjectRecordV1` */
import{expect,test}from"bun:test";
import{mkdtempSync,realpathSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";

const observation=await import("../../🟨️.mjs") as any;
let together=true;
const verified:number[]=[];
observation.registerClosureProviderV1({answer:()=>"true",settle:async()=>{},verify:async(rows:unknown[])=>{verified.push(rows.length);return rows.length<2||together;}});
const root=mkdtempSync(join(realpathSync(tmpdir()),"stale-whole-")),file=join(root,"input.txt");
writeFileSync(file,"one");
await new Promise(done=>setTimeout(done,2200));
const written=async(name:string)=>{
 const cachePath=join(root,"np",name+".json");
 await observation.observeInference({cachePath,key:[name]},async()=>{for(const path of[JSON.stringify([root,"a",["digest"]]),JSON.stringify([root,"b",["digest"]])])observation.recordClosureFactV1(path,"true");return{text:observation.observedFs.readFileSync(file,"utf8")};});
 return cachePath;
};

test("a record that holds names nothing",async()=>{
 const said=await observation.verifyStoredProjectRecordV1({cachePath:await written("holds"),key:["holds"]});
 expect(said.verdict).toBe("holds");expect(said.staleRow).toBeUndefined();
});

test("a record whose closure rows fail only together is named by a whole row with the judgment it came from",async()=>{
 const cachePath=await written("whole");
 verified.length=0;
 together=false;
 const first=await observation.verifyStoredProjectRecordV1({cachePath,key:["whole"]}),second=await observation.verifyStoredProjectRecordV1({cachePath,key:["whole"]});
 together=true;
 expect(first.verdict).toBe("stale");
 expect(first.staleRow.kind).toBe("whole");
 expect(JSON.parse(first.staleRow.fact)).toMatchObject({closure:2});
 expect(first.staleRow.why).toContain("consistent true, other rows true, 2 closure rows together false");
 expect(verified).toEqual([2,2]);
 const judge=(row:{why:string})=>Number(/judge (\d+) of pid /u.exec(row.why)![1]);
 expect(judge(second.staleRow)).toBe(judge(first.staleRow)+1);expect(first.staleRow.why).toContain("of pid "+process.pid);
});

test("a row that fails alone keeps its own name and carries the judgment too",async()=>{
 const cachePath=await written("edited");
 writeFileSync(file,"two, longer");
 const said=await observation.verifyStoredProjectRecordV1({cachePath,key:["edited"]});
 writeFileSync(file,"one");
 expect(said.verdict).toBe("stale");expect(said.staleRow.kind).toBe("file");expect(said.staleRow.why).toMatch(/^judge \d+ of pid \d+$/u);
});
