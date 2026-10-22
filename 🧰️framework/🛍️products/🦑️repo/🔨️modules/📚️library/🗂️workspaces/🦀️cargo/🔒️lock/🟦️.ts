import{existsSync,mkdirSync}from"node:fs";
import{join}from"node:path";
import{ensureCargoLockV1,registerCargoLockEnsurerV1,type CargoLockEnsureRequestV1,type CargoLockOutcomeV1,type CargoLockPortV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import{acquireResourceLease,checkResourceLeaseControl}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{captureOwnedCommandResultV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import{resolveLanguageV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🟦️.ts";
import{CATALOG_LANGUAGES_V1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import{repositoryCachePartitionRootV1,repositoryLeaseDirectoryV1}from"../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import{cargoInvocationManifestV1}from"../🟦️.ts";

const limits=Object.freeze({maximumBytes:16777216,maximumLines:65536}),poll=250;

/** 🗃️ Locates the machine-local lock receipts beside the Cargo preparation receipts, inside the Rust partition once it is mounted. */
export function repositoryCargoLockReceiptDirectoryV1(workspace:string):string|null{const root=repositoryCachePartitionRootV1(workspace,"rust");if(!existsSync(root))return null;const directory=join(root,"cargo/lock");mkdirSync(directory,{recursive:true});return directory;}

/** 🔌️ Runs the lock refresh with the caller's own Cargo program, environment, cancellation and a per-workspace try-lease of the repository lease directory. */
function repositoryCargoLockPortV1(request:CargoLockEnsureRequestV1,leaseDirectory:string):CargoLockPortV1{
 const control=request.control;
 return{
  async run(args){checkResourceLeaseControl(control);const result=await captureOwnedCommandResultV1(request.program,[...request.prefix,...args],request.cwd,{control,env:request.environment,output:limits});return{status:result.signal===null?result.status:null,stdout:result.stdout,stderr:result.stderr};},
  async tryHold(resource){checkResourceLeaseControl(control);try{const lease=await acquireResourceLease({directory:leaseDirectory,resource,mode:"exclusive",signal:control.signal,control,timeoutMs:0});return{release:()=>{lease.release();}};}catch(error){if(error instanceof Error&&error.message==="Resource lease timed out: "+resource)return null;throw error;}},
  async pause(milliseconds){checkResourceLeaseControl(control);await control.publish({resource:"cargo-lock:wait",mode:"exclusive",elapsedMs:0});await new Promise<void>((done,fail)=>{const timer=setTimeout(()=>{control.signal.removeEventListener("abort",abort);done();},milliseconds),abort=()=>{clearTimeout(timer);fail(control.signal.reason);};control.signal.addEventListener("abort",abort,{once:true});});await control.yieldContinuation();},
  report:line=>{process.stderr.write(line+"\n");},
 };
}

/** 🔐️ Verifies the lockfile of one Cargo invocation against its manifest set through the repository's receipt and refreshes it from the shared registry when it lags. */
export function ensureRepositoryCargoLockV1(request:CargoLockEnsureRequestV1):Promise<CargoLockOutcomeV1>{
 const manifest=cargoInvocationManifestV1(request.args,request.cwd);
 return ensureCargoLockV1({workspaceRoot:request.workspaceRoot,manifest,receiptDirectory:repositoryCargoLockReceiptDirectoryV1(request.workspaceRoot),language:resolveLanguageV1(CATALOG_LANGUAGES_V1,[],request.environment).language,offline:request.environment.CARGO_NET_OFFLINE==="true"||request.args.includes("--offline")||request.args.includes("--frozen"),force:request.force},repositoryCargoLockPortV1(request,repositoryLeaseDirectoryV1(request.workspaceRoot)));
}
registerCargoLockEnsurerV1(ensureRepositoryCargoLockV1);
