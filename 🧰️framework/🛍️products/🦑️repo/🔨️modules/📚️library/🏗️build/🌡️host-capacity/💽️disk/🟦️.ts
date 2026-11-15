import {homedir} from "node:os";
import {resolveHostCapacityLanguageV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import {volumeMountOfV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🆓️free-space/💽️disk-full/🟦️.ts";
import {diskCollectingLineV1,refuseLowSpaceV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/💽️disk/🟦️.ts";
import {casHomeOfV1} from "../../🗄️cas/⌨️command/🟦️.ts";
import {createCasGuardV1,type CasGuardV1} from "../../🗄️cas/🚦️guard/🟦️.ts";
import {decideCasEnforcementV1,measureCasPartitionV1,type CasPartitionV1} from "../../🗄️cas/🌡️partition/🟦️.ts";
import {casRootV1} from "../../🗄️cas/🗃️store/🟦️.ts";
import {CAS_POLICY_V1} from "../../🗄️cas/🧬️schema/🟦️.ts";

/** 🔌️ What the disk admission touches, replaceable by tests: the guard of the cache store, the workspace whose partition is checked too, the measurement and the mount of a path, and the line writer. */
export type BuildDiskPortsV1=Readonly<{guard?:CasGuardV1;workspace?:string;partition?:(path:string)=>CasPartitionV1|undefined;mount?:(path:string)=>string;write?:(line:string)=>void;signal?:AbortSignal;expectedBytes?:number}>;

/** 💽️ Admits a heavy command to the cache partition before it starts writing. The partition of the host cache store is judged like a cache write of no bytes: at its high used share a collection of the least recently accessed objects starts in the background and the command goes on; at its critical share the command first waits for that collection (with a progress line that names the partition and its share) and is refused with the typed `disk.low-space` refusal when the share is still critical, which names the partition, its free bytes and the gc command. The partition of the workspace is held to the same critical share when it is another one. An environment that is no real user session (no home, no explicit store root) leaves the host alone. @see ../../🗄️cas/🌡️partition/🟦️.ts */
export async function admitBuildDiskV1(environment:Readonly<Record<string,string|undefined>>,ports:BuildDiskPortsV1={}):Promise<void>{
 const home=casHomeOfV1(environment);
 if(home===undefined&&!environment.SEMIO_CAS_ROOT)return;
 const root=casRootV1(environment,process.platform,home??homedir()),language=resolveHostCapacityLanguageV1(environment),write=ports.write??(line=>process.stderr.write(line+"\n")),measure=ports.partition??measureCasPartitionV1,mount=ports.mount??volumeMountOfV1;
 const guard=ports.guard??createCasGuardV1({root,home:home??homedir(),partition:measure,report:facts=>write(diskCollectingLineV1({...facts,limitFraction:CAS_POLICY_V1.partition.criticalUsedFraction},language))});
 const expected=Math.max(0,Math.floor(ports.expectedBytes??0));
 await guard.admit(expected,ports.signal);
 const workspace=ports.workspace??process.cwd();
 if(mount(workspace)===mount(root))return;
 const partition=measure(workspace);
 if(partition&&decideCasEnforcementV1({policy:CAS_POLICY_V1,storeBytes:0,incomingBytes:expected,partition,collected:true}).action==="refuse")throw refuseLowSpaceV1({partition:partition.partition,freeBytes:partition.freeBytes,totalBytes:partition.totalBytes,limitFraction:CAS_POLICY_V1.partition.criticalUsedFraction});
}

const EXPECTED_GROWTH_BYTES_V1:Readonly<Record<string,number>>=Object.freeze({cargo:4*1024**3});
/** 📈️ The disk growth a command class is expected to cause before it starts: `SEMIO_BUILD_EXPECTED_DISK_BYTES` when set, else 4 GiB for a Cargo build (target directories and unit outputs; a 47-minute plugin build grew them by 18 GB), nothing for the other classes. The admission refuses or waits for a collection when the growth would cross the critical share instead of discovering it at 98 %. */
export function expectedDiskGrowthBytesV1(environment:Readonly<Record<string,string|undefined>>,operation:string):number{
 const named=Number(environment.SEMIO_BUILD_EXPECTED_DISK_BYTES);
 return Number.isSafeInteger(named)&&named>=0&&environment.SEMIO_BUILD_EXPECTED_DISK_BYTES!==undefined?named:EXPECTED_GROWTH_BYTES_V1[operation]??0;
}
