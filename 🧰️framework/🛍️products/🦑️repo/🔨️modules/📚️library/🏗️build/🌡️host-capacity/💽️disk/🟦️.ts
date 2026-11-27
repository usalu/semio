import {homedir,tmpdir} from "node:os";
import {resolveHostCapacityLanguageV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import {volumeMountOfV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🆓️free-space/💽️disk-full/🟦️.ts";
import {diskCollectingLineV1,refuseLowSpaceV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/💽️disk/🟦️.ts";
import {casHomeOfV1} from "../../🗄️cas/⌨️command/🟦️.ts";
import {createCasGuardV1,type CasGuardV1} from "../../🗄️cas/🚦️guard/🟦️.ts";
import {casPartitionPolicyForV1,decideCasEnforcementV1,measureCasPartitionV1,type CasPartitionV1} from "../../🗄️cas/🌡️partition/🟦️.ts";
import {casRootV1} from "../../🗄️cas/🗃️store/🟦️.ts";
import {CAS_POLICY_V1} from "../../🗄️cas/🧬️schema/🟦️.ts";
import {DISK_RECOVERY_POLICY_V1,judgeDiskV1,recoverDiskV1,refuseDiskRecoveryV1,type DiskWriterV1} from "../🧹️disk-recovery/🟦️.ts";
import {realDiskWritersV1} from "../🧹️disk-recovery/🔌️real/🟦️.ts";

/** 🔌️ What the disk admission touches, replaceable by tests: the guard of the cache store, the workspace whose partition is checked too, the measurement and the mount of a path, and the line writer. */
export type BuildDiskPortsV1=Readonly<{guard?:CasGuardV1;workspace?:string;partition?:(path:string)=>CasPartitionV1|undefined;mount?:(path:string)=>string;write?:(line:string)=>void;signal?:AbortSignal;expectedBytes?:number;writers?:readonly DiskWriterV1[]}>;

/** 💽️ Admits a heavy command to the cache partition before it starts writing. The partition of the host cache store is judged like a cache write of no bytes: at its high used share a collection of the least recently accessed objects starts in the background and the command goes on; at its critical share the command first waits for that collection (with a progress line that names the partition and its share) and is refused with the typed `disk.low-space` refusal when the share is still critical, which names the partition, its free bytes and the gc command. The partition of the workspace is held to the same critical share when it is another one. An environment that is no real user session (no home, no explicit store root) leaves the host alone. @see ../../🗄️cas/🌡️partition/🟦️.ts */
export async function admitBuildDiskV1(environment:Readonly<Record<string,string|undefined>>,ports:BuildDiskPortsV1={}):Promise<DeferredReliefV1>{
 const home=casHomeOfV1(environment);
 if(home===undefined&&!environment.SEMIO_CAS_ROOT)return noRelief;
 const root=casRootV1(environment,process.platform,home??homedir()),language=resolveHostCapacityLanguageV1(environment),write=ports.write??(line=>process.stderr.write(line+"\n")),measure=ports.partition??measureCasPartitionV1,mount=ports.mount??volumeMountOfV1;
 const guard=ports.guard??createCasGuardV1({root,home:home??homedir(),partition:measure,report:facts=>write(diskCollectingLineV1({...facts,limitFraction:casPartitionPolicyForV1(CAS_POLICY_V1,measure(root)).partition.criticalUsedFraction},language))});
 const expected=Math.max(0,Math.floor(ports.expectedBytes??0));
 const relief=environment[DISK_RECOVERY_POLICY_V1.environmentChild]!=="1"?await recoverBuildDiskV1({environment,root,home:home??homedir(),workspace:ports.workspace??process.cwd(),need:expected,measure,language,write,signal:ports.signal,writers:ports.writers}):noRelief;
 await guard.admit(expected,ports.signal);
 const workspace=ports.workspace??process.cwd();
 if(mount(workspace)===mount(root))return relief;
 const partition=measure(workspace);
 if(partition){const policy=casPartitionPolicyForV1(CAS_POLICY_V1,partition);if(decideCasEnforcementV1({policy,storeBytes:0,incomingBytes:expected,partition,collected:true}).action==="refuse")throw refuseLowSpaceV1({partition:partition.partition,freeBytes:partition.freeBytes,totalBytes:partition.totalBytes,limitFraction:policy.partition.criticalUsedFraction});}
 return relief;
}

/** ⏭️ The recovery that only keeps the reserve and never blocks a command: it is not started while the command is admitted but handed back to run after the command ended (once, never failing). */
export type DeferredReliefV1=()=>Promise<void>;
const noRelief:DeferredReliefV1=async()=>undefined;
const recoveries=new Map<string,Promise<unknown>>();
/** 🧹️ Frees the machine before the gate judges it: a volume that can derive its disk budget and has less than the command's need plus its reserve free runs the registered collectors in priority order (derivable artefacts first, then the least recently used cache objects). Below the cushion the command waits for them and is refused with the largest remaining consumers when they free too little; between the cushion and the reserve they run after the command, not beside it, and the command goes on. One recovery per store root at a time. @see ../🧹️disk-recovery/🟦️.ts */
async function recoverBuildDiskV1(input:Readonly<{environment:Readonly<Record<string,string|undefined>>;root:string;home:string;workspace:string;need:number;measure:(path:string)=>CasPartitionV1|undefined;language:"en"|"de";write:(line:string)=>void;signal?:AbortSignal;writers?:readonly DiskWriterV1[]}>):Promise<DeferredReliefV1>{
 const facts=()=>input.measure(input.root),current=facts(),judged=judgeDiskV1(current,input.need);
 if(current===undefined||judged.action==="fixed-shares"||judged.action==="allow")return noRelief;
 const writers=input.writers??realDiskWritersV1({
  workspace:input.workspace,home:input.home,tmp:input.environment.TMPDIR??tmpdir(),uid:process.getuid?.()??0,environment:input.environment,background:judged.action==="collect-background",
  reclaimPressure:{path:current.partition,requiredFreeBytes:judged.targetFreeBytes,
   freeBytes:()=>{const measured=facts();return measured?.partition===current.partition?measured.freeBytes:undefined;}},
 });
 const start=()=>{const running=recoveries.get(input.root)??recoverDiskV1({facts,needBytes:input.need,writers,language:input.language,report:input.write,signal:input.signal}).finally(()=>{recoveries.delete(input.root);});recoveries.set(input.root,running);return running;};
 if(judged.action==="collect-background"){let once:Promise<void>|undefined;return()=>once??=start().then(()=>undefined,()=>undefined);}
 const outcome=await start() as Awaited<ReturnType<typeof recoverDiskV1>>,last=facts();
 if(outcome.verdict==="refuse"&&last)throw refuseDiskRecoveryV1(outcome,last);
 return noRelief;
}

const EXPECTED_GROWTH_BYTES_V1:Readonly<Record<string,number>>=Object.freeze({cargo:4*1024**3});
/** 📈️ The disk growth a command class is expected to cause before it starts: `SEMIO_BUILD_EXPECTED_DISK_BYTES` when set, else 4 GiB for a Cargo build (target directories and unit outputs; a 47-minute plugin build grew them by 18 GB), nothing for the other classes. The admission refuses or waits for a collection when the growth would cross the critical share instead of discovering it at 98 %. */
export function expectedDiskGrowthBytesV1(environment:Readonly<Record<string,string|undefined>>,operation:string):number{
 const named=Number(environment.SEMIO_BUILD_EXPECTED_DISK_BYTES);
 return Number.isSafeInteger(named)&&named>=0&&environment.SEMIO_BUILD_EXPECTED_DISK_BYTES!==undefined?named:EXPECTED_GROWTH_BYTES_V1[operation]??0;
}
