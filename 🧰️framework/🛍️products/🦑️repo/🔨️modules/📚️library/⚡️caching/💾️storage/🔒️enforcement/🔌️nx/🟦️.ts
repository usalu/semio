import {openRepositoryCachePartitionV1} from "../📦️budget/📥️opening/🟦️.ts";
import {mkdirSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {repositoryCacheBudgetV1} from "../📦️budget/🟦️.ts";
import {withRepositoryCacheStoreV1,type RepositoryCacheStoreV1} from "../📦️budget/📥️opening/🏪️store/🟦️.ts";
import {isNxSystemPathV1} from "./📦️outputs/📍️paths/🟦️.ts";
import {classifyNxOutputV1} from "./📦️outputs/🟦️.ts";
import {AsyncLocalStorage} from "node:async_hooks";
import {receiveScriptProcessInvocation,type ScriptInvocation} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {withResourceLeases,checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {repositoryStrictStorageV1} from "../🟦️.ts";
import cacheConfigurationSchema from "./🧬️schema/📋️configuration.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import {validateBoundedNxPathsV1} from "../../../📦️artifacts/🔒️bounded/🟦️.ts";

/** 📂️ Defines all native Nx artifact, metadata, loader and temporary write locations. */
export interface BoundedNxNamespaceV1 {readonly root:string;readonly environment:Readonly<Record<string,string|undefined>>}
type CacheScopeV1=Readonly<{workspace:string;toolingIdentity:string;environment:Readonly<Record<string,string|undefined>>;root:string;mode:"shared"|"exclusive";control:ResourceLeaseControl;leaseDirectory:string;original:ScriptInvocation;live:()=>boolean;products?:Readonly<{store:RepositoryCacheStoreV1;live:()=>boolean}>;outputs:ReadonlyMap<string,"shared"|"exclusive">;recheck:()=>Promise<void>}>;
const cacheScope=new AsyncLocalStorage<CacheScopeV1>();

/** 📤️ Retains the same exact unit resources as its compiler while cache put reads or restore replaces output. */
async function withOutputCustodyV1<T>(scope:CacheScopeV1,paths:readonly string[],mode:"shared"|"exclusive",body:()=>Promise<T>):Promise<T>{
 if(!scope.live())throw Error("Original Nx cache callback released");
 const namespace=boundedNxNamespaceV1(scope.root,scope.toolingIdentity),system=["NX_CACHE_DIRECTORY","NX_WORKSPACE_DATA_DIRECTORY","NX_NATIVE_FILE_CACHE_DIRECTORY","TMPDIR"].map(name=>namespace.environment[name]!);
 const roots={rust:scope.root,products:join(repositoryCacheBudgetV1(scope.workspace).partitions.products.directory,"m")},selected=paths.filter(path=>path!==scope.root&&!system.some(root=>isNxSystemPathV1({version:1,platform:process.platform==="win32"?"windows":"posix",root,path}))).map(path=>classifyNxOutputV1(roots,path));
 if(selected.some(row=>row.partition==="products")&&!scope.products)return withRepositoryCacheStoreV1(scope.workspace,"products",scope.original,async store=>{let live=true;const products={store,live:()=>live},context={...scope,products};try{return await cacheScope.run(context,()=>withOutputCustodyV1(context,paths,mode,body));}finally{live=false;}});
 if(scope.products){if(!scope.products.live())throw Error("Original Nx product callback released");await scope.products.store.recheck();}
 const desired=mode==="exclusive"?"shared":"exclusive",outputs=[...new Set(paths.filter(path=>path!==scope.root&&!system.some(root=>isNxSystemPathV1({version:1,platform:process.platform==="win32"?"windows":"posix",root,path}))))].sort(),missing:string[]=[];
 for(const path of paths)if(path!==scope.root&&!system.includes(path)){const selected=classifyNxOutputV1(roots,path);validateBoundedNxPathsV1(selected.partition==="rust"?scope.root:scope.products!.store.root,[path]);}
 for(const path of outputs){const held=scope.outputs.get(path);if(held==="shared"&&desired==="exclusive")throw Error("Original output reader cannot become a restore writer");if(!held)missing.push(path);}
 if(!missing.length)return body();
 return withResourceLeases({directory:scope.leaseDirectory,resources:missing.map(path=>({resource:classifyNxOutputV1(roots,path).resource,mode:desired})),signal:scope.control.signal,control:scope.control},()=>{const held=new Map(scope.outputs);for(const path of missing)held.set(path,desired);return cacheScope.run({...scope,outputs:held},body);});
}
/** 🧭️ Binds deterministic namespace paths inside a physically admitted backing. */
export function boundedNxNamespaceV1(root:string,toolingIdentity:string):BoundedNxNamespaceV1{
 if(!/^[a-f0-9]{64}$/u.test(toolingIdentity))throw Error("Original Nx tooling identity required");
 const cache=requireNativeStoragePathV1(root,join(root,"nx/artifacts")),data=requireNativeStoragePathV1(root,join(root,"nx/graphs",toolingIdentity.slice(0,20))),temporary=requireNativeStoragePathV1(root,join(root,"nx/tmp")),native=requireNativeStoragePathV1(root,join(root,"nx/native"));
 return Object.freeze({root,environment:Object.freeze({NX_CACHE_DIRECTORY:cache,NX_WORKSPACE_DATA_DIRECTORY:data,NX_PROJECT_GRAPH_CACHE_DIRECTORY:data,NX_NATIVE_FILE_CACHE_DIRECTORY:native,TMPDIR:temporary,TMP:temporary,TEMP:temporary,SEMIO_NX_FIXED_STORAGE:root})});
}
/** 📋️ Requires the authored default to name its exact finite native backing. */
export function readAuthoredNxCacheDirectoryV1(workspace:string,value:unknown):string{
 const expected=join(repositoryStrictStorageV1(workspace).directory,"m/nx/artifacts");if(validateJsonSchemaSubset(cacheConfigurationSchema,value).length||typeof value!=="string"||join(workspace,value)!==expected)throw Error("Authored bounded Nx cache directory required");return expected;
}
/** 🗜️ Retains the genuine original image owner before launching any native Nx cache or database constructor. */
export async function withRepositoryNxStorageV1<T>(workspace:string,original:ScriptInvocation,body:(bind:(toolingIdentity:string)=>Promise<BoundedNxNamespaceV1>)=>Promise<T>):Promise<T>{
 readAuthoredNxCacheDirectoryV1(workspace,JSON.parse(readFileSync(join(workspace,"nx.json"),"utf8")).cacheDirectory);const declaration=repositoryStrictStorageV1(workspace),control=scriptCompilerAdmissionControlV1(original,"nx:fixed-storage",0),image=await openRepositoryCachePartitionV1(workspace,"rust",control);
 let live=true;try{return await body(async identity=>{if(!live)throw Error("Original Nx storage owner released");await image.recheck();const namespace=boundedNxNamespaceV1(image.root,identity);for(const name of["NX_CACHE_DIRECTORY","NX_WORKSPACE_DATA_DIRECTORY","NX_NATIVE_FILE_CACHE_DIRECTORY","TMPDIR"]){const path=namespace.environment[name]!;requireNativeStoragePathV1(image.root,path);mkdirSync(path,{recursive:true});}await image.recheck();return namespace;});}finally{live=false;await image.close();}
}

/** 🔒️ Revalidates the actual fixed image and original cache reader or writer through the native SDK operation. */
export async function withOriginalNxFixedCacheV1<T>(workspace:string,toolingIdentity:string,environment:Readonly<Record<string,string|undefined>>,paths:readonly string[],mode:"shared"|"exclusive",body:()=>Promise<T>):Promise<T>{
 const held=cacheScope.getStore();if(held){if(held.workspace!==workspace||held.toolingIdentity!==toolingIdentity||held.environment!==environment||held.mode==="shared"&&mode==="exclusive")throw Error("Original nested cache custody refused");if(!held.live())throw Error("Original Nx cache callback released");checkResourceLeaseControl(held.control);await held.recheck();const value=await withOutputCustodyV1(held,paths,mode,body);await held.recheck();return value;}
 return receiveScriptProcessInvocation(environment,async original=>{
  const declaration=repositoryStrictStorageV1(workspace),control=scriptCompilerAdmissionControlV1(original,"nx:fixed-cache",0),image=await openRepositoryCachePartitionV1(workspace,"rust",control);
  try{const namespace=boundedNxNamespaceV1(image.root,toolingIdentity);for(const name of Object.keys(namespace.environment))if(environment[name]!==namespace.environment[name])throw Error("Original bounded Nx namespace required");
   return await withResourceLeases({directory:declaration.leaseDirectory,resources:[{resource:"nx:fixed-cache:"+image.root,mode}],signal:control.signal,control},async()=>{await image.recheck();let live=true;const scope={workspace,toolingIdentity,environment,root:image.root,mode,control,leaseDirectory:declaration.leaseDirectory,original,live:()=>live,outputs:new Map(),recheck:image.recheck};try{return await cacheScope.run(scope,async()=>{const value=await withOutputCustodyV1(scope,paths,mode,body);await image.recheck();return value;});}finally{live=false;}});
  }finally{await image.close();}
 });
}

