import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import ts from "typescript";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import fixture from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";
import {validateJsonSchemaSubset} from "../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";

/** 🎟️ Executes the actual metadata body against independently ordered phase ports without storage effects. */
export async function verifyStdioCargoPhaseV1(root:string):Promise<void>{
 assert.deepEqual(validateJsonSchemaSubset(schema,fixture),[]);assert(new Ajv({strict:true}).compile(schema)(fixture));
 const source=ts.createSourceFile(fixture.owner,readFileSync(join(root,fixture.owner),"utf8"),ts.ScriptTarget.Latest,true),declaration=source.statements.find((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text===fixture.function);assert(declaration);
 const javascript=ts.transpileModule(declaration.getText(source).replace(/^export /u,""),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText,database=new Database(":memory:");
 try{database.exec("CREATE TABLE phases(position INTEGER, name TEXT)");for(const[index,name]of fixture.phases.entries())database.query("INSERT INTO phases VALUES(?,?)").run(index,name);
 for(const row of fixture.cases){const events:string[]=[],original={},entry={},tools={},cargo={},cancelled=Error("Stdio fixture cargo phase cancelled");let stopped=false,captures=0;
 const check=(value:unknown)=>{assert.equal(value,cargo);if(stopped)throw cancelled;},ports={checkScriptInvocation:(value:unknown)=>assert.equal(value,original),nativeCallerPhaseV1:()=>({advance:async()=>{},budget:()=>10000,authority:{transport:{maximumBytes:16777216,maximumLines:65536}},options:(env:unknown)=>({env})}),withScriptInvocationSignalV1:async(_original:unknown,_signal:unknown,body:(value:unknown)=>Promise<unknown>)=>body(original),createIssuedResourceControlV1:()=>{events.push("entry");return entry;},withRepositoryToolingStoreV1:async(_root:unknown,value:unknown,phase:unknown,body:(store:unknown,phase:unknown)=>Promise<unknown>)=>{assert.equal(value,original);assert.equal(phase,entry);events.push("tools");return body({},tools);},withRepositoryRustToolchainV1:async(_root:unknown,_store:unknown,phase:unknown,body:(generation:unknown,phase:unknown)=>Promise<unknown>)=>{assert.equal(phase,tools);events.push("cargo");return body({cargo:"owned-cargo",environment:{},digest:"fixture",recheck:async()=>{if(row.cancelAtRecheck)stopped=true;}},cargo);},checkResourceLeaseControl:check,stdioArtifactPackageContract:()=>({packages:[{rust:{manifest:"Cargo.toml"}}]}),cargoWorkspaceForManifest:()=>({manifest:"Cargo.toml"}),captureOwnedCommandResultV1:async(_program:unknown,_args:unknown,_root:unknown,options:{control:unknown})=>{check(options.control);captures++;return{status:0,signal:null,stdout:JSON.stringify({packages:[]}),stderr:""};},assert,join,setTimeout,clearTimeout,AbortController,process,console:{log:()=>{}}};
 const run=Function(...Object.keys(ports),javascript+";return stdioCargoMetadata;")(...Object.values(ports)) as(root:string,original:unknown)=>Promise<unknown>;
 if(row.capture)assert.deepEqual(await run(root,original),{packages:[]});else await assert.rejects(run(root,original),error=>error===cancelled);
 assert.equal(captures,Number(row.capture));assert.deepEqual(events,database.query("SELECT name FROM phases ORDER BY position").all().map(value=>(value as {name:string}).name));
 }}finally{database.close();}
 console.log("[DEBUG] Stdio Cargo phase Source algorithm and independent SQLite/Ajv oracle verified");
}
