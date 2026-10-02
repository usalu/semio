import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, symlinkSync, writeFileSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { dirname, join, resolve } from "node:path";
import Ajv from "ajv/dist/2020.js";
import TOML from "@iarna/toml";
import { inspectRustModuleGraph, inspectRustModuleGraphFacts, rustModuleScopeProof } from "../../../../../🔍️discovery/🟦️.ts";
import { inspectRustBindingFacts, rustExternProviders, resolveRustImportBindings, type RustBindingProblemKind } from "../🟦️.ts";

const owner = resolve(import.meta.dir, "..");
const corpus = JSON.parse(readFileSync(resolve(owner,"🧫️fixtures/🔣️.json"),"utf8")) as {scopeCases:readonly {id:string;source:string;sourceScope:readonly string[];state:"resolved"|"unresolved";modulePath?:readonly string[];code?:string;count?:number}[];mountCase:{sources:Record<string,string>;source:string;expectedBindings:{from:string;root:string;provider:string;module:string[]}[];linkedAncestor:string;expectedLinkedProblem:RustBindingProblemKind};cases: {id:string;source:string;sourceLocator?:string;rootSources?:Record<string,string>;consumerManifest:string;compileKind:"library"|"test"|"build";manifests:Record<string,string>;expectedRoots:string[];expectedProblems:RustBindingProblemKind[];expectedProviders:string[];authorityWitness?:{manifest:string;field:"library-path"|"workspace-path"|"dependency-path"};inputNodes?:Record<string,"missing"|"symlink"|"file"|"directory">}[]};
test("closed portable binding corpus",()=>{
 const validate=new Ajv({strict:true}).compile(JSON.parse(readFileSync(resolve(owner,"🧬️schema/🔣️.json"),"utf8")));
 expect(validate(corpus)).toBe(true);
 expect(new Set(corpus.cases.map(row=>row.id)).size).toBe(corpus.cases.length);
});
for(const row of corpus.cases) test(row.id,()=>{
 const files=new Map(Object.entries(row.manifests));
 const sources=new Map(files);
 const from=row.sourceLocator??"consumer/lib.rs";
 sources.set(from,row.source);
 for(const [path,source] of Object.entries(row.rootSources??{})) sources.set(path,source);
 for(const path of files.keys()) if(path!==row.consumerManifest) sources.set(path.replace("Cargo.toml","lib.rs"),"pub const VALUE:u32=19;");
 const inventory=new Map<string,"file"|"directory"|"symlink">();
 for(const path of sources.keys()){const segments=path.split("/");for(let i=1;i<segments.length;i++)inventory.set(segments.slice(0,i).join("/"),"directory");inventory.set(path,"file");}
 for(const [path,kind] of Object.entries(row.inputNodes??{})) if(kind==="missing") inventory.delete(path); else inventory.set(path,kind);
 if(row.authorityWitness){
  const output=process.env.SEMIO_TEST_ARTIFACT_DIR;
  if(!output)throw Error("Caller-owned SEMIO_TEST_ARTIFACT_DIR is required");
  mkdirSync(output,{recursive:true});const root=mkdtempSync(join(output,"raw-cargo-path-"));
  for(const [path,source] of sources){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),source);}
  for(const [path,kind] of Object.entries(row.inputNodes??{})){
   if(kind==="file")writeFileSync(join(root,path),"blocked");
   else if(kind==="directory")mkdirSync(join(root,path),{recursive:true});
   else if(kind==="symlink"){const target=join(root,"owned-link-target");mkdirSync(target,{recursive:true});symlinkSync(target,join(root,path),process.platform==="win32"?"junction":"dir");}
  }
  const oracle=String.raw`const fs=require("node:fs"),p=require("node:path"),toml=require("@iarna/toml");const root=process.argv[1],w=JSON.parse(process.argv[2]);const d=toml.parse(fs.readFileSync(p.join(root,w.manifest),"utf8"));const raw=w.field==="library-path"?d.lib.path:w.field==="workspace-path"?d.package.workspace+"/Cargo.toml":d.dependencies.wire.path+"/Cargo.toml";const components=(p.posix.dirname(w.manifest)+"/"+raw).split("/");let verdict=null;for(let i=0;i<components.length;i++){const prefix=root+"/"+components.slice(0,i+1).join("/");try{const info=fs.lstatSync(prefix);if(info.isSymbolicLink()){verdict="linked-input";break;}if(i<components.length-1&&!info.isDirectory()){verdict="non-directory-ancestor";break;}}catch(error){if(error.code!=="ENOENT"&&error.code!=="ENOTDIR")throw error;verdict=error.code==="ENOENT"?"missing-input":"non-directory-ancestor";break;}}console.log(JSON.stringify({verdict}));`;
  const receipt=JSON.parse(execFileSync("node",["-e",oracle,root,JSON.stringify(row.authorityWitness)],{cwd:resolve(owner,"../../../../../../../../.."),encoding:"utf8",timeout:3_000})) as {verdict:string|null};
  writeFileSync(join(root,"node-physical-path-admission.json"),JSON.stringify(receipt));
  expect(receipt.verdict).toBe(row.expectedProblems[0]);
 }
 const sourceFiles=new Set([...sources.keys()].filter(path=>path.endsWith(".rs")));
 const graph=inspectRustModuleGraph([...sources.keys()],path=>sources.get(path),{strictManifests:true});
 const contexts=(graph.contexts.get(from)??[]).map(context=>{
  const authority=rustExternProviders({context,compileKind:row.compileKind,files,inventory,sourceFiles});
  const oracle=rustExternProviders({context,compileKind:row.compileKind,files,inventory,sourceFiles,parser:{parse:source=>TOML.parse(source)}});
  expect(authority).toEqual(oracle);
  return {context,authority};
 });
 const facts=inspectRustBindingFacts(row.source);
 expect(facts.imports.map(fact=>fact.path[0])).toEqual(row.expectedRoots);
 const result=resolveRustImportBindings({from,facts,contexts,rootFacts:new Map(contexts.map(row=>[row.context.crateRoot,inspectRustBindingFacts(sources.get(row.context.crateRoot)!)]))});
 expect(result.status).toBe(row.expectedProblems.length?"unproven":"complete");
 expect(result.bindings.map(row=>row.to)).toEqual(row.expectedProviders);
 expect(result.problems.map(p=>p.kind).sort()).toEqual([...row.expectedProblems].sort());
 for(const fact of facts.imports) expect(Buffer.from(row.source).subarray(fact.span.start,fact.span.end).toString()).toBe(row.source.slice(fact.characterSpan.start,fact.characterSpan.end));
});

test("one physical source keeps both graph-proven provider identities",()=>{
 const row=corpus.mountCase, sources=new Map(Object.entries(row.sources)), source=sources.get(row.source)!;
 const inventory=new Map<string,"file"|"directory"|"symlink">();
 for(const path of sources.keys()){const parts=path.split("/");for(let i=1;i<parts.length;i++)inventory.set(parts.slice(0,i).join("/"),"directory");inventory.set(path,"file");}
 const sourceFiles=new Set([...sources.keys()].filter(path=>path.endsWith(".rs")));
 const graph=inspectRustModuleGraph([...sources.keys()],path=>sources.get(path),{strictManifests:true});
 const contexts=(graph.contexts.get(row.source)??[]).map(context=>({context,authority:rustExternProviders({context,compileKind:"library",files:sources,inventory,sourceFiles})}));
 const actual=resolveRustImportBindings({from:row.source,facts:inspectRustBindingFacts(source),contexts});
 expect(actual.problems).toEqual([]);
 expect(actual.bindings.map(row=>({from:row.from,root:row.crateRoot,provider:row.to,module:row.modulePath}))).toEqual(row.expectedBindings);
 inventory.set(row.linkedAncestor,"symlink");
 expect(rustExternProviders({context:contexts[0]!.context,compileKind:"library",files:sources,inventory,sourceFiles}).problems.map(row=>row.kind)).toEqual([row.expectedLinkedProblem]);
});
test("binding preparation honors cancellation",()=>{
 expect(()=>inspectRustBindingFacts("use wire::VALUE;",{checkCancellation:()=>{throw Error("cancelled");}})).toThrow("cancelled");
});

for(const row of corpus.scopeCases)test(row.id,()=>{
 const proof=rustModuleScopeProof(inspectRustModuleGraphFacts(row.source),row.sourceScope);
 expect(proof.state).toBe(row.state);
 if(proof.state==="unresolved"){expect<readonly string[] | undefined>(proof.modulePath).toEqual(row.modulePath);expect<string | undefined>(proof.problem.code).toBe(row.code);if("count"in proof.problem)expect<number | undefined>(proof.problem.count).toBe(row.count);}
 const result=resolveRustImportBindings({from:"consumer/lib.rs",facts:inspectRustBindingFacts(row.source),contexts:[{context:{crateRoot:"consumer/lib.rs",manifestPath:"consumer/Cargo.toml",modulePath:row.sourceScope,sourceScope:row.sourceScope,moduleBase:"consumer",sourceChain:["consumer/lib.rs"],mount:{kind:"root"}},authority:{status:"complete",edition:"2021",problems:[],providers:[{externName:"wire",dependencyKey:"wire",dependencyKind:"normal",manifestPath:"provider/Cargo.toml",librarySource:"provider/lib.rs",packageName:"provider",libraryName:"wire",workspaceInherited:false,optional:false}]}}]});
 expect(result.bindings.length).toBe(row.state==="resolved"?1:0);
 if(row.state==="unresolved")expect(result.problems.some(problem=>problem.kind==="unproven-source-scope")).toBe(true);
});
