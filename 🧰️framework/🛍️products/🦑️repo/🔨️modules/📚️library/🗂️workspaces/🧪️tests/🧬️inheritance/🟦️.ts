import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{dirname,resolve,relative}from"node:path";
import * as TOML from"@iarna/toml";
import picomatch from"picomatch";
import Ajv from"ajv";
import law from"./🔣️.json";
import schema from"./🧬️schema/🔣️.json";
const root=process.cwd(),plain=(value:unknown)=>JSON.parse(JSON.stringify(value)),parsers=[(text:string)=>plain(TOML.parse(text)),(text:string)=>plain(Bun.TOML.parse(text))];
function inspect(parse:(text:string)=>any,remove=false){
 const read=(path:string)=>parse(readFileSync(resolve(root,path),"utf8")),missing:{manifest:string,table:string,name:string}[]=[];
 for(const member of law.chain){
  const manifest=read(member.manifest),workspace=resolve(root,dirname(member.manifest),manifest.package.workspace);
  expect(relative(root,resolve(workspace,"Cargo.toml")).replaceAll("\\","/")).toBe(member.workspace);
  const owner=read(member.workspace),memberPath=relative(dirname(resolve(root,member.workspace)),dirname(resolve(root,member.manifest))).replaceAll("\\","/");
  expect(owner.workspace.members.some((pattern:string)=>picomatch.isMatch(memberPath,pattern,{dot:true}))).toBe(true);
  expect((owner.workspace.exclude??[]).some((pattern:string)=>picomatch.isMatch(memberPath,pattern,{dot:true}))).toBe(false);
  if(remove&&member.workspace==="Cargo.toml")delete owner.workspace.dependencies[law.dependency.name];
  let count=0;
  const tables=(value:any,prefix="")=>{for(const[key,table]of Object.entries(value)){const coordinate=prefix?prefix+"."+key:key;if(["dependencies","dev-dependencies","build-dependencies"].includes(key)){for(const[name,entry]of Object.entries(table as any))if((entry as any)?.workspace===true){count++;if(!Object.hasOwn(owner.workspace.dependencies,name))missing.push({manifest:member.manifest,table:coordinate,name});}}else if(key==="target")for(const[target,contents]of Object.entries(table as any))tables(contents,"target."+target);}};
  tables(manifest);expect(count,member.manifest).toBe(member.inherited);
 }
 return missing;
}
test("neutral inheritance corpus has a closed schema",()=>expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true));
test("both independent TOML parsers reproduce the exact missing inherited dependency",()=>{for(const parse of parsers)expect(inspect(parse,true)).toEqual(law.baselineMissing);});
test("all inherited chain dependencies resolve to their declared workspace and canonical diagnostic owner",()=>{
 for(const parse of parsers){
  expect(inspect(parse)).toEqual([]);
  const read=(path:string)=>parse(readFileSync(resolve(root,path),"utf8")),declaration=read("Cargo.toml").workspace.dependencies[law.dependency.name];
  expect(declaration).toEqual({path:law.dependency.path});
  const manifest=read(law.dependency.path+"/Cargo.toml");
  expect(manifest.package.name).toBe(law.dependency.name);
  expect(relative(root,resolve(root,law.dependency.path,manifest.package.workspace,"Cargo.toml")).replaceAll("\\","/")).toBe(law.dependency.workspace);
  const owner=read(law.dependency.workspace),member=relative(dirname(resolve(root,law.dependency.workspace)),resolve(root,law.dependency.path)).replaceAll("\\","/");
  expect(owner.workspace.members.some((pattern:string)=>picomatch.isMatch(member,pattern,{dot:true}))).toBe(true);
  expect((owner.workspace.exclude??[]).some((pattern:string)=>picomatch.isMatch(member,pattern,{dot:true}))).toBe(false);
 }
});
