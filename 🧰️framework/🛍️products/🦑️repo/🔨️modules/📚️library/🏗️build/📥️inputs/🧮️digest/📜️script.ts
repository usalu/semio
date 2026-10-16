#!/usr/bin/env bun
import{sourceClosureDigest}from"./🟨️.mjs";
import{readExactSourceJsonV1,withExactSourceReadBatchV1}from"./📥️read/🟨️.mjs";
import{relative,resolve,sep}from"node:path";
import{receiveScriptProcessInvocation,createIssuedResourceControlV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
export{sourceClosureDigest}from"./🟨️.mjs";
/** 📥️ Exact repository-relative files admitted by the Source owner. */
export interface SourceClosure{readonly version:1;readonly files:readonly string[]}
if(import.meta.main)await receiveScriptProcessInvocation(process.env,async original=>{const[command,path,...extra]=process.argv.slice(2);if(!path||!["digest","source"].includes(command??"")||command==="digest"&&extra.length)throw Error("Expected digest <closure manifest> or source <repository files...>");const root=process.cwd(),control=createIssuedResourceControlV1(original,"source:digest",120000),contract=command==="source"?{version:1,files:[path,...extra]}:await withExactSourceReadBatchV1(root,control,async batch=>{const file=await readExactSourceJsonV1(batch,relative(root,resolve(root,path)).split(sep).join("/"));if(!file)throw Error("Exact Source inventory required");return file.value;});process.stdout.write(await sourceClosureDigest(root,contract,control));});
