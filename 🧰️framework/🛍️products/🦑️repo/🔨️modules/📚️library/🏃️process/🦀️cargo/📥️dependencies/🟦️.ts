import {AsyncLocalStorage} from "node:async_hooks";
import{profilePhaseV1}from"../../../🏗️build/⏱️profile/🟨️.mjs";
import{machinePathV1}from"../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import continuationSchema from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📥️inputs/🤲️continuations/🧬️schema/🔣️json/🟦️.ts";
import{fleetFetchContextV1}from"../../../⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🟦️.ts";
import{fleetCargoRegistryAroundV1}from"../../../⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🦀️crates/🟦️.ts";
import type {CargoCompilerInputsRequestV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📥️inputs/🟦️.ts";
import{requireIssuedResourceControlRefinementV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{cargoConfigurationArgumentsV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import type{CargoCompilerInputsPortV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📥️inputs/🟦️.ts";
import{repositoryCargoCommandV1}from"../../../⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/📥️arguments/📤️forwarding/🟦️.ts";
import cargoLaunchSchema from"../🚪️launcher/🧬️schema/🔣️.json" with { type: "json" };
const cargoLaunchPolicy=cargoLaunchSchema.definitions.Configuration.const;
import{readCargoProfileConsumptionV1,type CargoProfileConsumptionV1}from"../../../🏗️build/🎚️mode/🦀️profile/🟦️.ts";
import{requireIssuedResourceControlInvocationV1,withIssuedResourceOperationV1,withRequestedIssuedResourceControlV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{withCompilerWorkspaceSharedWriteV1}from"../../../⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts";
import {withRepositoryCargoConfigurationV1,repositoryCargoConfigurationArgumentsV1,repositoryCargoConfigurationStepArgumentsV1,type RepositoryCargoConfigurationViewV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🟦️.ts";
import {withRepositoryCargoRegistryReadyV1} from "./🟢️ready/💾️storage/🟦️.ts";
import type {CargoDependencyNetworkV1,RepositoryCargoInputsV1} from "./📋️contract/🟦️.ts";
import {admitRepositoryCompilerEnvironmentV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/🌍️environment/🟦️.ts";
import {admitRepositoryCargoStorageArgumentsV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/📥️arguments/🟦️.ts";
import {randomBytes} from "node:crypto";
import {isAbsolute,relative,resolve,join,sep} from "node:path";
import {acquireQueuedResourceLease,checkResourceLeaseControl,requireMaterializedPathV1,type ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {requireNativeStorageReservationV1,type NativeStorageReservationV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {captureOwnedCommandV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {validateJsonSchemaSubset} from "./../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {readNativeCallerAuthorityV1} from "./../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import {ensureRepositoryCargoLockV1} from "../../../🗂️workspaces/🦀️cargo/🔒️lock/🟦️.ts";
import {checkoutSnapshotOriginPathV1} from "../../../../../../../🔨️modules/🏃️process/🏗️build/📸️checkout-snapshot/🟦️.ts";
import {concurrentEditV1} from "../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import {cargoLockStaleFailureV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";


/** 📸️ Whether the manifest lies in an immutable checkout snapshot instead of the live workspace: its lockfile is read once as captured and is never refreshed in place. */
export function snapshotSourceV1(options:Pick<RepositoryCargoInputsV1,"workspace"|"sourceRoot">,manifest:string):boolean{
 if(options.sourceRoot===undefined||resolve(options.sourceRoot)===resolve(options.workspace))return false;
 const distance=relative(resolve(options.sourceRoot),resolve(manifest));
 return distance!==".."&&!distance.startsWith(".."+sep)&&!isAbsolute(distance);
}

/** ⚙️ The `.cargo/config.toml` a request reads and names in its `--config` argument: the snapshot's for a manifest inside the snapshot, the live workspace's for every other manifest. The argument builders and the storage admission of the configuration view must agree on it, so both call this. */
export function repositoryCargoConfigurationPathV1(options:Pick<RepositoryCargoInputsV1,"workspace"|"sourceRoot">,manifest:string):string{return join(snapshotSourceV1(options,manifest)?options.sourceRoot!:options.workspace,".cargo/config.toml");}

/** ⚙️ Gives `args` the `--config` selectors of the manifest's own configuration and writer: the only way a pass input, a compiler input or a further step of either may spell them, so none can name the live `.cargo/config.toml` while the context is a snapshot. */
export function repositoryCargoManifestArgumentsV1(options:Pick<RepositoryCargoInputsV1,"workspace"|"sourceRoot">,manifest:string,args:readonly string[],buildDirectory:string):string[]{return cargoConfigurationArgumentsV1(args,repositoryCargoConfigurationPathV1(options,manifest),buildDirectory);}

/** 🧊️ Retains exact per-selection issued arguments and actual phase across Cargo planning and execution. */
export interface RepositoryCargoInputSelectionV1{readonly manifest:string;readonly cwd:string;readonly arguments:readonly string[];readonly configuration:RepositoryCargoConfigurationViewV1}
export interface RepositoryCargoInputsViewV1{readonly environment:Readonly<Record<string,string|undefined>>;readonly control:ResourceLeaseControl;readonly selections:readonly RepositoryCargoInputSelectionV1[];recheck():Promise<void>}

/** 🪢️ A configuration view adopted up front for one declared pass input, with the environment and issued arguments its consumers receive. */
type AdoptedCargoSelectionV1=Readonly<{selection:RepositoryCargoInputSelectionV1;environment:Readonly<Record<string,string|undefined>>}>;
const adoptedCargoSelections=new AsyncLocalStorage<ReadonlyMap<NativeStorageReservationV1,ReadonlyMap<string,AdoptedCargoSelectionV1>>>();
/** 🔑️ Names one declared input by its exact manifest, directory, arguments, consumption and environment, so only the identical request adopts the view held for it. */
export function adoptedCargoSelectionKeyV1(manifest:string,cwd:string,args:readonly string[],consumption:CargoProfileConsumptionV1,environment:Readonly<Record<string,string|undefined>>):string{return JSON.stringify([resolve(manifest),resolve(cwd),args,consumption,Object.entries(environment).filter(([,value])=>value!==undefined).sort(([left],[right])=>left<right?-1:left>right?1:0)]);}

/** 🕸️ Projects only an actual native caller's authored network grant. */
export function repositoryCargoNetworkV1(invocation:ScriptInvocation,workspace:string):CargoDependencyNetworkV1{const authority=readNativeCallerAuthorityV1(invocation.capabilities);if(resolve(authority.repositoryRoot)!==resolve(workspace))throw Error("Original Cargo network grant belongs to another workspace");return authority.network;}


/** 🎬️ Closes actual Cargo argv to an explicit compiler or noncompiling resolution intent. */
export function repositoryCargoConsumptionV1(args:readonly string[],producer:Extract<CargoProfileConsumptionV1,{kind:"compiler"}>["producer"]):CargoProfileConsumptionV1{if(args.length>cargoLaunchPolicy.maximumArguments||args.some(arg=>typeof arg!=="string"||arg.length>cargoLaunchPolicy.maximumArgumentLength||arg.includes("\0")))throw Error("Bounded Cargo consumption arguments required");const command=repositoryCargoCommandV1(args);if(cargoLaunchPolicy.subcommands.compile.includes(command??""))return readCargoProfileConsumptionV1({kind:"compiler",producer,operation:command});if(cargoLaunchPolicy.subcommands.resolve.includes(command??"")||cargoLaunchPolicy.subcommands.lock.includes(command??""))return Object.freeze({kind:"resolution"});throw Error("Declared Cargo consumption command required");}

/** 🆓️ Names the Cargo commands that read only the workspace manifests (`metadata --no-deps`): they never resolve or open a dependency, so neither the lock check nor the registry fetch and its custody stand before them. */
export function repositoryCargoDependencyFreeV1(args:readonly string[]):boolean{const command=repositoryCargoCommandV1(args);return cargoLaunchPolicy.dependencyFree.some(row=>row.command===command&&row.flags.every(flag=>args.includes(flag)));}

/** 📋️ Fetches the locked original manifest for every platform its lockfile resolves, respecting its unchanged offline grant: compiler planning reads
 * `cargo metadata` over the complete resolve (host build scripts and proc-macros included), so a target-filtered fetch could never be planned offline.
 * @see ../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts */
export function cargoDependencyFetchArgumentsV1(manifest:string,args:readonly string[],network:CargoDependencyNetworkV1):string[]{
 if(validateJsonSchemaSubset(schema,network).length)throw Error("Original Cargo dependency network grant required");manifest=requireMaterializedPathV1(manifest);const result=["fetch","--locked","--manifest-path",manifest];if(network.offline||args.includes("--offline")||args.includes("--frozen"))result.push("--offline");return result;
}

/** 🧾️ Admits the bounded registry home, temporary and writer namespaces before any registry child observes them. */
async function admitRepositoryCargoInputsV1(options:RepositoryCargoInputsV1,configurations:readonly RepositoryCargoInputSelectionV1[]|null){
 const {control,storage}=options;requireIssuedResourceControlInvocationV1(control,options.invocation);readCargoProfileConsumptionV1(options.consumption);checkResourceLeaseControl(control);const selections=[{manifest:options.manifest,cwd:options.cwd,args:options.args},...(options.additionalManifests??[])];if(selections.length>64)throw Error("Original Cargo dependency selection limit exceeded");const requests=selections.map(row=>({manifest:row.manifest,cwd:row.cwd,args:cargoDependencyFetchArgumentsV1(row.manifest,row.args,options.network)})),args=requests[0]!.args;requireNativeStorageReservationV1(storage,control);if(storage.allocation.kind!=="compiler-workspace")throw Error("Original dependency compiler storage required");await storage.recheck();admitRepositoryCompilerEnvironmentV1(options.environment);const home=options.environment.CARGO_HOME;if(!home)throw Error("Bounded Cargo dependency home required");requireNativeStoragePathV1(storage.backing.root,home);for(const name of ["TMPDIR","TMP","TEMP"]){const path=options.environment[name];if(!path)throw Error("Bounded dependency temporary storage required");requireNativeStoragePathV1(storage.backing.root,path);}
 const buildDirectory=options.environment.CARGO_BUILD_BUILD_DIR,targetDirectory=options.environment.CARGO_TARGET_DIR;if(!buildDirectory||!targetDirectory)throw Error("Bounded Cargo compiler writer environment required");for(const[index,row]of selections.entries())await admitRepositoryCargoStorageArgumentsV1(row.args,{version:1,compilerConfiguration:repositoryCargoConfigurationPathV1(options,row.manifest),buildDirectory,targetDirectory},storage,control,configurations===null?null:configurations[index]!.configuration);
 return{requests,home,environment:Object.freeze({...options.environment,CARGO_NET_OFFLINE:String(args.includes("--offline"))}),leaseOptions:{directory:machinePathV1(options.workspace,".cache/b/l/m/data"),resource:"cargo:registry:"+home,signal:control.signal,control}};
}

/** 🔐️ Verifies the lockfile of every distinct selected manifest against its manifest set before the locked fetch, refreshing it from the shared registry when it lags. */
async function ensureRepositoryCargoLocksV1(options:RepositoryCargoInputsV1,admitted:Readonly<{requests:readonly Readonly<{manifest:string;cwd:string;args:readonly string[]}>[];environment:Readonly<Record<string,string|undefined>>}>):Promise<void>{
 const seen=new Set<string>();
 for(const request of admitted.requests){const manifest=resolve(request.manifest);if(seen.has(manifest)||snapshotSourceV1(options,manifest))continue;seen.add(manifest);await ensureRepositoryCargoLockV1({workspaceRoot:options.workspace,cwd:request.cwd,program:"cargo",prefix:[],args:request.args,environment:admitted.environment,control:options.control,force:false});}
}

/** 🏪️ Holds genuine bounded registry writer/read custody across fetch, planning and terminal compiler settlement; the compiler writer custody is taken BEFORE the fetch lease, so the exclusive fetch lease is held only while the fetch itself runs and never while waiting for the writer. */
async function withRepositoryCargoInputsOwnedV1<T>(options:RepositoryCargoInputsV1,view:RepositoryCargoInputsViewV1,body:(view:RepositoryCargoInputsViewV1)=>Promise<T>):Promise<T>{
 const {control,storage}=options,{requests,home,environment}=await profilePhaseV1("cargo.inputs.admit",()=>admitRepositoryCargoInputsV1(options,view.selections));
 return withRepositoryCargoRegistryReadyV1({...options,requests,home,environment},inner=>withCompilerWorkspaceSharedWriteV1(storage,machinePathV1(options.workspace,".cache/b/l/m/data"),control,async()=>{await storage.recheck();await inner();await storage.recheck();}),()=>profilePhaseV1("cargo.inputs.fetch",async()=>{try{for(const request of requests){checkResourceLeaseControl(control);const run=()=>captureOwnedCommandV1("cargo",request.args,request.cwd,{control,env:{...environment,CARGO_NET_OFFLINE:String(request.args.includes("--offline"))},output:{maximumBytes:16777216,maximumLines:65536}}),fetch=()=>request.args.includes("--offline")?run():fleetCargoRegistryAroundV1(fleetFetchContextV1(environment,options.workspace),{home,manifest:request.manifest,workspace:options.workspace},run);let output:string;try{output=await fetch();}catch(error){if(!cargoLockStaleFailureV1(error))throw error;if(snapshotSourceV1(options,request.manifest)){const origin=Object.freeze({root:resolve(options.sourceRoot!),origin:resolve(options.workspace)}),live=(value:string)=>checkoutSnapshotOriginPathV1(origin,value);await ensureRepositoryCargoLockV1({workspaceRoot:options.workspace,cwd:live(request.cwd),program:"cargo",prefix:[],args:request.args.map(live),environment,control:options.control,force:true});throw concurrentEditV1("input",request.manifest,"The captured source snapshot held a Cargo lockfile that did not match its manifests; the live lockfile was refreshed","build");}await ensureRepositoryCargoLockV1({workspaceRoot:options.workspace,cwd:request.cwd,program:"cargo",prefix:[],args:request.args,environment,control,force:true});output=await fetch();}if(output)process.stderr.write(output);}}catch(error){if(environment.CARGO_NET_OFFLINE==="true")throw Error("Original offline Cargo dependency acquisition refused: "+(error instanceof Error?error.message:String(error)));throw error;}}),()=>body(view));
}

/** 📝️ Holds the bounded registry for a lockfile resolution that must not first fetch its stale locked graph: always as a shared cohort reader, never exclusively, so a build or test that reads the registry for an hour never queues a resolve and a resolve never queues behind it (Cargo's own package-cache lock serializes an online index write, and readers re-verify their receipts, exactly as beside a ready fetch); the shared compiler writer custody (image) is entered after it. The body runs under that custody, so it must request no lease ranked below it, a Cargo build-directory custody belonging to compilers and never to a resolution, nor any lease ranked above the image mapping its storage checks request (the exclusive registry-fetch lease stays with the ready fetch). */
async function withRepositoryCargoResolutionOwnedV1<T>(options:RepositoryCargoInputsV1,view:RepositoryCargoInputsViewV1,body:(view:RepositoryCargoInputsViewV1)=>Promise<T>):Promise<T>{
 const {storage}=options,{leaseOptions}=await admitRepositoryCargoInputsV1(options,view.selections),registry=await acquireQueuedResourceLease({...leaseOptions,owner:randomBytes(8).toString("base64url"),mode:"shared",sharedCohort:true});
 try{return await withCompilerWorkspaceSharedWriteV1(storage,leaseOptions.directory,options.control,async()=>{await storage.recheck();return body(view);});}finally{registry.release();}
}

/** 🚩️ Requires equal rustc and rustdoc flag views among the selections of one target: a pass for another target (such as the host scheduler beside wasm passes) legitimately resolves its own target-scoped flags. */
export function requireCompatibleCargoFlagViewsV1(args:readonly(readonly string[])[],views:readonly Pick<RepositoryCargoConfigurationViewV1,"environment">[]):void{
 const targetOf=(row:readonly string[])=>{const at=row.indexOf("--target");return at>=0?row[at+1]:row.find(arg=>arg.startsWith("--target="))?.slice(9);};
 for(const[position,view]of views.entries())for(const[peer,other]of views.entries())if(peer<position&&targetOf(args[position]!)===targetOf(args[peer]!))for(const name of["CARGO_ENCODED_RUSTFLAGS","CARGO_ENCODED_RUSTDOCFLAGS"])if(view.environment[name]!==other.environment[name])throw Error("Distinct Cargo target flag views require distinct compiler invocation scopes");
}

async function withRepositoryCargoConfigurationViewsV1<T>(options:RepositoryCargoInputsV1,body:(view:RepositoryCargoInputsViewV1)=>Promise<T>):Promise<T>{
 const selections=[{manifest:options.manifest,cwd:options.cwd,args:options.args},...(options.additionalManifests??[])],views:RepositoryCargoConfigurationViewV1[]=[];
 const hold=(index:number,control:ResourceLeaseControl):Promise<T>=>index===selections.length?(async()=>{const first=views[0]!;requireCompatibleCargoFlagViewsV1(selections.map(row=>row.args),views);const rows:RepositoryCargoInputSelectionV1[]=[];for(const[position,row]of selections.entries()){const configuration=views[position]!;rows.push(Object.freeze({manifest:resolve(row.manifest),cwd:resolve(row.cwd),arguments:await repositoryCargoConfigurationArgumentsV1(configuration,row.args),configuration}));}const environment=Object.freeze({...first.environment,SEMIO_CARGO_CONFIGURATION_DIGESTS:JSON.stringify(views.map(view=>view.digest))}),recheck=()=>withIssuedResourceOperationV1(control,async()=>{for(const view of views)await view.recheck();}),view:RepositoryCargoInputsViewV1=Object.freeze({environment,control,selections:Object.freeze(rows),recheck});await recheck();const result=await body(view);await recheck();return result;})():withRepositoryCargoConfigurationV1({...options,manifest:selections[index]!.manifest,cwd:selections[index]!.cwd,args:selections[index]!.args,control},async view=>{views.push(view);try{return await hold(index+1,view.control);}finally{views.pop();}});
 return hold(0,options.control);
}
/** 📥️ Keeps admitted per-selection configuration bytes under exact reader custody through terminal consumption. */
export async function withRepositoryCargoInputsV1<T>(options:RepositoryCargoInputsV1,body:(view:RepositoryCargoInputsViewV1)=>Promise<T>):Promise<T>{if(options.consumption.kind==="resolution"&&!options.additionalManifests?.length&&repositoryCargoDependencyFreeV1(options.args))return withRepositoryCargoConfigurationViewsV1(options,async view=>{await admitRepositoryCargoInputsV1(options,view.selections);return body(view);});await profilePhaseV1("cargo.inputs.lock-check",async()=>ensureRepositoryCargoLocksV1(options,await admitRepositoryCargoInputsV1(options,null)));const adopted=options.additionalManifests?.length?undefined:adoptedCargoSelections.getStore()?.get(options.storage)?.get(adoptedCargoSelectionKeyV1(options.manifest,options.cwd,options.args,options.consumption,options.environment));if(adopted)return withRequestedIssuedResourceControlV1(options.control,0,control=>withRepositoryCargoInputsOwnedV1({...options,control,environment:adopted.environment,args:adopted.selection.arguments,additionalManifests:[]},Object.freeze({environment:adopted.environment,control,selections:Object.freeze([adopted.selection]),recheck:()=>withIssuedResourceOperationV1(control,()=>adopted.selection.configuration.recheck())}),body));return withRepositoryCargoConfigurationViewsV1(options,view=>withRepositoryCargoInputsOwnedV1({...options,control:view.control,environment:view.environment,args:view.selections[0]!.arguments,additionalManifests:view.selections.slice(1).map(row=>({manifest:row.manifest,cwd:row.cwd,args:row.arguments}))},view,body));}
/** 🔒️ Keeps the same genuine configuration reader across an exclusive original lock/resolve producer. */
export async function withRepositoryCargoResolutionV1<T>(options:RepositoryCargoInputsV1,body:(view:RepositoryCargoInputsViewV1)=>Promise<T>):Promise<T>{if(options.consumption.kind!=="resolution")throw Error("Explicit Cargo resolution consumption required");await admitRepositoryCargoInputsV1(options,null);return withRepositoryCargoConfigurationViewsV1(options,view=>withRepositoryCargoResolutionOwnedV1({...options,control:view.control,environment:view.environment},view,body));}

/** 🔌️ Retains genuine repository inputs around one final funded compiler request and its result consumer, and offers the shared compiler writer custody instead of holding it: the consumer enters it after its Cargo store, custody and recipe leases, so the writer (image) is never held while a lower-ranked Cargo lease is requested.
 * @see ../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️inputs/🟦️.ts */
export function repositoryCargoCompilerInputsV1(options:Pick<RepositoryCargoInputsV1,"workspace"|"sourceRoot"|"invocation"|"consumption"|"control"|"storage"|"network">):CargoCompilerInputsPortV1{
 requireIssuedResourceControlInvocationV1(options.control,options.invocation);requireNativeStorageReservationV1(options.storage,options.control);const held=Object.freeze({...options,consumption:Object.freeze({...options.consumption})});return(request,body)=>{requireIssuedResourceControlRefinementV1(held.control,request.control);const build=request.environment.CARGO_BUILD_BUILD_DIR;if(!build)throw Error("Actual final compiler writer environment required");const args=repositoryCargoManifestArgumentsV1(held,request.manifest,request.arguments,build);return withRepositoryCargoInputsV1({...held,cwd:request.cwd,manifest:request.manifest,args,control:request.control,environment:request.environment},async input=>{const selected=input.selections[0];if(input.selections.length!==1||!selected||selected.manifest!==resolve(request.manifest)||selected.cwd!==resolve(request.cwd))throw Error("One exact final compiler input view required");await input.recheck();const result=await body(Object.freeze({arguments:selected.arguments,environment:input.environment,control:input.control,admit:async(args:readonly string[],control:ResourceLeaseControl)=>{requireIssuedResourceControlRefinementV1(input.control,control);return repositoryCargoConfigurationStepArgumentsV1(selected.configuration,repositoryCargoManifestArgumentsV1(held,request.manifest,args,build));},writer:<R>(control:ResourceLeaseControl,run:(control:ResourceLeaseControl)=>Promise<R>)=>withCompilerWorkspaceSharedWriteV1(held.storage,machinePathV1(held.workspace,".cache/b/l/m/data"),control,run)}));await input.recheck();return result;});};
}

/** 📚️ Fetches every declared full pass resolve and adopts every declared configuration view before one real registry reader spans planning through final consumption; a later identical request reuses its held view instead of capturing, publishing or leasing configuration again under compiler custody. */
export function withRepositoryCargoPassInputsV1<T>(options:Pick<RepositoryCargoInputsV1,"workspace"|"sourceRoot"|"invocation"|"control"|"storage"|"network">,requests:readonly CargoCompilerInputsRequestV1[],body:(control:ResourceLeaseControl)=>Promise<T>):Promise<T>{
 requireIssuedResourceControlInvocationV1(options.control,options.invocation);requireNativeStorageReservationV1(options.storage,options.control);const first=requests[0];if(!first||requests.length>continuationSchema.const.maximumInputs)throw Error("Finite actual declared Cargo pass inputs required");const environment=Object.freeze({...first.environment});for(const row of requests){requireIssuedResourceControlRefinementV1(options.control,row.control);for(const name of["CARGO_HOME","CARGO_TARGET_DIR","CARGO_BUILD_BUILD_DIR"])if(!environment[name]||row.environment[name]!==environment[name])throw Error("Exact common declared pass storage required");}
 const consumption=repositoryCargoConsumptionV1(first.arguments,"artifact");
 const declared=requests.map(row=>({...row,arguments:repositoryCargoManifestArgumentsV1(options,row.manifest,row.arguments,environment.CARGO_BUILD_BUILD_DIR!)}));
 const selectionKey=(row:CargoCompilerInputsRequestV1,selectedEnvironment:CargoCompilerInputsRequestV1["environment"])=>adoptedCargoSelectionKeyV1(row.manifest,row.cwd,row.arguments,consumption,selectedEnvironment);
 const keys=declared.map(row=>selectionKey(row,row.environment));
 const held=adoptedCargoSelections.getStore()?.get(options.storage),borrowed=keys.map(key=>held?.get(key));
 if(borrowed.every((entry):entry is AdoptedCargoSelectionV1=>entry!==undefined)){
  return withRequestedIssuedResourceControlV1(options.control,0,async control=>{
   for(const entry of borrowed)await entry.selection.configuration.recheck();
   const result=await body(control);
   for(const entry of borrowed)await entry.selection.configuration.recheck();
   return result;
  });
 }
 const primary=declared[0]!;
 return withRepositoryCargoInputsV1({...options,manifest:primary.manifest,cwd:primary.cwd,args:primary.arguments,additionalManifests:declared.slice(1).map(row=>({manifest:row.manifest,cwd:row.cwd,args:row.arguments})),environment,consumption},async input=>{
  await input.recheck();
  const adopted=new Map(held);
  for(const [position,row] of declared.entries()){
   const key=selectionKey(row,environment),selection=input.selections[position]!;
   adopted.set(key,Object.freeze({selection,environment:Object.freeze({...selection.configuration.environment,SEMIO_CARGO_CONFIGURATION_DIGESTS:JSON.stringify([selection.configuration.digest])})}));
  }
  const groups=new Map(adoptedCargoSelections.getStore());
  groups.set(options.storage,adopted);
  try{return await adoptedCargoSelections.run(groups,async()=>{const result=await body(input.control);await input.recheck();return result;});}finally{adopted.clear();}
 });
}

/** 👓️ Holds actual current configuration and registry SH for observed bytes, without fetching or granting historical custody. */
export async function withRepositoryCargoInputReaderV1<T>(options:RepositoryCargoInputsV1,body:(input:RepositoryCargoInputsViewV1)=>Promise<T>):Promise<T>{
 await admitRepositoryCargoInputsV1(options,null);return withRepositoryCargoConfigurationViewsV1(options,async view=>{const actual={...options,control:view.control,environment:view.environment,args:view.selections[0]!.arguments,additionalManifests:view.selections.slice(1).map(row=>({manifest:row.manifest,cwd:row.cwd,args:row.arguments}))},{leaseOptions}=await admitRepositoryCargoInputsV1(actual,view.selections),reader=await acquireQueuedResourceLease({...leaseOptions,owner:randomBytes(8).toString("base64url"),mode:"shared",sharedCohort:true});try{await view.recheck();const result=await withRequestedIssuedResourceControlV1(view.control,0,control=>body(Object.freeze({...view,control,recheck:()=>withIssuedResourceOperationV1(control,async()=>{checkResourceLeaseControl(control);await view.recheck();checkResourceLeaseControl(control);})})));await view.recheck();return result;}finally{reader.release();}});
}
