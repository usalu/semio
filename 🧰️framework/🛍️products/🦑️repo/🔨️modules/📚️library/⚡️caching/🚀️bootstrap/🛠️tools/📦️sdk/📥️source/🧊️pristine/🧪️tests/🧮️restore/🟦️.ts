/** 🧊️ Contract: restoring the registry package over a snapshot clone leaves exactly the tree an independent Python oracle derives for every case of `./🔣️.json` (registry files replaced, shared binaries kept as hardlinks, everything else pruned, installed nested dependencies untouched), refuses unsafe clones and archives with the authored messages, and rechecks the store at the cadence the model states; the store and the lease are fakes passed explicitly. */
import {afterAll,beforeAll,expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {linkSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {restoreNxPackageSourceV1} from "../../🟦️.ts";
import archivePolicy from "../../../🗃️archive/📋️policy.json";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py"),posix=process.platform!=="win32";
const ask=(request:object):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify(request),encoding:"utf8",maxBuffer:256*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const messages:Record<string,string>={nested:"Registry package cannot own installed nested dependencies",shared:"Exact shared binary registry bytes required",unknown:"Unknown shared package inode refused",shape:"Registry package regular file shape required",namespace:"Bounded Nx package namespace required",ceiling:"Nx package path ceiling exceeded",missing:"ENOENT"};
let base="";
beforeAll(()=>{base=realpathSync(mkdtempSync(join(tmpdir(),"pristine-")));});
afterAll(()=>{rmSync(base,{recursive:true,force:true});});
const expandNodes=(row:(typeof corpus.cases)[number])=>[...row.clone,...(row.generate?Array.from({length:row.generate.count},(_,index)=>({kind:"file" as const,path:row.generate!.prefix+String(index).padStart(4,"0")+".txt",text:"g"})):[])] as any[];
function walk(root:string,path=""):{files:Record<string,{text:string;exec:boolean;links:number}>;dirs:string[]}{
 const files:Record<string,{text:string;exec:boolean;links:number}>={},dirs:string[]=[];
 for(const name of readdirSync(path?join(root,...path.split("/")):root)){
  const relative=path?path+"/"+name:name,full=join(root,...relative.split("/")),info=lstatSync(full);
  if(info.isDirectory()){dirs.push(relative);const inner=walk(root,relative);Object.assign(files,inner.files);dirs.push(...inner.dirs);}
  else files[relative]={text:readFileSync(full,"utf8"),exec:posix&&(info.mode&0o111)!==0,links:info.nlink};
 }
 return{files,dirs:dirs.sort()};
}

test("the corpus is closed by its schema and its limits are the archive policy's",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,cases:[{...corpus.cases[0],clone:[{kind:"socket",path:"x"}]}]})).toBe(false);
 expect(corpus.maximumEntries).toBe(archivePolicy.maximumEntries);expect(corpus.maximumPathBytes).toBe(archivePolicy.maximumPathBytes);
});

test.skipIf(!posix)("every case ends in the tree, the refusal and the recheck cadence the Python oracle derives, and in the authored refusal",async()=>{
 const rootLength=join(base,"c000","store","sdk","stage","node_modules","nx").length,answers=ask({cases:corpus.cases,rootLength,maximumEntries:corpus.maximumEntries,maximumPathBytes:corpus.maximumPathBytes,recheckEvery:corpus.recheckEvery});let restored=0,refused=0,windows=0;
 for(const[index,row]of corpus.cases.entries()){
  const label=row.name,top=join(base,"c"+String(index).padStart(3,"0")),storeRoot=join(top,"store"),directory=join(storeRoot,"sdk","stage"),root=join(directory,"node_modules","nx"),objects=join(top,"objects"),spare=join(top,"spare");
  mkdirSync(root,{recursive:true});mkdirSync(objects,{recursive:true});mkdirSync(spare,{recursive:true});
  for(const node of expandNodes(row)){
   const full=join(root,...node.path.split("/"));mkdirSync(dirname(full),{recursive:true});
   if(node.kind==="dir")mkdirSync(full,{recursive:true});
   else if(node.kind==="symlink")symlinkSync(node.to,full);
   else if(node.shared){const object=join(objects,node.path.split("/").join("_"));writeFileSync(object,node.text);linkSync(object,full);}
   else{writeFileSync(full,node.text,{mode:node.exec?0o755:0o644});for(let link=0;link<(node.extraLinks??0);link++)linkSync(full,join(spare,node.path.split("/").join("_")+link));}
  }
  const state={rechecks:0,publishes:0},store={root:storeRoot,recheck:async()=>{state.rechecks++;}} as never,control={signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>{state.publishes++;},yieldContinuation:async()=>undefined} as never;
  const archive={integrity:"sha512-x",entries:row.archive.map(entry=>({path:entry.path,kind:entry.kind,bytes:new TextEncoder().encode(entry.text),mode:entry.mode}))} as never,binaries={objects:row.shared.map(item=>({path:"node_modules/nx/"+item.path,bytes:Buffer.byteLength(item.text),sha256:createHash("sha256").update(item.text).digest("hex")}))} as never;
  let message="";try{await restoreNxPackageSourceV1(directory,archive,binaries,store,control);}catch(error){message=(error as Error).message+" "+((error as NodeJS.ErrnoException).code??"");}
  const expected=answers[index];
  if(expected.error){
   refused++;expect(message,label).toContain(messages[expected.error]!);if(row.error!==null)expect(message,label).toContain(messages[Object.keys(messages).find(key=>row.error===messages[key]||row.error===key)!]!);
   continue;
  }
  restored++;expect(message,label).toBe("");expect(row.error,label).toBeNull();
  const actual=walk(root);expect(actual.files,label).toEqual(expected.tree);
  const keep=new Set<string>();for(const path of Object.keys(expected.tree)){const parts=path.split("/").slice(0,-1);for(let at=1;at<=parts.length;at++)keep.add(parts.slice(0,at).join("/"));}
  expect(actual.dirs,label).toEqual([...keep].sort());
  expect(state.rechecks,label).toBe(expected.rechecks);expect(state.publishes,label).toBe(expected.publishes);if(expected.publishes>1)windows++;
 }
 expect(restored).toBeGreaterThan(8);expect(refused).toBeGreaterThan(8);expect(windows).toBe(1);
});

test("a restore that finds the store changed stops at the recheck and a cancelled control stops it between files",async()=>{
 const top=join(base,"recheck"),storeRoot=join(top,"store"),directory=join(storeRoot,"sdk","stage"),root=join(directory,"node_modules","nx");mkdirSync(root,{recursive:true});
 for(let index=0;index<70;index++)writeFileSync(join(root,"f"+index),"old");
 const archive={integrity:"x",entries:Array.from({length:70},(_,index)=>({path:"f"+index,kind:"file",bytes:new TextEncoder().encode("new"),mode:420}))} as never,control={signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined} as never;
 await expect(restoreNxPackageSourceV1(directory,archive,{objects:[]} as never,{root:storeRoot,recheck:async()=>{throw Error("Original cache store identity changed");}} as never,control)).rejects.toThrow("identity changed");
 const stopper=new AbortController(),aborting={signal:stopper.signal,remainingMilliseconds:()=>null,publish:async()=>{stopper.abort(Error("stopped restore"));},yieldContinuation:async()=>undefined} as never;
 for(let index=0;index<70;index++)writeFileSync(join(root,"f"+index),"old");
 await expect(restoreNxPackageSourceV1(directory,archive,{objects:[]} as never,{root:storeRoot,recheck:async()=>undefined} as never,aborting)).rejects.toThrow("stopped restore");
});
