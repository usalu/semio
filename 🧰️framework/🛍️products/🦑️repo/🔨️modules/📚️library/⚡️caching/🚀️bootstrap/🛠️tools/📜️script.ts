import{requireIssuedResourceControlInvocationV1,requireIssuedResourceControlRefinementV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{isToolingHeadroomRefusalV1,withToolingHeadroomRecoveryV1,type ToolingHeadroomRefusalV1}from"./🧹️retention/📏️headroom/🟦️.ts";
import {nxSdkCompilerEnvironmentV1} from "./📦️sdk/📥️source/🔗️derivative/🎛️environment/🟦️.ts";
import{requireNxSdkDerivativePatchImagesV1,requireNxSdkDerivativeReceiptImagesV1}from"./📦️sdk/📥️source/🔗️derivative/🩹️patch/🟦️.ts";
import snapshotPolicy from"./🔏️authorizers/📦️snapshot/🔣️policy.json";
import{withNxSdkDerivativeProjectionV1}from"./📦️sdk/📥️source/🔗️derivative/🟦️.ts";
import{withNxSdkDerivativeReaderV1}from"./📦️sdk/📥️source/🔗️derivative/📥️reader/🟦️.ts";
import{nxSdkDerivativeHeadroomBytesV1,captureNxSdkPublicationInputsV1,withNxSdkDerivativePublicationV1,type NxSdkClonePublicationV1}from"./📦️sdk/🎟️capacity/🟦️.ts";
import{nxDependencyInstallFilesV1}from"./📦️sdk/📥️source/📐️projection/🟦️.ts";
import{ensureNxPristineSdkV1,requireNxPristineSdkV1}from"./📦️sdk/📥️source/🧊️pristine/🟦️.ts";
import {bindNxSdkSnapshotBinariesV1} from "./📦️sdk/🧊️binary/📥️publication/🟦️.ts";
import {verifyNxSdkBinaryLinksV1} from "./📦️sdk/🧊️binary/🔎️verification/🟦️.ts";
import {nxToolingInstallIdentityV1} from "./📦️sdk/🪪️identity/🟦️.ts";
export {nxToolingInstallIdentityV1} from "./📦️sdk/🪪️identity/🟦️.ts";
import {prepareManagedToolingStorageV1,withManagedToolingProvisionV1,type ManagedToolingPublisherV1} from "./🧹️retention/📥️physical/🟦️.ts";
import retentionSchema from "./🧹️retention/📥️physical/🧬️schema/🔣️.json";
import {preflightNxAuthorizersV1} from "./🔏️authorizers/📥️loading/🟦️.ts";
import {repositoryCachePartitionRootV1} from "../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import {withRepositoryToolingStoreV1,type RepositoryToolingStoreV1} from "../../💾️storage/🔒️enforcement/🛠️tools/🟦️.ts";
import {requireNativeStorageReservationV1} from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import{statfs}from"node:fs/promises";
import{readNxSdkSourceIndexV1,readNxSdkSourceMetadataV1,nxSdkSourceHashV1,nxSdkSourceIndexPolicyV1}from"./📦️sdk/📥️source/🔗️derivative/🔏️index/🟦️.ts";
import{nxSdkDerivativeDispositionV1}from"./📦️sdk/📥️source/🔗️derivative/🟦️.ts";
import { createHash, randomUUID } from "node:crypto";
import { existsSync,lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, symlinkSync, unlinkSync, writeFileSync } from "node:fs";
import {toolPatchReceipt,assertToolingPatchV1,toolPatchAdditions,applyToolingPatchImagesV1} from "./🩹️patch/🟦️.ts";
import {captureNxAuthorizerSourcesV1} from "./🔏️authorizers/🟦️.ts";
import {snapshotNxAuthorizerSourcesV1,compileNxAuthorizerSnapshotV1} from "./🔏️authorizers/📦️snapshot/🟦️.ts";
import {nxAuthorizerReceiptHoldsV1} from "./🔏️authorizers/📤️receipt/🟦️.ts";
import {cloneNxSdkSnapshotV1,nxSdkSnapshotIdentityV1,nxSdkSnapshotReceiptHoldsV1} from "./📦️sdk/📥️snapshot/🟦️.ts";
import {receiveScriptProcessInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { createRequire } from "node:module";
import { basename, dirname, isAbsolute, join, relative } from "node:path";
import {runOwnedCommand} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import { prepareBun } from "./🟦️bun/📜️script.ts";
import { withResourceLeases } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

export type NxTooling = { readonly cli: string; readonly modulePath: string; readonly bun: string; readonly authorizer:string };
const RECIPE = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools";
const STORE = ".cache/b/t/m/nx";

/** 🕹️ Executes setup children with the original awaited progress, continuation, signal and remaining deadline. */
export async function runNxSetupCommandV1(command:string,args:string[],cwd:string,label:string,control:ResourceLeaseControl,environment:Readonly<Record<string,string|undefined>>):Promise<void>{
 const started=performance.now(),advance=async()=>{checkResourceLeaseControl(control);await control.publish({resource:label,mode:"exclusive",elapsedMs:performance.now()-started});checkResourceLeaseControl(control);await control.yieldContinuation();checkResourceLeaseControl(control);};
 await advance();const remaining=checkResourceLeaseControl(control);if(remaining!==null&&Math.floor(remaining)<1)throw Error("Original compiler admission deadline exhausted");await runOwnedCommand(command,args,cwd,label,remaining===null?0:Math.floor(remaining),{signal:control.signal,env:environment,onProgress:advance});await advance();
}

/** 🛣️ Rejects substituted directories before creating a tooling installation. */
function regularDirectory(path: string): void {
  for (let current = path; ; current = dirname(current)) {
    try { const stat = lstatSync(current); if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error(`Invalid tooling directory: ${current}`); }
    catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; }
    if (dirname(current) === current) break;
  }
  mkdirSync(path, { recursive: true });
}

/** 📂️ Distinguishes absent snapshots from substituted cache paths. */
function existsOwnedDirectory(path:string):boolean{
  try{const status=lstatSync(path);if(!status.isDirectory()||status.isSymbolicLink())throw Error("Original SDK snapshot directory required");return true;}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return false;throw error;}
}

/** 🔗️ Selects an immutable toolset through Nx's native installation directory. */
async function activateNxToolsOwnedV1(workspace: string, tooling: NxTooling, store:RepositoryToolingStoreV1,control:ResourceLeaseControl): Promise<void> {
  requireIssuedResourceControlRefinementV1(store.control,control);checkResourceLeaseControl(control);requireNativeStorageReservationV1(store.storage,store.control);await store.storage.recheck();checkResourceLeaseControl(control);const signal=control.signal;signal.throwIfAborted();
  const root = realpathSync(workspace), nx = join(root, ".nx"), pointer = join(nx, "installation"), selected = realpathSync(dirname(tooling.modulePath));
  if (dirname(selected) !== join(root, STORE) || !/^[a-f0-9]{64}$/.test(basename(selected))) throw new Error("Nx tooling must be an owned immutable installation");
  const selectedMarker=JSON.parse(readFileSync(join(selected,".semio-nx-tooling.json"),"utf8"));if(selectedMarker.digest!==basename(selected)||!nxAuthorizerReceiptHoldsV1(selected,selectedMarker.authorizers))throw Error("Original immutable activation receipt required");await preflightNxAuthorizersV1(selected,control,store.environment);await store.recheck();checkResourceLeaseControl(control);
  regularDirectory(nx);
  await withResourceLeases({ directory: join(root, ".cache/b/l/m/data"), signal,control, resources: [{ resource: "nx-tooling-activation", mode: "exclusive" }] }, async () => {
    let previous: string | undefined;
    let present = true;
    try { lstatSync(pointer); }
    catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; present = false; }
    if (present) {
      if (!lstatSync(pointer).isSymbolicLink()) throw new Error("The Nx installation directory has another owner");
      previous = realpathSync(pointer);
      const marker=JSON.parse(readFileSync(join(previous,".semio-nx-tooling.json"),"utf8"));
      if(isAbsolute(relative(root,previous))||relative(root,previous).startsWith("..")||!/^[a-f0-9]{64}$/.test(basename(previous))||marker.version!==1||marker.digest!==basename(previous)||!nxAuthorizerReceiptHoldsV1(previous,marker.authorizers))throw new Error("The Nx installation link has another owner");
      if (previous === selected) return;
    }
    const staging = join(nx, `.installation-${randomUUID()}`), kind = process.platform === "win32" ? "junction" : "dir";
    try {
      symlinkSync(selected, staging, kind);
      signal.throwIfAborted();
      try { renameSync(staging, pointer); }
      catch (error) {
        if (process.platform !== "win32" || !previous || !["EEXIST", "EPERM", "EACCES"].includes((error as NodeJS.ErrnoException).code ?? "")) throw error;
        unlinkSync(pointer);
        try { renameSync(staging, pointer); }
        catch (failure) { symlinkSync(previous, pointer, kind); throw failure; }
      }
    } finally { try { unlinkSync(staging); } catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; } }
  });
}

/** 🧷️ Installs the authored pinned toolset independently of the source package's workspace development dependencies. */
export function nxToolingInstallManifestV1(recipe:{name:string;private:boolean;packageManager:string;dependencies:Record<string,string>;semio:{toolPatches:Record<string,string>}}){
  return {name:recipe.name,private:recipe.private,packageManager:recipe.packageManager,dependencies:recipe.dependencies,patchedDependencies:recipe.semio.toolPatches};
}

/** 🧩️ Gives Bun each patch's immutable content identity while sharing ordinary package downloads. */
export function nxToolingInstallFilesV1(recipe: Parameters<typeof nxToolingInstallManifestV1>[0], lock: unknown, sources: ReadonlyMap<string,Uint8Array>): Map<string,Buffer> {
  if(!lock||typeof lock!=="object"||Array.isArray(lock))throw Error("Pinned tooling lock must be an object");
  const captured=lock as Readonly<Record<string,unknown>>;
  if(JSON.stringify(captured.patchedDependencies)!==JSON.stringify(recipe.semio.toolPatches))throw Error("Pinned tooling patch authority changed");
  const patches:Record<string,string>={},files=new Map<string,Buffer>();
  for(const [name,path] of Object.entries(recipe.semio.toolPatches)){
    const bytes=sources.get(path);if(!bytes)throw Error("Pinned tooling patch bytes required");
    assertToolingPatchV1(Buffer.from(bytes).toString("utf8"));
    const artifact=`patches/${createHash("sha256").update(bytes).digest("hex")}.patch`;patches[name]=artifact;files.set(artifact,Buffer.from(bytes));
  }
  return new Map([["package.json",Buffer.from(JSON.stringify({...nxToolingInstallManifestV1(recipe),patchedDependencies:patches},null,2)+"\n")],["bun.lock",Buffer.from(JSON.stringify({...captured,patchedDependencies:patches},null,2)+"\n")],...files]);
}

/** 📸️ Captures the same detached installation images for read-only planning and the eventual issued publication. */
async function captureNxToolingInputsV1(workspace:string,signal:AbortSignal){
  const root = realpathSync(workspace), recipe = join(root, RECIPE), files = new Map<string, Buffer>();
  for (const name of ["package.json", "bun.lock"]) files.set(name, readFileSync(join(recipe, name)));
  const manifest = JSON.parse(files.get("package.json")!.toString());
  if (!/^bun@\d+\.\d+\.\d+$/.test(manifest.packageManager)) throw new Error("Invalid Nx tooling Bun pin");
  const patches = manifest.semio?.toolPatches;
  if (!patches || typeof patches!=="object" || Array.isArray(patches) || Object.entries(patches).some(([name,path])=>!name || typeof path!=="string")) throw new Error("Invalid authored tooling patch contribution");
  for (const path of Object.values(patches) as string[]) {
    const canonical = realpathSync(join(root, path)), local = relative(root, canonical);
    if (isAbsolute(path) || path.includes("\\") || path.split("/").some(part => !part || part === "." || part === "..") || isAbsolute(local) || local.startsWith("..")) throw new Error(`Invalid tooling patch: ${path}`);
    files.set(path, readFileSync(canonical));
  }
  const capturedLock=Bun.JSONC.parse(files.get("bun.lock")!.toString()),installFiles=nxToolingInstallFilesV1(manifest,capturedLock,files),dependencyFiles=nxDependencyInstallFilesV1(manifest,capturedLock);
  files.clear();for(const [path,bytes] of installFiles)files.set(path,bytes);
  const authorizer=await captureNxAuthorizerSourcesV1(root,RECIPE+"/🔏️authorizers/📦️runtime/🟦️.ts",signal);
  files.set(".semio-authorizer-source.json",Buffer.from(JSON.stringify(authorizer.manifest)+"\n"));
  const compiler=await captureNxAuthorizerSourcesV1(root,RECIPE+"/🔏️authorizers/📜️script.ts",signal);
  files.set(".semio-authorizer-compiler.json",Buffer.from(JSON.stringify({version:1,mode:"release",bun:manifest.packageManager,resolutions:compiler.manifest.resolutions,sources:compiler.manifest.sources})+"\n"));
  const report = process.platform === "linux" ? process.report?.getReport?.() as { header?: { glibcVersionRuntime?: string } } : undefined;
  const libc = process.platform === "linux" ? report?.header?.glibcVersionRuntime ? "gnu" : "musl" : "";
  const environment={platform:process.platform,architecture:process.arch,libc},sdkDigest=nxSdkSnapshotIdentityV1(environment,dependencyFiles);
  return{root,files,manifest,dependencyFiles,authorizer,compiler,environment,sdkDigest};
}

/** 🛠️ Acquires the frozen Nx toolchain independently of application dependency synchronization. */
async function provisionNxToolsOwnedV1(workspace: string, store:RepositoryToolingStoreV1,publisher:ManagedToolingPublisherV1,control:ResourceLeaseControl): Promise<NxTooling|ToolingHeadroomRefusalV1> {
  requireIssuedResourceControlRefinementV1(store.control,control);checkResourceLeaseControl(control);requireNativeStorageReservationV1(store.storage,store.control);await store.storage.recheck();checkResourceLeaseControl(control);if(store.root!==repositoryCachePartitionRootV1(workspace,"tools"))throw Error("Original declared tooling partition required");const signal=control.signal;signal.throwIfAborted();
  const{root,files,manifest,dependencyFiles,authorizer,compiler,environment,sdkDigest}=await captureNxToolingInputsV1(workspace,signal),bun=await prepareBun(root,manifest.packageManager.slice(4),store,control,publisher);if(isToolingHeadroomRefusalV1(bun))return bun;const sdkBase=requireNativeStoragePathV1(store.root,join(store.root,"sdk")),sdkDirectory=join(sdkBase,sdkDigest);
  const pristine=await ensureNxPristineSdkV1(root,store,publisher,dependencyFiles,environment,control,async directory=>{const downloadCache=requireNativeStoragePathV1(store.root,join(store.root,"downloads"));regularDirectory(downloadCache);await withResourceLeases({directory:join(root,".cache/b/l/m/data"),signal,control,resources:[{resource:"nx-tool-download-cache",mode:"exclusive"}]},()=>runNxSetupCommandV1(bun,["install","--frozen-lockfile","--ignore-scripts","--cache-dir",downloadCache,"--backend","copyfile"],directory,"nx:install",control,store.environment));});
  if(isToolingHeadroomRefusalV1(pristine))return pristine;
  return withNxSdkDerivativeProjectionV1(root,store,control,sdkDirectory,dependencyFiles,async projection=>{
  files.set(".semio-sdk-material.json",Buffer.from(JSON.stringify(projection.material)+"\n"));
  const capturedInputs=captureNxSdkPublicationInputsV1(files);
  const digest = nxToolingInstallIdentityV1(environment, capturedInputs), base = requireNativeStoragePathV1(store.root,join(root, STORE)), destination = join(base, digest);
  const installed = async(directory: string): Promise<NxTooling> => withNxSdkDerivativeReaderV1(root,store,directory,digest,control,async()=>{
    requireNativeStoragePathV1(store.root,directory);if(!lstatSync(directory).isDirectory())throw Error("Original existing tooling directory required");
    const marker = JSON.parse(readFileSync(join(directory, ".semio-nx-tooling.json"), "utf8"));
    if (marker.version !== 1 || marker.digest !== digest) throw new Error("Nx tooling ownership mismatch");
    if(marker.binaries?.version!==1||marker.binaries.entry!==".semio-nx-sdk-binaries.json"||Object.keys(marker.binaries).sort().join(",")!=="digest,entry,version")throw Error("Original SDK binary authority required");await verifyNxSdkBinaryLinksV1(directory,marker.binaries.digest,store,control);
    if(!nxAuthorizerReceiptHoldsV1(directory,marker.authorizers)||marker.authorizers.sourceSha256!==authorizer.digest)throw Error("Nx immutable authorizer receipt changed");
    const modulePath = join(directory, "node_modules"), require = createRequire(join(directory, "package.json"));
    for (const [name, version] of Object.entries(manifest.dependencies)) {
      const path = realpathSync(require.resolve(name + "/package.json")), local = relative(modulePath, path);
      if (isAbsolute(local) || local.startsWith("..") || JSON.parse(readFileSync(path, "utf8")).version !== version) throw new Error(`Incomplete Nx tooling: ${name}`);
    }
    for(const [name,path] of Object.entries(JSON.parse(Buffer.from(capturedInputs.get("package.json")!).toString()).patchedDependencies) as [string,string][]){const packageName=name.slice(0,name.lastIndexOf("@")),receipt=toolPatchReceipt(dirname(require.resolve(packageName+"/package.json")),Buffer.from(capturedInputs.get(path)!).toString());if(JSON.stringify(marker.patches?.[name])!==JSON.stringify(receipt))throw Error("Nx patch receipt changed");}
    await preflightNxAuthorizersV1(directory,control,store.environment);await store.recheck();
    return { cli: require.resolve("nx/bin/nx.js"), modulePath, bun,authorizer:join(directory,"semio-authorizers.mjs") };
  });
  let present = true;
  try { lstatSync(destination); }
  catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; present = false; }
  if (present) return withResourceLeases({directory:join(root,".cache/b/l/m/data"),resources:[{resource:"nx-tooling:"+digest,mode:"shared"}],control,signal},()=>installed(destination));
  const publish=async(quote:NxSdkClonePublicationV1):Promise<NxTooling|ToolingHeadroomRefusalV1>=>{
  return publisher(async()=>{
  regularDirectory(base);
  const staging = mkdtempSync(join(base, ".stage-"));let stagingFailed=false;
  try {
    await quote.writeInputs(staging,control);
    const started=performance.now(),result=await quote.clone(staging,control),binaryReceipt=Buffer.from(JSON.stringify(result.binaries)+"\n");requireNxSdkDerivativeReceiptImagesV1([binaryReceipt]);writeFileSync(join(staging,".semio-nx-sdk-binaries.json"),binaryReceipt,{flag:"wx",mode:0o444});console.log("[DEBUG] Shared pristine dependency projection published files="+result.files+" ownedBytes="+result.bytes+" elapsedMs="+Math.round(performance.now()-started));
    let patchBytes=0;const replacements=new Map<string,{bytes:Uint8Array;mode:number}>(),batches:ReadonlyMap<string,Uint8Array>[]=[],applied:{root:string;patch:string}[]=[];
    for(const[name,path]of Object.entries(JSON.parse(Buffer.from(capturedInputs.get("package.json")!).toString()).patchedDependencies)as[string,string][]){if(name.slice(0,name.lastIndexOf("@"))!=="nx")throw Error("Explicit owned Nx patch required");const require=createRequire(join(staging,"package.json")),packageRoot=dirname(require.resolve(name.slice(0,name.lastIndexOf("@"))+"/package.json")),patch=Buffer.from(capturedInputs.get(path)!).toString(),source=new Map<string,Uint8Array>();for(const relativePath of toolPatchAdditions(patch).keys()){const absolute=requireNativeStoragePathV1(store.root,join(packageRoot,relativePath)),status=lstatSync(absolute);if(!status.isFile()||status.isSymbolicLink()||status.nlink!==1||status.size>16777216)throw Error("Bounded independently writable SDK patch Source required");const bytes=replacements.get(absolute)?.bytes??readFileSync(absolute);if(bytes.byteLength>16777216||patchBytes+bytes.byteLength>16777216)throw Error("Bounded independently writable SDK patch Source required");patchBytes+=bytes.byteLength;source.set(relativePath,bytes);}const images=applyToolingPatchImagesV1(source,patch);batches.push(images);for(const[relativePath,bytes]of images){const absolute=requireNativeStoragePathV1(store.root,join(packageRoot,relativePath)),status=lstatSync(absolute);if(!status.isFile()||status.isSymbolicLink()||status.nlink!==1)throw Error("Original private SDK patch inode required");replacements.set(absolute,{bytes,mode:status.mode&0o777});}applied.push({root:packageRoot,patch});}
    const replacementBytes=requireNxSdkDerivativePatchImagesV1(batches);await store.recheck();for(const[absolute,row]of replacements){const status=lstatSync(requireNativeStoragePathV1(store.root,absolute));if(!status.isFile()||status.isSymbolicLink()||status.nlink!==1)throw Error("Original private SDK patch inode required");unlinkSync(absolute);writeFileSync(absolute,row.bytes,{flag:"wx",mode:row.mode});}for(const row of applied)toolPatchReceipt(row.root,row.patch);console.log("[DEBUG] SDK derivative bounded replacements "+JSON.stringify({files:replacements.size,bytes:replacementBytes}));await store.recheck();
    signal.throwIfAborted();
    const captured=Buffer.from(JSON.stringify(snapshotNxAuthorizerSourcesV1(authorizer))+"\n"),snapshot=join(staging,".semio-authorizer-captured.json"),program=join(staging,".semio-authorizer-compiler.mjs"),sealed=await compileNxAuthorizerSnapshotV1(root,snapshotNxAuthorizerSourcesV1(compiler),control,"bun"),snapshotDigest=createHash("sha256").update(captured).digest("hex"),programDigest=createHash("sha256").update(sealed.bytes).digest("hex");if(captured.byteLength>snapshotPolicy.maximumSnapshotBytes||sealed.bytes.byteLength>snapshotPolicy.maximumOutputBytes)throw Error("SDK derivative compiler publication byte ceiling exceeded");writeFileSync(snapshot,captured,{flag:"wx",mode:0o444});writeFileSync(program,sealed.bytes,{flag:"wx",mode:0o444});await store.recheck();
    try{await withResourceLeases({directory:join(root,".cache/b/l/m/data"),control,signal,resources:[{resource:"nx-authorizer-source:"+snapshot,mode:"shared"},{resource:"nx-authorizer-compiler:"+program,mode:"shared"}]},async()=>{await runNxSetupCommandV1(bun,[program,"compile",root,snapshot,snapshotDigest,staging],root,"nx:authorizers",control,nxSdkCompilerEnvironmentV1(store.environment));await store.recheck();if(createHash("sha256").update(readFileSync(program)).digest("hex")!==programDigest||createHash("sha256").update(readFileSync(snapshot)).digest("hex")!==snapshotDigest)throw Error("Original authorizer compiler/snapshot bytes changed");});}finally{rmSync(snapshot,{force:true});rmSync(program,{force:true});}
    const patches:Record<string,ReturnType<typeof toolPatchReceipt>>={},require=createRequire(join(staging,"package.json"));
    for(const [name,path] of Object.entries(JSON.parse(Buffer.from(capturedInputs.get("package.json")!).toString()).patchedDependencies) as [string,string][])patches[name]=toolPatchReceipt(dirname(require.resolve(name.slice(0,name.lastIndexOf("@"))+"/package.json")),Buffer.from(capturedInputs.get(path)!).toString());
    const authorizers={version:1,mode:"release",sourceSha256:authorizer.digest,entry:"semio-authorizers.mjs",outputs:[{path:"semio-authorizers.mjs",sha256:createHash("sha256").update(readFileSync(join(staging,"semio-authorizers.mjs"))).digest("hex")}]};
    const toolingReceipt=Buffer.from(JSON.stringify({version:1,digest,patches,authorizers,binaries:{version:1,entry:".semio-nx-sdk-binaries.json",digest:result.binaries.digest}}));requireNxSdkDerivativeReceiptImagesV1([binaryReceipt,toolingReceipt]);writeFileSync(join(staging,".semio-nx-tooling.json"),toolingReceipt,{flag:"wx"});
    await installed(staging);
    try { renameSync(staging, destination); }
    catch (error) { if (!["EEXIST", "ENOTEMPTY", "EPERM"].includes((error as NodeJS.ErrnoException).code ?? "")) throw error; await installed(destination); }
    return await installed(destination);
  } catch(error){stagingFailed=true;throw error;}
  finally {try{rmSync(staging,{recursive:true,force:true});}catch(error){if(!stagingFailed)throw error;try{console.error("[nx] Staging cleanup failed after publication refusal",error);}catch{}}}
  },quote);
  };

  return withNxSdkDerivativePublicationV1(root,store,projection,capturedInputs,control,quote=>publish(quote));
  });
}

/** 🎟️ Keeps the actual loader inside the same Original registry and generation custody used by retirement. */
export function activateNxTools(workspace:string,tooling:NxTooling,store:RepositoryToolingStoreV1,control:ResourceLeaseControl):Promise<void>{requireIssuedResourceControlRefinementV1(store.control,control);checkResourceLeaseControl(control);return withResourceLeases({directory:join(workspace,".cache/b/l/m/data"),resources:[{resource:"nx-tooling-provision",mode:"shared"},{resource:"nx-tooling:"+basename(dirname(tooling.modulePath)),mode:"shared"}],control,signal:control.signal},()=>withNxSdkDerivativeReaderV1(workspace,store,dirname(tooling.modulePath),basename(dirname(tooling.modulePath)),control,()=>activateNxToolsOwnedV1(workspace,tooling,store,control)));}
/** 🛠️ Registers every actual provision reader before immutable receipt or SDK loading. */
export function provisionNxTools(workspace:string,store:RepositoryToolingStoreV1,control:ResourceLeaseControl):Promise<NxTooling|ToolingHeadroomRefusalV1>{return withManagedToolingProvisionV1(workspace,store,control,{version:1,minimumAvailableBytes:retentionSchema.definitions.Protocol.const.minimumInstallerAvailableBytes},(publisher,control)=>provisionNxToolsOwnedV1(workspace,store,publisher,control));}

/** 🧭️ Plans optional headroom from sealed metadata and current captured images; the later issued quote remains authoritative. */
export async function planNxToolingStorageV1(workspace:string,invocation:import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<Readonly<{version:1;minimumAvailableBytes:number;publicationRequired:boolean;material:Readonly<{version:1;entries:number;copyAllocatedBytes:number;inputBytes:number}>|null}>>{requireIssuedResourceControlInvocationV1(phaseControl,invocation);return withRepositoryToolingStoreV1(workspace,invocation,phaseControl,async(store,control)=>{const captured=await captureNxToolingInputsV1(workspace,control.signal);checkResourceLeaseControl(control);const directory=requireNativeStoragePathV1(store.root,join(store.root,"sdk",captured.sdkDigest));if(!existsSync(join(directory,nxSdkSourceIndexPolicyV1.index))){await store.recheck();checkResourceLeaseControl(control);return{version:1 as const,minimumAvailableBytes:retentionSchema.definitions.Protocol.const.minimumInstallerAvailableBytes,publicationRequired:true,material:null};}return withResourceLeases({directory:join(workspace,".cache/b/l/m/data"),resources:[{resource:"nx-sdk-snapshot:"+captured.sdkDigest,mode:"shared"}],control:control,signal:control.signal},async()=>{for(const[path,bytes]of captured.dependencyFiles)if(!(await readNxSdkSourceMetadataV1(join(directory,path),store,undefined,control)).equals(bytes))throw Error("Exact planned SDK dependency images required");const markerBytes=await readNxSdkSourceMetadataV1(join(directory,".semio-nx-sdk.json"),store,undefined,control),marker=JSON.parse(markerBytes.toString());if(!nxSdkSnapshotReceiptHoldsV1(marker,captured.sdkDigest)||Object.keys(marker.patches).length)throw Error("Authenticated planned pristine snapshot required");const bytes=await readNxSdkSourceMetadataV1(join(directory,nxSdkSourceIndexPolicyV1.index),store,undefined,control),index=readNxSdkSourceIndexV1(JSON.parse(bytes.toString()));if(index.snapshot!==captured.sdkDigest||JSON.stringify(index)+"\n"!==bytes.toString())throw Error("Exact planned sealed SDK index required");const facts=await statfs(store.root,{bigint:true}),unit=Number(facts.bsize);checkResourceLeaseControl(control);if(!Number.isSafeInteger(unit)||unit<=0||unit>65536)throw Error("Planned derivative allocation unit refused");let copyAllocatedBytes=0,ownedBytes=0,count=0;for(const row of index.entries){checkResourceLeaseControl(control);if(row.kind==="file"&&nxSdkDerivativeDispositionV1({path:row.path,kind:row.kind})==="copy"){ownedBytes+=row.bytes!;copyAllocatedBytes+=Math.ceil(row.bytes!/unit)*unit;}if(++count%128===0){await control.publish({resource:"nx:storage-planning",mode:"shared",elapsedMs:0});await control.yieldContinuation();checkResourceLeaseControl(control);}}if(ownedBytes>nxSdkSourceIndexPolicyV1.maximumOwnedBytes)throw Error("Planned owned Nx bytes exceed policy");captured.files.set(".semio-sdk-material.json",Buffer.from(JSON.stringify({version:1,kind:"shared-pristine",snapshot:captured.sdkDigest,sourceIndex:nxSdkSourceHashV1(bytes),ownedPackages:["nx"]})+"\n"));const images=captureNxSdkPublicationInputsV1(captured.files),material={version:1 as const,entries:index.entries.length,copyAllocatedBytes,inputBytes:[...images.values()].reduce((total,row)=>total+row.byteLength,0)},minimumAvailableBytes=nxSdkDerivativeHeadroomBytesV1(material);const identity=nxToolingInstallIdentityV1(captured.environment,images),generation=requireNativeStoragePathV1(store.root,join(store.root,"nx",identity));let publicationRequired=true;try{const marker=JSON.parse((await readNxSdkSourceMetadataV1(join(generation,".semio-nx-tooling.json"),store,undefined,control)).toString());checkResourceLeaseControl(control);publicationRequired=!(marker.version===1&&marker.digest===identity&&marker.authorizers?.sourceSha256===captured.authorizer.digest&&nxAuthorizerReceiptHoldsV1(generation,marker.authorizers));}catch(error){checkResourceLeaseControl(control);if((error as NodeJS.ErrnoException).code!=="ENOENT"&&(error as Error).message!=="Original SDK source metadata required")throw error;}if(!(await readNxSdkSourceMetadataV1(join(directory,nxSdkSourceIndexPolicyV1.index),store,undefined,control)).equals(bytes)||!(await readNxSdkSourceMetadataV1(join(directory,".semio-nx-sdk.json"),store,undefined,control)).equals(markerBytes))throw Error("Planned sealed SDK metadata changed");for(const[path,image]of captured.dependencyFiles)if(!(await readNxSdkSourceMetadataV1(join(directory,path),store,undefined,control)).equals(image))throw Error("Planned SDK dependency images changed");await store.recheck();checkResourceLeaseControl(control);console.log("[DEBUG] Planned derivative storage from current captured images "+JSON.stringify({...material,minimumAvailableBytes,publicationRequired,scope:"planning hint only; actual installed validation or issued quote remains authoritative"}));return{version:1 as const,minimumAvailableBytes,publicationRequired,material};});});}
/** 🚦️ Completes planning under SH, closes it, then requests optional whole-image recovery before the actual tooling reader. */
export async function prepareNxToolingStorageV1(workspace:string,invocation:import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<void>{requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);const request=await planNxToolingStorageV1(workspace,invocation,phaseControl);checkResourceLeaseControl(phaseControl);if(!request.publicationRequired)return;await prepareManagedToolingStorageV1(workspace,invocation,phaseControl,{version:1,minimumAvailableBytes:request.minimumAvailableBytes});checkResourceLeaseControl(phaseControl);}

/** ♻️ Retries one exact pre-write refusal only after all tooling readers and quote scopes have settled. */
export function withNxToolingHeadroomRecoveryV1<T>(workspace:string,invocation:import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation,phaseControl:ResourceLeaseControl,attempt:(actualPhase:ResourceLeaseControl)=>Promise<T|ToolingHeadroomRefusalV1>):Promise<T>{return withToolingHeadroomRecoveryV1(repositoryCachePartitionRootV1(workspace,"tools"),invocation,phaseControl,attempt,async(minimumAvailableBytes,actualPhase)=>{checkResourceLeaseControl(actualPhase);const{withRepositoryCacheStoreQueuedMaintenanceV1}=await import("../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts"),{retireQuiescentManagedToolingGenerationsV1}=await import("./🧹️retention/📥️physical/🟦️.ts");checkResourceLeaseControl(actualPhase);await withRepositoryCacheStoreQueuedMaintenanceV1(workspace,"tools",invocation,actualPhase,{version:1,mode:"queued",maximumWaitMilliseconds:0},(store,maintenancePhase)=>retireQuiescentManagedToolingGenerationsV1(workspace,store,maintenancePhase,{version:1,mode:"quiescent-generations",minimumAvailableBytes}));});}
