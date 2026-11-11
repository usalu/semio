import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {chmodSync,existsSync,lstatSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {joinV1,seedV1,type ReplicaEventV1} from "../../🟦️.ts";
import {BootstrapRefusedV1,bootstrapPathsV1,settleBootstrapV1,tarEndV1,tarEntryHeaderV1,tarPaddingV1,writeBootstrapArchiveV1} from "../🟦️.ts";

const python=testPythonV1(),oracle=join(import.meta.dirname,"🐍️.py"),system=Bun.which("tar"),scratch:string[]=[];
const temp=(label:string):string=>{const path=mkdtempSync(join(tmpdir(),`bootstrap-${label}-`));scratch.push(path);return path;};
const sha=(data:Uint8Array|string):string=>createHash("sha256").update(data).digest("hex");
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

const LONG=["🧰️framework","🛍️products","🦑️repo","🔨️modules","🤝️shared-worktree","🔁️replica","🧪️tests","🟦️.ts"].join("/");
const FILES:Readonly<Record<string,string|Buffer>>={[LONG]:"export const x=1;\n","a.txt":"hello\n","bin/run.sh":"#!/bin/sh\necho ok\n","empty":"","big/blob.bin":Buffer.alloc(3*1024*1024+17,7)};

function checksum(header:Buffer):void{
 const copy=Buffer.from(header);
 copy.fill(0x20,148,156);
 let sum=0;
 for(const byte of copy)sum+=byte;
 expect(parseInt(header.subarray(148,154).toString("latin1"),8)).toBe(sum);
 expect(header[154]).toBe(0);
 expect(header[155]).toBe(0x20);
}

function records(body:Buffer):Record<string,string>{
 const found:Record<string,string>={};
 let offset=0;
 while(offset<body.length){
  const space=body.indexOf(0x20,offset),length=Number(body.subarray(offset,space).toString("latin1"));
  expect(body[offset+length-1]).toBe(0x0a);
  const text=body.subarray(space+1,offset+length-1).toString("utf8"),cut=text.indexOf("=");
  found[text.slice(0,cut)]=text.slice(cut+1);
  offset+=length;
 }
 return found;
}

function entries(tar:Buffer):Map<string,Readonly<{size:number;data:Buffer;mode:number}>>{
 const found=new Map<string,{size:number;data:Buffer;mode:number}>();
 let offset=0,extended:Record<string,string>={};
 while(offset+512<=tar.length){
  const head=tar.subarray(offset,offset+512);
  if(head.every(byte=>byte===0))break;
  checksum(head);
  const type=String.fromCharCode(head[156]!),stated=parseInt(head.subarray(124,135).toString("latin1"),8),size=extended["size"]!==undefined?Number(extended["size"]):stated;
  offset+=512;
  if(type==="x"){extended=records(tar.subarray(offset,offset+stated));offset+=Math.ceil(stated/512)*512;continue;}
  expect(type).toBe("0");
  const name=extended["path"]??head.subarray(0,100).toString("latin1").replace(/\0.*$/u,"");
  found.set(name,{size,data:Buffer.from(tar.subarray(offset,offset+size)),mode:parseInt(head.subarray(100,107).toString("latin1"),8)});
  offset+=Math.ceil(size/512)*512;
  extended={};
 }
 return found;
}

async function server(label:string){
 const store=temp(`${label}-store`),worktree=temp(`${label}-tree`),options={worktree,storeRoot:store,originId:"g1-server",groupId:"g1"};
 for(const [path,content] of Object.entries(FILES)){
  const file=join(worktree,...path.split("/"));
  mkdirSync(dirname(file),{recursive:true});writeFileSync(file,content);
 }
 chmodSync(join(worktree,"bin","run.sh"),0o755);
 await seedV1(options);
 return {store,worktree,options};
}

const expectedList=(overrides:Readonly<Record<string,string|Buffer>>={})=>Object.entries({...FILES,...overrides}).map(([path,content])=>({path,bytes:Buffer.byteLength(content),sha256:sha(content),executable:path==="bin/run.sh"}));

test("an ASCII entry is one ustar block with a valid checksum, 512-byte blocking and padding",()=>{
 const header=tarEntryHeaderV1("a/b.txt",5,false,1_700_000_000);
 expect(header.length).toBe(512);
 expect(header.subarray(257,263).toString("latin1")).toBe("ustar\0");
 expect(header.subarray(263,265).toString("latin1")).toBe("00");
 expect(header.subarray(100,108).toString("latin1")).toBe("0000644\0");
 expect(header.subarray(124,136).toString("latin1")).toBe("00000000005\0");
 expect(String.fromCharCode(header[156]!)).toBe("0");
 checksum(header);
 expect(tarEntryHeaderV1("run.sh",5,true,0).subarray(100,108).toString("latin1")).toBe("0000755\0");
 expect([0,1,511,512,513,1024].map(tarPaddingV1)).toEqual([0,511,1,0,511,0]);
 expect(tarEndV1().length).toBe(1024);
 expect(tarEndV1().every(byte=>byte===0)).toBe(true);
});

test("a long emoji path travels in a pax header and the ustar name stays plain ASCII",()=>{
 expect(Buffer.byteLength(LONG)).toBeGreaterThan(100);
 const block=tarEntryHeaderV1(LONG,3,true,1_700_000_000);
 expect(block.length%512).toBe(0);
 expect(block.length).toBe(3*512);
 const extended=block.subarray(0,512),entry=block.subarray(1024,1536),size=parseInt(extended.subarray(124,135).toString("latin1"),8);
 expect(String.fromCharCode(extended[156]!)).toBe("x");
 checksum(extended);
 expect(records(block.subarray(512,512+size))).toEqual({path:LONG});
 expect(String.fromCharCode(entry[156]!)).toBe("0");
 checksum(entry);
 expect(entry.subarray(0,100).every(byte=>byte===0||(byte>=0x20&&byte<0x7f))).toBe(true);
 expect(entry.subarray(100,108).toString("latin1")).toBe("0000755\0");
 expect(parseInt(entry.subarray(124,135).toString("latin1"),8)).toBe(3);
});

test("a short non-ASCII name and a name of exactly 101 bytes also use pax, 100 ASCII bytes do not",()=>{
 expect(tarEntryHeaderV1("ü.txt",1,false,0).length).toBe(3*512);
 expect(tarEntryHeaderV1("a".repeat(100),1,false,0).length).toBe(512);
 const long=tarEntryHeaderV1("a".repeat(101),1,false,0);
 expect(long.length).toBe(3*512);
 expect(records(long.subarray(512,512+parseInt(long.subarray(124,135).toString("latin1"),8)))).toEqual({path:"a".repeat(101)});
});

test("a file of 8 GiB or more carries its size in a pax record and the ustar size field holds 0",()=>{
 const huge=tarEntryHeaderV1("big.bin",9*2**30,false,1);
 expect(huge.length).toBe(3*512);
 expect(String.fromCharCode(huge[0+156]!)).toBe("x");
 expect(records(huge.subarray(512,512+parseInt(huge.subarray(124,135).toString("latin1"),8)))).toEqual({size:"9663676416"});
 const entry=huge.subarray(1024,1536);
 expect(entry.subarray(124,136).toString("latin1")).toBe("00000000000\0");
 checksum(entry);
 expect(tarEntryHeaderV1("edge.bin",8**11-1,false,1).length).toBe(512);
 expect(tarEntryHeaderV1("edge.bin",8**11,false,1).length).toBe(3*512);
 const both=tarEntryHeaderV1(LONG,9*2**30,false,1);
 expect(records(both.subarray(512,512+parseInt(both.subarray(124,135).toString("latin1"),8)))).toEqual({path:LONG,size:"9663676416"});
});

test("the archive holds exactly the shared files with their bytes and executable bit, and manifest.json describes the tar",async()=>{
 const {store,options}=await server("write"),events:ReplicaEventV1[]=[];
 const result=await writeBootstrapArchiveV1(options,undefined,event=>events.push(event));
 const paths=bootstrapPathsV1(store),tar=readFileSync(paths.archive),found=entries(tar);
 expect(paths.archive).toBe(join(store,"v1","bootstrap","worktree.tar"));
 expect(tar.length%512).toBe(0);
 expect(tar.subarray(tar.length-1024).every(byte=>byte===0)).toBe(true);
 expect([...found.keys()].sort()).toEqual(Object.keys(FILES).sort());
 for(const [path,content] of Object.entries(FILES)){
  expect(found.get(path)!.data.equals(Buffer.from(content)),path).toBe(true);
  expect(found.get(path)!.mode).toBe(path==="bin/run.sh"?0o755:0o644);
 }
 const total=Object.values(FILES).reduce((sum,content)=>sum+Buffer.byteLength(content),0);
 expect(JSON.parse(readFileSync(paths.manifest,"utf8"))).toEqual({version:1,hlc:result.hlc,files:5,bytes:total,sha256:sha(tar)});
 expect(result).toMatchObject({files:5,bytes:total,skipped:0,sha256:sha(tar),archive:paths.archive});
 expect(result.hlc).toMatch(/^[0-9]{13}-[0-9]{6}-g1-server$/u);
 expect(readdirSync(join(store,"v1","tmp")).filter(name=>name.includes("bootstrap"))).toEqual([]);
 expect(events.some(event=>event.event==="progress"&&event.stage==="bootstrap"&&event.done===5&&event.total===5)).toBe(true);
});

test("a file that differs from the manifest or vanished is skipped and the tar stays consistent",async()=>{
 const {store,worktree,options}=await server("skip"),events:ReplicaEventV1[]=[];
 writeFileSync(join(worktree,"a.txt"),"HELLO\n");
 writeFileSync(join(worktree,"big","blob.bin"),Buffer.alloc(3*1024*1024+17,8));
 rmSync(join(worktree,"empty"));
 const result=await writeBootstrapArchiveV1(options,undefined,event=>events.push(event));
 expect(result).toMatchObject({files:2,skipped:3});
 const paths=bootstrapPathsV1(store),found=entries(readFileSync(paths.archive));
 expect([...found.keys()].sort()).toEqual([LONG,"bin/run.sh"].sort());
 expect(events.filter(event=>event.event==="skipped").map(event=>`${event.path}:${event.reason}`).sort()).toEqual(["a.txt:changed","big/blob.bin:changed","empty:missing"]);
 expect(JSON.parse(readFileSync(paths.manifest,"utf8")).sha256).toBe(sha(readFileSync(paths.archive)));
});

test("a cancelled build leaves neither an archive nor staging files, an empty store is refused",async()=>{
 const {store,options}=await server("cancel"),controller=new AbortController();
 controller.abort();
 await expect(writeBootstrapArchiveV1(options,controller.signal)).rejects.toThrow();
 expect(existsSync(bootstrapPathsV1(store).archive)).toBe(false);
 expect(existsSync(join(store,"v1","tmp"))?readdirSync(join(store,"v1","tmp")).filter(name=>name.includes("bootstrap")):[]).toEqual([]);
 const empty=temp("empty");
 mkdirSync(join(empty,"v1"),{recursive:true});
 await expect(writeBootstrapArchiveV1({worktree:temp("none"),storeRoot:empty,originId:"g1-server"})).rejects.toBeInstanceOf(BootstrapRefusedV1);
 await expect(settleBootstrapV1({worktree:temp("none"),storeRoot:empty})).rejects.toMatchObject({key:"empty"});
 await expect(settleBootstrapV1({worktree:temp("none"),storeRoot:join(empty,"missing")})).rejects.toMatchObject({key:"store"});
});

test.skipIf(!system)("the system tar extracts the archive with the long emoji paths, the content and the executable bit",async()=>{
 const {store,options}=await server("extract");
 await writeBootstrapArchiveV1(options);
 const target=temp("extracted"),locale=process.platform==="linux"?"C.UTF-8":"en_US.UTF-8",run=Bun.spawnSync([system!,"-xf",bootstrapPathsV1(store).archive,"-C",target],{env:{...process.env,LC_ALL:locale},stdout:"pipe",stderr:"pipe"});
 expect(run.stderr.toString()).toBe("");
 expect(run.exitCode).toBe(0);
 for(const [path,content] of Object.entries(FILES)){
  const file=join(target,...path.split("/"));
  expect(readFileSync(file).equals(Buffer.from(content)),path).toBe(true);
 }
 if(process.platform!=="win32")expect(lstatSync(join(target,"bin","run.sh")).mode&0o111).not.toBe(0);
});

test.skipIf(!system)("a tree extracted from an older archive settles against a newer store and then joins",async()=>{
 const {store,worktree,options}=await server("settle");
 await writeBootstrapArchiveV1(options);
 const target=temp("joiner"),locale=process.platform==="linux"?"C.UTF-8":"en_US.UTF-8";
 expect(Bun.spawnSync([system!,"-xf",bootstrapPathsV1(store).archive,"-C",target],{env:{...process.env,LC_ALL:locale}}).exitCode).toBe(0);
 writeFileSync(join(worktree,"a.txt"),"newer\n");
 writeFileSync(join(worktree,"added.txt"),"added\n");
 await seedV1(options);
 const settled=await settleBootstrapV1({worktree:target,storeRoot:store,now:()=>1_700_000_000_000});
 expect(settled).toMatchObject({kept:4,moved:1,folder:join(target,".cache","f","lost","bootstrap-1700000000000")});
 expect(readFileSync(join(settled.folder,"a.txt"),"utf8")).toBe("hello\n");
 expect(readFileSync(join(target,"a.txt"),"utf8")).toBe("newer\n");
 const joined=await joinV1({worktree:target,storeRoot:store,originId:"g1-m01",groupId:"g1"});
 expect(joined.queued).toBe(0);
 expect(joined.adopted).toBe(5);
 expect(readFileSync(join(target,"a.txt"),"utf8")).toBe("newer\n");
 expect(readFileSync(join(target,"added.txt"),"utf8")).toBe("added\n");
 await expect(settleBootstrapV1({worktree:target,storeRoot:store})).rejects.toMatchObject({key:"joined"});
});

test.skipIf(!system||process.platform==="win32")("settle also moves aside a file whose executable bit differs from the store, so that join never republishes a chmod",async()=>{
 const {store,options}=await server("mode");
 await writeBootstrapArchiveV1(options);
 const target=temp("mode-joiner"),locale=process.platform==="linux"?"C.UTF-8":"en_US.UTF-8";
 expect(Bun.spawnSync([system!,"-xf",bootstrapPathsV1(store).archive,"-C",target],{env:{...process.env,LC_ALL:locale}}).exitCode).toBe(0);
 chmodSync(join(target,"bin","run.sh"),0o644);chmodSync(join(target,"a.txt"),0o755);
 const events:ReplicaEventV1[]=[],settled=await settleBootstrapV1({worktree:target,storeRoot:store,now:()=>1_700_000_000_001},undefined,event=>events.push(event));
 expect(settled).toMatchObject({kept:3,moved:2});
 expect(events.filter(event=>event.event==="skipped").map(event=>event.path).sort()).toEqual(["a.txt","bin/run.sh"]);
 expect(events.some(event=>event.event==="progress"&&event.stage==="settle"&&event.done===5&&event.total===5)).toBe(true);
 const joined=await joinV1({worktree:target,storeRoot:store,originId:"g1-m02",groupId:"g1"});
 expect(joined).toMatchObject({adopted:5,queued:0});
 expect(lstatSync(join(target,"bin","run.sh")).mode&0o111).not.toBe(0);expect(lstatSync(join(target,"a.txt")).mode&0o111).toBe(0);
});

test.skipIf(!python)("the Python oracle reads the archive with tarfile and agrees on names, sizes, hashes and the manifest",async()=>{
 const {store,options}=await server("oracle");
 await writeBootstrapArchiveV1(options);
 const expected=join(temp("expected"),"expected.json"),run=(file?:string)=>Bun.spawnSync([python!,"-I",oracle,"check",store,...(file?[file]:[])],{stdout:"pipe",stderr:"pipe"});
 writeFileSync(expected,JSON.stringify(expectedList()));
 const good=run(expected);
 expect(good.stderr.toString()+good.stdout.toString()).toBe("");
 expect(good.exitCode).toBe(0);
 writeFileSync(expected,JSON.stringify(expectedList({"a.txt":"other\n"})));
 expect(run(expected).exitCode).toBe(1);
 const paths=bootstrapPathsV1(store),manifest=JSON.parse(readFileSync(paths.manifest,"utf8"));
 writeFileSync(paths.manifest,JSON.stringify({...manifest,sha256:"0".repeat(64)}));
 const bad=run();
 expect(bad.exitCode).toBe(1);
 expect(bad.stderr.toString()).toContain("sha256 of the tar differs");
});
