export type MachineSpeedPolicyV1=Readonly<{version:number;referenceCores:number;referenceMegabytes:number;memoryExponent:number;maximumFactor:number;livenessMilliseconds:number;livenessCpuMilliseconds:number}>;
export const MACHINE_SPEED_POLICY_V1:MachineSpeedPolicyV1;
export function machineSpeedFactorV1(machine:Readonly<{cores:number;totalMegabytes:number}>,policy?:MachineSpeedPolicyV1):number;
export function localMachineSpeedFactorV1(environment?:Readonly<Record<string,string|undefined>>,policy?:MachineSpeedPolicyV1):number;
export function scaledMillisecondsV1(milliseconds:number,factor:number):number;
export function processCpuMillisecondsV1(pid:number,platform?:string):number|undefined;
export function readProcessCpuMillisecondsV1(pid:number,options?:Readonly<{platform?:string;signal?:AbortSignal;run?:(command:string,args:readonly string[],signal?:AbortSignal)=>Promise<string>;read?:(path:string)=>Promise<string|null>}>):Promise<number|undefined>;
export function cpuTimeTextMillisecondsV1(text:string):number|undefined;
