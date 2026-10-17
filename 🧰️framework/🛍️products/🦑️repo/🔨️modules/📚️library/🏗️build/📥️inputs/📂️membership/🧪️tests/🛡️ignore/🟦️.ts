import {test,expect} from "bun:test";
import {readFileSync,mkdirSync,mkdtempSync,writeFileSync,rmSync} from "node:fs";
import {join,resolve,dirname} from "node:path";
import {createRequire} from "node:module";
import ignore from "ignore";
import corpus from "./🔣️.json";
import ts from "typescript";
import {cleanIsBuildArtifactDirName,cleanIsCargoTargetDirName} from "../../../../../🧼️workspace-cleanup/🛡️protection/🟦️.ts";

test("authored semantic target Source stays in the real Nx map while compiler output remains ignored",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Original test artifact directory required");mkdirSync(output,{recursive:true});const root=mkdtempSync(join(resolve(output),"ig-")),patterns=readFileSync(resolve(".gitignore"),"utf8"),reference=ignore().add(patterns),require=createRequire(import.meta.url),native=require("nx/src/native");
 try{
  writeFileSync(join(root,".gitignore"),patterns);mkdirSync(join(root,".git"));
  for(const row of corpus.cases){const path=join(root,row.path);if(path.length>256)throw Error("Portable Source fixture path exceeds capacity");mkdirSync(dirname(path),{recursive:true});writeFileSync(path,"export const source=1;\n");expect(reference.ignores(row.path),row.path).toBe(row.ignored);}
  const context=new native.WorkspaceContext(root,join(root,".nx"));context.glob(["**/*"]);await new Promise<void>(resolve=>setImmediate(resolve));const inventory=context.getWorkspaceFiles({domain:corpus.owner}),present=new Set([...Object.values(inventory.projectFileMap).flatMap((rows:any)=>rows.map((row:{file:string})=>row.file)),...inventory.globalFiles.map((row:{file:string})=>row.file)]);
  for(const row of corpus.cases){expect(present.has(row.path),row.path).toBe(!row.ignored);if(!row.ignored){expect(inventory.projectFileMap.domain.some((file:{file:string})=>file.file===row.path)).toBe(true);}}
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("actual TypeScript excludes preserve semantic target Source and exclude owned compiler output",()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Original test artifact directory required");mkdirSync(output,{recursive:true});
 for(const owner of ["🦑️repo","💻️os"]){
  const policy=JSON.parse(readFileSync(resolve("🧰️framework/🛍️products",owner,"tsconfig.json"),"utf8")),root=mkdtempSync(join(resolve(output),"ts-")),project=join(root,"product");mkdirSync(project);
  try{
   const files=corpus.cases.map(row=>({row,path:join(row.path.startsWith("domain/")?project:root,row.path.replace(/\.[^/.]+$/u,"")+".ts")}));
   for(const {path} of files){if(path.length>256)throw Error("Portable Source fixture path exceeds capacity");mkdirSync(dirname(path),{recursive:true});writeFileSync(path,"export const source=1;\n");}
   const parsed=ts.parseJsonConfigFileContent({...policy,extends:undefined,files:undefined,compilerOptions:{},include:["**/*.ts"]},ts.sys,project);
   for(const {row,path} of files)expect(parsed.fileNames.includes(path),owner+":"+row.path).toBe(!row.ignored);
  }finally{rmSync(root,{recursive:true,force:true});}
 }
});

test("cleanup name classification separates semantic target owners from Cargo output without removing files",()=>{
 for(const row of corpus.cases){const directories=row.path.split("/").slice(0,-1);expect(directories.filter(cleanIsCargoTargetDirName),row.path).toEqual(row.cargoDirectories);expect(directories.filter(cleanIsBuildArtifactDirName),row.path).toEqual(row.buildDirectories);}
});
