/** 🔎️ Law: the stale verdict names the exact refused row when the original verification identifies one.
 * @see ../../🟨️.mjs `verifyStoredProjectRecordV1` `firstFailingRowV1` */
import{expect,test}from"bun:test";
import{mkdtempSync,realpathSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";

const observation=await import("../../🟨️.mjs");
const refused=new Set<string>();
const verified:number[]=[];
observation.registerClosureProviderV1({answer:()=>"true",settle:async()=>{},verify:async(rows,_value,failed)=>{verified.push(rows.length);const row=rows.find(row=>refused.has(row.path));if(row)failed?.(row,"closure-value");return row===undefined;}});
const root=mkdtempSync(join(realpathSync(tmpdir()),"stale-row-")),file=join(root,"input.txt");
writeFileSync(file,"one");
await new Promise(done=>setTimeout(done,2200));
const written=async(name:string,paths:string[])=>{
 const cachePath=join(root,"np",name+".json");
 await observation.observeInference({cachePath,key:[name]},async()=>{for(const path of paths)observation.recordClosureFactV1(path,"true");return{text:observation.observedFs.readFileSync(file,"utf8")};});
 return cachePath;
};

test("a record that holds names no row, an edited input is named by kind and path",async()=>{
 const cachePath=await written("edited",[]);
 const first=await observation.verifyStoredProjectRecordV1({cachePath,key:["edited"]});
 expect(first.verdict).toBe("holds");expect(first.staleRow).toBeUndefined();
 writeFileSync(file,"two, longer");
 const second=await observation.verifyStoredProjectRecordV1({cachePath,key:["edited"]});
 expect(second.verdict).toBe("stale");
 expect(second.staleRow.kind).toBe("file");expect(second.staleRow.path).toContain("input.txt");
 expect(second.staleRow.path.length).toBeLessThanOrEqual(203);expect(second.staleRow.fact.length).toBeLessThanOrEqual(163);
 writeFileSync(file,"one");
});

test("a closure row that the provider refuses is the named row of a stale record",async()=>{
 await new Promise(done=>setTimeout(done,2200));
 const path=JSON.stringify([root,"closure-a",["generated"]]);
 const cachePath=await written("closure",[path,JSON.stringify([root,"closure-b",["digest"]])]);
 expect((await observation.verifyStoredProjectRecordV1({cachePath,key:["closure"]})).verdict).toBe("holds");
 verified.length=0;
 refused.add(path);
 const said=await observation.verifyStoredProjectRecordV1({cachePath,key:["closure"]});
 refused.clear();
 expect(said.verdict).toBe("stale");expect(said.staleRow.kind).toBe("closure");expect(said.staleRow.path).toContain("closure-a");
 expect(said.staleRow.why).toMatch(/^closure-value; judge /u);expect(verified).toEqual([2]);
});
