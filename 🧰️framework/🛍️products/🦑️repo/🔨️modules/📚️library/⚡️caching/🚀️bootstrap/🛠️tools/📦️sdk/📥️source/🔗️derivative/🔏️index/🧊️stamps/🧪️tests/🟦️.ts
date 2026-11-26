import Ajv from"ajv";
import{afterEach,beforeEach,expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync,mkdirSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{validateJsonSchemaSubset}from"../../../../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{requireTestToolV1}from"../../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import corpus from"./🔣️.json";
import{nxSdkStampBaseV1,nxSdkStampKeyV1,nxSdkStampMemoFileV1,nxSdkStampPolicyV1,openNxSdkStampMemoV1,resetNxSdkStampMemosV1}from"../🟦️.ts";
const oracle=JSON.parse(spawnSync(requireTestToolV1("python"),["-I",join(import.meta.dir,"🐍️oracle","🐍️.py")],{input:JSON.stringify(corpus),encoding:"utf8"}).stdout);
const facts=(row:Record<string,string>)=>({dev:BigInt(row.dev),ino:BigInt(row.ino),size:BigInt(row.size),mtimeNs:BigInt(row.mtimeNs),mode:BigInt(row.mode)});
const first=facts(corpus.keys[0]!);
let base="";
const environment=()=>({SEMIO_NX_SDK_STAMPS_DIR:base});
beforeEach(()=>{base=mkdtempSync(join(tmpdir(),"nx-sdk-stamps-"));resetNxSdkStampMemosV1();});
afterEach(()=>{rmSync(base,{recursive:true,force:true});});

test("the stamp key and the memo file name equal the independent Python oracle",()=>{
 expect(corpus.keys.map(row=>nxSdkStampKeyV1(facts(row)))).toEqual(oracle.keys);
 expect(corpus.files.map(row=>nxSdkStampMemoFileV1(row.base,row.directory,row.sourceIndex))).toEqual(oracle.files);
});

test("the memo schema agrees with Ajv and with the subset validator on every document",()=>{
 const ajv=new Ajv({strict:true}).compile({...schema.definitions.Memo,definitions:undefined});
 for(const row of corpus.documents){
  if(!("document" in row))continue;
  expect(validateJsonSchemaSubset(schema.definitions.Memo,row.document,schema).length===0,row.name).toBe(ajv(row.document));
 }
});

test("a memo document is read as warm exactly when the oracle says so",async()=>{
 const results:boolean[]=[];
 for(const row of corpus.documents){
  resetNxSdkStampMemosV1();
  const directory="/virtual/sdk/"+row.name,file=nxSdkStampMemoFileV1(base,directory,row.sourceIndex);
  mkdirSync(dirname(file),{recursive:true});
  const text="raw"in row&&row.raw!==undefined?row.raw:JSON.stringify(row.document);
  writeFileSync(file,text);
  const memo=await openNxSdkStampMemoV1(directory,row.sourceIndex,{environment:environment(),now:()=>row.now});
  const path="document"in row&&Object.keys(row.document?.stamps??{})[0]||"node_modules/a";
  results.push(memo.has(path,first));
 }
 const expected=oracle.documents.map((warm:boolean,index:number)=>warm&&"document"in corpus.documents[index]!&&Object.keys(corpus.documents[index]!.document!.stamps).length>0);
 expect(results).toEqual(expected);
});

test("a committed memo survives a new process, and any change of the stamp is a miss",async()=>{
 const directory="/virtual/sdk/live",index=corpus.index,now=()=>corpus.now;
 const writer=await openNxSdkStampMemoV1(directory,index,{environment:environment(),now});
 expect(writer.has("node_modules/a",first)).toBe(false);
 writer.add("node_modules/a",first);
 await writer.commit();
 expect(readdirSync(join(base,nxSdkStampPolicyV1.directory)).filter(name=>name.endsWith(".tmp"))).toEqual([]);
 resetNxSdkStampMemosV1();
 const reader=await openNxSdkStampMemoV1(directory,index,{environment:environment(),now});
 expect(reader.has("node_modules/a",first)).toBe(true);
 for(const changed of[{ino:first.ino+1n},{size:first.size+1n},{mtimeNs:first.mtimeNs+1n},{mode:first.mode-0o222n},{dev:first.dev+1n}])expect(reader.has("node_modules/a",{...first,...changed})).toBe(false);
 expect(reader.has("node_modules/b",first)).toBe(false);
 const other=await openNxSdkStampMemoV1(directory,corpus.other,{environment:environment(),now});
 expect(other.has("node_modules/a",first)).toBe(false);
});

test("the stamp ignores ctime and link count, so a new hardlink generation keeps the memo warm",async()=>{
 const memo=await openNxSdkStampMemoV1("/virtual/sdk/links",corpus.index,{environment:environment(),now:()=>corpus.now});
 memo.add("node_modules/a",first);
 expect(memo.has("node_modules/a",{...first,ctimeNs:123n,nlink:7n}as typeof first)).toBe(true);
});

test("a memo older than a day is cold in the same process and after a restart, and the next commit replaces it",async()=>{
 const directory="/virtual/sdk/old",opened=(at:number)=>openNxSdkStampMemoV1(directory,corpus.index,{environment:environment(),now:()=>at});
 const writer=await opened(corpus.now);
 writer.add("node_modules/a",first);
 await writer.commit();
 const late=corpus.now+nxSdkStampPolicyV1.maximumAgeMilliseconds+1;
 expect((await opened(late)).has("node_modules/a",first)).toBe(false);
 resetNxSdkStampMemosV1();
 const cold=await opened(late);
 expect(cold.has("node_modules/a",first)).toBe(false);
 cold.add("node_modules/a",first);
 await cold.commit();
 resetNxSdkStampMemosV1();
 expect((await opened(late)).has("node_modules/a",first)).toBe(true);
 const written=JSON.parse(readFileSync(join(base,nxSdkStampPolicyV1.directory,readdirSync(join(base,nxSdkStampPolicyV1.directory))[0]!),"utf8"));
 expect(written.verifiedAt).toBe(late);
});

test("an environment of off, an empty value or a relative path never remembers a stamp",async()=>{
 expect(nxSdkStampBaseV1({SEMIO_NX_SDK_STAMPS_DIR:"off"})).toBeUndefined();
 expect(nxSdkStampBaseV1({SEMIO_NX_SDK_STAMPS_DIR:"relative/dir"})).toBeUndefined();
 expect(nxSdkStampBaseV1({SEMIO_NX_SDK_STAMPS_DIR:"/absolute/dir"})).toBe("/absolute/dir");
 expect(nxSdkStampBaseV1({SEMIO_NX_SDK_STAMPS_DIR:"",HOME:"/home/x"})).toEqual(expect.stringContaining("semio-repo"));
 const memo=await openNxSdkStampMemoV1("/virtual/sdk/off",corpus.index,{environment:{SEMIO_NX_SDK_STAMPS_DIR:"off"}});
 memo.add("node_modules/a",first);
 await memo.commit();
 expect(memo.has("node_modules/a",first)).toBe(false);
});

test("the file sdk-verified/off turns the memo off for every process that reads the same base",()=>{
 expect(nxSdkStampBaseV1(environment())).toBe(base);
 mkdirSync(join(base,nxSdkStampPolicyV1.directory),{recursive:true});
 writeFileSync(join(base,nxSdkStampPolicyV1.directory,nxSdkStampPolicyV1.disabledMarker),"");
 expect(nxSdkStampBaseV1(environment())).toBeUndefined();
});

test("a memo that cannot be written never fails the verification that wants it",async()=>{
 const blocker=join(base,"blocked");
 writeFileSync(blocker,"file");
 const memo=await openNxSdkStampMemoV1("/virtual/sdk/blocked",corpus.index,{environment:{SEMIO_NX_SDK_STAMPS_DIR:blocker},now:()=>corpus.now});
 memo.add("node_modules/a",first);
 await expect(memo.commit()).resolves.toBeUndefined();
 expect(memo.has("node_modules/a",first)).toBe(true);
});
