import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020";
import ts from "typescript";
import {transformSync} from "esbuild";
import schema from "./🧫️fixtures/schema.json";
import fixture from "./🧫️fixtures/cases.json";

test("cache maintenance delegates finite test retention with original authority and selected language",async()=>{
 const source=ts.createSourceFile("workspace.ts",readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true),node=source.statements.find(value=>ts.isFunctionDeclaration(value)&&value.name?.text==="pruneTestEvidence")!;
 const owner=node.getText(source).replace(/^export /u,"").replaceAll(/import\(([^)]+)\)/gu,"load($1)"),validate=new Ajv().compile(schema),db=new Database(":memory:");
 try{for(const compiler of ["typescript","esbuild"]as const)for(const row of fixture.cases){
  expect(validate(row)).toBe(true);const original=Object.freeze({original:true}),control=Object.freeze({original}),policy={maximumRetainedBytes:2048,maximumRuns:7},outcome=Object.freeze({removed:["ended"],held:["active"],retainedRuns:1,freedBytes:1024}),refusal=Object.freeze({cancelled:true});let calls=0,raw=0;
  const ports={load:async(path:string)=>path.includes("policy")?{default:policy}:path.includes("storage")?{retainRepositoryTestArtifactsV1:async(root:string,invocation:unknown,phase:unknown,actual:unknown,tasks:unknown,options:unknown)=>{calls++;expect(root).toBe("/workspace");expect(invocation).toBe(original);expect(phase).toBe(control);expect(actual).toEqual({...policy,version:1});expect(tasks).toBeUndefined();expect(options).toEqual({dryRun:row.dry});if(row.cancelled)throw refusal;return outcome;},repositoryTestRetentionReportV1:(language:string,result:unknown,dry:boolean)=>{expect(language).toBe(row.language);expect(result).toBe(outcome);expect(dry).toBe(row.dry);return language;}}:{collectGarbage:()=>{raw++;return {};},formatGcReport:()=>"old",loadOracleRegistry:()=>({})},CACHE_POLICY:{storage:{tests:{unusedAgeMs:1}}}};
  const code=compiler==="typescript"?ts.transpileModule(owner,{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText:transformSync(owner,{loader:"ts",target:"es2022"}).code,run=new Function(...Object.keys(ports),code+";return pruneTestEvidence;")(...Object.values(ports));let result:unknown,caught:unknown;
  try{result=await run("/workspace",row.dry,original,control,row.language);}catch(error){caught=error;}
  expect(calls).toBe(1);expect(raw).toBe(0);expect(caught).toBe(row.cancelled?refusal:undefined);if(!row.cancelled)expect(result).toBe((db.query("SELECT ? AS language").get(row.language)as{language:string}).language);
 }}finally{db.close();}
});
