#!/usr/bin/env bun
import{selectedCargoLockDigest}from"./📥️read/🟨️.mjs";
import{receiveScriptProcessInvocation,createIssuedResourceControlV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
export{selectedCargoLockDigest}from"./📥️read/🟨️.mjs";
if(import.meta.main)await receiveScriptProcessInvocation(process.env,async original=>{const[command,manifest,...extra]=process.argv.slice(2);if(command!=="digest"||!manifest||extra.length)throw Error("Expected digest <manifest>");process.stdout.write(await selectedCargoLockDigest(process.cwd(),manifest,createIssuedResourceControlV1(original,"source:lock-digest",120000)));});
