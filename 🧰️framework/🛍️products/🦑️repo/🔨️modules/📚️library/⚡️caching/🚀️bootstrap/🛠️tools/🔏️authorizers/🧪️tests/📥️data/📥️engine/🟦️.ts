import{test,expect}from"bun:test";import{readFile}from"node:fs/promises";import{relative,join}from"node:path";import{createHash}from"node:crypto";import{Database}from"bun:sqlite";import Ajv from"ajv";
import{captureNxAuthorizerSourcesV1}from"../../../🟦️.ts";
import schema from"../../../🧬️schema/🔣️.json";
import{repositoryCommandKernelV1}from"../../../../../../../🏗️build/⌨️command/🧭️selection/🟦️.ts";
import fixture from"./🔣️.json";
test("actual runtime command captures the private engine schema as original DATA",async()=>{
 expect(new Ajv({strict:true}).validate(schema.$defs.RuntimeCompanionCase,fixture)).toBe(true);
 const root=process.cwd(),entry=relative(root,repositoryCommandKernelV1(["run","@semio-tech/mit-bestand-demonstrator:canonical-architecture"]).entry).replaceAll("\\","/"),captured=await captureNxAuthorizerSourcesV1(root,entry,AbortSignal.timeout(60000),{installed:true}),db=new Database(":memory:");
 try{const row=db.query("select json_extract(value,'$.path') path,json_extract(value,'$.sha256') sha256 from json_each(?) where json_extract(value,'$.path')=?").get(JSON.stringify(captured.manifest.sources),fixture.path);expect(row).toEqual({path:fixture.path,sha256:createHash("sha256").update(await readFile(join(root,fixture.path))).digest("hex")});expect(captured.manifest.sources.some(row=>row.path===fixture.excluded)).toBe(false);expect(schema.properties.sources.maxItems).toBe(2048);}finally{db.close();}
},65000);
