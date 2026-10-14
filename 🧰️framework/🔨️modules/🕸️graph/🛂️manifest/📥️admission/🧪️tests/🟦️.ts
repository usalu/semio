import {expect,test} from "bun:test";
import Ajv from "ajv";
import fg from "fast-glob";
import {mkdtempSync,mkdirSync,writeFileSync,symlinkSync,rmSync} from "node:fs";
import {join,dirname,relative} from "node:path";
import {findManifestFiles} from "../🟦️.ts";
import policy from "../🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
test("graph manifest admission excludes dependency owners before inspecting links and retains source link refusal",()=>{
 expect(new Ajv({strict:true}).compile(schema)(policy)).toBe(true);
 const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"ga-")),outside=join(root,"outside");mkdirSync(outside,{recursive:true});
 try{
  for(const name of fixture.files){const path=join(root,name);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,JSON.stringify({schema:"manifest",id:name}));}
  const dependency=join(root,fixture.dependencyLink);mkdirSync(dirname(dependency),{recursive:true});symlinkSync(outside,dependency,process.platform==="win32"?"junction":"dir");
  const oracle=fg.sync("src/**/*manifest.json",{cwd:root,followSymbolicLinks:false,ignore:["**/node_modules/**"]}).sort(),actual=findManifestFiles(root,["src"],[]).map(path=>relative(root,path).replaceAll("\\","/"));expect(oracle).toEqual(fixture.expected);expect(actual).toEqual(oracle);expect(findManifestFiles(root,[fixture.dependencyLink],[])).toEqual([]);
  symlinkSync(outside,join(root,fixture.sourceLink),process.platform==="win32"?"junction":"dir");expect(()=>findManifestFiles(root,["src"],[])).toThrow("symbolic link");console.log("[DEBUG] Graph manifest dependency exclusion precedes link admission; fast-glob oracle agrees and source links remain refused");
 }finally{rmSync(root,{recursive:true,force:true});}
});
