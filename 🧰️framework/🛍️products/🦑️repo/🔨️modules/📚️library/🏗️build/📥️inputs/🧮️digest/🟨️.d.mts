import type{ResourceLeaseControl}from"@semio-tech/process/leases";
/** 🧮️ Hashes one current bounded exact Source inventory and absence under actual carried control. */
export function sourceClosureDigest(workspaceRoot:string,value:unknown,control:ResourceLeaseControl):Promise<string>;
/** 🏗️ Captures bounded task Source inputs; the adapter validates returned environment DATA. */
export function sourceClosureTaskEnvironments(workspaceRoot:string,plans:Readonly<Record<string,readonly string[]>>,environments:Readonly<Record<string,Readonly<Record<string,string|undefined>>>>,control:ResourceLeaseControl):Promise<Readonly<Record<string,Readonly<Record<string,string|undefined>>>>>;
/** 🔐️ Compares a closed Source receipt with a fresh current-byte batch. */
export function sourceClosureReceiptHolds(workspaceRoot:string,receipt:unknown,control:ResourceLeaseControl):Promise<boolean>;
