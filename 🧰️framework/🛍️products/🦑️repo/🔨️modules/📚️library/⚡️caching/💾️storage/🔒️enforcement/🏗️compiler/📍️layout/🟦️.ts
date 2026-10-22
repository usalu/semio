import{posix,win32}from"node:path";
import{readBuildMode}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import policy from"./🔣️policy.json"with{type:"json"};
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
/** 📍️ Pure directory projections never issue physical compiler storage authority. */
export interface RepositoryCompilerLayoutV1{readonly baseDirectory:string;readonly buildDirectory:string;readonly captureDirectory:string;readonly temporaryDirectory:string;readonly homeDirectory:string;readonly targetDirectory:string;}
/** 🏠️ Names the one storage host that owns the Rust image, Cargo registry home and toolchain of every workspace inside one repository checkout; a workspace outside the repository root hosts itself. */
export function repositoryCompilerHostV1(repositoryRoot:string,workspace:string,platform:"posix"|"win32"=process.platform==="win32"?"win32":"posix"):string{
 const paths=platform==="win32"?win32:posix;
 for(const path of[repositoryRoot,workspace])if(!paths.isAbsolute(path)||path.includes("\0")||paths.resolve(path)!==path)throw Error("Canonical absolute repository root and workspace required");
 const distance=paths.relative(repositoryRoot,workspace);
 return distance===".."||distance.startsWith(".."+paths.sep)||paths.isAbsolute(distance)?workspace:repositoryRoot;
}
/** 🗜️ Compact build roots leave room for Cargo's own profile, package and object names. */
export function repositoryCompilerLayoutV1(root:string,requestedMode:unknown,platform:"posix"|"win32"=process.platform==="win32"?"win32":"posix"):RepositoryCompilerLayoutV1{
 const mode=readBuildMode(requestedMode),paths=platform==="win32"?win32:posix;if(validateJsonSchemaSubset(schema.$defs.Policy,policy).length||!paths.isAbsolute(root)||root.includes("\0"))throw Error("Canonical compiler layout root and policy required");const baseDirectory=paths.join(root,"cargo",mode);return Object.freeze({baseDirectory,buildDirectory:paths.join(root,policy.buildNamespace,policy.modes[mode]),captureDirectory:paths.join(baseDirectory,"captures"),temporaryDirectory:paths.join(baseDirectory,"tmp"),homeDirectory:paths.join(root,"cargo/home"),targetDirectory:paths.join(root,policy.targetNamespace,policy.modes[mode])});
}
