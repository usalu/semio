#!/usr/bin/env bun
import {createHash} from "node:crypto";
import {readFileSync} from "node:fs";
import {dirname,isAbsolute,join,relative,resolve,sep} from "node:path";
import {cargoPreparationInputV1,cargoPreparationSourceRootV1,cargoPreparationProgramRootV1,type CargoPreparationInputV1,type CargoPreparationResolutionV1} from "./🧾️custody/🟦️.ts";

/** 📚️ Captures a program closure in one fresh resolver process. */
function captureProgram(root:string,entry:string):Readonly<{sourceRoot:string;sources:readonly CargoPreparationInputV1[];resolverInputs:readonly CargoPreparationInputV1[];inputs:readonly CargoPreparationInputV1[];resolutions:readonly CargoPreparationResolutionV1[]}> {
 const sourceRoot=cargoPreparationSourceRootV1(),files=new Map<string,CargoPreparationInputV1>(),configuration=new Map<string,CargoPreparationInputV1>(),resolverConfiguration=new Map<string,CargoPreparationInputV1>(),pending=new Set([entry,import.meta.path]),resolutions:CargoPreparationResolutionV1[]=[];
 for(const path of pending){
  if(files.has(path))continue;const ownerRoot=cargoPreparationProgramRootV1(root,sourceRoot,path),input=cargoPreparationInputV1(ownerRoot,path,"file");if(!input.sha256)throw Error(`Preparation program source is missing: ${path}`);const bytes=readFileSync(path);files.set(path,{...input,sha256:createHash("sha256").update(bytes).digest("hex")});
  let owner=dirname(path);while(true){for(const name of ["package.json","tsconfig.json","bunfig.toml"]){const config=resolve(owner,name);(ownerRoot===root?configuration:resolverConfiguration).set(config,cargoPreparationInputV1(ownerRoot,config,"file"));}if(owner===ownerRoot)break;owner=dirname(owner);}
  if(!/\.[cm]?[jt]sx?$/.test(path))continue;
  const imports=new Bun.Transpiler({loader:path.endsWith("tsx")?"tsx":path.endsWith("jsx")?"jsx":path.endsWith("ts")?"ts":"js"}).scanImports(bytes.toString("utf8").replace(/^#![^\r\n]*/,""));
  for(const edge of imports){if(edge.kind!=="import-statement"&&edge.kind!=="require-call"||edge.path.startsWith("node:")||edge.path.startsWith("bun:"))continue;if([...edge.path].some(character=>character.charCodeAt(0)>255))throw Error("Unknown Bun import scanner representation");const encoded=Buffer.from(edge.path,"latin1"),specifier=new TextDecoder("utf-8",{fatal:true}).decode(encoded);if(!Buffer.from(specifier).equals(encoded))throw Error("Unknown Bun import scanner encoding");const selected=Bun.resolveSync(specifier,dirname(path)),selectedRoot=cargoPreparationProgramRootV1(root,sourceRoot,selected),local=relative(selectedRoot,selected);resolutions.push({source:path,specifier,selected});if(local.split(sep).includes("node_modules"))continue;pending.add(selected);}
 }
 for(const ownerRoot of new Set([root,sourceRoot]))for(const name of ["bun.lock","bun.lockb"]){const path=resolve(ownerRoot,name);(ownerRoot===root?configuration:resolverConfiguration).set(path,cargoPreparationInputV1(ownerRoot,path,"file"));}
 return{sourceRoot,sources:[...files.values()],resolverInputs:[...resolverConfiguration.values()],inputs:[...configuration.values()],resolutions};
}


if(import.meta.main){
 if(process.argv[2]==="capture-program"){
  if(process.argv.length!==5)throw Error("capture-program <root> <entry>");console.log(JSON.stringify(captureProgram(process.argv[3]!,process.argv[4]!)));
 }else if(process.argv[2]==="resolve-program"){
  if(process.argv.length!==3)throw Error("resolve-program accepts only its owned input stream");
  const edges=JSON.parse(await Bun.stdin.text());if(!Array.isArray(edges)||edges.some(edge=>typeof edge.source!=="string"||typeof edge.specifier!=="string"))throw Error("Invalid program resolution request");
  console.log(JSON.stringify(edges.map(edge=>Bun.resolveSync(edge.specifier,dirname(edge.source)))));
 }else throw Error("Select capture-program or resolve-program");
}
