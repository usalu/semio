/** 📝️ Contract: unmirror manifest edits list the declared dependency tables, insert path dependencies in byte order (creating the section when missing), and remove single-line dependency entries by plain line edits that keep hand-formatted manifests intact; the corpus is language-agnostic and `@iarna/toml` is the independent third-party oracle for the declared tables, the validity of every edited manifest and the survival of everything else. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {parse} from "@iarna/toml";
import {addDependencyLines,declaredDependencies,removeDependencyLines,type DependencyAddition} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const plain=(map:Map<string,Set<string>>):Record<string,string[]>=>Object.fromEntries([...map.entries()].map(([name,sections])=>[name,[...sections].sort()]));
const dependencyTables=(document:any):Record<string,string[]>=>{
 const found:Record<string,Set<string>>={};
 const visit=(table:any,path:string):void=>{
  if(table===null||typeof table!=="object")return;
  for(const[key,value]of Object.entries<any>(table)){
   if(/^(?:dev-|build-)?dependencies$/u.test(key)&&value&&typeof value==="object"){for(const name of Object.keys(value))(found[name]??=new Set()).add(path?path+"."+key:key);}
   else if(key==="target"&&value&&typeof value==="object"){for(const[cfg,inner]of Object.entries<any>(value))for(const section of Object.keys(inner)){if(/^(?:dev-|build-)?dependencies$/u.test(section))for(const name of Object.keys(inner[section]))(found[name]??=new Set()).add("target.'"+cfg+"'."+section);}}
   else if(key==="workspace"&&value&&typeof value==="object")visit(value,"workspace");
  }
 };
 visit(document,"");return Object.fromEntries(Object.entries(found).map(([name,sections])=>[name,[...sections].sort()]));
};
for(const row of corpus.declared)test("declared dependencies: "+row.name,()=>{expect<unknown>(plain(declaredDependencies(row.manifest))).toEqual(row.expected);});
for(const row of corpus.declaredDefects)test.failing("declared dependencies defect: "+row.name,()=>{expect(plain(declaredDependencies(row.manifest))).toEqual(row.expected);});
test("@iarna/toml reads the same declared dependencies for every well-formed manifest",()=>{for(const row of corpus.declared.filter(item=>item.manifest!==""))expect<unknown>(dependencyTables(parse(row.manifest)),row.name).toEqual(row.expected);for(const row of corpus.declaredDefects)expect(dependencyTables(parse(row.manifest)),row.name).toEqual(row.expected);});
for(const row of corpus.add)test("add dependency lines: "+row.name,()=>{expect(addDependencyLines(row.manifest,row.additions as DependencyAddition[])).toBe(row.expected);});
test("added dependencies are valid TOML path dependencies and every earlier dependency survives (@iarna/toml)",()=>{
 for(const row of corpus.add){
  const before=parse(row.manifest) as any,after=parse(addDependencyLines(row.manifest,row.additions as DependencyAddition[])) as any;
  for(const addition of row.additions)expect(after[addition.section][addition.name],row.name+" "+addition.name).toEqual({path:addition.path});
  for(const section of["dependencies","dev-dependencies"])for(const[name,value]of Object.entries<any>(before[section]??{}))expect(after[section][name],row.name+" "+name).toEqual(value);
  expect(after.package).toEqual(before.package);
 }
});
for(const row of corpus.remove)test("remove dependency lines: "+row.name,()=>{expect(removeDependencyLines(row.manifest,new Set(row.names))).toBe(row.expected);});
for(const row of corpus.removeDefects)test.failing("remove dependency lines defect: "+row.name,()=>{expect(removeDependencyLines(row.manifest,new Set(row.names))).toBe(row.expected);});
test("removal leaves valid TOML without the removed dependencies (@iarna/toml)",()=>{
 for(const row of corpus.remove.filter(item=>!item.manifest.includes("{\n"))){const after=parse(removeDependencyLines(row.manifest,new Set(row.names))) as any;for(const section of["dependencies","dev-dependencies"])for(const name of row.names)if(!row.name.includes("build"))expect(Object.keys(after[section]??{}),row.name).not.toContain(name);}
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
