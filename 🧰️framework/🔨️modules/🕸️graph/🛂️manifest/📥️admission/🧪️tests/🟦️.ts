import {expect,test} from "bun:test";
import Ajv from "ajv";
import fg from "fast-glob";
import {mkdtempSync,mkdirSync,readFileSync,writeFileSync,symlinkSync,rmSync} from "node:fs";
import {join,dirname,relative} from "node:path";
import {findManifestFiles,readGraphManifestDocuments} from "../🟦️.ts";
import policy from "../🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
test("graph manifest admission excludes dependency owners before inspecting links and retains source link refusal",()=>{
 expect(new Ajv({strict:true}).compile(schema)(policy)).toBe(true);
 const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"ga-")),outside=join(root,"outside");mkdirSync(outside,{recursive:true});
 try{
  for(const name of fixture.files){const path=join(root,name);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,JSON.stringify({schema:"manifest",id:name}));}
  const dependency=join(root,fixture.dependencyLink);mkdirSync(dirname(dependency),{recursive:true});symlinkSync(outside,dependency,process.platform==="win32"?"junction":"dir");
  for(const link of fixture.outputLinks)symlinkSync(outside,join(root,link),process.platform==="win32"?"junction":"dir");
  const oracle=fg.sync("src/**/*manifest.json",{cwd:root,followSymbolicLinks:false,ignore:["**/node_modules/**","**/dist/**","**/📤️dist/**"]}).sort(),actual=findManifestFiles(root,["src"],[]).map(path=>relative(root,path).replaceAll("\\","/"));expect(oracle).toEqual(fixture.expected);expect(actual).toEqual(oracle);expect(findManifestFiles(root,[fixture.dependencyLink],[])).toEqual([]);
  const directories:string[]=[], missing:string[]=[], files=new Map<string,string>();
  const local=(name:string)=>relative(root,name).replaceAll("\\","/");
  const documents=readGraphManifestDocuments(root,false,["src","missing/nested"],[],name=>{
   const bytes=readFileSync(name,"utf8");files.set(local(name),bytes);return bytes;
  },(name,kind,suffix)=>{
    if(kind==="discovery") expect(suffix).toBe("manifest.json");
    (kind==="discovery"?directories:missing).push(local(name));
  });
  expect([...files.keys()]).toEqual(oracle);
  expect(documents.map(document=>document.id)).toEqual(oracle);
  expect(directories.sort()).toEqual(["src","src/direct","src/package"]);
  expect(missing).toEqual(["missing"]);
  symlinkSync(outside,join(root,fixture.sourceLink),process.platform==="win32"?"junction":"dir");expect(()=>findManifestFiles(root,["src"],[])).toThrow("symbolic link");
 }finally{rmSync(root,{recursive:true,force:true});}
});
