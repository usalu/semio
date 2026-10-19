/** 🧾️ Describes closed retained usage without exporting a database type. */
export interface BuildGraphOptionalLedgerStateV1{readonly version:1;readonly root:string;readonly maximumBytes:number;readonly usedBytes:number;readonly dirty:boolean}
/** 📒️ Serializes only actual optional publication, retaining dirty recovery across crashes. */
export interface BuildGraphOptionalLedgerV1{observe():BuildGraphOptionalLedgerStateV1;publish(plan:{readonly peakBytes:number;readonly retainedDeltaBytes:number},write:()=>void):boolean;retire(write:()=>void):number;close():void}
/** 🗃️ Opens a finite system transaction against its caller's original physical inventory. */
export function openBuildGraphOptionalLedgerV1(options:{readonly image:import("../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts").NativeImageStorageV1;readonly control:import("@semio-tech/process/leases").ResourceLeaseControl;readonly leaseDirectory:string;readonly maximumBytes:number;readonly minimumAvailableBytes:number}):Promise<BuildGraphOptionalLedgerV1>;
