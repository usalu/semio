/** 🎯️ Law: an owner record is stale only when a file its inference read moved, and the verdict names that file with its recorded and current content digest; an edit of a file outside the read set of an owner leaves its record valid. The Python oracle (hashlib) predicts the stale owners and the digests independently of the implementation.
 * @see ./🐍️oracle.py
 * @see ../../🟨️.mjs `observeProjectInferenceV1` `projectRowsHoldV1` */
import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{mkdirSync,mkdtempSync,realpathSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";

const observation=await import("../../🟨️.mjs");
const refusedFiles=new Set<string>();
observation.registerClosureProviderV1({answer:()=>"true",settle:async()=>{},verify:async(rows,_value,failed)=>{const row=rows.find((row:any)=>refusedFiles.has(row.path));if(row)failed?.({kind:"file",path:row.path,fact:{sha256:"0".repeat(64)}},"closure-entry-stale (recipe test)");return row===undefined;}});
const root=mkdtempSync(join(realpathSync(tmpdir()),"read-set-"));
mkdirSync(join(root,"np"));
const names=["f0","f1","f2","f3","f4","f5"],before:Record<string,string>=Object.fromEntries(names.map(name=>[name,"content of "+name]));
for(const name of names)writeFileSync(join(root,name),before[name]);
const reads:Record<string,string[]>={A:["f0","f1"],B:["f2","f3"],C:["f1","f4"]};
const oracle=(edits:Record<string,string>)=>{const run=spawnSync("python3",["-I",join(import.meta.dir,"🐍️oracle.py")],{input:JSON.stringify({files:before,reads,edits}),encoding:"utf8"});if(run.status!==0)throw new Error(run.stderr);return JSON.parse(run.stdout);};
const owners=(told:Record<string,any[]>,computed:string[])=>Object.fromEntries(Object.keys(reads).map(name=>[name,()=>observation.observeProjectInferenceV1({cachePath:join(root,"np",name+".json"),key:[name],owner:name,stale:(row:any)=>(told[name]??=[]).push(row)},async()=>{computed.push(name);for(const file of reads[name])observation.observedFs.readFileSync(join(root,file),"utf8");return{name};})]));

test("an edit outside every read set leaves every record valid, an edit of a shared read file stales exactly its readers and names the file with both digests",async()=>{
 await new Promise(done=>setTimeout(done,2200));
 const told:Record<string,any[]>={},computed:string[]=[],run=owners(told,computed);
 for(const name of Object.keys(run))await run[name]();
 expect(computed.sort()).toEqual(["A","B","C"]);
 await new Promise(done=>setTimeout(done,2200));
 computed.length=0;
 const edits={f5:"unread edit"};
 writeFileSync(join(root,"f5"),edits.f5);
 for(const name of Object.keys(run))await run[name]();
 expect(oracle(edits).stale).toEqual({});expect(computed).toEqual([]);expect(told).toEqual({});
 const moved={f1:"f1 moved, longer"};
 writeFileSync(join(root,"f1"),moved.f1);
 const expected=oracle(moved);
 for(const name of Object.keys(run))await run[name]();
 expect(computed.sort()).toEqual(Object.keys(expected.stale).sort());
 expect(Object.keys(told).sort()).toEqual(Object.keys(expected.stale).sort());
 for(const[owner,said]of Object.entries(expected.stale as Record<string,any>)){
  expect(told[owner].length).toBe(1);
  expect([told[owner][0].kind,told[owner][0].recorded,told[owner][0].current]).toEqual(["file",said.recorded,said.current]);
  expect(told[owner][0].path).toContain(said.file);
 }
 expect(expected.holds).toEqual(["B"]);
},30000);

test("a closure row whose entry moved is named by the moved file the provider reports, not by the closure observations of the whole record",async()=>{
 await new Promise(done=>setTimeout(done,2200));
 const closurePath=JSON.stringify([root,"entry-a",["digest"]]),cachePath=join(root,"np","closure.json");
 await observation.observeInference({cachePath,key:["closure"]},async()=>{observation.recordClosureFactV1(closurePath,"true");return{ok:true};});
 expect((await observation.verifyStoredProjectRecordV1({cachePath,key:["closure"]})).verdict).toBe("holds");
 refusedFiles.add(closurePath);
 const said=await observation.verifyStoredProjectRecordV1({cachePath,key:["closure"]});
 refusedFiles.clear();
 expect(said.verdict).toBe("stale");
 expect(said.staleRow?.kind).toBe("file");expect(said.staleRow?.path).toContain("entry-a");
 expect(said.staleRow?.why).toMatch(/^closure-entry-stale \(recipe test\); judge /u);
});
