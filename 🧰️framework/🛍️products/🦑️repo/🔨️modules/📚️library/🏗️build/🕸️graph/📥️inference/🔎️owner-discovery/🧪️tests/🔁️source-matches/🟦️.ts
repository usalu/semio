import {expect,test} from "bun:test";
import * as fs from "node:fs";
import {dirname,join,posix} from "node:path";
import Ajv from "ajv";
import picomatch from "picomatch";
import {loadProductFunctionsSyncV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧩️extracted-function/🟦️.ts";
import {testPythonV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {testScratchRootV1} from "../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🟦️.ts";
import {createBuildInputPathMatcherV1} from "../../../../../📥️inputs/🟨️.mjs";
import {INFERENCE_LIMITS_V1} from "../../../🧬️schema/🟨️.mjs";
import fixture from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

function sourceMatcher(ports: Record<string,unknown>){
 return loadProductFunctionsSyncV1(new URL("../../🟨️.mjs",import.meta.url),["readBuildSourceInputMatchesV1"],{INFERENCE_LIMITS_V1,...ports}).readBuildSourceInputMatchesV1;
}

test("identical source glob questions retain each target's matches and avoid repeated candidate filtering",async()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const root=fs.mkdtempSync(join(testScratchRootV1("selected-source-matches"),"match-"));
 const scans=Array.from({length:fixture.scanCopies},(_,index)=>({target:"target-"+index,compile:index>0,patterns:fixture.patterns}));
 scans.push({target:"alternate",compile:true,patterns:[fixture.patterns[0]!,"!tree/keep/**/*.ts"]},{target:"shallow",compile:true,patterns:[fixture.shallow,...fixture.absent]});
 const checkpoints:number[]=[],counts=new Map<string,number>(),literal=join(root,"tree","literal.txt");
 let literalProbes=0,directoryExistenceProbes=0,directoryReads=0,yields=0,abortAt=Infinity,opaqueCalls=0;
 const abort=new AbortController(),reason=new Error("cancel during selected Source matching");
 const execute=sourceMatcher({
  join,dirname,buildPath:(path:string)=>path.replaceAll("\\","/"),POLICY:{generatedDirectories:[]},isOpaqueSourcePathV1:()=>{opaqueCalls++;return false;},EXAMPLE_COLLECTIONS:[],WORKSPACE_OWNERSHIP:{collections:{moduleMember:"member"}},lstatSync:fs.lstatSync,readdirSync:(path:string,options:{withFileTypes:true})=>{directoryReads++;return fs.readdirSync(path,options);},existsSync:(path:string)=>{if(path===literal)literalProbes++;else directoryExistenceProbes++;return fs.existsSync(path);},
  yieldInferenceV1:async(_context:unknown,_phase:string,index:number)=>{checkpoints.push(index);if(++yields===abortAt)abort.abort(reason);abort.signal.throwIfAborted();},
  loadedProjectInputs:{createBuildInputPathMatcherV1:(pattern:string)=>{const match=createBuildInputPathMatcherV1(pattern);return(path:string)=>{counts.set(pattern,(counts.get(pattern)??0)+1);return match(path);};}},
 });
 try{
  for(const folder of fixture.folders){
   const directory=join(root,"tree",folder,"files");fs.mkdirSync(directory,{recursive:true});
   for(let index=0;index<fixture.filesPerKind;index++)for(const extension of ["ts","json"])fs.writeFileSync(join(directory,"item-"+index+"."+extension),"");
  }
  fs.writeFileSync(literal,"");
  const python=testPythonV1(),oracle=Bun.spawnSync([python!,"-I",join(import.meta.dir,"🐍️.py")],{stdin:Buffer.from(JSON.stringify({root,scans}))});
  expect(oracle.exitCode,oracle.stderr.toString()).toBe(0);
  const result=await execute(scans,root,{}),sorted=(rows:Set<string>)=>[...rows].sort();
  expect({matched:sorted(result.matched),compileMatched:sorted(result.compileMatched),compileByTarget:Object.fromEntries([...result.compileByTarget].map(([target,rows])=>[target,sorted(rows)]))}).toEqual(JSON.parse(oracle.stdout.toString()));
  expect([...result.compileByTarget.keys()]).toEqual(scans.filter(scan=>scan.compile).map(scan=>scan.target));
  expect([...result.compileByTarget.get("target-1")]).toEqual([...result.compileByTarget.get("target-2")]);
  expect(counts.get(fixture.shallow)).toBe(result.compileByTarget.get("shallow").size);
   expect(directoryExistenceProbes).toBe(0);
   expect(literalProbes).toBe(1);
  expect(directoryReads).toBe(1+fixture.folders.length*2);
  expect(yields).toBeLessThanOrEqual(fixture.maximumCheckpoints);
  expect(Math.max(...checkpoints.map((index,at)=>index-(checkpoints[at-1]??0)))).toBeLessThanOrEqual(INFERENCE_LIMITS_V1.selectedSourceCheckpointEntries);
  const candidates=fixture.folders.length*fixture.filesPerKind*2+1;
  expect(counts.get(fixture.patterns[0]!)).toBe(candidates*2);
  expect(opaqueCalls).toBe(candidates+fixture.folders.length*2+fixture.absent.length+2);
  abortAt=yields+Math.floor(yields/2);
  await expect(execute(scans,root,{})).rejects.toBe(reason);
 }finally{fs.rmSync(root,{recursive:true,force:true});}
});

test("a recursive source walk shares visited nested roots in DFS order",async()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const {directories,symlink,opaque,scans,reads}=fixture.nested;
 const entries=new Map<string,string[]>(Object.entries(directories)),actualReads:string[]=[],typeReads:string[]=[],matchedCandidates=new Map<string,string[]>();
 const workspace="/fixture",local=(path:string)=>posix.relative(workspace,path);
 const execute=sourceMatcher({
  join:posix.join,dirname:posix.dirname,buildPath:(path:string)=>path,POLICY:{generatedDirectories:[]},EXAMPLE_COLLECTIONS:[],WORKSPACE_OWNERSHIP:{collections:{moduleMember:"member"}},
  isOpaqueSourcePathV1:(path:string)=>path===opaque||path.startsWith(opaque+"/"),existsSync:()=>false,
  lstatSync:(absolute:string)=>{const path=local(absolute);typeReads.push(path);return{isDirectory:()=>entries.has(path)};},
  readdirSync:(absolute:string)=>{
   const path=local(absolute);actualReads.push(path);
   return entries.get(path)!.map(name=>{
    const child=posix.join(path,name);
    return{name,isSymbolicLink:()=>child===symlink,isDirectory:()=>entries.has(child),isFile:()=>!entries.has(child)&&child!==symlink};
   });
  },
  yieldInferenceV1:async()=>{},loadedProjectInputs:{createBuildInputPathMatcherV1:(pattern:string)=>{
   const matches=createBuildInputPathMatcherV1(pattern),candidates:string[]=[];
   matchedCandidates.set(pattern,candidates);
   return(path:string)=>{candidates.push(path);return matches(path);};
  }},
 });
 const expected=scans.map(({patterns,candidates})=>candidates.filter(path=>
  patterns.some(pattern=>!pattern.startsWith("!")&&picomatch.isMatch(path,pattern))&&
  !patterns.some(pattern=>pattern.startsWith("!")&&picomatch.isMatch(path,pattern.slice(1)))));
 const unique=(paths:string[])=>[...new Set(paths)],compiled=expected.filter((paths,index)=>paths.length&&scans[index]!.compile);
 const actual=await execute(scans,workspace,{});
 expect([...actual.matched]).toEqual(unique(expected.flat()));
 expect([...actual.compileMatched]).toEqual(unique(compiled.flat()));
 expect([...actual.compileByTarget].map(([target,paths])=>[target,[...paths]])).toEqual(scans.flatMap(({target,compile},index)=>compile&&expected[index]!.length?[[target,expected[index]]]:[]));
 expect(actualReads).toEqual(reads);
 expect(typeReads).toEqual([...reads,symlink]);
 for(const scan of scans.filter(({target})=>target.startsWith("shallow-")))expect(matchedCandidates.get(scan.patterns[0]!)).toEqual(scan.candidates);
 actualReads.length=0;
 await execute([scans[1]!,scans[0]!],workspace,{});
 expect(actualReads.slice(0,5)).toEqual(["tree/lib","tree/lib/vacant","tree/lib/inner","tree/lib/inner/down","tree"]);
 entries.get("tree/lib")!.push("added.ts");
 const next=await execute(scans,workspace,{});
 expect([...next.compileByTarget.get("nested")]).toContain("tree/lib/added.ts");
});

test("directory-dense source walks use the same bounded checkpoint cadence as file matches",async()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const {pattern,directories,maximumCheckpoints}=fixture.directoryCadence;
 const names=Array.from({length:directories},(_,index)=>"dir-"+index),checkpoints:number[]=[];
 const paths=names.map(name=>`tree/${name}/schema.json`);
 const matches=createBuildInputPathMatcherV1(pattern),workspace="/fixture",tree=posix.join(workspace,"tree");
 let reads=0;
 const execute=sourceMatcher({
  join:posix.join,dirname:posix.dirname,buildPath:(path:string)=>path,POLICY:{generatedDirectories:[]},
  isOpaqueSourcePathV1:()=>false,EXAMPLE_COLLECTIONS:[],WORKSPACE_OWNERSHIP:{collections:{moduleMember:"member"}},
  lstatSync:()=>({isDirectory:()=>true}),readdirSync:(path:string)=>{
   reads++;
   return path===tree?names.map(name=>({name,isDirectory:()=>true,isFile:()=>false,isSymbolicLink:()=>false})):
    [{name:"schema.json",isDirectory:()=>false,isFile:()=>true,isSymbolicLink:()=>false}];
  },
  yieldInferenceV1:async(_context:unknown,_phase:string,index:number)=>{checkpoints.push(index);},
  loadedProjectInputs:{createBuildInputPathMatcherV1:()=>matches},
 });
 const result=await execute([{target:"schema",compile:true,patterns:[pattern]}],workspace,{});
 const expected=paths.filter(path=>picomatch.isMatch(path,pattern)).sort();
 expect([...result.matched].sort()).toEqual(expected);
 expect([...result.compileByTarget.get("schema")].sort()).toEqual(expected);
 expect(reads).toBe(directories+1);
 expect(Math.max(...checkpoints.map((index,at)=>index-(checkpoints[at-1]??0)))).toBeLessThanOrEqual(INFERENCE_LIMITS_V1.selectedSourceCheckpointEntries);
 expect(checkpoints.length).toBeLessThanOrEqual(maximumCheckpoints);
});
