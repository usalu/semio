import{test,expect}from"bun:test";
import{mkdtempSync,readFileSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import Ajv from"ajv";
import schema from"../🧬️schema/🔣️.json";
import corpus from"./🔣️.json";
import{causeKeyV1,collectErrorsDirectoryV1,createErrorCollectorV1,collectTextsV1}from"../🟨️.mjs";

/** 🧾️ Law of the diagnostic collect-errors mode: causes group, every failure is recorded, final errors are never swallowed, the report validates against the schema in EN and DE and the run always ends with BUILD_COLLECTED_ERRORS. */
test("collect-errors normalizes causes like the independent model",()=>{
 for(const row of corpus.causes)expect(causeKeyV1(row.message)).toBe(row.cause);
});

test("collect-errors mode is selected by the environment only",()=>{
 expect(collectErrorsDirectoryV1({})).toBeUndefined();
 expect(collectErrorsDirectoryV1({SEMIO_GRAPH_COLLECT_ERRORS:"0"})).toBeUndefined();
 expect(collectErrorsDirectoryV1({SEMIO_GRAPH_COLLECT_ERRORS:"1"})).toBe(join(tmpdir(),"semio-collect-errors"));
 expect(collectErrorsDirectoryV1({SEMIO_GRAPH_COLLECT_ERRORS:"/x/y"})).toBe("/x/y");
});

test("collect-errors records every failure, rethrows final ones and ends with a coded error and a valid EN/DE report",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"collect-law-")),written:string[]=[];
 try{
  const collector=createErrorCollectorV1({directory,workspaceRoot:"/w",texts:collectTextsV1,write:text=>{written.push(text);}});
  for(const f of corpus.failures)expect(await collector.guard({project:f.project,root:f.root,phase:f.phase},async()=>{throw Object.assign(Error(f.message),(f as {code?:string}).code?{code:(f as {code?:string}).code}:{});})).toBe(false);
  expect(await collector.guard({project:"ok",root:"r/ok",phase:"project"},async()=>{})).toBe(true);
  for(const code of["BUILD_AUTHORITY_CHANGED","BUILD_COLLECTED_ERRORS"])await expect(collector.guard({project:"p",root:"r",phase:"project"},async()=>{throw Object.assign(Error("final"),{code});})).rejects.toThrow("final");
  await expect(collector.guard({project:"p",root:"r",phase:"project"},async()=>{throw Error("Stale loaded x; restart required");})).rejects.toThrow("restart required");
  expect(collector.rows().length).toBe(corpus.failures.length);
  let thrown:{code?:string;message?:string}|undefined;
  try{await collector.finish({projects:10});}catch(error){thrown=error as {code?:string;message?:string};}
  expect(thrown?.code).toBe("BUILD_COLLECTED_ERRORS");
  expect(thrown?.message).toContain("4 of 10");expect(thrown?.message).toContain("4 von 10");
  const report=JSON.parse(readFileSync(join(directory,"report.json"),"utf8")),ajv=new Ajv({strict:false});
  expect(ajv.validate({...schema,$ref:"#/definitions/Report"},report)).toBe(true);
  expect(report.groups.map((g:{cause:string;count:number;projects:{project:string}[]})=>({cause:g.cause,count:g.count,projects:g.projects.map(p=>p.project)}))).toEqual(corpus.expectGroups);
  expect(readFileSync(join(directory,"summary.en.txt"),"utf8")).toContain("4 of 10 projects failed in 2 causes");
  expect(readFileSync(join(directory,"summary.de.txt"),"utf8")).toContain("4 von 10 Projekten fehlgeschlagen in 2 Ursachen");
  expect(written.length).toBe(2);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("collect-errors does not swallow a cancelled run",async()=>{
 const controller=new AbortController(),directory=mkdtempSync(join(tmpdir(),"collect-law-"));
 try{
  const collector=createErrorCollectorV1({directory,workspaceRoot:"/w",texts:collectTextsV1,signal:controller.signal,write:()=>{}});
  controller.abort();
  await expect(collector.guard({project:"p",root:"r",phase:"project"},async()=>{throw Error("boom");})).rejects.toThrow("boom");
  expect(collector.rows().length).toBe(0);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
