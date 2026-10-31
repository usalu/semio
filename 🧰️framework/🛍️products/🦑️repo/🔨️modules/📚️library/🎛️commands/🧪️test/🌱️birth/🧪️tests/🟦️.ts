import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import ts from "typescript";
import schema from "../🧬️schema/🔣️.json";
import cases from "./🔣️.json";
import {rootTestBuildArgumentsV1} from "../🟦️.ts";
const library=join(import.meta.dir,"../../../.."),validate=new Ajv({strict:false}).compile(schema.$defs.Case);
const declaration=(source:string,name:string)=>{const ast=ts.createSourceFile("source.ts",source,ts.ScriptTarget.Latest,true),node=ast.statements.find(row=>ts.isFunctionDeclaration(row)&&row.name?.text===name);if(!node)throw Error("Actual root birth declaration required: "+name);return node.getText(ast);};

test("closed fresh and received birth vectors preserve Original and agree with SQLite selection",async()=>{
 const source=readFileSync(join(import.meta.dir,"../🟦️.ts"),"utf8"),helper=readFileSync(join(library,"🚨️policy/🧭️entrypoints/🟦️.ts"),"utf8"),declared=["daemon-output-observation"],db=new Database(":memory:");
 try{for(const row of cases){expect(validate(row),JSON.stringify(validate.errors)).toBe(true);const query=db.query("SELECT CASE WHEN ?1 THEN 'received' WHEN (json_array_length(?2)=3 AND json_extract(?2,'$[0]')='test' AND json_extract(?2,'$[1]')='cache' AND EXISTS(SELECT 1 FROM json_each(?3) WHERE value=json_extract(?2,'$[2]'))) OR (json_array_length(?2)>=2 AND json_extract(?2,'$[0]')='schema' AND json_extract(?2,'$[1]') IN ('verify','entries','compile') AND NOT EXISTS(SELECT 1 FROM json_each(?2) WHERE value IN ('-h','--help'))) THEN 'build' ELSE 'workspace' END expected").get(row.received?1:0,JSON.stringify(row.argv),JSON.stringify(declared)) as {expected:string};expect(query.expected).toBe(row.expected);
 const original={identity:Symbol("actual received Original")},environment={marker:"env"},calls:unknown[][]=[],captured:unknown[]=[];
 const factoryCode=ts.transpile(declaration(source,"rootTestFreshCapabilitiesV1").replace(/^export /u,""),{target:ts.ScriptTarget.ESNext}).replace('import("../../../⚡️caching/🗂️suites/🟦️.ts")',"Promise.resolve(ports.table)").replace('import("../../../🏗️build/📥️caller/📤️child/🟦️.ts")',"Promise.resolve(ports.build)").replace('import("../../../🏃️process/🔌️providers/🟦️.ts")',"Promise.resolve(ports.providers)");
 const fresh=new Function("rootTestBuildArgumentsV1","ports",factoryCode+";return rootTestFreshCapabilitiesV1;")(rootTestBuildArgumentsV1,{table:{readCachingTestTableV1:()=>({codeSuites:declared})},providers:{issueRepositoryScriptProvidersV1:(environment:unknown)=>calls.push(["providers",environment])},build:{createBuildBootstrapCapabilitiesV1:async(...args:unknown[])=>{calls.push(args);return{native:"issued-at-birth"};}}});
 const helperCode=ts.transpile(declaration(helper,"runWorkspaceScriptMain").replace(/^export /u,""),{target:ts.ScriptTarget.ESNext});
 const run=new Function("process","SCRIPT_PROCESS_INVOCATION_ENV","runScriptMain","receiveScriptProcessInvocation","readScriptPolicy","withScriptProcessEnvelope","createScriptProcessEnvelope",helperCode+";return runWorkspaceScriptMain;")({env:row.received?{wire:"original-wire"}:{}},"wire",async(_router:unknown,options:{invocation:unknown})=>{captured.push(options.invocation);},async(env:{wire:string},body:(original:unknown)=>Promise<void>)=>{expect(env.wire).toBe("original-wire");await body(original);},(policy:unknown)=>policy,async(envelope:unknown,body:(original:unknown)=>Promise<void>)=>body(envelope),(_policy:unknown,capabilities:unknown)=>({capabilities}));
 await run({repoRoot:"/repo"},{freshCapabilities:()=>fresh("/repo",row.argv,environment)});
 if(row.received){expect(captured[0]).toBe(original);expect(calls).toEqual([]);}else if(row.expected==="build"){expect(calls).toEqual([["providers",environment],["/repo",row.argv,environment]]);expect(captured[0]).toEqual({capabilities:{native:"issued-at-birth"}});}else{expect(calls).toEqual([]);expect(captured[0]).toEqual({capabilities:{workspaceRoot:"/repo"}});}
 }}finally{db.close();}
});

