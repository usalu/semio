import type {IntrinsicValue} from "@semio-tech/value-ts/schema/intrinsic";
import {valueRefusalKindFromWire,type ValueRefusalKind} from "../../🌱️value/📦️packages/🟦️typescript/⚠️refusal/🟦️.ts";
import type {ValueError,ValueErrorRetainedProgress} from "../../🌱️value/📦️packages/🟦️typescript/⚠️refusal/🟦️.ts";
import type {TextSpan} from "../📍️span/🟦️.ts";
/** 🪢️ A diagnostic cause retains its original message and optional code. */
export interface FaultCause {message:string;code?:string;retainedProgress?:ValueErrorRetainedProgress}
/** 🧭️ Original unsigned source positions retain their exact integer values. */
export interface FaultSource {offset?:bigint;byte?:number;depth?:bigint}
/** 🧾️ The original diagnostic wire carries the same failed producer's physical receipt. */
export interface Fault {origin:"edge"|"renderer"|"os"|"module"|"plugin"|"app"|"extension"|"framework";code:string;severity:"info"|"warning"|"error"|"fatal";message:string;scope:{pluginId?:string;appId?:string;instanceId?:string;module?:string;bodyKey?:string};retryable:boolean;retainedProgress:ValueErrorRetainedProgress;params?:Record<string,string>;span?:TextSpan;source?:FaultSource;domain?:Extract<IntrinsicValue,{kind:"object"}>;causes?:FaultCause[]}
/** 🫴️ Converts an original typed refusal without replacing or inferring its receipt. */
export function faultFromValueError(error:ValueError):Fault{return{origin:"framework",code:"value.refusal",severity:"error",message:error.message,scope:{},params:{refusalKind:error.kind},retryable:false,retainedProgress:error.retainedProgress};}
/** 🏷️ Borrows the original canonical cause without replacing its receipt or parameter. */
export function faultValueRefusalKind(fault:Fault):ValueRefusalKind|undefined{return valueRefusalKindFromWire(fault.params?.refusalKind);}
