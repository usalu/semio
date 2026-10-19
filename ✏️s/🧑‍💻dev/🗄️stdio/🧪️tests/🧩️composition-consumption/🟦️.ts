import type{ScriptInvocation}from"@semio-tech/process/routing";
import assert from"node:assert/strict";
import{writeFileSync}from"node:fs";
import{join}from"node:path";
import{pathToFileURL}from"node:url";
import type{JsonMap}from"@semio-tech/stdio-inventory-ts";
import{COMPOSITION_TYPESCRIPT_NAME}from"../../🏘️composition/🪪️identity/🟦️.ts";
import { buildArtifactTypeScriptPackageV1, checkArtifactTypeScriptSourceV1, withArtifactDestinationTaskV1 } from "@semio-tech/repo-lib/artifacts";

/** 🧪️ Consumes the exact 36-export composition while its actual bounded compiler and output remain retained. */
export function testStdioComposition(packageRoot:string,invocation:ScriptInvocation):Promise<void>{return buildArtifactTypeScriptPackageV1(packageRoot,COMPOSITION_TYPESCRIPT_NAME,invocation,{},destination=>withArtifactDestinationTaskV1(destination,invocation,async context=>{const probe=join(context.directory,"🧪️consumer.ts");writeFileSync(probe,'import { binary as definition } from "./🟦️.js";\nvoid definition;\n',{flag:"wx"});await checkArtifactTypeScriptSourceV1(probe,invocation,context.control);const facade=await import(`${pathToFileURL(context.output).href}?stdio=${Date.now()}`),entries=Object.entries(facade);assert.equal(entries.length,36);for(const[artifact,namespace]of entries)assert.equal((namespace as JsonMap).definition?.id,`s.stdio.${artifact}`);}));}
