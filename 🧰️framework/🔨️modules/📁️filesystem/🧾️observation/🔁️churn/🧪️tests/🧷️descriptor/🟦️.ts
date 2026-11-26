/** 🧷️ Contract: an observed-then-opened file is verified by its class. An append-only file survives a sibling's append and a metadata-only touch (identity is device, inode and link count, the size only grows); an immutable file is strict but observed again a bounded number of times and then refused as a concurrent edit with an EN/DE text; a file replaced by rename is reopened by name and verified again. The stamp equals an independent Python stat. */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {appendFileSync,closeSync,fstatSync,lstatSync,mkdirSync,mkdtempSync,openSync,realpathSync,renameSync,rmSync,truncateSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import schema from "../../🧬️schema/🧷️descriptor.json";
import {ConcurrentEditErrorV1,isConcurrentEditV1} from "../../🟦️.ts";
import {descriptorFactsOfStampV1,descriptorFactsV1,descriptorProtocolV1,descriptorStampV1,settleDescriptorV1} from "../../🧷️descriptor/🟦️.ts";
import {testPythonV1} from "../../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";

const python=testPythonV1(),protocol=descriptorProtocolV1(),label={en:"Closure index",de:"Abschlussindex"};
let sandbox="";
beforeAll(()=>{sandbox=realpathSync(mkdtempSync(join(tmpdir(),"semio-descriptor-")));});
afterAll(()=>{rmSync(sandbox,{recursive:true,force:true});});
const facts=(path:string)=>descriptorFactsV1(lstatSync(path,{bigint:true}));
const none=()=>{};

test("the schema declares both languages for every class and the pauses",()=>{
 const ajv=new Ajv({strict:false}),text=ajv.compile({definitions:schema.definitions,$ref:"#/definitions/Text"});
 for(const kind of protocol.kinds)expect(text(protocol.texts[kind])).toBe(true);
 expect(protocol.pausesMilliseconds.length).toBeGreaterThan(0);
});

test("the stamp equals an independent Python stat and parses back",()=>{
 const path=join(sandbox,"stamp.json");writeFileSync(path,"{}\n");
 const mine=descriptorStampV1(facts(path));
 if(python){const answer=Bun.spawnSync([python,"-I","-c","import os,sys\ns=os.lstat(sys.argv[1])\nprint(':'.join(str(v) for v in (s.st_dev,s.st_ino,s.st_mode,s.st_nlink,s.st_size,s.st_mtime_ns,s.st_ctime_ns)))",path]);expect(answer.stdout.toString().trim()).toBe(mine);}
 expect(descriptorStampV1(descriptorFactsOfStampV1(mine))).toBe(mine);
});

test("append-only: a sibling's append and a metadata touch between the name and the descriptor are not a refusal (the m12 sequence)",()=>{
 const path=join(sandbox,"member.idx");writeFileSync(path,"one\n");
 const expected=facts(path),fd=openSync(path,"r");
 try{
  appendFileSync(path,"two\n");utimesSync(path,new Date(1e9),new Date(1e9));
  const verdict=settleDescriptorV1({kind:"append-only",path,label,expected,descriptor:()=>descriptorFactsV1(fstatSync(fd,{bigint:true})),sleepSync:none});
  expect(verdict.state).toBe("grew");expect(verdict.facts.size).toBe(8n);
  const touched=facts(path);utimesSync(path,new Date(2e9),new Date(2e9));
  expect(settleDescriptorV1({kind:"append-only",path,label,expected:touched,descriptor:()=>descriptorFactsV1(fstatSync(fd,{bigint:true})),sleepSync:none}).state).toBe("touched");
 }finally{closeSync(fd);}
});

test("append-only: a shortened or replaced file is observed again, then refused in both languages",()=>{
 const path=join(sandbox,"shorter.idx");writeFileSync(path,"abcdef");
 const expected=facts(path),fd=openSync(path,"r"),pauses:number[]=[];
 try{
  truncateSync(path,2);
  let error:unknown;try{settleDescriptorV1({kind:"append-only",path,label,expected,descriptor:()=>descriptorFactsV1(fstatSync(fd,{bigint:true})),sleepSync:ms=>pauses.push(ms)});}catch(raised){error=raised;}
  expect(error).toBeInstanceOf(ConcurrentEditErrorV1);expect(isConcurrentEditV1(error)).toBe(true);
  const message=(error as Error).message;expect(message).toContain("Closure index append-only file was replaced");expect(message).toContain("Abschlussindex anhängbare Datei wurde ersetzt");expect(message).toContain(path);
  expect(pauses).toEqual([...protocol.pausesMilliseconds]);
 }finally{closeSync(fd);}
 const swapped=join(sandbox,"swapped.idx");writeFileSync(swapped,"abc");const before=facts(swapped),old=openSync(swapped,"r");
 try{writeFileSync(swapped+".new","abc");renameSync(swapped+".new",swapped);const replaced=facts(swapped);expect(replaced.ino===before.ino).toBe(false);
  expect(()=>settleDescriptorV1({kind:"append-only",path:swapped,label,expected:replaced,descriptor:()=>descriptorFactsV1(fstatSync(old,{bigint:true})),sleepSync:none})).toThrow(ConcurrentEditErrorV1);
 }finally{closeSync(old);}
});

test("immutable: strict, a touch is refused after the bounded re-observation, a vanished difference is accepted",()=>{
 const path=join(sandbox,"leaf.json");writeFileSync(path,"{}");
 const expected=facts(path),fd=openSync(path,"r");
 try{
  expect(settleDescriptorV1({kind:"immutable",path,label,expected,descriptor:()=>descriptorFactsV1(fstatSync(fd,{bigint:true})),sleepSync:none}).state).toBe("same");
  let reads=0;const flicker=()=>{const real=descriptorFactsV1(fstatSync(fd,{bigint:true}));return reads++<1?{...real,ctimeNs:real.ctimeNs+1n}:real;};
  expect(settleDescriptorV1({kind:"immutable",path,label,expected,descriptor:flicker,sleepSync:none}).state).toBe("same");expect(reads).toBe(2);
  utimesSync(path,new Date(3e9),new Date(3e9));
  const pauses:number[]=[];let error:unknown;try{settleDescriptorV1({kind:"immutable",path,label,expected,descriptor:()=>descriptorFactsV1(fstatSync(fd,{bigint:true})),sleepSync:ms=>pauses.push(ms)});}catch(raised){error=raised;}
  expect(error).toBeInstanceOf(ConcurrentEditErrorV1);expect((error as Error).message).toContain("Closure index file changed while it was open / Abschlussindex Datei hat sich geändert, während sie offen war");
  expect(pauses.length).toBe(protocol.pausesMilliseconds.length);
 }finally{closeSync(fd);}
});

test("renamed: a file replaced by rename between the name and the descriptor is reopened by name and verified again",()=>{
 const path=join(sandbox,"receipt.json");writeFileSync(path,"old");
 const expected=facts(path);let fd=openSync(path,"r");
 try{
  writeFileSync(path+".new","newer");renameSync(path+".new",path);
  let reopened=0;
  const verdict=settleDescriptorV1({kind:"renamed",path,label,expected,descriptor:()=>descriptorFactsV1(fstatSync(fd,{bigint:true})),named:()=>facts(path),reopen:()=>{reopened++;closeSync(fd);fd=openSync(path,"r");},sleepSync:none});
  expect(verdict.state).toBe("renamed");expect(reopened).toBe(1);expect(verdict.facts.size).toBe(5n);expect(descriptorStampV1(verdict.facts)).toBe(descriptorStampV1(facts(path)));
  const stuck=settleDescriptorV1({kind:"renamed",path,label,expected:verdict.facts,descriptor:()=>descriptorFactsV1(fstatSync(fd,{bigint:true})),named:()=>facts(path),sleepSync:none});
  expect(stuck.state).toBe("same");
  mkdirSync(join(sandbox,"gone"));const gone=join(sandbox,"gone","x");writeFileSync(gone,"x");const keep=openSync(gone,"r");
  try{rmSync(gone);expect(()=>settleDescriptorV1({kind:"renamed",path:gone,label,expected:facts(path),descriptor:()=>descriptorFactsV1(fstatSync(keep,{bigint:true})),named:()=>facts(gone),sleepSync:none})).toThrow(ConcurrentEditErrorV1);}finally{closeSync(keep);}
 }finally{closeSync(fd);}
});
