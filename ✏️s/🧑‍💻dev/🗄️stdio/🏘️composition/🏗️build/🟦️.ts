import{resolve}from"node:path";
import type{ScriptInvocation}from"@semio-tech/process/routing";
import { buildArtifactTypeScriptPackageV1, checkArtifactTypeScriptSourceV1 } from "@semio-tech/repo-lib/artifacts";
import{COMPOSITION_TYPESCRIPT_NAME}from"../🪪️identity/🟦️.ts";

/** 🧾️ Checks real composition Source through the bounded shared no-output compiler. */
export function runStdioTypeScriptCompiler(source:string,invocation:ScriptInvocation):Promise<void>{return checkArtifactTypeScriptSourceV1(source,invocation);}

/** 🏗️ Publishes the actual composition through the shared fixed mode output and package binding owner. */
export function buildStdioComposition(packageRoot:string,invocation:ScriptInvocation):Promise<void>{return buildArtifactTypeScriptPackageV1(packageRoot,COMPOSITION_TYPESCRIPT_NAME,invocation);}

/** 🔎️ Checks composition sources through the independently installed TypeScript compiler. */
export async function checkStdioComposition(packageRoot:string,invocation:ScriptInvocation):Promise<void>{await runStdioTypeScriptCompiler(resolve(packageRoot,"../../🟦️.ts"),invocation);}
