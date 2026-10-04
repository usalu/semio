/** 🚫️ Compares explicit process path admission with independent filesystem and schema witnesses. */
import {test,expect} from "bun:test";
import {readFileSync,mkdirSync} from "node:fs";
import {resolve} from "node:path";
import Ajv from "ajv";
import {readProcessOwnerContextV1} from "../../🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../🧬️schema/✅️validator/🟦️.ts";
import {runBudgetedTestCommand} from "../../../🧪️testing/🎛️execution/🟦️.ts";
import {testLevelBudgetMs} from "../../../🧪️testing/🎚️budget/🟦️.ts";
type Case=Readonly<{id:string;field:"cwd"|"cacheRoot"|"leaseDirectory";kind:"normal"|"nul-prefix"|"nul-suffix"|"relative";valid:boolean}>;
test("Explicit process paths refuse NUL before filesystem access and retain owner binding",async()=>{
 const corpus=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🚫️paths/🔣️.json",import.meta.url),"utf8")) as {version:1;cases:Case[]},corpusSchema=JSON.parse(readFileSync(new URL("../../🧬️schema/🚫️paths/🔣️.json",import.meta.url),"utf8"));
 expect(new Ajv({strict:true}).compile(corpusSchema)(corpus)).toBe(true);expect(validateJsonSchemaSubset(corpusSchema,corpus)).toEqual([]);
 const directory=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!directory)throw Error("Caller-owned process test output required");const base=resolve(directory,"process-context-paths");mkdirSync(base,{recursive:true});
 const rows=corpus.cases.map(row=>{const context={version:1 as const,cwd:base,cacheRoot:base,leaseDirectory:base},value=context[row.field];context[row.field]=row.kind==="nul-prefix"?"\0"+value:row.kind==="nul-suffix"?value+"\0":row.kind==="relative"?"relative-owner":value;let admitted=true;try{readProcessOwnerContextV1({SEMIO_PROCESS_OWNER_CONTEXT:JSON.stringify(context)},context.cwd);}catch{admitted=false;}return {id:row.id,context,admitted,expected:row.valid};});
 const node=Bun.which("node");if(!node)throw Error("Independent Node filesystem oracle required");
 const code="const fs=require('node:fs'),p=require('node:path'),rows=JSON.parse(process.argv[1]);console.log(JSON.stringify(rows.map(r=>({id:r.id,valid:['cwd','cacheRoot','leaseDirectory'].every(k=>{if(!p.isAbsolute(r.context[k]))return false;try{fs.statSync(r.context[k]);return true;}catch(e){if(e.code==='ERR_INVALID_ARG_VALUE'||e.code==='ERR_INVALID_ARG_TYPE')return false;throw e;}})}))));";
 let stdout="";await runBudgetedTestCommand(node,["--eval",code,JSON.stringify(rows)],{cwd:base,env:process.env,budgetMs:testLevelBudgetMs(),throwOnFailure:true,captureStdout:{limitBytes:8192,onChunk:bytes=>{stdout+=Buffer.from(bytes).toString("utf8");}}});
 const oracle=JSON.parse(stdout) as {id:string;valid:boolean}[];expect(oracle).toEqual(rows.map(row=>({id:row.id,valid:row.expected})));
 console.log("[DEBUG] process-owner-path-admission "+JSON.stringify({observed:rows.map(row=>({id:row.id,admitted:row.admitted,expected:row.expected})),oracle}));
 expect(rows.map(row=>({id:row.id,valid:row.admitted}))).toEqual(oracle);
 const context={version:1,cwd:base,cacheRoot:base,leaseDirectory:base};expect(()=>readProcessOwnerContextV1({SEMIO_PROCESS_OWNER_CONTEXT:JSON.stringify(context)},resolve(base,"foreign"))).toThrow();
});
