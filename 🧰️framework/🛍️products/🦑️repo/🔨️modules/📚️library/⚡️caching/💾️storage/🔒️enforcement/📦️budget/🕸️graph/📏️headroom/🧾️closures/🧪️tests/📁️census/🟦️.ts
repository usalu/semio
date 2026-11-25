import{physicalSourceOriginV1}from"../../../../../../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import ts from"typescript";
import{readFile}from"node:fs/promises";
import{mkdtempSync,mkdirSync,writeFileSync,symlinkSync,lstatSync,readdirSync,renameSync,rmSync,utimesSync}from"node:fs";
import{join,relative,sep}from"node:path";
import{createIssuedResourceControlV1}from"@semio-tech/process/routing";
import{withRepositoryTestArtifactsV1,repositoryTestArtifactRootV1}from"@semio-tech/repo-test/storage";
import{nativeImageServiceEntryV1,requireNativeStoragePathV1}from"@semio-tech/process/artifacts/native-build/storage/platform/contract";
import{validateJsonSchemaSubset}from"@semio-tech/schema-validator-ts";
import schema from"./🧬️schema/🔣️.json";
import fixture from"./🧫️cases.json";
import component from"../../🧬️schema/🔣️.json";
import{privateRepositoryWorkspaceV1,withPrivateRepositoryBuildTestInvocationV1}from"../../../../../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import {declareLibraryBuildTestEnvelopeV1} from "../../../../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
declareLibraryBuildTestEnvelopeV1({suite:"storage-graph-processes"});
const repository=await privateRepositoryWorkspaceV1();
const stamp=(row:any)=>[row.dev,row.ino,row.mode,row.nlink,row.size,row.mtimeNs,row.ctimeNs].join(":"),ancestor=(row:any)=>[row.dev,row.ino,row.mode].join(":");

test("closure proof accepts derived-node writes and still rejects replacement of its root",async()=>withPrivateRepositoryBuildTestInvocationV1(async original=>withRepositoryTestArtifactsV1(repository.root,original,createIssuedResourceControlV1(original,"test:closure-census-artifacts",0),async()=>{
 const source=(await readFile(new URL("../../🏷️names/🟦️.ts",physicalSourceOriginV1(import.meta.url,(import.meta as ImportMeta&{semioPhysicalSourceURL?:string}).semioPhysicalSourceURL)),"utf8"))+"\n"+(await readFile(new URL("../../🟦️.ts",physicalSourceOriginV1(import.meta.url,(import.meta as ImportMeta&{semioPhysicalSourceURL?:string}).semioPhysicalSourceURL)),"utf8"));
 const tree=ts.createSourceFile("closures.ts",source,ts.ScriptTarget.Latest,true);
 const nodes=tree.statements.filter((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&["directories","recheck","spill","liveSpillDirectory"].includes(node.name?.text??""));
 expect(nodes).toHaveLength(4);
 const code=ts.transpileModule(nodes.map(node=>node.getText(tree)).join("\n"),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText;
 const ports={P:component.definitions.Protocol.const,join,relative,sep,stamp,ancestor,nativeImageServiceEntryV1,temporaryBase:()=>undefined,directoryChangeV1:()=>"",requireNativeStoragePathV1,lstatSync,
  named:(_proof:unknown,path:string)=>lstatSync(path,{bigint:true}),namesAt:(_proof:unknown,path:string)=>readdirSync(path).sort(),
  physicalRow:(_proof:unknown,path:string)=>{throw Error("Unexpected closure leaf "+path);},
  check:()=>{},requireCurrentGraphClosureImplementationV1:()=>{},requireRepositoryBuildGraphMaintenanceImageV1:()=>{}};
 const {directories,recheck}=Function(...Object.keys(ports),code+";return {directories,recheck}")(...Object.values(ports));
 const db=new Database(":memory:");
 const oracle=db.query("select case when ?1 != 'derived-nodes' and ?1 not like 'derived-nodes/%' then 'directory' when ?2 = 'replace' then 'ancestor' else 'accepted' end as result");
 try{
  expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
  expect(validateJsonSchemaSubset(schema,fixture,schema)).toEqual([]);
  for(const row of fixture.changes){
   expect(oracle.get(row.directory,row.change)).toEqual({result:row.result});
   const root=mkdtempSync(join(repositoryTestArtifactRootV1(repository.root),"cc-"));
   try{
    const directory=join(root,...row.directory.split("/"));
    mkdirSync(directory,{recursive:true});
    const proof={image:{root},directories:new Map(),ancestors:new Map(),rows:new Map(),missing:new Set(),refusals:[],temporaries:[]};
    directories(proof);
    if(row.change==="replace"){
     renameSync(directory,directory+".previous");
     mkdirSync(directory);
    }else{
     writeFileSync(join(directory,"entry"),"cached");
     utimesSync(directory,1,1);
    }
    if(row.result==="accepted"){
     expect(proof.ancestors.has(join(root,"derived-nodes"))).toBe(true);
     expect(proof.directories.has(directory)).toBe(false);
     expect(()=>recheck(proof),JSON.stringify(row)).not.toThrow();
    }
    if(row.result==="ancestor")expect(()=>recheck(proof),JSON.stringify(row)).toThrow("Closure proof ancestor changed");
    if(row.result==="directory")expect(()=>recheck(proof),JSON.stringify(row)).toThrow("Closure reference directory changed");
   }finally{rmSync(root,{recursive:true,force:true});}
  }
 }finally{db.close();}
})),60000);

test("closure census delegates root service ownership before application path access",async()=>withPrivateRepositoryBuildTestInvocationV1(async original=>withRepositoryTestArtifactsV1(repository.root,original,createIssuedResourceControlV1(original,"test:closure-census-artifacts",0),async()=>{const source=(await readFile(new URL("../../🏷️names/🟦️.ts",physicalSourceOriginV1(import.meta.url,(import.meta as ImportMeta&{semioPhysicalSourceURL?:string}).semioPhysicalSourceURL)),"utf8"))+"\n"+(await readFile(new URL("../../🟦️.ts",physicalSourceOriginV1(import.meta.url,(import.meta as ImportMeta&{semioPhysicalSourceURL?:string}).semioPhysicalSourceURL)),"utf8")),tree=ts.createSourceFile("closures.ts",source,ts.ScriptTarget.Latest,true),nodes=tree.statements.filter((value):value is ts.FunctionDeclaration=>ts.isFunctionDeclaration(value)&&["directories","spill","liveSpillDirectory","retiredSpill"].includes(value.name?.text??""));expect(nodes).toHaveLength(4);const code=ts.transpileModule(nodes.map(node=>node.getText(tree)).join("\n"),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText,db=new Database(":memory:");expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);expect(validateJsonSchemaSubset(schema,fixture,schema)).toEqual([]);try{for(const row of fixture.cases){expect(nativeImageServiceEntryV1(row.name)).toBe(row.service);const oracle=db.query("select lower(?1) in ('.fseventsd','.metadata_never_index') service,?2 and lower(?1) in ('.fseventsd','.metadata_never_index') omitted").get(row.name,Number(row.root))as{service:number;omitted:number};expect(Boolean(oracle.service)).toBe(row.service);expect(Boolean(oracle.omitted)).toBe(row.omitted);const root=mkdtempSync(join(repositoryTestArtifactRootV1(repository.root),"cc-")),parent=row.root?root:join(root,"ordinary");if(!row.root)mkdirSync(parent);const entry=join(parent,row.name);if(row.kind==="directory")mkdirSync(entry);else if(row.kind==="file")writeFileSync(entry,"");else symlinkSync(root,entry,process.platform==="win32"?"junction":"dir");let accessed=0;const proof={image:{root},directories:new Map(),ancestors:new Map(),rows:new Map(),refusals:[],temporaries:[]},application=(path:string)=>{if(path===entry)accessed++;return requireNativeStoragePathV1(root,path);},ports={P:component.definitions.Protocol.const,join,relative,sep,stamp,ancestor,nativeImageServiceEntryV1,temporaryBase:()=>undefined,directoryChangeV1:()=>"",check:()=>{},requireNativeStoragePathV1:(_root:string,path:string)=>application(path),named:(_proof:unknown,path:string)=>{application(path);return lstatSync(path,{bigint:true});},lstatSync,namesAt:(_proof:unknown,path:string)=>readdirSync(path).sort(),physicalRow:(_proof:unknown,path:string)=>{application(path);return{path};}},run=Function(...Object.keys(ports),code+";return directories")(...Object.values(ports));let accepted=true;try{run(proof);}catch{accepted=false;}try{expect(accepted,JSON.stringify(row)).toBe(row.accepted);expect(accessed===0,JSON.stringify(row)).toBe(row.omitted);if(row.omitted){expect(proof.rows.has(entry)).toBe(false);expect(proof.directories.has(entry)).toBe(false);expect(()=>requireNativeStoragePathV1(root,entry)).toThrow("Native application path enters reserved platform service domain");}}finally{rmSync(root,{recursive:true,force:true});}}}finally{db.close();}})),60000);

test("Native service classification is explicitly exported through its public contract",async()=>{const text=await readFile(new URL("../../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts",physicalSourceOriginV1(import.meta.url,(import.meta as ImportMeta&{semioPhysicalSourceURL?:string}).semioPhysicalSourceURL)),"utf8"),tree=ts.createSourceFile("public.ts",text,ts.ScriptTarget.Latest,true),exports=tree.statements.flatMap(node=>ts.isExportDeclaration(node)&&node.exportClause&&ts.isNamedExports(node.exportClause)?node.exportClause.elements.map(value=>value.name.text):[]);expect(exports).toContain("nativeImageServiceEntryV1");});
