#!/usr/bin/env bun
/** 🕸️ DAG public semantic parent package and strict owning Source consumers. */
import{runArtifactTypeScriptPackageMain}from"../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/📜️script.ts";
import{runRepositoryCommand}from"../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
import{readdirSync}from"node:fs";
import{join,resolve}from"node:path";
if(process.argv[2]==="verify"&&process.argv[3]==="snapshot-sqlite-source"){
 const repo=resolve(import.meta.dir,"../../../../../../.."),files:string[]=[],pending=[resolve(import.meta.dir,"../../🏅️standards")];
 while(pending.length){const dir=pending.pop()!;for(const entry of readdirSync(dir,{withFileTypes:true})){const path=join(dir,entry.name);if(entry.isDirectory()&&entry.name!=="🧪️tests")pending.push(path);else if(entry.isFile()&&entry.name.endsWith(".ts"))files.push(path);}}
 await runRepositoryCommand(process.execPath,["x","tsc",...files,"--noEmit","--strict","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--resolveJsonModule","--allowImportingTsExtensions","--esModuleInterop","--skipLibCheck"],repo,"dag-dag:published-source-consumers",120000);
 console.log(`[dag-dag] checked published consumers=${files.length}`);
}else await runArtifactTypeScriptPackageMain(import.meta.dir,"@semio-tech/dag-dag",{suites:["🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📋️contract/🟦️.ts"]});
