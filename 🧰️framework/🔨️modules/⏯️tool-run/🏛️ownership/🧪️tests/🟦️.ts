import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync,readdirSync} from "node:fs";
import {resolve} from "node:path";
import fixture from "../🧫️fixtures/🔣️.json";
import ts from "typescript";

const readsSpecific=(source:string)=>["/🛍️products/","/✏️s/","/🌎️hub/"].some(prefix=>source.includes(prefix));
const nativeSources=(root:string):string[]=>readdirSync(root,{withFileTypes:true}).flatMap(entry=>{
 if(["node_modules","target","🗑️generated"].includes(entry.name))return [];
 const path=resolve(root,entry.name);
 return entry.isDirectory()?nativeSources(path):entry.isFile()&&entry.name.endsWith(".rs")?[path]:[];
});
const scriptSources=(root:string):string[]=>readdirSync(root,{withFileTypes:true}).flatMap(entry=>{
 if(["node_modules","target","dist","🗑️generated"].includes(entry.name))return [];
 const path=resolve(root,entry.name);
 return entry.isDirectory()?scriptSources(path):entry.isFile()&&entry.name.endsWith(".ts")?[path]:[];
});
const normalized=(path:string)=>path.replaceAll("\\","/");
const isWithin=(path:string,boundary:string)=>path===boundary||path.startsWith(boundary+"/");
const isGeneralImport=(path:string,boundary:string,excluded:string)=>isWithin(normalized(path),normalized(boundary))&&!isWithin(normalized(path),normalized(excluded));

test("General ToolRun native and selected source laws remain independent of Specific modules and fixtures",()=>{
 expect(fixture.version).toBe(1);
 const db=new Database(":memory:");try{
  const oracle=db.query("SELECT (instr(?,'/🛍️products/')>0 OR instr(?,'/✏️s/')>0 OR instr(?,'/🌎️hub/')>0) AS specific");
  for(const row of fixture.cases){expect(readsSpecific(row.source)).toBe(row.specific);expect(oracle.get(row.source,row.source,row.source)).toEqual({specific:Number(row.specific)});}
  const root=resolve(import.meta.dir,"../.."),sources=[...nativeSources(root),...fixture.ownedSourceFiles.map(path=>resolve(root,path))];expect(sources.length).toBeGreaterThan(0);
  for(const path of sources){const source=readFileSync(path,"utf8");expect(readsSpecific(source),path).toBe(fixture.generalReadsSpecific);expect(oracle.get(source,source,source),path).toEqual({specific:Number(fixture.generalReadsSpecific)});}
 }finally{db.close();}
});

test("General ToolRun TypeScript imports remain inside the neutral Framework owner",()=>{
 expect(fixture.version).toBe(1);
 const db=new Database(":memory:");try{
  const oracle=db.query("SELECT ((?1=?2 OR instr(?1,?2||'/')=1) AND NOT (?1=?3 OR instr(?1,?3||'/')=1)) AS allowed");
  const law=fixture.moduleImports;
  for(const row of law.cases){expect(isGeneralImport(row.resolved,law.boundary,law.excludedBoundary)).toBe(row.allowed);expect(oracle.get(row.resolved,law.boundary,law.excludedBoundary)).toEqual({allowed:Number(row.allowed)});}
  const boundary=resolve(import.meta.dir,"../../../.."),excluded=resolve(boundary,"🛍️products"),sources=scriptSources(resolve(import.meta.dir,"../.."));expect(sources.length).toBeGreaterThan(0);
  for(const path of sources){
   const imports=ts.preProcessFile(readFileSync(path,"utf8"),true).importedFiles;
   for(const imported of imports){
    if(!imported.fileName.startsWith("."))continue;
    const target=ts.resolveModuleName(imported.fileName,path,{module:ts.ModuleKind.ESNext,moduleResolution:ts.ModuleResolutionKind.Bundler,resolveJsonModule:true,allowImportingTsExtensions:true},ts.sys).resolvedModule;
    expect(target,path+" -> "+imported.fileName).toBeDefined();const resolved=target!.resolvedFileName;
    expect(isGeneralImport(resolved,boundary,excluded),path+" -> "+resolved).toBe(true);
    expect(oracle.get(normalized(resolved),normalized(boundary),normalized(excluded)),path+" -> "+resolved).toEqual({allowed:1});
   }
  }
 }finally{db.close();}
});
