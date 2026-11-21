import {expect,test} from "bun:test";
import * as fs from "node:fs";
import {dirname,join} from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import {evaluateExtractedFunctionV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧩️extracted-function/🟦️.ts";
import {testPythonV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {testScratchRootV1} from "../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🟦️.ts";
import {createBuildInputPathMatcherV1} from "../../../../../📥️inputs/🟨️.mjs";
import fixture from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

test("identical source glob questions retain each target's matches and avoid repeated candidate filtering",async()=>{
 const source=ts.createSourceFile("owner.mjs",fs.readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true,ts.ScriptKind.JS);
 const declaration=source.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="readBuildSourceInputMatchesV1");
 if(!declaration)throw Error("Actual selected input matcher required");
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const root=fs.mkdtempSync(join(testScratchRootV1("selected-source-matches"),"match-"));
 const scans=Array.from({length:fixture.scanCopies},(_,index)=>({target:"target-"+index,compile:index>0,patterns:fixture.patterns}));
 scans.push({target:"alternate",compile:true,patterns:[fixture.patterns[0]!,"!tree/keep/**/*.ts"]});
 const counts=new Map<string,number>(),literal=join(root,"tree","literal.txt");
 let literalProbes=0,directoryReads=0,yields=0,abortAt=Infinity;
 const abort=new AbortController(),reason=new Error("cancel during selected Source matching");
 const execute=evaluateExtractedFunctionV1(declaration.getText(source).replace(/^export\s+/,""),"readBuildSourceInputMatchesV1",{
  join,dirname,buildPath:(path:string)=>path.replaceAll("\\","/"),POLICY:{generatedDirectories:[]},isOpaqueSourcePathV1:()=>false,EXAMPLE_COLLECTIONS:[],WORKSPACE_OWNERSHIP:{collections:{moduleMember:"member"}},lstatSync:fs.lstatSync,readdirSync:(path:string,options:{withFileTypes:true})=>{directoryReads++;return fs.readdirSync(path,options);},existsSync:(path:string)=>{if(path===literal)literalProbes++;return fs.existsSync(path);},
  yieldInferenceV1:async()=>{if(++yields===abortAt)abort.abort(reason);abort.signal.throwIfAborted();},
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
  expect(literalProbes).toBe(fixture.scanCopies);
  expect(directoryReads).toBe(1+fixture.folders.length*2);
  const candidates=fixture.folders.length*fixture.filesPerKind*2+1;
  expect(counts.get(fixture.patterns[0]!)).toBe(candidates*2);
  abortAt=yields+candidates*2+20;
  await expect(execute(scans,root,{})).rejects.toBe(reason);
 }finally{fs.rmSync(root,{recursive:true,force:true});}
});
