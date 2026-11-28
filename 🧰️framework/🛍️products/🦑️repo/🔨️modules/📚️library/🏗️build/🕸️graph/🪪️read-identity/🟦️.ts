import {existsSync} from "node:fs";
import {join,relative,resolve,sep} from "node:path";
import {fileURLToPath} from "node:url";
import {createSidecarClosureWatchV1} from "../../🌐️remote-execution/🛰️sidecar/🧮️closure/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🪪️ The declared scope of the code identity of a graph read: the entries whose import closure is the code of the read and the folders that are services around it. See `./🧬️schema/🔣️.json`. */
export const GRAPH_READ_CODE_SCOPE_V1=Object.freeze({...schema.definitions.Scope.const});

const LIBRARY_V1=fileURLToPath(new URL("../../",import.meta.url)),SEALED_V1=/^[0-9a-f]{64}$/u;

/** 🔌️ The seams of the tests: the library folder, the scope, the environment and the syntax check of changed files. */
export type GraphReadCodePortsV1=Readonly<{library?:string;scope?:Readonly<{entries:readonly string[];excluded:readonly string[]}>;environment?:NodeJS.ProcessEnv;syntaxRows?:(root:string,paths:readonly string[])=>Promise<readonly Readonly<{path:string;line:number;column:number;message:string}>[]>}>;

/** 🧬️ A watch over the code of a graph read: the digest of the import closure of the scope's entries, folders of `excluded` neither read nor followed. A file outside that closure, or inside an excluded folder, moves nothing. */
export function graphReadCodeWatchV1(ports:GraphReadCodePortsV1={}):ReturnType<typeof createSidecarClosureWatchV1>{
 const library=ports.library??LIBRARY_V1,scope=ports.scope??GRAPH_READ_CODE_SCOPE_V1,folders=scope.excluded.map(path=>resolve(library,path)+sep),root=library.endsWith(sep)?library:library+sep;
 return createSidecarClosureWatchV1(scope.entries.map(entry=>join(library,entry)),[],root,undefined,{exclude:path=>folders.some(folder=>path.startsWith(folder))});
}

const watchesV1=new Map<string,ReturnType<typeof graphReadCodeWatchV1>>(),runningV1=new Map<string,string>();

/** 🪪️ The code identity this process runs (fixed at the first call for `root`) and a reader of the identity the tree wants now; `current` answers undefined when the tree cannot be sealed or holds a file that does not parse. A sealed host (`SEMIO_SEALED_DOOR_DIGEST`) keeps the identity of its promoted snapshot; a live one is the closure digest of `graphReadCodeWatchV1`.
 * @see ../🛑️custody/🟦️.ts the caller */
export async function graphReadCodeIdentityV1(root:string,ports:GraphReadCodePortsV1={}):Promise<{digest:string;current:()=>Promise<string|undefined>}>{
 const environment=ports.environment??process.env;
 try{
  const door=environment.SEMIO_SEALED_DOOR_DIGEST;
  if(door!==undefined&&SEALED_V1.test(door)){
   const{hostImplementationV1}=await import("../../🧭️invocation/🛰️plan/🟦️.ts"),{hostReloadIdentityV1}=await import("../../🧭️invocation/🔄️reload/🟦️.ts"),digest=runningV1.get(root)??hostImplementationV1(root,environment);
   runningV1.set(root,digest);const identify=hostReloadIdentityV1(root,environment,digest);
   return{digest,current:async()=>{const found=await identify();return found.blocked?undefined:found.implementation;}};
  }
  const library=ports.library??LIBRARY_V1,key=root+"\0"+library,watch=watchesV1.get(key)??graphReadCodeWatchV1(ports);
  watchesV1.set(key,watch);const digest=runningV1.get(key)??watch.baseline;runningV1.set(key,digest);
  return{digest,current:async()=>{
   const found=watch.closure().digest;
   if(found===digest)return found;
   const paths=watch.changes().filter(path=>existsSync(resolve(library,path))).map(path=>relative(root,resolve(library,path)).split(sep).join("/")),rows=await(ports.syntaxRows??(async(base,files)=>(await import("../../🏛️sealed/📤️promotion/🟦️.ts")).syntaxRowsOfPathsV1(base,files)))(root,paths.filter(path=>/\.(?:ts|mjs)$/u.test(path)));
   return rows.length===0?found:undefined;
  }};
 }catch{return{digest:"",current:async()=>undefined};}
}
