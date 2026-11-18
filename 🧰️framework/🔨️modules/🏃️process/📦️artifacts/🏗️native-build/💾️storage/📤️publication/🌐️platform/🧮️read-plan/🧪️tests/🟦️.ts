import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,symlinkSync,writeFileSync,renameSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {protectedArtifactInputHashedV1,protectedArtifactInputWorkAllowanceV1,visitProtectedArtifactInputDirectoriesV1,type ProtectedArtifactInputHashModeV1} from "../🟦️.ts";
import {withArtifactRelativeDirectoryV1} from "@semio-tech/framework-filesystem/observation/descriptor";
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
  let refused="";
  try{await withArtifactRelativeDirectoryV1(base,()=>{},async root=>{await visitProtectedArtifactInputDirectoriesV1(root,["g/aa/w/0"],async()=>{});});}catch(error){refused=(error as Error).message;}
  expect(refused).toContain("same-device immutable input directory");
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("the platform reader walks each directory once and hashes by mode",()=>{
 const text=readFileSync(join(import.meta.dir,"..","..","🟦️.ts"),"utf8"),start=text.indexOf("async function verifyProtectedArtifactInputFilesV1("),end=text.indexOf("let nativeArtifactInputStepsV1=0;");
 expect(start).toBeGreaterThan(0);expect(end).toBeGreaterThan(start);
 const body=text.slice(start,end);
 expect(body).toContain("visitProtectedArtifactInputDirectoriesV1(root,groups.keys()");expect(body).not.toContain("withProtectedArtifactInputDirectoryV1(");expect(body).toContain("protectedArtifactInputHashedV1(mode,input,rootIndex,entry,recordedAtNs)");
 expect(text).toContain('false,false,undefined,"all");await verifyProtectedArtifactInputFilesV1(root,manifest,directories,files,check,actual,false);');
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
