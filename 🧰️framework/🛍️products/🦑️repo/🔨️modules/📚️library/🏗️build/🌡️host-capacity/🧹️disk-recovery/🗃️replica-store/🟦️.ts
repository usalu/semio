import {isAbsolute,join} from "node:path";
import {readFileSync} from "node:fs";
import {formatDiskBytesV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/💽️disk/🟦️.ts";
import {diskTextV1,type DiskLanguageV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

/** 🗃️ Who may compact the replica store: the file the machine keeps about itself and the suffix that makes an origin the group server. */
export type ReplicaStorePolicyV1=Readonly<{version:1;serverSuffix:string;machineFile:string;topology:string}>;
export const REPLICA_STORE_V1:ReplicaStorePolicyV1=Object.freeze(JSON.parse(JSON.stringify(schema.definitions.ReplicaStore.const)));

/** 🪪️ The role of this machine towards the replica store of its group: the group `server` (origin `<group>-server`, the only writer of its store), a `worker` (it mounts the store and never deletes in it) or `none` (no machine file, or a store path that is not absolute). */
export type ReplicaStoreRoleV1=Readonly<{kind:"server"|"worker"|"none";originId?:string;groupId?:string;storeRoot?:string}>;

/** 🧭️ Reads the machine file `.cache/f/machine.json`: the machine is the group server exactly when its origin is the group id plus the server suffix and it names an absolute store path. A deletion in the store is never published to the workers, they read the store itself, so the rule is by role and never by "somebody noticed". */
export function replicaStoreRoleV1(machine:unknown,policy:ReplicaStorePolicyV1=REPLICA_STORE_V1):ReplicaStoreRoleV1{
 if(machine===null||typeof machine!=="object")return{kind:"none"};
 const row=machine as Record<string,unknown>,originId=row.originId,groupId=row.groupId,storeRoot=row.storeRoot;
 if(typeof originId!=="string"||typeof groupId!=="string"||typeof storeRoot!=="string"||!isAbsolute(storeRoot))return{kind:"none"};
 return{kind:originId===groupId+policy.serverSuffix?"server":"worker",originId,groupId,storeRoot};
}
/** 📖️ Reads the machine file of a workspace, `none` when it is missing or unreadable. */
export function readReplicaStoreRoleV1(workspace:string,policy:ReplicaStorePolicyV1=REPLICA_STORE_V1):ReplicaStoreRoleV1{
 try{return replicaStoreRoleV1(JSON.parse(readFileSync(join(workspace,policy.machineFile),"utf8")),policy);}catch{return{kind:"none"};}
}

/** 🏁️ What the store's own compaction answered: the bytes it removed by blobs, segments and snapshots, or why it did nothing (`locked`: another machine holds the store-wide lock; `refused`, `empty`, `nosnapshot`, `aborted`: the store's own refusals). */
export type ReplicaCompactionV1=Readonly<{skipped:null|"locked"|"refused"|"empty"|"nosnapshot"|"aborted";bytes:number}>;

/** 🧹️ Compacts the replica store through the store's own compaction (`fleetCompactV1` in apply mode: snapshot, retirement of segments every live machine has passed, blobs that no snapshot entry and no later event names after `safetyHours`, superseded snapshots, litter), only on the group server and only under the store-wide compaction lock, with progress and cancellation. Anything else is a named skip, never a deletion. The returned bytes are what the volume gained, read again by the caller. */
export async function collectReplicaStoreV1(input:Readonly<{role:ReplicaStoreRoleV1;compact(storeRoot:string,originId:string,signal?:AbortSignal):Promise<ReplicaCompactionV1>;language:DiskLanguageV1;report?(line:string):void;signal?:AbortSignal;policy?:ReplicaStorePolicyV1}>):Promise<Readonly<{compacted:boolean;skipped:string|null;bytes:number}>>{
 const write=input.report??(()=>undefined),policy=input.policy??REPLICA_STORE_V1,text=(key:string,values:Record<string,string|number>={})=>diskTextV1(key,input.language,values),role=input.role;
 input.signal?.throwIfAborted();
 if(role.kind==="none"){write(text("store.none"));return{compacted:false,skipped:"none",bytes:0};}
 if(role.kind==="worker"){write(text("store.not-server",{server:role.groupId+policy.serverSuffix,origin:role.originId??""}));return{compacted:false,skipped:"not-server",bytes:0};}
 const result=await input.compact(role.storeRoot!,role.originId!,input.signal);
 if(result.skipped==="locked"){write(text("store.locked"));return{compacted:false,skipped:"locked",bytes:0};}
 if(result.skipped!==null){write(text("store.refused",{reason:result.skipped}));return{compacted:false,skipped:result.skipped,bytes:result.bytes};}
 write(text("store.compacted",{freed:formatDiskBytesV1(result.bytes)}));
 return{compacted:true,skipped:null,bytes:result.bytes};
}
