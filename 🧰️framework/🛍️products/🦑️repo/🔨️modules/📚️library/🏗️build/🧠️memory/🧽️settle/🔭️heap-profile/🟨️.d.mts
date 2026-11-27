/** 📏️ The declared numbers of the heap profile. */
export interface HeapProfilePolicyV1{readonly version:1;readonly minimumManagedMegabytes:number;readonly stepManagedMegabytes:number;readonly maxSnapshots:number;readonly budgetMilliseconds:number;readonly retainers:number;readonly classes:number;readonly groups:number;readonly hops:number;readonly depth:number;readonly focus:readonly string[]}
export declare const HEAP_PROFILE_POLICY_V1:HeapProfilePolicyV1;
/** 🔬️ What a heap snapshot holds: classes by self bytes, the deepest nodes of the heavy retention chains and, per focus class, the holder groups. */
export interface HeapHolderGroupV1{readonly class:string;readonly edge:string;readonly holder:string;readonly count:number;readonly bytes:number;readonly path:string}
export interface HeapProfileV1{readonly nodes:number;readonly edges:number;readonly totalBytes:number;readonly classes:readonly{name:string;count:number;bytes:number}[];readonly retainers:readonly{bytes:number;strings:number;path:string}[];readonly holders:Readonly<Record<string,readonly HeapHolderGroupV1[]>>}
export function summariseHeapSnapshotV1(snapshot:unknown,options?:{retainers?:number;classes?:number;hops?:number;groups?:number;depth?:number;focus?:readonly string[];signal?:AbortSignal}):HeapProfileV1;
/** 📸️ The snapshots one process took. */
export interface HeapProfileStateV1{taken:number;next:number|undefined;milliseconds:number;busy:boolean}
export function createHeapProfileStateV1():HeapProfileStateV1;
/** 📸️ One summarised snapshot per further step of the managed heap, bounded in number and time. */
export function profileHeapV1(options?:{directory?:string;managedMegabytes?:number;generate?:()=>unknown;name?:(sequence:number)=>string;minimum?:number;step?:number;limit?:number;budget?:number;state?:HeapProfileStateV1;signal?:AbortSignal;now?:()=>number;memory?:()=>Record<string,number>}):HeapProfileV1|undefined;
