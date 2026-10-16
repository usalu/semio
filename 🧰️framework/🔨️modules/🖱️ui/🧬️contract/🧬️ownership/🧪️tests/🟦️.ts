import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{resolve}from"node:path";
import Ajv from"ajv";
import TOML from"@iarna/toml";
import fixture from"./🔣️.json" with{type:"json"};
import schema from"../🧬️schema/🔣️.json" with{type:"json"};
import{readRustSourceClosureV1}from"@semio-tech/process/artifacts/native-build/source";
test("semantic UI Value contract owns its exact direct Value dependency",async()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const ui=resolve(import.meta.dir,"../.."),root=resolve(ui,"../../../.."),directory=resolve(ui,"📦️packages/🦀️rust"),manifest=TOML.parse(readFileSync(resolve(directory,"Cargo.toml"),"utf8"))as any;
 expect(manifest.package.name).toBe(fixture.owner);expect(manifest.dependencies[fixture.valueOwner]).toBeDefined();expect(manifest.dependencies[fixture.forbiddenDependency]).toBeUndefined();
 const cfg=['target_arch=aarch64','target_os=macos','test'],rows=await readRustSourceClosureV1([resolve(directory,manifest.lib.path)],root,cfg,async file=>{try{return readFileSync(file);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return null;throw error;}},{manifestDirectory:directory,generated:()=>{throw Error("UI ownership refuses generated Source mounts");}});
 expect(rows.length).toBeGreaterThan(1);for(const row of rows)if(row.kind==="source")expect(readFileSync(resolve(root,row.path),"utf8")).not.toContain(fixture.forbiddenPath);
 console.log("[DEBUG] Complete authored UI Source closure uses the exact canonical Value owner without Replication");
});
