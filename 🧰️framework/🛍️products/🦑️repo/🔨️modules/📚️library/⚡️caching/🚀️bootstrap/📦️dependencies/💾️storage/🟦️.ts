import {createHash,randomUUID} from "node:crypto";
import {lstatSync,realpathSync,mkdirSync,renameSync,symlinkSync,readFileSync,writeFileSync,rmSync,openSync,fsyncSync,closeSync,type Stats} from "node:fs";
import {join,resolve,dirname,posix} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {checkResourceLeaseControl,requireMaterializedPathV1,withResourceLeases} from "../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {requireNativeStorageReservationV1} from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {withRepositoryCacheStoreV1,type RepositoryCacheStoreV1} from "../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {repositoryCachePartitionRootV1,repositoryLeaseDirectoryV1} from "../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import bindingsSchema from "./🧬️schema/📥️bindings.json" with {type:"json"};

/** 🧭️ Resolves portable declared scope members inside their original repository Source authority. */
export function javascriptDependencyPackagePathsV1(scope:string,members:readonly string[]):readonly string[]{
 if(validateJsonSchemaSubset(schema,{version:1,scope,members,generation:"0".repeat(64)}).length||posix.normalize(scope)!==scope||scope.endsWith("/")||scope===".."||scope.startsWith("../"))throw Error("Closed JavaScript installation scope required");
 const paths=[scope,...members.map(member=>{if(posix.normalize(member)!==member||member.endsWith("/"))throw Error("Original canonical Bun workspace path required");return posix.join(scope,member);})];if(paths.some(path=>path===".."||path.startsWith("../")||posix.isAbsolute(path))||new Set(paths).size!==paths.length)throw Error("Disjoint original dependency package required");return Object.freeze(paths);
}

/** 🗂️ Gives each original physical Source package one compact dependency-only write namespace. */
export function javascriptDependencyBindingsV1(workspace:string,storageRoot:string,scope:string,members:readonly string[],generation:string):readonly Readonly<{sourceDirectory:string;targetDirectory:string}>[]{
 if(validateJsonSchemaSubset(schema,{version:1,scope,members,generation}).length)throw Error("Closed JavaScript installation scope required");const root=resolve(workspace),stage=join(resolve(storageRoot),"j",generation.slice(0,20)),paths=javascriptDependencyPackagePathsV1(scope,members);return Object.freeze(paths.map((path,index)=>{const sourceDirectory=requireMaterializedPathV1(join(root,path,"node_modules")),targetDirectory=requireMaterializedPathV1(index===0?join(stage,"node_modules"):join(stage,"w",String(index-1),"node_modules"));return Object.freeze({sourceDirectory,targetDirectory});}));
}
/** 🎟️ Refuses invented dependency stores before any path binding or installer child. */
export async function requireJavascriptDependencyStorageV1(workspace:string,store:RepositoryCacheStoreV1):Promise<void>{if((store?.invocation?.capabilities as {workspaceRoot?:unknown}|undefined)?.workspaceRoot!==resolve(workspace))throw Error("Original JavaScript repository required");requireNativeStorageReservationV1(store.storage,store.control);await store.recheck();if(store.root!==repositoryCachePartitionRootV1(workspace,"dependencies"))throw Error("Original dependencies partition required");}
/** 🪢️ Retains an unqualified original directory or link without attesting its dependency bytes. */
function retainJavascriptDependencyBaselineV1(workspace:string,store:RepositoryCacheStoreV1,binding:Readonly<{sourceDirectory:string;targetDirectory:string}>):void{
 const physical=lstatSync(binding.sourceDirectory),retained=requireMaterializedPathV1(join(workspace,".cache/held-javascript",createHash("sha256").update(binding.sourceDirectory).digest("hex").slice(0,24)+"-"+randomUUID().slice(0,8)));mkdirSync(dirname(retained),{recursive:true});renameSync(binding.sourceDirectory,retained);writeFileSync(requireNativeStoragePathV1(store.root,join(dirname(binding.targetDirectory),"retained.json")),JSON.stringify({version:1,source:binding.sourceDirectory,retained,dev:physical.dev,ino:physical.ino}));console.log("[DEBUG] Retained unqualified JavaScript dependency baseline "+JSON.stringify({source:binding.sourceDirectory,retained,dev:physical.dev,ino:physical.ino}));
}

/** 🔗️ Retains unqualified prior bindings and publishes only the caller's enforcing installed generation. */
export async function bindJavascriptDependencyDirectoriesV1(workspace:string,store:RepositoryCacheStoreV1,scope:string,members:readonly string[],generation:string):Promise<readonly Readonly<{sourceDirectory:string;targetDirectory:string}>[]>{
 await requireJavascriptDependencyStorageV1(workspace,store);const bindings=javascriptDependencyBindingsV1(workspace,store.root,scope,members,generation);
 for(const binding of bindings){
  requireNativeStoragePathV1(store.root,binding.targetDirectory);await store.recheck();mkdirSync(binding.targetDirectory,{recursive:true});const physical=lstatSync(binding.sourceDirectory,{throwIfNoEntry:false});
  if(physical?.isSymbolicLink()){
   const previous=realpathSync(binding.sourceDirectory);if(previous===binding.targetDirectory)continue;requireNativeStoragePathV1(store.root,previous);const receiptPath=join(previous.replace(/(?:[\\/]w[\\/][0-9]+)?[\\/]node_modules$/u,""),"bindings.json"),status=lstatSync(receiptPath,{throwIfNoEntry:false});
   if(status){if(!status.isFile()||status.isSymbolicLink()||status.nlink!==1||status.size>1048576)throw Error("Original dependency binding receipt required");const receipt=JSON.parse(readFileSync(receiptPath,"utf8"));if(validateJsonSchemaSubset(bindingsSchema,receipt).length||!receipt.bindings.some((row:any)=>row.sourceDirectory===binding.sourceDirectory&&row.targetDirectory===previous))throw Error("Foreign JavaScript dependency binding refused");const temporary=requireMaterializedPathV1(binding.sourceDirectory+"."+randomUUID().slice(0,8));try{symlinkSync(binding.targetDirectory,temporary,process.platform==="win32"?"junction":"dir");renameSync(temporary,binding.sourceDirectory);}finally{rmSync(temporary,{force:true});}continue;}
   retainJavascriptDependencyBaselineV1(workspace,store,binding);
  }else if(physical){if(!physical.isDirectory())throw Error("Original dependency directory required");retainJavascriptDependencyBaselineV1(workspace,store,binding);}
  mkdirSync(dirname(binding.sourceDirectory),{recursive:true});symlinkSync(binding.targetDirectory,binding.sourceDirectory,process.platform==="win32"?"junction":"dir");if(realpathSync(binding.sourceDirectory)!==binding.targetDirectory)throw Error("Actual dependency binding changed");
 }
 return bindings;
}
/** 🏪️ Holds the authentic dependency volume and original module binding custody throughout its whole installer operation. */
export function withJavascriptDependencyStorageV1<T>(workspace:string,invocation:ScriptInvocation,body:(store:RepositoryCacheStoreV1)=>Promise<T>):Promise<T>{return withRepositoryCacheStoreV1(workspace,"dependencies",invocation,store=>withResourceLeases({directory:repositoryLeaseDirectoryV1(workspace),signal:store.control.signal,control:store.control,resources:[{resource:"javascript-dependency-bindings:"+resolve(workspace),mode:"exclusive"}]},()=>body(store)));}

/** 🧬️ Canonical Bun material excludes only first-party top-level task metadata. */
export function javascriptDependencyInstallManifestV1(value:unknown):string {
 if(value===null||typeof value!=="object"||Array.isArray(value))throw Error("Original Bun manifest object required");
 const canonical=(value:unknown):unknown=>Array.isArray(value)?value.map(canonical):value!==null&&typeof value==="object"?Object.fromEntries(Object.entries(value).sort(([a],[b])=>Buffer.from(a).compare(Buffer.from(b))).map(([key,value])=>[key,canonical(value)])):value;
 return JSON.stringify(canonical(Object.fromEntries(Object.entries(value).filter(([key])=>key!=="semio"))));
}
/** 🔎️ Holds full original installer Source bytes and physical facts independently of install material. */
export type JavascriptDependencySourceV1=Readonly<{path:string;kind:"file"|"directory"|"missing";bytes:Buffer;dev:string;ino:string;mode:number}>;

/** 📦️ Projects canonical native inputs into compact enforcing paths under original membership custody. */
export async function stageJavascriptDependencyInputsV1(workspace:string,store:RepositoryCacheStoreV1,scope:string,mode:"sync"|"lock",members:readonly string[]):Promise<Readonly<{directory:string;generation:string;members:readonly string[];bindings:ReturnType<typeof javascriptDependencyBindingsV1>;sources:readonly JavascriptDependencySourceV1[];restoreLock:(bytes:string)=>string}>>{
 await requireJavascriptDependencyStorageV1(workspace,store);
 const packagePaths=javascriptDependencyPackagePathsV1(scope,members),sourceRoot=resolve(workspace,packagePaths[0]!);
 const sources:JavascriptDependencySourceV1[]=[],observed=new Set<string>(),material=new Map<string,Buffer>();
 const retain=(path:string,kind:"file"|"directory"|"missing",bytes:Buffer,facts:Stats|undefined)=>{if(!observed.has(path)){observed.add(path);sources.push(Object.freeze({path,kind,bytes,dev:facts?String(facts.dev):"",ino:facts?String(facts.ino):"",mode:facts?.mode??0}));}};
 const capture=(path:string,optional=false)=>{
  const file=requireMaterializedPathV1(path);
  for(let parent=dirname(file);;parent=dirname(parent)){if(!observed.has(parent)){const facts=lstatSync(parent);if(!facts.isDirectory()||facts.isSymbolicLink())throw Error("Original physical Bun input ancestor required");retain(parent,"directory",Buffer.alloc(0),facts);}if(parent===dirname(parent))break;}
  const facts=lstatSync(file,{throwIfNoEntry:false});if(!facts){retain(file,"missing",Buffer.alloc(0),undefined);if(optional)return undefined;throw Error("Original Bun input file missing: "+file);}
  if(!facts.isFile()||facts.isSymbolicLink()||facts.size>8388608)throw Error("Original bounded Bun input file required");const bytes=readFileSync(file);retain(file,"file",bytes,facts);material.set(file,bytes);return bytes;
 };
 const rootPath=join(sourceRoot,"package.json"),manifest=JSON.parse(capture(rootPath)!.toString()) as Record<string,any>;
 if(!Array.isArray(manifest.workspaces)||manifest.workspaces.length!==members.length||new Set(manifest.workspaces).size!==members.length||manifest.workspaces.some((member:unknown)=>typeof member!=="string"||!members.includes(member)))throw Error("Authored exact Bun workspace members required");
 material.set(rootPath,Buffer.from(javascriptDependencyInstallManifestV1(manifest)));
 const mapping=new Map<string,string>(members.map((member,index)=>[member.replaceAll("\\","/"),"w/"+index]));
 const patches:{target:string;bytes:Buffer}[]=[],patchSources=new Map<string,string>(),patchIdentities=new Map<string,{source:string;target:string}>();
 const projectPatches=(document:Record<string,any>,owner:string)=>{if(document.patchedDependencies===undefined)return document;const projected=Object.entries(document.patchedDependencies).sort(([a],[b])=>Buffer.from(a).compare(Buffer.from(b))).map(([name,path])=>{if(typeof path!=="string")throw Error("Original relative Bun patch required");const source=javascriptDependencyPackagePathsV1(owner,[path])[1]!,prior=patchIdentities.get(name);if(prior&&prior.source!==source)throw Error("Conflicting original Bun patch identity");let destination=patchSources.get(source);if(!destination){destination="patches/"+patches.length+".patch";patchSources.set(source,destination);patches.push({target:destination,bytes:capture(join(workspace,source))!});}patchIdentities.set(name,{source,target:destination});return[name,destination];});return{...document,patchedDependencies:Object.fromEntries(projected)};};
 const stagedRoot=projectPatches(manifest,scope);
 const manifests=[...mapping].map(([member,target],index)=>{const owner=packagePaths[index+1]!,path=join(workspace,owner,"package.json"),document=JSON.parse(capture(path)!.toString()),canonical=Object.fromEntries(Object.entries(document).filter(([key])=>key!=="workspaces"));material.set(path,Buffer.from(javascriptDependencyInstallManifestV1(canonical)));return{target,bytes:Buffer.from(javascriptDependencyInstallManifestV1(projectPatches(canonical,owner)))};});
 const lockBytes=capture(join(sourceRoot,"bun.lock"),mode==="lock");if(!lockBytes&&mode==="sync")throw Error("Original frozen Bun lock required");
 const ownConfig=join(sourceRoot,"bunfig.toml"),rootConfig=join(workspace,"bunfig.toml"),configuration=capture(ownConfig,true)??(ownConfig!==rootConfig?capture(rootConfig,true):undefined);
 recheckJavascriptDependencyInputsV1(sources);
 const hash=createHash("sha256").update(JSON.stringify(["javascript-install-projection",2,sourceRoot,process.versions.bun,process.platform,process.arch,mode==="lock"?randomUUID():"locked",members]));for(const[path,bytes]of material)hash.update(JSON.stringify([path,bytes.length])).update(bytes);
 const generation=hash.digest("hex"),bindings=javascriptDependencyBindingsV1(workspace,store.root,scope,members,generation),directory=dirname(bindings[0]!.targetDirectory);requireNativeStoragePathV1(store.root,directory);mkdirSync(directory,{recursive:true});if(configuration)writeFileSync(join(directory,"bunfig.toml"),configuration);
 const staged:Record<string,any>={...JSON.parse(javascriptDependencyInstallManifestV1(stagedRoot)),workspaces:[...mapping.values()]};
 for(const row of patches){mkdirSync(dirname(join(directory,row.target)),{recursive:true});writeFileSync(join(directory,row.target),row.bytes);}for(const row of manifests){mkdirSync(join(directory,row.target),{recursive:true});writeFileSync(join(directory,row.target,"package.json"),row.bytes);}writeFileSync(join(directory,"package.json"),javascriptDependencyInstallManifestV1(staged));
 const project=(lock:any,reverse:boolean)=>{const links=reverse?new Map([...mapping].map(([source,target])=>[target,source])):mapping;if(lock.patchedDependencies)lock.patchedDependencies=Object.fromEntries([...patchIdentities].map(([name,row])=>[name,reverse?posix.relative(scope,row.source):row.target]));lock.workspaces=Object.fromEntries(Object.entries(lock.workspaces??{}).map(([path,value])=>[links.get(path)??path,value]));for(const row of Object.values(lock.packages??{}) as any[]){if(!Array.isArray(row)||typeof row[0]!=="string")continue;const marker=row[0].indexOf("@workspace:");if(marker<0)continue;const path=row[0].slice(marker+11),target=links.get(path);if(!target)throw Error("Locked workspace package lacks original admitted Source");row[0]=row[0].slice(0,marker+11)+target;}return JSON.stringify(lock,null,2)+"\n";};if(lockBytes)writeFileSync(join(directory,"bun.lock"),project(Bun.JSONC.parse(lockBytes.toString()),false));
 return Object.freeze({directory,generation,members:Object.freeze([...members]),bindings,sources:Object.freeze(sources),restoreLock:(bytes:string)=>project(Bun.JSONC.parse(bytes),true)});
}
/** 🔍️ Replays full bytes and physical path/mode facts, including omitted metadata and absent config. */
export function recheckJavascriptDependencyInputsV1(sources:readonly JavascriptDependencySourceV1[]):void {
 for(const source of sources){const facts=lstatSync(source.path,{throwIfNoEntry:false});if(source.kind==="missing"){if(facts)throw Error("Original Bun dependency input appeared during installation");continue;}if(!facts||facts.isSymbolicLink()||(source.kind==="file"?!facts.isFile():!facts.isDirectory())||String(facts.dev)!==source.dev||String(facts.ino)!==source.ino||facts.mode!==source.mode||source.kind==="file"&&!readFileSync(source.path).equals(source.bytes))throw Error("Original Bun dependency input changed during installation");}
}
/** 🧾️ Durably witnesses the complete installed binding plan before any source link changes. */
function publishJavascriptBindingReceiptV1(store:RepositoryCacheStoreV1,stage:string,generation:string,bindings:readonly Readonly<{sourceDirectory:string;targetDirectory:string}>[]):void{
 const value={version:1,generation,bindings};if(validateJsonSchemaSubset(bindingsSchema,value).length)throw Error("Closed dependency binding receipt required");checkResourceLeaseControl(store.control);const path=requireNativeStoragePathV1(store.root,join(stage,"bindings.json")),temporary=requireNativeStoragePathV1(store.root,path+"."+randomUUID().slice(0,8));let descriptor:number|undefined;
 try{descriptor=openSync(temporary,"wx");writeFileSync(descriptor,JSON.stringify(value));fsyncSync(descriptor);closeSync(descriptor);descriptor=undefined;checkResourceLeaseControl(store.control);renameSync(temporary,path);const directory=openSync(stage,"r");try{fsyncSync(directory);}finally{closeSync(directory);}}finally{if(descriptor!==undefined)closeSync(descriptor);rmSync(temporary,{force:true});}
}
/** 🔁️ Resolves workspace dependency exports from their original Source owners after all bounded install writes finish. */
export async function publishJavascriptWorkspaceExportsV1(workspace:string,store:RepositoryCacheStoreV1,scope:string,members:readonly string[],generation:string):Promise<void>{await requireJavascriptDependencyStorageV1(workspace,store);const bindings=javascriptDependencyBindingsV1(workspace,store.root,scope,members,generation),stage=dirname(bindings[0]!.targetDirectory);for(const member of members){const source=resolve(workspace,scope,member),manifest=JSON.parse(readFileSync(join(source,"package.json"),"utf8"));if(typeof manifest.name!=="string"||!/^(@[A-Za-z0-9._-]+\/)?[A-Za-z0-9._-]+$/u.test(manifest.name))throw Error("Original Bun workspace package name required");const link=requireMaterializedPathV1(join(stage,"node_modules",manifest.name));requireNativeStoragePathV1(store.root,dirname(link));const physical=lstatSync(link,{throwIfNoEntry:false});if(physical){if(!physical.isSymbolicLink())throw Error("Original staged workspace package link required");rmSync(link);}mkdirSync(dirname(link),{recursive:true});symlinkSync(source,link,process.platform==="win32"?"junction":"dir");}await store.recheck();publishJavascriptBindingReceiptV1(store,stage,generation,bindings);await bindJavascriptDependencyDirectoriesV1(workspace,store,scope,members,generation);}
