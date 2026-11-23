import {casHomeOfV1,runCasCommandV1,type CasCommandContextV1} from "../⌨️command/🟦️.ts";
import {measureCasPartitionV1,type CasPartitionV1} from "../🌡️partition/🟦️.ts";
import {casRootV1} from "../🗃️store/🟦️.ts";
import {homedir} from "node:os";

/** 🆘️ The free bytes below which a partition counts as full for the reclaim commands: the sealed leases, source admission and the progress receipts of a command need a few megabytes before it reaches its own collection. */
export const CAS_FULL_DISK_RESERVE_BYTES_V1=67108864;

/** 🔌️ What the first collection needs from its host; tests replace the partition measurement and the collection. */
export type CasFullDiskPortsV1=Readonly<{workspace:string;environment:Readonly<Record<string,string|undefined>>;write(line:string):void;home?:string;reserveBytes?:number;partition?(root:string):CasPartitionV1|undefined;collect?(args:readonly string[],context:CasCommandContextV1,ports:Readonly<{signal:AbortSignal;progress(stage:string):Promise<void>}>):Promise<number>}>;

/** 🆘️ Whether a command line reclaims space itself (`cache gc` or `cache cas gc`, collecting for real and over the whole machine), so that it must not wait for a free byte it is about to create. */
export function casReclaimCommandV1(args:readonly string[]):boolean{
 const rest=args[0]==="cache"&&args[1]==="gc"?args.slice(2):args[0]==="cache"&&args[1]==="cas"&&args[2]==="gc"?args.slice(3):undefined;
 return rest!==undefined&&!rest.some(flag=>flag==="--dry-run"||flag==="--area"||flag.startsWith("--area="));
}

/** 🆘️ Collects the host store before anything of the command starts when its partition is full: the collection only deletes, so it needs no lease, no admission and no receipt, all of which write and fail on a full disk. Answers whether it ran; a failure is reported and left to the command that follows. */
export async function reclaimFullDiskFirstV1(args:readonly string[],ports:CasFullDiskPortsV1):Promise<boolean>{
 if(!casReclaimCommandV1(args))return false;
 const home=ports.home??casHomeOfV1(ports.environment)??homedir(),root=casRootV1(ports.environment,process.platform,home),partition=(ports.partition??measureCasPartitionV1)(root);
 if(partition===undefined||partition.freeBytes>=(ports.reserveBytes??CAS_FULL_DISK_RESERVE_BYTES_V1))return false;
 const controller=new AbortController(),stop=()=>controller.abort(),language=args.find(flag=>flag.startsWith("--lang="));
 process.once("SIGINT",stop);process.once("SIGTERM",stop);
 try{
  await(ports.collect??runCasCommandV1)(["gc",...language===undefined?[]:[language]],{workspace:ports.workspace,environment:ports.environment,write:ports.write,home,partition:()=>partition},{signal:controller.signal,progress:async()=>undefined});
  return true;
 }catch(error){
  try{ports.write("[cas] "+(error as Error).message);}catch{}
  return false;
 }finally{process.off("SIGINT",stop);process.off("SIGTERM",stop);}
}
