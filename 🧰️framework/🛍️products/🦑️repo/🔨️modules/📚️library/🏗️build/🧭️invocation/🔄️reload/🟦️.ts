import {relative,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {readSealedLoaderV1,readSealedPointerV1,readSealedReceiptV1,readSealedSnapshotV1,sealedEntryOfV1} from "../../🏛️sealed/🟦️.ts";
import {hostImplementationV1,hostServiceRevisionPathsV1} from "../🛰️plan/🟦️.ts";

/** 🏷️ What the tree wants the host service to run: the digest of that implementation, and the reason it must not be loaded when it cannot be sealed (bilingual). */
export type HostIdentityV1=Readonly<{implementation:string;blocked?:string}>;

/** 🔌️ The seams of the tests: the entry of the store, the revision of the live code and the syntax check of its files. */
export type HostIdentityPortsV1=Readonly<{entry?:string;revision?:()=>string;syntaxRows?:(root:string,paths:readonly string[])=>Promise<readonly Readonly<{path:string;line:number;column:number;message:string}>[]>}>;

const SHA256=/^[0-9a-f]{64}$/u,library=fileURLToPath(new URL("../../",import.meta.url));
const sealedEntryPathV1=(root:string):string=>relative(root,fileURLToPath(new URL("../../🏛️sealed/🚪️entry/🟦️.ts",import.meta.url))).split("\\").join("/");

/** 🧭️ The identity a running host service compares itself with, from the tree it was started in. A sealed host (the sealed door digest is in its environment) follows the pointer of the store: a pointer exists only for a snapshot that was parsed, booted and checked by the promoter, and it is read back intact (snapshot, loader and receipt) before it counts, so a promotion that failed or a damaged store never moves it. A live host recomputes its identity from the files that make up the service (`HostPolicy.serviceRevisionFiles`) and accepts a changed one only when every named file still parses, so a half-finished edit never replaces a working service. The answer keeps `running` as the implementation whenever nothing newer is wanted.
 * @see ../🛰️plan/🟦️.ts `hostImplementationV1`, ../../🏛️sealed/🟦️.ts the store, ../../🏡️resident/🔄️reload/🟨️.mjs the caller */
export function hostReloadIdentityV1(root:string,environment:NodeJS.ProcessEnv,running:string,ports:HostIdentityPortsV1={}):()=>Promise<HostIdentityV1>{
 return async()=>{
  const door=environment.SEMIO_SEALED_DOOR_DIGEST;
  if(door!==undefined&&SHA256.test(door)){
   const store=sealedEntryOfV1(root,ports.entry??sealedEntryPathV1(root)),pointer=readSealedPointerV1(store);
   if(pointer===undefined)return{implementation:running,blocked:"no sealed snapshot is published / kein versiegelter Schnappschuss veröffentlicht"};
   if(pointer.digest===running)return{implementation:running};
   const named=pointer.digest.slice(0,12);
   if(readSealedSnapshotV1(store,pointer)===undefined||readSealedLoaderV1(store,pointer)===undefined)return{implementation:pointer.digest,blocked:`the sealed snapshot ${named} is damaged or incomplete / der versiegelte Schnappschuss ${named} ist beschädigt oder unvollständig`};
   if(readSealedReceiptV1(store,pointer.digest)===undefined)return{implementation:pointer.digest,blocked:`the sealed snapshot ${named} has no seal receipt / der versiegelte Schnappschuss ${named} hat keine Siegelquittung`};
   return{implementation:pointer.digest};
  }
  const wanted=hostImplementationV1(root,environment,ports.revision);
  if(wanted===running)return{implementation:wanted};
  const paths=hostServiceRevisionPathsV1().map(path=>relative(root,resolve(library,path)).split("\\").join("/")),rows=await(ports.syntaxRows??(async(base,files)=>(await import("../../🏛️sealed/📤️promotion/🟦️.ts")).syntaxRowsOfPathsV1(base,files)))(root,paths);
  return rows.length===0?{implementation:wanted}:{implementation:wanted,blocked:`${rows[0]!.path}:${rows[0]!.line}:${rows[0]!.column} ${rows[0]!.message}`};
 };
}
