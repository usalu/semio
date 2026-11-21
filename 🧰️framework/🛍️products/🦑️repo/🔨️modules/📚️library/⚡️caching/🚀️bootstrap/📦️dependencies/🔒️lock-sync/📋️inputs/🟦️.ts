import {existsSync,readdirSync,readFileSync,realpathSync} from "node:fs";
import {isAbsolute,join,posix,relative,sep,win32} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {parseJsoncV1} from "../../🔌️managers/📖️formats/🟦️.ts";
import {refuseBootstrapV1} from "../../../📖️texts/🟦️.ts";

type ManifestV1={workspaces?:unknown;semio?:{workspace?:{owners?:unknown}}};
const ownerPattern=new RegExp(schema.$defs.Protocol.const.ownerPattern,"u");

/** 🧭️ Normalizes the declared paths used in manifests and in the lock's member names. */
export const javascriptLockMemberKeyV1=(member:string):string=>posix.normalize(member.replaceAll("\\","/").normalize("NFC")).replace(/\/$/u,"");

/** 📖️ Reads a published JavaScript package declaration using the lock check's parser. */
export function readJavascriptLockManifestV1(path:string):ManifestV1{
 const text=readFileSync(path,"utf8");
 try{return JSON.parse(text);}catch{return parseJsoncV1(text) as ManifestV1;}
}

/** 🧭️ Resolves the one-level owner declarations without walking member directories. */
export function javascriptLockOwnerDirectoriesV1(root:string,manifest:ManifestV1):readonly string[]{
 const owners=manifest?.semio?.workspace?.owners,paths=Array.isArray(owners)?owners.map(pattern=>{
  const match=typeof pattern==="string"?ownerPattern.exec(pattern):null;
  if(!match)throw refuseBootstrapV1("invalid","dependencies.owner-pattern-required",{pattern:String(pattern)});
  return match[1]!;
 }):[];
 if(!paths.length)return ["."];
 return [".",...readdirSync(root,{withFileTypes:true})
  .filter(entry=>entry.isDirectory()&&paths.some(path=>existsSync(join(root,entry.name,path))))
  .map(entry=>entry.name).sort()];
}

/** 📋️ Names the explicit package files in the last published roster. */
export function javascriptLockMemberFilesV1(scope:string,manifest:ManifestV1,manifestName:string):readonly string[]{
 const workspaces=manifest?.workspaces;
 if(!Array.isArray(workspaces))return [];
 const files=new Set<string>();
 for(const member of workspaces){
  if(typeof member!=="string"||member.includes("*")||win32.isAbsolute(member)||/^[A-Za-z]:/u.test(member))continue;
  const file=posix.join(javascriptLockMemberKeyV1(member),manifestName);
  if(posix.join(scope,file).startsWith("../"))continue;
  files.add(file);
 }
 return [...files];
}

/** 🔒️ Lists the published lock inputs, retaining the names of any members or locks that are absent. */
export function javascriptPublishedLockInputsV1(root:string,manifestName:string,lockName:string):readonly string[]{
 const workspace=realpathSync(root),rootFile=join(workspace,manifestName);
 if(!existsSync(rootFile))return [manifestName,lockName];
 const read=(path:string)=>{
  const resolved=realpathSync(path),inside=relative(workspace,resolved);
  if(inside===".."||inside.startsWith(".."+sep)||isAbsolute(inside))throw Error("Dependency scope manifest escapes the workspace: "+path);
  return readJavascriptLockManifestV1(resolved);
 };
 const rootManifest=read(rootFile),paths=new Set<string>();
 for(const scope of javascriptLockOwnerDirectoriesV1(workspace,rootManifest)){
  const base=posix.join(scope,manifestName),file=join(workspace,base);
  paths.add(base);paths.add(posix.join(scope,lockName));
  if(!existsSync(file))continue;
  const manifest=scope==="."?rootManifest:read(file);
  for(const member of javascriptLockMemberFilesV1(scope,manifest,manifestName))paths.add(posix.join(scope,member));
 }
 return [...paths].sort();
}
