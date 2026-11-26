/** 🔁️ Law: the installed dependency path scan names the path and the stat keys of a directory that changed under it, and rescans from scratch (bounded by `maximumScanAttempts`, ledger and counters reset) so a change it observed once does not fail the stage when the second walk is stable; the same digest as an undisturbed scan.
 * @see ../../🟦️.ts */
import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {lstatSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {javascriptInstallSchemaV1 as installSchema} from "../../🧱️schemas/🟦️.ts";
import { loadProductFunctionsV1 } from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧩️extracted-function/🟦️.ts";

type Scan=(root:string,control:unknown,check:()=>void,ledger?:{rows:unknown[];directory:(...rest:unknown[])=>void})=>Promise<{entries:number;sha256:string}>;
const build=async(attempts:number,stat:unknown)=>{
 const descriptors=await import("@semio-tech/framework-filesystem/observation/descriptor");
 return (await loadProductFunctionsV1(new URL("../../🟦️.ts",import.meta.url),["scanJavascriptDependencyInstalledPathsV1"],{installSchema:{$defs:{InstalledPathProtocol:{const:{...installSchema.$defs.InstalledPathProtocol.const,maximumScanAttempts:attempts}}}},createHash,lstatSync:stat,withArtifactReadDirectoryV1:descriptors.withArtifactReadDirectoryV1,join,requireIssuedResourceControlV1:()=>{},checkResourceLeaseControl:()=>{}})).scanJavascriptDependencyInstalledPathsV1 as Scan;
};
const control={signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>{},yieldContinuation:async()=>{}};
const disturb=(target:string,key:"mtimeNs"|"size")=>{let calls=0;return((path:string,options:unknown)=>{const facts=(lstatSync as any)(path,options);if(path===target&&++calls===2)return Object.create(facts,{[key]:{value:facts[key]+1n}});return facts;})as unknown;};
test("a directory that changed once is named with its stat keys and the scan restarts and agrees with an undisturbed scan",async()=>{
 const root=mkdtempSync(join(tmpdir(),"scan-rescan-"));
 try{
  for(let directory=0;directory<3;directory++){mkdirSync(join(root,"d"+directory));for(let file=0;file<5;file++)writeFileSync(join(root,"d"+directory,"f"+file+".js"),"x"+directory);}
  const clean=await(await build(3,lstatSync))(root,control,()=>{}),target=join(root,"d1");
  await expect((await build(1,disturb(target,"mtimeNs")))(root,control,()=>{})).rejects.toThrow(new RegExp("Original installed dependency directory changed: .*d1 \\(mtimeNs \\d+ -> \\d+\\)","u"));
  const seen:string[]=[],original=console.error;console.error=(...rest:unknown[])=>{seen.push(rest.join(" "));};
  const rows:unknown[]=[],ledger={rows,directory:(...row:unknown[])=>{rows.push(row);}};
  try{const again=await(await build(3,disturb(target,"size")))(root,control,()=>{},ledger);expect(again.sha256).toBe(clean.sha256);expect(again.entries).toBe(clean.entries);}finally{console.error=original;}
  expect(seen.filter(line=>line.includes("installed-dependency-scan-restart")).length).toBe(1);
  expect(rows.length).toBe(4);
 }finally{rmSync(root,{recursive:true,force:true});}
},120000);
