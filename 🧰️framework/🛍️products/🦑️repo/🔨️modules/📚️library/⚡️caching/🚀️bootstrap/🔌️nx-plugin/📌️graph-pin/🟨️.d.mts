import type { BuildGraphV1 } from "../../../../🏗️build/🕸️graph/📋️model/🟦️.ts";
/** 📌️ Reads the exact portable graph pinned by this owned build invocation under the injected implementation-identity check. */
export function withNxBuildGraphPublicationV1<T>(workspaceRoot:string,environment:Readonly<Record<string,string|undefined>>,body:(publication:Readonly<{graph:BuildGraphV1;digest:string}>)=>Promise<T>,implementation:()=>unknown):Promise<T>;
/** 🔏️ Exposes only immutable private backend DATA read under its genuine received Graph task. */
export function readPrivateNxBuildGraphV1(workspaceRoot:string,environment:Readonly<Record<string,string|undefined>>,implementation:()=>unknown):Promise<BuildGraphV1>;
