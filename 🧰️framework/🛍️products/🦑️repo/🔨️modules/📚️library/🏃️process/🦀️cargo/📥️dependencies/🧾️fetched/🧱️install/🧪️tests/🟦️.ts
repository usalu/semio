import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import fixture from "../../🧪️tests/🔣️.json" with {type:"json"};
import {fetchedArchiveFilesV1} from "../../🟦️.ts";
import {installUnpackedV1} from "../🟦️.ts";

const python=["python3","python"].find(name=>spawnSync(name,["-c","pass"]).status===0);
const archiveSource=`
import io,json,sys,tarfile
files=json.loads(sys.argv[2]);root=sys.argv[1]
with tarfile.open(sys.argv[3],"w:gz",format=tarfile.PAX_FORMAT) as tar:
    for name,text in sorted(files.items()):
        data=text.encode();info=tarfile.TarInfo(root+"/"+name);info.size=len(data);tar.addfile(info,io.BytesIO(data))
`;
const extractSource=`
import sys,tarfile
with tarfile.open(sys.argv[1],"r:gz") as tar:tar.extractall(sys.argv[2])
`;
const marker=new TextEncoder().encode('{"v":1}'),tree=(directory:string)=>Object.fromEntries(readdirSync(directory,{withFileTypes:true,recursive:true}).filter(entry=>entry.isFile()&&!entry.name.includes(".repair-")).map(entry=>[join(entry.parentPath,entry.name).slice(directory.length+1).split("\\").join("/"),readFileSync(join(entry.parentPath,entry.name),"utf8")]));
const tick=()=>new Promise<void>(done=>setImmediate(done));

test("installUnpackedV1: concurrent extractions and repairs never expose a missing or partial package to a reader",async()=>{
 if(!python)return;
 const work=mkdtempSync(join(tmpdir(),"install-race-"));
 try{
  const{name,files}=fixture.sources,crate=join(work,name+".crate"),oracle=join(work,"oracle"),target=join(work,"src","idx",name);
  mkdirSync(oracle);mkdirSync(dirname(target),{recursive:true});
  expect(spawnSync(python,["-I","-c",archiveSource,name,JSON.stringify(files),crate]).status).toBe(0);
  expect(spawnSync(python,["-I","-c",extractSource,crate,oracle]).status).toBe(0);
  const extracted=new Map([...fetchedArchiveFilesV1(new Uint8Array(readFileSync(crate)))!,[".cargo-ok",marker]]),expected={...files,".cargo-ok":'{"v":1}'};
  expect(tree(join(oracle,name))).toEqual(files);
  const violations:string[]=[],watch=()=>{let done=false;const seen=new Set<string>(),task=(async()=>{let existed=false;while(!done){
   const present=existsSync(target);if(existed&&!present)violations.push("directory vanished");existed||=present;
   if(present)for(const path of Object.keys(expected)){const here=existsSync(join(target,path));if(here)seen.add(path);else if(seen.has(path))violations.push("file vanished: "+path);}
   if(existsSync(join(target,".cargo-ok"))&&readFileSync(join(target,".cargo-ok"),"utf8")==='{"v":1}')for(const[path,text]of Object.entries(files))if(!existsSync(join(target,path))||readFileSync(join(target,path),"utf8")!==text)violations.push("marked complete but incomplete: "+path);
   await tick();}})();return async()=>{done=true;await task;};};
  const cargo=async()=>{mkdirSync(target,{recursive:true});writeFileSync(join(target,".cargo-ok"),"");for(const[path,text]of Object.entries(files)){mkdirSync(dirname(join(target,path)),{recursive:true});writeFileSync(join(target,path),text);await tick();}writeFileSync(join(target,".cargo-ok"),marker);};
  let stop=watch();
  await Promise.all([cargo(),...Array.from({length:6},()=>installUnpackedV1(target,extracted))]);
  await stop();
  writeFileSync(join(target,"Cargo.toml"),"tampered");rmSync(join(target,"src","lib.rs"));rmSync(join(target,".cargo-ok"));
  stop=watch();
  await Promise.all(Array.from({length:6},()=>installUnpackedV1(target,extracted)));
  await stop();
  expect(violations).toEqual([]);
  expect(tree(target)).toEqual(expected);
  expect(readdirSync(dirname(target)).filter(entry=>entry.includes(".repair-"))).toEqual([]);
 }finally{rmSync(work,{recursive:true,force:true});}
});
