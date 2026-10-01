import {test,expect} from "bun:test";
import Ajv from "ajv";
import glob from "fast-glob";
import {mkdtempSync,mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {parseBunWorkspaceDeclaration,discoverBunWorkspaces,bunWorkspacePackages,bunWorkspaceNativePatterns} from "../🟦️.ts";
const corpus=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json",import.meta.url),"utf8")),validate=new Ajv({strict:true}).compile(JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json",import.meta.url),"utf8")));
const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw new Error("SEMIO_TEST_ARTIFACT_DIR is required");mkdirSync(output,{recursive:true});
const put=(root:string,path:string,value:any):void=>{mkdirSync(join(root,path,".."),{recursive:true});writeFileSync(join(root,path),JSON.stringify(value));};
const scope=(name:string,members:string[],owners:string[]=[]):any=>({name,private:true,workspaces:members,semio:{workspace:{schemaVersion:1,members,owners}}});
test("portable Bun source declaration admits the same closed vectors as Ajv",()=>{for(const row of corpus.accepted){expect(validate(row)).toBe(true);expect(parseBunWorkspaceDeclaration(row)).toEqual(row);}for(const row of corpus.rejected){expect(validate(row)).toBe(false);expect(()=>parseBunWorkspaceDeclaration(row)).toThrow();}});
for(const row of corpus.removal)test(`native Bun selected scope survives removal ${JSON.stringify(row.removed)} and refuses invalid retained consumers`,()=>{const root=mkdtempSync(join(output!,"bun-owner-removal-"));put(root,"package.json",scope("workspace",["framework/**"],["*/package.json"]));put(root,"framework/kernel/package.json",{name:"neutral-kernel",version:"1.0.0"});
 if(!row.removed.includes("specific")){put(root,"specific/package.json",scope("specific-workspace",["../framework/**","models/**"]));put(root,"specific/models/domain/package.json",{name:"specific-model",version:"1.0.0",dependencies:{"neutral-kernel":"workspace:*"}});mkdirSync(join(root,"specific/models/ghost"),{recursive:true});}
 if(!row.removed.includes("product")){put(root,"product/package.json",scope("product-workspace",["../framework/**","../specific/models/**","apps/**"]));put(root,"product/apps/app/package.json",{name:"product-app",version:"1.0.0",dependencies:{"specific-model":"workspace:*"}});}
 for(const owner of discoverBunWorkspaces(root)){const native=Bun.spawnSync(["bun","install","--ignore-scripts"],{cwd:join(root,owner.directory),stdout:"pipe",stderr:"pipe"});expect(native.exitCode===0).toBe(owner.directory==="product"?row.productValid:true);const expected=owner.declaration.members.flatMap(p=>glob.sync(p+"/package.json",{cwd:join(root,owner.directory),onlyFiles:true,followSymbolicLinks:false})).map(p=>join(owner.directory,p).replaceAll("\\","/").replace(/\/package.json$/,""));expect(bunWorkspacePackages(root,owner)).toEqual([...new Set(expected)].sort((a,b)=>a.localeCompare(b)));}
});

test("native patterns refuse unrelated negative admission while retaining owner-bound payloads",()=>{const root=mkdtempSync(join(output!,"bun-source-negative-"));put(root,"package.json",scope("workspace",["framework/**"]));put(root,"framework/kernel/package.json",{name:"kernel",version:"1.0.0"});const owner=discoverBunWorkspaces(root)[0]!;expect(bunWorkspaceNativePatterns(root,owner)).toEqual(["framework/**"]);const row=JSON.parse(readFileSync(join(root,"package.json"),"utf8"));row.workspaces.push("!framework/kernel");put(root,"package.json",row);expect(()=>discoverBunWorkspaces(root)).toThrow();});
