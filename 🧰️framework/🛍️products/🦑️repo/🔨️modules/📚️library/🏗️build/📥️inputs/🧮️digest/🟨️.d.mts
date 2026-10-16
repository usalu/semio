/** 🧮️ Hashes a closed exact Source inventory with current bytes and absence. */
export function sourceClosureDigest(workspaceRoot:string,value:unknown,files?:Map<string,string>):string;
/** 🏗️ Reads task Source inputs; consumers validate the returned environment DATA. */
export function sourceClosureTaskEnvironments(workspaceRoot:string,plans:Readonly<Record<string,readonly string[]>>,environments:Readonly<Record<string,Readonly<Record<string,string|undefined>>>>):unknown;
/** 🔐️ Compares a closed Source receipt with its current digests. */
export function sourceClosureReceiptHolds(workspaceRoot:string,receipt:unknown):boolean;
