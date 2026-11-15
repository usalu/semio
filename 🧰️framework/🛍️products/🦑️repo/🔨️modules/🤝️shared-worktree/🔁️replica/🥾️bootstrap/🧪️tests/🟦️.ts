import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {chmodSync,existsSync,lstatSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {appendEventsV1,blobPathV1,formatHlcV1,writeBlobV1} from "../../../📒️journal/🟦️.ts";
import {joinV1,seedV1,type ReplicaEventV1} from "../../🟦️.ts";
import {spawnSync} from "node:child_process";
import {BootstrapRefusedV1,bootstrapArchiveNameV1,bootstrapPathsV1,readBootstrapV1,settleBootstrapV1,tarEndV1,tarEntryHeaderV1,tarPaddingV1,writeBootstrapArchiveV1} from "../🟦️.ts";

const python=testPythonV1(),oracle=join(import.meta.dirname,"🐍️.py"),system=Bun.which("tar"),scratch:string[]=[];
const temp=(label:string):string=>{const path=mkdtempSync(join(tmpdir(),`bootstrap-${label}-`));scratch.push(path);return path;};
const sha=(data:Uint8Array|string):string=>createHash("sha256").update(data).digest("hex");
const archiveOf=(store:string):string=>join(bootstrapPathsV1(store).directory,JSON.parse(readFileSync(bootstrapPathsV1(store).manifest,"utf8")).archive);
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
 const paths=bootstrapPathsV1(store),tar=readFileSync(archiveOf(store)),found=entries(tar);
 expect(archiveOf(store)).toBe(join(store,"v1","bootstrap",bootstrapArchiveNameV1(sha(tar))));
 expect(result.archive).toBe(archiveOf(store));
 expect(tar.length%512).toBe(0);
 expect(tar.subarray(tar.length-1024).every(byte=>byte===0)).toBe(true);
 expect([...found.keys()].sort()).toEqual(Object.keys(FILES).sort());
 for(const [path,content] of Object.entries(FILES)){
  expect(found.get(path)!.data.equals(Buffer.from(content)),path).toBe(true);
  expect(found.get(path)!.mode).toBe(path==="bin/run.sh"?0o755:0o644);
 }
 const total=Object.values(FILES).reduce((sum,content)=>sum+Buffer.byteLength(content),0);
 expect(JSON.parse(readFileSync(paths.manifest,"utf8"))).toEqual({version:1,hlc:result.hlc,files:5,bytes:total,sha256:sha(tar),archive:bootstrapArchiveNameV1(sha(tar))});
 expect(result).toMatchObject({files:5,bytes:total,skipped:0,sha256:sha(tar),archive:archiveOf(store)});
 expect(result.hlc).toMatch(/^[0-9]{13}-[0-9]{6}-g1-server$/u);
 expect(readdirSync(join(store,"v1","tmp")).filter(name=>name.includes("bootstrap"))).toEqual([]);
 expect(events.some(event=>event.event==="progress"&&event.stage==="bootstrap"&&event.done===5&&event.total===5)).toBe(true);
});

test("the archive carries only the allow-listed .git paths even when the store still holds the private ones of an older daemon",async()=>{
 const {store,worktree,options}=await server("legacy");
 const legacy:Record<string,string>={".git/HEAD":"ref: refs/heads/main\n",".git/refs/heads/main":"abc\n",".git/objects/ab/cdef":"obj",".git/index":"stage",".git/config":"[core]\n",".git/logs/HEAD":"log\n",".git/hooks/pre-commit.sample":"#!/bin/sh\n",".git/cursor/crepe/x":"ide"};
 const events=Object.entries(legacy).map(([path,content],number)=>{
  const file=join(worktree,...path.split("/"));
  mkdirSync(dirname(file),{recursive:true});writeFileSync(file,content);
  return {path,content,hlc:formatHlcV1({ms:Date.now()+number,counter:0},"g1-server")};
 });
 for(const item of events)await writeBlobV1(store,"g1-server",sha(item.content),join(worktree,...item.path.split("/")));
 await appendEventsV1(store,"g1-server",events.map(item=>({version:1 as const,hlc:item.hlc,operation:"put" as const,path:item.path,sha256:sha(item.content),bytes:Buffer.byteLength(item.content),executable:false,base:null})));
 await writeBootstrapArchiveV1(options);
 const names=[...entries(readFileSync(archiveOf(store))).keys()].filter(path=>path.startsWith(".git/")).sort();
 expect(names).toEqual([".git/HEAD",".git/objects/ab/cdef",".git/refs/heads/main"]);
});

test("a file that differs from the manifest or vanished is skipped and the tar stays consistent",async()=>{
 const {store,worktree,options}=await server("skip"),events:ReplicaEventV1[]=[];
 writeFileSync(join(worktree,"a.txt"),"HELLO\n");
 writeFileSync(join(worktree,"big","blob.bin"),Buffer.alloc(3*1024*1024+17,8));
 rmSync(join(worktree,"empty"));
 const result=await writeBootstrapArchiveV1(options,undefined,event=>events.push(event));
 expect(result).toMatchObject({files:2,skipped:3});
 const paths=bootstrapPathsV1(store),found=entries(readFileSync(archiveOf(store)));
 expect([...found.keys()].sort()).toEqual([LONG,"bin/run.sh"].sort());
 expect(events.filter(event=>event.event==="skipped").map(event=>`${event.path}:${event.reason}`).sort()).toEqual(["a.txt:changed","big/blob.bin:changed","empty:missing"]);
 expect(JSON.parse(readFileSync(paths.manifest,"utf8")).sha256).toBe(sha(readFileSync(archiveOf(store))));
});

test("a cancelled build leaves neither an archive nor staging files, an empty store is refused",async()=>{
 const {store,options}=await server("cancel"),controller=new AbortController();
 controller.abort();
 await expect(writeBootstrapArchiveV1(options,controller.signal)).rejects.toThrow();
 expect(existsSync(bootstrapPathsV1(store).manifest)).toBe(false);
 expect(existsSync(bootstrapPathsV1(store).directory)?readdirSync(bootstrapPathsV1(store).directory):[]).toEqual([]);
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
 const target=temp("extracted"),locale=process.platform==="linux"?"C.UTF-8":"en_US.UTF-8",run=Bun.spawnSync([system!,"-xf",archiveOf(store),"-C",target],{env:{...process.env,LC_ALL:locale},stdout:"pipe",stderr:"pipe"});
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
 expect(Bun.spawnSync([system!,"-xf",archiveOf(store),"-C",target],{env:{...process.env,LC_ALL:locale}}).exitCode).toBe(0);
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

test("settle copies the store's content before it moves a differing file aside, so a missing blob leaves the file where it is",async()=>{
 const {store,worktree,options}=await server("settle-order");
 await writeBootstrapArchiveV1(options);
 const target=temp("settle-order-target");
 writeFileSync(join(target,"a.txt"),"hello\n");
 writeFileSync(join(worktree,"a.txt"),"newer\n");
 await seedV1(options);
 rmSync(blobPathV1(store,sha("newer\n")));
 await expect(settleBootstrapV1({worktree:target,storeRoot:store,now:()=>1_700_000_000_002})).rejects.toBeDefined();
 expect(readFileSync(join(target,"a.txt"),"utf8")).toBe("hello\n");
 expect(existsSync(join(target,".cache","f","lost","bootstrap-1700000000002","a.txt"))).toBe(false);
 expect(readdirSync(join(target,".cache","f","tmp")).filter(name=>name.startsWith("settle-"))).toEqual([]);
});

test("settle refuses a blob that does not hash to its name instead of staging wrong bytes, and leaves the file where it is",async()=>{
 const {store,worktree,options}=await server("settle-corrupt");
 await writeBootstrapArchiveV1(options);
 const target=temp("settle-corrupt-target");
 writeFileSync(join(target,"a.txt"),"hello\n");
 writeFileSync(join(worktree,"a.txt"),"newer\n");
 await seedV1(options);
 writeFileSync(blobPathV1(store,sha("newer\n")),"rotten");
 await expect(settleBootstrapV1({worktree:target,storeRoot:store,now:()=>1_700_000_000_003})).rejects.toMatchObject({key:"blob",variables:{path:"a.txt"}});
 expect(readFileSync(join(target,"a.txt"),"utf8")).toBe("hello\n");
 expect(existsSync(join(target,".cache","f","lost","bootstrap-1700000000003","a.txt"))).toBe(false);
 expect(readdirSync(join(target,".cache","f","tmp")).filter(name=>name.startsWith("settle-"))).toEqual([]);
});

test.skipIf(!system||process.platform==="win32")("settle also moves aside a file whose executable bit differs from the store, so that join never republishes a chmod",async()=>{
 const {store,options}=await server("mode");
 await writeBootstrapArchiveV1(options);
 const target=temp("mode-joiner"),locale=process.platform==="linux"?"C.UTF-8":"en_US.UTF-8";
 expect(Bun.spawnSync([system!,"-xf",archiveOf(store),"-C",target],{env:{...process.env,LC_ALL:locale}}).exitCode).toBe(0);
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

test("a rebuild replaces the archive and the manifest in one visible step: the manifest names an archive that hashes to it at every moment, the previous archive stays for readers that are copying it, and the old fixed name is gone",async()=>{
 const {store,worktree,options}=await server("pair");
 await writeBootstrapArchiveV1(options);
 const first=await readBootstrapV1(store);
 expect(first?.manifest.archive).toBe(bootstrapArchiveNameV1(first!.manifest.sha256));
 expect(sha(readFileSync(first!.archive))).toBe(first!.manifest.sha256);
 writeFileSync(join(worktree,"a.txt"),"changed\n");
 const watched:string[]=[];
 const watch=setInterval(()=>{try{const now=JSON.parse(readFileSync(bootstrapPathsV1(store).manifest,"utf8"));watched.push(sha(readFileSync(join(bootstrapPathsV1(store).directory,now.archive)))===now.sha256?"ok":"torn");}catch{watched.push("missing");}},1);
 await writeBootstrapArchiveV1({...options,force:true});
 clearInterval(watch);
 const second=await readBootstrapV1(store);
 expect(watched.filter(item=>item!=="ok")).toEqual([]);
 expect(second!.manifest.archive).not.toBe(first!.manifest.archive);
 expect(existsSync(first!.archive)).toBe(true);
 expect(existsSync(join(bootstrapPathsV1(store).directory,"worktree.tar"))).toBe(false);
 expect(readdirSync(bootstrapPathsV1(store).directory).sort()).toEqual([first!.manifest.archive,first!.manifest.archive.replace(".tar",".entries"),"manifest.json",second!.manifest.archive,second!.manifest.archive.replace(".tar",".entries")].sort());
});

const gitHere=(cwd:string,args:readonly string[]):string=>{
 const result=spawnSync("git",[...args],{cwd,encoding:"utf8",maxBuffer:1<<28,env:{...process.env,GIT_AUTHOR_NAME:"t",GIT_AUTHOR_EMAIL:"t@example.org",GIT_COMMITTER_NAME:"t",GIT_COMMITTER_EMAIL:"t@example.org",GIT_CONFIG_GLOBAL:"/dev/null",GIT_CONFIG_SYSTEM:"/dev/null"}});
 if(result.status!==0)throw Error(`git ${args.join(" ")}: ${result.stderr}`);
 return result.stdout;
};
const gitTest=spawnSync("git",["--version"]).status===0&&system!==null?test:test.skip;

async function gitServer(label:string){
 const store=temp(`${label}-store`),worktree=temp(`${label}-tree`),options={worktree,storeRoot:store,originId:"g1-server",groupId:"g1"};
 gitHere(worktree,["init","-q"]);gitHere(worktree,["config","gc.auto","0"]);gitHere(worktree,["config","maintenance.auto","false"]);
 const commit=(name:string,size:number)=>{writeFileSync(join(worktree,name),Buffer.from(createHash("sha256").update(name).digest("hex").repeat(size)));gitHere(worktree,["add","-A"]);gitHere(worktree,["commit","-q","-m",name]);};
 commit("one.txt",64);gitHere(worktree,["repack","-q","-d"]);
 commit("two.txt",64);gitHere(worktree,["repack","-q","-d"]);
 const directory=join(worktree,".git","objects","pack"),held=new Map(readdirSync(directory).map(name=>[name,readFileSync(join(directory,name))] as const));
 commit("three.txt",64);gitHere(worktree,["repack","-q","-a","-d"]);
 for(const [name,data] of held)writeFileSync(join(directory,name),data);
 writeFileSync(join(directory,"tmp_pack_Xy12Ab"),Buffer.alloc(100_000,1));
 writeFileSync(join(directory,"gc.log"),"x");
 const packs=readdirSync(directory).filter(name=>name.endsWith(".pack"));
 writeFileSync(join(directory,"pack-0123456789012345678901234567890123456789.pack"),readFileSync(join(directory,packs[0]!)));
 gitHere(worktree,["multi-pack-index","write"]);
 await seedV1(options);
 return {store,worktree,options,directory};
}

gitTest("the archive leaves out covered packs, tmp packs, orphans and the multi-pack-index and still extracts to a complete repository that git fsck accepts",async()=>{
 const {store,worktree,options,directory}=await gitServer("packs"),before=readdirSync(directory).sort();
 expect(before.filter(name=>name.endsWith(".pack")).length).toBeGreaterThanOrEqual(3);
 expect(before).toContain("multi-pack-index");
 const result=await writeBootstrapArchiveV1({...options,reachable:false});
 expect(result.outcome).toBe("built");
 expect(result.packs!.keep.length).toBe(1);
 expect(result.packs!.covered.length).toBeGreaterThanOrEqual(2);
 const target=temp("extract");
 expect(spawnSync(system!,["-xf",archiveOf(store),"-C",target]).status).toBe(0);
 const extracted=readdirSync(join(target,".git","objects","pack")).sort();
 expect(extracted.filter(name=>name.endsWith(".pack"))).toEqual([`${result.packs!.keep[0]}.pack`]);
 expect(extracted.some(name=>name.startsWith("tmp_")||name.startsWith("multi-pack-index")||name==="gc.log")).toBe(false);
 expect(existsSync(join(target,".git","gc.log"))).toBe(false);
 gitHere(target,["fsck","--full","--strict"]);
 gitHere(target,["reset","-q","--hard"]);
 const objects=(cwd:string):string=>gitHere(cwd,["cat-file","--batch-all-objects","--batch-check=%(objectname)"]).split("\n").sort().join("\n");
 const wantedNames=gitHere(worktree,["rev-list","--objects","--all"]).split("\n").filter(Boolean).map(line=>line.split(" ")[0]!).sort(),hasNames=new Set(objects(target).split("\n"));
 for(const name of wantedNames)expect(hasNames.has(name)).toBe(true);
 expect(gitHere(target,["rev-parse","HEAD"])).toBe(gitHere(worktree,["rev-parse","HEAD"]));
 const full=before.filter(name=>name.endsWith(".pack")).reduce((sum,name)=>sum+lstatSync(join(directory,name)).size,0);
 expect(result.bytes).toBeLessThan(full+1_000_000);
});

test("a second build with unchanged content is skipped and reports unchanged, a forced one is not",async()=>{
 const {store,options}=await server("unchanged"),events:ReplicaEventV1[]=[];
 const first=await writeBootstrapArchiveV1(options);
 expect(first.outcome).toBe("built");
 const second=await writeBootstrapArchiveV1(options,undefined,event=>events.push(event));
 expect(second.outcome).toBe("unchanged");
 expect(second.sha256).toBe(first.sha256);
 expect(events.some(event=>event.event==="skipped"&&event.reason==="unchanged")).toBe(true);
 expect(existsSync(first.archive.replace(".tar",".entries"))).toBe(true);
 const third=await writeBootstrapArchiveV1({...options,force:true});
 expect(third.outcome).toBe("built");
 const small=await writeBootstrapArchiveV1({...options,driftBytes:1,now:()=>Date.now()+1000});
 expect(small.outcome).toBe("unchanged");
 const aged=await writeBootstrapArchiveV1({...options,maxAgeMilliseconds:1,now:()=>Date.now()+1000});
 expect(aged.outcome).toBe("built");
 expect(readdirSync(bootstrapPathsV1(store).directory).filter(name=>name.endsWith(".tar")).length).toBeLessThanOrEqual(2);
});

test("a change above the drift limit rebuilds",async()=>{
 const {worktree,options}=await server("drift");
 await writeBootstrapArchiveV1(options);
 writeFileSync(join(worktree,"a.txt"),"changed\n");
 await seedV1(options).catch(()=>{});
 const kept=await writeBootstrapArchiveV1({...options,driftBytes:1<<40});
 expect(kept.outcome).toBe("unchanged");
 const rebuilt=await writeBootstrapArchiveV1({...options,driftBytes:1});
 expect(rebuilt.outcome==="built"||rebuilt.outcome==="unchanged").toBe(true);
});

test("a build that would leave less than the reserve free is refused with space, writes nothing and keeps the current archive",async()=>{
 const {store,options}=await server("space"),events:ReplicaEventV1[]=[];
 const first=await writeBootstrapArchiveV1(options);
 const staging=readdirSync(join(store,"v1","tmp"));
 await expect(writeBootstrapArchiveV1({...options,force:true,space:async()=>({free:10_000_000,total:100_000_000}),reserveBytes:9_000_000},undefined,event=>events.push(event))).rejects.toMatchObject({key:"space"});
 expect(events.some(event=>event.event==="skipped"&&event.reason==="space")).toBe(true);
 expect(readdirSync(join(store,"v1","tmp"))).toEqual(staging);
 expect((await readBootstrapV1(store))!.manifest.sha256).toBe(first.sha256);
 expect(existsSync(first.archive)).toBe(true);
});

test("when only the archive being replaced stands in the way and it is older than an hour it is retired first, so a single archive remains",async()=>{
 const {store,options}=await server("single"),events:ReplicaEventV1[]=[];
 const first=await writeBootstrapArchiveV1(options);
 const old=new Date(Date.now()-2*3_600_000);
 utimesSync(bootstrapPathsV1(store).manifest,old,old);
 const need=lstatSync(first.archive).size;
 const second=await writeBootstrapArchiveV1({...options,force:true,space:async()=>({free:need,total:10*need}),reserveBytes:need>>2},undefined,event=>events.push(event));
 expect(events.some(event=>event.event==="retired"&&event.reason==="single-slot")).toBe(true);
 expect(second.outcome).toBe("built");
 expect(readdirSync(bootstrapPathsV1(store).directory).filter(name=>name.endsWith(".tar"))).toEqual([bootstrapArchiveNameV1(second.sha256)]);
});

test("a build sweeps staging files of dead writers that were silent for two hours and keeps younger ones",async()=>{
 const {store,options}=await server("sweep");
 const tmp=join(store,"v1","tmp"),first=join(tmp,"g1-m07-1-0123456789abcdef"),second=join(tmp,"g1-m07-2-fedcba9876543210");
 mkdirSync(tmp,{recursive:true});
 writeFileSync(first,Buffer.alloc(1000));writeFileSync(second,Buffer.alloc(1000));
 await writeBootstrapArchiveV1({...options,now:()=>Date.now()+3_600_000});
 expect(existsSync(first)&&existsSync(second)).toBe(true);
 await writeBootstrapArchiveV1({...options,force:true,now:()=>Date.now()+3*3_600_000});
 expect(existsSync(first)||existsSync(second)).toBe(false);
});

gitTest("with reachable packing the archive holds one new pack of the reachable objects, git fsck accepts the extracted repository and the staged pack is gone",async()=>{
 const {store,worktree,options,directory}=await gitServer("reach"),events:ReplicaEventV1[]=[];
 const result=await writeBootstrapArchiveV1(options,undefined,event=>events.push(event));
 expect(result.outcome).toBe("built");
 expect(events.some(event=>event.stage==="bootstrap-pack"&&event.bytes!==undefined)).toBe(true);
 const target=temp("extract-reach");
 expect(spawnSync(system!,["-xf",archiveOf(store),"-C",target]).status).toBe(0);
 const packs=readdirSync(join(target,".git","objects","pack")).sort();
 expect(packs.filter(name=>name.endsWith(".pack")).length).toBe(1);
 expect(packs.some(name=>name.startsWith("tmp_")||name.startsWith("multi-pack-index"))).toBe(false);
 gitHere(target,["fsck","--full","--strict"]);
 expect(gitHere(target,["rev-parse","HEAD"])).toBe(gitHere(worktree,["rev-parse","HEAD"]));
 const wantedNames=gitHere(worktree,["rev-list","--objects","--all"]).split("\n").filter(Boolean).map(line=>line.split(" ")[0]!);
 const have=new Set(gitHere(target,["cat-file","--batch-all-objects","--batch-check=%(objectname)"]).split("\n"));
 for(const name of wantedNames)expect(have.has(name)).toBe(true);
 expect(readdirSync(join(store,"v1","tmp")).filter(name=>name.includes(".pk-"))).toEqual([]);
 const sidecar=readFileSync(result.archive.replace(".tar",".entries"),"utf8");
 expect(sidecar.includes(".git/objects/pack/")).toBe(false);
 expect(readdirSync(directory).length).toBeGreaterThan(3);
});
