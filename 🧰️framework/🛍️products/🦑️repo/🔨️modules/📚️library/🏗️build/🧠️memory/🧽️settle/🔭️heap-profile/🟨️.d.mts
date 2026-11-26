/** 📏️ The declared numbers of the heap profile. */
export const HEAP_PROFILE_POLICY_V1:Readonly<{minimumManagedMegabytes:number;retainers:number;classes:number;hops:number}>;
/** 🔬️ What a heap snapshot holds: classes by self bytes and the deepest nodes of the heavy retention chains. */
export interface HeapProfileV1{readonly nodes:number;readonly edges:number;readonly totalBytes:number;readonly classes:readonly{name:string;count:number;bytes:number}[];readonly retainers:readonly{bytes:number;strings:number;path:string}[]}
export function summariseHeapSnapshotV1(snapshot:unknown,options?:{retainers?:number;classes?:number;hops?:number}):HeapProfileV1;
/** 📸️ One summarised snapshot per process once the managed heap passed the minimum and a directory is given. */
export function profileHeapOnceV1(options?:{directory?:string;managedMegabytes?:number;generate?:()=>unknown;name?:string;minimum?:number}):HeapProfileV1|undefined;
