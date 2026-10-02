import { readFileSync } from "node:fs";
import { isAbsolute, resolve, join } from "node:path";
import { validateJsonSchemaSubset } from "../../🧬️schema/✅️validator/🟦️.ts";

/** 📋️ Gives an owner its explicitly selected execution and compiler state stores. */
export type ProcessOwnerContextV1=Readonly<{version:1;cwd:string;cacheRoot:string;leaseDirectory:string}>;
const schema=JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json",import.meta.url),"utf8"));

/** 🔐️ Refuses absent, malformed or other-owner context before changing build state. */
export function readProcessOwnerContextV1(environment:Readonly<Record<string,string|undefined>>,cwd:string):ProcessOwnerContextV1{
  if(!environment.SEMIO_PROCESS_OWNER_CONTEXT)throw Error("Explicit process owner context required");
  const value=JSON.parse(environment.SEMIO_PROCESS_OWNER_CONTEXT) as ProcessOwnerContextV1;
  if(validateJsonSchemaSubset(schema,value).length||![value.cwd,value.cacheRoot,value.leaseDirectory].every(isAbsolute)||value.cwd!==resolve(cwd))throw Error("Invalid process owner context");
  return value;
}

/** 🗂️ Selects exact safe leaves inside the caller's declared compiler store. */
export function processCacheDirectoryV1(context:ProcessOwnerContextV1,...segments:string[]):string{
  if(!segments.length||segments.some(part=>!/^[A-Za-z0-9_-]+$/u.test(part)))throw Error("Invalid process cache identity");
  return join(context.cacheRoot,...segments);
}
