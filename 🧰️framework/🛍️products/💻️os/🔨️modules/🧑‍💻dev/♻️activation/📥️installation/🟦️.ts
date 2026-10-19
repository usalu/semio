import { artifactsStorageSchema as publicationSchema } from "@semio-tech/repo-lib/artifacts";
import {createRequestedIssuedResourceControlV1,type ScriptInvocation} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "@semio-tech/process/compiler-capacity";
import { readNativeCallerAuthorityV1, withRepositoryArtifactV1, withRepositoryArtifactTaskV1, withArtifactScratchV1, withArtifactDestinationTaskV1, withRepositoryArtifactDestinationV1, stageRepositoryArtifacts } from "@semio-tech/repo-lib/artifacts";
import { withRepositoryCacheStoreV1 } from "@semio-tech/repo-lib/storage";
/** 🧩️ Semantic activation installation owner. */

import { artifactFiles } from "@semio-tech/framework-plugin/browser-bundle/distribution/inventory";


import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { createHash } from "node:crypto";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { generatePluginRegistry, type DeployedRegistryEntryV1 } from "@semio-tech/plugin-registry/discovery";

import {
  ensurePreview2ShimVendorAt,
} from "@semio-tech/framework-plugin/browser-bundle/materialization/preview2-shim-patches";
import {
  hostShimSource,
} from "@semio-tech/framework-plugin/browser-bundle/materialization/host-shim-source";
import {
  PLUGIN_HOST_SHIM_FILE,
  GUESTSLIM_FONT_RELATIVE,
  SHARD_WORKER_FILE,
  type PluginWebMaterializeContext,
} from "@semio-tech/framework-plugin/browser-bundle/materialization/contract";
import {
  PREVIEW2_VENDOR_RELATIVE,
  rewritePreview2ShimImportSource,
} from "@semio-tech/framework-plugin/browser-bundle/imports";
import {
  pluginComponentBridgeSource,
} from "@semio-tech/framework-plugin/browser-bundle/materialization/component-bridge-source";
import {
  rewriteJcoComponentAssetUrls,
  rewriteJcoAsyncResultLifting,
} from "@semio-tech/framework-plugin/browser-bundle/materialization/jco-module-rewrites";
import {
  shardWorkerSource,
} from "@semio-tech/framework-plugin/browser-bundle/materialization/shard-worker-source";
import {
  rewritePreview2ShimImports,
  transpilePluginComponentAsync,
} from "@semio-tech/framework-plugin/browser-bundle/materialization";

import { defaultExtensionInstallRoot, EXTENSION_INSTALL_META, EXTENSION_WATCH_MARKER } from "../../../🔌️plugin/🏪️store/📥️installation/🟦️.ts";

import { MODULE_BRIDGE_FILE, MODULE_SHARD_DIRECTORY, MODULE_HOT_SWAP_FILE, MODULE_PLUGIN_ROUTE, MODULE_EXTENSION_ROUTE, moduleDirectoryName, parseModuleDirectories, moduleIdForDirectoryName, moduleRoutePath,componentModuleOwnerV1 } from "@semio-tech/plugin-registry/deployment";



/** 🔏️ Includes every completed module/support byte in its runtime activation identity. */
async function activationFilesDigest(files: ReadonlyMap<string, string>, signal: AbortSignal): Promise<string> {
  const hash = createHash("sha256");
  for (const [name, path] of [...files].sort(([a], [b]) => a < b ? -1 : a > b ? 1 : 0)) {
    signal.throwIfAborted();
    hash.update(JSON.stringify([name.replaceAll("\\", "/"), statSync(path).size]) + "\n");
    for await (const chunk of createReadStream(path, { signal })) hash.update(chunk);
  }
  return hash.digest("hex");
}

/** 🧩️ Installs source-owned extensions into this development variant's runtime namespace. */
async function publishActivatedExtension(workspace:string,invocation:ScriptInvocation,target:DeployedRegistryEntryV1,source:string,installRoot:string,artifactSha256:string,rebuiltAt:number,signal:AbortSignal):Promise<void>{
 const mode=readNativeCallerAuthorityV1(invocation.capabilities).original.buildMode,phase=createRequestedIssuedResourceControlV1(scriptCompilerAdmissionControlV1(invocation,"browser:extension-activation",0),0,signal),policy=publicationSchema.definitions.Protocol.const;
 const name=moduleDirectoryName(target.pluginId,parseModuleDirectories({version:1,modules:[{pluginId:target.pluginId,directoryName:target.directoryName}]})),output=join(installRoot,name);
 await withRepositoryCacheStoreV1(workspace,"products",invocation,phase,(store,storePhase)=>withRepositoryArtifactV1({workspace,invocation,storage:store.storage,control:store.control,phaseControl:storePhase,request:{version:1,path:source,owner:componentModuleOwnerV1(target.cratePath),maximumBytes:policy.maximumOutputBytes+policy.maximumManifestBytes,maximumFiles:policy.maximumFiles}},reader=>withRepositoryArtifactTaskV1(reader,invocation,artifact=>withArtifactScratchV1({workspace,invocation,storage:store.storage,control:store.control,phaseControl:artifact.control,request:{version:1,mode,owner:"browser/extension/"+target.pluginId}},token=>withArtifactDestinationTaskV1(token,invocation,async sink=>{
  const files=new Map(artifact.files.map(name=>[name,join(artifact.path,name)]));
  for(const[name,path]of files)if(name.endsWith(".js")){sink.control.signal.throwIfAborted();const rewritten=rewritePreview2ShimImportSource(readFileSync(path,"utf8"),`../..${MODULE_PLUGIN_ROUTE}/${PREVIEW2_VENDOR_RELATIVE}/`),destination=join(sink.directory,name);mkdirSync(dirname(destination),{recursive:true});writeFileSync(destination,rewritten);files.set(name,destination);}
  const record={extensionId:target.pluginId,directoryName:name,version:"0.0.0-dev",label:target.pluginId,extends:target.extends??"",moduleUrl:`${MODULE_EXTENSION_ROUTE}/${name}/${MODULE_BRIDGE_FILE}`,packageHash:artifactSha256,installedAt:rebuiltAt},metadata=join(sink.directory,EXTENSION_INSTALL_META);
  writeFileSync(metadata,JSON.stringify(record)+"\n");files.set(EXTENSION_INSTALL_META,metadata);
  await withRepositoryArtifactDestinationV1({workspace,invocation,storage:store.storage,control:store.control,phaseControl:sink.control,directory:output},destination=>stageRepositoryArtifacts(destination,"development-extension:"+target.pluginId,files));await artifact.recheck();console.log(`Activated extension ${target.pluginId}`);
 })))));
}

export { activationFilesDigest, publishActivatedExtension };
