import {expect,test} from "bun:test";
import Ajv from "ajv";
import {minimatch} from "minimatch";
import ts from "typescript";
import {readFileSync} from "node:fs";
import {dirname,join,relative,resolve} from "node:path";
import {runInNewContext} from "node:vm";
import * as inputModule from "../🟨️.mjs";
import {buildInputPathMatchesV1,compactSourceInputs} from "../🟨️.mjs";
import fixture from "./🧫️fixtures/🔣️.json";

test("selected source discovery compiles each reached glob once per operation",async()=>{
 const source=readFileSync(new URL("../../🕸️graph/📥️inference/🟨️.mjs",import.meta.url),"utf8"),tree=ts.createSourceFile("inference.mjs",source,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS),owner=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="discoverBuildOwnerAdmissionFactsV1");
 if(!owner||!ts.isFunctionDeclaration(owner)||!owner.body)throw Error("Actual selected owner declaration required");
 const selection=owner.body.statements.find(node=>ts.isIfStatement(node)&&node.expression.getText(tree)==="targets.length||inputNames.length");
 if(!selection||!ts.isIfStatement(selection)||!ts.isBlock(selection.thenStatement))throw Error("Actual selected Source block required");
 const statements=selection.thenStatement.statements,start=statements.find(node=>ts.isVariableStatement(node)&&node.declarationList.declarations.some(declaration=>ts.isIdentifier(declaration.name)&&declaration.name.text==="exclusions")),end=statements.find(node=>ts.isExpressionStatement(node)&&node.getText(tree).includes('stage("source-closure")'));
 if(!start||!end)throw Error("Actual selected pattern loop required");
 const program="(async()=>{"+source.slice(start.getStart(tree),end.getStart(tree))+"return [...matched].sort();})()",validate=new Ajv({strict:true}).compile(fixture.discovery.contract),workspaceRoot=resolve("/__source_glob_fixture__");
 for(const row of fixture.discovery.cases){
  expect(validate(row),row.id).toBe(true);
  const negatives=row.patterns.filter(value=>value.startsWith("!")).map(value=>value.slice(1)),oracle=row.files.filter(path=>row.patterns.some(pattern=>!pattern.startsWith("!")&&minimatch(path,pattern,{dot:true,nonegate:true}))&&!negatives.some(pattern=>minimatch(path,pattern,{dot:true,nonegate:true}))).sort();
  expect(oracle,row.id).toEqual(row.expected);
  const directories=new Map<string,Map<string,boolean>>();
  for(const file of row.files){const parts=file.split("/");for(let index=0;index<parts.length;index++){const parent=join(workspaceRoot,...parts.slice(0,index)),entries=directories.get(parent)??new Map();entries.set(parts[index]!,index<parts.length-1);directories.set(parent,entries);}}
  for(let attempt=0;attempt<2;attempt++){
   const original=globalThis.RegExp,compiled:string[]=[];let result:unknown;
   globalThis.RegExp=new Proxy(original,{construct(target,args,newTarget){compiled.push(String(args[0]));return Reflect.construct(target,args,newTarget);}});
   try{result=await runInNewContext(program,{patterns:row.patterns,workspaceRoot,context:{},loadedProjectInputs:inputModule,POLICY:{generatedDirectories:[]},EXAMPLE_COLLECTIONS:[],WORKSPACE_OWNERSHIP:{collections:{moduleMember:"modules"}},buildPath:(path:string)=>path.replaceAll("\\","/"),join,dirname,yieldInferenceV1:async()=>{},addFile:()=>{},existsSync:(path:string)=>directories.has(path)||row.files.includes(relative(workspaceRoot,path).replaceAll("\\","/")),readdirSync:(path:string)=>[...(directories.get(path)??new Map())].map(([name,directory])=>({name,isSymbolicLink:()=>false,isDirectory:()=>directory,isFile:()=>!directory}))});}finally{globalThis.RegExp=original;}
   expect(result,row.id).toEqual(row.expected);
   expect(compiled.length,row.id+":"+attempt).toBe(row.compilations);
  }
 }
 console.log("[DEBUG] Actual selected discovery loop retains lazy glob semantics and one compilation per reached pattern against Ajv/minimatch");
});

test("source input compaction prepares exact patterns once per operation without changing lazy precedence",()=>{
  const validate=new Ajv({strict:true}).compile(fixture.contract);
  for(const row of fixture.matches){expect(minimatch(row.path,row.pattern,{dot:true,nonegate:true})).toBe(row.expected);expect(buildInputPathMatchesV1(row.path,row.pattern)).toBe(row.expected);}
  for(const row of fixture.cases){
    expect(validate(row),row.id).toBe(true);
    const normalize=(input:string)=>input.replaceAll("{projectRoot}",row.root).replaceAll("{workspaceRoot}/",""),exact=row.inputs.filter(input=>typeof input==="string"&&input.startsWith("{workspaceRoot}/")&&!/[!*?{}[\]]/u.test(input.slice("{workspaceRoot}/".length))) as string[],negatives=row.inputs.filter(input=>typeof input==="string"&&input.startsWith("!")&&!input.startsWith("!^")) as string[];
    expect(exact.every(path=>negatives.some(pattern=>minimatch(normalize(path),normalize(pattern.slice(1)),{dot:true,nonegate:true}))),row.id).toBe(true);
    for(let attempt=0;attempt<2;attempt++){
      const original=globalThis.RegExp,compiled:string[]=[];let result:unknown;
      globalThis.RegExp=new Proxy(original,{construct(target,args,newTarget){compiled.push(String(args[0]));return Reflect.construct(target,args,newTarget);}});
      try{result=compactSourceInputs(row.inputs,process.cwd(),row.root);}finally{globalThis.RegExp=original;}
      expect(result,row.id).toEqual(row.expected);
      expect(compiled.length,row.id+":"+attempt).toBe(row.compilations);
      if(!exact.length)expect(result).toBe(row.inputs);
    }
  }
  for(const row of fixture.errors)expect(()=>compactSourceInputs(row.inputs,process.cwd())).toThrow(row.message);
  console.log("[DEBUG] Source compaction operation-local compilation counts and lazy negation/path results agree with independent Ajv/minimatch");
});
