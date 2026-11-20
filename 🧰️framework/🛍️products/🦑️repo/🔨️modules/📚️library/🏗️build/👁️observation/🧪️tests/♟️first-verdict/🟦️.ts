import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import * as fs from "node:fs";
import {basename,join} from "node:path";
import Ajv from "ajv";
import {observedFieldV1,observeInference,projectRowsHoldV1,verifyStoredProjectRecordV1,withObservedWorldV1} from "../../🟨️.mjs";
import fixture from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

test("stored field diagnostics report the first judged observation without probing any field twice",async()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 using oracle=new Database(":memory:");
 const expected=oracle.query(`WITH facts(position,path,seen) AS (
   VALUES (1,'first.json',json_extract($fixture,'$.first[0]')),
          (2,'second.json',json_extract($fixture,'$.second[0]'))
  ) SELECT position,path,CASE seen
   WHEN 'changed' THEN 'field-value'
   WHEN 'unreadable' THEN 'field-source-unavailable'
   WHEN 'unparsable' THEN 'field-unparsable'
  END AS reason FROM facts WHERE seen<>'recorded' ORDER BY position LIMIT 1`);
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;
 if(!artifacts)throw Error("Private observation test directory required");
 const dir=fs.mkdtempSync(join(artifacts,"fv-")),first=join(dir,"first.json"),second=join(dir,"second.json"),recorded='{"value":1}';
 try{
  fs.writeFileSync(first,recorded);fs.writeFileSync(second,recorded);
  for(const [at,row] of fixture.entries()){
   const cachePath=join(dir,"np",`fields-${at}.json`),key=["fields",at],calls:string[]=[];
   await observeInference({cachePath,key},async()=>[observedFieldV1(first,"json","/value"),observedFieldV1(second,"json","/value")]);
   const here=new Map<string,string[]>([[first,[...row.first]],[second,[...row.second]]]);
   const verdict=await withObservedWorldV1({...fs,statSync:(path:fs.PathLike,options?:unknown)=>{
    const status=(fs.statSync as Function)(path,options);
    return here.has(String(path))?Object.assign(status,{mtimeMs:Date.now()}):status;
   },readFileSync:(path:fs.PathOrFileDescriptor,options?:unknown)=>{
    const readings=here.get(String(path));if(!readings)return (fs.readFileSync as Function)(path,options);
    calls.push(basename(String(path)));
    const seen=readings.shift();
    if(seen==="unreadable")throw Object.assign(Error("private read failure"),{code:"EACCES"});
    return seen==="recorded"?recorded:seen==="unparsable"?"{":'{"value":2}';
   }} as typeof fs,()=>verifyStoredProjectRecordV1({cachePath,key}));
   const reason=expected.get({$fixture:JSON.stringify(row)}) as {position:number;path:string;reason:string}|null;
   expect(calls,row.name).toEqual(reason?.position===1?["first.json"]:["first.json","second.json"]);
   expect(verdict.verdict,row.name).toBe(reason===null?"holds":"stale");
   if(reason===null)expect(verdict.staleRow).toBeUndefined();
   else{
    expect(verdict.staleRow,row.name).toMatchObject({kind:"field",path:join(dir,reason.path)});
    expect(verdict.staleRow?.why,row.name).toMatch(new RegExp(`^${reason.reason}; judge \\d+ of pid ${process.pid}$`,"u"));
   }
  }
 }finally{fs.rmSync(dir,{recursive:true,force:true});}
});

test("the production closure provider reports its first observed row or aggregate and preserves entry cancellation",async()=>{
 const {libraryBootstrap}=await import("../../../🕸️graph/📥️inference/🟨️.mjs");
 await libraryBootstrap;
 const {runClosureTakeV1}=await import("../../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
 const unknown={kind:"closure",path:JSON.stringify(["unopened-workspace","unknown-fixture-recipe",["digest"]]),fact:"true"};
 const malformed={kind:"closure",path:"{",fact:"true"},reported:unknown[][]=[];
 const judge=(rows:readonly typeof malformed[])=>projectRowsHoldV1(rows,{},()=>true,(...failure)=>reported.push(failure));
 expect(await judge([unknown,malformed])).toBe(false);
 expect(reported[0]).toEqual([malformed,"closure-query",2]);
 expect(reported[0]?.[0]).toBe(malformed);
 expect(await judge([unknown])).toBe(false);
 expect(reported[1]).toEqual([undefined,"closure-recipe-unavailable",1]);
 const reason=Error("private closure cancelled");
 await expect(runClosureTakeV1(()=>judge([unknown,malformed]),AbortSignal.abort(reason))).rejects.toBe(reason);
 expect(reported).toHaveLength(2);
});
