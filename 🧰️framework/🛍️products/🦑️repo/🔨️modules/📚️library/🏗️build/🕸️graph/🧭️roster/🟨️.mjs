import cachePolicy from"../../../⚡️caching/🔣️policy.json"with{type:"json"};
import ownershipPolicy from"../../../🗂️workspaces/🔣️policy.json"with{type:"json"};
import{isOpaqueSourcePathV1}from"../../../⚡️caching/🔍️discovery/📂️source/🟦️.ts";

/** 🧭️ The file name of an authored project configuration. */
export const PROJECT_BASENAME_V1="📋️project.json";

/**
 * 🧭️ The pure rules that decide which directories the workspace walk enters and which configuration files and manifests are admitted as project candidates. Inference and the `project-set` node of the derived graph both call these functions, so the roster of one is the roster of the other by construction: neither keeps a rule of its own.
 * @see ../🧮️derived-nodes/🧩️kinds/🟨️.mjs
 */
export function createRosterRulesV1({generatedDirectories,collectionNames,moduleMember,projectBasename=PROJECT_BASENAME_V1,isOpaque=isOpaqueSourcePathV1}){
 const generated=new Set(generatedDirectories),excluded=new Set([...generatedDirectories,".git",".🧬semio",".nx",".cache",".r","node_modules","dist","target",".venv","venv","__pycache__",".pytest_cache",".next",".turbo","temp","🗑️generated"]),collections=new Set(collectionNames);
 const slashed=path=>path.split("\\").join("/"),segments=path=>path.split("/");
 const underGenerated=path=>segments(path).some(part=>generated.has(part));
 const rules={
  /** 🚶️ Whether the workspace walk enters a directory: not a generated, cache or dependency directory, not a build artifact directory of an extension host, not a collection directory below anything but a module member and not the composition directory. */
  descends:(name,path,parent)=>!isOpaque(path)&&!excluded.has(name)&&!name.startsWith("extension-host-build-")&&!(collections.has(name)&&parent.split("/").at(-1)!==moduleMember)&&path!=="compose",
  /** 🧫️ Whether a path lies below an example collection that is no module member. */
  inCollection:path=>{const parts=segments(slashed(path));return parts.some((part,index)=>collections.has(part)&&parts[index-1]!==moduleMember);},
  /** 🛡️ The textual part of the admission of a supplied candidate: a well formed relative path outside opaque, collection, generated, authority and composition places. */
  admitsCandidate(supplied){
   if(typeof supplied!=="string"||!supplied||supplied.includes("\0")||supplied.includes("\uFFFD"))return false;
   const path=slashed(supplied),parts=segments(path);
   if(isOpaque(path)||rules.inCollection(path)||path.startsWith("/")||/^[A-Za-z]:/u.test(path)||parts.some(part=>!part||part==="."||part===".."||part===".🧬semio"||generated.has(part))||path.startsWith("compose/")||path.startsWith("temp/compose/"))return false;
   return true;
  },
  /** 📋️ Whether a candidate is an authored project configuration. */
  isAuthoredProjectConfig:path=>!isOpaque(path)&&path.endsWith(projectBasename)&&!path.includes("\uFFFD")&&!path.includes(".🧬semio")&&!underGenerated(path),
  /** 🦀️ Whether a candidate Cargo manifest is considered for a native project. */
  admitsManifest:path=>!isOpaque(path)&&!path.includes("\uFFFD")&&!path.startsWith("compose/")&&!path.startsWith("temp/compose/")&&!path.includes(".🧬semio")&&!underGenerated(path),
  /** 🧭️ The project candidates of a workspace file list: authored project files and Cargo manifests that pass every rule. */
  candidates(files){
   const projectFiles=[],cargoFiles=[];
   for(const file of files){
    if(!rules.admitsCandidate(file))continue;
    if(rules.isAuthoredProjectConfig(file))projectFiles.push(file);
    else if(file.endsWith("Cargo.toml")&&rules.admitsManifest(file))cargoFiles.push(file);
   }
   return{projectFiles,cargoFiles};
  },
 };
 return Object.freeze(rules);
}

/** 🧭️ The rules of this repository, from its declared policies. */
export const ROSTER_RULES_V1=createRosterRulesV1({generatedDirectories:cachePolicy.generatedDirectories,collectionNames:ownershipPolicy.collections.names,moduleMember:ownershipPolicy.collections.moduleMember});
