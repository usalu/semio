import {isAbsolute,relative,resolve,sep,win32} from "node:path";
import {realpath} from "node:fs/promises";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🎯️ Selects a declared product without accepting executable or publication paths. */
export interface RepositoryCargoArtifactAcquisitionV1{readonly version:1;readonly project:string;readonly target:string;readonly configuration:"prototype"|"release";readonly arguments:readonly string[]}
/** 📦️ Names the repository-owned Cargo product, relative to its actual project. */
export interface RepositoryCargoArtifactDeclarationV1{readonly version:1;readonly kind:"cargo-executable";readonly manifest:string;readonly package:string;readonly binary:string;readonly sourcesRecord?:string}

/** ✅️ Admits only descriptive selection DATA; this function issues no authority. */
export function readRepositoryCargoArtifactAcquisitionV1(value:unknown):RepositoryCargoArtifactAcquisitionV1{
 if(validateJsonSchemaSubset(schema.definitions.Request,value).length)throw Error("Closed declared Cargo artifact selection required");
 const request=value as RepositoryCargoArtifactAcquisitionV1;return Object.freeze({...request,arguments:Object.freeze([...request.arguments])});
}

/** 🔏️ Refuses platform-independent path escapes before any host filesystem operation. */
export function readRepositoryCargoArtifactDeclarationV1(value:unknown):RepositoryCargoArtifactDeclarationV1{
 if(validateJsonSchemaSubset(schema.definitions.Declaration,value).length)throw Error("Closed declared Cargo artifact product required");
 const declaration=value as RepositoryCargoArtifactDeclarationV1;
 if(isAbsolute(declaration.manifest)||win32.isAbsolute(declaration.manifest)||declaration.manifest.includes(":"))throw Error("Project-relative Cargo artifact manifest required");
 return Object.freeze({...declaration});
}

/** 🛤️ Resolves admitted declaration DATA without converting it into a storage or compiler grant. */
export function declaredCargoArtifactManifestV1(workspace:string,projectRoot:string,value:unknown):string{
 const declaration=readRepositoryCargoArtifactDeclarationV1(value),root=resolve(workspace),project=resolve(root,projectRoot),manifest=resolve(project,declaration.manifest),owner=relative(root,project),inside=relative(project,manifest);
 if(isAbsolute(projectRoot)||win32.isAbsolute(projectRoot)||projectRoot.includes(":")||owner===".."||owner.startsWith(".."+sep)||isAbsolute(owner)||inside===".."||inside.startsWith(".."+sep)||isAbsolute(inside)||manifest.length>256)throw Error("Actual owned Cargo artifact manifest required");
 return manifest;
}

/** 🔎️ Checks physical containment as descriptive evidence; compiler authority remains with its caller. */
export async function canonicalDeclaredCargoArtifactManifestV1(workspace:string,projectRoot:string,value:unknown):Promise<string>{
 const manifest=declaredCargoArtifactManifestV1(workspace,projectRoot,value),[root,project,file]=await Promise.all([realpath(resolve(workspace)),realpath(resolve(workspace,projectRoot)),realpath(manifest)]),within=(base:string,path:string)=>{const part=relative(base,path);return part!==".."&&!part.startsWith(".."+sep)&&!isAbsolute(part);};
 if(!within(root,project)||!within(project,file))throw Error("Canonical Cargo artifact owner escaped its project");
 return manifest;
}
