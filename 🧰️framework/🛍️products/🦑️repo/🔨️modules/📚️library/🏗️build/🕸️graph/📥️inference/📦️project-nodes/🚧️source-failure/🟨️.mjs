import{createHash}from"node:crypto";
import{lstatSync,readFileSync,statSync}from"node:fs";
import{dirname,isAbsolute,relative,resolve}from"node:path";
import{fileURLToPath}from"node:url";
import{isHalfEditedModuleLoadV1}from"../../../../🧷️format-stamps/🟨️.mjs";
import{observedFs,observedRequireResolve,OBSERVATION_LIMITS_V1}from"../../../../👁️observation/🟨️.mjs";
import{INFERENCE_LIMITS_V1}from"../../🧬️schema/🟨️.mjs";

/** 🧰️ The directory of the Build library whose tooling this process runs from: a module of it that does not load is a stale implementation of the process, never a fact about an owner. */
export const IMPLEMENTATION_ROOT_V1=resolve(dirname(fileURLToPath(import.meta.url)),"../../../../..");

const MISSING=/Cannot find (?:package|module) '([^']+)'(?: imported from (\S+))?/u,EXPORT=/in module '([^']+)'/u,under=(root,path)=>{const row=relative(root,path);return row!==""&&!row.startsWith("..")&&!isAbsolute(row);};

const DIRECTORY_CALLS=new Set(["scandir","readdir","opendir"]),SOURCE_CODES=new Set(["MODULE_NOT_FOUND","ERR_MODULE_NOT_FOUND","ENOENT","ENOTDIR","EISDIR"]),ABSOLUTE=/(?:\/|[A-Za-z]:[\\/])[^\s'"`:]+/u,STACK=/Require stack:\s*-\s*(\S+)/u,FROM=/\b(?:from|in|of|at)\s+'?((?:\/|[A-Za-z]:[\\/])[^\s'"`]+)/u,SPECIFIER=/:\s+(\S+)\s+from\s/u,INFRASTRUCTURE=/(?:^|[\\/])(?:\.cache|node_modules|\.git)(?:[\\/]|$)/u;
const absolute=value=>typeof value==="string"&&ABSOLUTE.test(value)&&isAbsolute(value.replace(/[?#].*$/u,""))?value.replace(/[?#].*$/u,""):undefined;

/** 🔎️ The failure attributable to a source file, in an error or its causes (never a bundler aggregate, which is a product Source problem): a module load failure of a half-edited module, a module or workspace package import that is not there, a file that is not there and a source that does not parse, with what it names as the importing file, the module that is not there and the files whose change could fix it (`watched`, the package metadata a workspace import resolved through). */
export function moduleLoadFailureV1(error){
 const seen=new Set(),walk=(value,depth)=>{
  if(value===null||typeof value!=="object"||seen.has(value)||depth>8)return undefined;seen.add(value);
  const text=String(value.message??"");
  if(isHalfEditedModuleLoadV1(value)){
   const missing=MISSING.exec(text),referrer=value.referrer??missing?.[2]??EXPORT.exec(text)?.[1]??value.position?.file,specifier=value.specifier??missing?.[1];
   const file=typeof referrer==="string"?referrer.replace(/[?#].*$/u,""):undefined,importer=file!==undefined&&isAbsolute(file)?file:undefined,target=typeof specifier!=="string"?undefined:isAbsolute(specifier)?specifier:importer!==undefined&&specifier.startsWith(".")?resolve(dirname(importer),specifier):undefined;
   return{error:value,importer,specifier,target,watched:value.watched};
  }
  if(SOURCE_CODES.has(value.code)&&!DIRECTORY_CALLS.has(value.syscall)||value.unresolved===true||value.name==="SyntaxError"&&absolute(value.position?.file)!==undefined){
   const importer=absolute(value.importer)??absolute(value.source)??absolute(value.position?.file)??absolute(Array.isArray(value.requireStack)?value.requireStack[0]:undefined)??absolute(STACK.exec(text)?.[1])??absolute(FROM.exec(text)?.[1]),specifier=value.specifier??MISSING.exec(text)?.[1]??SPECIFIER.exec(text)?.[1];
   const target=absolute(value.path)??(typeof specifier==="string"&&isAbsolute(specifier)?specifier:undefined);
   if(importer!==undefined||target!==undefined)return{error:value,importer,specifier,target,watched:value.watched};
  }
  return walk(value.cause,depth+1);
 };
 return walk(error,0);
}

/** 🚧️ Turns the load failure of one owner's own source (a file that imports a module that is not there, as when a replica moved it a moment ago) into the withheld-source failure of that owner alone: the missing module is recorded as an observation of the owner, so the owner is checked again when the file appears, and the failure names the importing source. A failure of the library tooling, of an error that names no module or no source, and any other error answer `undefined`: those stay what they were. Recorded observations belong to the observation in progress.
 * @param {{workspaceRoot:string,fallback:string,implementationRoot?:string}} scope the workspace, the source named when the failure names none, the tooling root */
export function ownerSourceFailureV1(error,{workspaceRoot,fallback,implementationRoot=IMPLEMENTATION_ROOT_V1}){
 if(error?.code==="BUILD_SOURCE_UNPARSEABLE")return error;
 const failure=moduleLoadFailureV1(error);if(failure===undefined)return undefined;
 const{importer,specifier,target}=failure;
 if(importer===undefined&&target===undefined)return undefined;
 if([importer,target].some(path=>path!==undefined&&under(implementationRoot,path)))return undefined;
 const named=[importer,target].find(path=>path!==undefined&&under(workspaceRoot,path)&&!INFRASTRUCTURE.test(relative(workspaceRoot,path)));if(named===undefined)return undefined;
 let source=fallback,bytes;
 for(const path of[importer,fallback]){if(path===undefined)continue;try{if(lstatSync(path).isFile()){bytes=readFileSync(path);source=path;break;}}catch{}}
 if(bytes===undefined)return undefined;
 if(importer!==undefined&&specifier!==undefined&&!isAbsolute(specifier))try{observedRequireResolve(importer,specifier);}catch{}
 for(const path of[importer,target,...failure.watched??[]])if(path!==undefined)try{observedFs.statSync(path);}catch{}
 const text=String(failure.error.message).split("\n")[0].slice(0,512);
 return Object.assign(Error(text+" (imported by "+relative(workspaceRoot,source)+")",{cause:error}),{code:"BUILD_SOURCE_UNPARSEABLE",unresolvedImport:MISSING.test(String(failure.error.message))||failure.error.unresolved===true||/^E?R?R?_?MODULE_NOT_FOUND$/u.test(String(failure.error.code)),watched:failure.watched,source,sha256:createHash("sha256").update(bytes).digest("hex")});
}

/** ⏳️ Marks the failure of an owner's source as the footprint of a replica that has not settled when the source was written within the window of its kind (a module it imports that is not there: `unsettledImportMilliseconds`, any other failure: the observation racy rule) and answers it, so that every process that meets it waits for the replica and runs again; a source written before the window answers `undefined` and withholds its owner. @see ../../../../🧷️format-stamps/🟨️.mjs
 * @param {{source?:string,unresolvedImport?:boolean}} error
 * @param {{now?:number,modified?:(path:string)=>number}} [scope] */
export function replicaUnsettledV1(error,{now=Date.now(),modified=path=>statSync(path).mtimeMs}={}){
 let written=Infinity;
 try{written=modified(String(error.source));}catch{}
 for(const path of error.watched??[])try{written=Math.max(written===Infinity?-Infinity:written,modified(String(path)));}catch{}
 const window=error.unresolvedImport===true?INFERENCE_LIMITS_V1.unsettledImportMilliseconds:OBSERVATION_LIMITS_V1.racyMilliseconds;
 return written>=now-window?Object.assign(error,{replicaUnsettled:true}):undefined;
}

/** 🛂️ The boundary of one owner's observation: any failure attributable to a workspace source of that owner (see `moduleLoadFailureV1`) becomes the failure of that owner alone, either a replica that has not settled (the source or the package metadata it resolved through was written within the window: the process waits for the replica and runs again) or the withheld-source failure `BUILD_SOURCE_UNPARSEABLE` (stable: fatal only when the owner is in the closure asked for), with the paths it rests on recorded in the observation in progress so the owner is checked again when they change; every other failure answers `undefined` and stays fatal. @see ./🧪️tests/⏳️replica-unsettled/🟦️.ts
 * @param {{workspaceRoot:string,fallback:string,implementationRoot?:string}} scope */
export function ownerBoundaryFailureV1(error,scope){
 const failure=ownerSourceFailureV1(error,scope);
 return failure===undefined?undefined:replicaUnsettledV1(failure)??failure;
}
