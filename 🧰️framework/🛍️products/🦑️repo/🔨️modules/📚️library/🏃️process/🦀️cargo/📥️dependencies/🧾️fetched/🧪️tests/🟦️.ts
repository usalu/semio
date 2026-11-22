import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import fixture from "./🔣️.json" with {type:"json"};
import {captureFetchedV1,classifyFetchedLockV1,fetchedArchiveFilesV1,fetchedHoldsV1,fetchedLockedV1,fetchedSourcesV1,fetchedLockPathV1,fetchedPackagesV1,readFetchedReceiptV1,type FetchedPortV1,type FetchedReceiptV1} from "../🟦️.ts";
import {installUnpackedV1} from "../🧱️install/🟦️.ts";

const request="a".repeat(64),pause=()=>new Promise<void>(done=>setTimeout(done,15));
const portOf=(workspace:string,home:string):FetchedPortV1=>({
 read:async path=>{try{return new Uint8Array(readFileSync(join(workspace,path)));}catch{return undefined;}},
 digest:async path=>{try{return createHash("sha256").update(readFileSync(path)).digest("hex");}catch{return undefined;}},
 archive:async path=>{try{return new Uint8Array(readFileSync(path));}catch{return undefined;}},
 unpacked:async path=>{try{const rows=new Map<string,string>();for(const entry of readdirSync(path,{withFileTypes:true,recursive:true})){if(!entry.isFile())continue;const full=join(entry.parentPath,entry.name);rows.set(full.slice(path.length+1).split("\\").join("/"),createHash("sha256").update(readFileSync(full)).digest("hex"));}return rows;}catch{return undefined;}},
 restore:(path,files)=>installUnpackedV1(path,files),
 list:path=>{try{return readdirSync(path,{withFileTypes:true}).filter(entry=>entry.isDirectory()).map(entry=>entry.name);}catch{return undefined;}},
});
const lockOf=(name:string)=>(fixture.locks as Record<string,string>)[name]!;

for(const[name,expected]of Object.entries(fixture.classes))test("fetched lock classification: "+name,()=>{expect(classifyFetchedLockV1(lockOf(name))).toBe(expected as "registry"|"unsupported");});

for(const row of fixture.owners)test("fetched lock owner: "+row.name,async()=>{
 const workspace=mkdtempSync(join(tmpdir(),"fetched-owner-"));
 try{for(const[path,text]of Object.entries(row.files)){mkdirSync(dirname(join(workspace,path)),{recursive:true});writeFileSync(join(workspace,path),text);}expect(await fetchedLockPathV1(row.manifest,portOf(workspace,workspace).read)).toBe(row.lock);}finally{rmSync(workspace,{recursive:true,force:true});}
});

const python=["python3","python"].find(name=>spawnSync(name,["-c","pass"]).status===0);
const oracleSource=`
import hashlib,json,os,re,sys
workspace,home,receipt=sys.argv[1],sys.argv[2],json.loads(sys.argv[3])
locks=[];rows={}
for l in receipt["locks"]:
    data=open(os.path.join(workspace,l["path"]),"rb").read();locks.append({"path":l["path"],"digest":hashlib.sha256(data).hexdigest()})
    for block in data.decode().split("[[package]]")[1:]:
        if re.search(r"^source = ",block,re.M):
            name=re.search(r'^name = "([^"]+)"',block,re.M).group(1)+"-"+re.search(r'^version = "([^"]+)"',block,re.M).group(1)
            checksum=re.search(r'^checksum = "([^"]+)"',block,re.M).group(1);rows[name+" "+checksum]=[name,checksum]
cache=os.path.join(home,"registry","cache");held=[]
for key in sorted(rows):
    name,checksum=rows[key]
    if any(hashlib.sha256(open(os.path.join(cache,i,name+".crate"),"rb").read()).hexdigest()==checksum for i in os.listdir(cache) if os.path.exists(os.path.join(cache,i,name+".crate"))):held.append(rows[key])
print(json.dumps({"locks":locks,"registry":{"digest":hashlib.sha256(json.dumps(held,separators=(",",":")).encode()).hexdigest(),"count":len(held)}}))
`;

test("fetched receipt: follows the lockfile and the locked archive content through a mutation sequence, and ignores touch, recreation, index moves and every other package",async()=>{
 const workspace=mkdtempSync(join(tmpdir(),"fetched-ws-")),home=mkdtempSync(join(tmpdir(),"fetched-home-"));
 try{
  mkdirSync(join(workspace,"app"),{recursive:true});writeFileSync(join(workspace,"Cargo.toml"),"[workspace]\nmembers = [\"app\"]\n");writeFileSync(join(workspace,"app","Cargo.toml"),"[package]\nname = \"app\"\nversion = \"0.1.0\"\n");
  for(const index of["idx","idx2"]){mkdirSync(join(home,"registry","cache",index),{recursive:true});mkdirSync(join(home,"registry","src",index),{recursive:true});}
  const port=portOf(workspace,home);let receipt:FetchedReceiptV1|undefined,lock="registry";writeFileSync(join(workspace,"Cargo.lock"),lockOf(lock));
  for(const step of fixture.sequence as Array<{op:string;lock?:string;defined?:boolean;expect?:boolean;name?:string;index?:string;to?:string}>){
   await pause();
   const crate=()=>join(home,"registry","cache",step.index??"idx",step.name!);
   if(step.op==="writeLock"){lock=step.lock!;writeFileSync(join(workspace,"Cargo.lock"),lockOf(lock));}
   else if(step.op==="addCrate")writeFileSync(crate(),"crate");
   else if(step.op==="corruptCrate")writeFileSync(crate(),"crate again");
   else if(step.op==="touchCrate")writeFileSync(crate(),"crate");
   else if(step.op==="recreateCrate"){rmSync(crate(),{force:true});writeFileSync(crate(),"crate");}
   else if(step.op==="moveCrate")renameSync(crate(),join(home,"registry","cache",step.to!,step.name!));
   else if(step.op==="removeCrate")rmSync(crate());
   else if(step.op==="addSource")mkdirSync(join(home,"registry","src","idx",step.name!));
   else if(step.op==="capture"){writeFileSync(join(workspace,"Cargo.lock"),lockOf(step.lock!));const got=await captureFetchedV1({request,manifests:["app/Cargo.toml"],home},port);expect(got!==undefined).toBe(step.defined!);if(got){receipt=got;lock=step.lock!;if(python){const independent=JSON.parse(spawnSync(python,["-I","-c",oracleSource,workspace,home,JSON.stringify(got)],{encoding:"utf8"}).stdout);expect(independent.locks.map((row:{digest:string})=>row.digest)).toEqual(got.locks.map(row=>row.digest));expect(independent.registry).toEqual(got.registry);}}}
   else if(step.op==="holds")expect(await fetchedHoldsV1(receipt!,home,port)).toBe(step.expect!);
  }
 }finally{rmSync(workspace,{recursive:true,force:true});rmSync(home,{recursive:true,force:true});}
});

test("fetched packages: names and versions of registry packages only",()=>{
 expect(fetchedPackagesV1(lockOf("registry"))).toEqual(["serde-1.0.0"]);expect(fetchedPackagesV1(lockOf("path"))).toEqual([]);expect(fetchedPackagesV1(lockOf("broken"))).toBeUndefined();
});

for(const row of fixture.tamper)test("fetched receipt admission: "+row.name,async()=>{
 const workspace=mkdtempSync(join(tmpdir(),"fetched-adm-")),home=mkdtempSync(join(tmpdir(),"fetched-adm-home-"));
 try{
  mkdirSync(join(home,"registry","cache","idx"),{recursive:true});writeFileSync(join(home,"registry","cache","idx","serde-1.0.0.crate"),"crate");writeFileSync(join(workspace,"Cargo.toml"),"[workspace]\n");writeFileSync(join(workspace,"Cargo.lock"),lockOf("registry"));
  const receipt=await captureFetchedV1({request,manifests:["Cargo.toml"],home},portOf(workspace,home));expect(receipt).toBeDefined();
  const text=JSON.stringify(row.mutate==="extra"?{...receipt,extra:true}:receipt);
  expect(readFetchedReceiptV1(text,row.mutate==="request"?"b".repeat(64):request)!==undefined).toBe(row.admitted);
 }finally{rmSync(workspace,{recursive:true,force:true});rmSync(home,{recursive:true,force:true});}
});

const archiveSource=`
import io,json,sys,tarfile
files=json.loads(sys.argv[2]);root=sys.argv[1]
with tarfile.open(sys.argv[3],"w:gz",format=tarfile.PAX_FORMAT) as tar:
    for name,text in sorted(files.items()):
        data=text.encode();info=tarfile.TarInfo(root+"/"+name);info.size=len(data);tar.addfile(info,io.BytesIO(data))
`;

test("fetched sources: a missing, partial or altered extracted registry source is re-extracted from the checksum-verified archive",async()=>{
 if(!python)return;
 const home=mkdtempSync(join(tmpdir(),"fetched-src-home-")),work=mkdtempSync(join(tmpdir(),"fetched-src-work-"));
 try{
  const{name,files}=fixture.sources,crate=join(home,"registry","cache","idx",name+".crate");mkdirSync(dirname(crate),{recursive:true});
  expect(spawnSync(python,["-I","-c",archiveSource,name,JSON.stringify(files),crate]).status).toBe(0);
  const bytes=readFileSync(crate),checksum=createHash("sha256").update(bytes).digest("hex"),extracted=fetchedArchiveFilesV1(new Uint8Array(bytes))!;
  expect([...extracted.keys()].sort()).toEqual(Object.keys(files).sort());
  for(const[path,text]of Object.entries(files))expect(Buffer.from(extracted.get(path)!).toString("utf8")).toBe(text);
  const locked=fetchedLockedV1(lockOf("registry").replace(fixture.checksum,checksum))!,port=portOf(work,home),source=join(home,"registry","src","idx",name);
  expect(locked).toEqual([["serde-1.0.0",checksum]]);
  const sound=()=>fetchedSourcesV1(home,port,locked,false),repair=()=>fetchedSourcesV1(home,port,locked,true);
  expect(await sound()).toBe(false);
  expect(await repair()).toBe(true);expect(await sound()).toBe(true);expect(existsSync(join(source,".cargo-ok"))).toBe(true);
  for(const[path,text]of Object.entries(files))expect(readFileSync(join(source,path),"utf8")).toBe(text);
  rmSync(join(source,"src","lib.rs"));expect(await sound()).toBe(false);expect(await repair()).toBe(true);expect(readFileSync(join(source,"src","lib.rs"),"utf8")).toBe(files["src/lib.rs"]);
  writeFileSync(join(source,"Cargo.toml"),"tampered");expect(await sound()).toBe(false);expect(await repair()).toBe(true);expect(readFileSync(join(source,"Cargo.toml"),"utf8")).toBe(files["Cargo.toml"]);
  rmSync(join(source,".cargo-ok"));expect(await sound()).toBe(false);expect(await repair()).toBe(true);expect(await sound()).toBe(true);
  writeFileSync(join(source,"extra.txt"),"tolerated");expect(await sound()).toBe(true);
  writeFileSync(crate,"corrupt");expect(await repair()).toBe(false);
 }finally{rmSync(home,{recursive:true,force:true});rmSync(work,{recursive:true,force:true});}
});
