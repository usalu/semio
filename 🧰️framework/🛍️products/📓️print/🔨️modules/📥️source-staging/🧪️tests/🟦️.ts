import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{printSourceStagingPlanV1,printCompilerName}from"../🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import corpus from"./🔣️.json"with{type:"json"};
test("Print source staging uses closed bounded DATA and independent Ajv SQLite filename vectors",()=>{const validate=new Ajv({strict:true}).compile(schema),db=new Database(":memory:");try{for(const row of corpus.valid){expect(validate(row)).toBe(true);expect(JSON.stringify(printSourceStagingPlanV1(row))).toBe(JSON.stringify(row));}for(const row of corpus.invalid)expect(()=>printSourceStagingPlanV1(row)).toThrow();db.exec("create table names(source text primary key,staged text)");for(const row of corpus.names)db.query("insert into names values(?,?)").run(row.source,row.staged);for(const row of db.query<{source:string,staged:string},[]>("select * from names order by source").all())expect(printCompilerName(row.source)).toBe(row.staged);}finally{db.close();}});

test("Print Source staging captures plain entry DATA without invoking accessors",()=>{let reads=0;const value={version:1,directory:"source",get entries(){reads++;return["document.tex"];}};expect(()=>printSourceStagingPlanV1(value)).toThrow();expect(reads).toBe(0);});

import{printSourceReferenceMapV1,printSourceReferenceTextV1}from"../🟦️.ts";
import macro from"../../../🎮️commands/🧪️print-pipeline-verification/🧪️tests/🧫️fixtures/🖨️macro-staging/🔣️.json"with{type:"json"};
import{posix}from"node:path";
test("actual Source reference projection preserves the neutral macro roster and SQLite paths",()=>{const names=printSourceReferenceMapV1(Object.keys(macro.inputs)),database=new Database(":memory:");try{database.exec("create table outputs(source text primary key,destination text)");for(const[name,text]of Object.entries(macro.inputs)){const path=name.split("/").map(printCompilerName).join("/"),expected=Object.entries(macro.outputs).find(([key])=>key===path)?.[1];if(expected===undefined)throw Error("Authored macro output required");database.query("insert into outputs values(?,?)").run(name,path);expect(posix.normalize(path)).toBe(path);expect(/\.(?:tex|bib|sty|cls)$/.test(name)?printSourceReferenceTextV1(text,names):text).toBe(expected);}expect(database.query<{count:number},[]>("select count(distinct destination) count from outputs").get()?.count).toBe(Object.keys(macro.inputs).length);for(const row of macro.collisions)expect(()=>printSourceReferenceMapV1(row.entries)).toThrow();}finally{database.close();}});
