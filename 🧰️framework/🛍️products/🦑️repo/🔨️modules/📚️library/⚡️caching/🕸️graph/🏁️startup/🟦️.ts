import {withRepositoryRuntimeEntriesV1} from "../../../🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🟦️.ts";
import {readNativeCallerAuthorityV1} from "../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import {profilePhaseV1} from "../../../🏗️build/⏱️profile/🟨️.mjs";
import {prepareNxDaemonNativeV1} from "./🧲️native/🟦️.ts";
import {withRepositoryToolingStoreV1} from "../../💾️storage/🔒️enforcement/🛠️tools/🟦️.ts";
import {withNxDaemonServiceBirthV1} from "./🚦️service/🟦️.ts";
import {graphResourceLeaseDirectoryV1} from "./🗃️lease-directory/🟦️.ts";
import {createHash} from "node:crypto";
import {dirname,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {checkResourceLeaseControl,requireMaterializedPathV1,withResourceLeases,type ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {findWorkspaceRoot,receiveScriptProcessInvocation,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";

type Environment=Readonly<Record<string,string|undefined>>;
/** 🪜️ The steps of one daemon start, in the order they nest: the tools store and the birth (images, toolkit use, namespace) are open before the exclusive start lease is taken, so nothing waits for a storage lease while the start is held. */
export type NxDaemonStartupStepsV1=Readonly<{
 receive<T>(environment:Environment,run:(original:ScriptInvocation)=>Promise<T>):Promise<T>;
 admit(original:ScriptInvocation):ResourceLeaseControl;
 native(original:ScriptInvocation,control:ResourceLeaseControl,environment:Environment):Environment;
 entries<T>(original:ScriptInvocation,control:ResourceLeaseControl,run:()=>Promise<T>):Promise<T>;
 store<T>(original:ScriptInvocation,control:ResourceLeaseControl,run:(phase:ResourceLeaseControl)=>Promise<T>):Promise<T>;
 birth<T>(original:ScriptInvocation,phase:ResourceLeaseControl,namespace:string,toolingIdentity:string,environment:Environment,run:(environment:Environment,phase:ResourceLeaseControl)=>Promise<T>):Promise<T>;
 lease<T>(phase:ResourceLeaseControl,resource:string,run:()=>Promise<T>):Promise<T>;
}>;

const steps:NxDaemonStartupStepsV1=Object.freeze({
 receive:(environment,run)=>receiveScriptProcessInvocation(environment,run),
 admit:original=>scriptCompilerAdmissionControlV1(original,"nx:daemon-startup",0),
 native:prepareNxDaemonNativeV1,
 entries:(original,control,run)=>withRepositoryRuntimeEntriesV1(readNativeCallerAuthorityV1(original.capabilities).repositoryRoot,original,control,run),
 store:(original,control,run)=>withRepositoryToolingStoreV1(findWorkspaceRoot(dirname(fileURLToPath(import.meta.url))),original,control,(_store,storePhase)=>run(storePhase)),
 birth:(original,phase,namespace,toolingIdentity,environment,run)=>withNxDaemonServiceBirthV1(original,phase,namespace,toolingIdentity,environment,run),
 lease:(phase,resource,run)=>withResourceLeases({directory:graphResourceLeaseDirectoryV1(),resources:[{resource,mode:"exclusive"}],signal:phase.signal,control:phase},run)
});

/** 🏁️ One namespace's original clients recheck physical readiness before starting its immutable daemon. The exclusive start lease is the innermost lease of the start (class `daemon-startup`, the highest rank): the store, the images, the toolkit use and the namespace are opened first, so the start never holds it across a storage wait. */
export async function withNxDaemonStartupV1<T>(directory:string,toolingIdentity:string,environment:Environment,body:(control:ResourceLeaseControl,environment:Environment)=>Promise<T>,order:NxDaemonStartupStepsV1=steps):Promise<T>{
 if(!/^[a-f0-9]{64}$/u.test(toolingIdentity))throw Error("Original daemon startup toolkit required");
 return order.receive(environment,original=>{
  const control=order.admit(original),prepared=profilePhaseV1("nx.daemon.native",()=>order.native(original,control,environment));
  return order.entries(original,control,()=>{
   const namespace=requireMaterializedPathV1(resolve(directory)),resource="nx:daemon-startup:"+createHash("sha256").update(toolingIdentity+"\0"+namespace).digest("hex");
   return profilePhaseV1("nx.daemon.store",()=>order.store(original,control,storePhase=>profilePhaseV1("nx.daemon.birth",()=>order.birth(original,storePhase,namespace,toolingIdentity,prepared,(childEnvironment,birthPhase)=>profilePhaseV1("nx.daemon.lease",()=>order.lease(birthPhase,resource,async()=>{checkResourceLeaseControl(birthPhase);const result=await body(birthPhase,childEnvironment);checkResourceLeaseControl(birthPhase);return result;}))))));
  });
 });

}
