import {nxToolingInstallIdentityV1} from "./📦️sdk/🪪️identity/🟦️.ts";
export {nxToolingInstallIdentityV1} from "./📦️sdk/🪪️identity/🟦️.ts";
import {withManagedToolingProvisionV1} from "./🧹️retention/📥️physical/🟦️.ts";
import retentionSchema from "./🧹️retention/📥️physical/🧬️schema/🔣️.json";
import {preflightNxAuthorizersV1} from "./🔏️authorizers/📥️loading/🟦️.ts";
import {repositoryCachePartitionRootV1} from "../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import type {RepositoryToolingStoreV1} from "../../💾️storage/🔒️enforcement/🛠️tools/🟦️.ts";
import {requireNativeStorageReservationV1} from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import { createHash, randomUUID } from "node:crypto";
import { lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, symlinkSync, unlinkSync, writeFileSync } from "node:fs";
import {toolPatchReceipt,assertToolingPatchV1} from "./🩹️patch/🟦️.ts";
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
async function activateNxToolsOwnedV1(workspace: string, tooling: NxTooling, store:RepositoryToolingStoreV1): Promise<void> {
  requireNativeStorageReservationV1(store.storage,store.control);await store.storage.recheck();const signal=store.control.signal;signal.throwIfAborted();
  const root = realpathSync(workspace), nx = join(root, ".nx"), pointer = join(nx, "installation"), selected = realpathSync(dirname(tooling.modulePath));
  if (dirname(selected) !== join(root, STORE) || !/^[a-f0-9]{64}$/.test(basename(selected))) throw new Error("Nx tooling must be an owned immutable installation");
  const selectedMarker=JSON.parse(readFileSync(join(selected,".semio-nx-tooling.json"),"utf8"));if(selectedMarker.digest!==basename(selected)||!nxAuthorizerReceiptHoldsV1(selected,selectedMarker.authorizers))throw Error("Original immutable activation receipt required");await preflightNxAuthorizersV1(selected,store.control,store.environment);await store.recheck();
  regularDirectory(nx);
  await withResourceLeases({ directory: join(root, ".cache/b/l/m/data"), signal,control:store.control, resources: [{ resource: "nx-tooling-activation", mode: "exclusive" }] }, async () => {
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

/** 🛠️ Acquires the frozen Nx toolchain independently of application dependency synchronization. */
async function provisionNxToolsOwnedV1(workspace: string, store:RepositoryToolingStoreV1): Promise<NxTooling> {
  requireNativeStorageReservationV1(store.storage,store.control);await store.storage.recheck();if(store.root!==repositoryCachePartitionRootV1(workspace,"tools"))throw Error("Original declared tooling partition required");const signal=store.control.signal;signal.throwIfAborted();
  const root = realpathSync(workspace), recipe = join(root, RECIPE), files = new Map<string, Buffer>();
  for (const name of ["package.json", "bun.lock"]) files.set(name, readFileSync(join(recipe, name)));
  const manifest = JSON.parse(files.get("package.json")!.toString());
  if (!/^bun@\d+\.\d+\.\d+$/.test(manifest.packageManager)) throw new Error("Invalid Nx tooling Bun pin");
  const bun = await prepareBun(root,manifest.packageManager.slice(4),store);
  const patches = manifest.semio?.toolPatches;
  if (!patches || typeof patches!=="object" || Array.isArray(patches) || Object.entries(patches).some(([name,path])=>!name || typeof path!=="string")) throw new Error("Invalid authored tooling patch contribution");
  for (const path of Object.values(patches) as string[]) {
    const canonical = realpathSync(join(root, path)), local = relative(root, canonical);
    if (isAbsolute(path) || path.includes("\\") || path.split("/").some(part => !part || part === "." || part === "..") || isAbsolute(local) || local.startsWith("..")) throw new Error(`Invalid tooling patch: ${path}`);
    files.set(path, readFileSync(canonical));
  }
  const installFiles=nxToolingInstallFilesV1(manifest,Bun.JSONC.parse(files.get("bun.lock")!.toString()),files);
  files.clear();for(const [path,bytes] of installFiles)files.set(path,bytes);
  const authorizer=await captureNxAuthorizerSourcesV1(root,RECIPE+"/🔏️authorizers/📦️runtime/🟦️.ts",signal);
  files.set(".semio-authorizer-source.json",Buffer.from(JSON.stringify(authorizer.manifest)+"\n"));
  const compiler=await captureNxAuthorizerSourcesV1(root,RECIPE+"/🔏️authorizers/📜️script.ts",signal);
  files.set(".semio-authorizer-compiler.json",Buffer.from(JSON.stringify({version:1,mode:"release",bun:manifest.packageManager,sources:compiler.manifest.sources})+"\n"));
  const report = process.platform === "linux" ? process.report?.getReport?.() as { header?: { glibcVersionRuntime?: string } } : undefined;
  const libc = process.platform === "linux" ? report?.header?.glibcVersionRuntime ? "gnu" : "musl" : "";
  const environment={platform:process.platform,architecture:process.arch,libc},sdkDigest=nxSdkSnapshotIdentityV1(environment,installFiles),sdkBase=requireNativeStoragePathV1(store.root,join(store.root,"sdk")),sdkDirectory=join(sdkBase,sdkDigest);
  const digest = nxToolingInstallIdentityV1(environment, files), base = requireNativeStoragePathV1(store.root,join(root, STORE)), destination = join(base, digest);
  regularDirectory(base);
  const sdkReceipt=(directory:string):Record<string,ReturnType<typeof toolPatchReceipt>>=>{
    regularDirectory(directory);for(const[path,bytes]of installFiles)if(!readFileSync(join(directory,path)).equals(bytes))throw Error("Pinned SDK installation input changed");
    const modulePath=join(directory,"node_modules"),require=createRequire(join(directory,"package.json")),receipts:Record<string,ReturnType<typeof toolPatchReceipt>>={};
    for(const[name,version]of Object.entries(manifest.dependencies)){const path=realpathSync(require.resolve(name+"/package.json")),local=relative(modulePath,path);if(isAbsolute(local)||local.startsWith("..")||JSON.parse(readFileSync(path,"utf8")).version!==version)throw Error("Pinned SDK dependency changed: "+name);}
    for(const[name,path]of Object.entries(JSON.parse(installFiles.get("package.json")!.toString()).patchedDependencies)as[string,string][])receipts[name]=toolPatchReceipt(dirname(require.resolve(name.slice(0,name.lastIndexOf("@"))+"/package.json")),installFiles.get(path)!.toString());
    return receipts;
  };
  const sdkInstalled=(directory:string):void=>{const marker=JSON.parse(readFileSync(join(directory,".semio-nx-sdk.json"),"utf8"));if(!nxSdkSnapshotReceiptHoldsV1(marker,sdkDigest)||JSON.stringify(marker.patches)!==JSON.stringify(sdkReceipt(directory)))throw Error("Pinned SDK snapshot receipt changed");};
  const installed = async(directory: string): Promise<NxTooling> => {
    regularDirectory(directory);
    const marker = JSON.parse(readFileSync(join(directory, ".semio-nx-tooling.json"), "utf8"));
    if (marker.version !== 1 || marker.digest !== digest) throw new Error("Nx tooling ownership mismatch");
    if(!nxAuthorizerReceiptHoldsV1(directory,marker.authorizers)||marker.authorizers.sourceSha256!==authorizer.digest)throw Error("Nx immutable authorizer receipt changed");
    const modulePath = join(directory, "node_modules"), require = createRequire(join(directory, "package.json"));
    for (const [name, version] of Object.entries(manifest.dependencies)) {
      const path = realpathSync(require.resolve(name + "/package.json")), local = relative(modulePath, path);
      if (isAbsolute(local) || local.startsWith("..") || JSON.parse(readFileSync(path, "utf8")).version !== version) throw new Error(`Incomplete Nx tooling: ${name}`);
    }
    for(const [name,path] of Object.entries(JSON.parse(files.get("package.json")!.toString()).patchedDependencies) as [string,string][]){const packageName=name.slice(0,name.lastIndexOf("@")),receipt=toolPatchReceipt(dirname(require.resolve(packageName+"/package.json")),files.get(path)!.toString());if(JSON.stringify(marker.patches?.[name])!==JSON.stringify(receipt))throw Error("Nx patch receipt changed");}
    await preflightNxAuthorizersV1(directory,store.control,store.environment);await store.recheck();
    return { cli: require.resolve("nx/bin/nx.js"), modulePath, bun,authorizer:join(directory,"semio-authorizers.mjs") };
  };
  let present = true;
  try { lstatSync(destination); }
  catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; present = false; }
  if (present) return installed(destination);
  const staging = mkdtempSync(join(base, ".stage-"));
  try {
    for (const [name, bytes] of files) { mkdirSync(dirname(join(staging, name)), { recursive: true }); writeFileSync(join(staging, name), bytes); }
    const downloadCache=requireNativeStoragePathV1(store.root,join(store.root,"downloads"));regularDirectory(downloadCache);
    regularDirectory(sdkBase);
    await withResourceLeases({directory:join(root,".cache/b/l/m/data"),signal,control:store.control,resources:[{resource:"nx-tool-download-cache",mode:"exclusive"}]},async()=>{
      if(existsOwnedDirectory(sdkDirectory)){sdkInstalled(sdkDirectory);return;}
      const sdkStaging=mkdtempSync(join(sdkBase,".stage-"));
      try{
        for(const[path,bytes]of installFiles){mkdirSync(dirname(join(sdkStaging,path)),{recursive:true});writeFileSync(join(sdkStaging,path),bytes);}
        console.log(`Acquiring pinned Nx ${manifest.dependencies.nx} SDK snapshot…`);
        for(const entry of readdirSync(downloadCache))if(entry.includes("_patch_hash="))rmSync(join(downloadCache,entry),{recursive:true,force:true});
        await runNxSetupCommandV1(bun,["install","--frozen-lockfile","--ignore-scripts","--cache-dir",downloadCache,"--backend","copyfile"],sdkStaging,"nx:install",store.control,store.environment);
        signal.throwIfAborted();writeFileSync(join(sdkStaging,".semio-nx-sdk.json"),JSON.stringify({version:1,digest:sdkDigest,patches:sdkReceipt(sdkStaging)}));sdkInstalled(sdkStaging);renameSync(sdkStaging,sdkDirectory);
      }finally{rmSync(sdkStaging,{recursive:true,force:true});}
    });
    await receiveScriptProcessInvocation(process.env,async original=>{
      const control=scriptCompilerAdmissionControlV1(original,"nx:sdk-snapshot:clone",120000,signal),started=performance.now();
      await withResourceLeases({directory:join(root,".cache/b/l/m/data"),signal:control.signal,control,resources:[{resource:"nx-sdk-snapshot:"+sdkDigest,mode:"shared"}]},async()=>{const result=await cloneNxSdkSnapshotV1(sdkDirectory,staging,control);console.log("[DEBUG] pinned SDK snapshot reused files="+result.files+" bytes="+result.bytes+" elapsedMs="+Math.round(performance.now()-started));});
    });
    signal.throwIfAborted();
    const captured=Buffer.from(JSON.stringify(snapshotNxAuthorizerSourcesV1(authorizer))+"\n"),snapshot=join(staging,".semio-authorizer-captured.json"),program=join(staging,".semio-authorizer-compiler.mjs"),sealed=await compileNxAuthorizerSnapshotV1(root,snapshotNxAuthorizerSourcesV1(compiler),store.control,"bun"),snapshotDigest=createHash("sha256").update(captured).digest("hex"),programDigest=createHash("sha256").update(sealed.bytes).digest("hex");writeFileSync(snapshot,captured,{flag:"wx",mode:0o444});writeFileSync(program,sealed.bytes,{flag:"wx",mode:0o444});await store.recheck();
    try{await withResourceLeases({directory:join(root,".cache/b/l/m/data"),control:store.control,signal,resources:[{resource:"nx-authorizer-source:"+snapshot,mode:"shared"},{resource:"nx-authorizer-compiler:"+program,mode:"shared"}]},async()=>{await runNxSetupCommandV1(bun,[program,"compile",root,snapshot,snapshotDigest,staging],root,"nx:authorizers",store.control,store.environment);await store.recheck();if(createHash("sha256").update(readFileSync(program)).digest("hex")!==programDigest||createHash("sha256").update(readFileSync(snapshot)).digest("hex")!==snapshotDigest)throw Error("Original authorizer compiler/snapshot bytes changed");});}finally{rmSync(snapshot,{force:true});rmSync(program,{force:true});}
    const patches:Record<string,ReturnType<typeof toolPatchReceipt>>={},require=createRequire(join(staging,"package.json"));
    for(const [name,path] of Object.entries(JSON.parse(files.get("package.json")!.toString()).patchedDependencies) as [string,string][])patches[name]=toolPatchReceipt(dirname(require.resolve(name.slice(0,name.lastIndexOf("@"))+"/package.json")),files.get(path)!.toString());
    const authorizers={version:1,mode:"release",sourceSha256:authorizer.digest,entry:"semio-authorizers.mjs",outputs:[{path:"semio-authorizers.mjs",sha256:createHash("sha256").update(readFileSync(join(staging,"semio-authorizers.mjs"))).digest("hex")}]};
    writeFileSync(join(staging, ".semio-nx-tooling.json"), JSON.stringify({ version: 1, digest,patches,authorizers }));
    await installed(staging);
    try { renameSync(staging, destination); }
    catch (error) { if (!["EEXIST", "ENOTEMPTY", "EPERM"].includes((error as NodeJS.ErrnoException).code ?? "")) throw error; await installed(destination); }
    return installed(destination);
  } finally { rmSync(staging, { recursive: true, force: true }); }
}

/** 🎟️ Keeps the actual loader inside the same Original registry and generation custody used by retirement. */
export function activateNxTools(workspace:string,tooling:NxTooling,store:RepositoryToolingStoreV1):Promise<void>{return withResourceLeases({directory:join(workspace,".cache/b/l/m/data"),resources:[{resource:"nx-tooling-provision",mode:"shared"},{resource:"nx-tooling:"+basename(dirname(tooling.modulePath)),mode:"shared"}],control:store.control,signal:store.control.signal},()=>activateNxToolsOwnedV1(workspace,tooling,store));}
/** 🛠️ Registers every actual provision reader before immutable receipt or SDK loading. */
export function provisionNxTools(workspace:string,store:RepositoryToolingStoreV1):Promise<NxTooling>{return withManagedToolingProvisionV1(workspace,store,{version:1,minimumAvailableBytes:retentionSchema.definitions.Protocol.const.minimumInstallerAvailableBytes},()=>provisionNxToolsOwnedV1(workspace,store));}
