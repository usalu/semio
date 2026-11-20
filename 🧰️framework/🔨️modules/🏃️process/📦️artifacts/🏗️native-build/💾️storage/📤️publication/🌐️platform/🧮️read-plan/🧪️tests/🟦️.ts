import {expect,test} from "bun:test";
import Ajv from "ajv";
import {createHash} from "node:crypto";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {spawnSync} from "node:child_process";
import {lstatSync,mkdirSync,mkdtempSync,readFileSync,rmSync,symlinkSync,writeFileSync,renameSync} from "node:fs";
import {freemem,tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {protectedArtifactInputChunkV1,readProtectedArtifactInputV1,protectedArtifactInputHashedV1,protectedArtifactInputWorkAllowanceV1,visitProtectedArtifactInputDirectoriesV1,type ProtectedArtifactInputHashModeV1} from "../🟦️.ts";
import {artifactDescriptorCustodyV1,withArtifactRelativeDirectoryV1,type ArtifactRelativeDirectoryV1,type ArtifactRelativePathEntryV1,type ArtifactReadFileV1} from "@semio-tech/framework-filesystem/observation/descriptor";
import fixture from "./🔣️.json" with {type:"json"};

const row=(c:typeof fixture.decisions[number])=>protectedArtifactInputHashedV1(c.mode as ProtectedArtifactInputHashModeV1,{root:c.fileRoot},(c as {rootIndex?:number}).rootIndex,{mtimeNs:BigInt(c.mtimeNs),ctimeNs:BigInt(c.ctimeNs)},(c as {recordedAtNs?:string}).recordedAtNs===undefined?undefined:BigInt((c as {recordedAtNs?:string}).recordedAtNs!));

test("a read hashes exactly the files the table names",()=>{for(const c of fixture.decisions)expect(row(c),c.name).toBe(c.hashed);});

test("the independent Python oracle decides the same table",()=>{
 const run=spawnSync("python3",["-I",join(import.meta.dir,"oracle.py"),join(import.meta.dir,"🔣️.json")],{encoding:"utf8"});
 expect(run.status).toBe(0);expect(JSON.parse(run.stdout)).toEqual(fixture.decisions.map(row));
});

test("a probe over many roots hashes a few passes of the group, not one per root",()=>{
 for(const c of fixture.counts){
  let hashed=0;const recorded=BigInt(900_000_000_000_000);
  for(const pass of c.passes){
   const[mode,times]=pass.split(" x"),repeat=times===undefined?1:Number(times);
   for(let at=0;at<repeat;at++)for(let file=0;file<c.files;file++){
    const root=file%c.roots;
    if(protectedArtifactInputHashedV1(mode as ProtectedArtifactInputHashModeV1,{root},mode==="root"?at:undefined,{mtimeNs:1n,ctimeNs:1n},recorded))hashed++;
   }
  }
  expect(hashed,c.name).toBe(c.hashed);
  const before=c.files*(3+c.roots*2+2);expect(hashed).toBeLessThan(before);
 }
});

test("each requested directory is visited once and a replaced directory is refused",async()=>{
 const base=mkdtempSync(join(tmpdir(),"r99-"));
 try{
  const wanted:string[]=[];
  for(const scope of["aa","bb"])for(let member=0;member<3;member++){const dir=join(base,"g",scope,"w",String(member));mkdirSync(dir,{recursive:true});writeFileSync(join(dir,"package.json"),"{}");wanted.push(`g/${scope}/w/${member}`);}
  wanted.push("g/aa","g/bb");
  const seen:string[]=[],names=new Map<string,readonly string[]>();
  await withArtifactRelativeDirectoryV1(base,()=>{},async root=>{await visitProtectedArtifactInputDirectoriesV1(root,[...wanted,...wanted],async(path,directory)=>{seen.push(path);names.set(path,await directory.names());});});
  expect([...seen].sort()).toEqual([...new Set(wanted)].sort());
  expect(names.get("g/aa/w/1")).toEqual(["package.json"]);expect(names.get("g/bb")).toEqual(["w"]);
  const other=join(base,"elsewhere");mkdirSync(other);renameSync(join(base,"g","aa"),join(base,"aa-moved"));symlinkSync(other,join(base,"g","aa"));
  const oracle=spawnSync("python3",["-I",join(import.meta.dir,"oracle.py"),join(import.meta.dir,"🔣️.json"),"directories",base],{encoding:"utf8"});
  expect(oracle.status).toBe(0);expect(JSON.parse(oracle.stdout)).toEqual(fixture.refusedDirectories);
  for(const row of fixture.refusedDirectories){
   await expect(withArtifactRelativeDirectoryV1(base,()=>{},root=>visitProtectedArtifactInputDirectoriesV1(root,[row.path],async()=>{}))).rejects.toThrow(
    `Actual same-device immutable input directory required at ${JSON.stringify(row.component)} (observed ${row.observed})`
   );
  }
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("the platform reader walks each directory once and hashes by mode",()=>{
 const text=readFileSync(join(import.meta.dir,"..","..","🟦️.ts"),"utf8"),start=text.indexOf("async function verifyProtectedArtifactInputFilesV1("),end=text.indexOf("let nativeArtifactInputStepsV1=0;");
 expect(start).toBeGreaterThan(0);expect(end).toBeGreaterThan(start);
 const body=text.slice(start,end);
 expect(body).toContain("visitProtectedArtifactInputDirectoriesV1(root,groups.keys()");expect(body).not.toContain("withProtectedArtifactInputDirectoryV1(");expect(body).toContain("protectedArtifactInputHashedV1(mode,input,rootIndex,entry,recordedAtNs)");
 const capture=text.slice(text.indexOf("async function recordProtectedArtifactExistingInputGroupV1("),text.indexOf("export function stageNativePlatformProtectedArtifactInputGroupV1("));
 const observe=text.slice(text.indexOf("export function observeNativePlatformProtectedArtifactInputGroupV1("),text.indexOf("export function measureNativePlatformProtectedArtifactFundingV1("));
 expect(capture).toContain('verifyProtectedArtifactInputFilesV1(root,manifest,directories,files,check,control,false,false,undefined,"all")');
 expect(observe.indexOf("await recordProtectedArtifactExistingInputGroupV1(")).toBeGreaterThan(0);
 expect(observe.indexOf("await verifyProtectedArtifactInputFilesV1(")).toBeGreaterThan(observe.indexOf("await recordProtectedArtifactExistingInputGroupV1("));
 expect(text).toContain('from"./🧮️read-plan/🟦️.ts"');
});

test("the work allowance of a group is derived from its size and equals the independent Python model",()=>{
 const mine=fixture.allowances.map(c=>protectedArtifactInputWorkAllowanceV1(c,c.chunkBytes));
 expect(mine).toEqual(fixture.allowances.map(c=>c.allowance));
 const run=spawnSync("python3",["-I",join(import.meta.dir,"oracle.py"),join(import.meta.dir,"🔣️.json"),"allowance"],{encoding:"utf8"});
 expect(run.status).toBe(0);expect(JSON.parse(run.stdout)).toEqual(mine);
 const small=fixture.allowances[1]!,doubled={...small,files:[...small.files,...small.files]};
 expect(protectedArtifactInputWorkAllowanceV1(doubled,doubled.chunkBytes)).toBeGreaterThan(protectedArtifactInputWorkAllowanceV1(small,small.chunkBytes));
});

test("a Windows-shaped staging of 2584 chunk steps, which the fixed cap of 65536 refused after 11259 s, is inside its allowance",()=>{
 const w=fixture.windows,roots=["C:/Users/semio/AppData/Local/semio/tools/win32-x64"],files=Array.from({length:64},(_,index)=>({path:"Bin/Sub/Dir/Tool"+index+".DLL",bytes:40*49152}));
 const needed=w.chunkSteps*w.checkpointsPerChunkStep,allowance=protectedArtifactInputWorkAllowanceV1({roots,files},65536);
 expect(w.oldFixedCap).toBeLessThan(needed);
 expect(allowance).toBeGreaterThanOrEqual(needed);
});

test("the platform no longer counts staging, reading or observing against a fixed step cap",()=>{
 const text=readFileSync(join(import.meta.dir,"..","..","🟦️.ts"),"utf8");
 for(const message of["Immutable input physical work capacity exceeded","Immutable input read work capacity exceeded","Current immutable input observation work exceeded"]){
  const spots=text.split(message).slice(0,-1);
  expect(spots.length,message).toBeGreaterThan(0);
  for(const before of spots)expect(before.slice(-120),message).not.toContain("maximumWork");
 }
 expect(text).toContain("protectedArtifactInputWorkAllowanceV1(");
});

test("ordinary input read authenticates one retained directory traversal and every file",async()=>{
 const ts=await import("typescript"),layout=fixture.ordinaryRead,validate=new Ajv({strict:true}).compile(schema.definitions.OrdinaryReadFixture);
 expect(validate(layout)).toBe(true);
 const source=readFileSync(join(import.meta.dir,"..","..","🟦️.ts"),"utf8"),tree=ts.createSourceFile("owned-input-read.ts",source,ts.ScriptTarget.Latest,true);
 const names=["protectedArtifactInputAnchorV1","protectedArtifactInputStampV1","verifyProtectedArtifactInputFilesV1"],functions=names.map(name=>tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text===name)!);
 const code=ts.transpileModule(functions.map(node=>node.getText(tree)).join("\n")+";return verifyProtectedArtifactInputFilesV1;",{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
 for(const row of layout.cases){
  const base=mkdtempSync(join(tmpdir(),"ir-"));
  try{
   const directories=new Map<string,string>(),files=new Map<string,string>(),inputs:{root:number;path:string;bytes:number;sha256:string}[]=[],recorded=new WeakMap<object,bigint>();
   let newest=0n,opened=0,read=0,rosters=0;
   for(const [index,root] of layout.roots.entries())for(const name of layout.files){
    const full=root+"/"+name,parts=full.split("/");
    mkdirSync(join(base,...parts.slice(0,-1)),{recursive:true});
    const bytes=Buffer.from(full);
    writeFileSync(join(base,...parts),bytes);
    const file=lstatSync(join(base,...parts),{bigint:true});
    const stamp=[file.dev,file.ino,file.mode,file.nlink,file.size,file.mtimeNs,file.ctimeNs].join(":");
    const latest=file.mtimeNs>file.ctimeNs?file.mtimeNs:file.ctimeNs;if(latest>newest)newest=latest;
    files.set(full,stamp);inputs.push({root:index,path:name,bytes:bytes.length,sha256:createHash("sha256").update(bytes).digest("hex")});
    for(let count=1;count<parts.length;count++){
     const path=parts.slice(0,count).join("/"),dir=lstatSync(join(base,path),{bigint:true});
     directories.set(path,[dir.dev,dir.ino,dir.mode].join(":"));
    }
   }
   recorded.set(directories,row.hash?0n:newest+10000000000n);
   const physical=[...files.keys()][0]!,full=join(base,physical);
   if(row.change==="file"){
    renameSync(full,full+".prior");writeFileSync(full,physical);
   }
   if(row.change==="directory"){
    const parent=dirname(full);
    renameSync(parent,parent+".prior");symlinkSync(parent+".prior",parent,"junction");
   }
   const stop=new AbortController(),check=()=>stop.signal.throwIfAborted(),run=new Function("nativeProtectedArtifactInputProtocolV1","protectedArtifactInputChunkV1","freemem","protectedArtifactInputRecordedV1","visitProtectedArtifactInputDirectoriesV1","protectedArtifactInputHashedV1","artifactRetirementHashV1","readProtectedArtifactInputV1","nativeArtifactInputStepV1",code)(()=>({}),protectedArtifactInputChunkV1,freemem,recorded,visitProtectedArtifactInputDirectoriesV1,protectedArtifactInputHashedV1,createHash,readProtectedArtifactInputV1,()=>1);
   const tracked=(directory:ArtifactRelativeDirectoryV1):ArtifactRelativeDirectoryV1=>Object.freeze({...directory,
    stat:()=>directory.stat(),entry:(name:string)=>directory.entry(name),
    withDirectoryPath:async<T>(entries:readonly ArtifactRelativePathEntryV1[],body:(opened:ArtifactRelativeDirectoryV1)=>Promise<T>)=>directory.withDirectoryPath(entries,openedDirectory=>{opened++;return body(tracked(openedDirectory));}),
    withFile:async<T>(name:string,body:(file:ArtifactReadFileV1)=>Promise<T>)=>directory.withFile(name,file=>{read++;return body(file);}),
    names:()=>{rosters++;return directory.names();},
   });
   const before=artifactDescriptorCustodyV1().active;
   let error:unknown;
   try{
    await withArtifactRelativeDirectoryV1(base,check,async root=>{
     if(row.change==="cancel")stop.abort(new Error("original input read cancellation"));
     await run(tracked(root),{roots:layout.roots,files:inputs},directories,files,check,{publish:async()=>{},yieldContinuation:async()=>{}},false);
    });
   }catch(failure){error=failure;}
   expect(error===undefined,row.change+" "+String(error)).toBe(row.accepted);
   expect(artifactDescriptorCustodyV1().active).toBe(before);
   if(row.change==="cancel")expect(error).toBe(stop.signal.reason);
   if(row.accepted){
    const oracle=spawnSync("python3",["-I",join(import.meta.dir,"oracle.py"),join(import.meta.dir,"🔣️.json"),"ordinary",base],{encoding:"utf8"});
    expect(oracle.status).toBe(0);
    expect(opened).toBe(JSON.parse(oracle.stdout).directories);
    expect(read).toBe(row.hash?JSON.parse(oracle.stdout).files:0);
    expect(rosters).toBe(0);
   }
  }finally{rmSync(base,{recursive:true,force:true});}
 }
});
