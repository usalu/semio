/** 🤝️ Contract: exactly one process verifies each stored project record. A farm child verifies it (`verifyStoredProjectRecordV1`, a plain read, no custody) and answers a verdict that names the record by the digest of its header line, which covers every observed row; the reader that asked adopts the record on a `holds` verdict for that very header and settles, verifies and probes nothing itself, while a verdict for another record, a stale one or none leaves it to verify as before, and an edit of an input is seen by the next verdict. @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{mkdtempSync,realpathSync,rmSync,unlinkSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";

const observation=await import("../../🟨️.mjs") as any;
const calls={settle:0,verify:0,rows:0};
observation.registerClosureProviderV1({answer:()=>"true",settle:async()=>{calls.settle++;},verify:async(rows:unknown[])=>{calls.verify++;calls.rows+=rows.length;return true;}});
const root=mkdtempSync(join(realpathSync(tmpdir()),"adopted-verdict-")),file=join(root,"input.txt");
writeFileSync(file,"one");
await new Promise(done=>setTimeout(done,2200));
const written=async(name:string,size:number,source=file)=>{
 const cachePath=join(root,"np",name+".json"),paths=Array.from({length:size},(_all,row)=>JSON.stringify([root,name+"-"+row,["digest"]]));
 await observation.observeInference({cachePath,key:[name]},async()=>{for(const path of paths)observation.recordClosureFactV1(path,"true");return{text:observation.observedFs.readFileSync(source,"utf8")};});
 return cachePath;
};
const mustNotCompute=async()=>{throw new Error("computed");};
const reset=()=>{calls.settle=0;calls.verify=0;calls.rows=0;};

test("a holds verdict for the asked record is adopted: the reader settles, verifies and probes nothing",async()=>{
 const cachePath=await written("adopt",50),other=join(root,"other.txt");
 writeFileSync(other,"x");
 reset();
 const value=await observation.observeInference({cachePath,key:["adopt"],verdict:async({headerDigest}:{headerDigest:string})=>({verdict:"holds",headerDigest})},mustNotCompute);
 expect(value.text).toBe("one");
 expect(calls).toEqual({settle:0,verify:0,rows:0});
});

test("a verdict for another record, a stale one or a failing one is not adopted",async()=>{
 const cachePath=await written("refuse",5);
 for(const verdict of[async()=>({verdict:"holds",headerDigest:"0".repeat(64)}),async({headerDigest}:{headerDigest:string})=>({verdict:"stale",headerDigest}),async()=>{throw new Error("child lost");},async()=>undefined]){
  reset();
  const value=await observation.observeInference({cachePath,key:["refuse"],verdict},mustNotCompute);
  expect(value.text).toBe("one");
  expect(calls.settle).toBeGreaterThan(0);
  expect(calls.verify).toBeGreaterThan(0);
 }
});

test("the verifier of a child says holds, and an edit of an input is seen by the next verdict",async()=>{
 const input=join(root,"edited.txt");
 writeFileSync(input,"before");
 await new Promise(done=>setTimeout(done,2200));
 const cachePath=await written("edited",3,input);
 const first=await observation.verifyStoredProjectRecordV1({cachePath,key:["edited"]});
 expect(first.verdict).toBe("holds");
 expect(first.headerDigest).toMatch(/^[0-9a-f]{64}$/u);
 expect((await observation.verifyStoredProjectRecordV1({cachePath,key:["edited"]})).headerDigest).toBe(first.headerDigest);
 expect((await observation.verifyStoredProjectRecordV1({cachePath,key:["older"]})).verdict).toBe("other");
 expect((await observation.verifyStoredProjectRecordV1({cachePath:join(root,"np","none.json"),key:["none"]})).verdict).toBe("missing");
 writeFileSync(input,"after, longer");
 const second=await observation.verifyStoredProjectRecordV1({cachePath,key:["edited"]});
 expect(second.verdict).toBe("stale");
 let computed=0;
 const value=await observation.observeInference({cachePath,key:["edited"],verdict:()=>observation.verifyStoredProjectRecordV1({cachePath,key:["edited"]}).then((said:any)=>({...said,headerDigest:first.headerDigest}))},async()=>{computed++;return{text:observation.observedFs.readFileSync(input,"utf8")};});
 expect(computed).toBe(1);
 expect(value.text).toBe("after, longer");
});

/** 🤝️ Law: what the reader does for an adopted record does not grow with the closure it names: no settle, no verify, no row, and no file is probed (the input is gone and the record is still adopted). */
test("the work of the reader for adopted records is zero for 1, 10, 100 and 1000 closure paths",async()=>{
 const gone=join(root,"gone.txt");
 writeFileSync(gone,"g");
 await new Promise(done=>setTimeout(done,2200));
 const owners=[] as string[];
 for(const size of[1,10,100,1000])owners.push(await written("law-"+size,size,gone));
 unlinkSync(gone);
 for(const [at,cachePath] of owners.entries()){
  reset();
  const value=await observation.observeInference({cachePath,key:["law-"+[1,10,100,1000][at]],verdict:async({headerDigest}:{headerDigest:string})=>({verdict:"holds",headerDigest})},mustNotCompute);
  expect(value.text).toBe("g");
  expect(calls).toEqual({settle:0,verify:0,rows:0});
 }
 rmSync(root,{recursive:true,force:true});
});
