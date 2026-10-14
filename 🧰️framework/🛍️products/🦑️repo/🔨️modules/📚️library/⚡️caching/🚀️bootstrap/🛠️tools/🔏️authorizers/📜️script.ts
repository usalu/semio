#!/usr/bin/env bun
import {createHash} from "node:crypto";
import {lstat,mkdir,readFile,rename,writeFile} from "node:fs/promises";
import {extname,isAbsolute,join,relative,resolve} from "node:path";
import {captureNxAuthorizerSourcesV1,nxAuthorizerSourcesHoldV1,readNxAuthorizerSourceV1,nxAuthorizerExternalModulesV1,type NxAuthorizerSourceManifestV1} from "./🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {receiveScriptProcessInvocation,checkScriptInvocation,findWorkspaceRoot} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {acquireCompilerCapacityLeaseV1,scriptCompilerAdmissionControlV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";

if(import.meta.main)await receiveScriptProcessInvocation(process.env,async original=>{
 const[command,workspace,manifestPath,output,...rest]=process.argv.slice(2);
 if(command!=="compile"||!workspace||!manifestPath||!output||rest.length)throw Error("Expected compile <workspace> <source-manifest> <output>");
 const root=resolve(workspace),destination=resolve(output),signal=original.control.signal,bytes=await readFile(manifestPath),manifest=JSON.parse(bytes.toString())as NxAuthorizerSourceManifestV1;
 if(validateJsonSchemaSubset(schema,manifest).length||bytes.byteLength>1_048_576)throw Error("Closed release authorizer source manifest required");
 const captured=await captureNxAuthorizerSourcesV1(root,manifest.entry,signal);
 if(JSON.stringify(captured.manifest)!==JSON.stringify(manifest))throw Error("Original authorizer source changed");
 const expected=new Map(manifest.sources.map(row=>[row.path,row.sha256])),loaded=new Set<string>(),control=scriptCompilerAdmissionControlV1(original,"nx:authorizers:release",120000);
 const lease=await acquireCompilerCapacityLeaseV1({directory:join(findWorkspaceRoot(import.meta.dir),".cache/b/l/m/data"),control});
 try{
  const result=await Bun.build({entrypoints:[join(root,manifest.entry)],root,target:"node",format:"esm",naming:"semio-authorizers.mjs",treeShaking:true,minify:true,splitting:false,sourcemap:"none",metafile:true,define:{"import.meta.vitest":"undefined","import.meta.semioFrozenAuthorizers":"true"},external:[...nxAuthorizerExternalModulesV1,"bun:sqlite"],plugins:[{name:"semio-frozen-authorizers",setup(build){
   build.onLoad({filter:/.*/,namespace:"file"},async args=>{
    checkScriptInvocation(original);const path=relative(root,args.path).replaceAll("\\","/"),bytes=await readNxAuthorizerSourceV1(root,path,signal),sha256=createHash("sha256").update(bytes).digest("hex");
    if(expected.get(path)!==sha256)throw Error("Original authorizer source changed: "+path);loaded.add(path);
    await control.publish({resource:"nx-authorizers:"+path,mode:"shared",elapsedMs:0});await control.yieldContinuation();checkScriptInvocation(original);
    return {contents:Buffer.from(bytes).toString("utf8"),loader:extname(path)===".json"?"json":/\.[cm]?tsx?$/u.test(path)?"ts":"js"};
   });
  }}]});
  if(!result.success||result.outputs.length!==1||!result.metafile)throw Error("Authorizer release compilation refused: "+result.logs.map(row=>row.message).join("; "));
  const observed=Object.keys(result.metafile.inputs).map(path=>relative(root,isAbsolute(path)?path:resolve(root,path)).replaceAll("\\","/")).sort();
  if(JSON.stringify(observed)!==JSON.stringify([...loaded].sort())||[...loaded].some(path=>!expected.has(path))||!await nxAuthorizerSourcesHoldV1(root,captured,signal))throw Error("Original authorizer source coverage changed");
  const body=new Uint8Array(await result.outputs[0]!.arrayBuffer());if(body.byteLength>8_388_608||destination.length>220)throw Error("Authorizer output capacity exceeded");
  await mkdir(destination,{recursive:true});if(!(await lstat(destination)).isDirectory()||(await lstat(destination)).isSymbolicLink())throw Error("Original authorizer output directory required");
  const file=join(destination,"semio-authorizers.mjs"),temporary=file+"."+process.pid+".tmp";await writeFile(temporary,body,{flag:"wx"});checkScriptInvocation(original);await rename(temporary,file);
  console.log("[DEBUG] immutable authorizer release inputs="+loaded.size+" bytes="+body.byteLength);
 }finally{lease.release();}
});
