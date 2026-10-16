import {requireNxGraphNativeDirectoryV1,withReceivedNxGraphTaskV1} from "../🟦️.ts";
import {sourceClosureTaskEnvironments as taskEnvironments,sourceClosureReceiptHolds as receiptHolds} from "../../../../../../🏗️build/📥️inputs/🧮️digest/🟨️.mjs";

import{captureNxSourceHashInputsV1,captureNxSourceReceiptV1,captureNxSourceEnvironmentsV1,type NxSourceHashPlansV1,type NxSourceHashEnvironmentsV1}from"./📋️data/🟦️.ts";
export type{NxSourceHashPlansV1,NxSourceHashEnvironmentsV1}from"./📋️data/🟦️.ts";

/** 🏗️ Retains genuine received Graph custody through exact Source hashing and its terminal checks. */
export async function sourceClosureTaskEnvironments(workspaceRoot:string,plans:NxSourceHashPlansV1,values:NxSourceHashEnvironmentsV1,environment:Readonly<Record<string,string|undefined>>):Promise<NxSourceHashEnvironmentsV1>{
 requireNxGraphNativeDirectoryV1(workspaceRoot,environment);const captured=captureNxSourceHashInputsV1(plans,values);return withReceivedNxGraphTaskV1(workspaceRoot,environment,async actual=>{await actual.yieldContinuation();actual.signal.throwIfAborted();const result=captureNxSourceEnvironmentsV1(taskEnvironments(workspaceRoot,captured.plans,captured.values));actual.signal.throwIfAborted();return result;});
}

/** 🔐️ Rechecks promotion receipts only inside the live received Graph reader operation. */
export async function sourceClosureReceiptHolds(workspaceRoot:string,receipt:unknown,environment:Readonly<Record<string,string|undefined>>):Promise<boolean>{
 requireNxGraphNativeDirectoryV1(workspaceRoot,environment);const captured=captureNxSourceReceiptV1(receipt);return withReceivedNxGraphTaskV1(workspaceRoot,environment,async actual=>{await actual.yieldContinuation();actual.signal.throwIfAborted();const result:unknown=receiptHolds(workspaceRoot,captured);if(typeof result!=="boolean")throw Error("Source receipt decision DATA required");actual.signal.throwIfAborted();return result;});
}
