import type {BuildBackendV1} from "../🟦️.ts";
import {buildBackendRowV1} from "../🟦️.ts";

const loaded=new Map<string,Promise<BuildBackendV1>>();
let verified:Promise<unknown>|undefined;
/** ✔️ Validates the registry once, on first driver use. */
const verifiedRegistryV1=():Promise<unknown>=>verified??=import("../📜️registry/🟦️.ts").then(module=>module.readBackendRegistryV1());

/** 🚚️ Loads the adapter module a registry row names and checks that it exports a backend of the same id; only command kernels import this module, so the door never captures the adapters.
 * @see ../🚀️launch/🟦️.ts what the door itself needs of a backend */
export async function loadBuildBackendV1(id:string):Promise<BuildBackendV1>{
 await verifiedRegistryV1();
 const row=buildBackendRowV1(id);
 let pending=loaded.get(row.id);
 if(!pending){
  pending=import(new URL(row.module,new URL("../🟦️.ts",import.meta.url)).href).then((module:{backend?:BuildBackendV1})=>{
   if(!module.backend||module.backend.id!==row.id)throw Error(`Build backend module ${row.module} must export backend with id ${row.id}`);
   return module.backend;
  });
  loaded.set(row.id,pending);
 }
 return pending;
}
