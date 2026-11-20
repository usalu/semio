import {test,expect} from "bun:test";
import {mkdtempSync,mkdirSync,writeFileSync,symlinkSync,linkSync,rmSync,truncateSync} from "node:fs";
import {join,resolve} from "node:path";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {phaseCompilerAdmissionControlV1} from "../../../../../../🏗️build/🎟️capacity/🟦️.ts";
import {readPhysicalCargoSourceV1} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "../🧬️schema/🔣️.json";
test("physical Source reads agree with neutral limits and independent SQLite ownership oracle",async()=>{
 const base=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!base)throw Error("Original diagnostic fixture directory required");const root=mkdtempSync(join(resolve(base),"sr-")),database=new Database(":memory:"),ajv=new Ajv(),control=phaseCompilerAdmissionControlV1(new AbortController().signal,60000,()=>undefined,()=>new Promise(done=>setImmediate(done)));
 try{for(const row of corpus.cases){expect(ajv.validate(schema.definitions.Case,row)).toBe(true);const directory=join(root,row.name);mkdirSync(directory);let path=join(directory,"source.rs");if(row.kind!=="missing"){writeFileSync(path,Buffer.alloc(row.bytes,65));if(row.kind==="hardlink")linkSync(path,join(directory,"alias.rs"));if(row.kind==="symlink"){symlinkSync(path,join(directory,"link.rs"));path=join(directory,"link.rs");}if(row.kind==="ancestor"){symlinkSync(directory,join(root,"linked"),process.platform==="win32"?"junction":"dir");path=join(root,"linked/source.rs");}}const oracle=database.query<{accepted:number},[string,number,number,number]>("select ?1='missing' or (?1='regular' and ?2<=?3 and ?4>0) accepted").get(row.kind,row.bytes,row.maximumBytes,row.remainingFiles)!;expect(Boolean(oracle.accepted)).toBe(row.accepted);const read=()=>readPhysicalCargoSourceV1(path,{version:1,root,expectedBytes:null,maximumBytes:row.maximumBytes,remainingFiles:row.remainingFiles},control);if(row.accepted)expect((await read())?.byteLength??null).toBe(row.kind==="missing"?null:row.bytes);else await expect(read()).rejects.toThrow();}}finally{database.close();rmSync(root,{recursive:true,force:true});}
});

test("oversized Source never allocates its payload and changed Source refuses after original continuation",async()=>{
 const base=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!base)throw Error("Original diagnostic fixture directory required");const root=mkdtempSync(join(resolve(base),"sb-")),path=join(root,"source.rs"),original=Buffer.alloc;let allocations=0;
 try{writeFileSync(path,"");truncateSync(path,16777216);const control=phaseCompilerAdmissionControlV1(new AbortController().signal,60000,()=>undefined,()=>new Promise(done=>setImmediate(done)));Buffer.alloc=((...args:Parameters<typeof Buffer.alloc>)=>{allocations++;return original(...args);}) as typeof Buffer.alloc;await expect(readPhysicalCargoSourceV1(path,{version:1,root,expectedBytes:null,maximumBytes:8,remainingFiles:1},control)).rejects.toThrow("process.source.read-bytes-exceeded");expect(allocations).toBe(0);Buffer.alloc=original;writeFileSync(path,original(131072,65));let changed=false,flips=0;const mutation=phaseCompilerAdmissionControlV1(new AbortController().signal,60000,()=>undefined,async()=>{changed=true;writeFileSync(path,original(131072,66+(++flips)%2));});await expect(readPhysicalCargoSourceV1(path,{version:1,root,expectedBytes:null,maximumBytes:131072,remainingFiles:1},mutation)).rejects.toThrow(/kept changing in all 6 attempts/);expect(changed).toBe(true);expect(flips).toBeGreaterThan(1);}finally{Buffer.alloc=original;rmSync(root,{recursive:true,force:true});}
});

test("physical Source boundary refusals retain bounded exact facts and independent reasons",async()=>{
 const base=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!base)throw Error("Original diagnostic fixture directory required");const root=mkdtempSync(join(resolve(base),"sf-")),path=join(root,"source.rs"),db=new Database(":memory:"),ajv=new Ajv(),control=phaseCompilerAdmissionControlV1(new AbortController().signal,60000,()=>undefined,()=>new Promise(done=>setImmediate(done)));
 try{for(const row of corpus.refusals){expect(ajv.validate(schema.definitions.RefusalCase,row)).toBe(true);writeFileSync(path,Buffer.alloc(row.bytes,65));let caught:Error|undefined;try{await readPhysicalCargoSourceV1(path,{version:1,root,expectedBytes:row.expectedBytes,maximumBytes:row.maximumBytes,remainingFiles:row.remainingFiles},control);}catch(error){caught=error as Error;}expect(caught).toBeInstanceOf(Error);const facts=caught!.cause as any;expect(ajv.validate(schema.definitions.Refusal,facts)).toBe(true);const expected=db.query("SELECT CASE WHEN ?1=0 THEN 'files' WHEN ?2>?3 THEN 'bytes' ELSE 'extent' END reason").get(row.remainingFiles,row.bytes,row.maximumBytes)as{reason:string};expect(facts).toEqual({version:1,reason:expected.reason,path,observedBytes:String(row.bytes),maximumBytes:row.maximumBytes,remainingFiles:row.remainingFiles,expectedBytes:row.expectedBytes});expect(facts.reason).toBe(row.reason);expect((caught as any).key).toBe(row.key);expect((caught as any).values).toEqual(row.reason==="files"?{file:path,remainingFiles:row.remainingFiles}:row.reason==="bytes"?{file:path,observedBytes:String(row.bytes),maximumBytes:row.maximumBytes}:{file:path,observedBytes:String(row.bytes),expectedBytes:row.expectedBytes});expect(caught!.message.length).toBeLessThan(1024);}}finally{db.close();rmSync(root,{recursive:true,force:true});}
});

import {spyOn} from "bun:test";
import * as filesystem from "node:fs/promises";
import {renameSync,utimesSync} from "node:fs";
import {createPhysicalCargoSourceReaderV1} from "../🟦️.ts";
const issued=()=>phaseCompilerAdmissionControlV1(new AbortController().signal,60000,()=>undefined,()=>new Promise(done=>setImmediate(done)));

test("one run verifies each directory once, opens each unchanged file once and stays bounded per file",async()=>{
 const base=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!base)throw Error("Original diagnostic fixture directory required");const{throughput}=corpus,ajv=new Ajv();expect(ajv.validate(schema.definitions.Throughput,throughput)).toBe(true);
 const root=mkdtempSync(join(resolve(base),"st-")),directory=Array.from({length:throughput.depth},(_,index)=>"d"+index).reduce((path,name)=>join(path,name),root),paths=Array.from({length:throughput.files},(_,index)=>join(directory,"f"+index+".rs"));
 mkdirSync(directory,{recursive:true});paths.forEach((path,index)=>writeFileSync(path,"fn f"+index+"(){}\n"));await new Promise(done=>setTimeout(done,2200));
 const lstats=spyOn(filesystem,"lstat"),opens=spyOn(filesystem,"open"),control=issued(),reader=createPhysicalCargoSourceReaderV1();
 try{const first=new Map<string,Uint8Array|null>();for(let unit=0;unit<throughput.units;unit++){for(const path of paths){const bytes=await reader.read(path,{version:1,root,expectedBytes:null,maximumBytes:1024,remainingFiles:8},control);expect(bytes).not.toBeNull();if(unit===0)first.set(path,bytes);else expect(bytes).toBe(first.get(path)!);}await reader.sweep(control);}
  expect(opens.mock.calls.length).toBe(throughput.opens);expect(lstats.mock.calls.length/(throughput.files*throughput.units)).toBeLessThan(throughput.maximumLstatsPerFile);}finally{lstats.mockRestore();opens.mockRestore();rmSync(root,{recursive:true,force:true});}
},60000);

test("a directory replaced after a shared read is refused by the next sweep and a symbolic ancestor is refused at once",async()=>{
 const base=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!base)throw Error("Original diagnostic fixture directory required");const root=mkdtempSync(join(resolve(base),"sw-")),control=issued(),reader=createPhysicalCargoSourceReaderV1(),request={version:1 as const,root,expectedBytes:null,maximumBytes:64,remainingFiles:4};
 try{mkdirSync(join(root,"a/b"),{recursive:true});writeFileSync(join(root,"a/b/x.rs"),"fn x(){}");expect((await reader.read(join(root,"a/b/x.rs"),request,control))?.byteLength).toBe(8);await reader.sweep(control);
  expect((await reader.read(join(root,"a/b/x.rs"),request,control))?.byteLength).toBe(8);renameSync(join(root,"a"),join(root,"old"));mkdirSync(join(root,"a/b"),{recursive:true});writeFileSync(join(root,"a/b/x.rs"),"fn x(){}");
  await expect(reader.sweep(control)).rejects.toThrow(/ancestry/);
  mkdirSync(join(root,"real"));writeFileSync(join(root,"real/y.rs"),"fn y(){}");symlinkSync(join(root,"real"),join(root,"link"),process.platform==="win32"?"junction":"dir");
  await expect(createPhysicalCargoSourceReaderV1().read(join(root,"link/y.rs"),request,control)).rejects.toThrow(/no-follow/);}finally{rmSync(root,{recursive:true,force:true});}
},60000);

test("a shared file is read again once changed and a cached file still obeys the bounds of every request",async()=>{
 const base=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!base)throw Error("Original diagnostic fixture directory required");const root=mkdtempSync(join(resolve(base),"sc-")),path=join(root,"s.rs"),control=issued(),reader=createPhysicalCargoSourceReaderV1(),request={version:1 as const,root,expectedBytes:null,maximumBytes:64,remainingFiles:1};
 try{writeFileSync(path,"fn one(){}");await new Promise(done=>setTimeout(done,2200));const first=await reader.read(path,request,control),again=await reader.read(path,request,control);expect(again).toBe(first!);
  await expect(reader.read(path,{...request,maximumBytes:4},control)).rejects.toThrow("process.source.read-bytes-exceeded");await expect(reader.read(path,{...request,remainingFiles:0},control)).rejects.toThrow("process.source.read-files-exhausted");await expect(reader.read(path,{...request,expectedBytes:3},control)).rejects.toThrow("process.source.read-extent-mismatched");
  writeFileSync(path,"fn two(){}");utimesSync(path,new Date(),new Date());const changed=await reader.read(path,request,control);expect(Buffer.from(changed!).toString()).toBe("fn two(){}");}finally{rmSync(root,{recursive:true,force:true});}
},60000);
