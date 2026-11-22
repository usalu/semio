import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdtempSync,readdirSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {ANALYSIS_STORE_POLICY_V1,analysisStoreKeyV1,collectAnalysisStoreV1,localAnalysisTextV1,neutralAnalysisTextV1,readStoredAnalysisV1,writeStoredAnalysisV1} from "../🟨️.mjs";

test("the key of a stored analysis equals the independent digest, is the same for two checkouts at different paths, and is none below the minimum size or outside the workspace",()=>{
 expect(new Ajv({strict:false}).validate({$ref:"#/definitions/Policy",definitions:schema.definitions},ANALYSIS_STORE_POLICY_V1)).toBe(true);
 expect(fixture.domain).toBe(ANALYSIS_STORE_POLICY_V1.domain);expect(fixture.minimumSourceBytes).toBe(ANALYSIS_STORE_POLICY_V1.minimumSourceBytes);
 const run=spawnSync("python3",["-I",join(import.meta.dir,"🐍️.py"),join(import.meta.dir,"🔣️.json")],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const keys=fixture.cases.map(row=>analysisStoreKeyV1(row.root,row.provider,row.revision,row.path,row.sha,row.bytes));
 expect(keys.map(key=>key??null)).toEqual(JSON.parse(run.stdout));
 expect(keys[0]).toBe(keys[1]);expect(keys[2]).toBeUndefined();expect(keys[3]).toBeUndefined();
});

test("an analysis is stored with the workspace root as a token, taken back with the root of another checkout, and a damaged or foreign entry is no entry",()=>{
 const directory=mkdtempSync(join(tmpdir(),"analysis-store-")),first="/Users/a/semio",second="/home/b/work/semio",key=analysisStoreKeyV1(first,"p","r",first+"/x/y.ts","aa".repeat(32),20000)!;
 try{
  const result={image:JSON.stringify({imports:["./z"]}),nestedObservations:[{kind:"exists",path:first+"/x/z.ts",fact:true}]};
  expect(writeStoredAnalysisV1(directory,first,key,result)).toBe(true);
  const taken=readStoredAnalysisV1(directory,second,key);
  expect(taken).toEqual({image:result.image,nestedObservations:[{kind:"exists",path:second+"/x/z.ts",fact:true}]});
  expect(JSON.stringify(readStoredAnalysisV1(directory,first,key))).toContain(first);
  expect(neutralAnalysisTextV1("a"+ANALYSIS_STORE_POLICY_V1.rootToken,first)).toBeUndefined();
  expect(localAnalysisTextV1(neutralAnalysisTextV1("/Users/a/semio/q /Users/a/semiox",first)!,first)).toBe("/Users/a/semio/q /Users/a/semiox");
  const other="bb".repeat(32);expect(readStoredAnalysisV1(directory,first,other)).toBeUndefined();
  const shard=join(directory,key.slice(0,2));writeFileSync(join(shard,key+".json.br"),"damaged");expect(readStoredAnalysisV1(directory,first,key)).toBeUndefined();
  expect(readdirSync(shard).filter(name=>name.endsWith(".tmp")).length).toBe(0);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("the collector keeps the directory under its budget by removing the oldest entries first and drops stale temporary files",()=>{
 const directory=mkdtempSync(join(tmpdir(),"analysis-store-")),root="/r";
 try{
  const keys=Array.from({length:6},(_,at)=>analysisStoreKeyV1(root,"p","r",root+"/f"+at+".ts",String(at).repeat(64).slice(0,64),20000)!);
  keys.forEach((key,at)=>{expect(writeStoredAnalysisV1(directory,root,key,{image:"x".repeat(4000)+at,nestedObservations:[]})).toBe(true);const file=join(directory,key.slice(0,2),key+".json.br");utimesSync(file,1000+at,1000+at);});
  const stale=join(directory,keys[0]!.slice(0,2),"dead.json.br.1.tmp");writeFileSync(stale,"x");utimesSync(stale,1,1);
  const size=(key:string)=>statSync(join(directory,key.slice(0,2),key+".json.br")).size,total=keys.reduce((sum,key)=>sum+size(key),0);
  expect(collectAnalysisStoreV1(directory,total+1,Date.now())).toBe(0);expect(existsSync(stale)).toBe(false);
  const removed=collectAnalysisStoreV1(directory,Math.floor(total*0.7),Date.now());
  expect(removed).toBeGreaterThan(0);
  const left=keys.filter(key=>existsSync(join(directory,key.slice(0,2),key+".json.br")));
  expect(left.length).toBeLessThan(6);expect(left).toEqual(keys.slice(6-left.length));
  expect(left.reduce((sum,key)=>sum+size(key),0)).toBeLessThanOrEqual(Math.floor(total*0.7*ANALYSIS_STORE_POLICY_V1.lowWaterPercent/100)+size(left[0]!));
 }finally{rmSync(directory,{recursive:true,force:true});}
});
