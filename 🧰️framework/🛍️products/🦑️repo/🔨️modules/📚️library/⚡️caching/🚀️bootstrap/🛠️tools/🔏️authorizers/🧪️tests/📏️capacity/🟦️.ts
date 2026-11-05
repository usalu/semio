import{expect,test}from"bun:test";
import{mkdtemp,mkdir,writeFile,rm}from"node:fs/promises";
import{join,relative,resolve}from"node:path";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{captureNxAuthorizerSourcesV1}from"../../🟦️.ts";
import schema from"../../🧬️schema/🔣️.json";
import fixture from"./🔣️.json";

test("actual Source byte refusal retains bounded origin and independent largest-source census",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Ticket-owned test artifacts required");
 await mkdir(artifacts,{recursive:true});const area=await mkdtemp(join(artifacts,"cap-")),root=process.cwd(),entry=join(area,"entry.ts"),paths=Array.from({length:fixture.parts},(_,index)=>join(area,String(index).padStart(2,"0")+".ts")),bytes='export default '+JSON.stringify("x".repeat(fixture.partBytes-18))+';',source=paths.map((_,index)=>'import part'+index+' from "./'+String(index).padStart(2,"0")+'.ts";').join("\n"),rows=[{path:relative(root,entry).replaceAll("\\","/"),bytes:Buffer.byteLength(source)},...paths.map(path=>({path:relative(root,path).replaceAll("\\","/"),bytes:Buffer.byteLength(bytes)}))];
 try{
  await writeFile(entry,source);await Promise.all(paths.map(path=>writeFile(path,bytes)));let failure:any;
  try{await captureNxAuthorizerSourcesV1(root,relative(root,entry).replaceAll("\\","/"),AbortSignal.timeout(30000),{installed:true});}catch(error){failure=error;}
  expect(failure?.message).toContain("Authorizer source closure capacity exceeded");expect(failure?.diagnostic).toBeDefined();
  const diagnostic=failure.diagnostic,ajv=new Ajv({strict:true});expect(ajv.validate(schema.$defs.SourceCapacityDiagnostic,diagnostic),JSON.stringify(ajv.errors)).toBe(true);
  const db=new Database(":memory:");try{const expected=db.query("select json_extract(value,'$.path') path, json_extract(value,'$.bytes') bytes from json_each(?) order by bytes desc,path collate binary limit ?").all(JSON.stringify(rows),fixture.maximumLargest);expect(diagnostic.largest).toEqual(expected);expect(diagnostic.bytes).toBe((db.query("select sum(json_extract(value,'$.bytes')) bytes from json_each(?)").get(JSON.stringify(rows))as{bytes:number}).bytes);}finally{db.close();}
  const census=failure.sourceCensus;expect(census).toBeDefined();expect(census.length).toBe(fixture.expectedFiles);expect(census.length).toBeLessThanOrEqual(2049);expect(Object.isFrozen(census)).toBe(true);
  expect(census.map((row:any)=>({path:row.path,bytes:row.bytes})).sort((a:any,b:any)=>a.path<b.path?-1:1)).toEqual([...rows].sort((a,b)=>a.path<b.path?-1:1));
  expect(census.every((row:any)=>Object.isFrozen(row)&&row.origin===(row.path===rows[0].path?"entry":"import")&&row.importer===(row.path===rows[0].path?null:rows[0].path))).toBe(true);
  expect(Object.getOwnPropertyDescriptor(failure,"sourceCensus")).toMatchObject({enumerable:false,writable:false,configurable:false});expect(JSON.stringify(failure)).not.toContain("sourceCensus");
  expect(diagnostic.files).toBe(fixture.expectedFiles);expect(diagnostic.chain).toEqual([diagnostic.path,rows[0].path]);expect(diagnostic.origin).toBe("import");expect(rows.some(row=>row.path===diagnostic.path)).toBe(true);expect(JSON.stringify(diagnostic).length).toBeLessThan(4096);
 }finally{await rm(area,{recursive:true,force:true});}
},45000);
