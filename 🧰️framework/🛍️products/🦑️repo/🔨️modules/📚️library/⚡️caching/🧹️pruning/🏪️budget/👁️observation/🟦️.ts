import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import{nativeDirectoryHostAvailableBytesV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🟦️.ts";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

/** 👁️ Chooses only a local read-only process observation; this grants no descriptor or retirement custody. */
export function cargoMaintenanceObservationPlanV1(platform:string):Readonly<{command:string;args:readonly string[]}>{const platforms=schema.definitions.Observation.const.platforms;if(!Object.hasOwn(platforms,platform))throw Error("Qualified maintenance observation platform required");const selected=platforms[platform as keyof typeof platforms];return Object.freeze({command:selected.command,args:Object.freeze([...selected.args])});}

/** 💽️ Observes actual available bytes through the existing POSIX/Win32 host interface and the caller's unchanged control. */
export function cargoMaintenanceAvailableBytesV1(root:string,control:ResourceLeaseControl):number{checkResourceLeaseControl(control);const value=nativeDirectoryHostAvailableBytesV1(root);checkResourceLeaseControl(control);if(!Number.isSafeInteger(value)||value<0)throw Error("Actual physical headroom required");return value;}
