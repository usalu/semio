import {expect,test} from "bun:test";
import * as fs from "node:fs";
import {dirname,join,relative} from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import {evaluateExtractedFunctionV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧩️extracted-function/🟦️.ts";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {testScratchRootV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🟦️.ts";
import {buildInputSourceRoleV1,buildNativeSourceObservationV1,buildSourceDemandV1,registryOwnerIndexesV1} from "../../../🕸️graph/🚪️admission/🟦️.ts";
import ownerAdmissionSchema from "../../../🧬️schema/🚪️admission.json";
import * as observation from "../../🟨️.mjs";
import fixture from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

const {captureObservedSourceInputsV1,observationsHold,observedFs,observedSourceAnalysisV1,registerSourceAnalysisV1,withObservedWorldV1}=observation;
type Row=observation.SourceObservationV1;
const pair=observation.observedSourceAnalysisPairV1;
const revision="1".repeat(64);
const withFiles=async(body:(file:string,nested:string)=>Promise<void>)=>{
 const root=fs.mkdtempSync(join(testScratchRootV1("source-analysis-pair"),"pair-")),file=join(root,"input.ts"),nested=join(root,"nested.txt");
 try{fs.writeFileSync(file,fixture.original);fs.writeFileSync(nested,fixture.nested);await body(file,nested);}finally{fs.rmSync(root,{recursive:true,force:true});}
};
const providers=(role:string,nested:string)=>[
 registerSourceAnalysisV1({id:`test:source-analysis-pair:${role}:${fixture.providers[0]}`,revision,analyze:(_path,bytes)=>{observedFs.readFileSync(nested);return{head:bytes.toString("utf8").split(/\s+/u)[0]};}}),
 registerSourceAnalysisV1({id:`test:source-analysis-pair:${role}:${fixture.providers[1]}`,revision,analyze:(_path,bytes)=>({length:bytes.length})}),
] as const;

test("two exact source analyses share one physical read, preserving each provider and nested fact",()=>withFiles(async(file,nested)=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const python=testPythonV1(),oracle=Bun.spawnSync([python!,"-I",join(import.meta.dir,"🐍️.py")],{stdin:Buffer.from(JSON.stringify(fixture))});
 expect(oracle.exitCode,oracle.stderr.toString()).toBe(0);
 const modeled=JSON.parse(oracle.stdout.toString());
 const [first,second]=providers("stable",nested),original=await captureObservedSourceInputsV1(()=>[observedSourceAnalysisV1(first,file),observedSourceAnalysisV1(second,file)],undefined,false);
 let stats=0,reads=0;
 const world={...fs,statSync:((path:fs.PathLike)=>{if(path===file)stats++;return fs.statSync(path);}) as typeof fs.statSync,readFileSync:((path:fs.PathOrFileDescriptor)=>{if(path===file)reads++;return fs.readFileSync(path);}) as typeof fs.readFileSync};
 const current=await withObservedWorldV1(world,()=>captureObservedSourceInputsV1(()=>pair(first,second,file),undefined,false));
 expect({stats,reads}).toEqual({stats:1,reads:1});
 expect(current.value).toEqual(fixture.expectedResults);
 expect(current.value).toEqual(modeled.values);
 expect(current).toEqual(original);
 const analyses=current.observations.filter(row=>row.kind==="analysis");
 expect(analyses.map(row=>row.fact.provider)).toEqual([first.id,second.id]);
 expect(analyses.map(row=>row.fact.resultSha256)).toEqual(modeled.resultSha256);
 expect(analyses.map(row=>row.fact.source.sha256)).toEqual([modeled.originalSha256,modeled.originalSha256]);
 expect(analyses[0]?.fact.nestedObservations[0]).toMatchObject({kind:"file",path:nested,fact:{sha256:modeled.nestedSha256}});
 expect(analyses[1]?.fact.nestedObservations).toEqual([]);
 expect(await observationsHold(current.observations)).toBe(true);
 fs.writeFileSync(nested,fixture.replacement);
 expect(await observationsHold(current.observations)).toBe(false);
}));

test("a write between parsers cannot mix bytes or pass the original exact physical authority",()=>withFiles(async(file,nested)=>{
 const [,second]=providers("race",nested),first=registerSourceAnalysisV1({id:"test:source-analysis-pair:race-edit",revision,analyze:(_path,bytes)=>{
  fs.writeFileSync(file,fixture.replacement);
  bytes.fill(0);
  return{consumed:bytes.length};
 }});
 const current=await captureObservedSourceInputsV1(()=>{observedFs.readFileSync(file);return pair(first,second,file);},undefined,false);
 expect(current.value[1]).toEqual(fixture.expectedResults[1]);
 const exact=current.observations.find(row=>row.kind==="file"&&row.path===file);
 const analyses=current.observations.filter(row=>row.kind==="analysis");
 expect(analyses).toHaveLength(2);
 expect(analyses.every(row=>row.fact.source.sha256===(exact?.fact as observation.SourceFileFactV1).sha256)).toBe(true);
 expect(await observationsHold(current.observations)).toBe(false);
}));

test("a missing file or failing provider keeps the same failure and its original Source evidence",()=>withFiles(async(file,nested)=>{
 const [first,second]=providers("failures",nested),missing=join(file,"../absent.txt");
 let facts:readonly Row[]=[];
 const save=(rows:Row[])=>{facts=rows;};
 await expect(captureObservedSourceInputsV1(()=>pair(first,second,missing),save,false)).rejects.toMatchObject({code:"ENOENT"});
 expect(facts).toEqual([{kind:"file",path:missing,fact:{error:"ENOENT"}}]);
 const sentinel=Error("provider failed"),invalid=registerSourceAnalysisV1({id:"test:source-analysis-pair:invalid",revision,analyze:()=>{throw sentinel;}});
 await expect(captureObservedSourceInputsV1(()=>pair(first,invalid,file),save,false)).rejects.toBe(sentinel);
 expect(facts.filter(row=>row.kind==="analysis").map(row=>row.fact.provider)).toEqual([first.id]);
 expect(facts.find(row=>row.kind==="file"&&row.path===file)).toMatchObject({fact:{size:fixture.original.length}});
 expect(facts.some(row=>row.kind==="file"&&row.path===nested)).toBe(true);
}));

test("actual owner compile discovery keeps the exact physical Source row alongside both parser facts",()=>withFiles(async file=>{
 const source=ts.createSourceFile("owner.mjs",fs.readFileSync(new URL("../../../🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true,ts.ScriptKind.JS);
 const declaration=source.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="discoverBuildOwnerAdmissionFactsV1");
 if(!declaration)throw Error("Actual owner discovery required");
 const workspaceRoot=dirname(file),owner={name:"fixture",root:".",manifest:"📋️project.json",packages:[]},input=relative(workspaceRoot,file);
 const module=registerSourceAnalysisV1({id:"test:source-analysis-pair:owner-module",revision,analyze:()=>({imports:[],runtimeImports:[]})});
 const references=registerSourceAnalysisV1({id:"test:source-analysis-pair:owner-references",revision,analyze:()=>({runtime:{imports:[],requires:[]}})});
 const execute=evaluateExtractedFunctionV1(declaration.getText(source).replace(/^export\s+/,""),"discoverBuildOwnerAdmissionFactsV1",{
  libraryBootstrap:Promise.resolve(),join,relative,buildPath:(path:string)=>path.replaceAll("\\","/"),readPhysicalSource:(path:string)=>path===file?observedFs.readFileSync(file):Buffer.from(JSON.stringify({name:owner.name})),createScriptInputCache:()=>({}),cargoTargets:async()=>({}),componentBuildTargets:()=>({}),existsSync:()=>false,yieldInferenceV1:async()=>{},buildInputSourceRoleV1,buildNativeSourceObservationV1,buildSourceDemandV1,ownerAdmissionSchema,registryOwnerIndexesV1,parserTypescriptV1:()=>({sys:{}}),observedTypeScriptHost:()=>({}),
  projectWithDefaults:async()=>({namedInputs:{chosen:[]}}),WORKSPACE_POLICY:{},readBuildSourceInputMatchesV1:async()=>({matched:[input],compileByTarget:new Map([["chosen",new Set([input])]])}),relativeScriptInputs:async()=>[],prepareSourceAnalysesV1:()=>({module,references}),observedSourceAnalysisPairV1:pair,
 });
 const current=await captureObservedSourceInputsV1(()=>execute(owner,[],{version:1,owners:[owner],catalogProviders:[]},{workspaceRoot},["chosen"]),undefined,false);
 expect(current.value.observedFiles).toEqual([input]);
 expect(current.observations.filter(row=>row.kind==="analysis").map(row=>row.fact.provider)).toEqual([module.id,references.id]);
 expect(current.observations.some(row=>row.kind==="file"&&row.path===file)).toBe(true);
 expect(await observationsHold(current.observations)).toBe(true);
}));
